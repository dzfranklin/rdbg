use std::sync::Once;

use server::Trace;

pub type Result = eyre::Result<()>;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .pretty()
            .init();

        color_eyre::install().unwrap();
    });
}

pub fn trace_hello_world() -> Trace {
    let dir = camino::Utf8PathBuf::from("tests/samples/traces/hello_world");
    let bin = dir.join("mmap_hardlink_4_hello_world");
    // Safety: Bin must point to a file which must never be modified
    //   The file is in our source directory, and we never modify it.
    //   Another program modifying it is considered out of scope.
    unsafe { Trace::new(dir, bin) }.unwrap()
}

pub fn trace_simple() -> Trace {
    let dir = camino::Utf8PathBuf::from("tests/samples/traces/simple");
    let bin = dir.join("mmap_hardlink_4_blog");
    // Safety: Bin must point to a file which must never be modified
    //   The file is in our source directory, and we never modify it.
    //   Another program modifying it is considered out of scope.
    unsafe { Trace::new(dir, bin) }.unwrap()
}
