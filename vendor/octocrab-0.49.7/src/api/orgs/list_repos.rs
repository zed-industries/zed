use super::*;

#[derive(serde::Serialize)]
pub struct ListReposBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b OrgHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<crate::params::repos::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<crate::params::repos::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<crate::params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListReposBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b OrgHandler<'octo>) -> Self {
        Self {
            handler,
            r#type: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// Filter repostories by their type
    pub fn repo_type(mut self, r#type: impl Into<Option<crate::params::repos::Type>>) -> Self {
        self.r#type = r#type.into();
        self
    }

    /// What to sort results by. Can be either `created`, `updated`, `pushed`,
    /// or `full_name`.
    pub fn sort(mut self, sort: impl Into<crate::params::repos::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either `Ascending` or `Descending`.
    /// Default: ascending when sort is `full_name`, otherwise descending.
    pub fn direction(mut self, direction: impl Into<crate::params::Direction>) -> Self {
        self.direction = Some(direction.into());
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
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Repository>> {
        let route = format!("/orgs/{owner}/repos", owner = self.handler.owner);
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.pulls("rust-lang", "rust");
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .head("master")
            .base("branch")
            .sort(crate::params::pulls::Sort::Popularity)
            .direction(crate::params::Direction::Ascending)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "head": "master",
                "base": "branch",
                "sort": "popularity",
                "direction": "asc",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
