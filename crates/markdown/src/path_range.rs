use std::ops::Range;

#[derive(Debug)]
pub struct PathRange<'a> {
    pub path: &'a str,
    pub range: Option<Range<LineCol>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: Option<u32>,
}

impl LineCol {
    pub fn new(str: impl AsRef<str>) -> Option<Self> {
        let str = str.as_ref();
        match str.split_once(':') {
            Some((line, col)) => match (line.parse::<u32>(), col.parse::<u32>()) {
                (Ok(line), Ok(col)) => Some(Self {
                    line,
                    col: Some(col),
                }),
                _ => None,
            },
            None => match str.parse::<u32>() {
                Ok(line) => Some(Self { line, col: None }),
                Err(_) => None,
            },
        }
    }
}

impl<'a> PathRange<'a> {
    pub fn new(str: &'a str) -> Self {
        // Sometimes the model will include a language at the start,
        // e.g. "```rust zed/crates/markdown/src/markdown.rs#L1"
        // We just discard that.
        let str = match str.trim_end().rfind(' ') {
            Some(space) => &str[space + 1..],
            None => str.trim_start(),
        };

        match str.rsplit_once('#') {
            Some((path, after_hash)) => {
                // Be tolerant to the model omitting the "L" prefix, lowercasing it,
                // or including it more than once.
                let after_hash = after_hash.replace('L', "").replace('l', "");

                let range = {
                    let mut iter = after_hash.split('-').flat_map(LineCol::new);
                    iter.next()
                        .map(|start| iter.next().map(|end| start..end).unwrap_or(start..start))
                };

                Self { path, range }
            }
            None => Self {
                path: str,
                range: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper method to handle the case where we want to return None if the path doesn't contain a range
    fn path_range_from_str(str: &str) -> Option<PathRange> {
        let path_range = PathRange::new(str);
        if path_range.range.is_some() {
            Some(path_range)
        } else {
            None
        }
    }

    #[test]
    fn test_linecol_parsing() {
        // Test line and column format
        let line_col = LineCol::new("10:5");
        assert_eq!(
            line_col,
            Some(LineCol {
                line: 10,
                col: Some(5)
            })
        );

        // Test line-only format
        let line_only = LineCol::new("42");
        assert_eq!(
            line_only,
            Some(LineCol {
                line: 42,
                col: None
            })
        );

        // Test invalid inputs
        assert_eq!(LineCol::new(""), None);
        assert_eq!(LineCol::new("not a number"), None);
        assert_eq!(LineCol::new("10:not a number"), None);
        assert_eq!(LineCol::new("not:5"), None);
    }

    #[test]
    fn test_pathrange_parsing() {
        // Test path with line range
        let path_range = PathRange::new("file.rs#L10-L20");
        assert_eq!(path_range.path, "file.rs");
        assert!(path_range.range.is_some());
        if let Some(range) = path_range.range {
            assert_eq!(range.start.line, 10);
            assert_eq!(range.start.col, None);
            assert_eq!(range.end.line, 20);
            assert_eq!(range.end.col, None);
        }

        // Test path with single line
        let single_line = PathRange::new("file.rs#L15");
        assert_eq!(single_line.path, "file.rs");
        assert!(single_line.range.is_some());
        if let Some(range) = single_line.range {
            assert_eq!(range.start.line, 15);
            assert_eq!(range.end.line, 15);
        }

        // Test path with no range
        let no_range = PathRange::new("file.rs");
        assert_eq!(no_range.path, "file.rs");
        assert!(no_range.range.is_none());

        // Test lowercase 'l' handling
        let lowercase = PathRange::new("file.rs#l5-l10");
        assert_eq!(lowercase.path, "file.rs");
        assert!(lowercase.range.is_some());
        if let Some(range) = lowercase.range {
            assert_eq!(range.start.line, 5);
            assert_eq!(range.end.line, 10);
        }

        // Test with complex path
        let complex = PathRange::new("src/path/to/file.rs#L100");
        assert_eq!(complex.path, "src/path/to/file.rs");
        assert!(complex.range.is_some());
    }

    #[test]
    fn test_pathrange_from_str() {
        // Test with range - should return Some
        let with_range = path_range_from_str("file.rs#L10-L20");
        assert!(with_range.is_some());
        if let Some(path_range) = with_range {
            assert_eq!(path_range.path, "file.rs");
            assert!(path_range.range.is_some());
        }

        // Test without range - should return None
        let without_range = path_range_from_str("file.rs");
        assert!(without_range.is_none());

        // Test with single line - should return Some
        let single_line = path_range_from_str("file.rs#L15");
        assert!(single_line.is_some());
    }

    #[test]
    fn test_pathrange_leading_text_trimming() {
        // Test with language identifier prefix
        let with_language = PathRange::new("```rust file.rs#L10");
        assert_eq!(with_language.path, "file.rs");
        assert!(with_language.range.is_some());
        if let Some(range) = with_language.range {
            assert_eq!(range.start.line, 10);
        }

        // Test with multiple spaces
        let with_spaces = PathRange::new("```    file.rs#L10-L20");
        assert_eq!(with_spaces.path, "file.rs");
        assert!(with_spaces.range.is_some());

        // Test with multiple words before path
        let with_words = PathRange::new("```rust code example file.rs#L15:10");
        assert_eq!(with_words.path, "file.rs");
        assert!(with_words.range.is_some());
        if let Some(range) = with_words.range {
            assert_eq!(range.start.line, 15);
            assert_eq!(range.start.col, Some(10));
        }

        // Test with just whitespace at start
        let with_whitespace = PathRange::new("  file.rs#L5");
        assert_eq!(with_whitespace.path, "file.rs");
        assert!(with_whitespace.range.is_some());

        // Test with no leading text (unchanged behavior)
        let no_leading = PathRange::new("file.rs#L10");
        assert_eq!(no_leading.path, "file.rs");
        assert!(no_leading.range.is_some());
    }

    #[test]
    fn test_pathrange_with_line_and_column() {
        // Test with single line and column
        let line_and_col = PathRange::new("file.rs#L10:5");
        assert_eq!(line_and_col.path, "file.rs");
        assert!(line_and_col.range.is_some());
        if let Some(range) = line_and_col.range {
            assert_eq!(range.start.line, 10);
            assert_eq!(range.start.col, Some(5));
            assert_eq!(range.end.line, 10); // Same as start for single-line
            assert_eq!(range.end.col, Some(5));
        }

        // Test with range from line+col to line+col
        let full_range = PathRange::new("file.rs#L10:5-L20:15");
        assert_eq!(full_range.path, "file.rs");
        assert!(full_range.range.is_some());
        if let Some(range) = full_range.range {
            assert_eq!(range.start.line, 10);
            assert_eq!(range.start.col, Some(5));
            assert_eq!(range.end.line, 20);
            assert_eq!(range.end.col, Some(15));
        }

        // Test with range from line+col to just line
        let mixed_range1 = PathRange::new("file.rs#L10:5-L20");
        assert_eq!(mixed_range1.path, "file.rs");
        assert!(mixed_range1.range.is_some());
        if let Some(range) = mixed_range1.range {
            assert_eq!(range.start.line, 10);
            assert_eq!(range.start.col, Some(5));
            assert_eq!(range.end.line, 20);
            assert_eq!(range.end.col, None);
        }

        // Test with range from just line to line+col
        let mixed_range2 = PathRange::new("file.rs#L10-L20:15");
        assert_eq!(mixed_range2.path, "file.rs");
        assert!(mixed_range2.range.is_some());
        if let Some(range) = mixed_range2.range {
            assert_eq!(range.start.line, 10);
            assert_eq!(range.start.col, None);
            assert_eq!(range.end.line, 20);
            assert_eq!(range.end.col, Some(15));
        }
    }
}