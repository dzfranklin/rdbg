use camino::Utf8PathBuf;

use crate::Replay;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Trace {
    pub(crate) trace_dir: Utf8PathBuf,
    pub(crate) crate_name: String,
}

impl Trace {
    pub fn new(trace_dir: impl Into<Utf8PathBuf>, crate_name: impl Into<String>) -> Self {
        Self {
            trace_dir: trace_dir.into(),
            crate_name: crate_name.into(),
        }
    }

    pub async fn replay(self) -> eyre::Result<Replay> {
        Replay::spawn(self)
    }
}
