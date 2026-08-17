use super::*;
use crate::params::teams::Permission;

#[derive(serde::Serialize)]
pub struct ListCollaboratorsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<Permission>,
}

impl<'octo, 'r> ListCollaboratorsBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            permission: None,
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

    /// Filter collaborators by the permissions they have on the repository.
    /// If not specified, all collaborators will be returned.
    /// Can be one of: pull, triage, push, maintain, admin
    pub fn permission(mut self, permission: Permission) -> Self {
        self.permission = Some(permission);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Collaborator>> {
        let route = format!("/{}/collaborators", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct GetCollaboratorPermissionBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    /// The handle for the GitHub user account.
    #[serde(skip)]
    username: String,
}

impl<'octo, 'r> GetCollaboratorPermissionBuilder<'octo, 'r> {
    pub fn new(handler: &'r RepoHandler<'octo>, username: impl Into<String>) -> Self {
        Self {
            handler,
            username: username.into(),
        }
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::repos::RepoPermission> {
        let route = format!(
            "/{}/collaborators/{}/permission",
            self.handler.repo, self.username
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}
