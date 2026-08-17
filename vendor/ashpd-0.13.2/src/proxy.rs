#![allow(missing_docs)]
use std::{fmt::Debug, future::ready, ops::Deref, sync::OnceLock};

use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tracing")]
use zbus::Message;
use zbus::zvariant::{ObjectPath, OwnedValue, Type};

use crate::{
    Error, PortalError,
    desktop::{HandleToken, Request},
};

pub(crate) const DESKTOP_DESTINATION: &str = "org.freedesktop.portal.Desktop";
pub(crate) const DESKTOP_PATH: &str = "/org/freedesktop/portal/desktop";

pub(crate) const DOCUMENTS_DESTINATION: &str = "org.freedesktop.portal.Documents";
pub(crate) const DOCUMENTS_PATH: &str = "/org/freedesktop/portal/documents";

pub(crate) const FLATPAK_DESTINATION: &str = "org.freedesktop.portal.Flatpak";
pub(crate) const FLATPAK_PATH: &str = "/org/freedesktop/portal/Flatpak";

pub(crate) const FLATPAK_DEVELOPMENT_DESTINATION: &str = "org.freedesktop.Flatpak";
pub(crate) const FLATPAK_DEVELOPMENT_PATH: &str = "/org/freedesktop/Flatpak/Development";

static SESSION: OnceLock<zbus::Connection> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Proxy<'a> {
    inner: zbus::Proxy<'a>,
    version: u32,
}

impl<'a> Proxy<'a> {
    pub(crate) async fn connection() -> zbus::Result<zbus::Connection> {
        if let Some(cnx) = SESSION.get() {
            Ok(cnx.clone())
        } else {
            let cnx = zbus::Connection::session().await?;
            // during `await` another task may have initialized the cell
            Ok(SESSION.get_or_init(|| cnx).clone())
        }
    }

    pub async fn unique_name(
        connection: &zbus::Connection,
        prefix: &str,
        handle_token: &HandleToken,
    ) -> Result<ObjectPath<'static>, Error> {
        let unique_name = connection.unique_name().unwrap();
        let unique_identifier = unique_name.trim_start_matches(':').replace('.', "_");
        ObjectPath::try_from(format!("{prefix}/{unique_identifier}/{handle_token}"))
            .map_err(From::from)
    }

    pub async fn new<P>(
        interface: &'a str,
        path: P,
        destination: &'a str,
    ) -> Result<Proxy<'a>, Error>
    where
        P: TryInto<ObjectPath<'a>>,
        P::Error: Into<zbus::Error>,
    {
        let connection = Self::connection().await?;
        Self::with_connection(connection, interface, path, destination).await
    }

    pub async fn with_connection<P>(
        connection: zbus::Connection,
        interface: &'a str,
        path: P,
        destination: &'a str,
    ) -> Result<Proxy<'a>, Error>
    where
        P: TryInto<ObjectPath<'a>>,
        P::Error: Into<zbus::Error>,
    {
        let inner: zbus::Proxy = zbus::proxy::Builder::new(&connection)
            .interface(interface)?
            .path(path)?
            .destination(destination)?
            .build()
            .await?;

        let version = match inner
            .get_property::<u32>("version")
            .await
            .map_err(zbus::fdo::Error::from)
        {
            Ok(v) => Ok(v),
            Err(zbus::fdo::Error::InvalidArgs(details)) => {
                if details.contains(interface) {
                    Err(crate::Error::PortalNotFound(
                        // We are sure it is a valid interface name, should fix the type system
                        // here
                        zbus::names::OwnedInterfaceName::try_from(interface).unwrap(),
                    ))
                } else {
                    Ok(1)
                }
            }
            _ => Ok(1),
        }?;
        Ok(Self { inner, version })
    }

    pub async fn new_desktop_with_path<P>(
        connection: zbus::Connection,
        interface: &'a str,
        path: P,
    ) -> Result<Proxy<'a>, Error>
    where
        P: TryInto<ObjectPath<'a>>,
        P::Error: Into<zbus::Error>,
    {
        Self::with_connection(connection, interface, path, DESKTOP_DESTINATION).await
    }

    pub async fn new_desktop(interface: &'a str) -> Result<Proxy<'a>, Error> {
        Self::new(interface, DESKTOP_PATH, DESKTOP_DESTINATION).await
    }

    pub async fn new_desktop_with_connection(
        connection: zbus::Connection,
        interface: &'a str,
    ) -> Result<Proxy<'a>, Error> {
        Self::with_connection(connection, interface, DESKTOP_PATH, DESKTOP_DESTINATION).await
    }

    pub async fn new_documents(interface: &'a str) -> Result<Proxy<'a>, Error> {
        Self::new(interface, DOCUMENTS_PATH, DOCUMENTS_DESTINATION).await
    }

    pub async fn new_documents_with_connection(
        connection: zbus::Connection,
        interface: &'a str,
    ) -> Result<Proxy<'a>, Error> {
        Self::with_connection(connection, interface, DOCUMENTS_PATH, DOCUMENTS_DESTINATION).await
    }

    pub async fn new_flatpak(interface: &'a str) -> Result<Proxy<'a>, Error> {
        Self::new(interface, FLATPAK_PATH, FLATPAK_DESTINATION).await
    }

    pub async fn new_flatpak_with_connection(
        connection: zbus::Connection,
        interface: &'a str,
    ) -> Result<Proxy<'a>, Error> {
        Self::with_connection(connection, interface, FLATPAK_PATH, FLATPAK_DESTINATION).await
    }

    pub async fn new_flatpak_with_path<P>(
        connection: zbus::Connection,
        interface: &'a str,
        path: P,
    ) -> Result<Proxy<'a>, Error>
    where
        P: TryInto<ObjectPath<'a>>,
        P::Error: Into<zbus::Error>,
    {
        Self::with_connection(connection, interface, path, FLATPAK_DESTINATION).await
    }

    pub async fn new_flatpak_development(interface: &'a str) -> Result<Proxy<'a>, Error> {
        Self::new(
            interface,
            FLATPAK_DEVELOPMENT_PATH,
            FLATPAK_DEVELOPMENT_DESTINATION,
        )
        .await
    }

    pub async fn new_flatpak_development_with_connection(
        connection: zbus::Connection,
        interface: &'a str,
    ) -> Result<Proxy<'a>, Error> {
        Self::with_connection(
            connection,
            interface,
            FLATPAK_DEVELOPMENT_PATH,
            FLATPAK_DEVELOPMENT_DESTINATION,
        )
        .await
    }

    pub async fn request_versioned<T>(
        &self,
        handle_token: &HandleToken,
        method_name: &'static str,
        body: impl Serialize + Type + Debug,
        req_version: u32,
    ) -> Result<Request<T>, Error>
    where
        T: for<'de> Deserialize<'de> + Type + Debug,
    {
        let version = self.version();
        if version >= req_version {
            self.request(handle_token, method_name, body).await
        } else {
            Err(Error::RequiresVersion(req_version, version))
        }
    }

    pub async fn request<T>(
        &self,
        handle_token: &HandleToken,
        method_name: &'static str,
        body: impl Serialize + Type + Debug,
    ) -> Result<Request<T>, Error>
    where
        T: for<'de> Deserialize<'de> + Type + Debug,
    {
        let connection = self.inner.connection();
        let mut request = Request::from_unique_name(connection.clone(), handle_token).await?;
        futures_util::try_join!(request.prepare_response(), async {
            self.call_method(method_name, &body)
                .await
                .map_err::<PortalError, _>(From::from)
                .map_err(From::from)
        })?;
        Ok(request)
    }

    pub(crate) async fn empty_request(
        &self,
        handle_token: &HandleToken,
        method_name: &'static str,
        body: impl Serialize + Type + Debug,
    ) -> Result<Request<()>, Error> {
        self.request(handle_token, method_name, body).await
    }

    /// Returns the version of the interface
    pub fn version(&self) -> u32 {
        self.version
    }

    pub(crate) async fn call<R>(
        &self,
        method_name: &'static str,
        body: impl Serialize + Type + Debug,
    ) -> Result<R, Error>
    where
        R: for<'de> Deserialize<'de> + Type,
    {
        #[cfg(feature = "tracing")]
        {
            tracing::info!("Calling method {}:{}", self.interface(), method_name);
            tracing::debug!("With body {:#?}", body);
        }
        let msg = self
            .call_method(method_name, &body)
            .await
            .map_err::<PortalError, _>(From::from)?;
        let reply = msg.body().deserialize::<R>()?;

        Ok(reply)
    }

    pub(crate) async fn call_versioned<R>(
        &self,
        method_name: &'static str,
        body: impl Serialize + Type + Debug,
        req_version: u32,
    ) -> Result<R, Error>
    where
        R: for<'de> Deserialize<'de> + Type,
    {
        let version = self.version();
        if version >= req_version {
            self.call::<R>(method_name, body).await
        } else {
            Err(Error::RequiresVersion(req_version, version))
        }
    }

    pub async fn property<T>(&self, property_name: &'static str) -> Result<T, Error>
    where
        T: TryFrom<OwnedValue>,
        zbus::Error: From<<T as TryFrom<OwnedValue>>::Error>,
    {
        self.inner
            .get_property::<T>(property_name)
            .await
            .map_err(From::from)
    }

    pub(crate) async fn property_versioned<T>(
        &self,
        property_name: &'static str,
        req_version: u32,
    ) -> Result<T, Error>
    where
        T: TryFrom<OwnedValue>,
        zbus::Error: From<<T as TryFrom<OwnedValue>>::Error>,
    {
        let version = self.version();
        if version >= req_version {
            self.property::<T>(property_name).await
        } else {
            Err(Error::RequiresVersion(req_version, version))
        }
    }

    pub(crate) async fn signal_with_args<I>(
        &self,
        name: &'static str,
        args: &[(u8, &str)],
    ) -> Result<impl Stream<Item = I> + use<I>, Error>
    where
        I: for<'de> Deserialize<'de> + Type + Debug,
    {
        Ok(self
            .inner
            .receive_signal_with_args(name, args)
            .await?
            .filter_map({
                #[cfg(not(feature = "tracing"))]
                {
                    move |msg| ready(msg.body().deserialize().ok())
                }
                #[cfg(feature = "tracing")]
                {
                    let ifc = self.interface().to_owned();
                    move |msg| ready(trace_body(name, &ifc, msg))
                }
            }))
    }

    pub(crate) async fn signal<I>(&self, name: &'static str) -> Result<impl Stream<Item = I>, Error>
    where
        I: for<'de> Deserialize<'de> + Type + Debug,
    {
        Ok(self.inner.receive_signal(name).await?.filter_map({
            #[cfg(not(feature = "tracing"))]
            {
                move |msg| ready(msg.body().deserialize().ok())
            }
            #[cfg(feature = "tracing")]
            {
                let ifc = self.interface().to_owned();
                move |msg| ready(trace_body(name, &ifc, msg))
            }
        }))
    }
}

#[cfg(feature = "tracing")]
fn trace_body<I>(name: &'static str, ifc: &str, msg: Message) -> Option<I>
where
    I: for<'de> Deserialize<'de> + Type + Debug,
{
    tracing::info!("Received signal '{name}' on '{ifc}'");
    match msg.body().deserialize() {
        Ok(body) => {
            tracing::debug!("With body {body:#?}");
            Some(body)
        }
        Err(e) => {
            tracing::warn!("Error obtaining body: {e:#?}");
            None
        }
    }
}

impl<'a> Deref for Proxy<'a> {
    type Target = zbus::Proxy<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
