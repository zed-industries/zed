use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use extension::{Extension, ExtensionManifest, Command, ProjectDelegate, ContextServerConfiguration};
use extension::{SchemaVersion, LibManifestEntry, ExtensionLibraryKind, ContextServerManifestEntry};
use std::path::Path;
use serde_json::Value;

pub struct VectorStoreExtension {
    work_dir: Arc<Path>,
}

impl VectorStoreExtension {
    pub fn new(manifest: Arc<Value>, work_dir: Arc<Path>) -> Self {
        Self {
            work_dir,
        }
    }
}

#[async_trait]
impl Extension for VectorStoreExtension {
    fn id(&self) -> String {
        "zed.vector-store-context-server".to_string()
    }

    fn schema_version(&self) -> SchemaVersion {
        SchemaVersion(0)
    }

    fn register_commands(&self) -> Vec<Command> {
        vec![
            Command {
                name: "vector-create".to_string(),
                title: "Create Vector Store".to_string(),
                description: Some("Create a new vector store".to_string()),
                command_group: None,
                palette_kind: None,
                command_kind: None,
            },
            Command {
                name: "vector-add".to_string(),
                title: "Add Vector".to_string(),
                description: Some("Add vectors to a store".to_string()),
                command_group: None,
                palette_kind: None,
                command_kind: None,
            },
            Command {
                name: "vector-search".to_string(),
                title: "Search Vectors".to_string(),
                description: Some("Search for similar vectors in the vector store".to_string()),
                command_group: None,
                palette_kind: None,
                command_kind: None,
            },
            Command {
                name: "vector-info".to_string(),
                title: "Vector Stores Info".to_string(),
                description: Some("Get information about vector stores".to_string()),
                command_group: None,
                palette_kind: None,
                command_kind: None,
            },
        ]
    }

    fn context_server_configs(&self) -> Vec<ContextServerConfiguration> {
        vec![ContextServerConfiguration {
            name: "vector-store".to_string(),
            description: "A context server for vector storage and semantic search capabilities".to_string(),
            capabilities: Some(vec![
                "SlashCommands".to_string(),
                "Tools".to_string(),
            ]),
            slash_commands: Some(vec![
                "vector-search".to_string(),
                "vector-info".to_string(),
                "vector-create".to_string(),
                "vector-add".to_string(),
            ]),
        }]
    }

    async fn context_server_binary(
        &self,
        name: &str,
        _project_root: Option<PathBuf>,
    ) -> Result<ContextServerManifestEntry> {
        if name != "vector-store" {
            return Err(anyhow!("Unknown context server: {}", name));
        }

        Ok(ContextServerManifestEntry::Library(LibManifestEntry {
            name: "vector_store_context_server".to_string(),
            kind: ExtensionLibraryKind::Rust,
            functions: vec!["create_extension".to_string()],
        }))
    }

    async fn handle_command(
        &self,
        _command_name: &str,
        _args: Value,
        _project_root: Option<PathBuf>,
        _project_delegate: Option<Box<dyn ProjectDelegate>>,
    ) -> Result<Value> {
        Err(anyhow!("Command not handled by extension directly, should be handled by context server"))
    }

    fn manifest(&self) -> Arc<ExtensionManifest> {
        // Create a minimal static manifest
        Arc::new(ExtensionManifest {
            id: Arc::from("vector-store-context-server"),
            name: "vector-store-context-server".to_string(),
            version: Arc::from("0.1.0"),
            schema_version: SchemaVersion(0),
            description: Some("Provides vector storage and semantic search capabilities".to_string()),
            repository: None,
            authors: Vec::new(),
            lib: LibManifestEntry {
                kind: Some(ExtensionLibraryKind::Rust),
                version: None,
            },
            themes: Vec::new(),
            icon_themes: Vec::new(),
            languages: Vec::new(),
            grammars: Default::default(),
            language_servers: Default::default(),
            context_servers: [(
                Arc::from("vector-store"),
                ContextServerManifestEntry::Library(LibManifestEntry {
                    name: "vector_store_context_server".to_string(),
                    kind: ExtensionLibraryKind::Rust,
                    functions: vec!["create_extension".to_string()],
                }),
            )]
            .into_iter()
            .collect(),
            slash_commands: Default::default(),
            indexed_docs_providers: Default::default(),
            snippets: None,
            capabilities: Vec::new(),
        })
    }

    fn work_dir(&self) -> Arc<Path> {
        self.work_dir.clone()
    }

    // The following methods are required by the Extension trait
    // but we provide minimal implementations
    
    async fn language_server_command(
        &self,
        _language_server_id: Arc<str>,
        _language_name: Arc<str>,
        _worktree: Arc<dyn extension::WorktreeDelegate>,
    ) -> Result<Command> {
        Err(anyhow!("Not supported"))
    }

    async fn language_server_initialization_options(
        &self,
        _language_server_id: Arc<str>,
        _language_name: Arc<str>,
        _worktree: Arc<dyn extension::WorktreeDelegate>,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    async fn context_server_configuration(
        &self,
        context_server_id: Arc<str>,
        _project: Arc<dyn ProjectDelegate>,
    ) -> Result<Option<String>> {
        if context_server_id.as_ref() != "vector-store" {
            return Err(anyhow!("Unknown context server: {}", context_server_id));
        }
        Ok(None)
    }
} 