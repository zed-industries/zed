use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiteMetadata {
    pub deployed_sha: String,
    pub commit: String,
    pub cdn_base: Url,
    pub read_only: bool,
    #[serde(default)]
    pub banner_message: Option<String>,
}
