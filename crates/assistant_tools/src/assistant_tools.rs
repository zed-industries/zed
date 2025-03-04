mod now_tool;

use assistant_tool::ToolRegistry;
use gpui::App;

use crate::now_tool::NowTool;

pub fn init(cx: &mut App) {
    assistant_tool::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(NowTool);
}
