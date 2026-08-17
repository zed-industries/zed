use super::*;

#[derive(serde::Serialize)]
pub struct ListCommitsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b GistsHandler<'octo>,
    #[serde(skip)]
    gist_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListCommitsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b GistsHandler<'octo>, gist_id: String) -> Self {
        Self {
            handler,
            gist_id,
            per_page: None,
            page: None,
        }
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
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::gists::GistCommit>> {
        let route = format!("/gists/{gist_id}/commits", gist_id = self.gist_id);
        self.handler.crab.get(route, Some(&self)).await
    }
}
