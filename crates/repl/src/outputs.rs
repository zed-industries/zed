use std::sync::Arc;

use crate::stdio::TerminalOutput;
use anyhow::Result;
use gpui::{img, AnyElement, FontWeight, ImageData, Render, View};
use runtimelib::datatable::TableSchema;
use runtimelib::media::datatable::TabularDataResource;
use runtimelib::{ExecutionState, JupyterMessageContent, MimeBundle, MimeType};
use serde_json::Value;
use ui::{div, prelude::*, v_flex, IntoElement, Styled, ViewContext};

// Given these outputs are destined for the editor with the block decorations API, all of them must report
// how many lines they will take up in the editor.
pub trait LineHeight: Sized {
    fn num_lines(&self, cx: &mut WindowContext) -> u8;
}

// When deciding what to render from a collection of mediatypes, we need to rank them in order of importance
fn rank_mime_type(mimetype: &MimeType) -> usize {
    match mimetype {
        MimeType::DataTable(_) => 6,
        MimeType::Png(_) => 4,
        MimeType::Jpeg(_) => 3,
        MimeType::Markdown(_) => 2,
        MimeType::Plain(_) => 1,
        // All other media types are not supported in Zed at this time
        _ => 0,
    }
}

/// ImageView renders an image inline in an editor, adapting to the line height to fit the image.
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

    fn from(base64_encoded_data: &str) -> Result<Self> {
        let bytes = base64::decode(base64_encoded_data)?;

        let format = image::guess_format(&bytes)?;
        let data = image::load_from_memory_with_format(&bytes, format)?.into_bgra8();

        let height = data.height();
        let width = data.width();

        let gpui_image_data = ImageData::new(data);

        return Ok(ImageView {
            height,
            width,
            image: Arc::new(gpui_image_data),
        });
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

/// TableView renders a static table inline in a buffer.
/// It uses the https://specs.frictionlessdata.io/tabular-data-resource/ specification for data interchange.
pub struct TableView {
    pub table: TabularDataResource,
}

impl TableView {
    pub fn render(&self, cx: &ViewContext<ExecutionView>) -> AnyElement {
        let data = match &self.table.data {
            Some(data) => data,
            None => return div().into_any_element(),
        };

        // todo!(): compute the width of each column by finding the widest cell in each column

        let mut headings = serde_json::Map::new();
        for field in &self.table.schema.fields {
            headings.insert(field.name.clone(), Value::String(field.name.clone()));
        }
        let header = self.render_row(&self.table.schema, true, &Value::Object(headings), cx);

        let body = data
            .iter()
            .map(|row| self.render_row(&self.table.schema, false, &row, cx));

        v_flex()
            .w_full()
            .child(header)
            .children(body)
            .into_any_element()
    }

    pub fn render_row(
        &self,
        schema: &TableSchema,
        is_header: bool,
        row: &Value,
        cx: &ViewContext<ExecutionView>,
    ) -> AnyElement {
        let theme = cx.theme();

        let row_cells = schema
            .fields
            .iter()
            .map(|field| {
                let container = match field.field_type {
                    runtimelib::datatable::FieldType::String => div(),

                    runtimelib::datatable::FieldType::Number
                    | runtimelib::datatable::FieldType::Integer
                    | runtimelib::datatable::FieldType::Date
                    | runtimelib::datatable::FieldType::Time
                    | runtimelib::datatable::FieldType::Datetime
                    | runtimelib::datatable::FieldType::Year
                    | runtimelib::datatable::FieldType::Duration
                    | runtimelib::datatable::FieldType::Yearmonth => v_flex().items_end(),

                    _ => div(),
                };

                let value = match row.get(&field.name) {
                    Some(Value::String(s)) => s.clone(),
                    Some(Value::Number(n)) => n.to_string(),
                    Some(Value::Bool(b)) => b.to_string(),
                    Some(Value::Array(arr)) => format!("{:?}", arr),
                    Some(Value::Object(obj)) => format!("{:?}", obj),
                    Some(Value::Null) | None => String::new(),
                };

                let mut cell = container
                    .w_full()
                    .child(value)
                    .px_2()
                    .py_1()
                    .border_color(theme.colors().border);

                if is_header {
                    cell = cell.border_2().bg(theme.colors().border_focused)
                } else {
                    cell = cell.border_1()
                }
                cell
            })
            .collect::<Vec<_>>();

        h_flex().children(row_cells).into_any_element()
    }
}

impl LineHeight for TableView {
    fn num_lines(&self, _cx: &mut WindowContext) -> u8 {
        let num_rows = match &self.table.data {
            Some(data) => data.len(),
            // We don't support Path based data sources
            None => 0,
        };

        // Given that each cell has both `py_1` and a border, we have to estimate
        // a reasonable size to add on, then round up.
        let row_heights = (num_rows as f32 * 1.2) + 1.0;

        (row_heights as u8).saturating_add(2) // Header + spacing
    }
}

// Userspace error from the kernel
pub struct ErrorView {
    pub ename: String,
    pub evalue: String,
    pub traceback: TerminalOutput,
}

impl ErrorView {
    fn render(&self, cx: &ViewContext<ExecutionView>) -> Option<AnyElement> {
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
                        .child(format!("{}: {}", self.ename, self.evalue)),
                )
                .child(self.traceback.render(cx))
                .into_any_element(),
        )
    }
}

impl LineHeight for ErrorView {
    fn num_lines(&self, cx: &mut WindowContext) -> u8 {
        let mut height: u8 = 0;
        height = height.saturating_add(self.ename.lines().count() as u8);
        height = height.saturating_add(self.evalue.lines().count() as u8);
        height = height.saturating_add(self.traceback.num_lines(cx));
        height
    }
}

pub enum OutputType {
    Plain(TerminalOutput),
    Stream(TerminalOutput),
    Image(ImageView),
    ErrorOutput(ErrorView),
    Message(String),
    Table(TableView),
    ClearOutputWaitMarker,
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
            Self::Table(table) => Some(table.render(cx)),
            Self::ErrorOutput(error_view) => error_view.render(cx),
            Self::ClearOutputWaitMarker => None,
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
            Self::Table(table) => table.num_lines(cx),
            Self::ErrorOutput(error_view) => error_view.num_lines(cx),
            Self::ClearOutputWaitMarker => 0,
        }
    }
}

impl From<&MimeBundle> for OutputType {
    fn from(data: &MimeBundle) -> Self {
        match data.richest(rank_mime_type) {
            Some(MimeType::Plain(text)) => OutputType::Plain(TerminalOutput::from(text)),
            Some(MimeType::Markdown(text)) => OutputType::Plain(TerminalOutput::from(text)),
            Some(MimeType::Png(data)) | Some(MimeType::Jpeg(data)) => match ImageView::from(data) {
                Ok(view) => OutputType::Image(view),
                Err(error) => OutputType::Message(format!("Failed to load image: {}", error)),
            },
            Some(MimeType::DataTable(data)) => OutputType::Table(TableView {
                table: data.clone(),
            }),
            // Any other media types are not supported
            _ => OutputType::Message("Unsupported media type".to_string()),
        }
    }
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
    pub outputs: Vec<OutputType>,
    pub status: ExecutionStatus,
}

impl ExecutionView {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            outputs: Default::default(),
            status: ExecutionStatus::Unknown,
        }
    }

    /// Accept a Jupyter message belonging to this execution
    pub fn push_message(&mut self, message: &JupyterMessageContent, cx: &mut ViewContext<Self>) {
        let output: OutputType = match message {
            JupyterMessageContent::ExecuteResult(result) => (&result.data).into(),
            JupyterMessageContent::DisplayData(result) => (&result.data).into(),
            JupyterMessageContent::StreamContent(result) => {
                // Previous stream data will combine together, handling colors, carriage returns, etc
                if let Some(new_terminal) = self.apply_terminal_text(&result.text) {
                    new_terminal
                } else {
                    cx.notify();
                    return;
                }
            }
            JupyterMessageContent::ErrorOutput(result) => {
                let mut terminal = TerminalOutput::new();
                terminal.append_text(&result.traceback.join("\n"));

                OutputType::ErrorOutput(ErrorView {
                    ename: result.ename.clone(),
                    evalue: result.evalue.clone(),
                    traceback: terminal,
                })
            }
            JupyterMessageContent::ExecuteReply(reply) => {
                for payload in reply.payload.iter() {
                    match payload {
                        // Pager data comes in via `?` at the end of a statement in Python, used for showing documentation.
                        // Some UI will show this as a popup. For ease of implementation, it's included as an output here.
                        runtimelib::Payload::Page { data, .. } => {
                            let output: OutputType = (data).into();
                            self.outputs.push(output);
                        }

                        // Set next input adds text to the next cell. Not required to support.
                        // However, this could be implemented by
                        // runtimelib::Payload::SetNextInput { text, replace } => todo!(),

                        // Not likely to be used in the context of Zed, where someone could just open the buffer themselves
                        // runtimelib::Payload::EditMagic { filename, line_number } => todo!(),

                        //
                        // runtimelib::Payload::AskExit { keepkernel } => todo!(),
                        _ => {}
                    }
                }
                cx.notify();
                return;
            }
            JupyterMessageContent::ClearOutput(options) => {
                if !options.wait {
                    self.outputs.clear();
                    cx.notify();
                    return;
                }

                // Create a marker to clear the output after we get in a new output
                OutputType::ClearOutputWaitMarker
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

        // Check for a clear output marker as the previous output, so we can clear it out
        if let Some(OutputType::ClearOutputWaitMarker) = self.outputs.last() {
            self.outputs.clear();
        }

        self.outputs.push(output);

        cx.notify();
    }

    fn apply_terminal_text(&mut self, text: &str) -> Option<OutputType> {
        if let Some(last_output) = self.outputs.last_mut() {
            match last_output {
                OutputType::Stream(last_stream) => {
                    last_stream.append_text(text);
                    // Don't need to add a new output, we already have a terminal output
                    return None;
                }
                // Edge case note: a clear output marker
                OutputType::ClearOutputWaitMarker => {
                    // Edge case note: a clear output marker is handled by the caller
                    // since we will return a new output at the end here as a new terminal output
                }
                // A different output type is "in the way", so we need to create a new output,
                // which is the same as having no prior output
                _ => {}
            }
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
