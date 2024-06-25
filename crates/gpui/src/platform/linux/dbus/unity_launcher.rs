//! Implements part of the [Unity Launcher API](https://wiki.ubuntu.com/Unity/LauncherAPI)

use std::hash::Hasher;

use seahash::SeaHasher;
use zbus::zvariant::{SerializeDict, Type};

use super::connection;

#[derive(SerializeDict, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Options to pass to [`LauncherEntryProxy::update`]
pub(crate) struct UpdateOptions {
    count: Option<u32>,
    count_visible: Option<bool>,
    progress: Option<f32>,
    progress_visible: Option<bool>,
    urgent: Option<bool>,
    // Dynamic quicklists are not supported.
}

#[allow(dead_code)]
impl UpdateOptions {
    /// A count badge that will be associated with the icon.
    pub fn count(mut self, count: impl Into<Option<u32>>) -> Self {
        self.count = count.into();
        self
    }

    /// Sets whether the `count` will be visible.
    pub fn count_visible(mut self, count_visible: impl Into<Option<bool>>) -> Self {
        self.count_visible = count_visible.into();
        self
    }

    /// Sets the progress to be associated with the icon. The progress value should be between `0.0` and `1.0`.
    pub fn progress(mut self, progress: impl Into<Option<f32>>) -> Self {
        self.progress = progress.into();
        self
    }

    /// Sets whether the `progress` will be visible.
    pub fn progress_visible(mut self, progress_visible: impl Into<Option<bool>>) -> Self {
        self.progress_visible = progress_visible.into();
        self
    }

    /// Sets whether the program is asking for user attention.
    pub fn urgent(mut self, urgent: impl Into<Option<bool>>) -> Self {
        self.urgent = urgent.into();
        self
    }
}

/// Wrapper for the [LauncherEntry](https://wiki.ubuntu.com/Unity/LauncherAPI#Low_level_DBus_API:_com.canonical.Unity.LauncherEntry) API.
pub(crate) struct LauncherEntryProxy;

impl LauncherEntryProxy {
    /// Sends a request to update the launcher entry.
    ///
    /// # Arguments
    ///
    /// * `app_id` - A URI of the form `application://$desktop_file`
    /// * `options` - The properties to update (will not override previous properties)
    pub async fn update(app_uri: &str, options: UpdateOptions) -> zbus::Result<()> {
        let path = format!(
            "/com/canonical/unity/launcherentry/{}",
            hash_app_uri(app_uri)
        );
        let msg = zbus::Message::signal(path, "com.canonical.Unity.LauncherEntry", "Update")?
            .build(&(app_uri, options))?;
        let conn = connection().await?;
        conn.send(&msg).await
    }
}

// libunity hashes the app_id, and this seems to be enforced by an AppArmor rule
fn hash_app_uri(app_id: &str) -> u64 {
    let mut hasher = SeaHasher::new();
    hasher.write(app_id.as_bytes());
    hasher.finish()
}
