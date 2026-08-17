use super::*;
use crate::models::TeamId;
use crate::params;

#[derive(serde::Serialize)]
pub struct EditTeamBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r TeamHandler<'octo>,
    #[serde(skip)]
    slug: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<params::teams::Privacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<params::teams::Permission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_team_id: Option<TeamId>,
}

impl<'octo, 'r> EditTeamBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r TeamHandler<'octo>, slug: String, name: String) -> Self {
        Self {
            handler,
            slug,
            name,
            description: None,
            privacy: None,
            permission: None,
            parent_team_id: None,
        }
    }

    /// The description of the team.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The level of privacy this team should have.
    ///
    /// For parents or child teams, only `Privacy::Closed` is valid.
    pub fn privacy(mut self, privacy: impl Into<params::teams::Privacy>) -> Self {
        self.privacy = Some(privacy.into());
        self
    }

    /// The ID of the team to set as the parent team.
    pub fn parent_team_id(mut self, parent_team_id: TeamId) -> Self {
        self.parent_team_id = Some(parent_team_id);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::teams::Team> {
        let route = format!(
            "/orgs/{org}/teams/{team}",
            org = self.handler.owner,
            team = self.slug,
        );
        self.handler.crab.patch(route, Some(&self)).await
    }
}
