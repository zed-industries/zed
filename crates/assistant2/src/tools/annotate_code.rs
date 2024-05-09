use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ProjectContext, ToolOutput};
use editor::{
    display_map::{BlockContext, BlockDisposition, BlockProperties, BlockStyle},
    Editor, MultiBuffer,
};
use gpui::{prelude::*, AnyElement, Model, Task, View, WeakView};
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
    title: String,
    /// Excerpts from the file to show to the user.
    excerpts: Vec<Excerpt>,
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
        cx.new_view(|_cx| AnnotationResultView {
            project: self.project.clone(),
            workspace: self.workspace.clone(),
            input: Default::default(),
            error: None,
        })
    }
}

impl AnnotationResultView {
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

pub struct AnnotationResultView {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    input: AnnotationInput,
    error: Option<anyhow::Error>,
}

impl Render for AnnotationResultView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        if let Some(error) = &self.error {
            div().child(error.to_string())
        } else {
            div().child(SharedString::from(format!(
                "opened {} excerpts in a buffer",
                self.input.excerpts.len()
            )))
        }
    }
}

impl ToolOutput for AnnotationResultView {
    type Input = AnnotationInput;
    type SerializedState = Option<String>;

    fn generate(&self, _: &mut ProjectContext, _: &mut ViewContext<Self>) -> String {
        if let Some(error) = &self.error {
            format!("Failed to create buffer: {error:?}")
        } else {
            format!("opened {} excerpts in a buffer", self.input.excerpts.len())
        }
    }

    fn set_input(&mut self, input: Self::Input, cx: &mut ViewContext<Self>) {
        self.input = input;
        cx.notify();
    }

    fn execute(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let excerpts = self.input.excerpts.clone();
        let title = self.input.title.clone();

        let worktree_id = project.update(cx, |project, cx| {
            let worktree = project.worktrees().next()?;
            let worktree_id = worktree.read(cx).id();
            Some(worktree_id)
        });

        let worktree_id = if let Some(worktree_id) = worktree_id {
            worktree_id
        } else {
            return Task::ready(Err(anyhow::anyhow!("No worktree found")));
        };

        let buffer_tasks = project.update(cx, |project, cx| {
            excerpts
                .iter()
                .map(|excerpt| {
                    project.open_buffer(
                        ProjectPath {
                            worktree_id,
                            path: Path::new(&excerpt.path).into(),
                        },
                        cx,
                    )
                })
                .collect::<Vec<_>>()
        });

        cx.spawn(move |this, mut cx| async move {
            let buffers = match futures::future::try_join_all(buffer_tasks).await {
                Ok(buffers) => buffers,
                Err(error) => {
                    return this.update(&mut cx, |this, cx| {
                        this.error = Some(error);
                        cx.notify();
                    })
                }
            };

            let multibuffer = cx.new_model(|_cx| {
                MultiBuffer::new(0, language::Capability::ReadWrite).with_title(title)
            })?;
            let editor =
                cx.new_view(|cx| Editor::for_multibuffer(multibuffer, Some(project), cx))?;

            for (excerpt, buffer) in excerpts.iter().zip(buffers.iter()) {
                let snapshot = buffer.update(&mut cx, |buffer, _cx| buffer.snapshot())?;

                let query =
                    SearchQuery::text(&excerpt.text_passage, false, false, false, vec![], vec![])?;

                let matches = query.search(&snapshot, None).await;
                let Some(first_match) = matches.first() else {
                    log::warn!(
                        "text {:?} does not appear in '{}'",
                        excerpt.text_passage,
                        excerpt.path
                    );
                    continue;
                };
                let mut start = first_match.start.to_point(&snapshot);
                start.column = 0;

                editor.update(&mut cx, |editor, cx| {
                    let ranges = editor.buffer().update(cx, |multibuffer, cx| {
                        multibuffer.push_excerpts_with_context_lines(
                            buffer.clone(),
                            vec![start..start],
                            5,
                            cx,
                        )
                    });
                    let annotation = SharedString::from(excerpt.annotation.clone());
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
                })?;
            }

            workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.add_item_to_active_pane(Box::new(editor.clone()), None, cx);
                })
                .log_err();

            Ok(())
        })
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
