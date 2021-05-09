use std::{
    io::{self, BufRead, BufReader},
    net::{SocketAddr, TcpStream},
    process::{Command, Stdio},
    str::FromStr,
    thread,
};

use crate::{
    gdb_client::{GdbClient, StopReason},
    Trace,
};
use cymbal::ParsedDwarf;
use lazy_static::lazy_static;
use regex::Regex;
use tracing::{info, warn};

pub struct Replay<'d, R, W> {
    dwarf: ParsedDwarf<'d>,
    client: GdbClient<R, W>,
    _server: std::process::Child,
}

impl<'d> Replay<'d, io::BufReader<TcpStream>, TcpStream> {
    pub fn new(trace: &'d Trace) -> eyre::Result<Self> {
        let dwarf = cymbal::ParsedDwarf::new(&*trace.bin)?;

        let (server, addr) = spawn_server(trace)?;
        let client = GdbClient::connect(addr)?;

        Ok(Self {
            dwarf,
            client,
            _server: server,
        })
    }
}

impl<'d, R: BufRead, W: io::Write> Replay<'d, R, W> {
    pub fn map_crate_functions(&mut self) {
        let funcs = self
            .dwarf
            .functions()
            .iter()
            .filter(|f| {
                f.low_pc != 0 // TODO: Also check in crate
            })
            .collect::<Vec<_>>();
    }

    pub fn query_halt_reason(&mut self) -> eyre::Result<StopReason> {
        Ok(self.client.query_stop_reason()?)
    }
}

pub(crate) fn spawn_server(trace: &Trace) -> eyre::Result<(std::process::Child, SocketAddr)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
                .*'target \s+ extended-remote \s+ (.*?)'.*
            "
        )
        .unwrap();
    }

    let mut server = Command::new("rr")
        .args(&["replay", "-s", "0", trace.dir.as_str()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = BufReader::new(server.stdout.take().unwrap());
    let mut stderr = BufReader::new(server.stderr.take().unwrap());

    // Get addr
    let mut line = String::new();
    stderr.read_line(&mut line)?; // skip line 1
    stderr.read_line(&mut line)?;
    let addr = RE
        .captures(&line)
        .ok_or_else(|| eyre::eyre!("Failed to parse output of `rr replay`"))?
        .get(1)
        .unwrap()
        .as_str();
    let addr = SocketAddr::from_str(addr)?;

    watch_stdio(stdout, stderr);

    Ok((server, addr))
}

fn watch_stdio(
    mut stdout: BufReader<std::process::ChildStdout>,
    mut stderr: BufReader<std::process::ChildStderr>,
) {
    thread::spawn(move || loop {
        let mut line = String::new();
        match stderr.read_line(&mut line) {
            Ok(0) => break,
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Ok(_) => {
                warn!("replay server stderr: {}", &line);
            }
            Err(err) => {
                warn!("Failed to read replay server stderr: {}", err);
            }
        }
    });

    thread::spawn(move || loop {
        let mut line = String::new();
        match stdout.read_line(&mut line) {
            Ok(0) => break,
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Ok(_) => {
                info!("replay server stdout: {}", &line);
            }
            Err(err) => {
                warn!("Failed to read replay server stdout: {}", err);
            }
        }
    });
}
