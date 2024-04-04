use language::{Grammar, Tree, PARSER};
use sha2::{Digest, Sha256};
use std::{cmp, ops::Range, sync::Arc};

const CHUNK_THRESHOLD: usize = 1500;

pub struct Chunk {
    pub range: Range<usize>,
    digest: [u8; 32],
}

pub fn chunk_text(text: &str, grammar: Option<&Arc<Grammar>>) -> Vec<Chunk> {
    if let Some(grammar) = grammar {
        let tree = PARSER.with(|parser| {
            let mut parser = parser.borrow_mut();
            parser
                .set_language(&grammar.ts_language)
                .expect("incompatible grammar");
            parser.parse(&text, None).expect("invalid language")
        });

        chunk_parse_tree(tree, &text)
    } else {
        chunk_lines(&text)
    }
}

fn chunk_parse_tree(tree: Tree, text: &str) -> Vec<Chunk> {
    let mut chunk_ranges = Vec::new();
    let mut cursor = tree.walk();

    let mut range = 0..0;
    loop {
        let node = cursor.node();

        // If adding the node to the current chunk exceeds the threshold
        if node.end_byte() - range.start > CHUNK_THRESHOLD {
            // Try to descend into its first child. If we can't, flush the current
            // range and try again.
            if cursor.goto_first_child() {
                continue;
            } else if !range.is_empty() {
                chunk_ranges.push(range.clone());
                range.start = range.end;
                continue;
            }

            // If we get here, the node itself has no children but is larger than the threshold.
            // Break its text into arbitrary chunks.
            split_text(text, range.clone(), node.end_byte(), &mut chunk_ranges);
        }
        range.end = node.end_byte();

        // If we get here, we consumed the node. Advance to the next child, ascending if there isn't one.
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                if !range.is_empty() {
                    chunk_ranges.push(range);
                }

                return chunk_ranges
                    .into_iter()
                    .map(|range| {
                        let mut hasher = Sha256::new();
                        hasher.update(&text[range.clone()]);
                        let mut digest = [0u8; 32];
                        digest.copy_from_slice(hasher.finalize().as_slice());
                        Chunk { range, digest }
                    })
                    .collect();
            }
        }
    }
}

fn chunk_lines(text: &str) -> Vec<Chunk> {
    let mut chunk_ranges = Vec::new();
    let mut range = 0..0;

    let mut newlines = text.match_indices('\n').peekable();
    while let Some((newline_ix, _)) = newlines.peek() {
        let newline_ix = newline_ix + 1;
        if newline_ix - range.start <= CHUNK_THRESHOLD {
            range.end = newline_ix;
            newlines.next();
        } else {
            if range.is_empty() {
                split_text(text, range, newline_ix, &mut chunk_ranges);
                range = newline_ix..newline_ix;
            } else {
                chunk_ranges.push(range.clone());
                range.start = range.end;
            }
        }
    }

    if !range.is_empty() {
        chunk_ranges.push(range);
    }

    chunk_ranges
        .into_iter()
        .map(|range| {
            let mut hasher = Sha256::new();
            hasher.update(&text[range.clone()]);
            let mut digest = [0u8; 32];
            digest.copy_from_slice(hasher.finalize().as_slice());
            Chunk { range, digest }
        })
        .collect()
}

fn split_text(
    text: &str,
    mut range: Range<usize>,
    max_end: usize,
    chunk_ranges: &mut Vec<Range<usize>>,
) {
    while range.start < max_end {
        range.end = cmp::min(range.start + CHUNK_THRESHOLD, max_end);
        while !text.is_char_boundary(range.end) {
            range.end -= 1;
        }
        chunk_ranges.push(range.clone());
        range.start = range.end;
    }
}
