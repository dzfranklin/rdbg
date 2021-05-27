use eyre::eyre;
use serde::{Deserialize, Serialize};

pub use crate::api::theme::{search_request, SearchReply, SearchRequest, ThemeMeta};

pub async fn search(req: SearchRequest) -> eyre::Result<SearchReply> {
    const SEARCH_URL: &str = "https://open-vsx.org/api/-/search";

    let sort_by = serialize_sort_by(req.sort_by());
    let sort_order = serialize_sort_order(req.sort_order());
    let params = OpenVsxQueryRequest {
        category: "Themes",
        include_all_versions: false,
        offset: req.offset,
        query: req.query,
        size: req.page_size,
        sort_by,
        sort_order,
    };

    let resp = client()?.get(SEARCH_URL).query(&params).send().await?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await?;
        return Err(eyre!("Request failed with status {}: {}", status, text));
    }

    let resp = match resp.json::<OpenVsxQueryResult>().await? {
        OpenVsxQueryResult::Err { error } => {
            return Err(eyre!("Request returned error: {}", error))
        }
        OpenVsxQueryResult::Ok(reply) => reply,
    };

    let themes = resp
        .extensions
        .into_iter()
        .map(|api| ThemeMeta {
            download_url: api.files.download,
            icon_url: api.files.icon,
            name: api.name,
            namespace: api.namespace,
            version: api.version,
            rating: api.average_rating,
            downloads: api.download_count,
            display_name: api.display_name,
            description: api.description,
        })
        .collect();

    Ok(SearchReply {
        themes,
        offset: resp.offset,
        total_size: resp.total_size,
    })
}

static USER_AGENT: &str = concat!("rdbg-server", "/", env!("CARGO_PKG_VERSION"),);

fn client() -> eyre::Result<reqwest::Client> {
    Ok(reqwest::Client::builder().user_agent(USER_AGENT).build()?)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpenVsxQueryRequest {
    category: &'static str,
    include_all_versions: bool,
    offset: u32,
    query: String,
    size: u32,
    sort_by: &'static str,
    sort_order: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
enum OpenVsxQueryResult {
    Err { error: String },
    Ok(OpenVsxQueryReply),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenVsxQueryReply {
    offset: u32,
    total_size: u32,
    extensions: Vec<OpenVsxQueryItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenVsxQueryItem {
    url: String,
    files: OpenVsxQueryItemFiles,
    name: String,
    namespace: String,
    version: String,
    average_rating: Option<f32>,
    download_count: u32,
    display_name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct OpenVsxQueryItemFiles {
    download: String,
    icon: Option<String>,
}

fn serialize_sort_by(val: search_request::SortBy) -> &'static str {
    match val {
        search_request::SortBy::Relevance => "relevance",
        search_request::SortBy::Rating => "averageRating",
        search_request::SortBy::Downloads => "downloadCount",
        search_request::SortBy::Time => "timestamp",
    }
}

fn serialize_sort_order(val: search_request::SortOrder) -> &'static str {
    match val {
        search_request::SortOrder::Desc => "desc",
        search_request::SortOrder::Asc => "asc",
    }
}
