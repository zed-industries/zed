use std::sync::OnceLock;

pub(crate) mod unity_launcher;

static SESSION: OnceLock<zbus::Connection> = OnceLock::new();

// TODO: this will spawn a new thread, maybe we could find a way to reuse the ashpd connection?
pub(crate) async fn connection() -> zbus::Result<zbus::Connection> {
    if let Some(conn) = SESSION.get() {
        Ok(conn.clone())
    } else {
        let conn = zbus::Connection::session().await?;
        Ok(SESSION.get_or_init(|| conn).clone())
    }
}
