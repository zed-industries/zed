use super::{
    create_label_for_command, search_command::add_search_result_section, SlashCommand,
    SlashCommandOutput,
};
use crate::PromptBuilder;
use anyhow::{anyhow, Result};
use assistant_slash_command::{ArgumentCompletion, SlashCommandOutputSection};
use feature_flags::FeatureFlag;
use gpui::{AppContext, Task, WeakView, WindowContext};
use language::{Anchor, CodeLabel, LspAdapterDelegate};
use language_model::{LanguageModelRegistry, LanguageModelTool};
use schemars::JsonSchema;
use semantic_index::SemanticDb;
use serde::Deserialize;

pub struct ProjectSlashCommandFeatureFlag;

impl FeatureFlag for ProjectSlashCommandFeatureFlag {
    const NAME: &'static str = "project-slash-command";
}

use std::{
    fmt::Write as _,
    ops::DerefMut,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{BorrowAppContext as _, IconName};
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
        "project".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("project", &[], cx)
    }

    fn description(&self) -> String {
        "Generate a semantic search based on context".into()
    }

    fn menu_text(&self) -> String {
        self.description()
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
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let current_model = model_registry.active_model();
        let prompt_builder = self.prompt_builder.clone();

        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };
        let project = workspace.read(cx).project().clone();
        let fs = project.read(cx).fs().clone();
        let Some(project_index) =
            cx.update_global(|index: &mut SemanticDb, cx| index.project_index(project, cx))
        else {
            return Task::ready(Err(anyhow::anyhow!("no project indexer")));
        };

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
                        temperature: None,
                    },
                    cx.deref_mut(),
                )
                .await?
                .search_queries;

            let results = project_index
                .read_with(&cx, |project_index, cx| {
                    project_index.search(search_queries.clone(), 25, cx)
                })?
                .await?;

            let results = SemanticDb::load_results(results, &fs, &cx).await?;

            cx.background_executor()
                .spawn(async move {
                    let mut output = "Project context:\n".to_string();
                    let mut sections = Vec::new();

                    for (ix, query) in search_queries.into_iter().enumerate() {
                        let start_ix = output.len();
                        writeln!(&mut output, "Results for {query}:").unwrap();
                        let mut has_results = false;
                        for result in &results {
                            if result.query_index == ix {
                                add_search_result_section(result, &mut output, &mut sections);
                                has_results = true;
                            }
                        }
                        if has_results {
                            sections.push(SlashCommandOutputSection {
                                range: start_ix..output.len(),
                                icon: IconName::MagnifyingGlass,
                                label: query.into(),
                                metadata: None,
                            });
                            output.push('\n');
                        } else {
                            output.truncate(start_ix);
                        }
                    }

                    sections.push(SlashCommandOutputSection {
                        range: 0..output.len(),
                        icon: IconName::Book,
                        label: "Project context".into(),
                        metadata: None,
                    });

                    Ok(SlashCommandOutput {
                        text: output,
                        sections,
                        run_commands_in_text: true,
                    })
                })
                .await
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
