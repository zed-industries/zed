use super::*;
use crate::models::TeamId;
use crate::params;

#[derive(serde::Serialize)]
pub struct CreateTeamBuilder<'octo, 'h, 'a, 'b> {
    #[serde(skip)]
    handler: &'h TeamHandler<'octo>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainers: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_names: Option<&'b [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<params::teams::Privacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<params::teams::Permission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_team_id: Option<TeamId>,
}

impl<'octo, 'h, 'a, 'b> CreateTeamBuilder<'octo, 'h, 'a, 'b> {
    pub(crate) fn new(handler: &'h TeamHandler<'octo>, name: String) -> Self {
        Self {
            handler,
            name,
            description: None,
            maintainers: None,
            repo_names: None,
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

    /// The organization members who will become team maintainers.
    pub fn maintainers(mut self, maintainers: &'a (impl AsRef<[String]> + ?Sized)) -> Self {
        self.maintainers = Some(maintainers.as_ref());
        self
    }

    /// The repositories to add the team to.
    ///
    /// Note: the repo name must be its full name, e.g. `"org/repo"`.
    pub fn repo_names(mut self, repo_names: &'b (impl AsRef<[String]> + ?Sized)) -> Self {
        self.repo_names = Some(repo_names.as_ref());
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
        let route = format!("/orgs/{org}/teams", org = self.handler.owner,);
        self.handler.crab.post(route, Some(&self)).await
    }
}
