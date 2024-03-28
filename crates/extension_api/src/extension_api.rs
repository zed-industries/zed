pub use wit::*;
pub type Result<T, E = String> = core::result::Result<T, E>;

pub trait Extension: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn language_server_command(
        &mut self,
        config: LanguageServerConfig,
        worktree: &Worktree,
    ) -> Result<Command>;

    fn language_server_initialization_options(
        &mut self,
        _config: LanguageServerConfig,
        _worktree: &Worktree,
    ) -> Result<Option<String>> {
        Ok(None)
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
        path: "./wit/0.0.4",
    });
}

wit::export!(Component);

struct Component;

impl wit::Guest for Component {
    fn language_server_command(
        config: wit::LanguageServerConfig,
        worktree: &wit::Worktree,
    ) -> Result<wit::Command> {
        extension().language_server_command(config, worktree)
    }

    fn language_server_initialization_options(
        config: LanguageServerConfig,
        worktree: &Worktree,
    ) -> Result<Option<String>, String> {
        extension().language_server_initialization_options(config, worktree)
    }
}
