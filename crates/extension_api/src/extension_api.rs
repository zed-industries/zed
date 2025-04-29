//! The Zed Rust Extension API allows you write extensions for [Zed](https://zed.dev/) in Rust.

pub mod http_client;
pub mod process;
pub mod settings;

use core::fmt;

use wit::*;

pub use serde_json;

// WIT re-exports.
//
// We explicitly enumerate the symbols we want to re-export, as there are some
// that we may want to shadow to provide a cleaner Rust API.
pub use wit::{
    CodeLabel, CodeLabelSpan, CodeLabelSpanLiteral, Command, DownloadedFileType, EnvVars,
    KeyValueStore, LanguageServerInstallationStatus, Project, Range, Worktree, download_file,
    make_file_executable,
    zed::extension::context_server::ContextServerConfiguration,
    zed::extension::github::{
        GithubRelease, GithubReleaseAsset, GithubReleaseOptions, github_release_by_tag_name,
        latest_github_release,
    },
    zed::extension::nodejs::{
        node_binary_path, npm_install_package, npm_package_installed_version,
        npm_package_latest_version,
    },
    zed::extension::platform::{Architecture, Os, current_platform},
    zed::extension::slash_command::{
        SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, SlashCommandOutputSection,
    },
};

// Undocumented WIT re-exports.
//
// These are symbols that need to be public for the purposes of implementing
// the extension host, but aren't relevant to extension authors.
#[doc(hidden)]
pub use wit::Guest;

/// Constructs for interacting with language servers over the
/// Language Server Protocol (LSP).
pub mod lsp {
    pub use crate::wit::zed::extension::lsp::{
        Completion, CompletionKind, InsertTextFormat, Symbol, SymbolKind,
    };
}

/// A result returned from a Zed extension.
pub type Result<T, E = String> = core::result::Result<T, E>;

/// Updates the installation status for the given language server.
pub fn set_language_server_installation_status(
    language_server_id: &LanguageServerId,
    status: &LanguageServerInstallationStatus,
) {
    wit::set_language_server_installation_status(&language_server_id.0, status)
}

/// A Zed extension.
pub trait Extension: Send + Sync {
    /// Returns a new instance of the extension.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the command used to start the language server for the specified
    /// language.
    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        Err("`language_server_command` not implemented".to_string())
    }

    /// Returns the initialization options to pass to the specified language server.
    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }

    /// Returns the workspace configuration options to pass to the language server.
    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }

    /// Returns the initialization options to pass to the other language server.
    fn language_server_additional_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _target_language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }

    /// Returns the workspace configuration options to pass to the other language server.
    fn language_server_additional_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _target_language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }

    /// Returns the label for the given completion.
    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        _completion: Completion,
    ) -> Option<CodeLabel> {
        None
    }

    /// Returns the label for the given symbol.
    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        _symbol: Symbol,
    ) -> Option<CodeLabel> {
        None
    }

    /// Returns the completions that should be shown when completing the provided slash command with the given query.
    fn complete_slash_command_argument(
        &self,
        _command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        Ok(Vec::new())
    }

    /// Returns the output from running the provided slash command.
    fn run_slash_command(
        &self,
        _command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        Err("`run_slash_command` not implemented".to_string())
    }

    /// Returns the command used to start a context server.
    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        Err("`context_server_command` not implemented".to_string())
    }

    ///
    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(None)
    }

    /// Returns a list of package names as suggestions to be included in the
    /// search results of the `/docs` slash command.
    ///
    /// This can be used to provide completions for known packages (e.g., from the
    /// local project or a registry) before a package has been indexed.
    fn suggest_docs_packages(&self, _provider: String) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    /// Indexes the docs for the specified package.
    fn index_docs(
        &self,
        _provider: String,
        _package: String,
        _database: &KeyValueStore,
    ) -> Result<(), String> {
        Err("`index_docs` not implemented".to_string())
    }
}

/// Registers the provided type as a Zed extension.
///
/// The type must implement the [`Extension`] trait.
#[macro_export]
macro_rules! register_extension {
    ($extension_type:ty) => {
        #[unsafe(export_name = "init-extension")]
        pub extern "C" fn __init_extension() {
            std::env::set_current_dir(std::env::var("PWD").unwrap()).unwrap();
            zed_extension_api::register_extension(|| {
                Box::new(<$extension_type as zed_extension_api::Extension>::new())
            });
        }
    };
}

#[doc(hidden)]
pub fn register_extension(build_extension: fn() -> Box<dyn Extension>) {
    unsafe { EXTENSION = Some((build_extension)()) }
}

fn extension() -> &'static mut dyn Extension {
    #[expect(static_mut_refs)]
    unsafe {
        EXTENSION.as_deref_mut().unwrap()
    }
}

static mut EXTENSION: Option<Box<dyn Extension>> = None;

#[cfg(target_arch = "wasm32")]
#[unsafe(link_section = "zed:api-version")]
#[doc(hidden)]
pub static ZED_API_VERSION: [u8; 6] = *include_bytes!(concat!(env!("OUT_DIR"), "/version_bytes"));

mod wit {

    wit_bindgen::generate!({
        skip: ["init-extension"],
        path: "./wit/since_v0.5.0",
    });
}

wit::export!(Component);

struct Component;

impl wit::Guest for Component {
    fn language_server_command(
        language_server_id: String,
        worktree: &wit::Worktree,
    ) -> Result<wit::Command> {
        let language_server_id = LanguageServerId(language_server_id);
        extension().language_server_command(&language_server_id, worktree)
    }

    fn language_server_initialization_options(
        language_server_id: String,
        worktree: &Worktree,
    ) -> Result<Option<String>, String> {
        let language_server_id = LanguageServerId(language_server_id);
        Ok(extension()
            .language_server_initialization_options(&language_server_id, worktree)?
            .and_then(|value| serde_json::to_string(&value).ok()))
    }

    fn language_server_workspace_configuration(
        language_server_id: String,
        worktree: &Worktree,
    ) -> Result<Option<String>, String> {
        let language_server_id = LanguageServerId(language_server_id);
        Ok(extension()
            .language_server_workspace_configuration(&language_server_id, worktree)?
            .and_then(|value| serde_json::to_string(&value).ok()))
    }

    fn language_server_additional_initialization_options(
        language_server_id: String,
        target_language_server_id: String,
        worktree: &Worktree,
    ) -> Result<Option<String>, String> {
        let language_server_id = LanguageServerId(language_server_id);
        let target_language_server_id = LanguageServerId(target_language_server_id);
        Ok(extension()
            .language_server_additional_initialization_options(
                &language_server_id,
                &target_language_server_id,
                worktree,
            )?
            .and_then(|value| serde_json::to_string(&value).ok()))
    }

    fn language_server_additional_workspace_configuration(
        language_server_id: String,
        target_language_server_id: String,
        worktree: &Worktree,
    ) -> Result<Option<String>, String> {
        let language_server_id = LanguageServerId(language_server_id);
        let target_language_server_id = LanguageServerId(target_language_server_id);
        Ok(extension()
            .language_server_additional_workspace_configuration(
                &language_server_id,
                &target_language_server_id,
                worktree,
            )?
            .and_then(|value| serde_json::to_string(&value).ok()))
    }

    fn labels_for_completions(
        language_server_id: String,
        completions: Vec<Completion>,
    ) -> Result<Vec<Option<CodeLabel>>, String> {
        let language_server_id = LanguageServerId(language_server_id);
        let mut labels = Vec::new();
        for (ix, completion) in completions.into_iter().enumerate() {
            let label = extension().label_for_completion(&language_server_id, completion);
            if let Some(label) = label {
                labels.resize(ix + 1, None);
                *labels.last_mut().unwrap() = Some(label);
            }
        }
        Ok(labels)
    }

    fn labels_for_symbols(
        language_server_id: String,
        symbols: Vec<Symbol>,
    ) -> Result<Vec<Option<CodeLabel>>, String> {
        let language_server_id = LanguageServerId(language_server_id);
        let mut labels = Vec::new();
        for (ix, symbol) in symbols.into_iter().enumerate() {
            let label = extension().label_for_symbol(&language_server_id, symbol);
            if let Some(label) = label {
                labels.resize(ix + 1, None);
                *labels.last_mut().unwrap() = Some(label);
            }
        }
        Ok(labels)
    }

    fn complete_slash_command_argument(
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        extension().complete_slash_command_argument(command, args)
    }

    fn run_slash_command(
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        extension().run_slash_command(command, args, worktree)
    }

    fn context_server_command(
        context_server_id: String,
        project: &Project,
    ) -> Result<wit::Command> {
        let context_server_id = ContextServerId(context_server_id);
        extension().context_server_command(&context_server_id, project)
    }

    fn context_server_configuration(
        context_server_id: String,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>, String> {
        let context_server_id = ContextServerId(context_server_id);
        extension().context_server_configuration(&context_server_id, project)
    }

    fn suggest_docs_packages(provider: String) -> Result<Vec<String>, String> {
        extension().suggest_docs_packages(provider)
    }

    fn index_docs(
        provider: String,
        package: String,
        database: &KeyValueStore,
    ) -> Result<(), String> {
        extension().index_docs(provider, package, database)
    }
}

/// The ID of a language server.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct LanguageServerId(String);

impl AsRef<str> for LanguageServerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LanguageServerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The ID of a context server.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ContextServerId(String);

impl AsRef<str> for ContextServerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContextServerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CodeLabelSpan {
    /// Returns a [`CodeLabelSpan::CodeRange`].
    pub fn code_range(range: impl Into<wit::Range>) -> Self {
        Self::CodeRange(range.into())
    }

    /// Returns a [`CodeLabelSpan::Literal`].
    pub fn literal(text: impl Into<String>, highlight_name: Option<String>) -> Self {
        Self::Literal(CodeLabelSpanLiteral {
            text: text.into(),
            highlight_name,
        })
    }
}

impl From<std::ops::Range<u32>> for wit::Range {
    fn from(value: std::ops::Range<u32>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<std::ops::Range<usize>> for wit::Range {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start as u32,
            end: value.end as u32,
        }
    }
}
