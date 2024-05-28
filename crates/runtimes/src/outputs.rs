use std::sync::Arc;

use crate::stdio::TerminalOutput;
use crate::ExecutionId;
use anyhow::Result;
use gpui::{img, AnyElement, FontWeight, ImageData, Render, View};
use runtimelib::{ExecutionState, JupyterMessageContent, MimeType};
use ui::{div, prelude::*, v_flex, IntoElement, Styled, ViewContext};

pub struct ImageView {
    height: u32,
    width: u32,
    image: Arc<ImageData>,
}

impl ImageView {
    fn render(&self, cx: &ViewContext<ExecutionView>) -> AnyElement {
        let line_height = cx.line_height();

        let (height, width) = if self.height as f32 / line_height.0 == u8::MAX as f32 {
            let height = u8::MAX as f32 * line_height.0;
            let width = self.width as f32 * height / self.height as f32;
            (height, width)
        } else {
            (self.height as f32, self.width as f32)
        };

        let image = self.image.clone();

        div()
            .h(Pixels(height))
            .w(Pixels(width))
            .child(img(image))
            .into_any_element()
    }
}

impl LineHeight for ImageView {
    fn num_lines(&self, cx: &mut WindowContext) -> u8 {
        let line_height = cx.line_height();

        let lines = self.height as f32 / line_height.0;

        if lines > u8::MAX as f32 {
            return u8::MAX;
        }
        lines as u8
    }
}

pub enum OutputType {
    Plain(TerminalOutput),
    Stream(TerminalOutput),
    Image(ImageView),
    ErrorOutput {
        ename: String,
        evalue: String,
        traceback: TerminalOutput,
    },
    Message(String),
}

pub trait LineHeight: Sized {
    fn num_lines(&self, cx: &mut WindowContext) -> u8;
}

fn rank_mime_type(mimetype: &MimeType) -> usize {
    match mimetype {
        MimeType::Png(_) => 4,
        MimeType::Jpeg(_) => 3,
        MimeType::Markdown(_) => 2,
        MimeType::Plain(_) => 1,
        _ => 0,
    }
}

impl OutputType {
    fn render(&self, cx: &ViewContext<ExecutionView>) -> Option<AnyElement> {
        let el = match self {
            // Note: in typical frontends we would show the execute_result.execution_count
            // Here we can just handle either
            Self::Plain(stdio) => Some(stdio.render(cx)),
            // Self::Markdown(markdown) => Some(markdown.render(theme)),
            Self::Stream(stdio) => Some(stdio.render(cx)),
            Self::Image(image) => Some(image.render(cx)),
            Self::Message(message) => Some(div().child(message.clone()).into_any_element()),
            Self::ErrorOutput {
                ename,
                evalue,
                traceback,
            } => render_error_output(ename, evalue, traceback, cx),
        };

        el
    }
}

impl LineHeight for OutputType {
    /// Calculates the expected number of lines
    fn num_lines(&self, cx: &mut WindowContext) -> u8 {
        match self {
            Self::Plain(stdio) => stdio.num_lines(cx),
            Self::Stream(stdio) => stdio.num_lines(cx),
            Self::Image(image) => image.num_lines(cx),
            Self::Message(message) => message.lines().count() as u8,
            Self::ErrorOutput {
                ename,
                evalue,
                traceback,
            } => {
                let mut height: u8 = 0;
                height = height.saturating_add(ename.lines().count() as u8);
                height = height.saturating_add(evalue.lines().count() as u8);
                height = height.saturating_add(traceback.num_lines(cx));
                height
            }
        }
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
            .child(traceback.render(cx))
            .into_any_element(),
    )
}

#[derive(Default)]
pub enum ExecutionStatus {
    #[default]
    Unknown,
    #[allow(unused)]
    ConnectingToKernel,
    Executing,
    Finished,
}

pub struct ExecutionView {
    pub execution_id: ExecutionId,
    pub outputs: Vec<OutputType>,
    pub status: ExecutionStatus,
}

pub fn extract_image_output(base64_encoded_data: &str) -> Result<OutputType> {
    let bytes = base64::decode(base64_encoded_data)?;

    let format = image::guess_format(&bytes)?;
    let data = image::load_from_memory_with_format(&bytes, format)?.into_bgra8();

    let height = data.height();
    let width = data.width();

    let gpui_image_data = ImageData::new(data);

    return Ok(OutputType::Image(ImageView {
        height,
        width,
        image: Arc::new(gpui_image_data),
    }));
}

impl ExecutionView {
    pub fn new(execution_id: ExecutionId, _cx: &mut ViewContext<Self>) -> Self {
        Self {
            execution_id,
            outputs: Default::default(),
            status: ExecutionStatus::Unknown,
        }
    }

    /// Accept a Jupyter message belonging to this execution
    pub fn push_message(&mut self, message: &JupyterMessageContent, cx: &mut ViewContext<Self>) {
        let output = match message {
            JupyterMessageContent::ExecuteResult(result) => {
                match result.data.richest(rank_mime_type) {
                    Some(MimeType::Plain(text)) => OutputType::Plain(TerminalOutput::from(text)),
                    Some(MimeType::Markdown(text)) => OutputType::Plain(TerminalOutput::from(text)),
                    Some(MimeType::Png(data)) | Some(MimeType::Jpeg(data)) => {
                        match extract_image_output(&data) {
                            Ok(output) => output,
                            Err(error) => {
                                OutputType::Message(format!("Failed to load image: {}", error))
                            }
                        }
                    }
                    // Any other media types are not supported
                    _ => return,
                }
            }
            JupyterMessageContent::DisplayData(result) => {
                match result.data.richest(rank_mime_type) {
                    Some(MimeType::Plain(text)) => OutputType::Plain(TerminalOutput::from(text)),
                    Some(MimeType::Markdown(text)) => OutputType::Plain(TerminalOutput::from(text)),
                    Some(MimeType::Png(data)) | Some(MimeType::Jpeg(data)) => {
                        match extract_image_output(&data) {
                            Ok(output) => output,
                            Err(error) => {
                                OutputType::Message(format!("Failed to load image: {}", error))
                            }
                        }
                    }
                    // Any other media types are not supported
                    _ => return,
                }
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
            JupyterMessageContent::Status(status) => {
                match status.execution_state {
                    ExecutionState::Busy => {
                        self.status = ExecutionStatus::Executing;
                    }
                    ExecutionState::Idle => self.status = ExecutionStatus::Finished,
                }
                cx.notify();
                return;
            }
            _msg => {
                return;
            }
        };

        self.outputs.push(output);

        cx.notify();
    }

    fn apply_terminal_text(&mut self, text: &str) -> Option<OutputType> {
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

    pub fn set_status(&mut self, status: ExecutionStatus, cx: &mut ViewContext<Self>) {
        self.status = status;
        cx.notify();
    }
}

impl Render for ExecutionView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if self.outputs.len() == 0 {
            match self.status {
                ExecutionStatus::ConnectingToKernel => {
                    return div().child("Connecting to kernel...").into_any_element()
                }
                ExecutionStatus::Executing => {
                    return div().child("Executing...").into_any_element()
                }
                ExecutionStatus::Finished => {
                    return div().child(Icon::new(IconName::Check)).into_any_element()
                }
                ExecutionStatus::Unknown => return div().child("...").into_any_element(),
            }
        }

        div()
            .w_full()
            .children(self.outputs.iter().filter_map(|output| output.render(cx)))
            .into_any_element()
    }
}

impl LineHeight for ExecutionView {
    fn num_lines(&self, cx: &mut WindowContext) -> u8 {
        if self.outputs.is_empty() {
            return 1; // For the status message if outputs are not there
        }

        self.outputs
            .iter()
            .map(|output| output.num_lines(cx))
            .fold(0, |acc, additional_height| {
                acc.saturating_add(additional_height)
            })
    }
}

impl LineHeight for View<ExecutionView> {
    fn num_lines(&self, cx: &mut WindowContext) -> u8 {
        self.update(cx, |execution_view, cx| execution_view.num_lines(cx))
    }
}
