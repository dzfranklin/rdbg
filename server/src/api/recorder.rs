use tonic::{transport, Request, Response, Status};

mod gen {
    tonic::include_proto!("org.danielzfranklin.rdbg.recorder");
    pub use recorder_server::*;
}

#[derive(Debug, Default)]
pub struct Server {}

#[tonic::async_trait]
impl gen::Recorder for Server {
    async fn record_test(
        &self,
        req: Request<gen::RecordTestRequest>,
    ) -> Result<Response<gen::RecordTestReply>, Status> {
        let req = req.into_inner();
        todo!()
    }
}
