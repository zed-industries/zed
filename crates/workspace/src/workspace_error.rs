use std::sync::Arc;
use std::time::Duration;

use gpui::{SharedString, Window};
use ui::{Context, IconName};

use crate::notifications::simple_message_notification::MessageNotification;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Critical,
    Error,
    Warning,
}

impl ErrorSeverity {
    pub fn auto_dismiss_delay(&self) -> Option<Duration> {
        match self {
            ErrorSeverity::Critical => None,
            ErrorSeverity::Error => Some(Duration::from_secs(20)),
            ErrorSeverity::Warning => Some(Duration::from_secs(10)),
        }
    }
}

pub struct ErrorAction {
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub tooltip: Option<SharedString>,
    pub handler: Arc<dyn Fn(&mut Window, &mut Context<'_, MessageNotification>) + 'static>,
}

impl ErrorAction {
    pub fn new(
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut Context<'_, MessageNotification>) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            icon: None,
            tooltip: None,
            handler: Arc::new(handler),
        }
    }

    pub fn with_icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn link(label: impl Into<SharedString>, url: impl Into<Arc<str>>) -> Self {
        let url = url.into();
        Self::new(label, move |_window, cx| {
            cx.open_url(&url);
        })
        .with_icon(IconName::ArrowUpRight)
    }
}

pub trait WorkspaceError {
    fn primary_message(&self) -> SharedString;

    fn secondary_message(&self) -> Option<SharedString> {
        None
    }

    fn primary_action(&self) -> Option<ErrorAction> {
        None
    }

    fn secondary_action(&self) -> Option<ErrorAction> {
        None
    }

    fn severity(&self) -> ErrorSeverity;
}

impl WorkspaceError for String {
    fn primary_message(&self) -> SharedString {
        self.clone().into()
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Error
    }
}

impl WorkspaceError for anyhow::Error {
    fn primary_message(&self) -> SharedString {
        format!("{self}").into()
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Critical
    }
}

pub struct PortalError {
    message: String,
}

impl PortalError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl WorkspaceError for PortalError {
    fn primary_message(&self) -> SharedString {
        self.message.clone().into()
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Critical
    }

    fn primary_action(&self) -> Option<ErrorAction> {
        Some(ErrorAction::link(
            "See docs",
            "https://zed.dev/docs/linux#i-cant-open-any-files",
        ))
    }
}
