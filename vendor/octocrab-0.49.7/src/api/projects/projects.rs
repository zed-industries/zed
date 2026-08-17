//! A set of helper structs and implementations to manage projects

use std::marker::PhantomData;

use super::*;

/// Helper builder struct to get a project by its id.
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct GetProjectBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    project_id: u32,
}

impl<'octo, 'r> GetProjectBuilder<'octo, 'r> {
    pub fn new(handler: &'r ProjectHandler<'octo>, project_id: u32) -> Self {
        Self {
            handler,
            project_id,
        }
    }

    pub async fn send(self) -> crate::Result<crate::models::Project> {
        let route = format!("/projects/{project_id}", project_id = self.project_id);
        self.handler.crab.get(route, None::<&()>).await
    }
}

/// Helper builder struct to update a project by its id and body.
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct UpdateProjectBuilder<'octo, 'r, B>
where
    B: Serialize + ?Sized,
{
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    project_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'r B>,
}

impl<'octo, 'r, B> UpdateProjectBuilder<'octo, 'r, B>
where
    B: Serialize + ?Sized,
{
    pub fn new(handler: &'r ProjectHandler<'octo>, project_id: u32) -> Self {
        Self {
            handler,
            project_id,
            body: None,
        }
    }

    /// Set the body of the project.
    ///
    /// The input parameter `body` can specify the following keys:
    ///     - `name` (string) - name of the project
    ///     - `body` (string or null) - project description
    ///     - `state` (string) - `open` or `closed`
    ///     - `organization_permission` (string) - `read`, `write`, `admin`, `none`)
    ///     - `private` (boolean)
    pub fn body(mut self, body: &'r B) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::Result<crate::models::Project> {
        let route = format!("/projects/{project_id}", project_id = self.project_id);

        self.handler.crab.patch(route, Some(&self)).await
    }
}

/// Helper builder struct to delete a project by its id.
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct DeleteProjectBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    project_id: u32,
}

impl<'octo, 'r> DeleteProjectBuilder<'octo, 'r> {
    pub fn new(handler: &'r ProjectHandler<'octo>, project_id: u32) -> Self {
        Self {
            handler,
            project_id,
        }
    }

    pub async fn send(self) -> crate::Result<()> {
        let route = format!("/projects/{project_id}", project_id = self.project_id);

        crate::map_github_error(self.handler.crab._delete(route, None::<&()>).await?)
            .await
            .map(drop)
    }
}

/// Helper builder struct to create a user project given its name.
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct CreateUserProjectBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    name: String,
}
impl<'octo, 'r> CreateUserProjectBuilder<'octo, 'r> {
    pub fn new(handler: &'r ProjectHandler<'octo>, name: String) -> Self {
        Self {
            handler,
            name,
            body: None,
        }
    }

    // The description of the project
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub async fn send(self) -> crate::Result<crate::models::Project> {
        let route = "/user/projects";

        self.handler.crab.post(route, Some(&self.body)).await
    }
}

/// Helper builder struct to list user projects given the username of the user.
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct ListUserProjectsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    username: String,
}
impl<'octo, 'r> ListUserProjectsBuilder<'octo, 'r> {
    pub fn new(handler: &'r ProjectHandler<'octo>, username: String) -> Self {
        Self {
            handler,
            username,
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

    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Project>> {
        let route = format!("/users/{username}/projects", username = self.username);

        self.handler.crab.get(route, None::<&()>).await
    }
}

/// Helper builder struct to get a paged list of an organization's projects.
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct ListOrgProjectsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    org: String,
}

impl<'octo, 'r> ListOrgProjectsBuilder<'octo, 'r> {
    pub fn new(handler: &'r ProjectHandler<'octo>, org: String) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            state: None,
            org,
        }
    }

    // Indicates the state of the projects to return.
    //
    // * `state` - state of the project. Default is `open`
    //  can be one of `open`, `closed`, `all`
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
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

    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Project>> {
        let route = format!("/orgs/{org}/projects", org = self.org);
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// Helper builder struct to create an organization project.
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct CreateOrgProjectsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    name: String,
    org: String,
}

impl<'octo, 'r> CreateOrgProjectsBuilder<'octo, 'r> {
    pub fn new(handler: &'r ProjectHandler<'octo>, org: String, name: String) -> Self {
        Self {
            handler,
            body: None,
            name,
            org,
        }
    }

    /// Description of the project.
    pub fn body(mut self, description: impl Into<String>) -> Self {
        self.body = Some(description.into());
        self
    }

    pub async fn send(self) -> crate::Result<crate::models::Project> {
        let route = format!("/orgs/{org}/projects", org = self.org);
        self.handler.crab.post(route, Some(&self)).await
    }
}

/// Helper builder struct to get a paged list of repository projects
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct ListRepositoryProjectsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    owner: String,
    repo: String,
}

impl<'octo, 'r> ListRepositoryProjectsBuilder<'octo, 'r> {
    pub fn new(handler: &'r ProjectHandler<'octo>, owner: String, repo: String) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            owner,
            repo,
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

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Project>> {
        let route = format!(
            "/repos/{owner}/{repo}/projects",
            owner = self.owner,
            repo = self.repo
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

pub struct Named;
pub struct NotNamed;

/// Helper builder struct to get create a repository project
///
/// Used by [`Octocrab::projects`].
#[derive(serde::Serialize)]
pub struct CreateRepositoryProjectsBuilder<'octo, 'r, S> {
    #[serde(skip)]
    handler: &'r ProjectHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    owner: String,
    repo: String,
    project_name: Option<String>,
    _named: std::marker::PhantomData<S>,
}

impl<'octo, 'r> CreateRepositoryProjectsBuilder<'octo, 'r, NotNamed> {
    pub fn instantiate(handler: &'r ProjectHandler<'octo>, owner: String, repo: String) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            body: None,
            owner,
            repo,
            project_name: None,
            _named: PhantomData,
        }
    }

    pub fn project_name(
        self,
        name: impl Into<String>,
    ) -> CreateRepositoryProjectsBuilder<'octo, 'r, Named> {
        CreateRepositoryProjectsBuilder {
            handler: self.handler,
            per_page: self.per_page,
            page: self.page,
            body: self.body,
            owner: self.owner,
            repo: self.repo,
            project_name: Some(name.into()),
            _named: PhantomData,
        }
    }
}

impl<'octo, 'r> CreateRepositoryProjectsBuilder<'octo, 'r, Named> {
    pub fn new(repo_builder: CreateRepositoryProjectsBuilder<'octo, 'r, NotNamed>) -> Self {
        Self {
            handler: repo_builder.handler,
            per_page: None,
            page: None,
            body: None,
            owner: repo_builder.owner,
            repo: repo_builder.repo,
            project_name: repo_builder.project_name,
            _named: PhantomData,
        }
    }

    /// Description of the project.
    pub fn body(mut self, description: impl Into<String>) -> Self {
        self.body = Some(description.into());
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
    pub async fn send(self) -> crate::Result<crate::models::Project> {
        let route = format!(
            "/repos/{owner}/{repo}/projects",
            owner = self.owner,
            repo = self.repo
        );
        self.handler.crab.post(route, Some(&self)).await
    }
}
