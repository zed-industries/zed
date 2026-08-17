use super::*;
use crate::{models, Page, Result};

#[derive(serde::Serialize)]
pub struct ListTeamsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r TeamHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListTeamsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r TeamHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
        }
    }

    /// Results per page.
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
    pub async fn send(self) -> Result<Page<models::teams::RequestedTeam>> {
        let route = format!("/orgs/{owner}/teams", owner = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}
