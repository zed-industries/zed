//! The notification API allows extensions to show notifications to users.

use crate::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Notification severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum NotificationLevel {
    /// Informational message
    Info,
    /// Warning message 
    Warning,
    /// Error message
    Error,
}

/// A notification that can be displayed to the user
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Notification {
    /// The notification message
    pub message: String,
    /// The notification title (optional)
    pub title: Option<String>,
    /// The severity level of the notification
    pub level: NotificationLevel,
    /// URL to more information (optional)
    pub link_url: Option<String>,
    /// Text for the link button (optional, used with link_url)
    pub link_text: Option<String>,
}

impl Notification {
    /// Creates a new informational notification
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            level: NotificationLevel::Info,
            link_url: None,
            link_text: None,
        }
    }

    /// Creates a new warning notification
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            level: NotificationLevel::Warning,
            link_url: None,
            link_text: None,
        }
    }

    /// Creates a new error notification
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            level: NotificationLevel::Error,
            link_url: None,
            link_text: None,
        }
    }

    /// Sets the notification title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets a link for more information
    pub fn with_link(mut self, text: impl Into<String>, url: impl Into<String>) -> Self {
        self.link_text = Some(text.into());
        self.link_url = Some(url.into());
        self
    }
    
    /// Shows the notification in the UI
    pub fn show(&self) -> Result<()> {
        show_notification(self)
    }
}

/// Shows a notification in the UI
pub fn show_notification(notification: &Notification) -> Result<()> {
    let level_str = match notification.level {
        NotificationLevel::Info => "info",
        NotificationLevel::Warning => "warning",
        NotificationLevel::Error => "error",
    };
    
    // Convert the notification to JSON
    let notification_json = serde_json::to_string(notification)
        .map_err(|e| format!("Failed to serialize notification: {}", e))?;

    // Call the internal API to show the notification
    crate::show_notification(level_str, &notification_json)
} 