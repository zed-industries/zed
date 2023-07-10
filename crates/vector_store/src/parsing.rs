use std::{ops::Range, path::PathBuf, sync::Arc, time::SystemTime};

use anyhow::{anyhow, Ok, Result};
use project::Fs;
use tree_sitter::{Parser, QueryCursor};

use crate::PendingFile;

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub offset: usize,
    pub name: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedFile {
    pub path: PathBuf,
    pub mtime: SystemTime,
    pub documents: Vec<Document>,
}

pub struct CodeContextRetriever {
    pub parser: Parser,
    pub cursor: QueryCursor,
    pub fs: Arc<dyn Fs>,
}

impl CodeContextRetriever {
    pub async fn parse_file(
        &mut self,
        pending_file: PendingFile,
    ) -> Result<(ParsedFile, Vec<String>)> {
        let grammar = pending_file
            .language
            .grammar()
            .ok_or_else(|| anyhow!("no grammar for language"))?;
        let embedding_config = grammar
            .embedding_config
            .as_ref()
            .ok_or_else(|| anyhow!("no embedding queries"))?;

        let content = self.fs.load(&pending_file.absolute_path).await?;

        self.parser.set_language(grammar.ts_language).unwrap();

        let tree = self
            .parser
            .parse(&content, None)
            .ok_or_else(|| anyhow!("parsing failed"))?;

        let mut documents = Vec::new();
        let mut context_spans = Vec::new();

        // Iterate through query matches
        for mat in self.cursor.matches(
            &embedding_config.query,
            tree.root_node(),
            content.as_bytes(),
        ) {
            let mut item_range: Option<Range<usize>> = None;
            let mut name_range: Option<Range<usize>> = None;
            for capture in mat.captures {
                if capture.index == embedding_config.item_capture_ix {
                    item_range = Some(capture.node.byte_range());
                } else if capture.index == embedding_config.name_capture_ix {
                    name_range = Some(capture.node.byte_range());
                }
            }

            if let Some((item_range, name_range)) = item_range.zip(name_range) {
                if let Some((item, name)) =
                    content.get(item_range.clone()).zip(content.get(name_range))
                {
                    context_spans.push(item.to_string());
                    documents.push(Document {
                        name: name.to_string(),
                        offset: item_range.start,
                        embedding: Vec::new(),
                    });
                }
            }
        }

        return Ok((
            ParsedFile {
                path: pending_file.relative_path,
                mtime: pending_file.modified_time,
                documents,
            },
            context_spans,
        ));
    }
}
