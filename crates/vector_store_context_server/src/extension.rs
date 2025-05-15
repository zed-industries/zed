use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use extension::{Extension, ExtensionManifest, Command, ProjectDelegate, ContextServerConfiguration};
use extension::{SchemaVersion, LibManifestEntry, ExtensionLibraryKind, ContextServerManifestEntry};
use std::path::Path;
use serde_json::Value;

pub struct VectorStoreExtension {
    manifest_name: String,
    work_dir: Arc<Path>,
}

impl VectorStoreExtension {
    pub fn new(_manifest: Arc<Value>, work_dir: Arc<Path>) -> Self {
        Self {
            manifest_name: "vector-store-context-server".to_string(),
            work_dir,
        }
    }
}

#[async_trait]
impl Extension for VectorStoreExtension {
    fn manifest(&self) -> Arc<ExtensionManifest> {
        // Create a minimal static manifest
        Arc::new(ExtensionManifest {
            id: Arc::from("vector-store-context-server"),
            name: self.manifest_name.clone(),
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
                ContextServerManifestEntry {},
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

    async fn language_server_command(
        &self,
        _language_server_id: lsp::LanguageServerName,
        _language_name: language::LanguageName,
        _worktree: Arc<dyn extension::WorktreeDelegate>,
    ) -> Result<Command> {
        Err(anyhow!("Not supported"))
    }

    async fn language_server_initialization_options(
        &self,
        _language_server_id: lsp::LanguageServerName,
        _language_name: language::LanguageName,
        _worktree: Arc<dyn extension::WorktreeDelegate>,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    async fn language_server_workspace_configuration(
        &self,
        _language_server_id: lsp::LanguageServerName,
        _worktree: Arc<dyn extension::WorktreeDelegate>,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    async fn language_server_additional_initialization_options(
        &self,
        _language_server_id: lsp::LanguageServerName,
        _target_language_server_id: lsp::LanguageServerName,
        _worktree: Arc<dyn extension::WorktreeDelegate>,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    async fn language_server_additional_workspace_configuration(
        &self,
        _language_server_id: lsp::LanguageServerName,
        _target_language_server_id: lsp::LanguageServerName,
        _worktree: Arc<dyn extension::WorktreeDelegate>,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    async fn labels_for_completions(
        &self,
        _language_server_id: lsp::LanguageServerName,
        _completions: Vec<extension::Completion>,
    ) -> Result<Vec<Option<extension::CodeLabel>>> {
        Ok(Vec::new())
    }

    async fn labels_for_symbols(
        &self,
        _language_server_id: lsp::LanguageServerName,
        _symbols: Vec<extension::Symbol>,
    ) -> Result<Vec<Option<extension::CodeLabel>>> {
        Ok(Vec::new())
    }

    async fn complete_slash_command_argument(
        &self,
        _command: extension::SlashCommand,
        _arguments: Vec<String>,
    ) -> Result<Vec<extension::SlashCommandArgumentCompletion>> {
        Ok(Vec::new())
    }

    async fn run_slash_command(
        &self,
        _command: extension::SlashCommand,
        _arguments: Vec<String>,
        _worktree: Option<Arc<dyn extension::WorktreeDelegate>>,
    ) -> Result<extension::SlashCommandOutput> {
        Err(anyhow!("Not supported"))
    }

    async fn context_server_command(
        &self,
        context_server_id: Arc<str>,
        _project: Arc<dyn ProjectDelegate>,
    ) -> Result<Command> {
        // Make sure this is our context server
        if context_server_id.as_ref() != "vector-store" {
            return Err(anyhow!("Unknown context server ID: {}", context_server_id));
        }

        // Get the path to our executable
        let mut executable_path = std::env::current_exe()?;
        executable_path.pop(); // Remove the executable name
        executable_path.push("vector_store_context_server");

        // Prepare the command
        Ok(Command {
            command: executable_path.to_string_lossy().to_string(),
            args: vec![],
            env: Vec::new(),
        })
    }

    async fn context_server_configuration(
        &self,
        context_server_id: Arc<str>,
        _project: Arc<dyn ProjectDelegate>,
    ) -> Result<Option<ContextServerConfiguration>> {
        if context_server_id.as_ref() != "vector-store" {
            return Err(anyhow!("Unknown context server ID: {}", context_server_id));
        }

        Ok(Some(ContextServerConfiguration {
            installation_instructions: "The vector store context server is built into Zed".to_string(),
            settings_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "database_path": {
                        "type": "string",
                        "description": "Path for storing the vector databases",
                        "default": "~/.config/zed/vector_stores"
                    }
                }
            }),
            default_settings: r#"{ "database_path": "~/.config/zed/vector_stores" }"#.to_string(),
        }))
    }

    async fn suggest_docs_packages(&self, _provider: Arc<str>) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    async fn index_docs(
        &self,
        _provider: Arc<str>,
        _package_name: Arc<str>,
        _kv_store: Arc<dyn extension::KeyValueStoreDelegate>,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_dap_binary(
        &self,
        _dap_name: Arc<str>,
        _config: extension::DebugTaskDefinition,
        _user_installed_path: Option<PathBuf>,
    ) -> Result<extension::DebugAdapterBinary> {
        Err(anyhow!("Not supported"))
    }
} 