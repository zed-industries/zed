//! Github Starring API

use http::header::HeaderMap;
use http::header::ACCEPT;
use serde::Serialize;

use crate::models::activity::StarredRepository;
use crate::Octocrab;
use crate::Page;
use crate::Result;

/// Handler for GitHub's starring API.
///
/// Created with [`ActivityHandler::starring`].
///
/// [`ActivityHandler::starring`]: ../struct.ActivityHandler.html#method.starring
pub struct StarringHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> StarringHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// Lists repositories a user has starred.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .activity()
    ///     .starring()
    ///     .list_repos_starred_by_user("some_user")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/starring?apiVersion=2022-11-28#list-repositories-starred-by-a-user)
    pub fn list_repos_starred_by_user(
        &self,
        username: impl Into<String>,
    ) -> ListReposStarredByUserBuilder<'_> {
        ListReposStarredByUserBuilder::new(self.crab, username)
    }
}

/// A builder pattern struct for listing starred by user repositories.
///
/// Created by [`StarringHandler::list_repos_starred_by_user`].
///
/// [`StarringHandler::list_repos_starred_by_user`]: ./struct.StarringHandler.html#method.list_repos_starred_by_user
#[derive(Serialize)]
pub struct ListReposStarredByUserBuilder<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,

    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,

    #[serde(skip)]
    username: String,
}

impl<'octo> ListReposStarredByUserBuilder<'octo> {
    pub fn new(crab: &'octo Octocrab, username: impl Into<String>) -> Self {
        Self {
            crab,
            direction: None,
            page: None,
            per_page: None,
            sort: None,
            username: username.into(),
        }
    }

    /// The direction to sort the results by.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/starring?apiVersion=2022-11-28#list-repositories-starred-by-a-user--parameters)
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    /// The page number of the results to fetch.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/starring?apiVersion=2022-11-28#list-repositories-starred-by-a-user--parameters)
    pub fn page(mut self, page: impl Into<u8>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// The number of results per page (max `100`).
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/starring?apiVersion=2022-11-28#list-repositories-starred-by-a-user--parameters)
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    pub async fn send(self) -> Result<Page<StarredRepository>> {
        let mut headers = HeaderMap::new();

        headers.insert(
            ACCEPT,
            "application/vnd.github.star+json"
                .parse()
                .expect("valid header value"),
        );

        self.crab
            .get_with_headers(
                format!("/users/{username}/starred", username = self.username),
                None::<&()>,
                Some(headers),
            )
            .await
    }

    /// The property to sort the results by. `created` means when the repository was starred. `updated` means when the repository was last pushed to.
    ///
    /// [See the GitHub API documentation](https://docs.github.com/en/rest/activity/starring?apiVersion=2022-11-28#list-repositories-starred-by-a-user--parameters)
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}
