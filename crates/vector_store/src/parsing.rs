use std::{path::PathBuf, sync::Arc, time::SystemTime};

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

const CODE_CONTEXT_TEMPLATE: &str =
    "The below code snippet is from file '<path>'\n\n```<language>\n<item>\n```";

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
            // log::info!("-----MATCH-----");

            let mut name: Vec<&str> = vec![];
            let mut item: Option<&str> = None;
            let mut offset: Option<usize> = None;
            for capture in mat.captures {
                if capture.index == embedding_config.item_capture_ix {
                    offset = Some(capture.node.byte_range().start);
                    item = content.get(capture.node.byte_range());
                } else if capture.index == embedding_config.name_capture_ix {
                    if let Some(name_content) = content.get(capture.node.byte_range()) {
                        name.push(name_content);
                    }
                }

                if let Some(context_capture_ix) = embedding_config.context_capture_ix {
                    if capture.index == context_capture_ix {
                        if let Some(context) = content.get(capture.node.byte_range()) {
                            name.push(context);
                        }
                    }
                }
            }

            if item.is_some() && offset.is_some() && name.len() > 0 {
                let context_span = CODE_CONTEXT_TEMPLATE
                    .replace("<path>", pending_file.relative_path.to_str().unwrap())
                    .replace("<language>", &pending_file.language.name().to_lowercase())
                    .replace("<item>", item.unwrap());

                let mut truncated_span = context_span.clone();
                truncated_span.truncate(100);

                // log::info!("Name:       {:?}", name);
                // log::info!("Span:       {:?}", truncated_span);

                context_spans.push(context_span);
                documents.push(Document {
                    name: name.join(" "),
                    offset: offset.unwrap(),
                    embedding: Vec::new(),
                })
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
