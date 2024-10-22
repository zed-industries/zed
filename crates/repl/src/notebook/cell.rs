#![allow(unused, dead_code)]
use gpui::{prelude::*, View};
use serde::{Deserialize, Serialize};
use ui::prelude::*;
use uuid::Uuid;

use crate::notebook::{CODE_BLOCK_INSET, GUTTER_WIDTH};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct CellId(pub Uuid);

impl CellId {
    fn parse_id_or_default(id: Option<String>) -> Self {
        id.map(|id| CellId(Uuid::parse_str(&id).unwrap_or_default()))
            .unwrap_or_default()
    }
}

impl Default for CellId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for CellId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
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

// On disk format
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cell_type")]
pub enum DeserializedCell {
    #[serde(rename = "markdown")]
    Markdown {
        id: Option<String>,
        metadata: DeserializedCellMetadata,
        source: String,
        #[serde(default)]
        attachments: Option<serde_json::Value>,
    },
    #[serde(rename = "code")]
    Code {
        id: Option<String>,
        metadata: DeserializedCellMetadata,
        execution_count: Option<i32>,
        source: String,
        outputs: Vec<DeserializedOutput>,
    },
    #[serde(rename = "raw")]
    Raw {
        id: Option<String>,
        metadata: DeserializedCellMetadata,
        source: String,
    },
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
    Stream { name: String, text: String },
    #[serde(rename = "display_data")]
    DisplayData {
        data: serde_json::Value,
        metadata: serde_json::Value,
    },
    #[serde(rename = "execute_result")]
    ExecuteResult {
        execution_count: i32,
        data: serde_json::Value,
        metadata: serde_json::Value,
    },
    #[serde(rename = "error")]
    Error {
        ename: String,
        evalue: String,
        traceback: Vec<String>,
    },
}

/// A notebook cell
pub enum Cell {
    Code(CodeCell),
    Markdown(MarkdownCell),
    Raw(RawCell),
}

impl Cell {
    pub fn load(cell: DeserializedCell) -> Self {
        match cell {
            DeserializedCell::Markdown {
                id,
                metadata,
                source,
                attachments,
            } => Cell::Markdown(MarkdownCell {
                id: CellId::parse_id_or_default(id),
                cell_type: CellType::Markdown,
                metadata,
                source,
            }),
            DeserializedCell::Code {
                id,
                metadata,
                execution_count,
                source,
                outputs,
            } => Cell::Code(CodeCell {
                id: CellId::parse_id_or_default(id),
                cell_type: CellType::Code,
                metadata,
                execution_count,
                source,
                outputs,
            }),
            DeserializedCell::Raw {
                id,
                metadata,
                source,
            } => Cell::Raw(RawCell {
                id: CellId::parse_id_or_default(id),
                cell_type: CellType::Raw,
                metadata,
                source,
            }),
        }
    }
}

impl From<CodeCell> for Cell {
    fn from(cell: CodeCell) -> Self {
        Self::Code(cell)
    }
}

impl From<MarkdownCell> for Cell {
    fn from(cell: MarkdownCell) -> Self {
        Self::Markdown(cell)
    }
}

impl From<RawCell> for Cell {
    fn from(cell: RawCell) -> Self {
        Self::Raw(cell)
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
}

impl Default for MarkdownCell {
    fn default() -> Self {
        Self {
            id: Default::default(),
            cell_type: CellType::Markdown,
            metadata: Default::default(),
            source: "".to_string(),
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
        false
    }

    fn control(&self) -> Option<CellControl> {
        None
    }
}

impl Render for MarkdownCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

pub struct CodeCell {
    id: CellId,
    cell_type: CellType,
    metadata: DeserializedCellMetadata,
    execution_count: Option<i32>,
    source: String,
    outputs: Vec<DeserializedOutput>,
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

    fn selected(&self) -> bool {
        false
    }

    fn control(&self) -> Option<CellControl> {
        Some(CellControl::RunCell)
    }
}

impl Render for CodeCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}

pub struct RawCell {
    id: CellId,
    cell_type: CellType,
    metadata: DeserializedCellMetadata,
    source: String,
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

    fn selected(&self) -> bool {
        false
    }

    fn control(&self) -> Option<CellControl> {
        None
    }
}

impl Render for RawCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
    }
}
