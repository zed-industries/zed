use super::*;

#[derive(serde::Serialize)]
pub struct ListIssuesBuilder<'octo, 'b, 'c, 'd> {
    #[serde(skip)]
    handler: &'b IssueHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<params::issues::Filter<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee: Option<params::issues::Filter<&'c str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentioned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "comma_separated")]
    labels: Option<&'d [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<params::issues::Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<params::Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b, 'c, 'd> ListIssuesBuilder<'octo, 'b, 'c, 'd> {
    pub(crate) fn new(handler: &'b IssueHandler<'octo>) -> Self {
        Self {
            handler,
            state: None,
            milestone: None,
            assignee: None,
            creator: None,
            mentioned: None,
            labels: None,
            sort: None,
            direction: None,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// If an integer is passed, it should refer to a milestone by its number
    /// field. If the string `"*"` is passed, issues with any milestone are
    /// accepted. If the string none is passed, issues without milestones
    /// are returned.
    pub fn milestone(mut self, milestone: impl Into<params::issues::Filter<u64>>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    /// Filter by assignee, can be the name of a user. Pass in the string
    /// `"none"` for issues with no assigned user, and `"*"` for issues assigned
    /// to any user.
    pub fn assignee(mut self, assignee: impl Into<params::issues::Filter<&'c str>>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// Filter by the creator of the issue.
    pub fn creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    /// Filter by the creator of the issue.
    pub fn mentioned(mut self, mentioned: impl Into<String>) -> Self {
        self.mentioned = Some(mentioned.into());
        self
    }

    /// Filter pull requests by `state`.
    pub fn state(mut self, state: params::State) -> Self {
        self.state = Some(state);
        self
    }

    /// Filter issues by label.
    pub fn labels(mut self, labels: &'d (impl AsRef<[String]> + ?Sized)) -> Self {
        self.labels = Some(labels.as_ref());
        self
    }

    /// What to sort results by. Can be either `created`, `updated`,
    /// `popularity` (comment count) or `long-running` (age, filtering by pulls
    /// updated in the last month).
    pub fn sort(mut self, sort: impl Into<params::issues::Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// The direction of the sort. Can be either ascending or descending.
    /// Default: descending when sort is `created` or sort is not specified,
    /// otherwise ascending sort.
    pub fn direction(mut self, direction: impl Into<params::Direction>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Only return issues updated after the given timestamp.
    pub fn since(mut self, since: impl Into<chrono::DateTime<chrono::Utc>>) -> Self {
        self.since = Some(since.into());
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
    pub async fn send(self) -> crate::Result<crate::Page<models::issues::Issue>> {
        let route = format!("/{}/issues", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}

fn comma_separated<S: serde::Serializer>(
    labels: &Option<&[String]>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&labels.unwrap().join(","))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("rust-lang", "rust");
        let labels = vec![
            String::from("help wanted"),
            String::from("good first issue"),
        ];
        let list = handler
            .list()
            .state(crate::params::State::Open)
            .milestone(1234)
            .assignee("ferris")
            .creator("octocrab")
            .mentioned("octocat")
            .labels(&labels)
            .sort(crate::params::issues::Sort::Comments)
            .direction(crate::params::Direction::Ascending)
            .since(chrono::DateTime::parse_from_rfc3339("2003-07-01T10:52:37Z").unwrap())
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "state": "open",
                "milestone": 1234,
                "assignee": "ferris",
                "creator": "octocrab",
                "mentioned": "octocat",
                "labels": "help wanted,good first issue",
                "sort": "comments",
                "direction": "asc",
                "since": "2003-07-01T10:52:37Z",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
