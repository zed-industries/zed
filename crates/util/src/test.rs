mod assertions;
mod marked_text;

use std::path::{Path, PathBuf};
use tempdir::TempDir;

pub use assertions::*;
pub use marked_text::*;

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
