//! **Note** This portal doesn't work for sandboxed applications.
//! # Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::proxy_resolver::ProxyResolver;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = ProxyResolver::new().await?;
//!     let url = ashpd::Uri::parse("www.google.com").unwrap();
//!
//!     println!("{:#?}", proxy.lookup(&url).await?);
//!
//!     Ok(())
//! }
//! ```

use crate::{Error, Uri, proxy::Proxy};

/// The interface provides network proxy information to sandboxed applications.
///
/// It is not a portal in the strict sense, since it does not involve user
/// interaction. Applications are expected to use this interface indirectly,
/// via a library API such as the GLib [`gio::ProxyResolver`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.ProxyResolver.html) interface.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.ProxyResolver`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ProxyResolver.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.ProxyResolver")]
pub struct ProxyResolver(Proxy<'static>);

impl ProxyResolver {
    /// Create a new instance of [`ProxyResolver`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.ProxyResolver").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`ProxyResolver`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.ProxyResolver")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Looks up which proxy to use to connect to `uri`.
    ///
    /// # Returns
    ///
    /// A list of proxy uris of the form `protocol://[user[:password]host:port`
    /// The protocol can be `http`, `rtsp`, `socks` or another proxying
    /// protocol. `direct://` is used when no proxy is needed.
    ///
    /// # Specifications
    ///
    /// See also [`Lookup`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ProxyResolver.html#org-freedesktop-portal-proxyresolver-lookup).
    #[doc(alias = "Lookup")]
    pub async fn lookup(&self, uri: &Uri) -> Result<Vec<Uri>, Error> {
        self.0.call("Lookup", &(uri)).await
    }
}

impl std::ops::Deref for ProxyResolver {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
