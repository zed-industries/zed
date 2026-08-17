pub mod legacy;
pub mod v3;
pub mod v4;

use serde::Serialize as _;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotebookError {
    #[error("Unsupported notebook version: {0}.{1}")]
    UnsupportedVersion(i32, i32),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[derive(Debug)]
pub enum Notebook {
    V4(v4::Notebook),
    Legacy(legacy::Notebook),
    V3(v3::Notebook),
}

pub fn parse_notebook(json: &str) -> Result<Notebook, NotebookError> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let nbformat = value["nbformat"].as_i64().unwrap_or(0) as i32;
    let nbformat_minor = value["nbformat_minor"].as_i64().unwrap_or(0) as i32;

    match (nbformat, nbformat_minor) {
        (4, 5) => Ok(Notebook::V4(serde_json::from_value::<v4::Notebook>(value)?)),
        (4, 0) | (4, 1) | (4, 2) | (4, 3) | (4, 4) => Ok(Notebook::Legacy(
            serde_json::from_value::<legacy::Notebook>(value)?,
        )),
        (3, _) => Ok(Notebook::V3(serde_json::from_value::<v3::Notebook>(value)?)),
        _ => Err(NotebookError::UnsupportedVersion(nbformat, nbformat_minor)),
    }
}

pub fn serialize_notebook(notebook: &Notebook) -> Result<String, NotebookError> {
    match notebook {
        Notebook::V4(notebook) => {
            let value = serde_json::to_value(notebook)?;
            let mut buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b" ");
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            value.serialize(&mut ser)?;

            // Append a newline to the buffer to match the python implementation of nbformat
            buf.append(&mut b"\n".to_vec());

            let notebook_json = String::from_utf8(buf)
                .map_err(|e| NotebookError::ValidationError(e.to_string()))?;

            Ok(notebook_json)
        }
        Notebook::Legacy(notebook) => Err(NotebookError::UnsupportedVersion(
            notebook.nbformat,
            notebook.nbformat_minor,
        )),
        Notebook::V3(notebook) => Err(NotebookError::UnsupportedVersion(
            notebook.nbformat,
            notebook.nbformat_minor.unwrap_or(0),
        )),
    }
}

pub fn upgrade_legacy_notebook(legacy_notebook: legacy::Notebook) -> anyhow::Result<v4::Notebook> {
    let cells: Vec<v4::Cell> = legacy_notebook
        .cells
        .into_iter()
        .map(|cell: legacy::Cell| match cell {
            legacy::Cell::Markdown {
                id,
                metadata,
                source,
                attachments,
            } => v4::Cell::Markdown {
                id: id.unwrap_or_else(|| uuid::Uuid::new_v4().into()),
                metadata,
                source,
                attachments,
            },
            legacy::Cell::Code {
                id,
                metadata,
                execution_count,
                source,
                outputs,
            } => v4::Cell::Code {
                id: id.unwrap_or_else(|| uuid::Uuid::new_v4().into()),
                metadata,
                execution_count,
                source,
                outputs,
            },
            legacy::Cell::Raw {
                id,
                metadata,
                source,
            } => v4::Cell::Raw {
                id: id.unwrap_or_else(|| uuid::Uuid::new_v4().into()),
                metadata,
                source,
            },
        })
        .collect();

    // If any of the cell IDs are not unique, bail
    let mut seen_ids = std::collections::HashSet::new();
    for cell in &cells {
        if !seen_ids.insert(cell.id()) {
            return Err(anyhow::anyhow!("Duplicate Cell ID found: {}", cell.id()));
        }
    }

    Ok(v4::Notebook {
        cells,
        metadata: legacy_notebook.metadata,
        nbformat: 4,
        nbformat_minor: 5,
    })
}

pub fn upgrade_v3_notebook(v3_notebook: v3::Notebook) -> anyhow::Result<v4::Notebook> {
    let mut all_cells: Vec<v3::Cell> = Vec::new();

    if let Some(worksheets) = v3_notebook.worksheets {
        for worksheet in worksheets {
            all_cells.extend(worksheet.cells);
        }
    }

    let cells: Vec<v4::Cell> = all_cells
        .into_iter()
        .map(|cell: v3::Cell| match cell {
            v3::Cell::Heading {
                level,
                metadata,
                source,
            } => {
                let heading_prefix = "#".repeat(level as usize);
                // v3 heading source lines are plain text with no markdown prefix.
                // Join them into a single line and prepend the heading marker once.
                let joined = source.join("");
                let new_source = if joined.trim().is_empty() {
                    vec![format!("{}", heading_prefix)]
                } else {
                    vec![format!("{} {}", heading_prefix, joined)]
                };
                v4::Cell::Markdown {
                    id: uuid::Uuid::new_v4().into(),
                    metadata,
                    source: new_source,
                    attachments: None,
                }
            }
            v3::Cell::Markdown {
                metadata,
                source,
                attachments,
            } => v4::Cell::Markdown {
                id: uuid::Uuid::new_v4().into(),
                metadata,
                source,
                attachments,
            },
            v3::Cell::Code {
                metadata,
                prompt_number,
                input,
                language: _,
                outputs,
            } => v4::Cell::Code {
                id: uuid::Uuid::new_v4().into(),
                metadata,
                execution_count: prompt_number,
                source: input.unwrap_or_default(),
                outputs: outputs.into_iter().map(convert_v3_output).collect(),
            },
            v3::Cell::Raw { metadata, source } => v4::Cell::Raw {
                id: uuid::Uuid::new_v4().into(),
                metadata,
                source,
            },
        })
        .collect();

    // All v3 cells are assigned fresh UUIDs above, so duplicate IDs cannot occur.

    let metadata = convert_v3_metadata(v3_notebook.metadata.as_ref());

    Ok(v4::Notebook {
        cells,
        metadata,
        nbformat: 4,
        nbformat_minor: 5,
    })
}

fn convert_v3_metadata(v3_metadata: Option<&serde_json::Value>) -> v4::Metadata {
    let mut metadata = v4::Metadata::default();

    if let Some(v3_metadata) = v3_metadata {
        if let Some(obj) = v3_metadata.as_object() {
            // Extract language from language_info first so we can use it in kernelspec.
            let language = obj
                .get("language_info")
                .and_then(|li| li.get("name"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            if let Some(kernel_info) = obj.get("kernel_info") {
                if let Some(name) = kernel_info.get("name").and_then(|v| v.as_str()) {
                    metadata.kernelspec = Some(v4::KernelSpec {
                        display_name: name.to_string(),
                        name: name.to_string(),
                        // Use the actual language from language_info rather than
                        // assuming Python.
                        language: language.clone(),
                        additional: std::collections::HashMap::new(),
                    });
                }
            }

            if let Some(language_info) = obj.get("language_info") {
                if let Some(name) = language_info.get("name").and_then(|v| v.as_str()) {
                    let version = language_info
                        .get("version")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    metadata.language_info = Some(v4::LanguageInfo {
                        name: name.to_string(),
                        version,
                        codemirror_mode: None,
                        additional: std::collections::HashMap::new(),
                    });
                }
            }

            for (key, value) in obj {
                if key != "kernel_info" && key != "language_info" {
                    metadata.additional.insert(key.clone(), value.clone());
                }
            }
        }
    }

    metadata
}

fn map_v3_media_fields(
    fields: &serde_json::Map<String, serde_json::Value>,
    skip_keys: &[&str],
) -> Vec<jupyter_protocol::media::MediaType> {
    fields
        .iter()
        .filter(|(k, _)| !skip_keys.contains(&k.as_str()))
        .filter_map(|(k, v)| {
            let content = v3::join_media_value(v)?;
            let media_type = match k.as_str() {
                "text" => jupyter_protocol::media::MediaType::Plain(content),
                "html" => jupyter_protocol::media::MediaType::Html(content),
                "png" => jupyter_protocol::media::MediaType::Png(content),
                "jpeg" => jupyter_protocol::media::MediaType::Jpeg(content),
                "svg" => jupyter_protocol::media::MediaType::Svg(content),
                "latex" => jupyter_protocol::media::MediaType::Latex(content),
                "javascript" => jupyter_protocol::media::MediaType::Javascript(content),
                "json" => {
                    let parsed = serde_json::from_str(&content)
                        .unwrap_or(serde_json::Value::String(content));
                    return Some(jupyter_protocol::media::MediaType::Json(parsed));
                }
                _ => jupyter_protocol::media::MediaType::Other((
                    k.clone(),
                    serde_json::Value::String(content),
                )),
            };
            Some(media_type)
        })
        .collect()
}

fn convert_v3_output(v3_output: v3::Output) -> v4::Output {
    match v3_output {
        v3::Output::Stream { name, stream, text } => v4::Output::Stream {
            name: name.unwrap_or_else(|| stream.unwrap_or_else(|| "stdout".to_string())),
            text: v4::MultilineString(text.join("")),
        },
        v3::Output::PyOut {
            prompt_number,
            metadata,
            extra_fields,
        } => {
            let data = map_v3_media_fields(&extra_fields, &["output_type"]);

            let metadata = match metadata {
                serde_json::Value::Object(map) => map,
                _ => serde_json::Map::new(),
            };
            let execution_count = jupyter_protocol::ExecutionCount::new(
                prompt_number.unwrap_or(0).max(0) as usize,
            );
            v4::Output::ExecuteResult(v4::ExecuteResult {
                execution_count,
                data: jupyter_protocol::media::Media::new(data),
                metadata,
            })
        }
        v3::Output::DisplayData {
            metadata: _,
            extra_fields,
        } => {
            // v3 display_data also stores media as flat top-level keys. Skip the
            // structural fields that are not media.
            let media_vec =
                map_v3_media_fields(&extra_fields, &["output_type", "metadata"]);
            v4::Output::DisplayData(v4::DisplayData {
                data: jupyter_protocol::media::Media::new(media_vec),
                metadata: serde_json::Map::new(),
            })
        }
        v3::Output::PyErr {
            ename,
            evalue,
            traceback,
        } => v4::Output::Error(v4::ErrorOutput {
            ename: ename.unwrap_or_default(),
            evalue: evalue.unwrap_or_default(),
            traceback,
        }),
    }
}
