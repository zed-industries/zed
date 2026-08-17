use crate::v4::{CellId, CellMetadata, Metadata, Output, deserialize_outputs, deserialize_source};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Notebook {
    pub metadata: Metadata,
    pub nbformat: i32,
    // Only 1-4 are supported via legacy at this time
    pub nbformat_minor: i32,
    #[serde(deserialize_with = "deserialize_cells")]
    pub cells: Vec<Cell>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "markdown")]
    Markdown {
        id: Option<CellId>,
        metadata: CellMetadata,
        #[serde(deserialize_with = "deserialize_source")]
        source: Vec<String>,
        #[serde(default)]
        attachments: Option<Value>,
    },
    #[serde(rename = "code")]
    Code {
        id: Option<CellId>,
        metadata: CellMetadata,
        execution_count: Option<i32>,
        #[serde(deserialize_with = "deserialize_source")]
        source: Vec<String>,
        #[serde(deserialize_with = "deserialize_outputs")]
        outputs: Vec<Output>,
    },
    #[serde(rename = "raw")]
    Raw {
        id: Option<CellId>,
        metadata: CellMetadata,
        #[serde(deserialize_with = "deserialize_source")]
        source: Vec<String>,
    },
}

pub fn deserialize_cells<'de, D>(deserializer: D) -> Result<Vec<Cell>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let cells: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    let mut result = Vec::with_capacity(cells.len());
    for (index, cell) in cells.into_iter().enumerate() {
        match serde_json::from_value::<Cell>(cell) {
            Ok(cell) => result.push(cell),
            Err(e) => {
                return Err(serde::de::Error::custom(format!(
                    "Failed to deserialize cell at index {}: {}",
                    index, e
                )));
            }
        }
    }
    Ok(result)
}
