use anyhow::Result;
use gpui::{AsyncApp, App};
use serde::{Deserialize, Serialize};

/// Notification severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationLevel {
    /// Informational message
    Info,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

/// A notification that can be displayed to the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionNotification {
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

impl ExtensionNotification {
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
    pub fn show(&self, cx: &AsyncApp) -> Result<()> {
        show_notification(self, cx)
    }

    /// Shows the notification in the UI
    pub fn show_in_app(&self, cx: &mut App) -> Result<()> {
        // Create a sync version that converts to async context
        let notification = self.clone();
        let async_cx = cx.to_async();
        
        cx.spawn(async move |_| {
            let _ = notification.show(&async_cx);
        }).detach();
        
        Ok(())
    }
}

/// Shows a notification in the UI
pub fn show_notification(notification: &ExtensionNotification, cx: &AsyncApp) -> Result<()> {
    use anyhow::anyhow;
    
    // We need a structured approach to display the notification
    let notification = notification.clone();
    
    cx.update(|cx| {
        // Import workspace notification components
        use workspace::notifications::{NotificationId, show_app_notification, simple_message_notification::MessageNotification};
        use gpui::prelude::*;
        use ui::{IconName, Color};
        
        // Define a unique notification ID for extension notifications
        struct ExtensionNotificationId;
        
        let notification_id = NotificationId::unique::<ExtensionNotificationId>();
        let message = notification.message.clone();
        
        // Determine icon and color based on level
        let (icon, color) = match notification.level {
            NotificationLevel::Info => (IconName::Info, Color::Info),
            NotificationLevel::Warning => (IconName::Warning, Color::Warning),
            NotificationLevel::Error => (IconName::Alert, Color::Error),
        };
        
        // Create title (use default if not provided)
        let title = notification.title.unwrap_or_else(|| {
            match notification.level {
                NotificationLevel::Info => "Information",
                NotificationLevel::Warning => "Warning",
                NotificationLevel::Error => "Error",
            }.to_string()
        });
        
        // Build the notification
        let mut message_notification = MessageNotification::new(message, cx)
            .primary_message(title)
            .primary_icon(icon)
            .primary_icon_color(color)
            .show_close_button(true);
            
        // Add link if provided
        if let (Some(url), Some(text)) = (notification.link_url.as_ref(), notification.link_text.as_ref()) {
            message_notification = message_notification.more_info_message(text.clone()).more_info_url(url.clone());
        }
        
        // Show the notification
        show_app_notification(notification_id, cx, move |cx| {
            cx.new(move |cx| message_notification)
        });
    }).map_err(|e| anyhow!("Failed to show notification: {}", e))
} 