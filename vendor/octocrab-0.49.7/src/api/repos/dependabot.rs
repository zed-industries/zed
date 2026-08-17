use super::RepoHandler;

/// A client to GitHub's repository dependabot API.
///
/// Created with [`Octocrab::repos`](crate::Octocrab::repos).
pub struct RepoDependabotAlertsHandler<'octo> {
    handler: &'octo RepoHandler<'octo>,
    params: Params,
}

#[derive(serde::Serialize)]
struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ecosystem: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,
}

impl<'octo> RepoDependabotAlertsHandler<'octo> {
    pub(crate) fn new(repo: &'octo RepoHandler<'octo>) -> Self {
        Self {
            handler: repo,
            params: Params {
                per_page: None,
                page: None,
                state: None,
                severity: None,
                ecosystem: None,
                package: None,
                manifest: None,
                scope: None,
                sort: None,
                direction: None,
            },
        }
    }

    /// Lists all Dependabot Alerts available in a repository.
    /// You must authenticate using an access token with the `repo` or `security_events` scope to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let all_secrets = octocrab.repos("owner", "repo")
    ///     .dependabot()
    ///     .direction("asc")
    ///     .get_alerts()
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_alerts(
        &self,
    ) -> crate::Result<crate::Page<crate::models::repos::dependabot::DependabotAlert>> {
        let route = format!("/{}/dependabot/alerts", self.handler.repo);
        self.handler.crab.get(route, Some(&self.params)).await
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.params.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.params.page = Some(page.into());
        self
    }

    /// Filter Dependabot Alerts by state.
    pub fn state(mut self, state: impl Into<Vec<String>>) -> Self {
        self.params.state = Some(state.into());
        self
    }

    /// Filter Dependabot Alerts by severity.
    pub fn severity(mut self, severity: impl Into<Vec<String>>) -> Self {
        self.params.severity = Some(severity.into());
        self
    }

    /// Filter Dependabot Alerts by ecosystem.
    pub fn ecosystem(mut self, ecosystem: impl Into<Vec<String>>) -> Self {
        self.params.ecosystem = Some(ecosystem.into());
        self
    }

    /// Filter Dependabot Alerts by package.
    pub fn package(mut self, package: impl Into<Vec<String>>) -> Self {
        self.params.package = Some(package.into());
        self
    }

    /// Filter Dependabot Alerts by manifest.
    pub fn manifest(mut self, manifest: impl Into<Vec<String>>) -> Self {
        self.params.manifest = Some(manifest.into());
        self
    }

    /// Filter Dependabot Alerts by scope.
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.params.scope = Some(scope.into());
        self
    }

    /// Sort Dependabot Alerts.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.params.sort = Some(sort.into());
        self
    }

    /// Sort direction of Dependabot Alerts.
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.params.direction = Some(direction.into());
        self
    }

    /// Lists single Dependabot Alert for a repository.
    /// You must authenticate using an access token with the `repo` or `security_events` scope to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let all_secrets = octocrab.repos("owner", "repo")
    ///     .dependabot()
    ///     .get_alert(5)
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn get_alert(
        &self,
        alert_number: u32,
    ) -> crate::Result<crate::models::repos::dependabot::DependabotAlert> {
        let route = format!("/{}/dependabot/alerts/{}", self.handler.repo, alert_number);
        self.handler.crab.get(route, None::<&()>).await
    }

    /// Updates a dependabot alert.
    /// You must authenticate using an access token with the `security_events ` scope to use this endpoint.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models::repos::dependabot::UpdateDependabotAlert;
    ///
    /// let result = octocrab.repos("owner", "repo")
    ///     .dependabot()
    ///     .update_alert(
    ///         5,
    ///         Some(&UpdateDependabotAlert {
    ///             state: "dismissed",
    ///             dismissed_reason: Some("no_bandwidth"),
    ///             dismissed_comment: Some("I don't have time to fix this right now"),
    ///         })
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    pub async fn update_alert(
        &self,
        alert_number: u32,
        alert_update: Option<&crate::models::repos::dependabot::UpdateDependabotAlert<'_>>,
    ) -> crate::Result<crate::models::repos::dependabot::DependabotAlert> {
        let route = format!("/{}/dependabot/alerts/{}", self.handler.repo, alert_number);
        self.handler.crab.patch(route, alert_update).await
    }
}
