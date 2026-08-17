use crate::commits::CommitHandler;
use crate::models::checks::ListCheckRuns;
use crate::params::repos::Reference;
use crate::Result;

#[derive(serde::Serialize)]
pub struct AssociatedCheckRunsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r CommitHandler<'octo>,
    #[serde(skip)]
    reference: Reference,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> AssociatedCheckRunsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r CommitHandler<'octo>, reference: impl Into<Reference>) -> Self {
        Self {
            handler,
            reference: reference.into(),
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
    pub async fn send(self) -> Result<ListCheckRuns> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{reference}/check-runs",
            owner = self.handler.owner,
            repo = self.handler.repo,
            reference = self.reference.full_ref_url()
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}
