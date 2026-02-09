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
mod restore_file_from_disk_tool;
mod save_file_tool;
mod streaming_edit_file_tool;
mod subagent_tool;
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
pub use restore_file_from_disk_tool::*;
pub use save_file_tool::*;
pub use streaming_edit_file_tool::*;
pub use subagent_tool::*;
pub use terminal_tool::*;
pub use thinking_tool::*;
pub use web_search_tool::*;

macro_rules! tools {
    ($($tool:ty),* $(,)?) => {
        /// Every built-in tool name, determined at compile time.
        pub const ALL_TOOL_NAMES: &[&str] = &[
            $(<$tool>::NAME,)*
        ];

        const _: () = {
            const fn str_eq(a: &str, b: &str) -> bool {
                let a = a.as_bytes();
                let b = b.as_bytes();
                if a.len() != b.len() {
                    return false;
                }
                let mut i = 0;
                while i < a.len() {
                    if a[i] != b[i] {
                        return false;
                    }
                    i += 1;
                }
                true
            }

            const NAMES: &[&str] = ALL_TOOL_NAMES;
            let mut i = 0;
            while i < NAMES.len() {
                let mut j = i + 1;
                while j < NAMES.len() {
                    if str_eq(NAMES[i], NAMES[j]) {
                        panic!("Duplicate tool name in tools! macro");
                    }
                    j += 1;
                }
                i += 1;
            }
        };

        /// Returns whether the tool with the given name supports the given provider.
        pub fn tool_supports_provider(name: &str, provider: &language_model::LanguageModelProviderId) -> bool {
            $(
                if name == <$tool>::NAME {
                    return <$tool>::supports_provider(provider);
                }
            )*
            false
        }

        /// A list of all built-in tools
        pub fn built_in_tools() -> impl Iterator<Item = LanguageModelRequestTool> {
            fn language_model_tool<T: AgentTool>() -> LanguageModelRequestTool {
                LanguageModelRequestTool {
                    name: T::NAME.to_string(),
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
    RestoreFileFromDiskTool,
    SaveFileTool,
    SubagentTool,
    TerminalTool,
    ThinkingTool,
    WebSearchTool,
}
