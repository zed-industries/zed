use super::*;

#[derive(serde::Serialize)]
pub struct CompareCommitsBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::CommitHandler<'octo>,
    base: String,
    head: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'r> CompareCommitsBuilder<'octo, 'r> {
    pub(crate) fn new(
        handler: &'r super::CommitHandler<'octo>,
        base: String,
        head: String,
    ) -> Self {
        Self {
            handler,
            base,
            head,
            page: None,
            per_page: None,
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
    pub async fn send(self) -> crate::Result<models::commits::CommitComparison> {
        let route = format!(
            "/repos/{owner}/{repo}/compare/{base}...{head}",
            owner = self.handler.owner,
            repo = self.handler.repo,
            base = self.base,
            head = self.head,
        );

        self.handler.crab.get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn compare_commits_serializes_correctly() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.commits("owner", "repo");
        let comparison = handler.compare("base", "head");

        assert_eq!(
            serde_json::to_value(comparison).unwrap(),
            serde_json::json!({
                "base": "base",
                "head": "head",
            })
        );
    }
}
