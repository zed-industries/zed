use std::{
    collections::HashMap,
    ops::Range,
    path::{Path, PathBuf},
};
use tempdir::TempDir;

pub fn temp_tree(tree: serde_json::Value) -> TempDir {
    let dir = TempDir::new("").unwrap();
    write_tree(dir.path(), tree);
    dir
}

fn write_tree(path: &Path, tree: serde_json::Value) {
    use serde_json::Value;
    use std::fs;

    if let Value::Object(map) = tree {
        for (name, contents) in map {
            let mut path = PathBuf::from(path);
            path.push(name);
            match contents {
                Value::Object(_) => {
                    fs::create_dir(&path).unwrap();
                    write_tree(&path, contents);
                }
                Value::Null => {
                    fs::create_dir(&path).unwrap();
                }
                Value::String(contents) => {
                    fs::write(&path, contents).unwrap();
                }
                _ => {
                    panic!("JSON object must contain only objects, strings, or null");
                }
            }
        }
    } else {
        panic!("You must pass a JSON object to this helper")
    }
}

pub fn sample_text(rows: usize, cols: usize, start_char: char) -> String {
    let mut text = String::new();
    for row in 0..rows {
        let c: char = (start_char as u32 + row as u32) as u8 as char;
        let mut line = c.to_string().repeat(cols);
        if row < rows - 1 {
            line.push('\n');
        }
        text += &line;
    }
    text
}

pub fn marked_text_by(
    marked_text: &str,
    markers: Vec<char>,
) -> (String, HashMap<char, Vec<usize>>) {
    let mut extracted_markers: HashMap<char, Vec<usize>> = Default::default();
    let mut unmarked_text = String::new();

    for char in marked_text.chars() {
        if markers.contains(&char) {
            let char_offsets = extracted_markers.entry(char).or_insert(Vec::new());
            char_offsets.push(unmarked_text.len());
        } else {
            unmarked_text.push(char);
        }
    }

    (unmarked_text, extracted_markers)
}

pub fn marked_text(marked_text: &str) -> (String, Vec<usize>) {
    let (unmarked_text, mut markers) = marked_text_by(marked_text, vec!['|']);
    (unmarked_text, markers.remove(&'|').unwrap_or_else(Vec::new))
}

pub fn marked_text_ranges(
    marked_text: &str,
    range_markers: Vec<(char, char)>,
) -> (String, Vec<Range<usize>>) {
    let mut marker_chars = Vec::new();
    for (start, end) in range_markers.iter() {
        marker_chars.push(*start);
        marker_chars.push(*end);
    }
    let (unmarked_text, markers) = marked_text_by(marked_text, marker_chars);
    let ranges = range_markers
        .iter()
        .map(|(start_marker, end_marker)| {
            let start = markers.get(start_marker).unwrap()[0];
            let end = markers.get(end_marker).unwrap()[0];
            start..end
        })
        .collect();
    (unmarked_text, ranges)
}
