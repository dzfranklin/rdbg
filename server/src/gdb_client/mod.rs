mod common;
mod parser;
mod serializer;

use std::{
    convert::TryInto,
    io::{self, BufRead, BufReader},
    net::{TcpStream, ToSocketAddrs},
};
use tracing::{debug, warn};

use crate::gdb_client::{
    common::{ACK, NACK, OK},
    serializer::Q_THREAD_INFO,
};

pub use self::parser::{StopReason, ThreadId};
use self::serializer::{INTERRUPT, Q_HALT_REASON, Q_READ_GENERAL_REGISTERS};

pub type Result<T> = std::result::Result<T, Error>;

pub struct GdbClient<R, W> {
    recv: R,
    tx: W,
}

impl GdbClient<BufReader<TcpStream>, TcpStream> {
    pub fn connect(addr: impl ToSocketAddrs) -> Result<Self> {
        let tx = TcpStream::connect(addr)?;
        let recv = BufReader::new(tx.try_clone()?);

        Ok(GdbClient::new(recv, tx))
    }
}

impl<R: BufRead, W: io::Write> GdbClient<R, W> {
    pub fn new(read: R, write: W) -> Self {
        Self {
            recv: read,
            tx: write,
        }
    }

    // TODO: This just assumes x86-64

    pub fn read_program_counter(&mut self) -> Result<u64> {
        // See <https://www.embecosm.com/appnotes/ean3/embecosm-howto-gdb-porting-ean3-issue-2.html#id2694598> (2.3.5.  Specifying the Register Architecture)
        const REG_NUM_EIP: usize = 8;
        let counter = self.read_general_registers()?[REG_NUM_EIP];
        Ok(counter)
    }

    fn read_general_registers(&mut self) -> Result<Vec<u64>> {
        const GENERAL_REG_WIDTH: usize = 8;
        const GENERAL_REG_NUM: usize = 16;
        let registers = self.read_registers()?;
        let registers = registers[..GENERAL_REG_WIDTH * GENERAL_REG_NUM]
            .chunks_exact(GENERAL_REG_WIDTH)
            .map(|bytes| u64::from_le_bytes(bytes.try_into().unwrap()))
            .collect();
        Ok(registers)
    }

    fn read_registers(&mut self) -> Result<Vec<u8>> {
        // See gdb/remote.c remote_target::fetch_register_using_p
        // Despite being named general registers the reply is obviously much bigger
        // than the general registers. I think it's all the registers
        self.send_packet(Q_READ_GENERAL_REGISTERS)?;
        let resp = self.receive_packet()?;
        let (_, resp) = parser::general_registers(&resp)?;

        Ok(resp)
    }

    pub fn query_thread_ids(&mut self) -> Result<Vec<ThreadId>> {
        self.send_packet(Q_THREAD_INFO)?;
        let resp = self.receive_packet()?;
        let (_, resp) = parser::thread_info(&resp)?;
        Ok(resp)
    }

    pub fn cont(&mut self, thread: &ThreadId) -> Result<StopReason> {
        let req = serializer::vcont(b"c", thread);
        self.send_packet(&req)?;

        let resp = self.receive_packet()?;
        let (_, resp) = parser::stop_reason(&resp)?;

        Ok(resp)
    }

    pub fn break_at(&mut self, addr: u64, kind: u64) -> Result<()> {
        // TODO: Conditional breakpoints
        let req = serializer::hardware_breakpoint_req(true, addr, kind);
        self.send_packet(&req)?;
        self.expect_ok()?;
        Ok(())
    }

    pub fn remove_break_at(&mut self, addr: u64, kind: u64) -> Result<()> {
        let req = serializer::hardware_breakpoint_req(false, addr, kind);
        self.send_packet(&req)?;
        self.expect_ok()?;
        Ok(())
    }

    pub fn query_stop_reason(&mut self) -> Result<StopReason> {
        self.send_packet(Q_HALT_REASON)?;
        let body = self.receive_packet()?;
        let (_rest, reason) = parser::stop_reason(&body)?;
        Ok(reason)
    }

    pub fn interrupt(&mut self) -> Result<()> {
        self.tx.write_all(&[INTERRUPT])?;
        Ok(())
    }

    fn expect_ok(&mut self) -> Result<()> {
        let pkt = self.receive_packet()?; // will return Err on app-level error
        assert_eq!(OK, &pkt);
        Ok(())
    }

    fn receive_packet(&mut self) -> Result<Vec<u8>> {
        let mut retries = 0;
        loop {
            match self.read_packet_no_retries() {
                Ok(body) => break Ok(body),
                Err(err @ Error::App { .. }) => break Err(err),
                Err(err) => {
                    if retries >= MAX_RETRIES {
                        warn!("Failed to parse packet after retires, giving up: {}", err);
                        return Err(err);
                    }
                    warn!("Failed to parse packet, retrying: {}", err);
                    retries += 1;
                }
            }
        }
    }

    fn read_packet_no_retries(&mut self) -> Result<Vec<u8>> {
        self.expect_ack()?;
        let mut cap = PACKET_SIZE_GUESS;
        let mut len = 0;
        let mut buf = Vec::new();
        loop {
            buf.resize(cap, 0);
            let new = self.recv.read(&mut buf[len..])?;
            len += new;
            match parser::packet_body(&buf[..len]) {
                Ok((unused, body)) => {
                    assert!(unused.is_empty());

                    debug!(
                        len,
                        cap,
                        "Received packet: \"{}\" ({:?})",
                        String::from_utf8_lossy(&body),
                        body
                    );

                    self.send_ack()?;
                    break Ok(body);
                }
                Err(nom::Err::Incomplete(nom::Needed::Unknown)) => {
                    debug!(len, cap, "Incomplete packet, unknown needed");
                    if len >= (cap * 2) / 3 {
                        cap *= 2;
                        debug!("Resized buf to {cap}");
                    }
                    self.send_nack()?;
                }
                Err(nom::Err::Incomplete(nom::Needed::Size(needed))) => {
                    debug!(len, cap, needed, "Incomplete packet, known needed");
                    cap += needed.get();
                    self.send_nack()?;
                }
                Err(err) => return Err(err.into()),
            }
        }
    }

    fn send_packet(&mut self, body: &[u8]) -> Result<()> {
        debug!(
            "Sending packet: \"{}\" ({:?})",
            String::from_utf8_lossy(body),
            body
        );

        let packet = serializer::packet(body);
        self.tx.write_all(&packet)?;
        self.tx.flush()?;
        Ok(())
    }

    fn expect_ack(&mut self) -> Result<()> {
        let mut buf = vec![0];
        self.recv.read_exact(&mut buf)?;
        match *buf.get(0).unwrap() {
            ACK => Ok(()),
            NACK => Err(Error::Nack),
            byte => Err(Error::ExpectedAck(byte as char)),
        }
    }

    fn send_ack(&mut self) -> Result<()> {
        self.tx.write_all(&[ACK])?;
        Ok(())
    }

    fn send_nack(&mut self) -> Result<()> {
        self.tx.write_all(&[NACK])?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// IO Error
    Io(#[from] io::Error),
    /// Received negative acknowledgement (NACK)
    Nack,
    /// Expected acknowledgment (ACK), got byte {0}
    ExpectedAck(char),
    /// Failed to parse response as incomplete
    IncompleteResponse,
    /// Failed to parse response: {kind}. Got: {got:?}
    ParseResponse {
        kind: parser::ParseErrorKind,
        got: Vec<u8>,
        context: Option<&'static str>,
        causes: Vec<parser::ErrorKind>,
    },
    /// Application level error: {0}
    App(parser::AppErrorKind),
}

impl From<nom::Err<parser::Error>> for Error {
    fn from(err: nom::Err<parser::Error>) -> Self {
        match err {
            nom::Err::Incomplete(_) => Self::IncompleteResponse,
            nom::Err::Error(err) | nom::Err::Failure(err) => match err.kind {
                parser::ErrorKind::Parse(kind) => Self::ParseResponse {
                    kind,
                    got: err.input,
                    context: err.context,
                    causes: err.causes,
                },
                parser::ErrorKind::App(kind) => Self::App(kind),
            },
        }
    }
}

const MAX_RETRIES: usize = 10;
const PACKET_SIZE_GUESS: usize = 512;

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;
    use crate::{replay::spawn_server, test_common::SIMPLE_MAIN};
    use crate::{
        test_common::{init, trace_simple, Result},
        Trace,
    };

    fn fixture() -> eyre::Result<(Trace, GdbClient<BufReader<TcpStream>, TcpStream>)> {
        init();
        let trace = trace_simple();
        let (_server, addr) = spawn_server(&trace)?;
        let subject = GdbClient::connect(addr)?;
        Ok((trace, subject))
    }

    #[test]
    fn test_read_program_counter() -> Result {
        let (_trace, mut subject) = fixture()?;
        // subject.break_at(SIMPLE_MAIN, 1)?;
        subject.cont(&ThreadId::All)?;
        let counter = subject.read_program_counter()?;
        eprintln!("{}", SIMPLE_MAIN);
        todo!("{:?}", subject.read_general_registers()?);
        assert_eq!(SIMPLE_MAIN, counter);
        Ok(())
    }

    #[test]
    fn test_read_general_registers() -> Result {
        let (_trace, mut subject) = fixture()?;
        subject.cont(&ThreadId::All)?;
        assert_debug_snapshot!(subject.read_general_registers()?);
        Ok(())
    }

    #[test]
    fn test_read_registers() -> Result {
        let (_trace, mut subject) = fixture()?;
        subject.cont(&ThreadId::All)?;
        assert_debug_snapshot!(subject.read_registers()?);
        Ok(())
    }

    #[test]
    fn test_query_thread_ids() -> Result {
        let (_trace, mut subject) = fixture()?;
        assert_debug_snapshot!(subject.query_thread_ids()?);
        Ok(())
    }

    #[test]
    fn test_set_remove_breakpoint() -> Result {
        let (_trace, mut subject) = fixture()?;

        subject.break_at(SIMPLE_MAIN, 1)?;
        subject.remove_break_at(SIMPLE_MAIN, 1)?;

        Ok(())
    }

    #[test]
    fn test_cont() -> Result {
        let (_trace, mut subject) = fixture()?;

        subject.cont(&ThreadId::All)?;
        subject.cont(&ThreadId::All)?;

        Ok(())
    }
}
