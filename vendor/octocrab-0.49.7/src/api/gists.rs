//! The gist API
//!
//! Supports CRUD operations on gists in GitHub.
//!
//! [Official documentation][docs]
//!
//! [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28
mod list_commits;
mod list_forks;
mod list_gists;

use http::StatusCode;
use serde::Serialize;
use std::collections::BTreeMap;

pub use self::list_commits::ListCommitsBuilder;
pub use self::list_gists::{ListAllGistsBuilder, ListPublicGistsBuilder, ListUserGistsBuilder};

use crate::{
    models::gists::{Gist, GistRevision},
    Octocrab, Result,
};

/// Handler for GitHub's gist API.
///
/// Created with [`Octocrab::gists`].
pub struct GistsHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> GistsHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// List all gists from GitHub's gist API.
    ///
    /// See: [GitHub API Documentation][docs] for `GET /gists`
    ///
    /// # Note
    /// * Calling with an authentication token will list all the gists of the
    ///   authenticated user
    ///
    /// * If no authentication token will list all the public gists from
    ///   GitHub's API. This can potentially produce a lot of results, so care is
    ///   advised.
    ///
    /// # Example
    ///
    /// 1) This shows one page of (10) results for all public gists created a
    ///    from the day before:
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///     let yesterday: chrono::DateTime<chrono::Utc> =
    ///         chrono::Utc::now()
    ///             .checked_sub_days(chrono::Days::new(1)).unwrap();
    ///     octocrab::instance()
    ///         .gists()
    ///         .list_all_gists()
    ///         .since(yesterday)
    ///         .page(1u32)
    ///         .per_page(10u8)
    ///         .send()
    ///         .await?;
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#list-gists-for-the-authenticated-user
    pub fn list_all_gists(&self) -> ListAllGistsBuilder<'octo> {
        ListAllGistsBuilder::new(self.crab)
    }

    /// List public gists sorted by most recently updated to least recently
    /// updated. This works similarly to the `GistsHandler::list_all_gists`
    ///
    /// See: [GitHub API Documentation][docs] for `GET /gists/public`
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///     let yesterday: chrono::DateTime<chrono::Utc> =
    ///         chrono::Utc::now()
    ///             .checked_sub_days(chrono::Days::new(1)).unwrap();
    ///     let all_public_gists = octocrab::instance()
    ///         .gists()
    ///         .list_all_recent_public_gists()
    ///         .since(yesterday)
    ///         .page(1u32)
    ///         .per_page(10u8)
    ///         .send()
    ///         .await?;
    /// #   Ok(())
    /// # }
    ///
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#list-public-gists
    pub fn list_all_recent_public_gists(&self) -> ListPublicGistsBuilder<'octo> {
        ListPublicGistsBuilder::new(self.crab)
    }

    /// List gists for the given username, allowing for pagination.
    ///
    /// See [GitHub API Documentation][docs] for details on `GET /users/{username}/gists`
    ///
    /// # Examples
    ///
    /// * Fetch 10 recent gists for the user with login "foouser":
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    ///     octocrab::instance()
    ///         .gists()
    ///         .list_user_gists("foouser")
    ///         .page(1u32)
    ///         .per_page(10u8)
    ///         .send()
    ///         .await?;
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#list-gists-for-a-user
    pub fn list_user_gists(&self, username: impl AsRef<str>) -> ListUserGistsBuilder<'octo> {
        ListUserGistsBuilder::new(self.crab, username.as_ref().to_string())
    }

    /// Create a new gist.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let gitignore = octocrab::instance()
    ///     .gists()
    ///     .create()
    ///     .file("hello_world.rs", "fn main() {\n println!(\"Hello World!\");\n}")
    ///     // Optional Parameters
    ///     .description("Hello World in Rust")
    ///     .public(false)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self) -> CreateGistBuilder<'octo> {
        CreateGistBuilder::new(self.crab)
    }

    /// Update an existing gist.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let gitignore = octocrab::instance()
    ///   .gists()
    ///   .update("aa5a315d61ae9438b18d")
    ///   // Optional Parameters
    ///   .description("Updated!")
    ///   .file("hello_world.rs")
    ///   .rename_to("fibonacci.rs")
    ///   .with_content("fn main() {\n println!(\"I should be a Fibonacci!\");\n}")
    ///   .file("delete_me.rs")
    ///   .delete()
    ///   .send()
    ///   .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, id: impl AsRef<str>) -> UpdateGistBuilder<'octo> {
        UpdateGistBuilder::new(self.crab, format!("/gists/{id}", id = id.as_ref()))
    }

    /// Get a single gist.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let gist = octocrab::instance().gists().get("00000000000000000000000000000000").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: impl AsRef<str>) -> Result<Gist> {
        let id = id.as_ref();
        self.crab.get(format!("/gists/{id}"), None::<&()>).await
    }

    /// Delete a single gist.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance().gists().delete("00000000000000000000000000000000").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, gist_id: impl AsRef<str>) -> Result<()> {
        let gist_id = gist_id.as_ref();
        let response = self
            .crab
            ._delete(format!("/gists/{gist_id}"), None::<&()>)
            .await?;

        if response.status() != StatusCode::NOT_MODIFIED && !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }

    /// Get a single gist revision.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let revision = octocrab::instance()
    ///     .gists()
    ///     .get_revision("00000000000000000000000000000000", "1111111111111111111111111111111111111111")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_revision(
        &self,
        id: impl AsRef<str>,
        sha1: impl AsRef<str>,
    ) -> Result<GistRevision> {
        let id = id.as_ref();
        let sha1 = sha1.as_ref();
        self.crab
            .get(format!("/gists/{id}/{sha1}"), None::<&()>)
            .await
    }

    /// List commits for the specified gist.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// use octocrab::params;
    ///
    /// // Get the least active repos belonging to `owner`.
    /// let page = octocrab::instance()
    ///     .gists()
    ///     .list_commits("00000000000000000000000000000000")
    ///     // Optional Parameters
    ///     .per_page(25)
    ///     .page(5u32)
    ///     // Send the request.
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_commits(
        &self,
        gist_id: impl Into<String>,
    ) -> list_commits::ListCommitsBuilder<'_, '_> {
        list_commits::ListCommitsBuilder::new(self, gist_id.into())
    }

    /// Check if the given is gist is already starred by the authenticated user.
    /// See [GitHub API Documentation][docs] more information about response
    /// data.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let is_starred: bool = octocrab::instance()
    ///     .gists()
    ///     .is_starred("00000000000000000000000000000000")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#check-if-a-gist-is-starred
    pub async fn is_starred(&self, gist_id: impl AsRef<str>) -> Result<bool> {
        let gist_id = gist_id.as_ref();
        let response = self.crab._get(format!("/gists/{gist_id}/star")).await?;
        // Gist API returns 204 (NO CONTENT) if a gist is starred

        match response.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => Err(crate::map_github_error(response).await.unwrap_err()),
        }
    }

    /// Star the given gist. See [GitHub API Documentation][docs] more
    /// information about response data.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .gists()
    ///     .star("00000000000000000000000000000000")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#star-a-gist
    pub async fn star(&self, gist_id: impl AsRef<str>) -> Result<()> {
        let gist_id = gist_id.as_ref();
        // PUT here returns an empty body, ignore it since it doesn't make
        // sense to deserialize it as JSON.
        let response = self
            .crab
            ._put(format!("/gists/{gist_id}/star"), None::<&()>)
            .await?;

        if !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }

    /// Unstar the given gist. See [GitHub API Documentation][docs] more
    /// information about response data.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .gists()
    ///     .unstar("00000000000000000000000000000000")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#unstar-a-gist
    pub async fn unstar(&self, gist_id: impl AsRef<str>) -> Result<()> {
        let gist_id = gist_id.as_ref();
        // DELETE here returns an empty body, ignore it since it doesn't make
        // sense to deserialize it as JSON.
        let response = self
            .crab
            ._delete(format!("/gists/{gist_id}/star"), None::<&()>)
            .await?;

        if response.status() != StatusCode::NOT_MODIFIED && !response.status().is_success() {
            return Err(crate::map_github_error(response).await.unwrap_err());
        }

        Ok(())
    }

    /// Retrieve all the gists that forked the given `gist_id`. See
    /// [GitHub API Docs][docs] for information about request parameters, and
    /// response schema.
    ///
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// octocrab::instance()
    ///     .gists()
    ///     .list_forks("00000000000000000000000000000000")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#list-gist-forks
    pub fn list_forks(
        &self,
        gist_id: impl Into<String>,
    ) -> list_forks::ListGistForksBuilder<'_, '_> {
        list_forks::ListGistForksBuilder::new(self, gist_id.into())
    }

    /// Create a fork of the given `gist_id` associated with the authenticated
    /// user's account. See [GitHub API docs][docs] for more information about
    /// request parameters and response schema.
    ///
    /// [docs]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28#fork-a-gist
    pub async fn fork(&self, gist_id: impl AsRef<str>) -> Result<Gist> {
        let route = format!("/gists/{gist_id}/forks", gist_id = gist_id.as_ref());
        self.crab.post(route, None::<&()>).await
    }
}

#[derive(Debug)]
pub struct CreateGistBuilder<'octo> {
    crab: &'octo Octocrab,
    data: CreateGist,
}

impl<'octo> CreateGistBuilder<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            data: Default::default(),
        }
    }

    /// Set a description for the gist to be created.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.data.description = Some(description.into());
        self
    }

    /// Set the `public` flag of the gist to be created.
    pub fn public(mut self, public: bool) -> Self {
        self.data.public = Some(public);
        self
    }

    /// Add a file to the gist with `filename` and `content`.
    pub fn file(mut self, filename: impl Into<String>, content: impl Into<String>) -> Self {
        let file = CreateGistFile {
            filename: Default::default(),
            content: content.into(),
        };
        self.data.files.insert(filename.into(), file);
        self
    }

    /// Send the `CreateGist` request to Github for execution.
    pub async fn send(self) -> Result<Gist> {
        self.crab.post("/gists", Some(&self.data)).await
    }
}

#[derive(Debug, Default, Serialize)]
struct CreateGist {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public: Option<bool>,
    files: BTreeMap<String, CreateGistFile>,
}

#[derive(Debug, Serialize)]
struct CreateGistFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    content: String,
}

#[derive(Debug)]
pub struct UpdateGistBuilder<'octo> {
    crab: &'octo Octocrab,
    gist_path: String,
    data: UpdateGist,
}

impl<'octo> UpdateGistBuilder<'octo> {
    fn new(crab: &'octo Octocrab, gist_path: String) -> Self {
        Self {
            crab,
            gist_path,
            data: Default::default(),
        }
    }

    /// Update the description of the gist with the content provided by `description`.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.data.description = Some(description.into());
        self
    }

    /// Update the file with the `filename`.
    ///
    /// The update operation is chosen in further calls to the returned builder.
    pub fn file(self, filename: impl Into<String>) -> UpdateGistFileBuilder<'octo> {
        UpdateGistFileBuilder::new(self, filename)
    }

    /// Send the `UpdateGist` command to Github for execution.
    pub async fn send(self) -> Result<Gist> {
        self.crab.patch(self.gist_path, Some(&self.data)).await
    }
}

#[derive(Debug, Default, Serialize)]
struct UpdateGist {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<BTreeMap<String, Option<UpdateGistFile>>>,
}

#[derive(Debug, Default, Serialize)]
pub struct UpdateGistFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

pub struct UpdateGistFileBuilder<'octo> {
    builder: UpdateGistBuilder<'octo>,
    filename: String,
    file: Option<UpdateGistFile>,
    ready: bool,
}

impl<'octo> UpdateGistFileBuilder<'octo> {
    fn new(builder: UpdateGistBuilder<'octo>, filename: impl Into<String>) -> Self {
        Self {
            builder,
            filename: filename.into(),
            file: None,
            ready: false,
        }
    }

    fn build(mut self) -> UpdateGistBuilder<'octo> {
        if self.ready {
            self.builder
                .data
                .files
                .get_or_insert_with(BTreeMap::new)
                .insert(self.filename, self.file);
        }
        self.builder
    }

    /// Delete the file from the gist.
    pub fn delete(mut self) -> UpdateGistBuilder<'octo> {
        self.ready = true;
        self.file = None;
        self.build()
    }

    /// Rename the file to `filename`.
    pub fn rename_to(mut self, filename: impl Into<String>) -> Self {
        self.ready = true;
        self.file.get_or_insert_with(Default::default).filename = Some(filename.into());
        self
    }

    /// Update the content of the file and overwrite it with `content`.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.ready = true;
        self.file.get_or_insert_with(Default::default).content = Some(content.into());
        self
    }

    /// Overwrite the Description of the gist with `description`.
    ///
    /// This will finalize the update operation and will continue to operate on the gist itself.
    pub fn description(self, description: impl Into<String>) -> UpdateGistBuilder<'octo> {
        self.build().description(description)
    }

    /// Update the next file identified by `filename`.
    ///
    /// This will finalize the update operation and will continue to operate on the gist itself.
    pub fn file(self, filename: impl Into<String>) -> UpdateGistFileBuilder<'octo> {
        self.build().file(filename)
    }

    /// Send the `UpdateGist` command to Github for execution.
    ///
    /// This will finalize the update operation before sending.
    pub async fn send(self) -> Result<Gist> {
        self.build().send().await
    }
}
