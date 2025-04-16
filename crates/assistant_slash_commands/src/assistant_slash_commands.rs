mod cargo_workspace_command;
mod context_server_command;
mod default_command;
mod delta_command;
mod diagnostics_command;
mod docs_command;
mod fetch_command;
mod file_command;
mod now_command;
mod prompt_command;
mod selection_command;
mod streaming_example_command;
mod symbols_command;
mod tab_command;
mod terminal_command;

use gpui::App;
use language::{CodeLabel, HighlightId};
use ui::ActiveTheme as _;

pub use crate::cargo_workspace_command::*;
pub use crate::context_server_command::*;
pub use crate::default_command::*;
pub use crate::delta_command::*;
pub use crate::diagnostics_command::*;
pub use crate::docs_command::*;
pub use crate::fetch_command::*;
pub use crate::file_command::*;
pub use crate::now_command::*;
pub use crate::prompt_command::*;
pub use crate::selection_command::*;
pub use crate::streaming_example_command::*;
pub use crate::symbols_command::*;
pub use crate::tab_command::*;
pub use crate::terminal_command::*;

pub fn create_label_for_command(command_name: &str, arguments: &[&str], cx: &App) -> CodeLabel {
    let mut label = CodeLabel::default();
    label.push_str(command_name, None);
    label.push_str(" ", None);
    label.push_str(
        &arguments.join(" "),
        cx.theme().syntax().highlight_id("comment").map(HighlightId),
    );
    label.filter_range = 0..command_name.len();
    label
}
