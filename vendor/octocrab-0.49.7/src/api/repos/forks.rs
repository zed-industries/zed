use super::params::repos::forks::Sort;
use super::*;

#[derive(serde::Serialize)]
pub struct ListForksBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,
}

impl<'octo, 'r> ListForksBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            per_page: None,
            page: None,
            sort: None,
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

    /// Sort order of the results.
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::Page<crate::models::Repository>> {
        let route = format!("/{}/forks", self.handler.repo);
        self.handler.crab.get(route, Some(&self)).await
    }
}
#[derive(serde::Serialize)]
pub struct CreateForkBuilder<'octo, 'r> {
    #[serde(skip)]
    handler: &'r RepoHandler<'octo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch_only: Option<bool>,
}
impl<'octo, 'r> CreateForkBuilder<'octo, 'r> {
    pub(crate) fn new(handler: &'r RepoHandler<'octo>) -> Self {
        Self {
            handler,
            organization: None,
            name: None,
            default_branch_only: None,
        }
    }

    /// The organization name, if forking into an organization.
    pub fn organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// When forking from an existing repository, a new name for the fork.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// When forking from an existing repository, fork with only the default branch.
    pub fn default_branch_only(mut self, default_branch_only: impl Into<bool>) -> Self {
        self.default_branch_only = Some(default_branch_only.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<crate::models::Repository> {
        let route = format!("/{}/forks", self.handler.repo);
        self.handler.crab.post(route, Some(&self)).await
    }
}

impl RepoHandler<'_> {
    /// List forks of a repository. Optionally, specify the
    /// [sort](ListForksBuilder::sort()) order,
    /// [page](ListForksBuilder::page()),
    /// and items [per_page](ListForksBuilder::per_page())
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params::repos::forks::Sort;
    /// let forks = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .list_forks()
    ///     // Optional Parameters
    ///     .sort(Sort::Oldest)
    ///     .page(2u32)
    ///     .per_page(30)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_forks(&self) -> ListForksBuilder<'_, '_> {
        ListForksBuilder::new(self)
    }

    /// Creates a fork of a repository. Optionally, specify the target
    /// [organization](CreateForkBuilder::organization())
    /// or [name](CreateForkBuilder::name()) to create the fork in,
    /// or [default_branch_only](CreateForkBuilder::default_branch_only()) to fork with
    /// only the default branch.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let new_fork = octocrab::instance()
    ///     .repos("owner", "repo")
    ///     .create_fork()
    ///     // Optional Parameters
    ///     .organization("weyland-yutani")
    ///     .name("new-repo-name")
    ///     .default_branch_only(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_fork(&self) -> CreateForkBuilder<'_, '_> {
        CreateForkBuilder::new(self)
    }
}
