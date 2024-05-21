use gpui::{AnyElement, FontWeight, Model, ModelContext, Render};
use runtimelib::{JupyterMessageContent, MimeType};
use ui::{div, prelude::*, v_flex, IntoElement, Styled, ViewContext};

use serde_json::Value;

use crate::ExecutionId;

use crate::stdio::TerminalOutput;

pub enum OutputType {
    Plain(TerminalOutput),
    Media((MimeType, Value)),
    Stream(TerminalOutput),
    ErrorOutput {
        ename: String,
        evalue: String,
        traceback: TerminalOutput,
    },
}

const PRIORITY_ORDER: &[MimeType] = &[MimeType::Plain, MimeType::Markdown];

impl OutputType {
    fn render(&self, cx: &ViewContext<ExecutionView>) -> Option<AnyElement> {
        let theme = cx.theme();

        let el = match self {
            // Note: in typical frontends we would show the execute_result.execution_count
            // Here we can just handle either
            Self::Plain(stdio) => Some(stdio.render(theme)),
            // Self::Markdown(markdown) => Some(markdown.render(theme)),
            Self::Media((mimetype, value)) => render_rich(mimetype, value),
            Self::Stream(stdio) => Some(stdio.render(theme)),
            Self::ErrorOutput {
                ename,
                evalue,
                traceback,
            } => render_error_output(ename, evalue, traceback, cx),
        };

        el
    }

    fn num_lines(&self) -> u8 {
        match self {
            Self::Plain(stdio) => stdio.num_lines(),
            Self::Media((_mimetype, value)) => value.as_str().unwrap_or("").lines().count() as u8,
            Self::Stream(stdio) => stdio.num_lines(),
            Self::ErrorOutput {
                ename,
                evalue,
                traceback,
            } => {
                let mut height: u8 = 0;
                height = height.saturating_add(ename.lines().count() as u8);
                height = height.saturating_add(evalue.lines().count() as u8);
                height = height.saturating_add(traceback.num_lines());
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
    ename: &String,
    evalue: &String,
    traceback: &TerminalOutput,
    cx: &ViewContext<ExecutionView>,
) -> Option<AnyElement> {
    let theme = cx.theme();

    let colors = cx.theme().colors();

    Some(
        v_flex()
            .w_full()
            .bg(colors.background)
            .p_4()
            .border_l_1()
            .border_color(theme.status().error_border)
            .child(
                h_flex()
                    .font_weight(FontWeight::BOLD)
                    .child(format!("{}: {}", ename, evalue)),
            )
            .child(traceback.render(theme))
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
    pub fn push_message(&mut self, message: &JupyterMessageContent, _cx: &mut ModelContext<Self>) {
        let output = match message {
            JupyterMessageContent::ExecuteResult(result) => {
                let (mimetype, value) =
                    if let Some((mimetype, value)) = result.data.richest(PRIORITY_ORDER) {
                        (mimetype, value)
                    } else {
                        // We don't support this media type, so just ignore it
                        return;
                    };

                match mimetype {
                    MimeType::Plain => {
                        OutputType::Plain(TerminalOutput::from(value.as_str().unwrap_or("")))
                    }
                    MimeType::Markdown => {
                        OutputType::Plain(TerminalOutput::from(value.as_str().unwrap_or("")))
                    }
                    // We don't handle this type, but ok
                    _ => OutputType::Media((mimetype, value)),
                }
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
                // Previous stream data will combine together, handling colors, carriage returns, etc
                if let Some(new_terminal) = self.apply_terminal_text(&result.text) {
                    new_terminal
                } else {
                    return;
                }
            }
            JupyterMessageContent::ErrorOutput(result) => {
                let mut terminal = TerminalOutput::new();

                terminal.append_text(&result.traceback.join("\n"));

                OutputType::ErrorOutput {
                    ename: result.ename.clone(),
                    evalue: result.evalue.clone(),
                    traceback: terminal,
                }
            }
            _msg => {
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
