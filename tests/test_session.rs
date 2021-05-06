mod common;
use common::{init, Result};

use server::{Session, Trace};

fn hello_world_trace() -> Trace {
    Trace::new("tests/samples/traces/hello_world")
}

#[test]
fn can_create_hello_world() -> Result {
    init();
    let trace = hello_world_trace();
    let _subject = Session::new(&trace)?;
    Ok(())
}
