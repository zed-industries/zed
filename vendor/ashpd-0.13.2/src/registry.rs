use serde::Serialize;
use zbus::zvariant;

use crate::{AppID, Error, proxy::Proxy};

#[derive(Debug, Serialize, zvariant::Type, Default)]
#[zvariant(signature = "dict")]
struct RegisterOptions {}

struct RegistryProxy(Proxy<'static>);

impl RegistryProxy {
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.host.portal.Registry").await?;
        Ok(Self(proxy))
    }

    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.host.portal.Registry")
                .await?;
        Ok(Self(proxy))
    }

    pub async fn register(&self, app_id: AppID, options: RegisterOptions) -> Result<(), Error> {
        self.0.call_method("Register", &(&app_id, &options)).await?;
        Ok(())
    }
}

impl std::ops::Deref for RegistryProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/// Registers a host application for portal usage.
///
/// Portals rely on the application ID to store and manage the permissions of
/// individual apps. However, for non-sandboxed (host) applications, this
/// information cannot be directly retrieved. To resolve this, the method first
/// verifies that the application is not sandboxed. It then uses the
/// `org.freedesktop.host.portal.Registry` interface to register the process
/// communicating over DBus with the portal as the owner of the specified
/// application ID.
/// For more technical details, see <https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.host.portal.Registry.html>
pub async fn register_host_app(app_id: AppID) -> crate::Result<()> {
    if crate::is_sandboxed() {
        return Ok(());
    }
    let proxy = RegistryProxy::new().await?;
    proxy.register(app_id, Default::default()).await?;
    Ok(())
}

/// Similar to [`register_host_app`] that takes a connection parameter.
pub async fn register_host_app_with_connection(
    connection: zbus::Connection,
    app_id: AppID,
) -> crate::Result<()> {
    if crate::is_sandboxed() {
        return Ok(());
    }
    let proxy = RegistryProxy::with_connection(connection).await?;
    proxy.register(app_id, Default::default()).await?;
    Ok(())
}
