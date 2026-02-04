mod config_options;
mod entry_view_state;
mod message_editor;
mod mode_selector;
mod model_selector;
mod model_selector_popover;
mod thread_history;
mod thread_view;

use gpui::SharedString;
use std::sync::Arc;
use workspace::WorkspaceId;

pub use mode_selector::ModeSelector;
pub use model_selector::AcpModelSelector;
pub use model_selector_popover::AcpModelSelectorPopover;
pub use thread_history::*;
pub use thread_view::AcpServerView;

#[derive(Debug, Clone)]
pub enum ThreadActivityEvent {
    MessageSent {
        thread_id: Arc<str>,
        title: SharedString,
        workspace_id: Option<WorkspaceId>,
        worktree_paths: Vec<String>,
    },
    Stopped {
        thread_id: Arc<str>,
    },
    Updated {
        thread_id: Arc<str>,
    },
    Deleted {
        thread_id: Arc<str>,
    },
    DeletedAll,
    TitleChanged {
        thread_id: Arc<str>,
        title: SharedString,
    },
}
