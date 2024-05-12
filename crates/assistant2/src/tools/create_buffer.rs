use anyhow::{anyhow, Result};
use assistant_tooling::{LanguageModelTool, ProjectContext, ToolView};
use editor::Editor;
use gpui::{prelude::*, Model, Task, View, WeakView};
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

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CreateBufferInput {
    /// The contents of the buffer.
    text: String,

    /// The name of the language to use for the buffer.
    ///
    /// This should be a human-readable name, like "Rust", "JavaScript", or "Python".
    language: String,
}

impl LanguageModelTool for CreateBufferTool {
    type View = CreateBufferView;

    fn name(&self) -> String {
        "create_file".to_string()
    }

    fn description(&self) -> String {
        "Create a new untitled file in the current codebase. Side effect: opens it in a new pane/tab for the user to edit.".to_string()
    }

    fn view(&self, cx: &mut WindowContext) -> View<Self::View> {
        cx.new_view(|_cx| CreateBufferView {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
            input: None,
            error: None,
        })
    }
}

pub struct CreateBufferView {
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    input: Option<CreateBufferInput>,
    error: Option<anyhow::Error>,
}

impl Render for CreateBufferView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        ui::Label::new("Opening a buffer")
    }
}

impl ToolView for CreateBufferView {
    type Input = CreateBufferInput;

    type SerializedState = ();

    fn generate(&self, _project: &mut ProjectContext, _cx: &mut ViewContext<Self>) -> String {
        let Some(input) = self.input.as_ref() else {
            return "No input".to_string();
        };

        match &self.error {
            None => format!("Created a new {} buffer", input.language),
            Some(err) => format!("Failed to create buffer: {err:?}"),
        }
    }

    fn set_input(&mut self, input: Self::Input, cx: &mut ViewContext<Self>) {
        self.input = Some(input);
        cx.notify();
    }

    fn execute(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        cx.spawn({
            let workspace = self.workspace.clone();
            let project = self.project.clone();
            let input = self.input.clone();
            |_this, mut cx| async move {
                let input = input.ok_or_else(|| anyhow!("no input"))?;

                let text = input.text.clone();
                let language_name = input.language.clone();
                let language = cx
                    .update(|cx| {
                        project
                            .read(cx)
                            .languages()
                            .language_for_name(&language_name)
                    })?
                    .await?;

                let buffer = cx
                    .update(|cx| project.update(cx, |project, cx| project.create_buffer(cx)))?
                    .await?;

                buffer.update(&mut cx, |buffer, cx| {
                    buffer.edit([(0..0, text)], None, cx);
                    buffer.set_language(Some(language), cx)
                })?;

                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.add_item_to_active_pane(
                            Box::new(
                                cx.new_view(|cx| Editor::for_buffer(buffer, Some(project), cx)),
                            ),
                            None,
                            cx,
                        );
                    })
                    .log_err();

                Ok(())
            }
        })
    }

    fn serialize(&self, _cx: &mut ViewContext<Self>) -> Self::SerializedState {
        ()
    }

    fn deserialize(
        &mut self,
        _output: Self::SerializedState,
        _cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        Ok(())
    }
}
