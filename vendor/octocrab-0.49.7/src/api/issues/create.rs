use super::*;

#[derive(serde::Serialize)]
pub struct CreateIssueBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r super::IssueHandler<'octo>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
}

impl<'octo, 'r> CreateIssueBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r super::IssueHandler<'octo>, title: String) -> Self {
        Self {
            handler,
            title,
            body: None,
            milestone: None,
            labels: None,
            assignees: None,
        }
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<models::issues::Issue> {
        let route = format!("/{}/issues", self.handler.repo);

        self.handler.crab.post(route, Some(&self)).await
    }

    /// The contents of the issue.
    pub fn body<A: Into<String>>(mut self, body: impl Into<Option<A>>) -> Self {
        self.body = body.into().map(A::into);
        self
    }

    /// The number of the milestone to associate this issue with. *NOTE: Only
    /// users with push access can set the milestone for new issues. The
    /// milestone is silently dropped otherwise.*
    pub fn milestone(mut self, milestone: impl Into<Option<u64>>) -> Self {
        self.milestone = milestone.into();
        self
    }

    /// Labels to associate with this issue. *NOTE: Only users with push access
    /// can set labels for new issues. Labels are silently dropped otherwise.*
    pub fn labels(mut self, labels: impl Into<Option<Vec<String>>>) -> Self {
        self.labels = labels.into();
        self
    }

    /// Logins for Users to assign to this issue. *NOTE: Only users with push
    /// access can set assignees for new issues. Assignees are silently
    /// dropped otherwise.*
    pub fn assignees(mut self, assignees: impl Into<Option<Vec<String>>>) -> Self {
        self.assignees = assignees.into();
        self
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("owner", "repo");
        let list = handler
            .create("test-issue")
            .body(String::from("testing..."))
            .milestone(3456)
            .labels(vec![String::from("help-wanted")])
            .assignees(vec![String::from("octocrab"), String::from("ferris")]);

        assert_eq!(
            serde_json::to_value(list).unwrap(),
            serde_json::json!({
                "title": "test-issue",
                "body": "testing...",
                "milestone": 3456,
                "labels": ["help-wanted"],
                "assignees": ["octocrab", "ferris"],
            })
        )
    }
}
