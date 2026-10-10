//! cedian inside the Zed fork (S9a spike): the dock panel that talks to OMP
//! through the cedian headless crates, and the agent-edit import.

pub mod browser;
mod context;
pub mod dialogs;
pub mod import;
pub mod instruments;
pub mod omp_link;
pub mod omp_settings;
pub mod panel;
pub mod review;
pub mod workflow_view;

pub use cedian_omp::{Toast, ToastLevel};
pub use omp_link::SessionSetting;
pub use omp_settings::OmpSettings;
pub use panel::{
    BROWSER_GATE, CedianPanel, Connection, DIALOG_TIMEOUT, InlineEdit, ModelPicker, Onboarding,
    RevertTurn, TASK_ID, ToggleFocus, Turn, init,
};
