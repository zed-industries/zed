use crate::{gists::GistsHandler, models::gists::Gist, Page, Result};
use serde;

#[derive(serde::Serialize)]
pub struct ListGistForksBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b GistsHandler<'octo>,
    #[serde(skip)]
    gist_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListGistForksBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b GistsHandler<'octo>, gist_id: String) -> Self {
        Self {
            handler,
            gist_id,
            per_page: None,
            page: None,
        }
    }

    /// Set the `per_page` query parameter on the builder.
    ///
    /// Controls the number of results to return per "page" of results.
    /// The maximum value is 100 results per page retrieved. Values larger than
    /// `100` are clamped to `100` by GitHub's API
    pub fn per_page(mut self, count: u8) -> Self {
        self.per_page = Some(count);
        self
    }

    /// Sets the `page` query parameter on the builder.
    ///
    /// Controls which page of the result set should be retrieved.
    /// All pages are retrieved if this is omitted.
    pub fn page(mut self, page_num: u32) -> Self {
        self.page = Some(page_num);
        self
    }

    /// Sends the actual request to GitHub's API
    pub async fn send(self) -> Result<Page<Gist>> {
        let route = format!("/gists/{gist_id}/forks", gist_id = self.gist_id);
        self.handler.crab.get(route, Some(&self)).await
    }
}
