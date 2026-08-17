#[cfg(test)]
mod test {
    use nbformat::legacy::Cell as LegacyCell;
    use nbformat::v4::{Cell, CellId, Output};
    use nbformat::{Notebook, parse_notebook, serialize_notebook};
    use serde_json::Value;
    use std::fs;
    use std::path::Path;

    fn read_notebook(path: &str) -> String {
        fs::read_to_string(Path::new(path)).expect("Failed to read notebook file")
    }

    #[test]
    fn test_parse_legacy_v4_notebook() {
        let notebook_json = read_notebook("tests/notebooks/test4.ipynb");
        let notebook = parse_notebook(&notebook_json).expect("Failed to parse notebook");

        let notebook = if let Notebook::Legacy(notebook) = notebook {
            notebook
        } else {
            panic!("Expected v4.1 - v4.4 notebook");
        };

        assert_eq!(notebook.nbformat, 4);
        assert_eq!(notebook.nbformat_minor, 1);

        assert_eq!(notebook.cells.len(), 9);

        assert!(notebook.metadata.kernelspec.is_none());
        assert!(notebook.metadata.language_info.is_none());

        // Check first cell (markdown)
        let first_cell = &notebook.cells[0];
        if let LegacyCell::Markdown { source, .. } = first_cell {
            assert_eq!(source, &vec!["# nbconvert latex test"]);
        } else {
            panic!("First cell should be markdown");
        }

        // Check a code cell
        let code_cell = &notebook.cells[3];
        if let LegacyCell::Code {
            source,
            execution_count,
            outputs,
            ..
        } = code_cell
        {
            assert_eq!(source, &vec!["print(\"hello\")"]);
            assert_eq!(*execution_count, Some(1));
            assert_eq!(outputs.len(), 1);
            if let Output::Stream { name, text } = &outputs[0] {
                assert_eq!(name, "stdout");
                assert_eq!(text.0, "hello\n");
            } else {
                panic!("Expected stream output");
            }
        } else {
            panic!("Expected code cell");
        }
    }
    #[test]
    fn test_parse_v4_5_notebook() {
        let notebook_json = read_notebook("tests/notebooks/test4.5.ipynb");
        let notebook = parse_notebook(&notebook_json).expect("Failed to parse notebook");

        let notebook = if let Notebook::V4(notebook) = notebook {
            notebook
        } else {
            panic!("Expected v4.1 - v4.4 notebook");
        };

        assert_eq!(notebook.nbformat, 4);
        assert_eq!(notebook.nbformat_minor, 5);
        assert!(!notebook.cells.is_empty());

        // Check metadata
        assert!(notebook.metadata.kernelspec.is_some());
        let kernelspec = notebook.metadata.kernelspec.as_ref().unwrap();
        assert_eq!(kernelspec.name, "python3");

        assert!(notebook.metadata.language_info.is_some());
        let lang_info = notebook.metadata.language_info.as_ref().unwrap();
        assert_eq!(lang_info.name, "python");

        // Check a code cell
        let code_cell = notebook
            .cells
            .iter()
            .find(|cell| matches!(cell, Cell::Code { .. }))
            .unwrap();
        if let Cell::Code {
            id,
            metadata: _,
            execution_count,
            source,
            outputs,
        } = code_cell
        {
            assert_eq!(id.as_str(), "38f37a24");
            // assert!(metadata.id.is_some());
            assert!(execution_count.is_some());
            assert!(!source.is_empty());
            assert!(!outputs.is_empty());
        } else {
            panic!("Expected code cell");
        }

        // Check a markdown cell
        let markdown_cell = notebook
            .cells
            .iter()
            .find(|cell| matches!(cell, Cell::Markdown { .. }))
            .unwrap();
        if let Cell::Markdown {
            id,
            metadata: _,
            source,
            attachments,
        } = markdown_cell
        {
            assert_eq!(id.as_str(), "2fcdfa53");
            assert!(!source.is_empty());
            assert!(attachments.is_none() || attachments.as_ref().unwrap().is_object());
        } else {
            panic!("Expected markdown cell");
        }
    }

    #[test]
    fn test_open_all_notebooks_in_dir() {
        let dir = Path::new("tests/notebooks");
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            let path_str = path.to_str().expect("Failed to convert path to string");
            if path_str.ends_with(".ipynb") {
                // If the file starts with `test3`, let's check that we got an error
                let notebook_json = read_notebook(path_str);
                let notebook = parse_notebook(&notebook_json);

                println!("Parsing notebook: {}", path_str);
                if let Err(ref e) = notebook {
                    println!("Error for {}: {:?}", path_str, e);
                }
                if path_str.contains("invalid_cell_id") || path_str.contains("invalid_metadata") {
                    assert!(
                        matches!(notebook, Err(nbformat::NotebookError::JsonError(_))),
                        "Expected JsonError for invalid data in {}",
                        path_str
                    );
                } else if path_str.starts_with("tests/notebooks/test2")
                    || path_str.starts_with("tests/notebooks/test4plus")
                    || path_str.starts_with("tests/notebooks/invalid")
                    || path_str.starts_with("tests/notebooks/no_min_version")
                {
                    assert!(notebook.is_err(), "Expected error for {}", path_str);
                } else {
                    assert!(notebook.is_ok(), "Failed to parse notebook: {}", path_str);
                }
            }
        }
    }

    /// Compare notebook JSON at a key level so that mismatches bubble up as lines like `Serialization mismatch: Extra key 'attachments' in serialized at root.cells[0]`
    fn compare_notebook_json(original: &Value, serialized: &Value) -> Result<(), String> {
        fn compare_values(path: &str, v1: &Value, v2: &Value) -> Result<(), String> {
            match (v1, v2) {
                (Value::Object(o1), Value::Object(o2)) => {
                    for (k, v) in o1 {
                        if !o2.contains_key(k) {
                            return Err(format!("Key '{}' missing in serialized at {}", k, path));
                        }
                        compare_values(&format!("{}.{}", path, k), v, &o2[k])?;
                    }
                    for k in o2.keys() {
                        if !o1.contains_key(k) {
                            return Err(format!("Extra key '{}' in serialized at {}", k, path));
                        }
                    }
                }
                (Value::Array(a1), Value::Array(a2)) => {
                    if a1.len() != a2.len() {
                        return Err(format!("Array length mismatch at {}", path));
                    }
                    for (i, (v1, v2)) in a1.iter().zip(a2.iter()).enumerate() {
                        compare_values(&format!("{}[{}]", path, i), v1, v2)?;
                    }
                }
                (Value::String(s1), Value::String(s2)) => {
                    if s1.trim() != s2.trim() {
                        return Err(format!("String mismatch at {}: '{}' vs '{}'", path, s1, s2));
                    }
                }
                (v1, v2) => {
                    if v1 != v2 {
                        return Err(format!("Value mismatch at {}: {:?} vs {:?}", path, v1, v2));
                    }
                }
            }
            Ok(())
        }

        compare_values("root", original, serialized)
    }

    #[test]
    fn test_serialize_deserialize() {
        let notebook_json = read_notebook("tests/notebooks/test4.5.ipynb");
        let notebook = parse_notebook(&notebook_json).expect("Failed to parse notebook");

        let serialized = serialize_notebook(&notebook).expect("Failed to serialize notebook");

        let original_value: Value =
            serde_json::from_str(&notebook_json).expect("Failed to parse original JSON");
        let serialized_value: Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

        if let Err(diff) = compare_notebook_json(&original_value, &serialized_value) {
            panic!("Serialization mismatch: {}", diff);
        }

        println!("Structures match in contents!");

        println!("Original:\n\n{}", notebook_json);
        println!("Serialized:\n\n{}", serialized);

        // Now for the hardest part -- seeing if we can get exact text back
        assert_eq!(notebook_json, serialized);
    }

    #[test]
    fn test_serialize_deserialize_another() {
        let notebook_json = read_notebook("tests/notebooks/Mediatypes.ipynb");
        let notebook = parse_notebook(&notebook_json).expect("Failed to parse notebook");

        let serialized = serialize_notebook(&notebook).expect("Failed to serialize notebook");

        let original_value: Value =
            serde_json::from_str(&notebook_json).expect("Failed to parse original JSON");
        let serialized_value: Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

        if let Err(diff) = compare_notebook_json(&original_value, &serialized_value) {
            panic!("Serialization mismatch: {}", diff);
        }

        println!("Structures match in contents!");

        // std::fs::write("og.json", &notebook_json).expect("Failed to write original JSON");
        // std::fs::write("ser.json", &serialized).expect("Failed to write serialized JSON");

        assert_eq!(notebook_json, serialized);
    }

    #[test]
    fn test_unknown_media_types() {
        let notebook_json = r###"{
 "cells": [
  {
   "cell_type": "markdown",
   "id": "example-1",
   "metadata": {},
   "source": [
    "# nbconvert latex test"
   ]
  },
  {
   "cell_type": "markdown",
   "id": "example-2",
   "metadata": {},
   "source": [
    "**Lorem ipsum** dolor sit amet, consectetur adipiscing elit. Nunc luctus bibendum felis dictum sodales. Ut suscipit, orci ut interdum imperdiet, purus ligula mollis *justo*, non malesuada nisl augue eget lorem. Donec bibendum, erat sit amet porttitor aliquam, urna lorem ornare libero, in vehicula diam diam ut ante. Nam non urna rhoncus, accumsan elit sit amet, mollis tellus. Vestibulum nec tellus metus. Vestibulum tempor, ligula et vehicula rhoncus, sapien turpis faucibus lorem, id dapibus turpis mauris ac orci. Sed volutpat vestibulum venenatis."
   ]
  },
  {
   "cell_type": "markdown",
   "id": "example-3",
   "metadata": {},
   "source": [
    "## Printed Using Python"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 1,
   "id": "example-4",
   "metadata": {
    "collapsed": false
   },
   "outputs": [
    {
     "name": "stdout",
     "output_type": "stream",
     "text": [
      "hello\n"
     ]
    }
   ],
   "source": [
    "print(\"hello\")"
   ]
  },
  {
   "cell_type": "markdown",
   "id": "example-5",
   "metadata": {},
   "source": [
    "## Pyout"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 3,
   "id": "example-6",
   "metadata": {
    "collapsed": false
   },
   "outputs": [
    {
     "data": {
      "text/html": [
       "\n",
       "<script>\n",
       "console.log(\"hello\");\n",
       "</script>\n",
       "<b>HTML</b>\n"
      ],
      "text/plain": [
       "<IPython.core.display.HTML at 0x1112757d0>"
      ]
     },
     "execution_count": 3,
     "metadata": {},
     "output_type": "execute_result"
    }
   ],
   "source": [
    "from IPython.display import HTML\n",
    "\n",
    "HTML(\n",
    "    \"\"\"\n",
    "<script>\n",
    "console.log(\"hello\");\n",
    "</script>\n",
    "<b>HTML</b>\n",
    "\"\"\"\n",
    ")"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 7,
   "id": "example-7",
   "metadata": {
    "collapsed": false
   },
   "outputs": [
    {
     "data": {
      "application/javascript": [
       "console.log(\"hi\");"
      ],
      "text/hokey": [
       "fake output"
      ],
      "text/plain": [
       "<IPython.core.display.Javascript at 0x1112b4b50>"
      ]
     },
     "metadata": {},
     "output_type": "display_data"
    }
   ],
   "source": [
    "%%javascript\n",
    "console.log(\"hi\");"
   ]
  },
  {
   "cell_type": "markdown",
   "id": "example-8",
   "metadata": {},
   "source": [
    "# Image"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 6,
   "id": "example-9",
   "metadata": {
    "collapsed": false
   },
   "outputs": [
    {
     "data": {
      "image/png": [
       "iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAYAAACNMs+9AAAAFUlEQVR42mNk+M9Qz0AEYBxVSF+F\n",
       "AAhKDveksOjmAAAAAElFTkSuQmCC\n"
      ],
      "text/plain": [
       "<IPython.core.display.Image at 0x111275490>"
      ]
     },
     "execution_count": 6,
     "metadata": {},
     "output_type": "execute_result"
    }
   ],
   "source": [
    "from IPython.display import Image\n",
    "\n",
    "Image(\"fake.png\")"
   ]
  }
 ],
 "metadata": {},
 "nbformat": 4,
 "nbformat_minor": 5
}
"###;

        let notebook = parse_notebook(notebook_json).expect("Failed to parse notebook");

        match &notebook {
            Notebook::V4(notebook) => {
                if let Cell::Code { id, outputs, .. } = &notebook.cells[8] {
                    assert_eq!(id, &CellId::new("example-9").unwrap());
                    let output = outputs[0].clone();
                    match output {
                        Output::Stream { .. } => panic!("Expected image output"),
                        Output::DisplayData(..) => panic!("Expected image result"),
                        Output::ExecuteResult(execute_result) => {
                            let content = execute_result.data.content;

                            for media in content {
                                match media {
                                    jupyter_protocol::media::MediaType::Png(data) => {
                                        assert_eq!(
                                            data,
                                            "iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAYAAACNMs+9AAAAFUlEQVR42mNk+M9Qz0AEYBxVSF+F\nAAhKDveksOjmAAAAAElFTkSuQmCC\n"
                                        );
                                    }
                                    jupyter_protocol::media::MediaType::Plain(data) => {
                                        assert_eq!(
                                            data,
                                            "<IPython.core.display.Image at 0x111275490>"
                                        );
                                    }
                                    jupyter_protocol::media::MediaType::Other((
                                        mimetype,
                                        value,
                                    )) => {
                                        panic!(
                                            "Unexpected othering of media type: {} {:?}",
                                            mimetype, value
                                        );
                                    }
                                    _ => {
                                        dbg!(&media);

                                        panic!("Unexpected mime type")
                                    }
                                }
                            }
                        }
                        Output::Error(..) => panic!("Expected image result"),
                    }
                } else {
                    panic!("Expected code cell");
                }
            }
            Notebook::Legacy(_) => panic!("Expected V4 notebook, got legacy"),
            Notebook::V3(_) => panic!("Expected V4 notebook, got v3"),
        }

        let serialized = serialize_notebook(&notebook).expect("Failed to serialize notebook");

        let original_value: Value =
            serde_json::from_str(notebook_json).expect("Failed to parse original JSON");
        let serialized_value: Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

        if let Err(diff) = compare_notebook_json(&original_value, &serialized_value) {
            panic!("Serialization mismatch: {}", diff);
        }

        println!("Structures match in contents!");

        assert_eq!(notebook_json, serialized);
    }

    #[test]
    fn test_pandas_notebook_roundtrip() {
        let notebook_json = read_notebook("tests/notebooks/pandas_basic.ipynb");
        let notebook = parse_notebook(&notebook_json).expect("Failed to parse pandas notebook");

        // Verify structure
        let nb = if let Notebook::V4(ref nb) = notebook {
            nb
        } else {
            panic!("Expected V4 notebook");
        };

        assert_eq!(nb.nbformat, 4);
        assert_eq!(nb.nbformat_minor, 5);
        assert_eq!(nb.cells.len(), 4);

        // Cell 0: import pandas as pd
        assert!(matches!(&nb.cells[0], Cell::Code { source, outputs, execution_count: Some(1), .. } if source == &vec!["import pandas as pd"] && outputs.is_empty()));

        // Cell 1: create DataFrame and display
        if let Cell::Code { source, outputs, execution_count: Some(2), .. } = &nb.cells[1] {
            assert_eq!(source.len(), 6); // 6 lines of source
            assert_eq!(outputs.len(), 1);
            assert!(matches!(&outputs[0], Output::ExecuteResult(_)));
        } else {
            panic!("Expected code cell with execution_count 2");
        }

        // Cell 2: df
        if let Cell::Code { source, outputs, execution_count: Some(3), .. } = &nb.cells[2] {
            assert_eq!(source, &vec!["df"]);
            assert_eq!(outputs.len(), 1);
            assert!(matches!(&outputs[0], Output::ExecuteResult(_)));
        } else {
            panic!("Expected code cell with execution_count 3");
        }

        // Cell 3: df.describe()
        if let Cell::Code { source, outputs, execution_count: Some(4), .. } = &nb.cells[3] {
            assert_eq!(source, &vec!["df.describe()"]);
            assert_eq!(outputs.len(), 1);
            assert!(matches!(&outputs[0], Output::ExecuteResult(_)));
        } else {
            panic!("Expected code cell with execution_count 4");
        }

        // First roundtrip: serialize and compare byte-for-byte
        let serialized = serialize_notebook(&notebook).expect("Failed to serialize notebook");
        assert_eq!(
            notebook_json, serialized,
            "First roundtrip: serialized output does not match original"
        );

        // Second roundtrip: parse the serialized output, serialize again
        let notebook2 = parse_notebook(&serialized).expect("Failed to parse serialized notebook");
        let serialized2 =
            serialize_notebook(&notebook2).expect("Failed to serialize notebook a second time");
        assert_eq!(
            notebook_json, serialized2,
            "Second roundtrip: re-serialized output does not match original"
        );
    }

    #[test]
    fn test_parse_notebook_with_string_source() {
        let notebook_json = r###"{
 "cells": [
  {
   "metadata": {},
   "cell_type": "markdown",
   "source": "# Notebook test",
   "id": "4fa80f351e5e4f77"
  },
  {
   "metadata": {},
   "cell_type": "code",
   "outputs": [],
   "execution_count": null,
   "source": "print(\"Cell 1\")",
   "id": "93b25f370baef7fa"
  },
  {
   "metadata": {},
   "cell_type": "code",
   "outputs": [],
   "execution_count": null,
   "source": "print(\"Cell 2\")",
   "id": "b232b4b6e4fbed68"
  }
 ],
 "metadata": {
  "kernelspec": {
   "name": "python3",
   "language": "python",
   "display_name": "Python 3 (ipykernel)"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 5
}"###;

        let notebook =
            parse_notebook(notebook_json).expect("Failed to parse notebook with string source");

        match &notebook {
            Notebook::V4(notebook) => {
                assert_eq!(notebook.cells.len(), 3);

                if let Cell::Markdown { source, .. } = &notebook.cells[0] {
                    assert_eq!(source, &vec!["# Notebook test".to_string()]);
                } else {
                    panic!("Expected markdown cell");
                }

                if let Cell::Code {
                    source,
                    execution_count,
                    outputs,
                    ..
                } = &notebook.cells[1]
                {
                    assert_eq!(source, &vec!["print(\"Cell 1\")".to_string()]);
                    assert_eq!(*execution_count, None);
                    assert!(outputs.is_empty());
                } else {
                    panic!("Expected code cell");
                }

                if let Cell::Code { source, .. } = &notebook.cells[2] {
                    assert_eq!(source, &vec!["print(\"Cell 2\")".to_string()]);
                } else {
                    panic!("Expected code cell");
                }
            }
            Notebook::Legacy(_) => panic!("Expected V4 notebook, got legacy"),
            Notebook::V3(_) => panic!("Expected V4 notebook, got v3"),
        }
    }

    // V3 upconversion tests <-> mirrors Python nbformat's own test suite.

    fn parse_v3_and_upgrade(path: &str) -> nbformat::v4::Notebook {
        let json = read_notebook(path);
        let notebook = parse_notebook(&json)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {:?}", path, e));
        match notebook {
            Notebook::V3(v3) => nbformat::upgrade_v3_notebook(v3)
                .unwrap_or_else(|e| panic!("Failed to upgrade {}: {:?}", path, e)),
            other => panic!("Expected V3 notebook from {}, got {:?}", path, other),
        }
    }

    fn has_media_type(
        media: &jupyter_protocol::media::Media,
        pred: fn(&jupyter_protocol::media::MediaType) -> bool,
    ) -> bool {
        media.content.iter().any(pred)
    }

    /// Checks that every output type and
    /// every media key from _mime_map survives the v3->v4 upgrade.
    #[test]
    fn test_upgrade_v3_notebook() {
        let v4 = parse_v3_and_upgrade("tests/notebooks/test3_alloutputs.ipynb");

        // nb0 has 2 worksheets (6 cells + 0 cells) -> must be flattened
        assert_eq!(v4.cells.len(), 6);
        assert_eq!(v4.nbformat, 4);
        assert_eq!(v4.nbformat_minor, 5);

        // cell[3] heading(h2) -> markdown
        if let Cell::Markdown { source, .. } = &v4.cells[3] {
            assert_eq!(source.as_slice(), ["## My Heading"]);
        } else {
            panic!("Expected markdown from h2 heading, got {:?}", v4.cells[3]);
        }

        // cell[5] is the all-outputs code cell: pyout, display_data, pyerr, stream x2
        if let Cell::Code { outputs, execution_count, .. } = &v4.cells[5] {
            assert_eq!(execution_count, &Some(3));
            assert_eq!(outputs.len(), 5);

            // pyout -> ExecuteResult with all _mime_map keys
            let result = if let Output::ExecuteResult(r) = &outputs[0] { r }
                else { panic!("Expected ExecuteResult, got {:?}", outputs[0]) };
            assert_eq!(result.execution_count.value(), 3);
            for (check, label) in [
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Plain(_))),      "text->Plain"),
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Html(_))),       "html->Html"),
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Svg(_))),        "svg->Svg"),
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Png(_))),        "png->Png"),
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Jpeg(_))),       "jpeg->Jpeg"),
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Latex(_))),      "latex->Latex"),
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Javascript(_))), "javascript->Javascript"),
                (has_media_type(&result.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Json(_))),       "json->Json"),
            ] {
                assert!(check, "pyout {label} missing in ExecuteResult");
            }

            let json_val = result.data.content.iter().find_map(|mt| {
                if let jupyter_protocol::media::MediaType::Json(v) = mt { Some(v) } else { None }
            });
            assert!(
                json_val.map(|v| v.is_object()).unwrap_or(false),
                "pyout json field should be parsed into a JSON object, got {:?}", json_val
            );

            // display_data with same flat media keys
            let dd = if let Output::DisplayData(d) = &outputs[1] { d }
                else { panic!("Expected DisplayData, got {:?}", outputs[1]) };
            for (check, label) in [
                (has_media_type(&dd.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Plain(_))),      "text->Plain"),
                (has_media_type(&dd.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Html(_))),       "html->Html"),
                (has_media_type(&dd.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Png(_))),        "png->Png"),
                (has_media_type(&dd.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Javascript(_))), "javascript->Javascript"),
                (has_media_type(&dd.data, |mt| matches!(mt, jupyter_protocol::media::MediaType::Json(_))),       "json->Json"),
            ] {
                assert!(check, "display_data {label} missing");
            }

            // pyerr -> Error
            if let Output::Error(err) = &outputs[2] {
                assert_eq!(err.ename, "NameError");
                assert_eq!(err.evalue, "NameError was here");
                assert_eq!(err.traceback, vec!["frame 0", "frame 1", "frame 2"]);
            } else {
                panic!("Expected Error, got {:?}", outputs[2]);
            }

            // stream: stdout then stderr (Python: name = output.pop("stream", "stdout"))
            if let Output::Stream { name, text } = &outputs[3] {
                assert_eq!(name, "stdout");
                assert_eq!(text.0, "foo\rbar\r\n");
            } else {
                panic!("Expected stdout stream, got {:?}", outputs[3]);
            }
            if let Output::Stream { name, .. } = &outputs[4] {
                assert_eq!(name, "stderr");
            } else {
                panic!("Expected stderr stream, got {:?}", outputs[4]);
            }
        } else {
            panic!("Expected code cell at index 5");
        }
    }

    #[test]
    fn test_upgrade_v3_heading() {
        // test3.ipynb layout: heading(h1), markdown, heading(h2), code,
        //                     heading(h2), code, code, heading(h3), code
        let v4 = parse_v3_and_upgrade("tests/notebooks/test3.ipynb");

        if let Cell::Markdown { source, .. } = &v4.cells[0] {
            assert_eq!(source.as_slice(), ["# nbconvert latex test"]);
        } else {
            panic!("Expected h1 markdown, got {:?}", v4.cells[0]);
        }
        if let Cell::Markdown { source, .. } = &v4.cells[2] {
            assert_eq!(source.as_slice(), ["## Printed Using Python"]);
        } else {
            panic!("Expected h2 markdown, got {:?}", v4.cells[2]);
        }
        if let Cell::Markdown { source, .. } = &v4.cells[7] {
            assert_eq!(source.as_slice(), ["### Image"]);
        } else {
            panic!("Expected h3 markdown, got {:?}", v4.cells[7]);
        }
    }

    /// no-worksheets, missing prompt_number, missing metadata.
    #[test]
    fn test_upgrade_v3_edge_cases() {
        // no worksheets key -> empty cells
        let v4 = parse_v3_and_upgrade("tests/notebooks/test3_no_worksheets.ipynb");
        assert!(v4.cells.is_empty());

        // worksheet present but no cells -> empty cells
        let v4 = parse_v3_and_upgrade("tests/notebooks/test3_worksheet_with_no_cells.ipynb");
        assert!(v4.cells.is_empty());

        // no metadata -> no kernelspec, no language_info
        let v4 = parse_v3_and_upgrade("tests/notebooks/test3_no_metadata.ipynb");
        assert!(v4.metadata.kernelspec.is_none());
        assert!(v4.metadata.language_info.is_none());

        // missing prompt_number -> cell execution_count is None
        let json = r#"{"nbformat":3,"nbformat_minor":0,"metadata":{},
            "worksheets":[{"cells":[{"cell_type":"code","metadata":{},
            "input":["x = 1"],"language":"python","outputs":[]}]}]}"#;
        let nb = parse_notebook(json).expect("parse failed");
        let v3 = if let Notebook::V3(v3) = nb { v3 } else { panic!() };
        let v4 = nbformat::upgrade_v3_notebook(v3).expect("upgrade failed");
        if let Cell::Code { execution_count, .. } = &v4.cells[0] {
            assert_eq!(*execution_count, None);
        } else {
            panic!("Expected code cell");
        }
    }



    #[test]
    fn test_parse_notebook_with_mixed_source_formats() {
        let notebook_json = r###"{
 "cells": [
  {
   "cell_type": "markdown",
   "id": "cell-array",
   "metadata": {},
   "source": [
    "# Array format\n",
    "This is the array format."
   ]
  },
  {
   "cell_type": "code",
   "id": "cell-string",
   "metadata": {},
   "execution_count": null,
   "outputs": [],
   "source": "# String format\nprint('hello')"
  }
 ],
 "metadata": {
  "kernelspec": {
   "name": "python3",
   "language": "python",
   "display_name": "Python 3"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 5
}"###;

        let notebook =
            parse_notebook(notebook_json).expect("Failed to parse mixed format notebook");

        match &notebook {
            Notebook::V4(notebook) => {
                assert_eq!(notebook.cells.len(), 2);

                if let Cell::Markdown { source, .. } = &notebook.cells[0] {
                    assert_eq!(
                        source,
                        &vec![
                            "# Array format\n".to_string(),
                            "This is the array format.".to_string()
                        ]
                    );
                } else {
                    panic!("Expected markdown cell");
                }

                if let Cell::Code { source, .. } = &notebook.cells[1] {
                    assert_eq!(source, &vec!["# String format\nprint('hello')".to_string()]);
                } else {
                    panic!("Expected code cell");
                }
            }
            Notebook::Legacy(_) => panic!("Expected V4 notebook, got legacy"),
            Notebook::V3(_) => panic!("Expected V4 notebook, got v3"),
        }
    }

    #[test]
    fn test_stream_output_roundtrip() {
        // Test round-tripping stream output through serialize/deserialize,
        // which exercises MultilineString with the proper custom deserializer.
        let cases = vec![
            ("trailing newline", "hello\n"),
            ("no trailing newline", "hello"),
            ("multi-line with trailing", "line1\nline2\n"),
            ("multi-line no trailing", "line1\nline2"),
            ("empty string", ""),
            ("single newline", "\n"),
            ("multiple trailing newlines", "hello\n\n"),
        ];

        for (label, input) in cases {
            let output = Output::Stream {
                name: "stdout".to_string(),
                text: nbformat::v4::MultilineString(input.to_string()),
            };
            let serialized = serde_json::to_string(&output)
                .unwrap_or_else(|e| panic!("{label}: serialize failed: {e}"));
            let deserialized: Output = serde_json::from_str(&serialized)
                .unwrap_or_else(|e| panic!("{label}: deserialize failed: {e}"));
            if let Output::Stream { text, .. } = deserialized {
                assert_eq!(
                    text.0, input,
                    "{label}: roundtrip mismatch — input={input:?}, serialized={serialized}, got={:?}",
                    text.0
                );
            } else {
                panic!("{label}: expected Stream output after roundtrip");
            }
        }
    }

    #[test]
    fn test_multiline_string_preserves_lines() {
        use nbformat::v4::MultilineString;

        // "hello\n" should serialize as ["hello\n"], not ["hello\n\n"]
        let ms = MultilineString("hello\n".to_string());
        let serialized: Vec<String> = serde_json::from_str(
            &serde_json::to_string(&ms).unwrap(),
        )
        .unwrap();
        assert_eq!(serialized, vec!["hello\n"]);

        // "hello" (no trailing newline) should serialize as ["hello"]
        let ms = MultilineString("hello".to_string());
        let serialized: Vec<String> = serde_json::from_str(
            &serde_json::to_string(&ms).unwrap(),
        )
        .unwrap();
        assert_eq!(serialized, vec!["hello"]);

        // Multi-line: "a\nb\n" should serialize as ["a\n", "b\n"]
        let ms = MultilineString("a\nb\n".to_string());
        let serialized: Vec<String> = serde_json::from_str(
            &serde_json::to_string(&ms).unwrap(),
        )
        .unwrap();
        assert_eq!(serialized, vec!["a\n", "b\n"]);

        // Multi-line without trailing: "a\nb" should serialize as ["a\n", "b"]
        let ms = MultilineString("a\nb".to_string());
        let serialized: Vec<String> = serde_json::from_str(
            &serde_json::to_string(&ms).unwrap(),
        )
        .unwrap();
        assert_eq!(serialized, vec!["a\n", "b"]);
    }
}
