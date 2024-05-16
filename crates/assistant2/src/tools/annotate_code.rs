use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ProjectContext, ToolView};
use editor::{
    display_map::{BlockContext, BlockDisposition, BlockProperties, BlockStyle},
    Editor, MultiBuffer,
};
use futures::{channel::mpsc::UnboundedSender, StreamExt as _};
use gpui::{prelude::*, AnyElement, AsyncWindowContext, Model, Task, View, WeakView};
use language::ToPoint;
use project::{search::SearchQuery, Project, ProjectPath};
use schemars::JsonSchema;
use serde::Deserialize;
use std::path::Path;
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;

pub struct AnnotationTool {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
}

impl AnnotationTool {
    pub fn new(workspace: WeakView<Workspace>, project: Model<Project>) -> Self {
        Self { workspace, project }
    }
}

#[derive(Default, Debug, Deserialize, JsonSchema, Clone)]
pub struct AnnotationInput {
    /// Name for this set of annotations
    #[serde(default = "default_title")]
    title: String,
    /// Excerpts from the file to show to the user.
    excerpts: Vec<Excerpt>,
}

fn default_title() -> String {
    "Untitled".to_string()
}

#[derive(Debug, Deserialize, JsonSchema, Clone)]
struct Excerpt {
    /// Path to the file
    path: String,
    /// A short, distinctive string that appears in the file, used to define a location in the file.
    text_passage: String,
    /// Text to display above the code excerpt
    annotation: String,
}

impl LanguageModelTool for AnnotationTool {
    type View = AnnotationResultView;

    fn name(&self) -> String {
        "annotate_code".to_string()
    }

    fn description(&self) -> String {
        "Dynamically annotate symbols in the current codebase. Opens a buffer in a panel in their editor, to the side of the conversation. The annotations are shown in the editor as a block decoration.".to_string()
    }

    fn view(&self, cx: &mut WindowContext) -> View<Self::View> {
        cx.new_view(|cx| {
            let (tx, mut rx) = futures::channel::mpsc::unbounded();
            cx.spawn(|view, mut cx| async move {
                while let Some(excerpt) = rx.next().await {
                    AnnotationResultView::add_excerpt(view.clone(), excerpt, &mut cx).await?;
                }
                anyhow::Ok(())
            })
            .detach();

            AnnotationResultView {
                project: self.project.clone(),
                workspace: self.workspace.clone(),
                tx,
                pending_excerpt: None,
                added_editor_to_workspace: false,
                editor: None,
                error: None,
                rendered_excerpt_count: 0,
            }
        })
    }
}

pub struct AnnotationResultView {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    pending_excerpt: Option<Excerpt>,
    added_editor_to_workspace: bool,
    editor: Option<View<Editor>>,
    tx: UnboundedSender<Excerpt>,
    error: Option<anyhow::Error>,
    rendered_excerpt_count: usize,
}

impl AnnotationResultView {
    async fn add_excerpt(
        this: WeakView<Self>,
        excerpt: Excerpt,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let project = this.update(cx, |this, _cx| this.project.clone())?;

        let worktree_id = project.update(cx, |project, cx| {
            let worktree = project.worktrees().next()?;
            let worktree_id = worktree.read(cx).id();
            Some(worktree_id)
        })?;

        let worktree_id = if let Some(worktree_id) = worktree_id {
            worktree_id
        } else {
            return Err(anyhow::anyhow!("No worktree found"));
        };

        let buffer_task = project.update(cx, |project, cx| {
            project.open_buffer(
                ProjectPath {
                    worktree_id,
                    path: Path::new(&excerpt.path).into(),
                },
                cx,
            )
        })?;

        let buffer = match buffer_task.await {
            Ok(buffer) => buffer,
            Err(error) => {
                return this.update(cx, |this, cx| {
                    this.error = Some(error);
                    cx.notify();
                })
            }
        };

        let snapshot = buffer.update(cx, |buffer, _cx| buffer.snapshot())?;
        let query = SearchQuery::text(&excerpt.text_passage, false, false, false, vec![], vec![])?;
        let matches = query.search(&snapshot, None).await;
        let Some(first_match) = matches.first() else {
            log::warn!(
                "text {:?} does not appear in '{}'",
                excerpt.text_passage,
                excerpt.path
            );
            return Ok(());
        };

        this.update(cx, |this, cx| {
            let mut start = first_match.start.to_point(&snapshot);
            start.column = 0;

            if let Some(editor) = &this.editor {
                editor.update(cx, |editor, cx| {
                    let ranges = editor.buffer().update(cx, |multibuffer, cx| {
                        multibuffer.push_excerpts_with_context_lines(
                            buffer.clone(),
                            vec![start..start],
                            5,
                            cx,
                        )
                    });

                    let annotation = SharedString::from(excerpt.annotation);
                    editor.insert_blocks(
                        [BlockProperties {
                            position: ranges[0].start,
                            height: annotation.split('\n').count() as u8 + 1,
                            style: BlockStyle::Fixed,
                            render: Box::new(move |cx| Self::render_note_block(&annotation, cx)),
                            disposition: BlockDisposition::Above,
                        }],
                        None,
                        cx,
                    );
                });

                if !this.added_editor_to_workspace {
                    this.added_editor_to_workspace = true;
                    this.workspace
                        .update(cx, |workspace, cx| {
                            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, cx);
                        })
                        .log_err();
                }
            }
        })?;

        Ok(())
    }

    fn render_note_block(explanation: &SharedString, cx: &mut BlockContext) -> AnyElement {
        let anchor_x = cx.anchor_x;
        let gutter_width = cx.gutter_dimensions.width;

        h_flex()
            .w_full()
            .py_2()
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .justify_center()
                    .w(gutter_width)
                    .child(Icon::new(IconName::Ai).color(Color::Hint)),
            )
            .child(
                h_flex()
                    .w_full()
                    .ml(anchor_x - gutter_width)
                    .child(explanation.clone()),
            )
            .into_any_element()
    }
}

impl Render for AnnotationResultView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        if let Some(error) = &self.error {
            ui::Label::new(error.to_string()).into_any_element()
        } else {
            ui::Label::new(SharedString::from(format!(
                "Opened a buffer with {} excerpts",
                self.rendered_excerpt_count
            )))
            .into_any_element()
        }
    }
}

impl ToolView for AnnotationResultView {
    type Input = AnnotationInput;
    type SerializedState = Option<String>;

    fn generate(&self, _: &mut ProjectContext, _: &mut ViewContext<Self>) -> String {
        if let Some(error) = &self.error {
            format!("Failed to create buffer: {error:?}")
        } else {
            format!(
                "opened {} excerpts in a buffer",
                self.rendered_excerpt_count
            )
        }
    }

    fn set_input(&mut self, mut input: Self::Input, cx: &mut ViewContext<Self>) {
        let editor = if let Some(editor) = &self.editor {
            editor.clone()
        } else {
            let multibuffer = cx.new_model(|_cx| {
                MultiBuffer::new(0, language::Capability::ReadWrite).with_title(String::new())
            });
            let editor = cx.new_view(|cx| {
                Editor::for_multibuffer(multibuffer.clone(), Some(self.project.clone()), cx)
            });

            self.editor = Some(editor.clone());
            editor
        };

        editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |multibuffer, cx| {
                if multibuffer.title(cx) != input.title {
                    multibuffer.set_title(input.title.clone(), cx);
                }
            });

            self.pending_excerpt = input.excerpts.pop();
            for excerpt in input.excerpts.iter().skip(self.rendered_excerpt_count) {
                self.tx.unbounded_send(excerpt.clone()).ok();
            }
            self.rendered_excerpt_count = input.excerpts.len();
        });

        cx.notify();
    }

    fn execute(&mut self, _cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        if let Some(excerpt) = self.pending_excerpt.take() {
            self.rendered_excerpt_count += 1;
            self.tx.unbounded_send(excerpt.clone()).ok();
        }

        self.tx.close_channel();
        Task::ready(Ok(()))
    }

    fn serialize(&self, _cx: &mut ViewContext<Self>) -> Self::SerializedState {
        self.error.as_ref().map(|error| error.to_string())
    }

    fn deserialize(
        &mut self,
        output: Self::SerializedState,
        _cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        if let Some(error_message) = output {
            self.error = Some(anyhow::anyhow!("{}", error_message));
        }
        Ok(())
    }
}
