use http::StatusCode;

use crate::models::repos::RepoVariables;

use super::RepoHandler;

/// A client to GitHub's repository variables API.
///
/// Created with [`RepoHandler`].
#[derive(serde::Serialize)]
pub struct RepoVariablesHandler<'octo> {
    #[serde(skip)]
    handler: &'octo RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> RepoVariablesHandler<'octo> {
    pub(crate) fn new(repo: &'octo RepoHandler<'octo>) -> Self {
        Self {
            handler: repo,
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

    /// Lists all repository variables.
    /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
    /// OAuth app tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # use octocrab::models::repos::RepoVariables;
    /// # async fn run() -> octocrab::Result<RepoVariables> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let variables = octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .list()
    ///     .await?;
    ///
    /// # Ok(variables)
    /// # }
    /// ```
    pub async fn list(&self) -> crate::Result<RepoVariables> {
        let route = format!("/{}/actions/variables", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }

    /// Gets a specific variable in a repository.
    /// The authenticated user must have collaborator access to the repository to use this endpoint.
    /// OAuth app tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # use octocrab::models::repos::RepoVariable;
    /// # async fn run() -> octocrab::Result<RepoVariable> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let variable = octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .get("EMAIL")
    ///     .await?;
    ///
    /// # Ok(variable)
    /// # }
    pub async fn get(
        &self,
        variable_name: impl AsRef<str>,
    ) -> crate::Result<crate::models::repos::RepoVariable> {
        let route = format!(
            "/{}/actions/variables/{variable_name}",
            self.handler.repo,
            variable_name = variable_name.as_ref()
        );
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Creates a repository variable that you can reference in a GitHub Actions workflow.
    /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
    /// OAuth tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .create("EMAIL", "octocat@github.com")
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn create(&self, variable_name: &str, variable_value: &str) -> crate::Result<()> {
        let route = format!("/{}/actions/variables", self.handler.repo);
        let variable = serde_json::json!({ "name": variable_name, "value": variable_value });

        let resp = self.handler.crab._post(route, Some(&variable)).await?;

        let resp = crate::map_github_error(resp).await?;
        match resp.status() {
            StatusCode::CREATED => Ok(()),
            status_code => Err(crate::Error::Other {
                source: format!(
                    "Unexpected status code from create request: {}",
                    status_code.as_str()
                )
                .into(),
                backtrace: snafu::Backtrace::capture(),
            }),
        }
    }

    /// Updates a repository variable that you can reference in a GitHub Actions workflow.
    /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
    /// OAuth app tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .update("EMAIL", "octocat@github.com")
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn update(&self, variable_name: &str, variable_value: &str) -> crate::Result<()> {
        let route = format!(
            "/{}/actions/variables/{variable_name}",
            self.handler.repo,
            variable_name = variable_name
        );
        let body = serde_json::json!({ "value": variable_value });
        let resp = self.handler.crab._patch(route, Some(&body)).await?;
        let resp = crate::map_github_error(resp).await?;
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            status_code => Err(crate::Error::Other {
                source: format!(
                    "Unexpected status code from update request: {}",
                    status_code.as_str()
                )
                .into(),
                backtrace: snafu::Backtrace::capture(),
            }),
        }
    }

    /// Deletes a repository variable using the variable name.
    /// Authenticated users must have collaborator access to a repository to create, update, or read variables.
    /// OAuth tokens and personal access tokens (classic) need the repo scope to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let repo = octocrab.repos("owner", "repo")
    ///     .variables()
    ///     .delete("EMAIL")
    ///     .await?;
    ///
    /// # Ok(())
    /// # }
    pub async fn delete(&self, variable_name: impl AsRef<str>) -> crate::Result<()> {
        let route = format!(
            "/{}/actions/variables/{variable_name}",
            self.handler.repo,
            variable_name = variable_name.as_ref()
        );

        let resp = self.handler.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(resp).await?;
        Ok(())
    }
}
