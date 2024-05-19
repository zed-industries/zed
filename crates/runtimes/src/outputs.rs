use gpui::{AnyElement, Hsla, Model, ModelContext, Render};
use runtimelib::{ErrorOutput, JupyterMessageContent, MimeType};
use ui::{
    div, prelude::*, v_flex, Color, Context as _, IntoElement, ParentElement as _, SharedString,
    Styled, ViewContext, WindowContext,
};

use serde_json::Value;

use crate::ExecutionId;

#[derive(Clone, Debug)]
pub enum OutputType {
    Media((MimeType, Value)),
    Stream(SharedString),
    ErrorOutput(ErrorOutput),
}

const PRIORITY_ORDER: &[MimeType] = &[MimeType::Plain, MimeType::Markdown];

impl OutputType {
    fn render(&self, cx: &ViewContext<ExecutionView>) -> Option<AnyElement> {
        let el = match self {
            // Note: in typical frontends we would show the execute_result.execution_count
            // Here we can just handle either
            Self::Media((mimetype, value)) => render_rich(mimetype, value),
            Self::Stream(stdio) => render_stdio(stdio),
            Self::ErrorOutput(error_output) => render_error_output(&error_output, cx),
        };

        el
    }
}

fn render_rich(mimetype: &MimeType, value: &Value) -> Option<AnyElement> {
    // TODO: Make the media types be enums that contain their values to make this more readable
    match mimetype {
        MimeType::Plain => Some(
            div()
                .child(value.as_str().unwrap_or("").to_string())
                .into_any_element(),
        ),
        MimeType::Markdown => Some(
            div()
                .child(value.as_str().unwrap_or("").to_string())
                .into_any_element(),
        ),
        _ => None,
    }
}

fn render_stdio(stdio: &SharedString) -> Option<AnyElement> {
    // todo()!: process terminal colors, etc.

    // For aesthethics, trim the end of the string to remove trailing newlines.
    // Helps when using `console.log()`, `print`, and similar.
    let trimmed_string = stdio.trim_end().to_string();

    Some(
        div()
            .child(trimmed_string.into_any_element())
            .into_any_element(),
    )
}

fn render_error_output(
    error_output: &ErrorOutput,
    cx: &ViewContext<ExecutionView>,
) -> Option<AnyElement> {
    let status_colors = cx.theme().status();

    Some(
        v_flex()
            .bg(status_colors.error_background)
            .p_2()
            .border_1()
            .border_color(status_colors.error_border)
            .child(
                div()
                    .text_color(status_colors.error)
                    .child(error_output.ename.clone()),
            )
            .children(
                error_output
                    .traceback
                    .iter()
                    .map(|line| div().child(line.clone()).into_any_element()),
            )
            .into_any_element(),
    )
}

#[derive(Clone, Debug)]
pub struct Execution {
    pub execution_id: ExecutionId,
    // pub anchor: Anchor,
    pub outputs: Vec<OutputType>,
}

impl Execution {
    pub fn new(execution_id: ExecutionId, _cx: &mut ModelContext<Self>) -> Self {
        Self {
            execution_id,
            outputs: Default::default(),
        }
    }

    pub fn outputs(&self) -> &[OutputType] {
        &self.outputs
    }

    /// Push a new output and return the height of the output
    pub fn push_output(&mut self, output: &OutputType) -> u8 {
        // We're going to lean hard into this being an editor of text, and just use the height coming out of the
        // text output.
        let height = match output {
            // TODO: when we get stream input, we should combine it with all previous streams (before any other output type)
            // When we do that, we'll want to return the height of the _entire_ collection of outputs, not just this one.
            OutputType::Stream(text) => text.lines().count() as u8,
            OutputType::ErrorOutput(error_output) => {
                let mut height: u8 = 0;

                height = height.saturating_add(error_output.ename.lines().count() as u8);
                // Note: skipping evalue in error output for now
                height = height.saturating_add(error_output.traceback.len() as u8);

                height
            }
            OutputType::Media((_mime_type, value)) => {
                // Convert to string, and then count the lines
                let text = value.as_str().unwrap_or("").to_string();
                text.lines().count() as u8
            }
        };

        if height > 0 {
            self.outputs.push(output.clone());
        }
        height
    }
}

pub struct ExecutionView {
    execution: Model<Execution>,
}

impl ExecutionView {
    pub fn new(execution_id: ExecutionId, cx: &mut ViewContext<Self>) -> Self {
        let execution = cx.new_model(|cx| Execution::new(execution_id, cx));

        Self { execution }
    }

    pub fn push_message(
        &mut self,
        message: JupyterMessageContent,
        cx: &mut ViewContext<Self>,
    ) -> u8 {
        let height: u8 = match message {
            JupyterMessageContent::ExecuteResult(result) => {
                let (mimetype, value) =
                    if let Some((mimetype, value)) = result.data.richest(PRIORITY_ORDER) {
                        (mimetype, value)
                    } else {
                        return 0;
                    };

                self.execution.update(cx, |execution, _cx| {
                    execution.push_output(&OutputType::Media((mimetype, value)))
                })
            }
            JupyterMessageContent::DisplayData(result) => {
                let (mimetype, value) =
                    if let Some((mimetype, value)) = result.data.richest(PRIORITY_ORDER) {
                        (mimetype, value)
                    } else {
                        return 0;
                    };

                self.execution.update(cx, |execution, _cx| {
                    execution.push_output(&OutputType::Media((mimetype, value)))
                })
            }
            JupyterMessageContent::StreamContent(result) => {
                self.execution.update(cx, |execution, _cx| {
                    // TODO: Join previous stream content, if not broken up with displays, errors, etc.
                    execution.push_output(&OutputType::Stream(SharedString::from(result.text)))
                })
            }
            JupyterMessageContent::ErrorOutput(result) => {
                self.execution.update(cx, |execution, _cx| {
                    execution.push_output(&OutputType::ErrorOutput(result))
                })
            }
            _ => 0,
        };
        cx.notify();

        return height;
    }
}

impl Render for ExecutionView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let outputs = self.execution.read(cx).outputs();

        div()
            .children(outputs.iter().filter_map(|output| output.render(cx)))
            .into_any_element()
    }
}
