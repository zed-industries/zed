use gpui::prelude::*;
use serde::{Deserialize, Serialize};
use ui::prelude::*;
use uuid::Uuid;

use crate::notebook_ui::{CODE_BLOCK_INSET, GUTTER_WIDTH};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct CellId(pub Uuid);

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
        metadata: DeserializedCellMetadata,
        source: String,
        #[serde(default)]
        attachments: Option<serde_json::Value>,
    },
    #[serde(rename = "code")]
    Code {
        metadata: DeserializedCellMetadata,
        execution_count: Option<i32>,
        source: String,
        outputs: Vec<DeserializedOutput>,
    },
    #[serde(rename = "raw")]
    Raw {
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

pub trait RenderableCell: Render {
    const CELL_TYPE: CellType;
    // fn new(cx: &ViewContext<Self>) -> Self;
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

impl RenderableCell for MarkdownCell {
    const CELL_TYPE: CellType = CellType::Markdown;

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
    source: String,
    outputs: Vec<DeserializedOutput>,
}
