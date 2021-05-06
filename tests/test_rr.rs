use server::rr::{replay_session, ReplaySession};

#[test]
fn create_default_session_flags() {
    let flags = replay_session::Flags::default();
}

#[test]
fn create_session() {
    let _subject = ReplaySession::create(
        "tests/sample_traces/hello_world_bin/".into(),
        replay_session::Flags::default(),
    );
}
