use super::*;

#[derive(serde::Serialize)]
pub struct GetContentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
}

impl<'octo, 'r> GetContentBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            path: None,
            r#ref: None,
        }
    }

    /// The content path.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// The name of the commit/branch/tag.
    /// Default: the repository’s default branch (usually `master)
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// Get the file content raw.
    pub fn raw(self) -> GetRawContentBuilder<'octo, 'r> {
        GetRawContentBuilder {
            content_builder: self,
        }
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::ContentItems> {
        let path = self.path.clone().unwrap_or(String::from(""));
        let route = format!("/{}/contents/{path}", self.handler.repo, path = path,);
        self.handler.crab.get(route, Some(&self)).await
    }
}

pub struct GetRawContentBuilder<'octo, 'r> {
    content_builder: GetContentBuilder<'octo, 'r>,
}

impl<'octo, 'r> GetRawContentBuilder<'octo, 'r> {
    /// The content path.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.content_builder = self.content_builder.path(path);
        self
    }

    /// The name of the commit/branch/tag.
    /// Default: the repository’s default branch (usually `master)
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.content_builder = self.content_builder.r#ref(r#ref);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<http::Response<BoxBody<Bytes, crate::Error>>> {
        let handler = self.content_builder.handler;

        let path = self
            .content_builder
            .path
            .clone()
            .unwrap_or(String::from(""));
        let route = format!("/{}/contents/{path}", handler.repo, path = path,);
        let uri = handler
            .crab
            .parameterized_uri(route, Some(&self.content_builder))?;

        let mut headers = http::header::HeaderMap::with_capacity(1);
        headers.insert(ACCEPT, "application/vnd.github.v3.raw".parse().unwrap());

        crate::map_github_error(handler.crab._get_with_headers(uri, Some(headers)).await?).await
    }
}

#[derive(serde::Serialize)]
pub struct GetReadmeBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#ref: Option<String>,
}

impl<'octo, 'r> GetReadmeBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            path: None,
            r#ref: None,
        }
    }

    /// The content path.
    /// Default: none (the repository's root directory)
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// The name of the commit/branch/tag.
    /// Default: the repository’s default branch (usually `master)
    pub fn r#ref(mut self, r#ref: impl Into<String>) -> Self {
        self.r#ref = Some(r#ref.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::Content> {
        let path = self.path.clone().unwrap_or(String::from(""));
        let route = format!("/{}/readme/{path}", self.handler.repo, path = path,);
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct UpdateFileBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: String,
    message: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commiter: Option<models::repos::CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<models::repos::CommitAuthor>,
}

impl<'octo, 'r> UpdateFileBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r RepoHandler<'octo>,
        path: String,
        message: String,
        content: String,
        sha: Option<String>,
    ) -> Self {
        Self {
            handler,
            path,
            message,
            content,
            sha,
            branch: None,
            commiter: None,
            author: None,
        }
    }

    /// The branch to commit to.
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// The person that committed the file.
    pub fn commiter(mut self, commiter: impl Into<models::repos::CommitAuthor>) -> Self {
        self.commiter = Some(commiter.into());
        self
    }

    /// The author of the file.
    pub fn author(mut self, author: impl Into<models::repos::CommitAuthor>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::FileUpdate> {
        let route = format!("/{}/contents/{path}", self.handler.repo, path = self.path,);
        self.handler.crab.put(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct DeleteFileBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip)]
    path: String,
    message: String,
    sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commiter: Option<models::repos::CommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<models::repos::CommitAuthor>,
}

impl<'octo, 'r> DeleteFileBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r RepoHandler<'octo>,
        path: String,
        message: String,
        sha: String,
    ) -> Self {
        Self {
            handler,
            path,
            message,
            sha,
            branch: None,
            commiter: None,
            author: None,
        }
    }

    /// The branch to commit to.
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// The person that committed the file.
    pub fn commiter(mut self, commiter: impl Into<models::repos::CommitAuthor>) -> Self {
        self.commiter = Some(commiter.into());
        self
    }

    /// The author of the file.
    pub fn author(mut self, author: impl Into<models::repos::CommitAuthor>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<models::repos::FileDeletion> {
        let route = format!("/{}/contents/{path}", self.handler.repo, path = self.path,);
        self.handler.crab.delete(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::repos::CommitAuthor;

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::instance();
        let repo = octocrab.repos("owner", "repo");
        let builder = repo
            .update_file(
                "tests/test.txt",
                "Update test.txt",
                "This is a test.",
                "testsha",
            )
            .branch("not-master")
            .commiter(CommitAuthor {
                name: "Octocat".to_string(),
                email: Some("octocat@github.com".to_string()),
                date: None,
            })
            .author(CommitAuthor {
                name: "Ferris".to_string(),
                email: Some("ferris@rust-lang.org".to_string()),
                date: None,
            });

        use base64::{engine::general_purpose, Engine as _};

        assert_eq!(
            serde_json::to_value(builder).unwrap(),
            serde_json::json!({
                "message": "Update test.txt",
                "content": general_purpose::STANDARD.encode("This is a test."),
                "sha": "testsha",
                "branch": "not-master",
                "commiter": {
                    "name": "Octocat",
                    "email": "octocat@github.com"
                },
                "author": {
                    "name": "Ferris",
                    "email": "ferris@rust-lang.org"
                }
            })
        )
    }

    #[tokio::test]
    async fn serialize_delete() {
        let octocrab = crate::instance();
        let repo = octocrab.repos("owner", "repo");
        let builder = repo
            .delete_file("tests/test.txt", "Update test.txt", "testsha")
            .branch("not-master")
            .commiter(CommitAuthor {
                name: "Octocat".to_string(),
                email: Some("octocat@github.com".to_string()),
                date: None,
            })
            .author(CommitAuthor {
                name: "Ferris".to_string(),
                email: Some("ferris@rust-lang.org".to_string()),
                date: None,
            });

        assert_eq!(
            serde_json::to_value(builder).unwrap(),
            serde_json::json!({
                "message": "Update test.txt",
                "sha": "testsha",
                "branch": "not-master",
                "commiter": {
                    "name": "Octocat",
                    "email": "octocat@github.com"
                },
                "author": {
                    "name": "Ferris",
                    "email": "ferris@rust-lang.org"
                }
            })
        )
    }
}
