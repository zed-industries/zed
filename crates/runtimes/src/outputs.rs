use std::sync::Arc;

use crate::stdio::TerminalOutput;
use crate::ExecutionId;
use anyhow::Result;
use gpui::{img, AnyElement, FontWeight, ImageData, Render, View};
use runtimelib::datatable::TableSchema;
use runtimelib::media::datatable::TabularDataResource;
use runtimelib::{ExecutionState, JupyterMessageContent, MimeBundle, MimeType};
use serde_json::Value;
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
    Table(TableView),
}

pub trait LineHeight: Sized {
    fn num_lines(&self, cx: &mut WindowContext) -> u8;
}

fn rank_mime_type(mimetype: &MimeType) -> usize {
    match mimetype {
        MimeType::DataTable(_) => 6,
        // SVG Rendering is incomplete so we don't show it
        // MimeType::Svg(_) => 5,
        MimeType::Png(_) => 4,
        MimeType::Jpeg(_) => 3,
        MimeType::Markdown(_) => 2,
        MimeType::Plain(_) => 1,
        _ => 0,
    }
}

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

    pub fn num_rows(&self) -> usize {
        match &self.table.data {
            Some(data) => data.len(),
            // We don't support Path based data sources
            None => 0,
        }
    }
}

impl LineHeight for TableView {
    fn num_lines(&self, _cx: &mut WindowContext) -> u8 {
        // Given that each cell has both `py_1` and a border, we have to estimate
        // a reasonable size to add on, then round up.
        let row_heights = (self.num_rows() as f32 * 1.2) + 1.0;

        (row_heights as u8).saturating_add(2)
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
            Self::Table(table) => Some(table.render(cx)),
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
            Self::Table(table) => table.num_lines(cx),
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

pub fn svg_to_vec(text: &str, scale: f32) -> Result<OutputType> {
    let tree = usvg::Tree::from_data(text.as_bytes(), &usvg::Options::default())?;

    let (height, width) = (tree.size().height() * scale, tree.size().width() * scale);

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width as u32, height as u32)
        .ok_or(usvg::Error::InvalidSize)?;

    let transform = tree.view_box().to_transform(
        resvg::tiny_skia::Size::from_wh(width, height).ok_or(usvg::Error::InvalidSize)?,
    );

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let data = image::load_from_memory_with_format(&pixmap.encode_png()?, image::ImageFormat::Png)?
        .into_bgra8();

    let gpui_image_data = ImageData::new(data);

    return Ok(OutputType::Image(ImageView {
        height: height as u32,
        width: width as u32,
        image: Arc::new(gpui_image_data),
    }));
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

impl From<&MimeBundle> for OutputType {
    fn from(data: &MimeBundle) -> Self {
        match data.richest(rank_mime_type) {
            Some(MimeType::Plain(text)) => OutputType::Plain(TerminalOutput::from(text)),
            Some(MimeType::Markdown(text)) => OutputType::Plain(TerminalOutput::from(text)),
            Some(MimeType::Svg(text)) => match svg_to_vec(text, 1.0) {
                Ok(output) => output,
                Err(error) => OutputType::Message(format!("Failed to load image: {}", error)),
            },
            Some(MimeType::Png(data)) | Some(MimeType::Jpeg(data)) => {
                match extract_image_output(&data) {
                    Ok(output) => output,
                    Err(error) => OutputType::Message(format!("Failed to load image: {}", error)),
                }
            }
            Some(MimeType::DataTable(data)) => OutputType::Table(TableView {
                table: data.clone(),
            }),
            // Any other media types are not supported
            _ => OutputType::Message("Unsupported media type".to_string()),
        }
    }
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
        let output: OutputType = match message {
            JupyterMessageContent::ExecuteResult(result) => (&result.data).into(),
            JupyterMessageContent::DisplayData(result) => (&result.data).into(),
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
