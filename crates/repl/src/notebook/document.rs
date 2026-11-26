use std::ops::Range;

use anyhow::{Context as _, Result, ensure};
use collections::HashSet;
use nbformat::v4::CellId;
use serde::{Deserialize, Deserializer};
use serde_json::{Value, value::RawValue};

pub(super) struct ParsedNotebook {
    pub notebook: nbformat::v4::Notebook,
    pub fields: Vec<CellFields<usize>>,
}

pub(super) struct CellFields<T> {
    pub source: Range<T>,
    pub multiline: bool,
    pub execution: Option<ExecutionFields<T>>,
    pub(super) last_source: String,
}

pub(super) struct ExecutionFields<T> {
    pub outputs: Range<T>,
    pub count: Range<T>,
    outputs_first: bool,
}

#[derive(Deserialize)]
struct RawNotebook<'a, Cells> {
    nbformat: u32,
    nbformat_minor: u32,
    #[serde(borrow)]
    metadata: &'a RawValue,
    cells: Cells,
}

#[derive(Deserialize)]
struct RawCell<'a> {
    id: Option<CellId>,
    cell_type: CellKind,
    metadata: &'a RawValue,
    source: &'a RawValue,
    #[serde(borrow)]
    outputs: Option<&'a RawValue>,
    #[serde(default, borrow, deserialize_with = "present_value")]
    execution_count: Option<&'a RawValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum CellKind {
    Code,
    Markdown,
    Raw,
}

fn present_value<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<&'de RawValue>, D::Error> {
    <&RawValue>::deserialize(deserializer).map(Some)
}

impl<'a, Cells: Deserialize<'a>> RawNotebook<'a, Cells> {
    fn parse(text: &'a str) -> Result<Self> {
        let notebook: Self = serde_json::from_str(text).map_err(|error| {
            anyhow::anyhow!(
                "Invalid notebook at line {}, column {}",
                error.line(),
                error.column()
            )
        })?;
        ensure!(
            notebook.nbformat == 4 && notebook.nbformat_minor <= 5,
            "Unsupported notebook version {}.{}",
            notebook.nbformat,
            notebook.nbformat_minor
        );
        ensure!(
            notebook.metadata.get().starts_with('{'),
            "Invalid notebook metadata"
        );
        Ok(notebook)
    }
}

impl RawNotebook<'_, Vec<RawCell<'_>>> {
    fn validate_cells(&self) -> Result<()> {
        let mut ids = HashSet::default();
        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(id) = &cell.id {
                ensure!(
                    ids.insert(id),
                    "Duplicate notebook cell ID at cell {}",
                    index + 1
                );
            } else {
                ensure!(self.nbformat_minor < 5, "Missing notebook cell ID");
            }
            ensure!(
                cell.metadata.get().starts_with('{'),
                "Invalid notebook cell metadata"
            );
            ensure!(
                cell.source.get().starts_with(['"', '[']),
                "Invalid notebook cell source"
            );
            if matches!(cell.cell_type, CellKind::Code) {
                let (outputs, count) = cell.execution_fields()?;
                ensure!(
                    outputs.get().starts_with('['),
                    "Invalid notebook cell outputs"
                );
                // nbformat's lenient cell decoder prints any output it fails to decode.
                // Validate with the same value decoder before giving it the notebook.
                let outputs: Vec<Value> = serde_json::from_str(outputs.get())
                    .map_err(|_| anyhow::anyhow!("Invalid outputs in cell {}", index + 1))?;
                for (output_index, output) in outputs.into_iter().enumerate() {
                    serde_json::from_value::<nbformat::v4::Output>(output).map_err(|_| {
                        anyhow::anyhow!(
                            "Unsupported output {} in cell {}",
                            output_index + 1,
                            index + 1
                        )
                    })?;
                }
                let count: Option<i32> = serde_json::from_str(count.get()).map_err(|_| {
                    anyhow::anyhow!("Invalid execution count in cell {}", index + 1)
                })?;
                ensure!(
                    count.is_none_or(|count| count >= 0),
                    "Invalid notebook execution count"
                );
            }
        }
        Ok(())
    }
}

impl<'a> RawCell<'a> {
    fn execution_fields(&self) -> Result<(&'a RawValue, &'a RawValue)> {
        ensure!(
            matches!(self.cell_type, CellKind::Code),
            "Only code cells can run"
        );
        Ok((
            self.outputs.context("Code cell is missing outputs")?,
            self.execution_count
                .context("Code cell is missing an execution count")?,
        ))
    }
}

fn raw_range(text: &str, value: &RawValue) -> Result<Range<usize>> {
    text.substr_range(value.get())
        .map(Into::into)
        .context("JSON value is outside the notebook")
}

#[derive(Clone, Copy)]
pub(super) enum CellChange {
    InsertCode,
    InsertMarkdown,
    Delete,
    MoveUp,
    MoveDown,
}

pub(super) enum CellEdit {
    Add {
        index: usize,
        markdown: bool,
    },
    Remove(usize),
    Restore {
        index: usize,
        raw: String,
        offset: usize,
    },
    Swap(usize),
}

pub(super) enum Projection {
    Remove(usize),
    Insert {
        index: usize,
        cell: nbformat::v4::Cell,
        fields: CellFields<usize>,
    },
    Swap {
        index: usize,
        left: Range<usize>,
        right: Range<usize>,
    },
}

pub(super) fn plan_cell_edit(
    text: &str,
    edit: &CellEdit,
) -> Result<(Vec<(Range<usize>, String)>, CellEdit, Projection)> {
    let notebook = RawNotebook::<&RawValue>::parse(text)?;
    let cells: Vec<&RawValue> = serde_json::from_str(notebook.cells.get())?;
    let array = raw_range(text, notebook.cells)?;
    let start = array.start + 1;
    let cell =
        |index: usize| raw_range(text, cells.get(index).context("No cell at this position")?);
    let insertion = |index: usize, offset: usize| -> Result<usize> {
        ensure!(index <= cells.len(), "No insertion slot at this position");
        if index < cells.len() {
            Ok(cell(index)?.start)
        } else if index > 0 {
            Ok(cell(index - 1)?.end)
        } else {
            ensure!(
                offset <= array.len() - 2,
                "Invalid position in empty cell array"
            );
            Ok(start + offset)
        }
    };
    let insert = |index, position, raw: String| -> Result<_> {
        let object = raw.trim_matches([',', ' ', '\t', '\r', '\n']);
        let start = position
            + raw
                .substr_range(object)
                .context("Missing cell object")?
                .start;
        let prefix = r#"{"cells":["#;
        let mut parsed = parse_view(&format!(
            r#"{prefix}{object}],"metadata":{{}},"nbformat":4,"nbformat_minor":{}}}"#,
            notebook.nbformat_minor
        ))?;
        let cell = parsed
            .notebook
            .cells
            .pop()
            .context("Missing inserted cell")?;
        let fields = parsed
            .fields
            .pop()
            .context("Missing inserted fields")?
            .map_offsets(|range| {
                start + (range.start - prefix.len())..start + (range.end - prefix.len())
            });
        Ok((
            vec![(position..position, raw)],
            CellEdit::Remove(index),
            Projection::Insert {
                index,
                cell,
                fields,
            },
        ))
    };

    Ok(match *edit {
        CellEdit::Add { index, markdown } => {
            let position = insertion(index, 0)?;
            let mut new_cell = if markdown {
                serde_json::json!({"cell_type":"markdown", "metadata":{}, "source":[]})
            } else {
                serde_json::json!({"cell_type":"code", "metadata":{}, "source":[], "outputs":[], "execution_count":null})
            };
            if notebook.nbformat_minor == 5 {
                new_cell["id"] = uuid::Uuid::new_v4().to_string().into();
            }
            let raw = if cells.is_empty() {
                new_cell.to_string()
            } else {
                let separator = if cells.len() == 1 {
                    &format!(",{}", &text[start..cell(0)?.start])
                } else {
                    let left = index.saturating_sub(1).min(cells.len() - 2);
                    &text[cell(left)?.end..cell(left + 1)?.start]
                };
                if index < cells.len() {
                    format!("{new_cell}{separator}")
                } else {
                    format!("{separator}{new_cell}")
                }
            };
            return insert(index, position, raw);
        }
        CellEdit::Remove(index) => {
            let mut range = cell(index)?;
            if index + 1 < cells.len() {
                range.end = cell(index + 1)?.start;
            } else if index > 0 {
                range.start = cell(index - 1)?.end;
            }
            let restore = CellEdit::Restore {
                index,
                raw: text[range.clone()].to_owned(),
                offset: range.start - start,
            };
            (
                vec![(range, String::new())],
                restore,
                Projection::Remove(index),
            )
        }
        CellEdit::Restore {
            index,
            ref raw,
            offset,
        } => {
            let position = insertion(index, offset)?;
            return insert(index, position, raw.clone());
        }
        CellEdit::Swap(index) => {
            let first = cell(index)?;
            let second = cell(index + 1)?;
            let separator = &text[first.end..second.start];
            let edits = if first.len() <= second.len() {
                vec![
                    (first.start..second.start, String::new()),
                    (
                        second.end..second.end,
                        [separator, &text[first.clone()]].concat(),
                    ),
                ]
            } else {
                vec![
                    (
                        first.start..first.start,
                        [&text[second.clone()], separator].concat(),
                    ),
                    (first.end..second.end, String::new()),
                ]
            };
            (
                edits,
                CellEdit::Swap(index),
                Projection::Swap {
                    index,
                    left: first,
                    right: second,
                },
            )
        }
    })
}

pub(super) fn parse_view(text: &str) -> Result<ParsedNotebook> {
    let raw = RawNotebook::parse(text)?;
    raw.validate_cells()?;
    let notebook = super::notebook_ui::upgrade_notebook(
        nbformat::parse_notebook(text).map_err(|_| anyhow::anyhow!("Invalid notebook contents"))?,
    )?;
    let mut fields = Vec::with_capacity(raw.cells.len());
    for (raw, cell) in raw.cells.into_iter().zip(&notebook.cells) {
        let (source, execution) = match cell {
            nbformat::v4::Cell::Code { source, .. } => {
                let (saved_outputs, count) = raw.execution_fields()?;
                let outputs = raw_range(text, saved_outputs)?;
                let count = raw_range(text, count)?;
                (
                    source,
                    Some(ExecutionFields {
                        outputs_first: outputs.start < count.start,
                        outputs,
                        count,
                    }),
                )
            }
            nbformat::v4::Cell::Markdown { source, .. }
            | nbformat::v4::Cell::Raw { source, .. } => (source, None),
        };
        fields.push(CellFields {
            source: raw_range(text, raw.source)?,
            multiline: raw.source.get().starts_with('['),
            execution,
            last_source: source.concat(),
        });
    }
    Ok(ParsedNotebook { notebook, fields })
}

impl<T> CellFields<T> {
    pub(super) fn map_offsets<U>(self, mut map: impl FnMut(Range<T>) -> Range<U>) -> CellFields<U> {
        CellFields {
            source: map(self.source),
            multiline: self.multiline,
            execution: self.execution.map(|fields| ExecutionFields {
                outputs: map(fields.outputs),
                count: map(fields.count),
                outputs_first: fields.outputs_first,
            }),
            last_source: self.last_source,
        }
    }
}

impl<T: Clone> CellFields<T> {
    pub(super) fn edit_source(&mut self, source: &str) -> Result<Vec<(Range<T>, String)>> {
        if self.last_source == source {
            return Ok(Vec::new());
        }
        let replacement = if self.multiline {
            serde_json::to_string(&source.split_inclusive('\n').collect::<Vec<_>>())?
        } else {
            serde_json::to_string(source)?
        };
        source.clone_into(&mut self.last_source);
        Ok(vec![(self.source.clone(), replacement)])
    }

    pub(super) fn edit_execution(
        &self,
        outputs: &[Value],
        execution_count: Option<i64>,
    ) -> Result<Vec<(Range<T>, String)>> {
        let fields = self.execution.as_ref().context("Only code cells can run")?;
        ensure!(
            execution_count.is_none_or(|count| (0..=i64::from(i32::MAX)).contains(&count)),
            "Unsupported notebook execution count"
        );
        let outputs = (fields.outputs.clone(), serde_json::to_string(outputs)?);
        let count = (
            fields.count.clone(),
            serde_json::to_string(&execution_count)?,
        );
        Ok(if fields.outputs_first {
            vec![outputs, count]
        } else {
            vec![count, outputs]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const NOTEBOOK: &str = r#"{
  "nbformat":4, "nbformat_minor":4, "metadata": {"extra": 987654321012345678909876543210},
  "cells": [
    {"cell_type":"markdown","source":"same","metadata":{},"attachments":{"plot":{"image/png":"abc"}}},
    {"cell_type":"code","source":[ "sa", "\u006de" ],"metadata":{},"extra":1.230000e+07,
     "outputs" : [ {"output_type":"stream","name":"stdout","text":["old\n"]} ], "execution_count":9},
    {"cell_type":"code","metadata":{},"execution_count":9,"source":"same","outputs":[]}
  ]
}
"#;

    fn apply(text: &str, edits: Vec<(Range<usize>, String)>) -> String {
        let mut result = text.to_owned();
        for (range, replacement) in edits.into_iter().rev() {
            result.replace_range(range, &replacement);
        }
        result
    }

    fn change_cell(text: &str, edit: &CellEdit) -> Result<(String, CellEdit)> {
        let (edits, inverse, _) = plan_cell_edit(text, edit)?;
        Ok((apply(text, edits), inverse))
    }

    #[test]
    fn moving_and_deleting_cells_preserves_raw_objects_and_separators() -> Result<()> {
        use CellEdit::{Remove, Swap};
        let prefix = r#"{ "metadata":{"π":"[{}]", "cells":[]}, "nbformat_minor":4, "nbformat":4, "cells" : [  "#;
        let suffix = "\r\n ] }\n";
        let markdown =
            r#"{"source":"猫", "attachments":{"a":{}}, "metadata":{}, "cell_type":"markdown"}"#;
        let code = r#"{"cell_type":"code","execution_count":3,"outputs":[{"output_type":"stream","name":"stdout","text":["\u03c0\n"]}],"metadata":{"x":1.20e-7},"source":["print('π')\n"]}"#;
        let raw = r#"{ "cell_type":"raw", "metadata":{}, "source":"]}," }"#;
        let left = " \t,\r\n  ";
        let right = ",\n\t";
        let original = format!("{prefix}{markdown}{left}{code}{right}{raw}{suffix}");

        for (edit, expected) in [
            (Swap(0), format!("{code}{left}{markdown}{right}{raw}")),
            (Swap(1), format!("{markdown}{left}{raw}{right}{code}")),
            (Remove(0), format!("{code}{right}{raw}")),
            (Remove(1), format!("{markdown}{left}{raw}")),
            (Remove(2), format!("{markdown}{left}{code}")),
        ] {
            let (saved, inverse) = change_cell(&original, &edit)?;
            assert_eq!(saved, format!("{prefix}{expected}{suffix}"));
            assert_eq!(change_cell(&saved, &inverse)?.0, original);
        }

        let single = format!("{prefix}{markdown}{suffix}");
        let (empty, restore) = change_cell(&single, &Remove(0))?;
        assert_eq!(empty, format!("{prefix}{suffix}"));
        assert_eq!(change_cell(&empty, &restore)?.0, single);
        for (text, edit) in [
            (&empty, Remove(0)),
            (&single, Swap(0)),
            (&original, Swap(2)),
            (&original, Remove(3)),
            (
                &original,
                CellEdit::Add {
                    index: 4,
                    markdown: false,
                },
            ),
        ] {
            assert!(plan_cell_edit(text, &edit).is_err());
        }

        let (removed, restore) = change_cell(&original, &Remove(1))?;
        let edited = removed.replace("猫", "longer Markdown text");
        let (edits, _, projection) = plan_cell_edit(&edited, &restore)?;
        let restored = apply(&edited, edits);
        assert_eq!(restored, original.replace("猫", "longer Markdown text"));
        let Projection::Insert {
            index: 1,
            cell: nbformat::v4::Cell::Code { .. },
            fields,
        } = projection
        else {
            anyhow::bail!("expected restored code cell");
        };
        assert_eq!(&restored[fields.source], r#"["print('π')\n"]"#);
        let execution = fields.execution.context("missing execution fields")?;
        assert_eq!(&restored[execution.count], "3");
        assert_eq!(
            &restored[execution.outputs],
            r#"[{"output_type":"stream","name":"stdout","text":["\u03c0\n"]}]"#
        );
        Ok(())
    }

    #[test]
    fn swapping_a_large_output_moves_only_the_small_cell_and_separator() -> Result<()> {
        let small = r#"{"source":"猫","metadata":{},"cell_type":"markdown"}"#;
        let large = format!(
            r#"{{"cell_type":"code","metadata":{{}},"source":[],"execution_count":1,"outputs":[{{"output_type":"display_data","metadata":{{}},"data":{{"image/png":"{}"}}}}]}}"#,
            "x".repeat(1 << 20)
        );
        let separator = " \t,\n ";
        for (left, right) in [(small, large.as_str()), (large.as_str(), small)] {
            let prefix = r#"{"nbformat":4,"nbformat_minor":4,"metadata":{},"cells":["#;
            let original = format!("{prefix}{left}{separator}{right}]}}");
            let (edits, _, projection) = plan_cell_edit(&original, &CellEdit::Swap(0))?;
            let Projection::Swap {
                index: 0,
                left: first,
                right: second,
            } = projection
            else {
                anyhow::bail!("expected swapped cells");
            };
            assert_eq!(&original[first], left);
            assert_eq!(&original[second], right);
            let moved = small.len() + separator.len();
            assert_eq!(
                edits.iter().map(|(range, _)| range.len()).sum::<usize>(),
                moved
            );
            assert_eq!(edits.iter().map(|(_, raw)| raw.len()).sum::<usize>(), moved);
            assert_eq!(
                apply(&original, edits),
                format!("{prefix}{right}{separator}{left}]}}")
            );
        }
        Ok(())
    }

    #[test]
    fn inserted_cells_have_the_right_shape_and_leave_existing_bytes_intact() -> Result<()> {
        for index in 0..=3 {
            for (markdown, expected) in [
                (
                    false,
                    json!({"cell_type":"code", "source":[], "metadata":{}, "outputs":[], "execution_count":null}),
                ),
                (
                    true,
                    json!({"cell_type":"markdown", "source":[], "metadata":{}}),
                ),
            ] {
                let (saved, undo) = change_cell(NOTEBOOK, &CellEdit::Add { index, markdown })?;
                let cells: Value = serde_json::from_str(&saved)?;
                assert_eq!(cells["cells"][index], expected);
                assert_eq!(parse_view(&saved)?.notebook.cells.len(), 4);
                assert_eq!(change_cell(&saved, &undo)?.0, NOTEBOOK);
            }
        }
        let empty = r#"{ "cells" : [  ], "metadata":{}, "nbformat":4, "nbformat_minor":5 }"#;
        let (one, undo_first) = change_cell(
            empty,
            &CellEdit::Add {
                index: 0,
                markdown: true,
            },
        )?;
        let (two, undo_second) = change_cell(
            &one,
            &CellEdit::Add {
                index: 1,
                markdown: false,
            },
        )?;
        assert_eq!(parse_view(&two)?.notebook.cells.len(), 2);
        let value: Value = serde_json::from_str(&two)?;
        let markdown = &value["cells"][0];
        let code = &value["cells"][1];
        assert_eq!(markdown["cell_type"], "markdown");
        assert_eq!(code["cell_type"], "code");
        assert!(markdown["id"].is_string() && code["id"].is_string());
        assert_ne!(markdown["id"], code["id"]);
        let (saved, redo_second) = change_cell(&two, &undo_second)?;
        assert_eq!(saved, one);
        assert_eq!(change_cell(&one, &redo_second)?.0, two);
        assert_eq!(change_cell(&one, &undo_first)?.0, empty);
        Ok(())
    }

    #[test]
    fn source_edits_keep_saved_outputs_and_unrelated_bytes() -> Result<()> {
        let parsed = parse_view(NOTEBOOK)?;
        assert_eq!(parsed.notebook.cells.len(), 3);
        let mut fields = parsed.fields.into_iter();
        let mut markdown = fields.next().context("missing markdown cell")?;
        let mut code = fields.next().context("missing code cell")?;
        assert!(code.edit_source("same")?.is_empty());

        let source = "print('π')\r\n\nend";
        let saved = apply(NOTEBOOK, code.edit_source(source)?);
        assert!(code.edit_source(source)?.is_empty());
        assert_eq!(
            saved,
            NOTEBOOK.replace(r#"[ "sa", "\u006de" ]"#, r#"["print('π')\r\n","\n","end"]"#)
        );
        let reparsed = parse_view(&saved)?;
        assert!(
            matches!(reparsed.notebook.cells.get(1), Some(nbformat::v4::Cell::Code { source: lines, .. }) if lines.concat() == source)
        );
        assert_eq!(
            apply(NOTEBOOK, markdown.edit_source("new\n")?),
            NOTEBOOK.replacen(r#""source":"same""#, r#""source":"new\n""#, 1)
        );
        Ok(())
    }

    #[test]
    fn rerunning_replaces_only_the_executed_cells_results() -> Result<()> {
        let parsed = parse_view(NOTEBOOK)?;
        let mut fields = parsed.fields.into_iter();
        let markdown = fields.next().context("missing markdown cell")?;
        let first = fields.next().context("missing first code cell")?;
        let second = fields.next().context("missing second code cell")?;
        let outputs = [json!({"output_type":"stream","name":"stderr","text":"new\n"})];
        let saved = apply(NOTEBOOK, first.edit_execution(&outputs, Some(10))?);
        assert_eq!(
            saved,
            NOTEBOOK
                .replace(
                    r#"[ {"output_type":"stream","name":"stdout","text":["old\n"]} ]"#,
                    &serde_json::to_string(&outputs)?
                )
                .replacen(r#""execution_count":9"#, r#""execution_count":10"#, 1)
        );
        assert!(markdown.edit_execution(&outputs, None).is_err());
        assert!(fields.next().is_none());
        let second = second.map_offsets(|range| range.start + 1..range.end + 1);
        let saved = apply(
            &format!(" {NOTEBOOK}"),
            second.edit_execution(&outputs, None)?,
        );
        let parsed = parse_view(&saved)?;
        assert!(
            matches!(parsed.notebook.cells.last(), Some(nbformat::v4::Cell::Code { outputs, execution_count: None, .. }) if outputs.len() == 1)
        );
        Ok(())
    }

    #[test]
    fn unsupported_outputs_do_not_print_notebook_contents() -> Result<()> {
        const CHILD: &str = "ZED_TEST_NOTEBOOK_OUTPUT_DIAGNOSTICS";
        const MARKER: &str = "synthetic-notebook-output-1784";
        if std::env::var_os(CHILD).is_some() {
            for invalid_output in [
                json!({"output_type":MARKER, "text":MARKER}),
                json!({"output_type":"stream","name":"stdout","text":[{ "payload": MARKER }]}),
            ] {
                let mut notebook: Value = serde_json::from_str(NOTEBOOK)?;
                notebook["cells"][1]["outputs"] = json!([invalid_output]);
                let error = parse_view(&notebook.to_string())
                    .err()
                    .context("expected raw fallback")?;
                assert_eq!(error.to_string(), "Unsupported output 1 in cell 2");
            }
            return Ok(());
        }

        let output = smol::block_on(
            util::command::new_command(std::env::current_exe()?)
                .args([
                    "notebook::document::tests::unsupported_outputs_do_not_print_notebook_contents",
                    "--exact",
                    "--nocapture",
                ])
                .env(CHILD, "1")
                .output(),
        )?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success() && stdout.contains("1 passed"),
            "{stdout}\n{stderr}"
        );
        assert!(!stdout.contains(MARKER) && !stderr.contains(MARKER));
        Ok(())
    }

    #[test]
    fn unsupported_notebooks_are_not_editable() {
        for text in [
            NOTEBOOK.replace(r#""nbformat":4"#, r#""nbformat":3"#),
            NOTEBOOK.replace(r#""nbformat_minor":4"#, r#""nbformat_minor":6"#),
            NOTEBOOK.replace(r#""nbformat_minor":4"#, r#""nbformat_minor":5"#),
            NOTEBOOK.replace(
                r#""cell_type":"code""#,
                r#""cell_type":"code","id":"repeated""#,
            ),
            NOTEBOOK.replace(r#""cell_type":"markdown""#, r#""cell_type":"drawing""#),
            NOTEBOOK.replace(r#", "execution_count":9"#, ""),
        ] {
            assert!(parse_view(&text).is_err());
        }
        let unsupported_output =
            NOTEBOOK.replace(r#""output_type":"stream""#, r#""output_type":"unknown""#);
        assert!(parse_view(&unsupported_output).is_err());
    }
}
