use server::rr::{replay_session, ReplaySession};

fn main() {
    let _subject = ReplaySession::create(
        "/home/daniel/rr/test_crate-a593336f3d14b759-7".into(),
        replay_session::Flags::default(),
    );
    println!("Hello, world!");
}
