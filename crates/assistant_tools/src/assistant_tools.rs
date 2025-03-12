mod edit_files_tool;
mod list_directory_tool;
mod now_tool;
mod read_file_tool;
mod regex_search;

use assistant_tool::ToolRegistry;
use gpui::App;

use crate::edit_files_tool::EditFilesTool;
use crate::list_directory_tool::ListDirectoryTool;
use crate::now_tool::NowTool;
use crate::read_file_tool::ReadFileTool;
use crate::regex_search::RegexSearchTool;

pub fn init(cx: &mut App) {
    assistant_tool::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(NowTool);
    registry.register_tool(ReadFileTool);
    registry.register_tool(ListDirectoryTool);
    registry.register_tool(EditFilesTool);
    registry.register_tool(RegexSearchTool);
}
