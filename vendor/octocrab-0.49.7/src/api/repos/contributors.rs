use super::*;

#[derive(serde::Serialize)]
pub struct ListContributorsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListContributorsBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            anon: None,
            per_page: None,
            page: None,
        }
    }

    /// Set to 1 or true to include anonymous contributors in results.
    pub fn anon(mut self, include_anon: impl Into<bool>) -> Self {
        self.anon = Some(include_anon.into());
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
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Contributor>> {
        let route = format!("/{}/contributors", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
