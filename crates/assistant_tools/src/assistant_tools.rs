mod list_worktrees_tool;
mod now_tool;
mod read_file_tool;

use assistant_tool::ToolRegistry;
use gpui::App;

use crate::list_worktrees_tool::ListWorktreesTool;
use crate::now_tool::NowTool;
use crate::read_file_tool::ReadFileTool;

pub fn init(cx: &mut App) {
    assistant_tool::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(NowTool);
    registry.register_tool(ListWorktreesTool);
    registry.register_tool(ReadFileTool);
}
