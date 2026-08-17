#![expect(missing_docs, reason = "fields mostly self-describing")]

use crate::codegen::Prefix;
use std::ops::Index;
use std::path::{Path, PathBuf};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Files {
    /// Arena of filenames from the input source.
    ///
    /// Indexed via `Pos::file`.
    pub file_names: Vec<String>,

    /// Arena of file source texts.
    ///
    /// Indexed via `Pos::file`.
    pub file_texts: Vec<String>,

    /// Arena of file line maps.
    ///
    /// Indexed via `Pos::file`.
    pub file_line_maps: Vec<LineMap>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct LineMap {
    /// Mapping from line number to starting byte position.
    line_ends: Vec<usize>,
}

impl Index<usize> for LineMap {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.line_ends[index]
    }
}

impl LineMap {
    pub fn from_str(text: &str) -> Self {
        let line_ends = text.match_indices('\n').map(|(i, _)| i + 1).collect();
        Self { line_ends }
    }

    /// Get the line on which `pos` occurs
    pub fn line(&self, pos: usize) -> usize {
        self.line_ends.partition_point(|&end| end <= pos)
    }

    /// Get the starting byte position of `line`.
    pub fn get(&self, line: usize) -> Option<&usize> {
        self.line_ends.get(line)
    }
}

impl Files {
    pub fn from_paths<P: AsRef<Path>>(
        paths: impl IntoIterator<Item = P>,
        prefixes: &[crate::codegen::Prefix],
    ) -> Result<Self, (PathBuf, std::io::Error)> {
        fn replace_prefixes(prefixes: &[Prefix], path: String) -> String {
            for Prefix { prefix, name } in prefixes {
                if path.starts_with(prefix) {
                    return path.replacen(prefix, name, 1);
                }
            }
            path
        }

        let mut file_names = Vec::new();
        let mut file_texts = Vec::new();
        let mut file_line_maps = Vec::new();

        for path in paths {
            let path = path.as_ref();
            let contents =
                std::fs::read_to_string(path).map_err(|err| (path.to_path_buf(), err))?;
            let name = replace_prefixes(prefixes, path.display().to_string());

            file_line_maps.push(LineMap::from_str(&contents));
            file_names.push(name);
            file_texts.push(contents);
        }

        Ok(Self {
            file_names,
            file_texts,
            file_line_maps,
        })
    }

    pub fn from_names_and_contents(files: impl IntoIterator<Item = (String, String)>) -> Self {
        let mut file_names = Vec::new();
        let mut file_texts = Vec::new();
        let mut file_line_maps = Vec::new();

        for (name, contents) in files {
            file_line_maps.push(LineMap::from_str(&contents));
            file_names.push(name);
            file_texts.push(contents);
        }

        Self {
            file_names,
            file_texts,
            file_line_maps,
        }
    }

    pub fn file_name(&self, file: usize) -> Option<&str> {
        self.file_names.get(file).map(|x| x.as_str())
    }

    pub fn file_text(&self, file: usize) -> Option<&str> {
        self.file_texts.get(file).map(|x| x.as_str())
    }

    pub fn file_line_map(&self, file: usize) -> Option<&LineMap> {
        self.file_line_maps.get(file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_map() {
        let line_map = LineMap::from_str("");
        assert_eq!(line_map.line_ends, &[]);
        assert_eq!(line_map.line(0), 0);
        assert_eq!(line_map.line(100), 0);

        let line_map = LineMap::from_str("line 0");
        assert_eq!(line_map.line_ends, &[]);
        assert_eq!(line_map.line(0), 0);
        assert_eq!(line_map.line(100), 0);

        let line_map = LineMap::from_str("line 0\nline 1");
        assert_eq!(line_map.line_ends, &[7]);
        assert_eq!(line_map.line(0), 0);
        assert_eq!(line_map.line(100), 1);
    }
}
