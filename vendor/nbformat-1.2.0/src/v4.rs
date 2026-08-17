use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;
use uuid::Uuid;

use jupyter_protocol::{ExecutionCount, media::Media, media::serialize_media_for_notebook};

use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct MultilineString(pub String);

impl Serialize for MultilineString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let lines: Vec<String> = if self.0.is_empty() {
            vec!["".to_string()]
        } else {
            self.0.split_inclusive('\n').map(|s| s.to_string()).collect()
        };
        serializer.collect_seq(lines)
    }
}

impl<'de> Deserialize<'de> for MultilineString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(MultilineString(s))
    }
}
fn deserialize_multiline_string<'de, D>(deserializer: D) -> Result<MultilineString, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct MultilineStringVisitor;

    impl<'de> serde::de::Visitor<'de> for MultilineStringVisitor {
        type Value = MultilineString;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or array of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(MultilineString(value.to_string()))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(MultilineString(value))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut result = String::new();
            while let Some(s) = seq.next_element::<String>()? {
                result.push_str(&s);
            }
            Ok(MultilineString(result))
        }
    }

    deserializer.deserialize_any(MultilineStringVisitor)
}

pub(crate) fn deserialize_source<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct SourceVisitor;

    impl<'de> serde::de::Visitor<'de> for SourceVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or array of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![value.to_string()])
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![value])
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(s) = seq.next_element::<String>()? {
                result.push(s);
            }
            Ok(result)
        }
    }

    deserializer.deserialize_any(SourceVisitor)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DisplayData {
    #[serde(serialize_with = "serialize_media_for_notebook")]
    pub data: Media,
    pub metadata: serde_json::Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteResult {
    pub execution_count: ExecutionCount,
    #[serde(serialize_with = "serialize_media_for_notebook")]
    pub data: Media,
    pub metadata: serde_json::Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorOutput {
    pub ename: String,
    pub evalue: String,
    pub traceback: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Notebook {
    pub metadata: Metadata,
    pub nbformat: i32,
    pub nbformat_minor: i32,
    #[serde(deserialize_with = "deserialize_cells")]
    pub cells: Vec<Cell>,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernelspec: Option<KernelSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_info: Option<LanguageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Author>>,
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KernelSpec {
    pub display_name: String,
    pub name: String,
    pub language: Option<String>,
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LanguageInfo {
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub codemirror_mode: Option<CodemirrorMode>,
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum CodemirrorMode {
    String(String),
    Object(Value),
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct CellId(String);

impl CellId {
    fn is_valid(s: &str) -> bool {
        !s.is_empty()
            && s.len() <= 64
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }

    pub fn new(s: &str) -> Result<Self, &'static str> {
        if Self::is_valid(s) {
            Ok(CellId(s.to_string()))
        } else {
            Err("Invalid cell ID")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for CellId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for CellId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for CellId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        CellId::new(&s).map_err(de::Error::custom)
    }
}

impl From<Uuid> for CellId {
    fn from(uuid: Uuid) -> Self {
        // Assume UUID is always valid for CellId
        CellId(uuid.to_string())
    }
}

impl TryFrom<String> for CellId {
    type Error = &'static str;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        CellId::new(&string)
    }
}

impl<'a> TryFrom<&'a str> for CellId {
    type Error = &'static str;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        CellId::new(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Code,
    Markdown,
    Raw,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "markdown")]
    Markdown {
        id: CellId,
        metadata: CellMetadata,
        #[serde(deserialize_with = "deserialize_source")]
        source: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attachments: Option<Value>,
    },
    #[serde(rename = "code")]
    Code {
        id: CellId,
        metadata: CellMetadata,
        execution_count: Option<i32>,
        #[serde(deserialize_with = "deserialize_source")]
        source: Vec<String>,
        #[serde(deserialize_with = "deserialize_outputs")]
        outputs: Vec<Output>,
    },
    #[serde(rename = "raw")]
    Raw {
        id: CellId,
        metadata: CellMetadata,
        #[serde(deserialize_with = "deserialize_source")]
        source: Vec<String>,
    },
}

impl Cell {
    pub fn id(&self) -> &CellId {
        match self {
            Cell::Markdown { id, .. } => id,
            Cell::Code { id, .. } => id,
            Cell::Raw { id, .. } => id,
        }
    }

    pub fn metadata(&self) -> &CellMetadata {
        match self {
            Cell::Markdown { metadata, .. } => metadata,
            Cell::Code { metadata, .. } => metadata,
            Cell::Raw { metadata, .. } => metadata,
        }
    }

    pub fn source(&self) -> &[String] {
        match self {
            Cell::Markdown { source, .. } => source,
            Cell::Code { source, .. } => source,
            Cell::Raw { source, .. } => source,
        }
    }
}

use std::collections::HashSet;

fn validate_unique_cell_ids(cells: &[Cell]) -> Result<(), String> {
    let mut seen_ids = HashSet::new();
    for cell in cells {
        let id = match cell {
            Cell::Markdown { id, .. } => id,
            Cell::Code { id, .. } => id,
            Cell::Raw { id, .. } => id,
        };
        if !seen_ids.insert(id) {
            return Err(format!("Duplicate Cell ID found: {}", id));
        }
    }
    Ok(())
}

pub fn deserialize_cells<'de, D>(deserializer: D) -> Result<Vec<Cell>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let cells: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    let deserialized_cells: Vec<Cell> = cells
        .into_iter()
        .enumerate()
        .map(|(index, cell)| {
            serde_json::from_value::<Cell>(cell).map_err(|e| {
                de::Error::custom(format!(
                    "Failed to deserialize cell at index {}: {}",
                    index, e
                ))
            })
        })
        .collect::<Result<_, _>>()?;

    validate_unique_cell_ids(&deserialized_cells).map_err(de::Error::custom)?;
    Ok(deserialized_cells)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CellMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrolled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jupyter: Option<JupyterCellMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionMetadata>,
    // For retaining any additional fields introduced by other jupyter clients
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JupyterCellMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs_hidden: Option<bool>,
    // For retaining any additional fields introduced by other jupyter clients
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ExecutionMetadata {
    #[serde(
        rename = "iopub.execute_input",
        skip_serializing_if = "Option::is_none"
    )]
    pub iopub_execute_input: Option<String>,
    #[serde(rename = "iopub.status.busy", skip_serializing_if = "Option::is_none")]
    pub iopub_status_busy: Option<String>,
    #[serde(
        rename = "shell.execute_reply",
        skip_serializing_if = "Option::is_none"
    )]
    pub shell_execute_reply: Option<String>,
    #[serde(
        rename = "shell.execute_reply.started",
        skip_serializing_if = "Option::is_none"
    )]
    pub shell_execute_reply_started: Option<String>,
    #[serde(rename = "iopub.status.idle", skip_serializing_if = "Option::is_none")]
    pub iopub_status_idle: Option<String>,
    // For retaining any additional fields introduced by other jupyter clients
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "output_type")]
pub enum Output {
    #[serde(rename = "stream")]
    Stream {
        name: String,
        #[serde(deserialize_with = "deserialize_multiline_string")]
        text: MultilineString,
    },
    // todo!(): transient does not belong _in_ nbformat, though it is on the raw
    // jupyter protocol. We'll need to accept a subset here.
    #[serde(rename = "display_data")]
    DisplayData(DisplayData),
    // todo!() Same goes for handling execute result
    #[serde(rename = "execute_result")]
    ExecuteResult(ExecuteResult),
    #[serde(rename = "error")]
    Error(ErrorOutput),
}

pub fn deserialize_outputs<'de, D>(deserializer: D) -> Result<Vec<Output>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let outputs: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    outputs
        .into_iter()
        .enumerate()
        .filter_map(
            |(index, output)| match serde_json::from_value::<Output>(output.clone()) {
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
            },
        )
        .collect()
}
