use crate::theme;
use tonic::{transport, Request, Response, Status};

mod gen {
    tonic::include_proto!("org.danielzfranklin.rdbg.theme");
}

#[derive(Debug, Default)]
pub struct Server {}

#[tonic::async_trait]
impl gen::theme_server::Theme for Server {
    async fn search(
        &self,
        req: Request<gen::SearchRequest>,
    ) -> Result<Response<gen::SearchReply>, Status> {
        let req = req.into_inner();
        let resp = theme::search(req)
            .await
            .map_err(|e| Status::unavailable(format!("{:?}", e)))?;
        Ok(Response::new(resp))
    }
}

pub use gen::{search_request, SearchReply, SearchRequest, ThemeMeta};
