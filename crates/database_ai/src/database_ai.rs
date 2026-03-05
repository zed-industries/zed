pub mod autonomous_agent;
pub mod mention_provider;
pub mod plan_analyzer;
pub mod query_generator;
pub mod query_optimizer;
pub mod schema_context;
pub mod slash_commands;
mod mcp_server_manager;
mod mcp_tools;
mod tools;
#[cfg(test)]
mod tests;

pub use tools::{
    DescribeObjectTool, ExecuteQueryTool, ExplainQueryTool, GetSchemaTool, ListObjectsTool,
    ModifyDataTool,
};

use agent::Thread;
use assistant_slash_command::SlashCommandRegistry;
use gpui::App;
use slash_commands::{DbExplainSlashCommand, DbQuerySlashCommand, DbSchemaSlashCommand};

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

    let slash_registry = SlashCommandRegistry::default_global(cx);
    slash_registry.register_command(DbSchemaSlashCommand, true);
    slash_registry.register_command(DbQuerySlashCommand, true);
    slash_registry.register_command(DbExplainSlashCommand, true);

    let _mcp_manager = mcp_server_manager::DatabaseMcpServerManager::start(cx);
    // Leak the entity to keep it alive for the lifetime of the app
    std::mem::forget(_mcp_manager);
}
