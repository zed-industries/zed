//! The code scanning API.
use crate::{models, params, Octocrab, Result};

mod list;
mod update;

/// Handler for GitHub's code scanning API.
///
/// Created with [`Octocrab::code_scannings`].
pub struct CodeScanningHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: Option<String>,
}

impl<'octo> CodeScanningHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: Option<String>) -> Self {
        Self { crab, owner, repo }
    }

    /// Gets an code scanning from the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let code_scanning = octocrab.code_scannings("owner", "repo").get(3).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&mut self, number: u64) -> Result<models::code_scannings::CodeScanningAlert> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/alerts/{number}",
            owner = self.owner,
            repo = self.repo.as_mut().expect("Repository must be specified"),
            number = number,
        );

        self.crab.get(route, None::<&()>).await
    }

    /// List code scannings in the repository.
    pub fn list(&self) -> list::ListCodeScanningsBuilder<'_, '_> {
        list::ListCodeScanningsBuilder::new(self)
    }

    /// Update a code scanning alert
    /// ```no_run
    /// # use octocrab::params;
    ///
    ///  async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// use octocrab::models;
    ///
    /// let issue = octocrab.code_scannings("owner", "repo")
    ///     .update(1234u64)
    ///     .state(params::AlertState::Dismissed)
    ///     // Send the request
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, number: u64) -> update::UpdateCodeScanningBuilder<'_, '_> {
        update::UpdateCodeScanningBuilder::new(self, number)
    }
}
