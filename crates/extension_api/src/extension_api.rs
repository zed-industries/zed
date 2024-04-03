use core::fmt;

use wit::*;
pub use wit::{
    current_platform, download_file, latest_github_release, make_file_executable, node_binary_path,
    npm_install_package, npm_package_installed_version, npm_package_latest_version,
    zed::extension::lsp, Architecture, CodeLabel, CodeLabelSpan, CodeLabelSpanLiteral, Command,
    Completion, DownloadedFileType, EnvVars, GithubRelease, GithubReleaseAsset,
    GithubReleaseOptions, Guest, LanguageServerInstallationStatus, Os, Range, Worktree,
};

pub type Result<T, E = String> = core::result::Result<T, E>;

pub fn set_language_server_installation_status(
    language_server_id: &LanguageServerId,
    status: &LanguageServerInstallationStatus,
) {
    wit::set_language_server_installation_status(&language_server_id.0, status)
}

pub trait Extension: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command>;

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        _completion: Completion,
    ) -> Option<CodeLabel> {
        None
    }
}

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
    wit_bindgen::generate!({
        skip: ["init-extension"],
        path: "./wit/since_v0.0.6",
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
        extension().language_server_initialization_options(&language_server_id, worktree)
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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct LanguageServerId(String);

impl fmt::Display for LanguageServerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
