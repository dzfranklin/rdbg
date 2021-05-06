use std::process::{self, Command, Stdio};

use crate::Trace;

#[derive(Debug)]
pub struct Session {
    proc: process::Child,
    stdin: process::ChildStdin,
    stdout: process::ChildStdout,
}

impl Session {
    pub fn new(trace: &Trace) -> eyre::Result<Self> {
        let mut proc = Command::new("rr")
            .args(&["replay", trace.as_str(), "--", "--interpreter=mi3"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = proc.stdin.take().expect("Stdin present");
        let stdout = proc.stdout.take().expect("Stdout present");

        Ok(Self {
            proc,
            stdin,
            stdout,
        })
    }
}
