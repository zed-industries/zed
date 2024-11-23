//! Provides access to Zed settings.

#[path = "../wit/since_v0.2.0/settings.rs"]
mod types;

use crate::{wit, Os, Project, Result, SettingsLocation, Worktree};
use serde_json;
pub use types::*;

/// The result of searching for a language server binary.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum LanguageServerPath {
    /// A binary was located.
    Command(wit::Command),
    /// A cached binary was located.
    CachedCommand(wit::Command),
    /// A binary was not located and it should be automatically downloaded.
    AutomaticDownload,
    /// No binary was located and should be considered unavailable.
    None,
}

impl From<LanguageServerPath> for Option<wit::Command> {
    #[inline]
    fn from(path: LanguageServerPath) -> Self {
        match path {
            LanguageServerPath::Command(command) => Some(command),
            _ => None,
        }
    }
}

impl LanguageSettings {
    /// Returns the [`LanguageSettings`] for the given language.
    pub fn for_worktree(language: Option<&str>, worktree: &Worktree) -> Result<Self> {
        get_settings("language", language, Some(worktree.id()))
    }
}

impl LspSettings {
    /// Returns the [`LspSettings`] for the given language server.
    pub fn for_worktree(language_server_name: &str, worktree: &Worktree) -> Result<Self> {
        get_settings("lsp", Some(language_server_name), Some(worktree.id()))
    }
}

/// Information about an LSP server binary.
#[derive(Debug, Default)]
pub struct LspServerInfo {
    /// The name of the binary to search for in the PATH.
    pub binary_name: Option<String>,
    /// The path to the cached binary from a previous automatic download.
    pub cached_binary_path: Option<String>,
    /// The default arguments to pass to the binary if both the binary and arguments are not set.
    pub default_arguments: Vec<String>,
}

impl Worktree {
    /// Creates a [`wit::Command`] from the given [`CommandSettings`] and path. The environment variables are merged
    /// with the shell environment.
    pub fn create_command(&self, command: &CommandSettings, path: String) -> wit::Command {
        let (platform, _) = crate::current_platform();
        let env = match platform {
            Os::Mac | Os::Linux => self.shell_env(),
            Os::Windows => Default::default(),
        };

        let env = env
            .into_iter()
            .chain(
                command
                    .env
                    .iter()
                    .flatten()
                    .map(|(k, v)| (k.clone(), v.clone())),
            )
            .collect();

        wit::Command {
            command: path,
            args: command.arguments.clone().unwrap_or_default(),
            env,
        }
    }

    /// Searches for a language server binary.
    ///
    /// This function will search for the binary in the PATH, then check the configured binary path, and finally check
    /// the cached binary path.
    pub fn find_language_server(
        &self,
        settings: &LspSettings,
        info: &LspServerInfo,
    ) -> LanguageServerPath {
        let mut path = None;
        if let Some(binary_name) = settings
            .allow_path_search
            .unwrap_or(true)
            .then_some(info.binary_name.as_deref())
            .flatten()
        {
            path = check_exists(self.which(binary_name));
        }

        path = path.or_else(|| check_exists(settings.binary.as_ref().and_then(|v| v.path.clone())));

        let is_cached = path.is_none();
        if settings.allow_automatic_download.unwrap_or(true) {
            path = path.or_else(|| check_exists(info.cached_binary_path.clone()));
        }

        let default = CommandSettings::default();
        let mut command = path.map(|command| {
            self.create_command(settings.binary.as_ref().unwrap_or(&default), command)
        });

        if settings
            .binary
            .as_ref()
            .map_or(true, |v| v.path.is_none() && v.arguments.is_none())
        {
            if let Some(ref mut command) = command {
                command.args.extend(info.default_arguments.iter().cloned());
            }
        }

        match command {
            Some(command) if is_cached => LanguageServerPath::CachedCommand(command),
            Some(command) => LanguageServerPath::Command(command),
            None if settings.allow_automatic_download.unwrap_or(true) => {
                LanguageServerPath::AutomaticDownload
            }
            _ => LanguageServerPath::None,
        }
    }
}

fn check_exists(mut path: Option<String>) -> Option<String> {
    if !path.as_deref().map_or(true, |path| {
        std::fs::metadata(path).map_or(false, |stat| stat.is_file())
    }) {
        path = None;
    }
    path
}

impl ContextServerSettings {
    /// Returns the [`ContextServerSettings`] for the given context server.
    pub fn for_project(context_server_id: &str, project: &Project) -> Result<Self> {
        let global_setting: Self = get_settings("context_servers", Some(context_server_id), None)?;

        for worktree_id in project.worktree_ids() {
            let settings = get_settings(
                "context_servers",
                Some(context_server_id),
                Some(worktree_id),
            )?;
            if settings != global_setting {
                return Ok(settings);
            }
        }

        Ok(global_setting)
    }
}

fn get_settings<T: serde::de::DeserializeOwned>(
    settings_type: &str,
    settings_name: Option<&str>,
    worktree_id: Option<u64>,
) -> Result<T> {
    let location = worktree_id.map(|worktree_id| SettingsLocation {
        worktree_id,
        path: String::new(),
    });
    let settings_json = wit::get_settings(location.as_ref(), settings_type, settings_name)?;
    let settings: T = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;
    Ok(settings)
}
