mod handle_token;
pub(crate) mod request;
mod session;
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
#[cfg(feature = "backend")]
pub use self::handle_token::HandleToken;
#[cfg(not(feature = "backend"))]
pub(crate) use self::handle_token::HandleToken;
pub use self::{
    request::{Request, Response, ResponseError, ResponseType},
    session::{CreateSessionOptions, Session, SessionPortal},
};
#[cfg(any(feature = "screenshot", feature = "settings"))]
mod color;
#[cfg_attr(docsrs, doc(cfg(any(feature = "screenshot", feature = "settings",))))]
#[cfg(any(feature = "screenshot", feature = "settings",))]
pub use color::Color;

#[cfg(any(
    feature = "notification",
    feature = "dynamic_launcher",
    feature = "backend"
))]
mod icon;
#[cfg_attr(
    docsrs,
    doc(cfg(any(
        feature = "notification",
        feature = "dynamic_launcher",
        feature = "backend"
    )))
)]
#[cfg(any(
    feature = "notification",
    feature = "dynamic_launcher",
    feature = "backend"
))]
pub use icon::Icon;

#[cfg_attr(docsrs, doc(cfg(feature = "account")))]
#[cfg(feature = "account")]
pub mod account;
#[cfg_attr(docsrs, doc(cfg(feature = "background")))]
#[cfg(feature = "background")]
pub mod background;
#[cfg_attr(docsrs, doc(cfg(feature = "camera")))]
#[cfg(feature = "camera")]
pub mod camera;
#[cfg_attr(docsrs, doc(cfg(feature = "clipboard")))]
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg_attr(docsrs, doc(cfg(feature = "dynamic_launcher")))]
#[cfg(feature = "dynamic_launcher")]
pub mod dynamic_launcher;
#[cfg_attr(docsrs, doc(cfg(feature = "email")))]
#[cfg(feature = "email")]
pub mod email;
/// Open/save file(s) chooser.
#[cfg_attr(docsrs, doc(cfg(feature = "file_chooser")))]
#[cfg(feature = "file_chooser")]
pub mod file_chooser;
/// Enable/disable/query the status of Game Mode.
#[cfg_attr(docsrs, doc(cfg(feature = "game_mode")))]
#[cfg(feature = "game_mode")]
pub mod game_mode;
/// Register global shortcuts
#[cfg_attr(docsrs, doc(cfg(feature = "global_shortcuts")))]
#[cfg(feature = "global_shortcuts")]
pub mod global_shortcuts;
/// Inhibit the session from being restarted or the user from logging out.
#[cfg_attr(docsrs, doc(cfg(feature = "inhibit")))]
#[cfg(feature = "inhibit")]
pub mod inhibit;
/// Capture input events from physical or logical devices.
#[cfg_attr(docsrs, doc(cfg(feature = "input_capture")))]
#[cfg(feature = "input_capture")]
pub mod input_capture;
/// Query the user's GPS location.
#[cfg_attr(docsrs, doc(cfg(feature = "location")))]
#[cfg(feature = "location")]
pub mod location;
/// Monitor memory level.
#[cfg_attr(docsrs, doc(cfg(feature = "memory_monitor")))]
#[cfg(feature = "memory_monitor")]
pub mod memory_monitor;
/// Check the status of the network on a user's machine.
#[cfg_attr(docsrs, doc(cfg(feature = "network_monitor")))]
#[cfg(feature = "network_monitor")]
pub mod network_monitor;
/// Send/withdraw notifications.
#[cfg_attr(docsrs, doc(cfg(feature = "notification")))]
#[cfg(feature = "notification")]
pub mod notification;
#[cfg_attr(docsrs, doc(cfg(feature = "open_uri")))]
#[cfg(feature = "open_uri")]
pub mod open_uri;
/// Power profile monitoring.
#[cfg_attr(docsrs, doc(cfg(feature = "power_profile_monitor")))]
#[cfg(feature = "power_profile_monitor")]
pub mod power_profile_monitor;
/// Print a document.
#[cfg_attr(docsrs, doc(cfg(feature = "print")))]
#[cfg(feature = "print")]
pub mod print;
/// Proxy information.
#[cfg_attr(docsrs, doc(cfg(feature = "proxy_resolver")))]
#[cfg(feature = "proxy_resolver")]
pub mod proxy_resolver;
#[cfg_attr(docsrs, doc(cfg(feature = "realtime")))]
#[cfg(feature = "realtime")]
pub mod realtime;
/// Start a remote desktop session and interact with it.
#[cfg_attr(docsrs, doc(cfg(feature = "remote_desktop")))]
#[cfg(feature = "remote_desktop")]
pub mod remote_desktop;
#[cfg_attr(docsrs, doc(cfg(feature = "screencast")))]
#[cfg(feature = "screencast")]
pub mod screencast;
#[cfg_attr(docsrs, doc(cfg(feature = "screenshot")))]
#[cfg(feature = "screenshot")]
pub mod screenshot;
/// Retrieve a per-application secret used to encrypt confidential data inside
/// the sandbox.
#[cfg_attr(docsrs, doc(cfg(feature = "secret")))]
#[cfg(feature = "secret")]
pub mod secret;
/// Read & listen to system settings changes.
#[cfg_attr(docsrs, doc(cfg(feature = "settings")))]
#[cfg(feature = "settings")]
pub mod settings;
#[cfg_attr(docsrs, doc(cfg(feature = "trash")))]
#[cfg(feature = "trash")]
pub mod trash;
#[cfg_attr(docsrs, doc(cfg(feature = "usb")))]
#[cfg(feature = "usb")]
pub mod usb;
#[cfg_attr(docsrs, doc(cfg(feature = "wallpaper")))]
#[cfg(feature = "wallpaper")]
pub mod wallpaper;

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdPersistMode"))]
#[derive(
    Default,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr,
    PartialEq,
    Eq,
    Debug,
    Copy,
    Clone,
    zbus::zvariant::Type,
)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "screencast", feature = "remote_desktop")))
)]
#[cfg(any(feature = "screencast", feature = "remote_desktop"))]
#[doc(alias = "XdpPersistMode")]
#[repr(u32)]
/// Persistence mode for a screencast or remote desktop session.
pub enum PersistMode {
    #[doc(alias = "XDP_PERSIST_MODE_NONE")]
    #[default]
    /// Do not persist.
    DoNot = 0,
    #[doc(alias = "XDP_PERSIST_MODE_TRANSIENT")]
    /// Persist while the application is running.
    Application = 1,
    #[doc(alias = "XDP_PERSIST_MODE_PERSISTENT")]
    /// Persist until explicitly revoked.
    ExplicitlyRevoked = 2,
}
