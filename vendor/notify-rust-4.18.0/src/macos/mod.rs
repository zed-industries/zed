/// `NSUserNotificationCenter` backend.
#[cfg(not(feature = "preview-macos-un"))]
mod nsusernotifications;

#[cfg(not(feature = "preview-macos-un"))]
pub(crate) use nsusernotifications::{schedule_notification, show_notification};

#[cfg(not(feature = "preview-macos-un"))]
pub use nsusernotifications::{
    ApplicationError, MacOsError, NotificationError, NotificationHandle,
};

// #[cfg(not(feature = "preview-macos-un"))]
#[cfg_attr(
    feature = "preview-macos-un",
    deprecated(note = "these functions have no effect in this configuration")
)]
pub use mac_notification_sys::{get_bundle_identifier_or_default, set_application};

/// `UNUserNotificationCenter` backend (opt-in via `preview-macos-un`).
#[cfg(feature = "preview-macos-un")]
mod unusernotifications;

#[cfg(feature = "preview-macos-un")]
pub(crate) use unusernotifications::{
    schedule_notification, show_notification, show_notification_async,
};

#[cfg(feature = "preview-macos-un")]
pub use unusernotifications::{MacOsError, NotificationHandle};

#[cfg(feature = "preview-macos-un")]
pub use mac_usernotifications::blocking::{
    get_notification_settings as get_notification_settings_blocking,
    request_auth as request_auth_blocking,
};
#[cfg(feature = "preview-macos-un")]
pub use mac_usernotifications::{check_bundle, get_notification_settings, request_auth};
