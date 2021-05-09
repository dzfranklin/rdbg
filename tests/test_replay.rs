mod common;
use common::{init, trace_hello_world, trace_simple, Result};
use insta::assert_debug_snapshot;

use server::Replay;

#[test]
fn test_new_hello_world() -> Result {
    init();
    let trace = trace_hello_world();
    let _subject = Replay::new(&trace)?;
    Ok(())
}

#[test]
fn test_query_halt_reason() -> Result {
    init();
    let trace = trace_hello_world();
    let mut subject = Replay::new(&trace)?;
    assert_debug_snapshot!(subject.query_halt_reason()?);
    Ok(())
}
