use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use gpui::{App, SharedString, Window};
use ui::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Info,
}

impl ErrorSeverity {
    pub fn auto_dismiss_delay(&self) -> Option<Duration> {
        match self {
            ErrorSeverity::Error => None,
            ErrorSeverity::Warning => Some(Duration::from_secs(10)),
            ErrorSeverity::Info => Some(Duration::from_secs(5)),
        }
    }
}

pub struct ErrorAction {
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub tooltip: Option<SharedString>,
    pub handler: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl ErrorAction {
    pub fn new(
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
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
        ErrorSeverity::Error
    }

    fn primary_action(&self) -> Option<ErrorAction> {
        Some(ErrorAction::link(
            "See docs",
            "https://zed.dev/docs/linux#i-cant-open-any-files",
        ))
    }
}

pub struct AnyhowWorkspaceError {
    error: anyhow::Error,
}

impl AnyhowWorkspaceError {
    pub fn new(error: anyhow::Error) -> Self {
        Self { error }
    }
}

impl From<anyhow::Error> for AnyhowWorkspaceError {
    fn from(error: anyhow::Error) -> Self {
        Self::new(error)
    }
}

impl WorkspaceError for AnyhowWorkspaceError {
    fn primary_message(&self) -> SharedString {
        format!("{}", self.error).into()
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Error
    }
}

pub struct StringWorkspaceError {
    message: String,
}

impl StringWorkspaceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<String> for StringWorkspaceError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl WorkspaceError for StringWorkspaceError {
    fn primary_message(&self) -> SharedString {
        self.message.clone().into()
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Error
    }
}

pub struct DisplayWorkspaceError {
    message: String,
}

impl DisplayWorkspaceError {
    pub fn new<E: fmt::Display + fmt::Debug>(error: &E) -> Self {
        Self {
            message: format!("{}", error),
        }
    }

    pub fn new_with_prefix<E: fmt::Display + fmt::Debug>(error: &E) -> Self {
        Self {
            message: format!("Error: {}", error),
        }
    }
}

impl WorkspaceError for DisplayWorkspaceError {
    fn primary_message(&self) -> SharedString {
        self.message.clone().into()
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Error
    }
}
