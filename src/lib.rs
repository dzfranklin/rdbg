#![feature(format_args_capture, assert_matches)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::similar_names
)]

mod gdb_client;
pub mod replay;
pub mod trace;

pub use gdb_client::GdbClient;
pub use replay::Replay;
pub use trace::Trace;
