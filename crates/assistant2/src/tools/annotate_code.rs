use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ProjectContext, ToolOutput};
use editor::{
    display_map::{BlockContext, BlockDisposition, BlockProperties, BlockStyle},
    Editor, MultiBuffer,
};
use gpui::{prelude::*, AnyElement, Model, Task, View, WeakView};
use language::ToPoint;
use project::{Project, ProjectPath};
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

#[derive(Debug, Deserialize, JsonSchema, Clone)]
pub struct AnnotationInput {
    /// Name for this set of annotations
    title: String,
    annotations: Vec<Annotation>,
}

#[derive(Debug, Deserialize, JsonSchema, Clone)]
struct Annotation {
    /// Path to the file
    path: String,
    /// Name of a symbol in the code
    symbol_name: String,
    /// Text to display near the symbol definition
    text: String,
}

impl LanguageModelTool for AnnotationTool {
    type Input = AnnotationInput;
    type Output = String;
    type View = AnnotationResultView;

    fn name(&self) -> String {
        "annotate_code".to_string()
    }

    fn description(&self) -> String {
        "Dynamically annotate symbols in the current codebase. Opens a buffer in a panel in their editor, to the side of the conversation. The annotations are shown in the editor as a block decoration.".to_string()
    }

    fn execute(&self, input: &Self::Input, cx: &mut WindowContext) -> Task<Result<Self::Output>> {
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let excerpts = input.annotations.clone();
        let title = input.title.clone();

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
            let excerpts = excerpts.clone();
            excerpts
                .iter()
                .map(|excerpt| {
                    let project_path = ProjectPath {
                        worktree_id,
                        path: Path::new(&excerpt.path).into(),
                    };
                    project.open_buffer(project_path.clone(), cx)
                })
                .collect::<Vec<_>>()
        });

        cx.spawn(move |mut cx| async move {
            let buffers = futures::future::try_join_all(buffer_tasks).await?;

            let multibuffer = cx.new_model(|_cx| {
                MultiBuffer::new(0, language::Capability::ReadWrite).with_title(title)
            })?;
            let editor =
                cx.new_view(|cx| Editor::for_multibuffer(multibuffer, Some(project), cx))?;

            for (excerpt, buffer) in excerpts.iter().zip(buffers.iter()) {
                let snapshot = buffer.update(&mut cx, |buffer, _cx| buffer.snapshot())?;

                if let Some(outline) = snapshot.outline(None) {
                    let matches = outline
                        .search(&excerpt.symbol_name, cx.background_executor().clone())
                        .await;
                    if let Some(mat) = matches.first() {
                        let item = &outline.items[mat.candidate_id];
                        let start = item.range.start.to_point(&snapshot);
                        editor.update(&mut cx, |editor, cx| {
                            let ranges = editor.buffer().update(cx, |multibuffer, cx| {
                                multibuffer.push_excerpts_with_context_lines(
                                    buffer.clone(),
                                    vec![start..start],
                                    5,
                                    cx,
                                )
                            });
                            let explanation = SharedString::from(excerpt.text.clone());
                            editor.insert_blocks(
                                [BlockProperties {
                                    position: ranges[0].start,
                                    height: 2,
                                    style: BlockStyle::Fixed,
                                    render: Box::new(move |cx| {
                                        Self::render_note_block(&explanation, cx)
                                    }),
                                    disposition: BlockDisposition::Above,
                                }],
                                None,
                                cx,
                            );
                        })?;
                    }
                }
            }

            workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.add_item_to_active_pane(Box::new(editor.clone()), None, cx);
                })
                .log_err();

            anyhow::Ok("showed comments to users in a new view".into())
        })
    }

    fn output_view(
        _: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> View<Self::View> {
        cx.new_view(|_cx| AnnotationResultView { output })
    }
}

impl AnnotationTool {
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
    output: Result<String>,
}

impl Render for AnnotationResultView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        match &self.output {
            Ok(output) => div().child(output.clone().into_any_element()),
            Err(error) => div().child(format!("failed to open path: {:?}", error)),
        }
    }
}

impl ToolOutput for AnnotationResultView {
    fn generate(&self, _: &mut ProjectContext, _: &mut WindowContext) -> String {
        match &self.output {
            Ok(output) => output.clone(),
            Err(err) => format!("Failed to create buffer: {err:?}"),
        }
    }
}
