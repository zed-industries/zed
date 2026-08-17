use super::*;
use crate::Page;

/// A builder pattern struct for listing installations.
///
/// created by [`AppsRequestHandler::installations`]
///
/// [`AppsRequestHandler::installations`]: ./struct.AppsRequestHandler.html#method.installations
#[derive(serde::Serialize)]
pub struct InstallationsRequestBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b AppsRequestHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> InstallationsRequestBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b AppsRequestHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            since: None,
        }
    }

    /// Only installations created at or after this time are returned.
    pub fn since(mut self, since: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<crate::models::Installation>> {
        let route = "/app/installations";
        self.handler.http_get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();

        let handler = octocrab.apps();
        let yesterday = chrono::Utc::now() - chrono::Duration::days(1);

        let list = handler
            .installations()
            .since(yesterday)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "since": yesterday,
                "per_page": 100,
                "page": 1
            })
        )
    }
}
