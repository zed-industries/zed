use zbus::interface;

use super::watcher::StatusNotifierWatcher;

const STATUS_NOTIFIER_HOST_PATH: &str = "/StatusNotifierHost";

struct StatusNotifierHostInterface;

#[interface(name = "org.kde.StatusNotifierHost")]
impl StatusNotifierHostInterface {}

pub struct StatusNotifierHost(zbus::Connection);

impl StatusNotifierHost {
    pub async fn new(id: String) -> zbus::Result<Self> {
        let watcher = StatusNotifierWatcher::new().await?;
        let name = format!("org.freedesktop.StatusNotifierHost-{}", id);
        let conn = zbus::connection::Builder::session()?
            .name(name.clone())?
            .serve_at(STATUS_NOTIFIER_HOST_PATH, StatusNotifierHostInterface)?
            .build()
            .await?;
        watcher.register_status_notifier_host(name).await?;
        Ok(Self(conn))
    }
}
