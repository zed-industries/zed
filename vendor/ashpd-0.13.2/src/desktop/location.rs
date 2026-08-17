//! # Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::location::{Accuracy, CreateSessionOptions, LocationProxy};
//! use futures_util::{FutureExt, StreamExt};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = LocationProxy::new().await?;
//!     let session = proxy
//!         .create_session(CreateSessionOptions::default().set_accuracy(Accuracy::Street))
//!         .await?;
//!     let mut stream = proxy.receive_location_updated().await?;
//!     let (_, location) = futures_util::join!(
//!         proxy
//!             .start(&session, None, Default::default())
//!             .map(|e| e.expect("Couldn't start session")),
//!         stream.next().map(|e| e.expect("Stream is exhausted"))
//!     );
//!     println!("{}", location.accuracy());
//!     println!("{}", location.longitude());
//!     println!("{}", location.latitude());
//!     session.close().await?;
//!     Ok(())
//! }
//! ```

use std::fmt::Debug;

use futures_util::Stream;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use zbus::zvariant::{
    ObjectPath, Optional, OwnedObjectPath, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request, Session, session::SessionPortal};
use crate::{Error, WindowIdentifier, proxy::Proxy};

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdLocationAccuracy"))]
#[derive(Serialize_repr, PartialEq, Eq, Clone, Copy, Debug, Type)]
#[doc(alias = "XdpLocationAccuracy")]
#[repr(u32)]
/// The accuracy of the location.
pub enum Accuracy {
    #[doc(alias = "XDP_LOCATION_ACCURACY_NONE")]
    /// None.
    None = 0,
    #[doc(alias = "XDP_LOCATION_ACCURACY_COUNTRY")]
    /// Country.
    Country = 1,
    #[doc(alias = "XDP_LOCATION_ACCURACY_CITY")]
    /// City.
    City = 2,
    #[doc(alias = "XDP_LOCATION_ACCURACY_NEIGHBORHOOD")]
    /// Neighborhood.
    Neighborhood = 3,
    #[doc(alias = "XDP_LOCATION_ACCURACY_STREET")]
    /// Street.
    Street = 4,
    #[doc(alias = "XDP_LOCATION_ACCURACY_EXACT")]
    /// The exact location.
    Exact = 5,
}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`LocationProxy::create_session`] request.
#[zvariant(signature = "dict")]
pub struct CreateSessionOptions {
    #[serde(with = "as_value")]
    session_handle_token: HandleToken,
    #[serde(
        rename = "distance-threshold",
        with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    distance_threshold: Option<u32>,
    #[serde(
        rename = "time-threshold",
        with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    time_threshold: Option<u32>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    accuracy: Option<Accuracy>,
}

impl CreateSessionOptions {
    /// Distance threshold in meters. Default is 0.
    pub fn set_distance_threshold(mut self, distance_threshold: impl Into<Option<u32>>) -> Self {
        self.distance_threshold = distance_threshold.into();
        self
    }

    /// Time threshold in seconds. Default is 0.
    pub fn set_time_threshold(mut self, time_threshold: impl Into<Option<u32>>) -> Self {
        self.time_threshold = time_threshold.into();
        self
    }

    /// Requested accuracy. Default is `Accuracy::Exact`.
    pub fn set_accuracy(mut self, accuracy: impl Into<Option<Accuracy>>) -> Self {
        self.accuracy = accuracy.into();
        self
    }
}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`LocationProxy::start`] request.
#[zvariant(signature = "dict")]
pub struct StartOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
}

#[derive(Deserialize, Type)]
/// The response received on a `location_updated` signal.
pub struct Location(OwnedObjectPath, LocationInner);

impl Location {
    /// The associated session.
    pub fn session_handle(&self) -> ObjectPath<'_> {
        self.0.as_ref()
    }

    /// The accuracy, in meters.
    pub fn accuracy(&self) -> f64 {
        self.1.accuracy
    }

    /// The altitude, in meters.
    pub fn altitude(&self) -> Option<f64> {
        if self.1.altitude == -f64::MAX {
            None
        } else {
            Some(self.1.altitude)
        }
    }

    /// The speed, in meters per second.
    pub fn speed(&self) -> Option<f64> {
        if self.1.speed == -1f64 {
            None
        } else {
            Some(self.1.speed)
        }
    }

    /// The heading, in degrees, going clockwise. North 0, East 90, South 180,
    /// West 270.
    pub fn heading(&self) -> Option<f64> {
        if self.1.heading == -1f64 {
            None
        } else {
            Some(self.1.heading)
        }
    }

    /// The location description.
    pub fn description(&self) -> Option<&str> {
        if self.1.description.is_empty() {
            None
        } else {
            Some(&self.1.description)
        }
    }

    /// The latitude, in degrees.
    pub fn latitude(&self) -> f64 {
        self.1.latitude
    }

    /// The longitude, in degrees.
    pub fn longitude(&self) -> f64 {
        self.1.longitude
    }

    /// The timestamp when the location was retrieved.
    pub fn timestamp(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.1.timestamp.0)
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Location")
            .field("accuracy", &self.accuracy())
            .field("altitude", &self.altitude())
            .field("speed", &self.speed())
            .field("heading", &self.heading())
            .field("description", &self.description())
            .field("latitude", &self.latitude())
            .field("longitude", &self.longitude())
            .field("timestamp", &self.timestamp())
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[zvariant(signature = "dict")]
#[serde(rename_all = "PascalCase")]
struct LocationInner {
    #[serde(with = "as_value")]
    accuracy: f64,
    #[serde(with = "as_value")]
    altitude: f64,
    #[serde(with = "as_value")]
    speed: f64,
    #[serde(with = "as_value")]
    heading: f64,
    #[serde(with = "as_value")]
    description: String,
    #[serde(with = "as_value")]
    latitude: f64,
    #[serde(with = "as_value")]
    longitude: f64,
    #[serde(with = "as_value")]
    timestamp: (u64, u64),
}

/// The interface lets sandboxed applications query basic information about the
/// location.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Location`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Location.html).
#[derive(Debug, Clone)]
#[doc(alias = "org.freedesktop.portal.Location")]
pub struct LocationProxy(Proxy<'static>);

impl LocationProxy {
    /// Create a new instance of [`LocationProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Location").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`LocationProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Location")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Signal emitted when the user location is updated.
    ///
    /// # Specifications
    ///
    /// See also [`LocationUpdated`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Location.html#org-freedesktop-portal-location-locationupdated).
    #[doc(alias = "LocationUpdated")]
    #[doc(alias = "XdpPortal::location-updated")]
    pub async fn receive_location_updated(
        &self,
    ) -> Result<impl Stream<Item = Location> + use<'_>, Error> {
        self.0.signal("LocationUpdated").await
    }

    /// Create a location session.
    ///
    /// # Arguments
    ///
    /// * `distance_threshold` - Sets the distance threshold in meters, default
    ///   to `0`.
    /// * `time_threshold` - Sets the time threshold in seconds, default to `0`.
    /// * `accuracy` - Sets the location accuracy, default to
    ///   [`Accuracy::Exact`].
    ///
    /// # Specifications
    ///
    /// See also [`CreateSession`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Location.html#org-freedesktop-portal-location-createsession).
    #[doc(alias = "CreateSession")]
    pub async fn create_session(
        &self,
        options: CreateSessionOptions,
    ) -> Result<Session<Self>, Error> {
        let (path, proxy) = futures_util::try_join!(
            self.0.call::<OwnedObjectPath>("CreateSession", &(options)),
            Session::from_unique_name(self.0.connection().clone(), &options.session_handle_token),
        )?;
        assert_eq!(proxy.path(), &path.into_inner());
        Ok(proxy)
    }

    /// Start the location session.
    /// An application can only attempt start a session once.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`LocationProxy::create_session`].
    /// * `identifier` - Identifier for the application window.
    ///
    /// # Specifications
    ///
    /// See also [`Start`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Location.html#org-freedesktop-portal-location-start).
    #[doc(alias = "Start")]
    #[doc(alias = "xdp_portal_location_monitor_start")]
    pub async fn start(
        &self,
        session: &Session<Self>,
        identifier: Option<&WindowIdentifier>,
        options: StartOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "Start",
                &(session, identifier, &options),
            )
            .await
    }
}

impl crate::Sealed for LocationProxy {}
impl SessionPortal for LocationProxy {}

impl std::ops::Deref for LocationProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
