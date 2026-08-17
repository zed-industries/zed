use super::*;

#[derive(serde::Serialize)]
pub struct ListLabelsForIssueBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip)]
    number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListLabelsForIssueBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>, number: u64) -> Self {
        Self {
            handler,
            number,
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

    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<models::Label>> {
        let route = format!(
            "/{}/issues/{number}/labels",
            self.handler.repo,
            number = self.number,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListLabelsForRepoBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r IssueHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListLabelsForRepoBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r IssueHandler<'octo>) -> Self {
        Self {
            handler,
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

    /// Send the actual request.
    pub async fn send(self) -> Result<crate::Page<models::Label>> {
        let route = format!("/{}/labels", self.handler.repo);

        self.handler.crab.get(route, Some(&self)).await
    }
}
