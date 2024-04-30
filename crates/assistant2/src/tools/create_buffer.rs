use anyhow::Result;
use assistant_tooling::LanguageModelTool;
use editor::Editor;
use gpui::{prelude::*, AppContext, Model, Task, View, WeakView};
use language::Buffer;
use project::Project;
use schemars::JsonSchema;
use serde::Deserialize;
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;

pub struct CreateBufferTool {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
}

impl CreateBufferTool {
    pub fn new(workspace: WeakView<Workspace>, project: Model<Project>) -> Self {
        Self { workspace, project }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateBufferInput {
    /// The contents of the buffer.
    text: String,

    /// The name of the language to use for the buffer.
    ///
    /// This should be a human-readable name, like "Rust", "JavaScript", or "Python".
    language: String,
}

pub struct CreateBufferOutput {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    buffer: Model<Buffer>,
}

impl LanguageModelTool for CreateBufferTool {
    type Input = CreateBufferInput;
    type Output = CreateBufferOutput;
    type View = CreateBufferView;

    fn name(&self) -> String {
        "create_buffer".to_string()
    }

    fn description(&self) -> String {
        "Create a new buffer in the current codebase".to_string()
    }

    fn execute(&self, input: &Self::Input, cx: &AppContext) -> Task<Result<Self::Output>> {
        cx.spawn({
            let workspace = self.workspace.clone();
            let project = self.project.clone();
            let text = input.text.clone();
            let language_name = input.language.clone();
            |cx| async move {
                let language = cx
                    .update(|cx| {
                        project
                            .read(cx)
                            .languages()
                            .language_for_name(&language_name)
                    })?
                    .await?;

                let buffer = cx.update(|cx| {
                    project.update(cx, |project, cx| {
                        project.create_buffer(&text, Some(language), cx)
                    })
                })??;

                Ok(CreateBufferOutput {
                    workspace,
                    project,
                    buffer,
                })
            }
        })
    }

    fn format(_input: &Self::Input, _output: &Result<Self::Output>) -> String {
        "".to_string()
    }

    fn output_view(
        _tool_call_id: String,
        _input: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> View<Self::View> {
        cx.new_view(|cx| match output {
            Ok(output) => {
                output
                    .workspace
                    .update(cx, |workspace, cx| {
                        workspace.add_item_to_active_pane(
                            Box::new(cx.new_view(|cx| {
                                Editor::for_buffer(output.buffer, Some(output.project), cx)
                            })),
                            None,
                            cx,
                        );
                    })
                    .log_err();

                CreateBufferView {}
            }
            Err(_) => CreateBufferView {},
        })
    }
}

pub struct CreateBufferView {}

impl Render for CreateBufferView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}
