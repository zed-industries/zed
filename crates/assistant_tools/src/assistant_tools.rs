mod copy_path_tool;
mod create_directory_tool;
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
mod schema;
mod templates;
mod terminal_tool;
mod thinking_tool;
mod ui;
mod web_search_tool;

use std::sync::Arc;

use assistant_tool::ToolRegistry;
use copy_path_tool::CopyPathTool;
use gpui::{App, Entity};
use http_client::HttpClientWithUrl;
use language_model::LanguageModelRegistry;
use move_path_tool::MovePathTool;
use web_search_tool::WebSearchTool;

pub(crate) use templates::*;

use crate::create_directory_tool::CreateDirectoryTool;
use crate::delete_path_tool::DeletePathTool;
use crate::diagnostics_tool::DiagnosticsTool;
use crate::edit_file_tool::EditFileTool;
use crate::fetch_tool::FetchTool;
use crate::find_path_tool::FindPathTool;
use crate::grep_tool::GrepTool;
use crate::list_directory_tool::ListDirectoryTool;
use crate::now_tool::NowTool;
use crate::thinking_tool::ThinkingTool;

pub use edit_file_tool::{EditFileMode, EditFileToolInput};
pub use find_path_tool::FindPathToolInput;
pub use open_tool::OpenTool;
pub use read_file_tool::{ReadFileTool, ReadFileToolInput};
pub use terminal_tool::TerminalTool;

pub fn init(http_client: Arc<HttpClientWithUrl>, cx: &mut App) {
    assistant_tool::init(cx);

    let registry = ToolRegistry::global(cx);
    registry.register_tool(TerminalTool::new(cx));
    registry.register_tool(CreateDirectoryTool);
    registry.register_tool(CopyPathTool);
    registry.register_tool(DeletePathTool);
    registry.register_tool(MovePathTool);
    registry.register_tool(DiagnosticsTool);
    registry.register_tool(ListDirectoryTool);
    registry.register_tool(NowTool);
    registry.register_tool(OpenTool);
    registry.register_tool(FindPathTool);
    registry.register_tool(ReadFileTool);
    registry.register_tool(GrepTool);
    registry.register_tool(ThinkingTool);
    registry.register_tool(FetchTool::new(http_client));
    registry.register_tool(EditFileTool);

    register_web_search_tool(&LanguageModelRegistry::global(cx), cx);
    cx.subscribe(
        &LanguageModelRegistry::global(cx),
        move |registry, event, cx| match event {
            language_model::Event::DefaultModelChanged => {
                register_web_search_tool(&registry, cx);
            }
            _ => {}
        },
    )
    .detach();
}

fn register_web_search_tool(registry: &Entity<LanguageModelRegistry>, cx: &mut App) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use assistant_settings::AssistantSettings;
    use client::Client;
    use clock::FakeSystemClock;
    use http_client::FakeHttpClient;
    use schemars::JsonSchema;
    use serde::Serialize;
    use settings::Settings;

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
