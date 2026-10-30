use std::fmt;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone)]
pub struct ApiToken(String);

impl ApiToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub(crate) fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ApiToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ApiToken(<redacted>)")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiteMetadata {
    pub deployed_sha: String,
    pub commit: String,
    pub cdn_base: Url,
    pub read_only: bool,
    #[serde(default)]
    pub banner_message: Option<String>,
}
