use collections::HashMap;
use futures::channel::mpsc::UnboundedReceiver;
use language::{Language, LanguageRegistry};
use lsp::{
    FakeLanguageServer, LanguageServerBinary, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};
use parking_lot::Mutex;
use project::Fs;
use std::{ops::Range, path::PathBuf, sync::Arc};
use tree_sitter::{Parser, QueryCursor, StreamingIterator, Tree};

/// Registers a fake language server that implements go-to-definition using tree-sitter,
/// making the assumption that all names are unique, and all variables' types are
/// explicitly declared.
pub fn register_fake_definition_server(
    language_registry: &Arc<LanguageRegistry>,
    language: Arc<Language>,
    fs: Arc<dyn Fs>,
) -> UnboundedReceiver<FakeLanguageServer> {
    let index = Arc::new(Mutex::new(DefinitionIndex::new(language.clone())));

    language_registry.register_fake_lsp(
        language.name(),
        language::FakeLspAdapter {
            name: "fake-definition-lsp",
            initialization_options: None,
            prettier_plugins: Vec::new(),
            disk_based_diagnostics_progress_token: None,
            disk_based_diagnostics_sources: Vec::new(),
            language_server_binary: LanguageServerBinary {
                path: PathBuf::from("fake-definition-lsp"),
                arguments: Vec::new(),
                env: None,
            },
            capabilities: lsp::ServerCapabilities {
                definition_provider: Some(lsp::OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            label_for_completion: None,
            initializer: Some(Box::new({
                move |server| {
                    server.handle_notification::<lsp::notification::DidOpenTextDocument, _>({
                        let index = index.clone();
                        move |params, _cx| {
                            index
                                .lock()
                                .open_buffer(params.text_document.uri, &params.text_document.text);
                        }
                    });

                    server.handle_notification::<lsp::notification::DidCloseTextDocument, _>({
                        let index = index.clone();
                        let fs = fs.clone();
                        move |params, cx| {
                            let uri = params.text_document.uri;
                            let path = uri.to_file_path().ok();
                            index.lock().mark_buffer_closed(&uri);

                            if let Some(path) = path {
                                let index = index.clone();
                                let fs = fs.clone();
                                cx.spawn(async move |_cx| {
                                    if let Ok(content) = fs.load(&path).await {
                                        index.lock().index_file(uri, &content);
                                    }
                                })
                                .detach();
                            }
                        }
                    });

                    server.handle_notification::<lsp::notification::DidChangeWatchedFiles, _>({
                        let index = index.clone();
                        let fs = fs.clone();
                        move |params, cx| {
                            let index = index.clone();
                            let fs = fs.clone();
                            cx.spawn(async move |_cx| {
                                for event in params.changes {
                                    if index.lock().is_buffer_open(&event.uri) {
                                        continue;
                                    }

                                    match event.typ {
                                        lsp::FileChangeType::DELETED => {
                                            index.lock().remove_definitions_for_file(&event.uri);
                                        }
                                        lsp::FileChangeType::CREATED
                                        | lsp::FileChangeType::CHANGED => {
                                            if let Some(path) = event.uri.to_file_path().ok() {
                                                if let Ok(content) = fs.load(&path).await {
                                                    index.lock().index_file(event.uri, &content);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            })
                            .detach();
                        }
                    });

                    server.handle_notification::<lsp::notification::DidChangeTextDocument, _>({
                        let index = index.clone();
                        move |params, _cx| {
                            if let Some(change) = params.content_changes.into_iter().last() {
                                index
                                    .lock()
                                    .index_file(params.text_document.uri, &change.text);
                            }
                        }
                    });

                    server.handle_notification::<lsp::notification::DidChangeWorkspaceFolders, _>(
                        {
                            let index = index.clone();
                            let fs = fs.clone();
                            move |params, cx| {
                                let index = index.clone();
                                let fs = fs.clone();
                                let files = fs.as_fake().files();
                                cx.spawn(async move |_cx| {
                                    for folder in params.event.added {
                                        let Ok(path) = folder.uri.to_file_path() else {
                                            continue;
                                        };
                                        for file in &files {
                                            if let Some(uri) = Uri::from_file_path(&file).ok()
                                                && file.starts_with(&path)
                                                && let Ok(content) = fs.load(&file).await
                                            {
                                                index.lock().index_file(uri, &content);
                                            }
                                        }
                                    }
                                })
                                .detach();
                            }
                        },
                    );

                    server.set_request_handler::<lsp::request::GotoDefinition, _, _>({
                        let index = index.clone();
                        move |params, _cx| {
                            let result = index.lock().get_definitions(
                                params.text_document_position_params.text_document.uri,
                                params.text_document_position_params.position,
                            );
                            async move { Ok(result) }
                        }
                    });
                }
            })),
        },
    )
}

struct DefinitionIndex {
    language: Arc<Language>,
    definitions: HashMap<String, Vec<lsp::Location>>,
    files: HashMap<Uri, FileEntry>,
}

#[derive(Debug)]
struct FileEntry {
    contents: String,
    is_open_in_buffer: bool,
}

impl DefinitionIndex {
    fn new(language: Arc<Language>) -> Self {
        Self {
            language,
            definitions: HashMap::default(),
            files: HashMap::default(),
        }
    }

    fn remove_definitions_for_file(&mut self, uri: &Uri) {
        self.definitions.retain(|_, locations| {
            locations.retain(|loc| &loc.uri != uri);
            !locations.is_empty()
        });
        self.files.remove(uri);
    }

    fn open_buffer(&mut self, uri: Uri, content: &str) {
        self.index_file_inner(uri, content, true);
    }

    fn mark_buffer_closed(&mut self, uri: &Uri) {
        if let Some(entry) = self.files.get_mut(uri) {
            entry.is_open_in_buffer = false;
        }
    }

    fn is_buffer_open(&self, uri: &Uri) -> bool {
        self.files
            .get(uri)
            .map(|entry| entry.is_open_in_buffer)
            .unwrap_or(false)
    }

    fn index_file(&mut self, uri: Uri, content: &str) {
        self.index_file_inner(uri, content, false);
    }

    fn index_file_inner(&mut self, uri: Uri, content: &str, is_open_in_buffer: bool) -> Option<()> {
        self.remove_definitions_for_file(&uri);
        let grammar = self.language.grammar()?;
        let outline_config = grammar.outline_config.as_ref()?;
        let mut parser = Parser::new();
        parser.set_language(&grammar.ts_language).ok()?;
        let tree = parser.parse(content, None)?;
        let declarations = extract_declarations_from_tree(&tree, content, outline_config);
        for (name, byte_range) in declarations {
            let range = byte_range_to_lsp_range(content, byte_range);
            let location = lsp::Location {
                uri: uri.clone(),
                range,
            };
            self.definitions
                .entry(name)
                .or_insert_with(Vec::new)
                .push(location);
        }
        self.files.insert(
            uri,
            FileEntry {
                contents: content.to_string(),
                is_open_in_buffer,
            },
        );

        Some(())
    }

    fn get_definitions(
        &mut self,
        uri: Uri,
        position: lsp::Position,
    ) -> Option<lsp::GotoDefinitionResponse> {
        let entry = self.files.get(&uri)?;
        let name = word_at_position(&entry.contents, position)?;
        let locations = self.definitions.get(name).cloned()?;
        Some(lsp::GotoDefinitionResponse::Array(locations))
    }
}

fn extract_declarations_from_tree(
    tree: &Tree,
    content: &str,
    outline_config: &language::OutlineConfig,
) -> Vec<(String, Range<usize>)> {
    let mut cursor = QueryCursor::new();
    let mut declarations = Vec::new();
    let mut matches = cursor.matches(&outline_config.query, tree.root_node(), content.as_bytes());
    while let Some(query_match) = matches.next() {
        let mut name_range: Option<Range<usize>> = None;
        let mut has_item_range = false;

        for capture in query_match.captures {
            let range = capture.node.byte_range();
            if capture.index == outline_config.name_capture_ix {
                name_range = Some(range);
            } else if capture.index == outline_config.item_capture_ix {
                has_item_range = true;
            }
        }

        if let Some(name_range) = name_range
            && has_item_range
        {
            let name = content[name_range.clone()].to_string();
            if declarations.iter().any(|(n, _)| n == &name) {
                continue;
            }
            declarations.push((name, name_range));
        }
    }
    declarations
}

fn byte_range_to_lsp_range(content: &str, byte_range: Range<usize>) -> lsp::Range {
    let start = byte_offset_to_position(content, byte_range.start);
    let end = byte_offset_to_position(content, byte_range.end);
    lsp::Range { start, end }
}

fn byte_offset_to_position(content: &str, offset: usize) -> lsp::Position {
    let mut line = 0;
    let mut character = 0;
    let mut current_offset = 0;
    for ch in content.chars() {
        if current_offset >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
        current_offset += ch.len_utf8();
    }
    lsp::Position { line, character }
}

fn word_at_position(content: &str, position: lsp::Position) -> Option<&str> {
    let mut lines = content.lines();
    let line = lines.nth(position.line as usize)?;
    let column = position.character as usize;
    if column > line.len() {
        return None;
    }
    let start = line[..column]
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    let end = line[column..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + column)
        .unwrap_or(line.len());
    Some(&line[start..end]).filter(|word| !word.is_empty())
}
