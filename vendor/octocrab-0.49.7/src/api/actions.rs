//! GitHub Actions
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Collected};
use snafu::ResultExt;

pub mod self_hosted_runners;

use self::self_hosted_runners::{CreateJitRunnerConfigBuilder, ListSelfHostedRunnersBuilder};
use crate::error::HttpSnafu;
use crate::etag::{EntityTag, Etagged};
use crate::models::{
    workflows::WorkflowDispatch, workflows::WorkflowListArtifact, ArtifactId, RepositoryId, RunId,
};
use crate::models::{RunnerGroupId, RunnerId};
use crate::{params, FromResponse, Octocrab, Page};
use http::request::Builder;
use http::{header::HeaderMap, Method, StatusCode, Uri};

pub struct ListWorkflowRunArtifacts<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
    run_id: RunId,
    per_page: Option<u8>,
    page: Option<u32>,
    etag: Option<EntityTag>,
}

impl<'octo> ListWorkflowRunArtifacts<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: String, run_id: RunId) -> Self {
        Self {
            crab,
            owner,
            repo,
            run_id,
            per_page: None,
            page: None,
            etag: None,
        }
    }

    /// Etag for this request.
    pub fn etag(mut self, etag: Option<EntityTag>) -> Self {
        self.etag = etag;
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

    pub async fn send(self) -> crate::Result<Etagged<Page<WorkflowListArtifact>>> {
        let path = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts",
            owner = self.owner,
            repo = self.repo,
            run_id = self.run_id
        );
        let uri = Uri::builder()
            .path_and_query(path)
            .build()
            .context(HttpSnafu)?;
        let mut headers = HeaderMap::new();
        if let Some(etag) = self.etag {
            EntityTag::insert_if_none_match_header(&mut headers, etag)?;
        }

        let request = self
            .crab
            .build_request(Builder::new().method(Method::GET).uri(uri), None::<&()>)?;
        let response = self.crab.execute(request).await?;
        let etag = EntityTag::extract_from_response(&response);
        if response.status() == StatusCode::NOT_MODIFIED {
            Ok(Etagged { etag, value: None })
        } else {
            <Page<WorkflowListArtifact>>::from_response(crate::map_github_error(response).await?)
                .await
                .map(|page| Etagged {
                    etag,
                    value: Some(page),
                })
        }
    }
}

pub struct WorkflowDispatchBuilder<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: String,
    workflow_id: String,
    data: WorkflowDispatch,
}

impl<'octo> WorkflowDispatchBuilder<'octo> {
    pub(crate) fn new(
        crab: &'octo Octocrab,
        owner: String,
        repo: String,
        workflow_id: String,
        r#ref: String,
    ) -> Self {
        let mut this = Self {
            crab,
            owner,
            repo,
            workflow_id,
            data: Default::default(),
        };
        this.data.r#ref = r#ref;
        this
    }

    /// Input keys and values configured in the workflow file. The maximum number of properties is 10.
    /// Any default properties configured in the workflow file will be used when inputs are omitted.
    ///
    /// # Panics
    /// If `inputs` is not `Value::Object`.
    pub fn inputs(mut self, inputs: serde_json::Value) -> Self {
        assert!(inputs.is_object(), "Inputs should be a JSON object");
        self.data.inputs = inputs;
        self
    }

    pub async fn send(self) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches",
            owner = self.owner,
            repo = self.repo,
            workflow_id = self.workflow_id
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        let response = self.crab._post(uri, Some(&self.data)).await?;
        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }
}

/// Handler for GitHub's actions API.
///
/// Created with [`Octocrab::actions`].
pub struct ActionsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ActionsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Adds a repository to an organization secret when the visibility for
    /// repository access is set to selected. The visibility is set when you
    /// create or update an organization secret. You must authenticate using an
    /// access token with the admin:org scope to use this endpoint. GitHub Apps
    /// must have the secrets organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .add_selected_repo_to_org_secret("org", "secret_name", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_selected_repo_to_org_secret(
        &self,
        org: impl AsRef<str>,
        secret_name: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",
            org = org.as_ref(),
            secret_name = secret_name.as_ref(),
            repository_id = repository_id,
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._put(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Removes a repository from an organization secret when the visibility for
    /// repository access is set to selected. The visibility is set when you
    /// create or update an organization secret. You must authenticate using an
    /// access token with the admin:org scope to use this endpoint. GitHub Apps
    /// must have the secrets organization permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .remove_selected_repo_from_org_secret("org", "secret_name", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_selected_repo_from_org_secret(
        &self,
        org: impl AsRef<str>,
        secret_name: impl AsRef<str>,
        repository_id: RepositoryId,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}",
            org = org.as_ref(),
            secret_name = secret_name.as_ref(),
            repository_id = repository_id,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._delete(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Cancels a workflow run using its id. You must authenticate using an
    /// access token with the `repo` scope to use this endpoint. GitHub Apps
    /// must have the `actions:write` permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .cancel_workflow_run("owner", "repo", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_workflow_run(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: RunId,
    ) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/cancel",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );
        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._post(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    async fn follow_location_to_data(
        &self,
        response: http::Response<BoxBody<Bytes, crate::Error>>,
    ) -> crate::Result<bytes::Bytes> {
        let data_response = self.crab.follow_location_to_data(response).await?;

        let body = data_response.into_body();

        body.collect().await.map(Collected::to_bytes)
    }

    /// Downloads and returns the raw data representing a zip of the logs from
    /// the workflow run specified by `run_id`.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .download_workflow_run_logs("owner", "repo", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_workflow_run_logs(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: RunId,
    ) -> crate::Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/logs",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        self.follow_location_to_data(self.crab._get(uri).await?)
            .await
    }

    /// Downloads and returns the raw data representing an artifact from a
    /// repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::actions::ArchiveFormat;
    ///
    /// octocrab::instance()
    ///     .actions()
    ///     .download_artifact("owner", "repo", 1234u64.into(), ArchiveFormat::Zip)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_artifact(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        artifact_id: ArtifactId,
        archive_format: params::actions::ArchiveFormat,
    ) -> crate::Result<bytes::Bytes> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            artifact_id = artifact_id,
            archive_format = archive_format,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;

        self.follow_location_to_data(self.crab._get(uri).await?)
            .await
    }

    /// Deletes all logs for a workflow run. You must authenticate using an
    /// access token with the `repo` scope to use this endpoint. GitHub Apps
    /// must have the `actions:write` permission to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .actions()
    ///     .delete_workflow_run_logs("owner", "repo", 1234u64.into())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_workflow_run_logs(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        run_id: RunId,
    ) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runs/{run_id}/logs",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
            run_id = run_id,
        );

        let uri = Uri::builder()
            .path_and_query(route)
            .build()
            .context(HttpSnafu)?;
        crate::map_github_error(self.crab._delete(uri, None::<&()>).await?)
            .await
            .map(drop)
    }

    /// Get an organization's public key, which you need to encrypt secrets.
    /// You need to encrypt a secret before you can create or update secrets.
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the secrets organization
    /// permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let org = octocrab.actions().get_org_public_key("org").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_org_public_key(
        &self,
        org: impl AsRef<str>,
    ) -> crate::Result<crate::models::PublicKey> {
        let route = format!("/orgs/{org}/actions/secrets/public-key", org = org.as_ref());

        self.crab.get(route, None::<&()>).await
    }

    /// Lists artifacts for a workflow run. Anyone with read access to the
    /// repository can use this endpoint. If the repository is private you
    /// must use an access token with the `repo` scope. GitHub Apps must have
    /// the `actions:read` permission to use this endpoint.
    pub fn list_workflow_run_artifacts(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        run_id: RunId,
    ) -> ListWorkflowRunArtifacts<'_> {
        ListWorkflowRunArtifacts::new(self.crab, owner.into(), repo.into(), run_id)
    }

    /// Dispatch a workflow run. You must authenticate using an
    /// access token with the `repo` scope to use this endpoint. GitHub Apps
    /// must have the `actions:write` permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.actions()
    ///    .create_workflow_dispatch("org", "repo", "workflow.yaml", "ref")
    ///    // optional
    ///    .inputs(serde_json::json!({"my-key": "my-value"}))
    ///    .send()
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub fn create_workflow_dispatch(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        workflow_id: impl Into<String>,
        r#ref: impl Into<String>,
    ) -> WorkflowDispatchBuilder<'_> {
        WorkflowDispatchBuilder::new(
            self.crab,
            owner.into(),
            repo.into(),
            workflow_id.into(),
            r#ref.into(),
        )
    }

    /// List all self-hosted runners configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the
    /// `organization_self_hosted_runners` permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let runners = octocrab.actions()
    ///    .list_org_self_hosted_runners("org")
    ///    // optional
    ///    .name("my-runner") // filter by name
    ///    .per_page(15)
    ///    .page(2u32)
    ///    .send()
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub fn list_org_self_hosted_runners(
        &self,
        org: impl Into<String>,
    ) -> ListSelfHostedRunnersBuilder<'_, '_> {
        ListSelfHostedRunnersBuilder::new_org(self, org.into())
    }

    /// Generates a configuration that can be passed to the runner application
    /// at startup.
    ///
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the
    /// `organization_self_hosted_runners` permission to use this endpoint.
    ///
    /// The `labels` member of the `CreateJitRunnerConfig` must have between
    /// 1 and 100 labels, inclusive.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let jit_config = octocrab
    ///     .actions()
    ///     .create_repo_jit_runner_config(
    ///         "owner",
    ///         "repo_name",
    ///         "my_runner_name",
    ///         2.into(),
    ///         ["label-1".into(), "label-2".into()],
    ///     )
    ///     .send()
    ///     .await?;
    /// // jit_config.encoded_jit_config contains the base64-encoded runner configuration
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_org_jit_runner_config(
        &self,
        org: impl Into<String>,
        name: impl Into<String>,
        runner_group_id: RunnerGroupId,
        labels: impl Into<Vec<String>>,
    ) -> CreateJitRunnerConfigBuilder<'_, '_> {
        CreateJitRunnerConfigBuilder::new_org(
            self,
            org.into(),
            name.into(),
            runner_group_id,
            labels.into(),
        )
    }

    /// Returns a token that you can pass to the self-hosted runner config
    /// script to register a self-hosted runner with an organization. The token
    /// expires after one hour.
    ///
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the
    /// `organization_self_hosted_runners` permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let token_resp = octocrab.actions()
    ///    .create_org_runner_registration_token("org")
    ///    .await?;
    /// // token_resp.token contains the token
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn create_org_runner_registration_token(
        &self,
        org: impl AsRef<str>,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerToken> {
        let route = format!(
            "/orgs/{org}/actions/runners/registration-token",
            org = org.as_ref()
        );

        self.crab.post(route, None::<&()>).await
    }

    /// Returns a token that you can pass to the self-hosted runner config
    /// script to remove a self-hosted runner from an organization. The token
    /// expires after one hour.
    ///
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the
    /// `organization_self_hosted_runners` permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let token_resp = octocrab.actions()
    ///    .create_org_runner_remove_token("org")
    ///    .await?;
    /// // token_resp.token contains the token
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn create_org_runner_remove_token(
        &self,
        org: impl AsRef<str>,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerToken> {
        let route = format!(
            "/orgs/{org}/actions/runners/remove-token",
            org = org.as_ref()
        );

        self.crab.post(route, None::<&()>).await
    }

    /// Gets a specific self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the
    /// `organization_self_hosted_runners` permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let runner = octocrab.actions()
    ///    .get_org_runner("org", 27.into())
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn get_org_runner(
        &self,
        org: impl AsRef<str>,
        runner_id: RunnerId,
    ) -> crate::Result<crate::models::actions::SelfHostedRunner> {
        let route = format!(
            "/orgs/{org}/actions/runners/{runner_id}",
            org = org.as_ref()
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Forces the removal of a self-hosted runner from an organization. You
    /// can use this endpoint to completely remove the runner when the machine
    /// you were using no longer exists.
    ///
    /// You must authenticate using an access token with the `admin:org` scope
    /// to use this endpoint. GitHub Apps must have the
    /// `organization_self_hosted_runners` permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.actions()
    ///    .delete_org_runner("org", 27.into())
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn delete_org_runner(
        &self,
        org: impl AsRef<str>,
        runner_id: RunnerId,
    ) -> crate::Result<()> {
        let route = format!(
            "/orgs/{org}/actions/runners/{runner_id}",
            org = org.as_ref()
        );

        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }

    /// List all self-hosted runners configured in a repository.
    ///
    /// You must authenticate using an access token with the `repo` scope
    /// to use this endpoint. GitHub Apps must have the `administration`
    /// permission for repositories to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let runners = octocrab.actions()
    ///    .list_repo_self_hosted_runners("owner", "repo")
    ///    // optional
    ///    .name("my-runner") // filter by name
    ///    .per_page(15)
    ///    .page(2u32)
    ///    .send()
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub fn list_repo_self_hosted_runners(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
    ) -> ListSelfHostedRunnersBuilder<'_, '_> {
        ListSelfHostedRunnersBuilder::new_repo(self, owner.into(), repo.into())
    }

    /// Generates a configuration that can be passed to the runner application
    /// at startup.
    ///
    /// You must authenticate using an access token with the `repo` scope
    /// to use this endpoint. GitHub Apps must have the `administration`
    /// permission for repositories to use this endpoint.
    ///
    /// The `labels` member of the `CreateJitRunnerConfig` must have between
    /// 1 and 100 labels, inclusive.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let jit_config = octocrab
    ///     .actions()
    ///     .create_org_jit_runner_config(
    ///         "org",
    ///         "my_runner_name",
    ///         2.into(),
    ///         ["label-1".into(), "label-2".into()],
    ///     )
    ///     .send()
    ///     .await?;
    /// // jit_config.encoded_jit_config contains the base64-encoded runner configuration
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_repo_jit_runner_config(
        &self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        name: impl Into<String>,
        runner_group_id: RunnerGroupId,
        labels: impl Into<Vec<String>>,
    ) -> CreateJitRunnerConfigBuilder<'_, '_> {
        CreateJitRunnerConfigBuilder::new_repo(
            self,
            owner.into(),
            repo.into(),
            name.into(),
            runner_group_id,
            labels.into(),
        )
    }

    /// Returns a token that you can pass to the self-hosted runner config
    /// script to register a self-hosted runner with an organization. The token
    /// expires after one hour.
    ///
    /// You must authenticate using an access token with the `repo` scope
    /// to use this endpoint. GitHub Apps must have the `administration`
    /// permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let token_resp = octocrab.actions()
    ///    .create_repo_runner_registration_token("owner", "repo")
    ///    .await?;
    /// // token_resp.token contains the token
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn create_repo_runner_registration_token(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerToken> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/registration-token",
            owner = owner.as_ref(),
            repo = repo.as_ref()
        );

        self.crab.post(route, None::<&()>).await
    }

    /// Returns a token that you can pass to the self-hosted runner config
    /// script to remove a self-hosted runner from an organization. The token
    /// expires after one hour.
    ///
    /// You must authenticate using an access token with the `repo` scope
    /// to use this endpoint. GitHub Apps must have the `administration`
    /// permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let token_resp = octocrab.actions()
    ///    .create_repo_runner_registration_token("owner", "repo")
    ///    .await?;
    /// // token_resp.token contains the token
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn create_repo_runner_remove_token(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> crate::Result<crate::models::actions::SelfHostedRunnerToken> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/remove-token",
            owner = owner.as_ref(),
            repo = repo.as_ref()
        );

        self.crab.post(route, None::<&()>).await
    }

    /// Gets a specific self-hosted runner configured in an organization.
    ///
    /// You must authenticate using an access token with the `repo` scope
    /// to use this endpoint. GitHub Apps must have the `administration`
    /// permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let runner = octocrab.actions()
    ///    .get_repo_runner("owner", "repo", 27.into())
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn get_repo_runner(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        runner_id: RunnerId,
    ) -> crate::Result<crate::models::actions::SelfHostedRunner> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/{runner_id}",
            owner = owner.as_ref(),
            repo = repo.as_ref()
        );

        self.crab.get(route, None::<&()>).await
    }

    /// Forces the removal of a self-hosted runner from an organization. You
    /// can use this endpoint to completely remove the runner when the machine
    /// you were using no longer exists.
    ///
    /// You must authenticate using an access token with the `repo` scope
    /// to use this endpoint. GitHub Apps must have the `administration`
    /// permission to use this endpoint.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// octocrab.actions()
    ///    .delete_repo_runner("owner", "repo", 27.into())
    ///    .await?;
    /// # return Ok(());
    /// # }
    /// ```
    pub async fn delete_repo_runner(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
        runner_id: RunnerId,
    ) -> crate::Result<()> {
        let route = format!(
            "/repos/{owner}/{repo}/actions/runners/{runner_id}",
            owner = owner.as_ref(),
            repo = repo.as_ref()
        );

        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }
}

/*
#[derive(serde::Serialize)]
pub struct RenderMarkdownBuilder<'octo, 'r, 'text> {
    #[serde(skip)]
    handler: &'r ActionsHandler<'octo>,
    text: &'text str,
    mode: Option<crate::params::markdown::Mode>,
    context: Option<String>,
}

impl<'octo, 'r, 'text> RenderMarkdownBuilder<'octo, 'r, 'text> {
    pub(crate) fn new(handler: &'r ActionsHandler<'octo>, text: &'text str) -> Self {
        Self {
            handler,
            text,
            mode: None,
            context: None,
        }
    }

    /// The repository context to use when creating references in `Mode::Gfm`.
    /// Omit this parameter when using markdown mode.
    pub fn context<A: Into<String>>(mut self, context: impl Into<Option<A>>) -> Self {
        self.context = context.into().map(A::into);
        self
    }

    /// The rendering mode.
    pub fn mode(mut self, mode: impl Into<Option<crate::params::markdown::Mode>>) -> Self {
        self.mode = mode.into();
        self
    }

    /// Send the actual request.
    pub async fn send(self) -> crate::Result<String> {
        self.handler
            .crab
            ._post(self.handler.crab.absolute_url("markdown")?, Some(&self))
            .await?
            .text()
            .await
            .context(crate::error::Http)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn serialize() {
        let octocrab = crate::instance();
        let handler = octocrab.markdown();
        let render = handler
            .render("**Markdown**")
            .mode(crate::params::markdown::Mode::Gfm)
            .context("owner/repo");


        assert_eq!(
            serde_json::to_value(render).unwrap(),
            serde_json::json!({
                "text": "**Markdown**",
                "mode": "gfm",
                "context": "owner/repo",
            })
        )
    }
}
*/
