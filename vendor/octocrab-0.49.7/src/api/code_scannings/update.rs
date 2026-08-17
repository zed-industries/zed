use super::*;

#[derive(serde::Serialize)]
pub struct UpdateCodeScanningBuilder<'octo, 'a> {
    #[serde(skip)]
    handler: &'a CodeScanningHandler<'octo>,
    #[serde(skip)]
    number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<params::AlertState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissed_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dismissed_comment: Option<String>,
}

impl<'octo, 'a> UpdateCodeScanningBuilder<'octo, 'a> {
    pub(crate) fn new(handler: &'a CodeScanningHandler<'octo>, number: u64) -> Self {
        Self {
            handler,
            number,
            state: None,
            dismissed_reason: None,
            dismissed_comment: None,
        }
    }

    /// The title of the code scanning.
    pub fn state(mut self, state: impl Into<params::AlertState>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn dismissed_reason(mut self, dismissed_reason: impl Into<String>) -> Self {
        self.dismissed_reason = Some(dismissed_reason.into());
        self
    }

    pub fn dismissed_comment(mut self, dismissed_comment: impl Into<String>) -> Self {
        self.dismissed_comment = Some(dismissed_comment.into());
        self
    }

    pub async fn send(self) -> Result<models::code_scannings::CodeScanningAlert> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/alerts/{code_scanning}",
            owner = self.handler.owner,
            repo = self.handler.repo.as_ref().expect("Repository is required"),
            code_scanning = self.number,
        );

        self.handler.crab.patch(route, Some(&self)).await
    }
}
