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
    Trace::new("tests/samples/traces/hello_world")
}
