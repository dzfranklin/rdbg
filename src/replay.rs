use std::{
    io::{self, BufRead, BufReader},
    net::TcpStream,
    process::{Command, Stdio},
    thread,
};

use crate::{gdb_client::HaltReason, GdbClient, Trace};
use lazy_static::lazy_static;
use regex::Regex;
use tracing::{info, warn};

pub struct Replay<R, W> {
    client: GdbClient<R, W>,
    _server: std::process::Child,
}

impl Replay<io::BufReader<TcpStream>, TcpStream> {
    pub fn new(trace: &Trace) -> eyre::Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                .*'target \s+ extended-remote \s+ (.*?)'.*
            "
            )
            .unwrap();
        }

        let mut server = Command::new("rr")
            .args(&["replay", "-s", "0", trace.as_str()])
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

        Self::watch_stdio(stderr, stdout);

        let client = GdbClient::connect(addr)?;

        Ok(Self {
            client,
            _server: server,
        })
    }

    pub fn query_halt_reason(&mut self) -> eyre::Result<HaltReason> {
        self.client.query_halt_reason()
    }

    fn watch_stdio(
        mut stderr: BufReader<std::process::ChildStderr>,
        mut stdout: BufReader<std::process::ChildStdout>,
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
}
