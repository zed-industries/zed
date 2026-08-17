pub(crate) const DESTINATION: zbus::names::BusName<'static> = zbus::names::BusName::WellKnown(
    zbus::names::WellKnownName::from_static_str_unchecked("org.freedesktop.secrets"),
);
pub(crate) const PATH: zbus::zvariant::ObjectPath<'static> =
    zbus::zvariant::ObjectPath::from_static_str_unchecked("/org/freedesktop/secrets");

/// A common trait implemented by objects that can be
/// locked or unlocked. Like [`Collection`] or [`Item`].
pub trait Unlockable: serde::Serialize + zbus::zvariant::Type {}

impl Unlockable for zbus::zvariant::ObjectPath<'_> {}
impl Unlockable for zbus::zvariant::OwnedObjectPath {}
impl Unlockable for &zbus::zvariant::ObjectPath<'_> {}
impl Unlockable for &zbus::zvariant::OwnedObjectPath {}

mod collection;
mod item;
mod prompt;
mod properties;
mod secret;
mod service;
mod session;

pub use collection::Collection;
pub use item::Item;
pub use prompt::Prompt;
#[cfg(not(feature = "unstable"))]
pub(crate) use properties::Properties;
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use properties::Properties;
pub use secret::DBusSecret;
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use secret::DBusSecretInner;
pub use service::Service;
pub use session::Session;
