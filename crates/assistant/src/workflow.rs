use crate::{AssistantPanel, Context, InlineAssistId, InlineAssistant};
use anyhow::{anyhow, Error, Result};
use collections::HashMap;
use editor::Editor;
use futures::future;
use gpui::{Model, ModelContext, Task, UpdateGlobal as _, View, WeakView, WindowContext};
use language::{Anchor, Buffer, BufferSnapshot};
use language_model::{LanguageModelRegistry, LanguageModelRequestMessage, Role};
use project::Project;
use rope::Point;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::{cmp, ops::Range, sync::Arc};
use text::{AnchorRangeExt as _, OffsetRangeExt as _};
use util::ResultExt as _;
use workspace::Workspace;

pub struct WorkflowStepResolution {
    tagged_range: Range<Anchor>,
    output: String,
    pub result: Option<Result<ResolvedWorkflowStep, Arc<Error>>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedWorkflowStep {
    pub title: String,
    pub suggestions: HashMap<Model<Buffer>, Vec<WorkflowSuggestionGroup>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowSuggestionGroup {
    pub context_range: Range<language::Anchor>,
    pub suggestions: Vec<WorkflowSuggestion>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowSuggestion {
    Update {
        range: Range<language::Anchor>,
        description: String,
    },
    CreateFile {
        description: String,
    },
    InsertSiblingBefore {
        position: language::Anchor,
        description: String,
    },
    InsertSiblingAfter {
        position: language::Anchor,
        description: String,
    },
    PrependChild {
        position: language::Anchor,
        description: String,
    },
    AppendChild {
        position: language::Anchor,
        description: String,
    },
    Delete {
        range: Range<language::Anchor>,
    },
}

impl WorkflowStepResolution {
    pub fn new(range: Range<Anchor>) -> Self {
        Self {
            tagged_range: range,
            output: String::new(),
            result: None,
        }
    }

    pub fn resolve(
        &mut self,
        context: &Context,
        cx: &mut ModelContext<WorkflowStepResolution>,
    ) -> Option<Task<()>> {
        let project = context.project()?;
        let context_buffer = context.buffer().clone();
        let prompt_builder = context.prompt_builder();
        let mut request = context.to_completion_request(cx);
        let model = LanguageModelRegistry::read_global(cx).active_model();
        let step_text = context_buffer
            .read(cx)
            .text_for_range(self.tagged_range.clone())
            .collect::<String>();

        Some(cx.spawn(|this, mut cx| async move {
            let result = async {
                let Some(model) = model else {
                    return Err(anyhow!("no model selected"));
                };

                this.update(&mut cx, |this, cx| {
                    this.output.clear();
                    this.result = None;
                    cx.notify();
                })?;

                let mut prompt = prompt_builder.generate_step_resolution_prompt()?;
                prompt.push_str(&step_text);
                request.messages.push(LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![prompt.into()],
                });

                // Invoke the model to get its edit suggestions for this workflow step.
                let mut stream = model
                    .use_tool_stream::<tool::WorkflowStepResolution>(request, &cx)
                    .await?;
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    this.update(&mut cx, |this, cx| {
                        this.output.push_str(&chunk);
                        cx.notify();
                    })?;
                }

                let resolution = this.update(&mut cx, |this, _| {
                    serde_json::from_str::<tool::WorkflowStepResolution>(&this.output)
                })??;

                // Translate the parsed suggestions to our internal types, which anchor the suggestions to locations in the code.
                let suggestion_tasks: Vec<_> = resolution
                    .suggestions
                    .iter()
                    .map(|suggestion| suggestion.resolve(project.clone(), cx.clone()))
                    .collect();

                // Expand the context ranges of each suggestion and group suggestions with overlapping context ranges.
                let suggestions = future::join_all(suggestion_tasks)
                    .await
                    .into_iter()
                    .filter_map(|task| task.log_err())
                    .collect::<Vec<_>>();

                let mut suggestions_by_buffer = HashMap::default();
                for (buffer, suggestion) in suggestions {
                    suggestions_by_buffer
                        .entry(buffer)
                        .or_insert_with(Vec::new)
                        .push(suggestion);
                }

                let mut suggestion_groups_by_buffer = HashMap::default();
                for (buffer, mut suggestions) in suggestions_by_buffer {
                    let mut suggestion_groups = Vec::<WorkflowSuggestionGroup>::new();
                    let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
                    // Sort suggestions by their range so that earlier, larger ranges come first
                    suggestions.sort_by(|a, b| a.range().cmp(&b.range(), &snapshot));

                    // Merge overlapping suggestions
                    suggestions.dedup_by(|a, b| b.try_merge(&a, &snapshot));

                    // Create context ranges for each suggestion
                    for suggestion in suggestions {
                        let context_range = {
                            let suggestion_point_range = suggestion.range().to_point(&snapshot);
                            let start_row = suggestion_point_range.start.row.saturating_sub(5);
                            let end_row = cmp::min(
                                suggestion_point_range.end.row + 5,
                                snapshot.max_point().row,
                            );
                            let start = snapshot.anchor_before(Point::new(start_row, 0));
                            let end = snapshot
                                .anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
                            start..end
                        };

                        if let Some(last_group) = suggestion_groups.last_mut() {
                            if last_group
                                .context_range
                                .end
                                .cmp(&context_range.start, &snapshot)
                                .is_ge()
                            {
                                // Merge with the previous group if context ranges overlap
                                last_group.context_range.end = context_range.end;
                                last_group.suggestions.push(suggestion);
                            } else {
                                // Create a new group
                                suggestion_groups.push(WorkflowSuggestionGroup {
                                    context_range,
                                    suggestions: vec![suggestion],
                                });
                            }
                        } else {
                            // Create the first group
                            suggestion_groups.push(WorkflowSuggestionGroup {
                                context_range,
                                suggestions: vec![suggestion],
                            });
                        }
                    }

                    suggestion_groups_by_buffer.insert(buffer, suggestion_groups);
                }

                Ok((resolution.step_title, suggestion_groups_by_buffer))
            };

            let result = result.await;
            this.update(&mut cx, |this, cx| {
                this.result = Some(match result {
                    Ok((title, suggestions)) => Ok(ResolvedWorkflowStep { title, suggestions }),
                    Err(error) => Err(Arc::new(error)),
                });
                cx.notify();
            })
            .ok();
        }))
    }
}

impl WorkflowSuggestion {
    pub fn range(&self) -> Range<language::Anchor> {
        match self {
            WorkflowSuggestion::Update { range, .. } => range.clone(),
            WorkflowSuggestion::CreateFile { .. } => language::Anchor::MIN..language::Anchor::MAX,
            WorkflowSuggestion::InsertSiblingBefore { position, .. }
            | WorkflowSuggestion::InsertSiblingAfter { position, .. }
            | WorkflowSuggestion::PrependChild { position, .. }
            | WorkflowSuggestion::AppendChild { position, .. } => *position..*position,
            WorkflowSuggestion::Delete { range } => range.clone(),
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            WorkflowSuggestion::Update { description, .. }
            | WorkflowSuggestion::CreateFile { description }
            | WorkflowSuggestion::InsertSiblingBefore { description, .. }
            | WorkflowSuggestion::InsertSiblingAfter { description, .. }
            | WorkflowSuggestion::PrependChild { description, .. }
            | WorkflowSuggestion::AppendChild { description, .. } => Some(description),
            WorkflowSuggestion::Delete { .. } => None,
        }
    }

    fn description_mut(&mut self) -> Option<&mut String> {
        match self {
            WorkflowSuggestion::Update { description, .. }
            | WorkflowSuggestion::CreateFile { description }
            | WorkflowSuggestion::InsertSiblingBefore { description, .. }
            | WorkflowSuggestion::InsertSiblingAfter { description, .. }
            | WorkflowSuggestion::PrependChild { description, .. }
            | WorkflowSuggestion::AppendChild { description, .. } => Some(description),
            WorkflowSuggestion::Delete { .. } => None,
        }
    }

    fn try_merge(&mut self, other: &Self, buffer: &BufferSnapshot) -> bool {
        let range = self.range();
        let other_range = other.range();

        // Don't merge if we don't contain the other suggestion.
        if range.start.cmp(&other_range.start, buffer).is_gt()
            || range.end.cmp(&other_range.end, buffer).is_lt()
        {
            return false;
        }

        if let Some(description) = self.description_mut() {
            if let Some(other_description) = other.description() {
                description.push('\n');
                description.push_str(other_description);
            }
        }
        true
    }

    pub fn show(
        &self,
        editor: &View<Editor>,
        excerpt_id: editor::ExcerptId,
        workspace: &WeakView<Workspace>,
        assistant_panel: &View<AssistantPanel>,
        cx: &mut WindowContext,
    ) -> Option<InlineAssistId> {
        let mut initial_transaction_id = None;
        let initial_prompt;
        let suggestion_range;
        let buffer = editor.read(cx).buffer().clone();
        let snapshot = buffer.read(cx).snapshot(cx);

        match self {
            WorkflowSuggestion::Update { range, description } => {
                initial_prompt = description.clone();
                suggestion_range = snapshot.anchor_in_excerpt(excerpt_id, range.start)?
                    ..snapshot.anchor_in_excerpt(excerpt_id, range.end)?;
            }
            WorkflowSuggestion::CreateFile { description } => {
                initial_prompt = description.clone();
                suggestion_range = editor::Anchor::min()..editor::Anchor::min();
            }
            WorkflowSuggestion::InsertSiblingBefore {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::InsertSiblingAfter {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::PrependChild {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, false, true, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::AppendChild {
                position,
                description,
            } => {
                let position = snapshot.anchor_in_excerpt(excerpt_id, *position)?;
                initial_prompt = description.clone();
                suggestion_range = buffer.update(cx, |buffer, cx| {
                    buffer.start_transaction(cx);
                    let line_start = buffer.insert_empty_line(position, true, false, cx);
                    initial_transaction_id = buffer.end_transaction(cx);
                    buffer.refresh_preview(cx);

                    let line_start = buffer.read(cx).anchor_before(line_start);
                    line_start..line_start
                });
            }
            WorkflowSuggestion::Delete { range } => {
                initial_prompt = "Delete".to_string();
                suggestion_range = snapshot.anchor_in_excerpt(excerpt_id, range.start)?
                    ..snapshot.anchor_in_excerpt(excerpt_id, range.end)?;
            }
        }

        InlineAssistant::update_global(cx, |inline_assistant, cx| {
            Some(inline_assistant.suggest_assist(
                editor,
                suggestion_range,
                initial_prompt,
                initial_transaction_id,
                Some(workspace.clone()),
                Some(assistant_panel),
                cx,
            ))
        })
    }
}

pub mod tool {
    use std::path::Path;

    use super::*;
    use anyhow::Context as _;
    use gpui::AsyncAppContext;
    use language::ParseStatus;
    use language_model::LanguageModelTool;
    use project::ProjectPath;
    use schemars::JsonSchema;

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    pub struct WorkflowStepResolution {
        /// An extremely short title for the edit step represented by these operations.
        pub step_title: String,
        /// A sequence of operations to apply to the codebase.
        /// When multiple operations are required for a step, be sure to include multiple operations in this list.
        pub suggestions: Vec<WorkflowSuggestion>,
    }

    impl LanguageModelTool for WorkflowStepResolution {
        fn name() -> String {
            "edit".into()
        }

        fn description() -> String {
            "suggest edits to one or more locations in the codebase".into()
        }
    }

    /// A description of an operation to apply to one location in the codebase.
    ///
    /// This object represents a single edit operation that can be performed on a specific file
    /// in the codebase. It encapsulates both the location (file path) and the nature of the
    /// edit to be made.
    ///
    /// # Fields
    ///
    /// * `path`: A string representing the file path where the edit operation should be applied.
    ///           This path is relative to the root of the project or repository.
    ///
    /// * `kind`: An enum representing the specific type of edit operation to be performed.
    ///
    /// # Usage
    ///
    /// `EditOperation` is used within a code editor to represent and apply
    /// programmatic changes to source code. It provides a structured way to describe
    /// edits for features like refactoring tools or AI-assisted coding suggestions.
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub struct WorkflowSuggestion {
        /// The path to the file containing the relevant operation
        pub path: String,
        #[serde(flatten)]
        pub kind: WorkflowSuggestionKind,
    }

    impl WorkflowSuggestion {
        pub(super) async fn resolve(
            &self,
            project: Model<Project>,
            mut cx: AsyncAppContext,
        ) -> Result<(Model<Buffer>, super::WorkflowSuggestion)> {
            let path = self.path.clone();
            let kind = self.kind.clone();
            let buffer = project
                .update(&mut cx, |project, cx| {
                    let project_path = project
                        .find_project_path(Path::new(&path), cx)
                        .or_else(|| {
                            // If we couldn't find a project path for it, put it in the active worktree
                            // so that when we create the buffer, it can be saved.
                            let worktree = project
                                .active_entry()
                                .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
                                .or_else(|| project.worktrees(cx).next())?;
                            let worktree = worktree.read(cx);

                            Some(ProjectPath {
                                worktree_id: worktree.id(),
                                path: Arc::from(Path::new(&path)),
                            })
                        })
                        .with_context(|| format!("worktree not found for {:?}", path))?;
                    anyhow::Ok(project.open_buffer(project_path, cx))
                })??
                .await?;

            let mut parse_status = buffer.read_with(&cx, |buffer, _cx| buffer.parse_status())?;
            while *parse_status.borrow() != ParseStatus::Idle {
                parse_status.changed().await?;
            }

            let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
            let outline = snapshot.outline(None).context("no outline for buffer")?;

            let suggestion;
            match kind {
                WorkflowSuggestionKind::Update {
                    symbol,
                    description,
                } => {
                    let symbol = outline
                        .find_most_similar(&symbol)
                        .with_context(|| format!("symbol not found: {:?}", symbol))?
                        .to_point(&snapshot);
                    let start = symbol
                        .annotation_range
                        .map_or(symbol.range.start, |range| range.start);
                    let start = Point::new(start.row, 0);
                    let end = Point::new(
                        symbol.range.end.row,
                        snapshot.line_len(symbol.range.end.row),
                    );
                    let range = snapshot.anchor_before(start)..snapshot.anchor_after(end);
                    suggestion = super::WorkflowSuggestion::Update { range, description };
                }
                WorkflowSuggestionKind::Create { description } => {
                    suggestion = super::WorkflowSuggestion::CreateFile { description };
                }
                WorkflowSuggestionKind::InsertSiblingBefore {
                    symbol,
                    description,
                } => {
                    let symbol = outline
                        .find_most_similar(&symbol)
                        .with_context(|| format!("symbol not found: {:?}", symbol))?
                        .to_point(&snapshot);
                    let position = snapshot.anchor_before(
                        symbol
                            .annotation_range
                            .map_or(symbol.range.start, |annotation_range| {
                                annotation_range.start
                            }),
                    );
                    suggestion = super::WorkflowSuggestion::InsertSiblingBefore {
                        position,
                        description,
                    };
                }
                WorkflowSuggestionKind::InsertSiblingAfter {
                    symbol,
                    description,
                } => {
                    let symbol = outline
                        .find_most_similar(&symbol)
                        .with_context(|| format!("symbol not found: {:?}", symbol))?
                        .to_point(&snapshot);
                    let position = snapshot.anchor_after(symbol.range.end);
                    suggestion = super::WorkflowSuggestion::InsertSiblingAfter {
                        position,
                        description,
                    };
                }
                WorkflowSuggestionKind::PrependChild {
                    symbol,
                    description,
                } => {
                    if let Some(symbol) = symbol {
                        let symbol = outline
                            .find_most_similar(&symbol)
                            .with_context(|| format!("symbol not found: {:?}", symbol))?
                            .to_point(&snapshot);

                        let position = snapshot.anchor_after(
                            symbol
                                .body_range
                                .map_or(symbol.range.start, |body_range| body_range.start),
                        );
                        suggestion = super::WorkflowSuggestion::PrependChild {
                            position,
                            description,
                        };
                    } else {
                        suggestion = super::WorkflowSuggestion::PrependChild {
                            position: language::Anchor::MIN,
                            description,
                        };
                    }
                }
                WorkflowSuggestionKind::AppendChild {
                    symbol,
                    description,
                } => {
                    if let Some(symbol) = symbol {
                        let symbol = outline
                            .find_most_similar(&symbol)
                            .with_context(|| format!("symbol not found: {:?}", symbol))?
                            .to_point(&snapshot);

                        let position = snapshot.anchor_before(
                            symbol
                                .body_range
                                .map_or(symbol.range.end, |body_range| body_range.end),
                        );
                        suggestion = super::WorkflowSuggestion::AppendChild {
                            position,
                            description,
                        };
                    } else {
                        suggestion = super::WorkflowSuggestion::PrependChild {
                            position: language::Anchor::MAX,
                            description,
                        };
                    }
                }
                WorkflowSuggestionKind::Delete { symbol } => {
                    let symbol = outline
                        .find_most_similar(&symbol)
                        .with_context(|| format!("symbol not found: {:?}", symbol))?
                        .to_point(&snapshot);
                    let start = symbol
                        .annotation_range
                        .map_or(symbol.range.start, |range| range.start);
                    let start = Point::new(start.row, 0);
                    let end = Point::new(
                        symbol.range.end.row,
                        snapshot.line_len(symbol.range.end.row),
                    );
                    let range = snapshot.anchor_before(start)..snapshot.anchor_after(end);
                    suggestion = super::WorkflowSuggestion::Delete { range };
                }
            }

            Ok((buffer, suggestion))
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(tag = "kind")]
    pub enum WorkflowSuggestionKind {
        /// Rewrites the specified symbol entirely based on the given description.
        /// This operation completely replaces the existing symbol with new content.
        Update {
            /// A fully-qualified reference to the symbol, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
            /// The path should uniquely identify the symbol within the containing file.
            symbol: String,
            /// A brief description of the transformation to apply to the symbol.
            description: String,
        },
        /// Creates a new file with the given path based on the provided description.
        /// This operation adds a new file to the codebase.
        Create {
            /// A brief description of the file to be created.
            description: String,
        },
        /// Inserts a new symbol based on the given description before the specified symbol.
        /// This operation adds new content immediately preceding an existing symbol.
        InsertSiblingBefore {
            /// A fully-qualified reference to the symbol, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
            /// The new content will be inserted immediately before this symbol.
            symbol: String,
            /// A brief description of the new symbol to be inserted.
            description: String,
        },
        /// Inserts a new symbol based on the given description after the specified symbol.
        /// This operation adds new content immediately following an existing symbol.
        InsertSiblingAfter {
            /// A fully-qualified reference to the symbol, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
            /// The new content will be inserted immediately after this symbol.
            symbol: String,
            /// A brief description of the new symbol to be inserted.
            description: String,
        },
        /// Inserts a new symbol as a child of the specified symbol at the start.
        /// This operation adds new content as the first child of an existing symbol (or file if no symbol is provided).
        PrependChild {
            /// An optional fully-qualified reference to the symbol after the code you want to insert, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
            /// If provided, the new content will be inserted as the first child of this symbol.
            /// If not provided, the new content will be inserted at the top of the file.
            symbol: Option<String>,
            /// A brief description of the new symbol to be inserted.
            description: String,
        },
        /// Inserts a new symbol as a child of the specified symbol at the end.
        /// This operation adds new content as the last child of an existing symbol (or file if no symbol is provided).
        AppendChild {
            /// An optional fully-qualified reference to the symbol before the code you want to insert, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
            /// If provided, the new content will be inserted as the last child of this symbol.
            /// If not provided, the new content will be applied at the bottom of the file.
            symbol: Option<String>,
            /// A brief description of the new symbol to be inserted.
            description: String,
        },
        /// Deletes the specified symbol from the containing file.
        Delete {
            /// An fully-qualified reference to the symbol to be deleted, e.g. `mod foo impl Bar pub fn baz` instead of just `fn baz`.
            symbol: String,
        },
    }
}
