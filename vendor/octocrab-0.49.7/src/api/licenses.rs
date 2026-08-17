//! Metadata about popular open source licenses and information about a
//! project's license file.

use crate::{models, Octocrab};

/// Handler for GitHub's license API.
///
/// Created with [`Octocrab::licenses`].
pub struct LicenseHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> LicenseHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// List commonly used licenses.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let licenses = octocrab::instance().licenses().list_commonly_used().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_commonly_used(&self) -> crate::Result<Vec<models::License>> {
        self.crab.get("/licenses", None::<&()>).await
    }

    /// Get an individual license.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// let license = octocrab::instance().licenses().get("mit").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, key: impl AsRef<str>) -> crate::Result<models::License> {
        self.crab
            .get(format!("/licenses/{}", key.as_ref()), None::<&()>)
            .await
    }
}
