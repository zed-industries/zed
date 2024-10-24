#![allow(unused, dead_code)]
use core::fmt;
use std::fmt::{Display, Formatter};

use gpui::{prelude::*, Hsla, View};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ui::prelude::*;
use uuid::Uuid;

use crate::{
    notebook::{CODE_BLOCK_INSET, GUTTER_WIDTH},
    outputs::{plain::TerminalOutput, user_error::ErrorView, Output},
};
use runtimelib::{DisplayData, ErrorOutput, ExecuteResult, StreamContent};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct CellId(String);

impl Display for CellId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for CellId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl From<Uuid> for CellId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid.to_string())
    }
}

impl From<String> for CellId {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl From<Option<String>> for CellId {
    fn from(string: Option<String>) -> Self {
        if string.is_some() {
            string.into()
        } else {
            Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Code,
    Markdown,
    // parse cell source as usual -> render as plain text
    Raw,
}

#[derive(IntoElement)]
pub enum CellControl {
    RunCell,
}

impl CellControl {
    fn icon_name(&self) -> IconName {
        match self {
            CellControl::RunCell => IconName::Play,
        }
    }
}

impl RenderOnce for CellControl {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cell_type")]
pub enum DeserializedCell {
    #[serde(rename = "markdown")]
    Markdown {
        id: Option<String>,
        metadata: DeserializedCellMetadata,
        source: Vec<String>,
        #[serde(default)]
        attachments: Option<Value>,
    },
    #[serde(rename = "code")]
    Code {
        id: Option<String>,
        metadata: DeserializedCellMetadata,
        execution_count: Option<i32>,
        source: Vec<String>,
        #[serde(deserialize_with = "deserialize_outputs")]
        outputs: Vec<DeserializedOutput>,
    },
    #[serde(rename = "raw")]
    Raw {
        id: Option<String>,
        metadata: DeserializedCellMetadata,
        source: Vec<String>,
    },
}

pub fn deserialize_cells<'de, D>(deserializer: D) -> Result<Vec<DeserializedCell>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let cells: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    cells
        .into_iter()
        .enumerate()
        .filter_map(
            |(index, cell)| match serde_json::from_value::<DeserializedCell>(cell) {
                Ok(cell) => Some(Ok(cell)),
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to deserialize cell at index {}: {}",
                        index, e
                    );
                    None
                }
            },
        )
        .collect()
}

// importing a notebook -> deserialize
// prefer to keep data as it was over adding optional fields
// when we serialize

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedCellMetadata {
    // https://nbformat.readthedocs.io/en/latest/format_description.html#cell-ids
    id: Option<String>, // make one once we load it. -> use uuid
    collapsed: Option<bool>,
    scrolled: Option<serde_json::Value>,
    deletable: Option<bool>,
    editable: Option<bool>,
    format: Option<String>,
    name: Option<String>,
    tags: Option<Vec<String>>,
}

impl Default for DeserializedCellMetadata {
    fn default() -> Self {
        Self {
            id: None,
            collapsed: None,
            scrolled: None,
            deletable: None,
            editable: None,
            format: None,
            name: None,
            tags: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "output_type")]
pub enum DeserializedOutput {
    #[serde(rename = "stream")]
    Stream(StreamContent),
    #[serde(rename = "display_data")]
    DisplayData(DisplayData),
    #[serde(rename = "execute_result")]
    ExecuteResult(ExecuteResult),
    #[serde(rename = "error")]
    Error(ErrorOutput),
}

pub fn deserialize_outputs<'de, D>(deserializer: D) -> Result<Vec<DeserializedOutput>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let outputs: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    outputs
        .into_iter()
        .enumerate()
        .filter_map(|(index, output)| {
            match serde_json::from_value::<DeserializedOutput>(output.clone()) {
                Ok(output) => Some(Ok(output)),
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to deserialize output at index {} of cell: {}",
                        index, e
                    );
                    eprintln!(
                        "Output JSON: {}",
                        serde_json::to_string_pretty(&output).unwrap_or_default()
                    );
                    None
                }
            }
        })
        .collect()
}

/// A notebook cell
#[derive(Clone)]
pub enum Cell {
    Code(View<CodeCell>),
    Markdown(View<MarkdownCell>),
    Raw(View<RawCell>),
}

fn convert_outputs(outputs: Vec<DeserializedOutput>, cx: &mut WindowContext) -> Vec<Output> {
    outputs
        .into_iter()
        .map(|output| match output {
            DeserializedOutput::Stream(stream) => Output::Stream {
                content: cx.new_view(|cx| TerminalOutput::from(&stream.text, cx)),
            },
            DeserializedOutput::DisplayData(display_data) => Output::new(
                &display_data.data,
                display_data.transient.display_id.clone(),
                cx,
            ),
            DeserializedOutput::ExecuteResult(execute_result) => Output::new(
                &execute_result.data,
                execute_result
                    .transient
                    .as_ref()
                    .and_then(|t| t.display_id.clone()),
                cx,
            ),
            DeserializedOutput::Error(error) => Output::ErrorOutput(ErrorView {
                ename: error.ename,
                evalue: error.evalue,
                traceback: cx.new_view(|cx| TerminalOutput::from(&error.traceback.join("\n"), cx)),
            }),
        })
        .collect()
}

impl Cell {
    pub fn load(cell: DeserializedCell, cx: &mut WindowContext) -> Self {
        match cell {
            DeserializedCell::Markdown {
                id,
                metadata,
                source,
                attachments,
            } => Cell::Markdown(cx.new_view(|cx| MarkdownCell {
                id: id.into(),
                cell_type: CellType::Markdown,
                metadata,
                source: source.join("\n"),
                selected: false,
            })),
            DeserializedCell::Code {
                id,
                metadata,
                execution_count,
                source,
                outputs,
            } => Cell::Code(cx.new_view(|cx| CodeCell {
                id: id.into(),
                cell_type: CellType::Code,
                metadata,
                execution_count,
                source: source.join("\n"),
                outputs: convert_outputs(outputs, cx),
                selected: false,
            })),
            DeserializedCell::Raw {
                id,
                metadata,
                source,
            } => Cell::Raw(cx.new_view(|_| RawCell {
                id: id.into(),
                cell_type: CellType::Raw,
                metadata,
                source: source.join("\n"),
                selected: false,
            })),
        }
    }
}

pub trait RenderableCell: Render {
    const CELL_TYPE: CellType;
    // fn new(cx: &mut WindowContext) -> View<Self>;
    fn id(&self) -> &CellId;
    fn cell_type(&self) -> &CellType;
    fn metadata(&self) -> &DeserializedCellMetadata;
    fn source(&self) -> &String;
    fn selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool) -> &mut Self;
    fn selected_bg_color(&self, cx: &ViewContext<Self>) -> Hsla {
        if self.selected() {
            let mut color = cx.theme().colors().icon_accent;
            color.fade_out(0.9);
            color
        } else {
            // TODO: this is wrong
            cx.theme().colors().tab_bar_background
        }
    }
    fn control(&self) -> Option<CellControl> {
        None
    }
    fn gutter(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let is_selected = self.selected();

        div()
            .relative()
            .h_full()
            .w(px(GUTTER_WIDTH))
            .child(
                div()
                    .w(px(GUTTER_WIDTH))
                    .flex()
                    .flex_none()
                    .justify_center()
                    .h_full()
                    .child(
                        div()
                            .flex_none()
                            .w(px(1.))
                            .h_full()
                            .when(is_selected, |this| this.bg(cx.theme().colors().icon_accent))
                            .when(!is_selected, |this| this.bg(cx.theme().colors().border)),
                    ),
            )
            .when_some(self.control(), |this, control| {
                this.child(
                    div()
                        .absolute()
                        .top(px(CODE_BLOCK_INSET - 2.0))
                        .left_0()
                        .flex()
                        .flex_none()
                        .w(px(GUTTER_WIDTH))
                        .h(px(GUTTER_WIDTH + 12.0))
                        .items_center()
                        .justify_center()
                        .bg(cx.theme().colors().tab_bar_background)
                        .child(IconButton::new("control", control.icon_name())),
                )
            })
    }
}

pub struct MarkdownCell {
    id: CellId,
    cell_type: CellType,
    metadata: DeserializedCellMetadata,
    source: String,
    selected: bool,
}

impl Default for MarkdownCell {
    fn default() -> Self {
        Self {
            id: Default::default(),
            cell_type: CellType::Markdown,
            metadata: Default::default(),
            source: "".to_string(),
            selected: false,
        }
    }
}

impl RenderableCell for MarkdownCell {
    const CELL_TYPE: CellType = CellType::Markdown;

    // fn new(cx: &mut WindowContext) -> View<Self> {
    //     cx.new_view(|cx| MarkdownCell::default())
    // }

    fn id(&self) -> &CellId {
        &self.id
    }

    fn cell_type(&self) -> &CellType {
        &self.cell_type
    }

    fn metadata(&self) -> &DeserializedCellMetadata {
        &self.metadata
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    fn control(&self) -> Option<CellControl> {
        None
    }
}

impl Render for MarkdownCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .pr_3()
            .items_start()
            .gap(Spacing::Large.rems(cx))
            .bg(self.selected_bg_color(cx))
            .child(self.gutter(cx))
            .child(
                div()
                    .flex()
                    .size_full()
                    .flex_1()
                    .p_3()
                    .font_ui(cx)
                    .text_size(TextSize::Default.rems(cx))
                    .child(self.source.clone()),
            )
    }
}

pub struct CodeCell {
    id: CellId,
    cell_type: CellType,
    metadata: DeserializedCellMetadata,
    execution_count: Option<i32>,
    source: String,
    outputs: Vec<Output>,
    selected: bool,
}

impl CodeCell {
    pub fn has_outputs(&self) -> bool {
        !self.outputs.is_empty()
    }

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }
}

impl RenderableCell for CodeCell {
    const CELL_TYPE: CellType = CellType::Code;

    fn id(&self) -> &CellId {
        &self.id
    }

    fn cell_type(&self) -> &CellType {
        &self.cell_type
    }

    fn metadata(&self) -> &DeserializedCellMetadata {
        &self.metadata
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn control(&self) -> Option<CellControl> {
        Some(CellControl::RunCell)
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }
}

impl Render for CodeCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .pr_3()
            .items_start()
            .gap(Spacing::Large.rems(cx))
            .bg(self.selected_bg_color(cx))
            .child(self.gutter(cx))
            .child(
                div().py_1p5().w_full().child(
                    div()
                        .flex()
                        .size_full()
                        .flex_1()
                        .py_3()
                        .px_5()
                        .rounded_lg()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().editor_background)
                        .font_buffer(cx)
                        .text_size(TextSize::Editor.rems(cx))
                        .child(self.source.clone()),
                ),
            )
    }
}

pub struct RawCell {
    id: CellId,
    cell_type: CellType,
    metadata: DeserializedCellMetadata,
    source: String,
    selected: bool,
}

impl RenderableCell for RawCell {
    const CELL_TYPE: CellType = CellType::Raw;

    fn id(&self) -> &CellId {
        &self.id
    }

    fn cell_type(&self) -> &CellType {
        &self.cell_type
    }

    fn metadata(&self) -> &DeserializedCellMetadata {
        &self.metadata
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn control(&self) -> Option<CellControl> {
        None
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }
}

impl Render for RawCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .pr_3()
            .items_start()
            .gap(Spacing::Large.rems(cx))
            .bg(self.selected_bg_color(cx))
            .child(self.gutter(cx))
            .child(
                div()
                    .flex()
                    .size_full()
                    .flex_1()
                    .p_3()
                    .font_ui(cx)
                    .text_size(TextSize::Default.rems(cx))
                    .child(self.source.clone()),
            )
    }
}
