mod batch_tool;
mod code_action_tool;
mod code_symbols_tool;
mod contents_tool;
mod copy_path_tool;
mod create_directory_tool;
mod create_file_tool;
mod delete_path_tool;
mod diagnostics_tool;
mod edit_file_tool;
mod fetch_tool;
mod grep_tool;
mod list_directory_tool;
mod move_path_tool;
mod now_tool;
mod open_tool;
mod path_search_tool;
mod read_file_tool;
mod rename_tool;
mod replace;
mod schema;
mod symbol_info_tool;
mod terminal_tool;
mod thinking_tool;
mod web_search_tool;

use std::sync::Arc;

use assistant_tool::ToolRegistry;
use copy_path_tool::CopyPathTool;
use feature_flags::FeatureFlagAppExt;
use gpui::App;
use http_client::HttpClientWithUrl;
use move_path_tool::MovePathTool;
use web_search_tool::WebSearchTool;

use crate::batch_tool::BatchTool;
use crate::code_action_tool::CodeActionTool;
use crate::code_symbols_tool::CodeSymbolsTool;
use crate::contents_tool::ContentsTool;
use crate::create_directory_tool::CreateDirectoryTool;
use crate::create_file_tool::CreateFileTool;
use crate::delete_path_tool::DeletePathTool;
use crate::diagnostics_tool::DiagnosticsTool;
use crate::edit_file_tool::EditFileTool;
use crate::fetch_tool::FetchTool;
use crate::grep_tool::GrepTool;
use crate::list_directory_tool::ListDirectoryTool;
use crate::now_tool::NowTool;
use crate::open_tool::OpenTool;
use crate::path_search_tool::PathSearchTool;
use crate::read_file_tool::ReadFileTool;
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
    registry.register_tool(EditFileTool);
    registry.register_tool(SymbolInfoTool);
    registry.register_tool(CodeActionTool);
    registry.register_tool(MovePathTool);
    registry.register_tool(DiagnosticsTool);
    registry.register_tool(ListDirectoryTool);
    registry.register_tool(NowTool);
    registry.register_tool(OpenTool);
    registry.register_tool(CodeSymbolsTool);
    registry.register_tool(ContentsTool);
    registry.register_tool(PathSearchTool);
    registry.register_tool(ReadFileTool);
    registry.register_tool(GrepTool);
    registry.register_tool(RenameTool);
    registry.register_tool(ThinkingTool);
    registry.register_tool(FetchTool::new(http_client));

    cx.observe_flag::<feature_flags::ZedProWebSearchTool, _>({
        move |is_enabled, cx| {
            if is_enabled {
                ToolRegistry::global(cx).register_tool(WebSearchTool);
            } else {
                ToolRegistry::global(cx).unregister_tool(WebSearchTool);
            }
        }
    })
    .detach();
}

#[cfg(test)]
mod tests {
    use http_client::FakeHttpClient;

    use super::*;

    #[gpui::test]
    fn test_builtin_tool_schema_compatibility(cx: &mut App) {
        crate::init(
            Arc::new(http_client::HttpClientWithUrl::new(
                FakeHttpClient::with_200_response(),
                "https://zed.dev",
                None,
            )),
            cx,
        );

        for tool in ToolRegistry::global(cx).tools() {
            let actual_schema = tool
                .input_schema(language_model::LanguageModelToolSchemaFormat::JsonSchemaSubset)
                .unwrap();
            let mut expected_schema = actual_schema.clone();
            assistant_tool::adapt_schema_to_format(
                &mut expected_schema,
                language_model::LanguageModelToolSchemaFormat::JsonSchemaSubset,
            )
            .unwrap();

            let error_message = format!(
                "Tool schema for `{}` is not compatible with `language_model::LanguageModelToolSchemaFormat::JsonSchemaSubset` (Gemini Models).\n\
                Are you using `schema::json_schema_for<T>(format)` to generate the schema?",
                tool.name(),
            );

            assert_eq!(actual_schema, expected_schema, "{}", error_message)
        }
    }
}
