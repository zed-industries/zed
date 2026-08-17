use crate::v4::{CellMetadata, deserialize_source};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "output_type")]
pub enum Output {
    #[serde(rename = "stream")]
    Stream {
        #[serde(default)]
        name: Option<String>,
        #[serde(rename = "stream", default)]
        stream: Option<String>,
        #[serde(default, deserialize_with = "deserialize_source")]
        text: Vec<String>,
    },
    #[serde(rename = "pyout")]
    PyOut {
        #[serde(default)]
        prompt_number: Option<i32>,
        #[serde(default)]
        metadata: Value,
        /// all remaining fields (the actual media payload: `text`, `html`, `png`, etc.)
        #[serde(flatten)]
        extra_fields: serde_json::Map<String, Value>,
    },
    /// display_data in v3 also stores media as flat top-level keys rather than under
    /// a nested "data" object.
    #[serde(rename = "display_data")]
    DisplayData {
        #[serde(default)]
        metadata: Value,
        #[serde(flatten)]
        extra_fields: serde_json::Map<String, Value>,
    },
    #[serde(rename = "pyerr")]
    PyErr {
        #[serde(default)]
        ename: Option<String>,
        #[serde(default)]
        evalue: Option<String>,
        #[serde(default)]
        traceback: Vec<String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Notebook {
    #[serde(default)]
    pub metadata: Option<Value>,
    pub nbformat: i32,
    #[serde(default)]
    pub nbformat_minor: Option<i32>,
    #[serde(default)]
    pub worksheets: Option<Vec<Worksheet>>,
}

#[derive(Deserialize, Debug)]
pub struct Worksheet {
    #[serde(default)]
    pub cells: Vec<Cell>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cell_type")]
pub enum Cell {
    #[serde(rename = "heading")]
    Heading {
        level: i32,
        metadata: CellMetadata,
        #[serde(default, deserialize_with = "deserialize_source")]
        source: Vec<String>,
    },
    #[serde(rename = "markdown")]
    Markdown {
        metadata: CellMetadata,
        #[serde(default, deserialize_with = "deserialize_source")]
        source: Vec<String>,
        #[serde(default)]
        attachments: Option<serde_json::Value>,
    },
    #[serde(rename = "code")]
    Code {
        metadata: CellMetadata,
        #[serde(default)]
        prompt_number: Option<i32>,
        #[serde(default, deserialize_with = "deserialize_optional_source")]
        input: Option<Vec<String>>,
        #[serde(default)]
        language: Option<String>,
        #[serde(default)]
        outputs: Vec<Output>,
    },
    #[serde(rename = "raw")]
    Raw {
        metadata: CellMetadata,
        #[serde(default, deserialize_with = "deserialize_source")]
        source: Vec<String>,
    },
}

/// Deserialize an optional field that can be either a string or an array of strings.
/// Returns `None` if the field is absent or null, `Some(Vec<String>)` otherwise.
pub fn deserialize_optional_source<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct OptionalSourceVisitor;

    impl<'de> serde::de::Visitor<'de> for OptionalSourceVisitor {
        type Value = Option<Vec<String>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string, an array of strings, or null")
        }

        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            Ok(Some(vec![v.to_string()]))
        }

        fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
            Ok(Some(vec![v]))
        }

        fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut lines = Vec::new();
            while let Some(line) = seq.next_element::<String>()? {
                lines.push(line);
            }
            Ok(Some(lines))
        }
    }

    deserializer.deserialize_any(OptionalSourceVisitor)
}

/// Normalise a single v3 media field value into a plain `String`.
///
/// In v3 notebooks, media values may be:
/// - A plain `String`
/// - An `Array` of strings (lines that should be concatenated)
///
/// Any other JSON shape is silently ignored (returns `None`).
pub fn join_media_value(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Array(arr) => Some(
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(""),
        ),
        _ => None,
    }
}
