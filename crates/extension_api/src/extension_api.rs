//! The Zed Rust Extension API allows you write extensions for [Zed](https://zed.dev/) in Rust.

/// Provides access to Zed settings.
pub mod settings;

use core::fmt;

use wit::*;

pub use serde_json;

// WIT re-exports.
//
// We explicitly enumerate the symbols we want to re-export, as there are some
// that we may want to shadow to provide a cleaner Rust API.
pub use wit::{
    download_file, make_file_executable,
    zed::extension::github::{
        github_release_by_tag_name, latest_github_release, GithubRelease, GithubReleaseAsset,
        GithubReleaseOptions,
    },
    zed::extension::nodejs::{
        node_binary_path, npm_install_package, npm_package_installed_version,
        npm_package_latest_version,
    },
    zed::extension::platform::{current_platform, Architecture, Os},
    zed::extension::slash_command::SlashCommand,
    CodeLabel, CodeLabelSpan, CodeLabelSpanLiteral, Command, DownloadedFileType, EnvVars,
    LanguageServerInstallationStatus, Range, Worktree,
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
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command>;

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

    /// Runs the given slash command.
    fn run_slash_command(
        &self,
        _command: SlashCommand,
        _argument: Option<String>,
        _worktree: &Worktree,
    ) -> Result<Option<String>, String> {
        Ok(None)
    }
}

/// Registers the provided type as a Zed extension.
///
/// The type must implement the [`Extension`] trait.
#[macro_export]
macro_rules! register_extension {
    ($extension_type:ty) => {
        #[export_name = "init-extension"]
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
    unsafe { EXTENSION.as_deref_mut().unwrap() }
}

static mut EXTENSION: Option<Box<dyn Extension>> = None;

#[cfg(target_arch = "wasm32")]
#[link_section = "zed:api-version"]
#[doc(hidden)]
pub static ZED_API_VERSION: [u8; 6] = *include_bytes!(concat!(env!("OUT_DIR"), "/version_bytes"));

mod wit {
    #![allow(clippy::too_many_arguments)]

    wit_bindgen::generate!({
        skip: ["init-extension"],
        path: "./wit/since_v0.0.7",
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

    fn run_slash_command(
        command: SlashCommand,
        argument: Option<String>,
        worktree: &Worktree,
    ) -> Result<Option<String>, String> {
        extension().run_slash_command(command, argument, worktree)
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
