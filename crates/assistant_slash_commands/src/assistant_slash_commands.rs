pub mod auto_command;
pub mod cargo_workspace_command;
pub mod context_server_command;
pub mod default_command;
pub mod delta_command;
pub mod diagnostics_command;
pub mod docs_command;
pub mod fetch_command;
pub mod file_command;
pub mod now_command;
pub mod project_command;
pub mod prompt_command;
pub mod search_command;
pub mod selection_command;
pub mod streaming_example_command;
pub mod symbols_command;
pub mod tab_command;
pub mod terminal_command;

use gpui::AppContext;
use language::{CodeLabel, HighlightId};
use ui::ActiveTheme as _;

pub fn create_label_for_command(
    command_name: &str,
    arguments: &[&str],
    cx: &AppContext,
) -> CodeLabel {
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
