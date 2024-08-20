use std::sync::Arc;
use std::time::Duration;

use crate::stdio::TerminalOutput;
use anyhow::Result;
use base64::prelude::*;
use gpui::{
    img, percentage, Animation, AnimationExt, AnyElement, FontWeight, Render, RenderImage, Task,
    TextRun, Transformation, View,
};
use runtimelib::datatable::TableSchema;
use runtimelib::media::datatable::TabularDataResource;
use runtimelib::{ExecutionState, JupyterMessageContent, MimeBundle, MimeType};
use serde_json::Value;
use settings::Settings;
use theme::ThemeSettings;
use ui::{div, prelude::*, v_flex, IntoElement, Styled, ViewContext};

use markdown_preview::{
    markdown_elements::ParsedMarkdown, markdown_parser::parse_markdown,
    markdown_renderer::render_markdown_block,
};

/// When deciding what to render from a collection of mediatypes, we need to rank them in order of importance
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
    image: Arc<RenderImage>,
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
        let bytes = BASE64_STANDARD.decode(base64_encoded_data)?;

        let format = image::guess_format(&bytes)?;
        let mut data = image::load_from_memory_with_format(&bytes, format)?.into_rgba8();

        // Convert from RGBA to BGRA.
        for pixel in data.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        let height = data.height();
        let width = data.width();

        let gpui_image_data = RenderImage::new(vec![image::Frame::new(data)]);

        return Ok(ImageView {
            height,
            width,
            image: Arc::new(gpui_image_data),
        });
    }
}

/// TableView renders a static table inline in a buffer.
/// It uses the https://specs.frictionlessdata.io/tabular-data-resource/ specification for data interchange.
pub struct TableView {
    pub table: TabularDataResource,
    pub widths: Vec<Pixels>,
}

fn cell_content(row: &Value, field: &str) -> String {
    match row.get(&field) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Array(arr)) => format!("{:?}", arr),
        Some(Value::Object(obj)) => format!("{:?}", obj),
        Some(Value::Null) | None => String::new(),
    }
}

// Declare constant for the padding multiple on the line height
const TABLE_Y_PADDING_MULTIPLE: f32 = 0.5;

impl TableView {
    pub fn new(table: TabularDataResource, cx: &mut WindowContext) -> Self {
        let mut widths = Vec::with_capacity(table.schema.fields.len());

        let text_system = cx.text_system();
        let text_style = cx.text_style();
        let text_font = ThemeSettings::get_global(cx).buffer_font.clone();
        let font_size = ThemeSettings::get_global(cx).buffer_font_size;
        let mut runs = [TextRun {
            len: 0,
            font: text_font,
            color: text_style.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        }];

        for field in table.schema.fields.iter() {
            runs[0].len = field.name.len();
            let mut width = text_system
                .layout_line(&field.name, font_size, &runs)
                .map(|layout| layout.width)
                .unwrap_or(px(0.));

            let Some(data) = table.data.as_ref() else {
                widths.push(width);
                continue;
            };

            for row in data {
                let content = cell_content(&row, &field.name);
                runs[0].len = content.len();
                let cell_width = cx
                    .text_system()
                    .layout_line(&content, font_size, &runs)
                    .map(|layout| layout.width)
                    .unwrap_or(px(0.));

                width = width.max(cell_width)
            }

            widths.push(width)
        }

        Self { table, widths }
    }

    pub fn render(&self, cx: &ViewContext<ExecutionView>) -> AnyElement {
        let data = match &self.table.data {
            Some(data) => data,
            None => return div().into_any_element(),
        };

        let mut headings = serde_json::Map::new();
        for field in &self.table.schema.fields {
            headings.insert(field.name.clone(), Value::String(field.name.clone()));
        }
        let header = self.render_row(&self.table.schema, true, &Value::Object(headings), cx);

        let body = data
            .iter()
            .map(|row| self.render_row(&self.table.schema, false, &row, cx));

        v_flex()
            .id("table")
            .overflow_x_scroll()
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

        let line_height = cx.line_height();

        let row_cells = schema
            .fields
            .iter()
            .zip(self.widths.iter())
            .map(|(field, width)| {
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

                let value = cell_content(row, &field.name);

                let mut cell = container
                    .min_w(*width + px(22.))
                    .w(*width + px(22.))
                    .child(value)
                    .px_2()
                    .py((TABLE_Y_PADDING_MULTIPLE / 2.0) * line_height)
                    .border_color(theme.colors().border);

                if is_header {
                    cell = cell.border_1().bg(theme.colors().border_focused)
                } else {
                    cell = cell.border_1()
                }
                cell
            })
            .collect::<Vec<_>>();

        let mut total_width = px(0.);
        for width in self.widths.iter() {
            // Width fudge factor: border + 2 (heading), padding
            total_width += *width + px(22.);
        }

        h_flex()
            .w(total_width)
            .children(row_cells)
            .into_any_element()
    }
}

/// Userspace error from the kernel
pub struct ErrorView {
    pub ename: String,
    pub evalue: String,
    pub traceback: TerminalOutput,
}

impl ErrorView {
    fn render(&self, cx: &mut ViewContext<ExecutionView>) -> Option<AnyElement> {
        let theme = cx.theme();

        let padding = cx.line_height() / 2.;

        Some(
            v_flex()
                .gap_3()
                .child(
                    h_flex()
                        .font_buffer(cx)
                        .child(
                            Label::new(format!("{}: ", self.ename.clone()))
                                // .size(LabelSize::Large)
                                .color(Color::Error)
                                .weight(FontWeight::BOLD),
                        )
                        .child(
                            Label::new(self.evalue.clone())
                                // .size(LabelSize::Large)
                                .weight(FontWeight::BOLD),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .px(padding)
                        .py(padding)
                        .border_l_1()
                        .border_color(theme.status().error_border)
                        .child(self.traceback.render(cx)),
                )
                .into_any_element(),
        )
    }
}

pub struct MarkdownView {
    contents: Option<ParsedMarkdown>,
    parsing_markdown_task: Option<Task<Result<()>>>,
}

impl MarkdownView {
    pub fn from(text: String, cx: &mut ViewContext<Self>) -> Self {
        let task = cx.spawn(|markdown, mut cx| async move {
            let text = text.clone();
            let parsed = cx
                .background_executor()
                .spawn(async move { parse_markdown(&text, None, None).await });

            let content = parsed.await;

            markdown.update(&mut cx, |markdown, cx| {
                markdown.parsing_markdown_task.take();
                markdown.contents = Some(content);
                cx.notify();
            })
        });

        Self {
            contents: None,
            parsing_markdown_task: Some(task),
        }
    }
}

impl Render for MarkdownView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(parsed) = self.contents.as_ref() else {
            return div().into_any_element();
        };

        let mut markdown_render_context =
            markdown_preview::markdown_renderer::RenderContext::new(None, cx);

        v_flex()
            .gap_3()
            .py_4()
            .children(parsed.children.iter().map(|child| {
                div().relative().child(
                    div()
                        .relative()
                        .child(render_markdown_block(child, &mut markdown_render_context)),
                )
            }))
            .into_any_element()
    }
}

pub struct Output {
    content: OutputContent,
    display_id: Option<String>,
}

impl Output {
    pub fn new(data: &MimeBundle, display_id: Option<String>, cx: &mut WindowContext) -> Self {
        Self {
            content: OutputContent::new(data, cx),
            display_id,
        }
    }

    pub fn from(content: OutputContent) -> Self {
        Self {
            content,
            display_id: None,
        }
    }
}

pub enum OutputContent {
    Plain(TerminalOutput),
    Stream(TerminalOutput),
    Image(ImageView),
    ErrorOutput(ErrorView),
    Message(String),
    Table(TableView),
    Markdown(View<MarkdownView>),
    ClearOutputWaitMarker,
}

impl OutputContent {
    fn render(&self, cx: &mut ViewContext<ExecutionView>) -> Option<AnyElement> {
        let el = match self {
            // Note: in typical frontends we would show the execute_result.execution_count
            // Here we can just handle either
            Self::Plain(stdio) => Some(stdio.render(cx)),
            Self::Markdown(markdown) => Some(markdown.clone().into_any_element()),
            Self::Stream(stdio) => Some(stdio.render(cx)),
            Self::Image(image) => Some(image.render(cx)),
            Self::Message(message) => Some(div().child(message.clone()).into_any_element()),
            Self::Table(table) => Some(table.render(cx)),
            Self::ErrorOutput(error_view) => error_view.render(cx),
            Self::ClearOutputWaitMarker => None,
        };

        el
    }

    pub fn new(data: &MimeBundle, cx: &mut WindowContext) -> Self {
        match data.richest(rank_mime_type) {
            Some(MimeType::Plain(text)) => OutputContent::Plain(TerminalOutput::from(text, cx)),
            Some(MimeType::Markdown(text)) => {
                let view = cx.new_view(|cx| MarkdownView::from(text.clone(), cx));
                OutputContent::Markdown(view)
            }
            Some(MimeType::Png(data)) | Some(MimeType::Jpeg(data)) => match ImageView::from(data) {
                Ok(view) => OutputContent::Image(view),
                Err(error) => OutputContent::Message(format!("Failed to load image: {}", error)),
            },
            Some(MimeType::DataTable(data)) => {
                OutputContent::Table(TableView::new(data.clone(), cx))
            }
            // Any other media types are not supported
            _ => OutputContent::Message("Unsupported media type".to_string()),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum ExecutionStatus {
    #[default]
    Unknown,
    ConnectingToKernel,
    Queued,
    Executing,
    Finished,
    ShuttingDown,
    Shutdown,
    KernelErrored(String),
}

pub struct ExecutionView {
    pub outputs: Vec<Output>,
    pub status: ExecutionStatus,
}

impl ExecutionView {
    pub fn new(status: ExecutionStatus, _cx: &mut ViewContext<Self>) -> Self {
        Self {
            outputs: Default::default(),
            status,
        }
    }

    /// Accept a Jupyter message belonging to this execution
    pub fn push_message(&mut self, message: &JupyterMessageContent, cx: &mut ViewContext<Self>) {
        let output: Output = match message {
            JupyterMessageContent::ExecuteResult(result) => Output::new(
                &result.data,
                result.transient.as_ref().and_then(|t| t.display_id.clone()),
                cx,
            ),
            JupyterMessageContent::DisplayData(result) => {
                Output::new(&result.data, result.transient.display_id.clone(), cx)
            }
            JupyterMessageContent::StreamContent(result) => {
                // Previous stream data will combine together, handling colors, carriage returns, etc
                if let Some(new_terminal) = self.apply_terminal_text(&result.text, cx) {
                    Output::from(new_terminal)
                } else {
                    return;
                }
            }
            JupyterMessageContent::ErrorOutput(result) => {
                let mut terminal = TerminalOutput::new(cx);
                terminal.append_text(&result.traceback.join("\n"));

                Output::from(OutputContent::ErrorOutput(ErrorView {
                    ename: result.ename.clone(),
                    evalue: result.evalue.clone(),
                    traceback: terminal,
                }))
            }
            JupyterMessageContent::ExecuteReply(reply) => {
                for payload in reply.payload.iter() {
                    match payload {
                        // Pager data comes in via `?` at the end of a statement in Python, used for showing documentation.
                        // Some UI will show this as a popup. For ease of implementation, it's included as an output here.
                        runtimelib::Payload::Page { data, .. } => {
                            let output = Output::new(data, None, cx);
                            self.outputs.push(output);
                        }

                        // There are other payloads that could be handled here, such as updating the input.
                        // Below are the other payloads that _could_ be handled, but are not required for Zed.

                        // Set next input adds text to the next cell. Not required to support.
                        // However, this could be implemented by adding text to the buffer.
                        // Trigger in python using `get_ipython().set_next_input("text")`
                        //
                        // runtimelib::Payload::SetNextInput { text, replace } => {},

                        // Not likely to be used in the context of Zed, where someone could just open the buffer themselves
                        // Python users can trigger this with the `%edit` magic command
                        // runtimelib::Payload::EditMagic { filename, line_number } => {},

                        // Ask the user if they want to exit the kernel. Not required to support.
                        // runtimelib::Payload::AskExit { keepkernel } => {},
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
                Output::from(OutputContent::ClearOutputWaitMarker)
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
        if let Some(output) = self.outputs.last() {
            if let OutputContent::ClearOutputWaitMarker = output.content {
                self.outputs.clear();
            }
        }

        self.outputs.push(output);

        cx.notify();
    }

    pub fn update_display_data(
        &mut self,
        data: &MimeBundle,
        display_id: &str,
        cx: &mut ViewContext<Self>,
    ) {
        let mut any = false;

        self.outputs.iter_mut().for_each(|output| {
            if let Some(other_display_id) = output.display_id.as_ref() {
                if other_display_id == display_id {
                    output.content = OutputContent::new(data, cx);
                    any = true;
                }
            }
        });

        if any {
            cx.notify();
        }
    }

    fn apply_terminal_text(
        &mut self,
        text: &str,
        cx: &mut ViewContext<Self>,
    ) -> Option<OutputContent> {
        if let Some(last_output) = self.outputs.last_mut() {
            match &mut last_output.content {
                OutputContent::Stream(last_stream) => {
                    last_stream.append_text(text);
                    // Don't need to add a new output, we already have a terminal output
                    cx.notify();
                    return None;
                }
                // Edge case note: a clear output marker
                OutputContent::ClearOutputWaitMarker => {
                    // Edge case note: a clear output marker is handled by the caller
                    // since we will return a new output at the end here as a new terminal output
                }
                // A different output type is "in the way", so we need to create a new output,
                // which is the same as having no prior output
                _ => {}
            }
        }

        let mut new_terminal = TerminalOutput::new(cx);
        new_terminal.append_text(text);
        Some(OutputContent::Stream(new_terminal))
    }
}

impl Render for ExecutionView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let status = match &self.status {
            ExecutionStatus::ConnectingToKernel => Label::new("Connecting to kernel...")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::Executing => h_flex()
                .gap_2()
                .child(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .with_animation(
                            "arrow-circle",
                            Animation::new(Duration::from_secs(3)).repeat(),
                            |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                        ),
                )
                .child(Label::new("Executing...").color(Color::Muted))
                .into_any_element(),
            ExecutionStatus::Finished => Icon::new(IconName::Check)
                .size(IconSize::Small)
                .into_any_element(),
            ExecutionStatus::Unknown => Label::new("Unknown status")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::ShuttingDown => Label::new("Kernel shutting down...")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::Shutdown => Label::new("Kernel shutdown")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::Queued => Label::new("Queued...")
                .color(Color::Muted)
                .into_any_element(),
            ExecutionStatus::KernelErrored(error) => Label::new(format!("Kernel error: {}", error))
                .color(Color::Error)
                .into_any_element(),
        };

        if self.outputs.len() == 0 {
            return v_flex()
                .min_h(cx.line_height())
                .justify_center()
                .child(status)
                .into_any_element();
        }

        div()
            .w_full()
            .children(
                self.outputs
                    .iter()
                    .filter_map(|output| output.content.render(cx)),
            )
            .children(match self.status {
                ExecutionStatus::Executing => vec![status],
                ExecutionStatus::Queued => vec![status],
                _ => vec![],
            })
            .into_any_element()
    }
}
