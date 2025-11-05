mod context_server_registry;
mod copy_path_tool;
mod create_directory_tool;
mod delete_path_tool;
mod diagnostics_tool;
mod edit_file_tool;
mod fetch_tool;
mod find_path_tool;
mod grep_tool;
mod list_directory_tool;
mod move_path_tool;
mod now_tool;
mod open_tool;
mod read_file_tool;
mod terminal_tool;
mod thinking_tool;
mod web_search_tool;

use crate::AgentTool;
use language_model::{LanguageModelRequestTool, LanguageModelToolSchemaFormat};

pub use context_server_registry::*;
pub use copy_path_tool::*;
pub use create_directory_tool::*;
pub use delete_path_tool::*;
pub use diagnostics_tool::*;
pub use edit_file_tool::*;
pub use fetch_tool::*;
pub use find_path_tool::*;
pub use grep_tool::*;
pub use list_directory_tool::*;
pub use move_path_tool::*;
pub use now_tool::*;
pub use open_tool::*;
pub use read_file_tool::*;
pub use terminal_tool::*;
pub use thinking_tool::*;
pub use web_search_tool::*;

macro_rules! tools {
    ($($tool:ty),* $(,)?) => {
        /// A list of all built-in tool names
        pub fn supported_built_in_tool_names(provider: Option<language_model::LanguageModelProviderId>) -> impl Iterator<Item = String> {
            [
                $(
                    (if let Some(provider) = provider.as_ref() {
                        <$tool>::supports_provider(provider)
                    } else {
                        true
                    })
                    .then(|| <$tool>::name().to_string()),
                )*
            ]
            .into_iter()
            .flatten()
        }

        /// A list of all built-in tools
        pub fn built_in_tools() -> impl Iterator<Item = LanguageModelRequestTool> {
            fn language_model_tool<T: AgentTool>() -> LanguageModelRequestTool {
                LanguageModelRequestTool {
                    name: T::name().to_string(),
                    description: T::description().to_string(),
                    input_schema: T::input_schema(LanguageModelToolSchemaFormat::JsonSchema).to_value(),
                }
            }
            [
                $(
                    language_model_tool::<$tool>(),
                )*
            ]
            .into_iter()
        }
    };
}

tools! {
    CopyPathTool,
    CreateDirectoryTool,
    DeletePathTool,
    DiagnosticsTool,
    EditFileTool,
    FetchTool,
    FindPathTool,
    GrepTool,
    ListDirectoryTool,
    MovePathTool,
    NowTool,
    OpenTool,
    ReadFileTool,
    TerminalTool,
    ThinkingTool,
    WebSearchTool,
}
