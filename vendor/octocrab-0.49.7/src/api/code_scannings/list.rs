use super::*;
use crate::params::Direction;

#[derive(crate::Serialize)]
pub struct ListCodeScanningsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b CodeScanningHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_guid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<params::code_scannings::Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<params::code_scannings::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<params::code_scannings::Severity>,
}

impl<'octo, 'b> ListCodeScanningsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b CodeScanningHandler<'octo>) -> Self {
        Self {
            handler,
            tool_name: None,
            tool_guid: None,
            per_page: None,
            page: None,
            reference: None,
            direction: None,
            sort: None,
            state: None,
            severity: None,
        }
    }

    /// Filter pull requests by `state`.
    pub fn state(mut self, state: params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// What to sort results by. Can be either `created`, `updated`,
    /// `popularity` (comment count) or `long-running` (age, filtering by pulls
    /// updated in the last month).
    pub fn sort(mut self, sort: impl Into<params::code_scannings::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either ascending or descending.
    /// Default: descending when sort is `created` or sort is not specified,
    /// otherwise ascending sort.
    pub fn direction(mut self, direction: impl Into<params::Direction>) -> Self {
        self.direction = Some(direction.into());
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
    pub async fn send(
        self,
    ) -> crate::Result<crate::Page<models::code_scannings::CodeScanningAlert>> {
        let route = self
            .handler
            .repo
            .as_ref()
            .map(|r| {
                format!(
                    "/repos/{owner}/{repo}/code-scanning/alerts",
                    owner = self.handler.owner,
                    repo = r,
                )
            })
            .unwrap_or(format!(
                "/orgs/{owner}/code-scanning/alerts",
                owner = self.handler.owner,
            ));

        self.handler.crab.get(route, Some(&self)).await
    }
}
