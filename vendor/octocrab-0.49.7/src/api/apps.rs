use crate::{models::InstallationId, Octocrab};
use http::request::Builder;
use http::Method;

mod installations;

/// A client to [GitHub's apps API][apps-api].
///
/// Created with [`Octocrab::apps`].
///
/// [apps-api]: https://docs.github.com/en/rest/reference/apps
pub struct AppsRequestHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> AppsRequestHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Get an installation for the authenticated app
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models::InstallationId;
    ///
    /// let installation = octocrab
    ///     .apps()
    ///     .installation(InstallationId(1))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn installation(
        &self,
        installation_id: InstallationId,
    ) -> crate::Result<crate::models::Installation> {
        let route = format!("/app/installations/{installation_id}",);

        self.crab.get(&route, None::<&()>).await
    }

    /// Creates a new `InstallationsBuilder` that can be configured to filter
    /// listing installations.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::params;
    ///
    /// let page = octocrab
    ///     .apps()
    ///     .installations()
    ///     // Optional Parameters
    ///     .since(chrono::Utc::now() - chrono::Duration::days(1))
    ///     .per_page(100)
    ///     .page(5u32)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn installations(&self) -> installations::InstallationsRequestBuilder<'_, '_> {
        installations::InstallationsRequestBuilder::new(self)
    }

    pub(crate) async fn http_get<R, A, P>(
        &self,
        route: A,
        parameters: Option<&P>,
    ) -> crate::Result<R>
    where
        A: AsRef<str>,
        P: serde::Serialize + ?Sized,
        R: crate::FromResponse,
    {
        let request = Builder::new()
            .method(Method::GET)
            .uri(self.crab.parameterized_uri(route, parameters)?);
        let request = self.crab.build_request(request, None::<&()>)?;
        R::from_response(crate::map_github_error(self.crab.execute(request).await?).await?).await
    }

    /// Get a repository installation for the authenticated app.
    pub async fn get_repository_installation(
        &self,
        owner: impl AsRef<str>,
        repo: impl AsRef<str>,
    ) -> crate::Result<crate::models::Installation> {
        let route = format!(
            "/repos/{owner}/{repo}/installation",
            owner = owner.as_ref(),
            repo = repo.as_ref(),
        );

        self.crab.get(&route, None::<&()>).await
    }

    /// Get an organization installation for the authenticated app.
    pub async fn get_org_installation(
        &self,
        owner: impl AsRef<str>,
    ) -> crate::Result<crate::models::Installation> {
        let route = format!("/orgs/{owner}/installation", owner = owner.as_ref(),);

        self.crab.get(&route, None::<&()>).await
    }
}
