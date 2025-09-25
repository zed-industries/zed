use agent_settings::AgentProfileId;
use anyhow::Result;
use async_trait::async_trait;
use util::rel_path::RelPath;

use crate::example::{Example, ExampleContext, ExampleMetadata, JudgeAssertion, LanguageServer};

pub struct AddArgToTraitMethod;

#[async_trait(?Send)]
impl Example for AddArgToTraitMethod {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "add_arg_to_trait_method".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "f69aeb6311dde3c0b8979c293d019d66498d54f2".to_string(),
            language_server: Some(LanguageServer {
                file_extension: "rs".to_string(),
                allow_preexisting_diagnostics: false,
            }),
            max_assertions: None,
            profile_id: AgentProfileId::default(),
            existing_thread_json: None,
            max_turns: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        const FILENAME: &str = "assistant_tool.rs";
        cx.push_user_message(format!(
            r#"
            Add a `window: Option<gpui::AnyWindowHandle>` argument to the `Tool::run` trait method in {FILENAME},
            and update all the implementations of the trait and call sites accordingly.
            "#
        ));

        let _ = cx.run_to_end().await?;

        // Adds ignored argument to all but `batch_tool`

        let add_ignored_window_paths = &[
            "code_action_tool",
            "code_symbols_tool",
            "contents_tool",
            "copy_path_tool",
            "create_directory_tool",
            "create_file_tool",
            "delete_path_tool",
            "diagnostics_tool",
            "edit_file_tool",
            "fetch_tool",
            "grep_tool",
            "list_directory_tool",
            "move_path_tool",
            "now_tool",
            "open_tool",
            "path_search_tool",
            "read_file_tool",
            "rename_tool",
            "symbol_info_tool",
            "terminal_tool",
            "thinking_tool",
            "web_search_tool",
        ];

        let edits = cx.edits();

        for tool_name in add_ignored_window_paths {
            let path_str = format!("crates/assistant_tools/src/{}.rs", tool_name);
            let edits = edits.get(RelPath::new(&path_str).unwrap());

            let ignored = edits.is_some_and(|edits| {
                edits.has_added_line("        _window: Option<gpui::AnyWindowHandle>,\n")
            });
            let uningored = edits.is_some_and(|edits| {
                edits.has_added_line("        window: Option<gpui::AnyWindowHandle>,\n")
            });

            cx.assert(ignored || uningored, format!("Argument:   {}", tool_name))
                .ok();

            cx.assert(ignored, format!("`_` prefix: {}", tool_name))
                .ok();
        }

        // Adds unignored argument to `batch_tool`

        let batch_tool_edits =
            edits.get(RelPath::new("crates/assistant_tools/src/batch_tool.rs").unwrap());

        cx.assert(
            batch_tool_edits.is_some_and(|edits| {
                edits.has_added_line("        window: Option<gpui::AnyWindowHandle>,\n")
            }),
            "Argument:   batch_tool",
        )
        .ok();

        Ok(())
    }

    fn diff_assertions(&self) -> Vec<JudgeAssertion> {
        vec![
            JudgeAssertion {
                id: "batch tool passes window to each".to_string(),
                description:
                    "batch_tool is modified to pass a clone of the window to each tool it calls."
                        .to_string(),
            },
            JudgeAssertion {
                id: "tool tests updated".to_string(),
                description:
                    "tool tests are updated to pass the new `window` argument (`None` is ok)."
                        .to_string(),
            },
        ]
    }
}
