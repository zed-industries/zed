//! The Teams API

mod children;
mod create;
mod edit;
mod invitations;
mod list;
mod members;
mod team_repos;

pub use self::{
    children::ListChildTeamsBuilder, create::CreateTeamBuilder, edit::EditTeamBuilder,
    invitations::ListTeamInvitationsBuilder, list::ListTeamsBuilder,
    members::ListTeamMembersBuilder, team_repos::TeamRepoHandler,
};
use http::Uri;
use snafu::ResultExt;

use crate::error::HttpSnafu;
use crate::{models, Octocrab, Result};

/// Handler for GitHub's teams API.
///
/// Created with [`Octocrab::teams`].
pub struct TeamHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
}

impl<'octo> TeamHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String) -> Self {
        Self { crab, owner }
    }

    /// Lists teams in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let teams = octocrab::instance()
    ///     .teams("owner")
    ///     .list()
    ///     .per_page(10)
    ///     .page(1u8)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListTeamsBuilder<'_, '_> {
        ListTeamsBuilder::new(self)
    }

    /// Gets a team from its slug.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let team = octocrab::instance()
    ///     .teams("owner")
    ///     .get("team")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, team_slug: impl Into<String>) -> Result<models::teams::Team> {
        let route = format!(
            "/orgs/{org}/teams/{team}",
            org = self.owner,
            team = team_slug.into(),
        );
        self.crab.get(route, None::<&()>).await
    }

    /// Creates a new team in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .create("new-team")
    ///     .description("My team created from Octocrab!")
    ///     .maintainers(&vec![String::from("ferris")])
    ///     .repo_names(&vec![String::from("crab-stuff")])
    ///     .privacy(params::teams::Privacy::Closed)
    ///     .parent_team_id(1u64.into())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, name: impl Into<String>) -> CreateTeamBuilder<'_, '_, '_, '_> {
        CreateTeamBuilder::new(self, name.into())
    }

    /// Creates a new team in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .edit("some-team", "Some Team")
    ///     .description("I edited from Octocrab!")
    ///     .privacy(params::teams::Privacy::Secret)
    ///     .parent_team_id(2u64.into())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit(
        &self,
        team_slug: impl Into<String>,
        name: impl Into<String>,
    ) -> EditTeamBuilder<'_, '_> {
        EditTeamBuilder::new(self, team_slug.into(), name.into())
    }

    /// Deletes a team from the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance().teams("owner").delete("some-team").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, team_slug: impl Into<String>) -> Result<()> {
        let route = format!(
            "/orgs/{org}/teams/{team}",
            org = self.owner,
            team = team_slug.into(),
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._delete(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// List the child teams of a team in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .list_children("parent-team")
    ///     .per_page(5)
    ///     .page(1u8)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_children(&self, team_slug: impl Into<String>) -> ListChildTeamsBuilder<'_, '_> {
        ListChildTeamsBuilder::new(self, team_slug.into())
    }

    /// Creates a new `TeamRepoHandler` for the specified team,
    /// that allows you to manage this team's repositories.
    pub fn repos(&self, team_slug: impl Into<String>) -> TeamRepoHandler<'_> {
        TeamRepoHandler::new(self.crab, self.owner.clone(), team_slug.into())
    }

    /// List the members of a team in the organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .members("team-name-here")
    ///     .per_page(5)
    ///     .page(1u8)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn members(&self, team_slug: impl Into<String>) -> ListTeamMembersBuilder<'_, '_> {
        ListTeamMembersBuilder::new(self, team_slug.into())
    }

    /// List the pending invitations for a team in an organization.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab::instance()
    ///     .teams("owner")
    ///     .invitations("team-name-here")
    ///     .per_page(5)
    ///     .page(1u8)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn invitations(&self, team_slug: impl Into<String>) -> ListTeamInvitationsBuilder<'_, '_> {
        ListTeamInvitationsBuilder::new(self, team_slug.into())
    }
}
