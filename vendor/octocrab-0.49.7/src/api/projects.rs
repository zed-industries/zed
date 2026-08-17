//! The Projects API.
//!
//! # Notes
//! Users need an account with sufficient privileges to interact with projects.

#[allow(clippy::module_inception)]
mod projects;

use self::projects::{
    CreateOrgProjectsBuilder, CreateRepositoryProjectsBuilder, CreateUserProjectBuilder,
    DeleteProjectBuilder, GetProjectBuilder, ListOrgProjectsBuilder, ListRepositoryProjectsBuilder,
    ListUserProjectsBuilder, NotNamed, UpdateProjectBuilder,
};
use crate::Octocrab;
use serde::Serialize;

/// A struct to access GitHub's projects API.
///
/// Created with [`Octocrab::projects`].
pub struct ProjectHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ProjectHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Get a project by its id.    
    ///
    /// # Arguments
    ///
    /// * `project_id` - id of the project to fetch
    ///   
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let project_id: u32 = 1002604;
    /// let project = octocrab::instance()
    ///     .projects()
    ///     .get_project(project_id)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_project(&self, project_id: impl Into<u32>) -> GetProjectBuilder<'_, '_> {
        GetProjectBuilder::new(self, project_id.into())
    }

    /// Updates a project given its project id.
    ///   
    /// # Arguments
    ///
    /// * `project_id` - id of the project to update
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {  
    /// let body = serde_json::json!({ "name": "Week One Sprint", "state": "open" });
    /// let project_id: u32 = 1002604;
    /// let project = octocrab::instance()
    ///     .projects()
    ///     .update_project(project_id)
    ///     .body(&body)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_project<B>(&self, project_id: impl Into<u32>) -> UpdateProjectBuilder<'_, '_, B>
    where
        B: Serialize + ?Sized,
    {
        UpdateProjectBuilder::new(self, project_id.into())
    }

    /// Deletes a project board.
    ///
    /// # Arguments
    ///
    /// * `project_id` - id of the project to delete
    ///   
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let project_id: u32 = 1002604;
    /// let project = octocrab::instance()
    ///     .projects()
    ///     .delete_project(project_id)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_project(&self, project_id: impl Into<u32>) -> DeleteProjectBuilder<'_, '_> {
        DeleteProjectBuilder::new(self, project_id.into())
    }

    /// Creates a user project board given its name.
    ///
    /// # Arguments
    ///
    /// * `username` - account username
    ///   
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let username = "octocat";
    /// let description = "Project Overview";
    /// let project = octocrab::instance()
    ///     .projects()
    ///     .create_user_project(username)
    ///     .body(description)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_user_project(
        &self,
        username: impl Into<String>,
    ) -> CreateUserProjectBuilder<'_, '_> {
        CreateUserProjectBuilder::new(self, username.into())
    }

    /// List a user's projects the username of the user
    ///
    /// # Arguments
    ///
    /// * `username` - account unsername
    ///   
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let username = "octocat";
    /// let project = octocrab::instance()
    ///     .projects()
    ///     .list_user_projects(username)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_user_projects(
        &self,
        username: impl Into<String>,
    ) -> ListUserProjectsBuilder<'_, '_> {
        ListUserProjectsBuilder::new(self, username.into())
    }

    /// List the projects of an organization.
    ///
    /// # Notes
    /// Only users with sufficient privileges can list an organization's projects.
    ///
    /// # Arguments
    ///
    /// * `org` - name of the organization
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let org = "octocrab";
    /// let projects = octocrab::instance()
    ///     .projects()
    ///     .list_organization_projects(org)
    ///     .state("all")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_organization_projects(
        &self,
        org: impl Into<String>,
    ) -> ListOrgProjectsBuilder<'_, '_> {
        ListOrgProjectsBuilder::new(self, org.into())
    }

    /// Create an organization project board.
    ///
    /// # Arguments
    ///
    /// * `org` - organization name.
    /// * `name` - name of the project.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let org = "octocrab";
    /// let name = "Organization Roadmap";
    /// let project = octocrab::instance()
    ///     .projects()
    ///     .create_organization_project(org, name)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_organization_project(
        &self,
        org: impl Into<String>,
        name: impl Into<String>,
    ) -> CreateOrgProjectsBuilder<'_, '_> {
        CreateOrgProjectsBuilder::new(self, org.into(), name.into())
    }

    /// Creates a repository project board.
    ///
    /// # Arguments
    ///
    /// * `owner` - repository owner.
    /// * `repo` - repository name.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let owner = "octocat";
    /// let repo = "octocrab";
    /// let name = "My Project";
    /// let description = "Project Overview";
    /// let tags = octocrab::instance()
    ///     .projects()
    ///     .create_repository_project("owner", "repo")
    ///     .project_name(name)
    ///     .body(description)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_repository_project(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> CreateRepositoryProjectsBuilder<'_, '_, NotNamed> {
        CreateRepositoryProjectsBuilder::instantiate(self, owner.into(), repo.into())
    }

    /// Lists the projects in a repository.
    ///
    /// # Arguments
    ///
    /// * `owner` - repository owner.
    /// * `repo` - repository name.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let owner = "octocat";
    /// let repo = "octocrab";
    /// let tags = octocrab::instance()
    ///     .projects()
    ///     .list_repository_projects("owner", "repo")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_repository_projects(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> ListRepositoryProjectsBuilder<'_, '_> {
        ListRepositoryProjectsBuilder::new(self, owner.into(), repo.into())
    }
}
