use std::{fs, path::Path};

use camino::Utf8PathBuf;
use memmap::Mmap;

#[derive(Debug)]
pub struct Trace {
    pub(crate) dir: Utf8PathBuf,
    pub(crate) bin: memmap::Mmap,
}

impl Trace {
    /// # Safety
    /// Bin must point to a file, and the file may not be modified once this
    /// function is called (the file is memory-mapped).
    pub unsafe fn new(dir: impl Into<Utf8PathBuf>, bin: impl AsRef<Path>) -> eyre::Result<Self> {
        let file = fs::File::open(bin)?;
        let bin = Mmap::map(&file)?;

        Ok(Self {
            dir: dir.into(),
            bin,
        })
    }
}
