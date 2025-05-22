use language::{Language, with_parser, with_query_cursor};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    cmp::{self, Reverse},
    ops::Range,
    path::Path,
    sync::Arc,
};
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCapture;
use util::ResultExt as _;

#[derive(Copy, Clone)]
struct ChunkSizeRange {
    min: usize,
    max: usize,
}

const CHUNK_SIZE_RANGE: ChunkSizeRange = ChunkSizeRange {
    min: 1024,
    max: 8192,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub range: Range<usize>,
    pub digest: [u8; 32],
}

pub fn chunk_text(text: &str, language: Option<&Arc<Language>>, path: &Path) -> Vec<Chunk> {
    chunk_text_with_size_range(text, language, path, CHUNK_SIZE_RANGE)
}

fn chunk_text_with_size_range(
    text: &str,
    language: Option<&Arc<Language>>,
    path: &Path,
    size_config: ChunkSizeRange,
) -> Vec<Chunk> {
    let ranges = syntactic_ranges(text, language, path).unwrap_or_default();
    chunk_text_with_syntactic_ranges(text, &ranges, size_config)
}

fn syntactic_ranges(
    text: &str,
    language: Option<&Arc<Language>>,
    path: &Path,
) -> Option<Vec<Range<usize>>> {
    let language = language?;
    let grammar = language.grammar()?;
    let outline = grammar.outline_config.as_ref()?;
    let tree = with_parser(|parser| {
        parser.set_language(&grammar.ts_language).log_err()?;
        parser.parse(text, None)
    });

    let Some(tree) = tree else {
        log::error!("failed to parse file {path:?} for chunking");
        return None;
    };

    struct RowInfo {
        offset: usize,
        is_comment: bool,
    }

    let scope = language.default_scope();
    let line_comment_prefixes = scope.line_comment_prefixes();
    let row_infos = text
        .split('\n')
        .map({
            let mut offset = 0;
            move |line| {
                let line = line.trim_start();
                let is_comment = line_comment_prefixes
                    .iter()
                    .any(|prefix| line.starts_with(prefix.as_ref()));
                let result = RowInfo { offset, is_comment };
                offset += line.len() + 1;
                result
            }
        })
        .collect::<Vec<_>>();

    // Retrieve a list of ranges of outline items (types, functions, etc) in the document.
    // Omit single-line outline items (e.g. struct fields, constant declarations), because
    // we'll already be attempting to split on lines.
    let mut ranges = with_query_cursor(|cursor| {
        cursor
            .matches(&outline.query, tree.root_node(), text.as_bytes())
            .filter_map_deref(|mat| {
                mat.captures
                    .iter()
                    .find_map(|QueryCapture { node, index }| {
                        if *index == outline.item_capture_ix {
                            let mut start_offset = node.start_byte();
                            let mut start_row = node.start_position().row;
                            let end_offset = node.end_byte();
                            let end_row = node.end_position().row;

                            // Expand the range to include any preceding comments.
                            while start_row > 0 && row_infos[start_row - 1].is_comment {
                                start_offset = row_infos[start_row - 1].offset;
                                start_row -= 1;
                            }

                            if end_row > start_row {
                                return Some(start_offset..end_offset);
                            }
                        }
                        None
                    })
            })
            .collect::<Vec<_>>()
    });

    ranges.sort_unstable_by_key(|range| (range.start, Reverse(range.end)));
    Some(ranges)
}

fn chunk_text_with_syntactic_ranges(
    text: &str,
    mut syntactic_ranges: &[Range<usize>],
    size_config: ChunkSizeRange,
) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut range = 0..0;
    let mut range_end_nesting_depth = 0;

    // Try to split the text at line boundaries.
    let mut line_ixs = text
        .match_indices('\n')
        .map(|(ix, _)| ix + 1)
        .chain(if text.ends_with('\n') {
            None
        } else {
            Some(text.len())
        })
        .peekable();

    while let Some(&line_ix) = line_ixs.peek() {
        // If the current position is beyond the maximum chunk size, then
        // start a new chunk.
        if line_ix - range.start > size_config.max {
            if range.is_empty() {
                range.end = cmp::min(range.start + size_config.max, line_ix);
                while !text.is_char_boundary(range.end) {
                    range.end -= 1;
                }
            }

            chunks.push(Chunk {
                range: range.clone(),
                digest: Sha256::digest(&text[range.clone()]).into(),
            });
            range_end_nesting_depth = 0;
            range.start = range.end;
            continue;
        }

        // Discard any syntactic ranges that end before the current position.
        while let Some(first_item) = syntactic_ranges.first() {
            if first_item.end < line_ix {
                syntactic_ranges = &syntactic_ranges[1..];
                continue;
            } else {
                break;
            }
        }

        // Count how many syntactic ranges contain the current position.
        let mut nesting_depth = 0;
        for range in syntactic_ranges {
            if range.start > line_ix {
                break;
            }
            if range.start < line_ix && range.end > line_ix {
                nesting_depth += 1;
            }
        }

        // Extend the current range to this position, unless an earlier candidate
        // end position was less nested syntactically.
        if range.len() < size_config.min || nesting_depth <= range_end_nesting_depth {
            range.end = line_ix;
            range_end_nesting_depth = nesting_depth;
        }

        line_ixs.next();
    }

    if !range.is_empty() {
        chunks.push(Chunk {
            range: range.clone(),
            digest: Sha256::digest(&text[range]).into(),
        });
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use language::{Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
    use unindent::Unindent as _;

    #[test]
    fn test_chunk_text_with_syntax() {
        let language = rust_language();

        let text = "
            struct Person {
                first_name: String,
                last_name: String,
                age: u32,
            }

            impl Person {
                fn new(first_name: String, last_name: String, age: u32) -> Self {
                    Self { first_name, last_name, age }
                }

                /// Returns the first name
                /// something something something
                fn first_name(&self) -> &str {
                    &self.first_name
                }

                fn last_name(&self) -> &str {
                    &self.last_name
                }

                fn age(&self) -> u32 {
                    self.age
                }
            }
        "
        .unindent();

        let chunks = chunk_text_with_size_range(
            &text,
            Some(&language),
            Path::new("lib.rs"),
            ChunkSizeRange {
                min: text.find('}').unwrap(),
                max: text.find("Self {").unwrap(),
            },
        );

        // The entire impl cannot fit in a chunk, so it is split.
        // Within the impl, two methods can fit in a chunk.
        assert_chunks(
            &text,
            &chunks,
            &[
                "struct Person {", // ...
                "impl Person {",
                "    /// Returns the first name",
                "    fn last_name",
            ],
        );

        let text = "
            struct T {}
            struct U {}
            struct V {}
            struct W {
                a: T,
                b: U,
            }
        "
        .unindent();

        let chunks = chunk_text_with_size_range(
            &text,
            Some(&language),
            Path::new("lib.rs"),
            ChunkSizeRange {
                min: text.find('{').unwrap(),
                max: text.find('V').unwrap(),
            },
        );

        // Two single-line structs can fit in a chunk.
        // The last struct cannot fit in a chunk together
        // with the previous single-line struct.
        assert_chunks(
            &text,
            &chunks,
            &[
                "struct T", // ...
                "struct V", // ...
                "struct W", // ...
                "}",
            ],
        );
    }

    #[test]
    fn test_chunk_with_long_lines() {
        let language = rust_language();

        let text = "
            struct S { a: u32 }
            struct T { a: u64 }
            struct U { a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, j: u64 }
            struct W { a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, j: u64 }
        "
        .unindent();

        let chunks = chunk_text_with_size_range(
            &text,
            Some(&language),
            Path::new("lib.rs"),
            ChunkSizeRange { min: 32, max: 64 },
        );

        // The line is too long to fit in one chunk
        assert_chunks(
            &text,
            &chunks,
            &[
                "struct S {", // ...
                "struct U",
                "4, h: u64, i: u64", // ...
                "struct W",
                "4, h: u64, i: u64", // ...
            ],
        );
    }

    #[track_caller]
    fn assert_chunks(text: &str, chunks: &[Chunk], expected_chunk_text_prefixes: &[&str]) {
        check_chunk_invariants(text, chunks);

        assert_eq!(
            chunks.len(),
            expected_chunk_text_prefixes.len(),
            "unexpected number of chunks: {chunks:?}",
        );

        let mut prev_chunk_end = 0;
        for (ix, chunk) in chunks.iter().enumerate() {
            let expected_prefix = expected_chunk_text_prefixes[ix];
            let chunk_text = &text[chunk.range.clone()];
            if !chunk_text.starts_with(expected_prefix) {
                let chunk_prefix_offset = text[prev_chunk_end..].find(expected_prefix);
                if let Some(chunk_prefix_offset) = chunk_prefix_offset {
                    panic!(
                        "chunk {ix} starts at unexpected offset {}. expected {}",
                        chunk.range.start,
                        chunk_prefix_offset + prev_chunk_end
                    );
                } else {
                    panic!("invalid expected chunk prefix {ix}: {expected_prefix:?}");
                }
            }
            prev_chunk_end = chunk.range.end;
        }
    }

    #[track_caller]
    fn check_chunk_invariants(text: &str, chunks: &[Chunk]) {
        for (ix, chunk) in chunks.iter().enumerate() {
            if ix > 0 && chunk.range.start != chunks[ix - 1].range.end {
                panic!("chunk ranges are not contiguous: {:?}", chunks);
            }
        }

        if text.is_empty() {
            assert!(chunks.is_empty())
        } else if chunks.first().unwrap().range.start != 0
            || chunks.last().unwrap().range.end != text.len()
        {
            panic!("chunks don't cover entire text {:?}", chunks);
        }
    }

    #[test]
    fn test_chunk_text() {
        let text = "a\n".repeat(1000);
        let chunks = chunk_text(&text, None, Path::new("lib.rs"));
        assert_eq!(
            chunks.len(),
            ((2000_f64) / (CHUNK_SIZE_RANGE.max as f64)).ceil() as usize
        );
    }

    fn rust_language() -> Arc<Language> {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Rust".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_outline_query(
                "
            (function_item name: (_) @name) @item
            (impl_item type: (_) @name) @item
            (struct_item name: (_) @name) @item
            (field_declaration name: (_) @name) @item
        ",
            )
            .unwrap(),
        )
    }
}
