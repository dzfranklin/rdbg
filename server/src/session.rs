use std::collections::HashMap;

use crate::{Replay, Trace};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{tcp, TcpStream},
};

#[derive(Debug)]
pub struct Session {
    trace: Trace,
    replays: HashMap<ReplayToken, Replay>,
    rx: BufReader<tcp::OwnedReadHalf>,
    tx: tcp::OwnedWriteHalf,
}

impl Session {
    pub fn new(trace: Trace, chan: TcpStream) -> Session {
        let (rx, tx) = chan.into_split();
        let rx = BufReader::new(rx);
        Self {
            trace,
            replays: HashMap::new(),
            rx,
            tx,
        }
    }

    pub async fn mainloop(&mut self) -> eyre::Result<()> {
        let mut buf = Vec::new();
        while self.rx.read_until(b'\n', &mut buf).await? > 0 {}

        Ok(())
    }

    pub async fn replay_create(&mut self) -> eyre::Result<ReplayToken> {
        let token = ReplayToken::generate();
        let replay = self.trace.clone().replay().await?;
        self.replays.insert(token, replay);
        Ok(token)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplayToken(u32);

impl ReplayToken {
    fn generate() -> Self {
        Self(rand::random())
    }
}
