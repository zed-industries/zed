//! Get data about the currently authenticated user.

use crate::models::interaction_limits::{
    InteractionLimit, InteractionLimitExpiry, InteractionLimitType,
};
use crate::models::{interaction_limits, UpdateUserProfile};
use crate::{
    models::{self, gists::Gist, orgs::MembershipInvitation, Installation, Repository},
    Octocrab, Page, Result,
};
use chrono::{DateTime, Utc};

/// Handler for the current authenication API. **Note** All of the methods
/// provided below require at least some authenication such as personal token
/// in order to be used.
///
/// Created with [`Octocrab::current`].
pub struct CurrentAuthHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> CurrentAuthHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Fetches information about the current user.
    pub async fn user(&self) -> Result<models::Author> {
        self.crab.get("/user", None::<&()>).await
    }

    /// ### Update the authenticated user
    ///
    ///works with the following fine-grained token types:
    ///
    /// * [GitHub App user access tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
    /// * [Fine-grained personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)
    ///
    /// The fine-grained token must have the following permission set:
    ///
    /// * "Profile" user permissions (write)
    pub async fn update_user(&self, new_data: UpdateUserProfile) -> Result<models::Author> {
        let params = serde_json::to_value(new_data).unwrap();
        self.crab.patch("/user", Some(&params)).await
    }

    /// Fetches information about the currently authenticated app.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    ///
    /// let app = octocrab
    ///     .current()
    ///     .app()
    ///     .await?;
    ///
    /// println!("{}", app.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn app(&self) -> Result<models::App> {
        self.crab.get("/app", None::<&()>).await
    }

    /// List repositories starred by current authenticated user.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_repos_starred_by_authenticated_user()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user)
    pub fn list_repos_starred_by_authenticated_user(&self) -> ListStarredReposBuilder<'octo> {
        ListStarredReposBuilder::new(self.crab)
    }

    /// Lists repositories that the current authenticated user.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_repos_for_authenticated_user()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user)
    pub fn list_repos_for_authenticated_user(&self) -> ListReposForAuthenticatedUserBuilder<'octo> {
        ListReposForAuthenticatedUserBuilder::new(self.crab)
    }

    /// List gists for the current authenticated user.
    ///
    /// # Examples
    ///
    /// 1. The following snippet retrieves the most recent gist:
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_gists_for_authenticated_user()
    ///     .per_page(1)
    ///     .page(1)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// 2. This retrieves the first 100 gists, which is maximum number that
    ///    can be fetched in a single page:
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_gists_for_authenticated_user()
    ///     .per_page(100)
    ///     .page(1)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/gists/gists?apiVersion=latest#list-gists-for-the-authenticated-user)
    pub fn list_gists_for_authenticated_user(&self) -> ListGistsForAuthenticatedUserBuilder<'octo> {
        // self.crab.get("/gists", None::<&()>).await
        ListGistsForAuthenticatedUserBuilder::new(self.crab)
    }

    /// List gists that were starred by the authenticated user.
    pub fn list_gists_starred_by_authenticated_user(&self) -> ListStarredGistsBuilder<'octo> {
        ListStarredGistsBuilder::new(self.crab)
    }

    /// Lists installations of your GitHub App that the authenticated user has explicit permission (:read, :write, or :admin) to access.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_app_installations_accessible_to_user()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/apps/installations?apiVersion=2022-11-28#list-app-installations-accessible-to-the-user-access-token)
    pub fn list_app_installations_accessible_to_user(
        &self,
    ) -> ListAppInstallationsAccessibleToUserBuilder<'octo> {
        ListAppInstallationsAccessibleToUserBuilder::new(self.crab)
    }

    /// Lists organizations that the current authenticated user is a member of.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .current()
    ///     .list_org_memberships_for_authenticated_user()
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/orgs/members#list-organization-memberships-for-the-authenticated-user)
    pub fn list_org_memberships_for_authenticated_user(
        &self,
    ) -> ListOrgMembershipsForAuthenticatedUserBuilder<'octo> {
        ListOrgMembershipsForAuthenticatedUserBuilder::new(self.crab)
    }

    /// ### Get interaction restrictions for your public repositories
    ///
    /// Shows which type of GitHub user can interact with your public repositories and when the restriction expires.
    ///
    /// Fine-grained access tokens for "Get interaction restrictions for your public repositories"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token must have the following permission set:
    ///
    /// - "Interaction limits" user permissions (read)
    ///
    pub async fn get_interaction_restrictions(
        &self,
    ) -> crate::Result<interaction_limits::InteractionLimit> {
        let route = "/user/interaction-limits";
        self.crab.get(route, None::<&()>).await
    }

    /// ### Set interaction restrictions for your public repositories
    ///
    /// Temporarily restricts which type of GitHub user can interact with your public repositories. Setting the interaction limit at the user level will overwrite any interaction limits that are set for individual repositories owned by the user.
    ///
    /// Fine-grained access tokens for "Set interaction restrictions for your public repositories"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token must have the following permission set:
    ///
    /// - "Interaction limits" user permissions (write)
    ///
    pub async fn set_interaction_restrictions(
        &self,
        limit_type: InteractionLimitType,
        expiry: InteractionLimitExpiry,
    ) -> crate::Result<InteractionLimit> {
        let route = "/user/interaction-limits";
        let body = serde_json::json!({
            "limit": limit_type,
            "expiry": expiry,
        });
        self.crab.put(route, Some(&body)).await
    }

    /// ### Remove interaction restrictions from your public repositories
    ///
    /// Removes any interaction restrictions from your public repositories.
    ///
    /// Fine-grained access tokens for "Remove interaction restrictions from your public repositories"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token must have the following permission set:
    ///
    /// - "Interaction limits" user permissions (write)
    ///
    pub async fn remove_interaction_restrictions(&self) -> crate::Result<()> {
        let route = "/user/interaction-limits";
        let response = self.crab._delete(route, None::<&()>).await?;
        crate::map_github_error(response).await.map(drop)
    }
}

/// A builder pattern struct for listing starred repositories.
///
/// Created by [`CurrentAuthHandler::list_repos_starred_by_authenticated_user`].
///
/// [`CurrentAuthHandler::list_repos_starred_by_authenticated_user`]: ./struct.CurrentAuthHandler.html#method.list_repos_starred_by_authenticated_user
#[derive(serde::Serialize)]
pub struct ListStarredReposBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo> ListStarredReposBuilder<'octo> {
    fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
        }
    }

    /// One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// One of `asc` (ascending) or `desc` (descending).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/activity#list-repositories-starred-by-the-authenticated-user--parameters)
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Repository>> {
        self.crab.get("/user/starred", Some(&self)).await
    }
}

/// A builder pattern struct for listing repositories for authenticated user.
///
/// Created by [`CurrentAuthHandler::list_repos_for_authenticated_user`].
///
/// [`CurrentAuthHandler::list_repos_for_authenticated_user`]: ./struct.CurrentAuthHandler.html#method.list_repos_for_authenticated_user
#[derive(serde::Serialize)]
pub struct ListReposForAuthenticatedUserBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    affiliation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DateTime<Utc>>,
}

impl<'octo> ListReposForAuthenticatedUserBuilder<'octo> {
    fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            visibility: None,
            affiliation: None,
            r#type: None,
            sort: None,
            direction: None,
            per_page: None,
            page: None,
            since: None,
            before: None,
        }
    }

    /// Can be one of `all`, `public`, or `private`. Note: For GitHub AE, can be one of `all`, `internal`, or `private`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }

    /// Comma-separated list of values. Can include:
    /// * `owner`: Repositories that are owned by the authenticated user.
    /// * `collaborator`: Repositories that the user has been added to as a collaborator.
    /// * `organization_member`: Repositories that the user has access to through being a member of an organization. This includes every repository on every team that the user is on.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn affiliation(mut self, affiliation: impl Into<String>) -> Self {
        self.affiliation = Some(affiliation.into());
        self
    }

    /// Can be one of `all`, `owner`, `public`, `private`, `member`.
    ///
    /// Note: For GitHub AE, can be one of `all`, `owner`, `internal`, `private`, `member`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.r#type = Some(type_.into());
        self
    }

    /// Can be one of `created`, `updated`, `pushed`, `full_name`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Can be one of `asc` or `desc`.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// Results per page (max 100).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Only show notifications updated after the given time.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn since(mut self, since: impl Into<DateTime<Utc>>) -> Self {
        self.since = Some(since.into());
        self
    }

    /// Only show notifications updated before the given time.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/reference/repos#list-repositories-for-the-authenticated-user--parameters)
    pub fn before(mut self, before: impl Into<DateTime<Utc>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Repository>> {
        self.crab.get("/user/repos", (&self).into()).await
    }
}

/// A builder struct for initializing query parameters for use with the
/// `/gists` endpoint.
///
/// Created by: [`CurrentAuthHandler::list_gists_for_authenticated_user`].
///
/// [`CurrentAuthHandler::list_repos_starred_by_authenticated_user`]: ./struct.CurrentAuthHandler.html#method.list_gists_for_authenticated_user
#[derive(serde::Serialize)]
pub struct ListGistsForAuthenticatedUserBuilder<'octo> {
    /// Client under use for building the request.
    #[serde(skip)]
    crab: &'octo Octocrab,

    /// Only show gists that were updated after the given ISO 8601 UTC timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    /// The number of results per page (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    /// Page number of the results to fetch, starting at 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListGistsForAuthenticatedUserBuilder<'octo> {
    /// Create a new builder using the given client and default options as
    /// described in GitHub's API docs.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/gists/gists?apiVersion=latest#list-gists-for-the-authenticated-user)
    pub fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// Only show gists that were updated after the given ISO 8601 UTC timestamp.
    pub fn since(mut self, last_updated: DateTime<Utc>) -> Self {
        self.since = Some(last_updated);
        self
    }

    /// The number of results per page (max 100).
    pub fn per_page(mut self, count: u8) -> Self {
        self.per_page = Some(count);
        self
    }

    /// Page number of the results to fetch, starting at 1.
    pub fn page(mut self, page_num: u32) -> Self {
        self.page = Some(page_num);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Gist>> {
        self.crab.get("/gists", Some(&self)).await
    }
}

#[derive(serde::Serialize)]
pub struct ListStarredGistsBuilder<'octo> {
    /// Client under use for building the request.
    #[serde(skip)]
    crab: &'octo Octocrab,

    /// Only show gists that were starred after the given ISO 8601 UTC timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    since: Option<DateTime<Utc>>,

    /// Number of results to return per page. Maximum supported value is `100`.
    /// Larger values are clamped to `100`. Defaults to `30`
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    /// Page number of the results to fetch. Defaults to `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ListStarredGistsBuilder<'octo> {
    pub fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            since: None,
            per_page: None,
            page: None,
        }
    }

    /// Only show gists that were starred after the given ISO 8601 UTC timestamp.
    pub fn since(mut self, last_updated: DateTime<Utc>) -> Self {
        self.since = Some(last_updated);
        self
    }

    /// The page number from the result set to fetch.
    pub fn page(mut self, page_num: u32) -> Self {
        self.page = Some(page_num);
        self
    }

    pub fn per_page(mut self, count: u8) -> Self {
        self.per_page = Some(count);
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Gist>> {
        self.crab.get("/gists/starred", Some(&self)).await
    }
}

/// A builder pattern struct for listing organizations the authenticated user is a member of.
///
/// Created by [`CurrentAuthHandler::list_org_memberships_for_authenticated_user`].
///
/// [`CurrentAuthHandler::list_org_memberships_for_authenticated_user`]: ./struct.CurrentAuthHandler.html#method.list_org_memberships_for_authenticated_user
#[derive(serde::Serialize)]
pub struct ListOrgMembershipsForAuthenticatedUserBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo> ListOrgMembershipsForAuthenticatedUserBuilder<'octo> {
    fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/orgs/members#list-organization-memberships-for-the-authenticated-user--parameters)
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/orgs/members#list-organization-memberships-for-the-authenticated-user--parameters)
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<MembershipInvitation>> {
        self.crab
            .get("/user/memberships/orgs", (&self).into())
            .await
    }
}

/// A builder pattern struct for listing the installations accessible to a user access token.
///
/// Created by [`CurrentAuthHandler::list_app_installations_accessible_to_user`].
///
/// [`CurrentAuthHandler::list_app_installations_accessible_to_user`]: ./struct.CurrentAuthHandler.html#method.list_app_installations_accessible_to_user
#[derive(serde::Serialize)]
pub struct ListAppInstallationsAccessibleToUserBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,
}

impl<'octo> ListAppInstallationsAccessibleToUserBuilder<'octo> {
    fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/apps/installations?apiVersion=2022-11-28#list-app-installations-accessible-to-the-user-access-token--parameters)
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/apps/installations?apiVersion=2022-11-28#list-app-installations-accessible-to-the-user-access-token--parameters)
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Sends the actual request.
    pub async fn send(self) -> crate::Result<Page<Installation>> {
        self.crab.get("/user/installations", (&self).into()).await
    }
}
