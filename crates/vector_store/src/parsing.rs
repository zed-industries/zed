use anyhow::{anyhow, Ok, Result};
use language::Language;
use std::{ops::Range, path::Path, sync::Arc};
use tree_sitter::{Parser, QueryCursor};

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub name: String,
    pub range: Range<usize>,
    pub content: String,
    pub embedding: Vec<f32>,
}

const CODE_CONTEXT_TEMPLATE: &str =
    "The below code snippet is from file '<path>'\n\n```<language>\n<item>\n```";

pub struct CodeContextRetriever {
    pub parser: Parser,
    pub cursor: QueryCursor,
}

impl CodeContextRetriever {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            cursor: QueryCursor::new(),
        }
    }

    pub fn parse_file(
        &mut self,
        relative_path: &Path,
        content: &str,
        language: Arc<Language>,
    ) -> Result<Vec<Document>> {
        let grammar = language
            .grammar()
            .ok_or_else(|| anyhow!("no grammar for language"))?;
        let embedding_config = grammar
            .embedding_config
            .as_ref()
            .ok_or_else(|| anyhow!("no embedding queries"))?;

        self.parser.set_language(grammar.ts_language).unwrap();

        let tree = self
            .parser
            .parse(&content, None)
            .ok_or_else(|| anyhow!("parsing failed"))?;

        let mut documents = Vec::new();

        // Iterate through query matches
        for mat in self.cursor.matches(
            &embedding_config.query,
            tree.root_node(),
            content.as_bytes(),
        ) {
            let mut name: Vec<&str> = vec![];
            let mut item: Option<&str> = None;
            let mut byte_range: Option<Range<usize>> = None;
            let mut context_spans: Vec<&str> = vec![];
            for capture in mat.captures {
                if capture.index == embedding_config.item_capture_ix {
                    byte_range = Some(capture.node.byte_range());
                    item = content.get(capture.node.byte_range());
                } else if capture.index == embedding_config.name_capture_ix {
                    if let Some(name_content) = content.get(capture.node.byte_range()) {
                        name.push(name_content);
                    }
                }

                if let Some(context_capture_ix) = embedding_config.context_capture_ix {
                    if capture.index == context_capture_ix {
                        if let Some(context) = content.get(capture.node.byte_range()) {
                            context_spans.push(context);
                        }
                    }
                }
            }

            if let Some((item, byte_range)) = item.zip(byte_range) {
                if !name.is_empty() {
                    let item = format!("{}\n{}", context_spans.join("\n"), item);

                    let document_text = CODE_CONTEXT_TEMPLATE
                        .replace("<path>", relative_path.to_str().unwrap())
                        .replace("<language>", &language.name().to_lowercase())
                        .replace("<item>", item.as_str());

                    documents.push(Document {
                        range: byte_range,
                        content: document_text,
                        embedding: Vec::new(),
                        name: name.join(" ").to_string(),
                    });
                }
            }
        }

        return Ok(documents);
    }
}
