use gpui::{AnyElement, FontWeight, Model, ModelContext, Render};
use runtimelib::{ErrorOutput, JupyterMessageContent, MimeType};
use ui::{div, prelude::*, v_flex, IntoElement, Styled, ViewContext};

use serde_json::Value;

use crate::ExecutionId;

use crate::stdio::TerminalOutput;

pub enum OutputType {
    Media((MimeType, Value)),
    Stream(TerminalOutput),
    ErrorOutput(ErrorOutput),
}

const PRIORITY_ORDER: &[MimeType] = &[MimeType::Plain, MimeType::Markdown];

impl OutputType {
    fn render(&self, cx: &ViewContext<ExecutionView>) -> Option<AnyElement> {
        let theme = cx.theme();

        let el = match self {
            // Note: in typical frontends we would show the execute_result.execution_count
            // Here we can just handle either
            Self::Media((mimetype, value)) => render_rich(mimetype, value),
            Self::Stream(stdio) => Some(stdio.render(theme)),
            Self::ErrorOutput(error_output) => render_error_output(&error_output, cx),
        };

        el
    }

    fn num_lines(&self) -> u8 {
        match self {
            Self::Media((_mimetype, value)) => value.as_str().unwrap_or("").lines().count() as u8,
            Self::Stream(stdio) => stdio.num_lines(),
            Self::ErrorOutput(error_output) => {
                let mut height: u8 = 1;

                height = height.saturating_add(error_output.ename.lines().count() as u8);
                // Note: skipping evalue in error output for now
                height = height.saturating_add(error_output.traceback.len() as u8);

                height
            }
        }
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

fn render_error_output(
    error_output: &ErrorOutput,
    cx: &ViewContext<ExecutionView>,
) -> Option<AnyElement> {
    let status_colors = cx.theme().status();

    Some(
        v_flex()
            .w_full()
            .bg(status_colors.error_background)
            .p_2()
            .border_1()
            .border_color(status_colors.error_border)
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
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

    pub fn num_lines(&self) -> u8 {
        self.outputs.iter().map(|output| output.num_lines()).sum()
    }

    /// Push a new message
    pub fn push_message(&mut self, message: &JupyterMessageContent, cx: &mut ModelContext<Self>) {
        let output = match message {
            JupyterMessageContent::ExecuteResult(result) => {
                let (mimetype, value) =
                    if let Some((mimetype, value)) = result.data.richest(PRIORITY_ORDER) {
                        (mimetype, value)
                    } else {
                        // We don't support this media type, so just ignore it
                        return;
                    };

                OutputType::Media((mimetype, value))
            }
            JupyterMessageContent::DisplayData(result) => {
                let (mimetype, value) =
                    if let Some((mimetype, value)) = result.data.richest(PRIORITY_ORDER) {
                        (mimetype, value)
                    } else {
                        // We don't support this media type, so just ignore it
                        return;
                    };

                OutputType::Media((mimetype, value))
            }
            JupyterMessageContent::StreamContent(result) => {
                if let Some(new_terminal) = self.apply_terminal_text(&result.text) {
                    new_terminal
                } else {
                    return;
                }
            }
            JupyterMessageContent::ErrorOutput(result) => OutputType::ErrorOutput(result.clone()),
            _ => {
                return;
            }
        };

        self.outputs.push(output);
    }

    fn apply_terminal_text(&mut self, text: &str) -> Option<OutputType> {
        // This doesn't handle the base case where there is no last output

        if let Some(last_output) = self.outputs.last_mut() {
            if let OutputType::Stream(last_stream) = last_output {
                last_stream.append_text(text);
                // Don't need to add a new output, we already have a terminal output
                return None;
            }
            // A different output type is "in the way", so we need to create a new output,
            // which is the same as having no prior output
        }

        let mut new_terminal = TerminalOutput::new();
        new_terminal.append_text(text);
        Some(OutputType::Stream(new_terminal))
    }
}

pub struct ExecutionView {
    pub execution: Model<Execution>,
}

impl ExecutionView {
    pub fn new(execution_id: ExecutionId, cx: &mut ViewContext<Self>) -> Self {
        let execution = cx.new_model(|cx| Execution::new(execution_id, cx));

        Self { execution }
    }

    pub fn push_message(&mut self, message: &JupyterMessageContent, cx: &mut ViewContext<Self>) {
        self.execution
            .update(cx, |execution, cx| execution.push_message(message, cx));

        cx.notify();
    }
}

impl Render for ExecutionView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let outputs = self.execution.read(cx).outputs();

        div()
            .w_full()
            .children(outputs.iter().filter_map(|output| output.render(cx)))
            .into_any_element()
    }
}
