use crate::error::HttpSnafu;
use crate::{repos::RepoHandler, Error};
use http::request::Builder;
use http::Uri;
use snafu::ResultExt;

#[derive(serde::Serialize)]
pub struct GenerateRepositoryBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all_branches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
}

impl<'octo, 'r> GenerateRepositoryBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>, name: impl Into<String>) -> Self {
        Self {
            handler,
            name: name.into(),
            owner: None,
            description: None,
            include_all_branches: None,
            private: None,
        }
    }

    /// New owner of the newly created repository from selected template.
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// Description of the newly created repository.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Whether to include all branches from template repository.
    pub fn include_all_branches(mut self, include_all_branches: impl Into<bool>) -> Self {
        self.include_all_branches = Some(include_all_branches.into());
        self
    }

    /// Whether to set newly created repository to private .
    pub fn private(mut self, private: impl Into<bool>) -> Self {
        self.private = Some(private.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<(), Error> {
        let route = format!("/{}/generate", self.handler.repo);
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        let request = Builder::new().uri(uri).method(http::Method::POST).header(
            http::header::ACCEPT,
            "application/vnd.github.baptiste-preview+json",
        );

        let request = self.handler.crab.build_request(request, Some(&self))?;

        let response = self.handler.crab.execute(request).await?;
        crate::map_github_error(response).await.map(drop)
    }
}
