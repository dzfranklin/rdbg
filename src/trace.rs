use camino::Utf8PathBuf;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Trace(Utf8PathBuf);

impl Trace {
    pub fn new(dir: impl Into<Utf8PathBuf>) -> Self {
        Self(dir.into())
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
