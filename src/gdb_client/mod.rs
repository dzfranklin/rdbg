mod common;
mod parser;
mod serializer;

use std::{
    io::{self, BufRead, BufReader},
    net::{TcpStream, ToSocketAddrs},
};
use tracing::{debug, warn};

pub use self::parser::HaltReason;

pub struct GdbClient<R, W> {
    recv: R,
    tx: W,
}

impl GdbClient<BufReader<TcpStream>, TcpStream> {
    pub fn connect(addr: impl ToSocketAddrs) -> eyre::Result<Self> {
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

    pub fn query_halt_reason(&mut self) -> eyre::Result<HaltReason> {
        self.send_packet(common::Q_HALT_REASON)?;
        let body = self.read_packet()?;
        let (_rest, reason) = parser::halt_reason(&body)?;
        Ok(reason)
    }

    pub fn interrupt(&mut self) -> eyre::Result<()> {
        self.tx.write_all(&[common::INTERRUPT])?;
        Ok(())
    }

    fn read_packet(&mut self) -> eyre::Result<Vec<u8>> {
        let mut retries = 0;
        loop {
            match self.read_packet_no_retries() {
                Ok(body) => break Ok(body),
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

    fn read_packet_no_retries(&mut self) -> eyre::Result<Vec<u8>> {
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
                    debug!(len, cap, "Got packet");
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

    fn send_packet(&mut self, body: &[u8]) -> eyre::Result<()> {
        let packet = serializer::packet(body);
        self.tx.write_all(&packet)?;
        self.tx.flush()?;
        Ok(())
    }

    fn expect_ack(&mut self) -> eyre::Result<()> {
        let mut buf = vec![0];
        self.recv.read_exact(&mut buf)?;
        match *buf.get(0).unwrap() {
            common::ACK => Ok(()),
            common::NACK => Err(eyre::eyre!("Received NACK")),
            byte => Err(eyre::eyre!("Expected ACK, got {:?}", byte)),
        }
    }

    fn send_ack(&mut self) -> eyre::Result<()> {
        self.tx.write_all(&[common::ACK])?;
        Ok(())
    }

    fn send_nack(&mut self) -> eyre::Result<()> {
        self.tx.write_all(&[common::NACK])?;
        Ok(())
    }
}

const MAX_RETRIES: usize = 10;
const PACKET_SIZE_GUESS: usize = 512;
