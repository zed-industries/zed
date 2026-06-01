use std::{sync::Arc, time::Duration};

use gpui::{Action, SharedString};
use ui::IconName;
use zed_actions::OpenBrowser;

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

/// The behavior triggered when the user invokes an [`ErrorAction`].
pub enum ErrorActionHandler {
    /// Run the provided callback when the action is invoked.
    /// The notification is still dismissed afterwards by the button's click handler.
    Action(Box<dyn Action>),
    /// Dismiss the notification without running any extra logic.
    Dismiss,
}

pub struct ErrorAction {
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub tooltip: Option<SharedString>,
    pub handler: ErrorActionHandler,
}

impl ErrorAction {
    pub fn new<A: Action + 'static>(label: impl Into<SharedString>, handler: A) -> Self {
        Self {
            label: label.into(),
            icon: None,
            tooltip: None,
            handler: ErrorActionHandler::Action(Box::new(handler)),
        }
    }

    /// Creates a dismiss-only action labelled "Dismiss".
    ///
    /// Useful as a sensible default for [`WorkspaceError::primary_action`] when the error has no
    /// recovery affordance beyond closing the notification.
    pub fn dismiss() -> Self {
        Self {
            label: "Dismiss".into(),
            icon: None,
            tooltip: None,
            handler: ErrorActionHandler::Dismiss,
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
        Self::new(label, OpenBrowser { url: url.into() }).with_icon(IconName::ArrowUpRight)
    }
}

pub trait WorkspaceError {
    fn primary_message(&self) -> SharedString;

    fn secondary_message(&self) -> Option<SharedString> {
        None
    }

    /// The primary action shown in the error notification.
    ///
    /// If in doubt, use [`ErrorAction::dismiss`].
    fn primary_action(&self) -> ErrorAction;

    fn secondary_action(&self) -> Option<ErrorAction> {
        None
    }

    fn severity(&self) -> ErrorSeverity;
}

impl WorkspaceError for String {
    fn primary_message(&self) -> SharedString {
        self.clone().into()
    }

    fn primary_action(&self) -> ErrorAction {
        ErrorAction::dismiss()
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Error
    }
}

impl WorkspaceError for anyhow::Error {
    fn primary_message(&self) -> SharedString {
        format!("{self}").into()
    }

    fn primary_action(&self) -> ErrorAction {
        ErrorAction::dismiss()
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

    fn primary_action(&self) -> ErrorAction {
        ErrorAction::link(
            "See docs",
            "https://zed.dev/docs/linux#i-cant-open-any-files",
        )
    }
}
