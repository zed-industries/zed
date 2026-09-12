//! System notification types shared by `gpui` and its platform backends.

use gpui_shared_string::SharedString;

/// A notification posted to the operating system's notification center,
/// rather than rendered as in-app UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemNotification {
    /// Stable identity for the notification. Posting a new notification with
    /// the same tag replaces the previous one where the platform supports it,
    /// and responses carry the tag back to the application.
    pub tag: SharedString,
    /// The notification's headline.
    pub title: SharedString,
    /// Additional text displayed below the title.
    pub body: SharedString,
    /// Buttons offered on the notification. Platforms that cannot display
    /// action buttons show the notification without them.
    pub actions: Vec<SystemNotificationAction>,
}

/// A button offered on a [`SystemNotification`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SystemNotificationAction {
    /// Identifies the action in [`SystemNotificationResponse::action_id`]
    /// when the user presses this button.
    pub id: SharedString,
    /// The button's user-visible label.
    pub label: SharedString,
}

/// The user's activation of a [`SystemNotification`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemNotificationResponse {
    /// The [`SystemNotification::tag`] of the activated notification.
    pub tag: SharedString,
    /// The pressed action button's [`SystemNotificationAction::id`], or
    /// `None` when the user activated the notification body itself.
    pub action_id: Option<SharedString>,
}
