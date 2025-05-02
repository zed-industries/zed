mod batch_tool;
mod code_action_tool;
mod code_symbols_tool;
mod contents_tool;
mod copy_path_tool;
mod create_directory_tool;
mod create_file_tool;
mod delete_path_tool;
mod diagnostics_tool;
mod edit_agent;
mod edit_file_tool;
mod fetch_tool;
mod find_path_tool;
mod grep_tool;
mod list_directory_tool;
mod move_path_tool;
mod now_tool;
mod open_tool;
mod read_file_tool;
mod rename_tool;
mod replace;
mod schema;
mod streaming_edit_file_tool;
mod symbol_info_tool;
mod templates;
mod terminal_tool;
mod thinking_tool;
mod ui;
mod web_search_tool;

use std::sync::Arc;

use assistant_settings::AssistantSettings;
use assistant_tool::ToolRegistry;
use copy_path_tool::CopyPathTool;
use feature_flags::{AgentStreamEditsFeatureFlag, FeatureFlagAppExt};
use gpui::App;
use http_client::HttpClientWithUrl;
use language_model::LanguageModelRegistry;
use move_path_tool::MovePathTool;
use settings::{Settings, SettingsStore};
use web_search_tool::WebSearchTool;

pub(crate) use templates::*;

use crate::batch_tool::BatchTool;
use crate::code_action_tool::CodeActionTool;
use crate::code_symbols_tool::CodeSymbolsTool;
use crate::contents_tool::ContentsTool;
use crate::create_directory_tool::CreateDirectoryTool;
use crate::delete_path_tool::DeletePathTool;
use crate::diagnostics_tool::DiagnosticsTool;
use crate::fetch_tool::FetchTool;
use crate::find_path_tool::FindPathTool;
use crate::grep_tool::GrepTool;
use crate::list_directory_tool::ListDirectoryTool;
use crate::now_tool::NowTool;
use crate::read_file_tool::ReadFileTool;
use crate::rename_tool::RenameTool;
use crate::streaming_edit_file_tool::StreamingEditFileTool;
use crate::symbol_info_tool::SymbolInfoTool;
use crate::thinking_tool::ThinkingTool;

pub use create_file_tool::{CreateFileTool, CreateFileToolInput};
pub use edit_file_tool::{EditFileTool, EditFileToolInput};
pub use find_path_tool::FindPathToolInput;
pub use open_tool::OpenTool;
pub use read_file_tool::ReadFileToolInput;
pub use terminal_tool::TerminalTool;

pub fn init(http_client: Arc<HttpClientWithUrl>, cx: &mut App) {
    assistant_tool::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(TerminalTool);
    registry.register_tool(BatchTool);
    registry.register_tool(CreateDirectoryTool);
    registry.register_tool(CopyPathTool);
    registry.register_tool(DeletePathTool);
    registry.register_tool(SymbolInfoTool);
    registry.register_tool(CodeActionTool);
    registry.register_tool(MovePathTool);
    registry.register_tool(DiagnosticsTool);
    registry.register_tool(ListDirectoryTool);
    registry.register_tool(NowTool);
    registry.register_tool(OpenTool);
    registry.register_tool(CodeSymbolsTool);
    registry.register_tool(ContentsTool);
    registry.register_tool(FindPathTool);
    registry.register_tool(ReadFileTool);
    registry.register_tool(GrepTool);
    registry.register_tool(RenameTool);
    registry.register_tool(ThinkingTool);
    registry.register_tool(FetchTool::new(http_client));

    register_edit_file_tool(cx);
    cx.observe_flag::<AgentStreamEditsFeatureFlag, _>(|_, cx| register_edit_file_tool(cx))
        .detach();
    cx.observe_global::<SettingsStore>(register_edit_file_tool)
        .detach();

    cx.subscribe(
        &LanguageModelRegistry::global(cx),
        move |registry, event, cx| match event {
            language_model::Event::DefaultModelChanged => {
                let using_zed_provider = registry
                    .read(cx)
                    .default_model()
                    .map_or(false, |default| default.is_provided_by_zed());
                if using_zed_provider {
                    ToolRegistry::global(cx).register_tool(WebSearchTool);
                } else {
                    ToolRegistry::global(cx).unregister_tool(WebSearchTool);
                }
            }
            _ => {}
        },
    )
    .detach();
}

fn register_edit_file_tool(cx: &mut App) {
    let registry = ToolRegistry::global(cx);

    registry.unregister_tool(CreateFileTool);
    registry.unregister_tool(EditFileTool);
    registry.unregister_tool(StreamingEditFileTool);

    if AssistantSettings::get_global(cx).stream_edits(cx) {
        registry.register_tool(StreamingEditFileTool);
    } else {
        registry.register_tool(CreateFileTool);
        registry.register_tool(EditFileTool);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::Client;
    use clock::FakeSystemClock;
    use http_client::FakeHttpClient;
    use schemars::JsonSchema;
    use serde::Serialize;

    #[test]
    fn test_json_schema() {
        #[derive(Serialize, JsonSchema)]
        struct GetWeatherTool {
            location: String,
        }

        let schema = schema::json_schema_for::<GetWeatherTool>(
            language_model::LanguageModelToolSchemaFormat::JsonSchema,
        )
        .unwrap();

        assert_eq!(
            schema,
            serde_json::json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string"
                    }
                },
                "required": ["location"],
            })
        );
    }

    #[gpui::test]
    fn test_builtin_tool_schema_compatibility(cx: &mut App) {
        settings::init(cx);
        AssistantSettings::register(cx);

        let client = Client::new(
            Arc::new(FakeSystemClock::new()),
            FakeHttpClient::with_200_response(),
            cx,
        );
        language_model::init(client.clone(), cx);
        crate::init(client.http_client(), cx);

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
