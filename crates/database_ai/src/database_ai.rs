mod mcp_server_manager;
mod mcp_tools;
mod tools;

pub use tools::{
    DescribeObjectTool, ExecuteQueryTool, ExplainQueryTool, GetSchemaTool, ListObjectsTool,
    ModifyDataTool,
};

use agent::Thread;
use gpui::App;

pub fn init(cx: &mut App) {
    cx.observe_new(|thread: &mut Thread, _window, _cx| {
        thread.add_tool(ExecuteQueryTool);
        thread.add_tool(DescribeObjectTool);
        thread.add_tool(ListObjectsTool);
        thread.add_tool(ExplainQueryTool);
        thread.add_tool(ModifyDataTool);
        thread.add_tool(GetSchemaTool);
    })
    .detach();

    let _mcp_manager = mcp_server_manager::DatabaseMcpServerManager::start(cx);
    // Leak the entity to keep it alive for the lifetime of the app
    std::mem::forget(_mcp_manager);
}
