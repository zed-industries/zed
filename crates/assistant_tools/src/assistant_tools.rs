mod now_tool;

use assistant_tool::ToolRegistry;
use gpui::AppContext;

use crate::now_tool::NowTool;

pub fn init(cx: &mut AppContext) {
    assistant_tool::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(NowTool);
}
