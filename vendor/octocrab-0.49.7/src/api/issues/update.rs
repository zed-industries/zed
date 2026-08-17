use super::*;

#[derive(serde::Serialize)]
pub struct UpdateIssueBuilder<'octo, 'a, 'b, 'c, 'd, 'e> {
    #[serde(skip)]
    handler: &'a IssueHandler<'octo>,
    #[serde(skip)]
    number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'b str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'c str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<&'d [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<models::IssueState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_reason: Option<models::issues::IssueStateReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<&'e [String]>,
}

impl<'octo, 'a, 'b, 'c, 'd, 'e> UpdateIssueBuilder<'octo, 'a, 'b, 'c, 'd, 'e> {
    pub(crate) fn new(handler: &'a IssueHandler<'octo>, number: u64) -> Self {
        Self {
            handler,
            number,
            title: None,
            body: None,
            assignees: None,
            state: None,
            state_reason: None,
            milestone: None,
            labels: None,
        }
    }

    /// The title of the issue.
    pub fn title(mut self, title: &'b (impl AsRef<str> + ?Sized)) -> Self {
        self.title = Some(title.as_ref());
        self
    }

    /// The body of the issue.
    pub fn body(mut self, body: &'c (impl AsRef<str> + ?Sized)) -> Self {
        self.body = Some(body.as_ref());
        self
    }

    /// The assignees of the issue.
    pub fn assignees(mut self, assignees: &'d (impl AsRef<[String]> + ?Sized)) -> Self {
        self.assignees = Some(assignees.as_ref());
        self
    }

    /// The state of the issue.
    pub fn state(mut self, state: impl Into<models::IssueState>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// The state reason of the issue.
    pub fn state_reason(
        mut self,
        state_reason: impl Into<models::issues::IssueStateReason>,
    ) -> Self {
        self.state_reason = Some(state_reason.into());
        self
    }

    /// The milestone of the issue.
    pub fn milestone(mut self, milestone: impl Into<u64>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    /// The labels of the issue.
    pub fn labels(mut self, labels: &'e (impl AsRef<[String]> + ?Sized)) -> Self {
        self.labels = Some(labels.as_ref());
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> Result<models::issues::Issue> {
        let route = format!(
            "/{repo}/issues/{issue}",
            repo = self.handler.repo,
            issue = self.number,
        );

        self.handler.crab.patch(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        let octocrab = crate::Octocrab::default();
        let handler = octocrab.issues("rust-lang", "rust");
        let assignees = &[String::from("ferris")];
        let labels = &[
            String::from("help wanted"),
            String::from("good first issue"),
        ];
        let update = handler
            .update(1234)
            .title("Updated title")
            .body("New body")
            .state(crate::models::IssueState::Closed)
            .state_reason(crate::models::issues::IssueStateReason::Completed)
            .milestone(1234u64)
            .assignees(assignees)
            .labels(labels);

        assert_eq!(
            serde_json::to_value(update).unwrap(),
            serde_json::json!({
                "title": "Updated title",
                "body": "New body",
                "state": "closed",
                "state_reason": "completed",
                "milestone": 1234,
                "assignees": ["ferris"],
                "labels": ["help wanted", "good first issue"],
            })
        )
    }
}
