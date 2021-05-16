use std::{
    collections::{BTreeMap, HashMap, HashSet},
    future::Future,
    stream::Stream,
};

use gdbmi::{
    address::Address,
    checkpoint::Checkpoint,
    frame::Frame,
    status::{Status, Stopped},
    variable::Variable,
    Gdb, GdbBuilder,
};
use serde::Serialize;
use tracing::{debug, error, info, warn};

use crate::Trace;

#[derive(Debug)]
pub struct Replay {
    gdb: Gdb,
    trace: Trace,
}

impl Replay {
    pub fn spawn(trace: Trace) -> eyre::Result<Self> {
        let gdb = GdbBuilder::rr(&trace.trace_dir).spawn()?;
        Ok(Self { gdb, trace })
    }

    async fn index<C, F>(&self, mut on_frame: C) -> eyre::Result<()>
    where
        C: FnMut(IndexFrame) -> F,
        F: Future<Output = ()>,
    {
        let gdb = &self.gdb;

        let crate_fns_pat = format!("^{}::", self.trace.crate_name);
        gdb.break_insert_re(&crate_fns_pat).await?;

        gdb.exec_run().await?;
        gdb.exec_continue().await?;

        loop {
            gdb.await_stopped(None).await?;
            if let Stopped::Exited(_) = self.gdb.await_stopped(None).await? {
                break;
            }

            let frame = gdb.stack_info_frame().await?;
            let vars = gdb.stack_info_variables(false).await?;
            let parents = gdb.stack_info_parents().await?;
            let checkpoint = gdb.save_checkpoint().await?;

            on_frame(IndexFrame {
                frame,
                vars,
                parents,
                checkpoint,
            })
            .await;

            gdb.exec_continue().await?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct IndexFrame {
    frame: Frame,
    vars: Vec<Variable>,
    parents: Vec<Frame>,
    checkpoint: Checkpoint,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{init, trace_simple, Result};

    async fn fixture() -> eyre::Result<Replay> {
        init();
        let trace = trace_simple();
        let subject = trace.replay().await?;
        Ok(subject)
    }

    #[tokio::test]
    async fn test_index() -> Result {
        let subject = fixture().await?;
        subject.index(async move |_| ()).await?;
        todo!()
    }
}
