use super::*;
use crate::{models::teams, Page, Result};

#[derive(serde::Serialize)]
pub struct ListTeamInvitationsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r TeamHandler<'octo>,
    #[serde(skip)]
    slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> ListTeamInvitationsBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r TeamHandler<'octo>, slug: String) -> Self {
        Self {
            handler,
            slug,
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
    pub async fn send(self) -> Result<Page<teams::TeamInvitation>> {
        let route = format!(
            "/orgs/{org}/teams/{team}/invitations",
            org = self.handler.owner,
            team = self.slug,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
