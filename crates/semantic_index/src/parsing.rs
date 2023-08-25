use anyhow::{anyhow, Ok, Result};
use language::{Grammar, Language};
use sha1::{Digest, Sha1};
use std::{
    cmp::{self, Reverse},
    collections::HashSet,
    ops::Range,
    path::Path,
    sync::Arc,
};
use tree_sitter::{Parser, QueryCursor};

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub name: String,
    pub range: Range<usize>,
    pub content: String,
    pub embedding: Vec<f32>,
    pub sha1: [u8; 20],
}

const CODE_CONTEXT_TEMPLATE: &str =
    "The below code snippet is from file '<path>'\n\n```<language>\n<item>\n```";
const ENTIRE_FILE_TEMPLATE: &str =
    "The below snippet is from file '<path>'\n\n```<language>\n<item>\n```";
const MARKDOWN_CONTEXT_TEMPLATE: &str = "The below file contents is from file '<path>'\n\n<item>";
pub const PARSEABLE_ENTIRE_FILE_TYPES: &[&str] =
    &["TOML", "YAML", "CSS", "HEEX", "ERB", "SVELTE", "HTML"];

pub struct CodeContextRetriever {
    pub parser: Parser,
    pub cursor: QueryCursor,
}

// Every match has an item, this represents the fundamental treesitter symbol and anchors the search
// Every match has one or more 'name' captures. These indicate the display range of the item for deduplication.
// If there are preceeding comments, we track this with a context capture
// If there is a piece that should be collapsed in hierarchical queries, we capture it with a collapse capture
// If there is a piece that should be kept inside a collapsed node, we capture it with a keep capture
#[derive(Debug, Clone)]
pub struct CodeContextMatch {
    pub start_col: usize,
    pub item_range: Option<Range<usize>>,
    pub name_range: Option<Range<usize>>,
    pub context_ranges: Vec<Range<usize>>,
    pub collapse_ranges: Vec<Range<usize>>,
}

impl CodeContextRetriever {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            cursor: QueryCursor::new(),
        }
    }

    fn parse_entire_file(
        &self,
        relative_path: &Path,
        language_name: Arc<str>,
        content: &str,
    ) -> Result<Vec<Document>> {
        let document_span = ENTIRE_FILE_TEMPLATE
            .replace("<path>", relative_path.to_string_lossy().as_ref())
            .replace("<language>", language_name.as_ref())
            .replace("<item>", &content);

        let mut sha1 = Sha1::new();
        sha1.update(&document_span);

        Ok(vec![Document {
            range: 0..content.len(),
            content: document_span,
            embedding: Vec::new(),
            name: language_name.to_string(),
            sha1: sha1.finalize().into(),
        }])
    }

    fn parse_markdown_file(&self, relative_path: &Path, content: &str) -> Result<Vec<Document>> {
        let document_span = MARKDOWN_CONTEXT_TEMPLATE
            .replace("<path>", relative_path.to_string_lossy().as_ref())
            .replace("<item>", &content);

        let mut sha1 = Sha1::new();
        sha1.update(&document_span);

        Ok(vec![Document {
            range: 0..content.len(),
            content: document_span,
            embedding: Vec::new(),
            name: "Markdown".to_string(),
            sha1: sha1.finalize().into(),
        }])
    }

    fn get_matches_in_file(
        &mut self,
        content: &str,
        grammar: &Arc<Grammar>,
    ) -> Result<Vec<CodeContextMatch>> {
        let embedding_config = grammar
            .embedding_config
            .as_ref()
            .ok_or_else(|| anyhow!("no embedding queries"))?;
        self.parser.set_language(grammar.ts_language).unwrap();

        let tree = self
            .parser
            .parse(&content, None)
            .ok_or_else(|| anyhow!("parsing failed"))?;

        let mut captures: Vec<CodeContextMatch> = Vec::new();
        let mut collapse_ranges: Vec<Range<usize>> = Vec::new();
        let mut keep_ranges: Vec<Range<usize>> = Vec::new();
        for mat in self.cursor.matches(
            &embedding_config.query,
            tree.root_node(),
            content.as_bytes(),
        ) {
            let mut start_col = 0;
            let mut item_range: Option<Range<usize>> = None;
            let mut name_range: Option<Range<usize>> = None;
            let mut context_ranges: Vec<Range<usize>> = Vec::new();
            collapse_ranges.clear();
            keep_ranges.clear();
            for capture in mat.captures {
                if capture.index == embedding_config.item_capture_ix {
                    item_range = Some(capture.node.byte_range());
                    start_col = capture.node.start_position().column;
                } else if Some(capture.index) == embedding_config.name_capture_ix {
                    name_range = Some(capture.node.byte_range());
                } else if Some(capture.index) == embedding_config.context_capture_ix {
                    context_ranges.push(capture.node.byte_range());
                } else if Some(capture.index) == embedding_config.collapse_capture_ix {
                    collapse_ranges.push(capture.node.byte_range());
                } else if Some(capture.index) == embedding_config.keep_capture_ix {
                    keep_ranges.push(capture.node.byte_range());
                }
            }

            captures.push(CodeContextMatch {
                start_col,
                item_range,
                name_range,
                context_ranges,
                collapse_ranges: subtract_ranges(&collapse_ranges, &keep_ranges),
            });
        }
        Ok(captures)
    }

    pub fn parse_file_with_template(
        &mut self,
        relative_path: &Path,
        content: &str,
        language: Arc<Language>,
    ) -> Result<Vec<Document>> {
        let language_name = language.name();

        if PARSEABLE_ENTIRE_FILE_TYPES.contains(&language_name.as_ref()) {
            return self.parse_entire_file(relative_path, language_name, &content);
        } else if &language_name.to_string() == &"Markdown".to_string() {
            return self.parse_markdown_file(relative_path, &content);
        }

        let mut documents = self.parse_file(content, language)?;
        for document in &mut documents {
            document.content = CODE_CONTEXT_TEMPLATE
                .replace("<path>", relative_path.to_string_lossy().as_ref())
                .replace("<language>", language_name.as_ref())
                .replace("item", &document.content);
        }
        Ok(documents)
    }

    pub fn parse_file(&mut self, content: &str, language: Arc<Language>) -> Result<Vec<Document>> {
        let grammar = language
            .grammar()
            .ok_or_else(|| anyhow!("no grammar for language"))?;

        // Iterate through query matches
        let matches = self.get_matches_in_file(content, grammar)?;

        let language_scope = language.default_scope();
        let placeholder = language_scope.collapsed_placeholder();

        let mut documents = Vec::new();
        let mut collapsed_ranges_within = Vec::new();
        let mut parsed_name_ranges = HashSet::new();
        for (i, context_match) in matches.iter().enumerate() {
            // Items which are collapsible but not embeddable have no item range
            let item_range = if let Some(item_range) = context_match.item_range.clone() {
                item_range
            } else {
                continue;
            };

            // Checks for deduplication
            let name;
            if let Some(name_range) = context_match.name_range.clone() {
                name = content
                    .get(name_range.clone())
                    .map_or(String::new(), |s| s.to_string());
                if parsed_name_ranges.contains(&name_range) {
                    continue;
                }
                parsed_name_ranges.insert(name_range);
            } else {
                name = String::new();
            }

            collapsed_ranges_within.clear();
            'outer: for remaining_match in &matches[(i + 1)..] {
                for collapsed_range in &remaining_match.collapse_ranges {
                    if item_range.start <= collapsed_range.start
                        && item_range.end >= collapsed_range.end
                    {
                        collapsed_ranges_within.push(collapsed_range.clone());
                    } else {
                        break 'outer;
                    }
                }
            }

            collapsed_ranges_within.sort_by_key(|r| (r.start, Reverse(r.end)));

            let mut document_content = String::new();
            for context_range in &context_match.context_ranges {
                add_content_from_range(
                    &mut document_content,
                    content,
                    context_range.clone(),
                    context_match.start_col,
                );
                document_content.push_str("\n");
            }

            let mut offset = item_range.start;
            for collapsed_range in &collapsed_ranges_within {
                if collapsed_range.start > offset {
                    add_content_from_range(
                        &mut document_content,
                        content,
                        offset..collapsed_range.start,
                        context_match.start_col,
                    );
                    offset = collapsed_range.start;
                }

                if collapsed_range.end > offset {
                    document_content.push_str(placeholder);
                    offset = collapsed_range.end;
                }
            }

            if offset < item_range.end {
                add_content_from_range(
                    &mut document_content,
                    content,
                    offset..item_range.end,
                    context_match.start_col,
                );
            }

            let mut sha1 = Sha1::new();
            sha1.update(&document_content);

            documents.push(Document {
                name,
                content: document_content,
                range: item_range.clone(),
                embedding: vec![],
                sha1: sha1.finalize().into(),
            })
        }

        return Ok(documents);
    }
}

pub(crate) fn subtract_ranges(
    ranges: &[Range<usize>],
    ranges_to_subtract: &[Range<usize>],
) -> Vec<Range<usize>> {
    let mut result = Vec::new();

    let mut ranges_to_subtract = ranges_to_subtract.iter().peekable();

    for range in ranges {
        let mut offset = range.start;

        while offset < range.end {
            if let Some(range_to_subtract) = ranges_to_subtract.peek() {
                if offset < range_to_subtract.start {
                    let next_offset = cmp::min(range_to_subtract.start, range.end);
                    result.push(offset..next_offset);
                    offset = next_offset;
                } else {
                    let next_offset = cmp::min(range_to_subtract.end, range.end);
                    offset = next_offset;
                }

                if offset >= range_to_subtract.end {
                    ranges_to_subtract.next();
                }
            } else {
                result.push(offset..range.end);
                offset = range.end;
            }
        }
    }

    result
}

fn add_content_from_range(
    output: &mut String,
    content: &str,
    range: Range<usize>,
    start_col: usize,
) {
    for mut line in content.get(range.clone()).unwrap_or("").lines() {
        for _ in 0..start_col {
            if line.starts_with(' ') {
                line = &line[1..];
            } else {
                break;
            }
        }
        output.push_str(line);
        output.push('\n');
    }
    output.pop();
}
