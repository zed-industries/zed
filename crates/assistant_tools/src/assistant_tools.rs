mod batch_tool;
mod code_action_tool;
mod code_symbols_tool;
mod copy_path_tool;
mod create_directory_tool;
mod create_file_tool;
mod delete_path_tool;
mod diagnostics_tool;
mod fetch_tool;
mod find_replace_file_tool;
mod list_directory_tool;
mod move_path_tool;
mod now_tool;
mod open_tool;
mod path_search_tool;
mod read_file_tool;
mod regex_search_tool;
mod rename_tool;
mod replace;
mod schema;
mod symbol_info_tool;
mod terminal_tool;
mod thinking_tool;

use std::sync::Arc;

use assistant_tool::ToolRegistry;
use copy_path_tool::CopyPathTool;
use gpui::App;
use http_client::HttpClientWithUrl;
use move_path_tool::MovePathTool;

use crate::batch_tool::BatchTool;
use crate::code_action_tool::CodeActionTool;
use crate::code_symbols_tool::CodeSymbolsTool;
use crate::create_directory_tool::CreateDirectoryTool;
use crate::create_file_tool::CreateFileTool;
use crate::delete_path_tool::DeletePathTool;
use crate::diagnostics_tool::DiagnosticsTool;
use crate::fetch_tool::FetchTool;
use crate::find_replace_file_tool::FindReplaceFileTool;
use crate::list_directory_tool::ListDirectoryTool;
use crate::now_tool::NowTool;
use crate::open_tool::OpenTool;
use crate::path_search_tool::PathSearchTool;
use crate::read_file_tool::ReadFileTool;
use crate::regex_search_tool::RegexSearchTool;
use crate::rename_tool::RenameTool;
use crate::symbol_info_tool::SymbolInfoTool;
use crate::terminal_tool::TerminalTool;
use crate::thinking_tool::ThinkingTool;

pub fn init(http_client: Arc<HttpClientWithUrl>, cx: &mut App) {
    assistant_tool::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(TerminalTool);
    registry.register_tool(BatchTool);
    registry.register_tool(CreateDirectoryTool);
    registry.register_tool(CreateFileTool);
    registry.register_tool(CopyPathTool);
    registry.register_tool(DeletePathTool);
    registry.register_tool(FindReplaceFileTool);
    registry.register_tool(SymbolInfoTool);
    registry.register_tool(CodeActionTool);
    registry.register_tool(MovePathTool);
    registry.register_tool(DiagnosticsTool);
    registry.register_tool(ListDirectoryTool);
    registry.register_tool(NowTool);
    registry.register_tool(OpenTool);
    registry.register_tool(CodeSymbolsTool);
    registry.register_tool(PathSearchTool);
    registry.register_tool(ReadFileTool);
    registry.register_tool(RegexSearchTool);
    registry.register_tool(RenameTool);
    registry.register_tool(ThinkingTool);
    registry.register_tool(FetchTool::new(http_client));
}
