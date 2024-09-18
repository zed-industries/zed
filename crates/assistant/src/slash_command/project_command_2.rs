use super::{create_label_for_command, SlashCommand, SlashCommandOutput};
use crate::PromptBuilder;
use anyhow::{anyhow, Result};
use assistant_slash_command::{ArgumentCompletion, SlashCommandOutputSection};
use gpui::{AppContext, Task, WeakView, WindowContext};
use language::{Anchor, CodeLabel, LspAdapterDelegate};
use language_model::{LanguageModelRegistry, LanguageModelTool};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    ops::DerefMut,
    sync::{atomic::AtomicBool, Arc},
};
use ui::IconName;
use workspace::Workspace;

pub struct ProjectSlashCommand {
    prompt_builder: Arc<PromptBuilder>,
}

impl ProjectSlashCommand {
    pub fn new(prompt_builder: Arc<PromptBuilder>) -> Self {
        Self { prompt_builder }
    }
}

impl SlashCommand for ProjectSlashCommand {
    fn name(&self) -> String {
        "codebase".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("codebase", &[], cx)
    }

    fn description(&self) -> String {
        "Generate semantic searches based on the current context".into()
    }

    fn menu_text(&self) -> String {
        "Project Context".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<Anchor>],
        context_buffer: language::BufferSnapshot,
        _workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let current_model = model_registry.active_model();
        let prompt_builder = self.prompt_builder.clone();

        cx.spawn(|mut cx| async move {
            let current_model = current_model.ok_or_else(|| anyhow!("no model selected"))?;

            let prompt =
                prompt_builder.generate_project_slash_command_prompt(context_buffer.text())?;

            let search_queries = current_model
                .use_tool::<SearchQueries>(
                    language_model::LanguageModelRequest {
                        messages: vec![language_model::LanguageModelRequestMessage {
                            role: language_model::Role::User,
                            content: vec![language_model::MessageContent::Text(prompt)],
                            cache: false,
                        }],
                        tools: vec![],
                        stop: vec![],
                        temperature: 1.0,
                    },
                    cx.deref_mut(),
                )
                .await?
                .search_queries;

            let mut output = "Project context:\n".to_string();
            for query in search_queries {
                let section_text = format!("/search {}", query);
                // sections.push(SlashCommandOutputSection {
                //     range: output.len()..output.len() + section_text.len(),
                //     icon: IconName::MagnifyingGlass,
                //     label: query.into(),
                //     metadata: None,
                // });
                output.push_str(&section_text);
                output.push('\n');
            }

            let sections = vec![SlashCommandOutputSection {
                range: 0..output.len(),
                icon: IconName::Book,
                label: "Project context".into(),
                metadata: None,
            }];

            Ok(SlashCommandOutput {
                text: output,
                sections,
                run_commands_in_text: true,
            })
        })
    }
}

#[derive(JsonSchema, Deserialize)]
struct SearchQueries {
    /// An array of semantic search queries.
    ///
    /// These queries will be used to search the user's codebase.
    /// The function can only accept 4 queries, otherwise it will error.
    /// As such, it's important that you limit the length of the search_queries array to 5 queries or less.
    search_queries: Vec<String>,
}

impl LanguageModelTool for SearchQueries {
    fn name() -> String {
        "search_queries".to_string()
    }

    fn description() -> String {
        "Generate semantic search queries based on context".to_string()
    }
}
