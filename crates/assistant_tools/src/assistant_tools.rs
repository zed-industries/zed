mod bash_tool;
mod copy_path_tool;
mod create_directory_tool;
mod create_file_tool;
mod delete_path_tool;
mod diagnostics_tool;
mod edit_files_tool;
mod fetch_tool;
mod find_replace_file_tool;
mod list_directory_tool;
mod move_path_tool;
mod now_tool;
mod path_search_tool;
mod read_file_tool;
mod regex_search_tool;
mod replace;
mod thinking_tool;

use std::sync::Arc;

use assistant_tool::ToolRegistry;
use copy_path_tool::CopyPathTool;
use gpui::App;
use http_client::HttpClientWithUrl;
use move_path_tool::MovePathTool;

use crate::bash_tool::BashTool;
use crate::create_directory_tool::CreateDirectoryTool;
use crate::create_file_tool::CreateFileTool;
use crate::delete_path_tool::DeletePathTool;
use crate::diagnostics_tool::DiagnosticsTool;
use crate::edit_files_tool::EditFilesTool;
use crate::fetch_tool::FetchTool;
use crate::find_replace_file_tool::FindReplaceFileTool;
use crate::list_directory_tool::ListDirectoryTool;
use crate::now_tool::NowTool;
use crate::path_search_tool::PathSearchTool;
use crate::read_file_tool::ReadFileTool;
use crate::regex_search_tool::RegexSearchTool;
use crate::thinking_tool::ThinkingTool;

pub fn init(http_client: Arc<HttpClientWithUrl>, cx: &mut App) {
    assistant_tool::init(cx);
    crate::edit_files_tool::log::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(BashTool);
    registry.register_tool(CreateDirectoryTool);
    registry.register_tool(CreateFileTool);
    registry.register_tool(CopyPathTool);
    registry.register_tool(DeletePathTool);
    registry.register_tool(FindReplaceFileTool);
    registry.register_tool(MovePathTool);
    registry.register_tool(DiagnosticsTool);
    registry.register_tool(EditFilesTool);
    registry.register_tool(ListDirectoryTool);
    registry.register_tool(NowTool);
    registry.register_tool(PathSearchTool);
    registry.register_tool(ReadFileTool);
    registry.register_tool(RegexSearchTool);
    registry.register_tool(ThinkingTool);
    registry.register_tool(FetchTool::new(http_client));
}
