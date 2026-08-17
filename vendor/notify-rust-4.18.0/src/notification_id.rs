/// A platform-independent notification identifier.
///
/// On XDG (Linux/BSD) notifications are identified by a server-assigned `u32`.
/// On macOS (`preview-macos-un`) they use a caller-supplied `String`.
///
/// Both variants implement `From`, so you can pass either type directly to
/// [`Notification::id`](crate::Notification::id):
///
/// ```no_run
/// # use notify_rust::Notification;
/// // XDG — pass a u32
/// # #[cfg(all(unix, not(target_os = "macos")))]
/// Notification::new().id(42u32).show().unwrap();
///
/// // macOS — pass a &str or String
/// # #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
/// Notification::new().id("my-app.status").show().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationId {
    /// XDG / D-Bus numeric identifier (Linux, BSD).
    Xdg(u32),

    /// macOS `UNNotificationRequest` string identifier.
    Mac(String),
}

impl From<u32> for NotificationId {
    fn from(value: u32) -> Self {
        NotificationId::Xdg(value)
    }
}

impl From<String> for NotificationId {
    fn from(value: String) -> Self {
        NotificationId::Mac(value)
    }
}

impl From<&str> for NotificationId {
    fn from(value: &str) -> Self {
        NotificationId::Mac(value.to_owned())
    }
}
