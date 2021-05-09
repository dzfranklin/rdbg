#![feature(format_args_capture, assert_matches)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::similar_names
)]

mod gdb_client;
pub mod replay;
#[cfg(test)]
pub(crate) mod test_common;
pub mod trace;

pub use replay::Replay;
pub use trace::Trace;
