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

pub struct OpenBufferTool {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
}

impl OpenBufferTool {
    pub fn new(workspace: WeakView<Workspace>, project: Model<Project>) -> Self {
        Self { workspace, project }
    }
}

#[derive(Debug, Deserialize, JsonSchema, Clone)]
pub struct ExplainInput {
    /// Name for this set of excerpts
    title: String,
    /// Excerpts from the file to show to the user.
    excerpts: Vec<ExplainedExcerpt>,
}

#[derive(Debug, Deserialize, JsonSchema, Clone)]
struct ExplainedExcerpt {
    /// Path to the file
    path: String,
    /// A short, distinctive string that appears at a certain location in the file.
    text_passage: String,
    /// Text to display near the symbol definition
    comment: String,
}

impl LanguageModelTool for OpenBufferTool {
    type Input = ExplainInput;
    type Output = String;
    type View = OpenBufferView;

    fn name(&self) -> String {
        "explain_code".to_string()
    }

    fn description(&self) -> String {
        "Show and explain one or more code snippets from files in the current project. Code snippets are identified using a file path and the name of a symbol defined in that file.".to_string()
    }

    fn execute(&self, input: &Self::Input, cx: &mut WindowContext) -> Task<Result<Self::Output>> {
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let excerpts = input.excerpts.clone();
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

        cx.spawn(move |mut cx| async move {
            let buffers = futures::future::try_join_all(buffer_tasks).await?;

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
                    let explanation = SharedString::from(excerpt.comment.clone());
                    editor.insert_blocks(
                        [BlockProperties {
                            position: ranges[0].start,
                            height: 1,
                            style: BlockStyle::Fixed,
                            render: Box::new(move |cx| Self::render_note_block(&explanation, cx)),
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

            anyhow::Ok("showed comments to users in a new view".into())
        })
    }

    fn view(
        &self,
        _: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> View<Self::View> {
        cx.new_view(|_cx| OpenBufferView { output })
    }
}

impl OpenBufferTool {
    fn render_note_block(explanation: &SharedString, _cx: &mut BlockContext) -> AnyElement {
        div().child(explanation.clone()).into_any_element()
    }
}

pub struct OpenBufferView {
    output: Result<String>,
}

impl Render for OpenBufferView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        match &self.output {
            Ok(output) => div().child(output.clone().into_any_element()),
            Err(error) => div().child(format!("failed to open path: {:?}", error)),
        }
    }
}

impl ToolOutput for OpenBufferView {
    fn generate(&self, _: &mut ProjectContext, _: &mut WindowContext) -> String {
        match &self.output {
            Ok(output) => output.clone(),
            Err(err) => format!("Failed to create buffer: {err:?}"),
        }
    }
}
