use super::*;

#[derive(serde::Serialize)]
pub struct CreateCommentBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::CommitHandler<'octo>,
    sha: String,
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u64>,
}

impl<'octo, 'r> CreateCommentBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r super::CommitHandler<'octo>, sha: String, body: String) -> Self {
        Self {
            handler,
            sha,
            body,
            path: None,
            position: None,
            line: None,
        }
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<models::commits::Comment> {
        let route = format!(
            "/repos/{owner}/{repo}/commits/{commit_sha}/comments",
            owner = self.handler.owner,
            repo = self.handler.repo,
            commit_sha = self.sha,
        );

        self.handler.crab.post(route, Some(&self)).await
    }

    /// Relative path of the file to comment on.
    ///
    /// Required if you provide position.
    ///
    /// For example, if you want to comment on a line in the file
    /// `lib/octocat.rb`, you would provide `lib/octocat.rb`.
    pub fn path<A: Into<String>>(mut self, path: impl Into<Option<A>>) -> Self {
        self.path = path.into().map(A::into);
        self
    }

    pub fn position(mut self, position: impl Into<Option<u64>>) -> Self {
        self.position = position.into();
        self
    }

    pub fn line(mut self, line: impl Into<Option<u64>>) -> Self {
        self.line = line.into();
        self
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.commits("owner", "repo");
        let list = handler
            .create_comment("95b3b039e71659a401ef39e86bab691ab6ce5fe5", "boo boo")
            .path("lib/octocat.rb")
            .position(10)
            .line(1);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "sha": "95b3b039e71659a401ef39e86bab691ab6ce5fe5",
                "body": "boo boo",
                "path": "lib/octocat.rb",
                "position": 10,
                "line": 1,
            })
        )
    }
}
