use crate::models::{CheckSuiteId, RunId};
use crate::{models, Octocrab, Page, Result};

pub struct WorkflowsHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
}

/// Handler for GitHub's workflows API for actions.
///
/// Created with [`Octocrab::workflows`].
impl<'octo> WorkflowsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String) -> Self {
        Self { crab, owner, repo }
    }

    /// List workflow definitions in the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    ///
    /// let issue = octocrab.workflows("owner", "repo")
    ///     .list()
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(1u8)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListWorkflowsBuilder<'_, '_> {
        ListWorkflowsBuilder::new(self)
    }

    pub async fn get(&self, run_id: RunId) -> Result<models::workflows::Run> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}",
            owner = self.owner,
            repo = self.repo,
            run_id = run_id,
        );

        self.crab.get(route, None::<&()>).await
    }

    /// List runs in the specified workflow.
    /// workflow_file_or_id can be either file name or numeric expression.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    ///
    /// let issue = octocrab.workflows("owner", "repo")
    ///     .list_runs("ci.yml")
    ///     // Optional Parameters
    ///     .actor("octocat")
    ///     .branch("master")
    ///     .event("push")
    ///     .status("success")
    ///     .per_page(100)
    ///     .page(1u8)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_runs(&self, workflow_file_or_id: impl Into<String>) -> ListRunsBuilder<'_, '_> {
        ListRunsBuilder::new(
            self,
            ListRunsRequestType::ByWorkflow(workflow_file_or_id.into()),
        )
    }

    /// List runs for the specified owner and repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let octocrab = octocrab::Octocrab::default();
    ///
    /// let runs = octocrab.workflows("owner", "repo")
    ///     .list_all_runs()
    ///     // Optional Parameters
    ///     .actor("octocat")
    ///     .branch("master")
    ///     .event("pull_request")
    ///     .status("success")
    ///     .per_page(100)
    ///     .page(1u8)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_all_runs(&self) -> ListRunsBuilder<'_, '_> {
        ListRunsBuilder::new(self, ListRunsRequestType::ByRepo)
    }

    /// List job results in the specified run.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params::workflows::Filter;
    ///
    /// let issue = octocrab.workflows("owner", "repo")
    ///     .list_jobs(1234u64.into())
    ///     // Optional Parameters
    ///     .per_page(100)
    ///     .page(1u8)
    ///     .filter(Filter::All)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_jobs(&self, run_id: RunId) -> ListJobsBuilder<'_, '_> {
        ListJobsBuilder::new(self, run_id)
    }
}

#[derive(serde::Serialize)]
pub struct ListWorkflowsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b WorkflowsHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListWorkflowsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b WorkflowsHandler<'octo>) -> Self {
        Self {
            handler,
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

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::workflows::WorkFlow>> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows",
            owner = self.handler.owner,
            repo = self.handler.repo
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

/// The type of list workflow runs request.
pub(crate) enum ListRunsRequestType {
    ByRepo,
    ByWorkflow(String),
}

#[derive(serde::Serialize)]
pub struct ListRunsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b WorkflowsHandler<'octo>,
    #[serde(skip)]
    r#type: ListRunsRequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_pull_requests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_suite_id: Option<CheckSuiteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_sha: Option<String>,
}

impl<'octo, 'b> ListRunsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b WorkflowsHandler<'octo>, r#type: ListRunsRequestType) -> Self {
        Self {
            handler,
            r#type,
            actor: None,
            branch: None,
            event: None,
            status: None,
            per_page: None,
            page: None,
            exclude_pull_requests: None,
            check_suite_id: None,
            head_sha: None,
        }
    }

    /// Someone who runs workflows. Use the login to specify a user.
    pub fn actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }

    /// A branch associated with workflows. Use the name of the branch of the push.
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// An event associated with workflows. Can be e.g. push, pull_request, issue,
    /// ... and many variations. See official "Events that trigger workflows." doc.
    pub fn event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    /// A status associated with workflows.
    /// status or conclusion can be specified. e.g. success, in_progress, waiting...
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
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

    /// Whether to exclude the pull requests or not.
    pub fn exclude_pull_requests(mut self, exclude_pull_requests: impl Into<bool>) -> Self {
        self.exclude_pull_requests = Some(exclude_pull_requests.into());
        self
    }

    /// Returns workflow runs with the check_suite_id that you specify
    pub fn check_suite_id(mut self, check_suite_id: impl Into<CheckSuiteId>) -> Self {
        self.check_suite_id = Some(check_suite_id.into());
        self
    }

    /// Only returns workflow runs that are associated with the specified head_sha.
    pub fn head_sha(mut self, head_sha: impl Into<String>) -> Self {
        self.head_sha = Some(head_sha.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::workflows::Run>> {
        let route = match self.r#type {
            ListRunsRequestType::ByRepo => format!(
                "/repos/{owner}/{repo}/actions/runs",
                owner = self.handler.owner,
                repo = self.handler.repo
            ),
            ListRunsRequestType::ByWorkflow(ref workflow_id) => format!(
                "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs",
                owner = self.handler.owner,
                repo = self.handler.repo,
                workflow_id = workflow_id
            ),
        };
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListJobsBuilder<'octo, 'b> {
    #[serde(skip)]
    handler: &'b WorkflowsHandler<'octo>,
    #[serde(skip)]
    run_id: RunId,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<crate::params::workflows::Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo, 'b> ListJobsBuilder<'octo, 'b> {
    pub(crate) fn new(handler: &'b WorkflowsHandler<'octo>, run_id: RunId) -> Self {
        Self {
            handler,
            run_id,
            per_page: None,
            page: None,
            filter: None,
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

    /// Filters jobs by their completed_at timestamp. Choose latest or all.
    pub fn filter(mut self, filter: impl Into<crate::params::workflows::Filter>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> Result<Page<models::workflows::Job>> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/jobs",
            owner = self.handler.owner,
            repo = self.handler.repo,
            run_id = self.run_id,
        );
        self.handler.crab.get(route, Some(&self)).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn serialize() {
        use crate::params::workflows::Filter;

        let octocrab = crate::Octocrab::default();
        let handler = octocrab.workflows("rust-lang", "rust");
        let list_jobs = handler
            .list_jobs(1234u64.into())
            .filter(Filter::All)
            .per_page(100)
            .page(1u8);

        assert_eq!(
            serde_json::to_value(list_jobs).unwrap(),
            serde_json::json!({
                "filter": "all",
                "per_page": 100,
                "page": 1,
            })
        )
    }
}
