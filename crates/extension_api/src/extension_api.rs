pub struct Guest;
pub use wit::*;

pub type Result<T, E = String> = core::result::Result<T, E>;

pub trait Extension: Send + Sync {
    fn language_server_command(
        &self,
        config: wit::LanguageServerConfig,
        worktree: &wit::Worktree,
    ) -> Result<Command>;
}

#[macro_export]
macro_rules! register_extension {
    ($extension:path) => {
        #[export_name = "init-extension"]
        pub extern "C" fn __init_extension() {
            zed_extension_api::register_extension(&$extension);
        }
    };
}

#[doc(hidden)]
pub fn register_extension(extension: &'static dyn Extension) {
    unsafe { EXTENSION = Some(extension) };
}

fn extension() -> &'static dyn Extension {
    unsafe { EXTENSION.unwrap() }
}

static mut EXTENSION: Option<&'static dyn Extension> = None;

#[link_section = "zed:api-version"]
#[doc(hidden)]
pub static ZED_API_VERSION: [u8; 6] = *include_bytes!(concat!(env!("OUT_DIR"), "/version_bytes"));

mod wit {
    wit_bindgen::generate!({
        exports: { world: super::Component },
        skip: ["init-extension"]
    });
}

struct Component;

impl wit::Guest for Component {
    fn language_server_command(
        config: wit::LanguageServerConfig,
        worktree: &wit::Worktree,
    ) -> Result<wit::Command> {
        extension().language_server_command(config, worktree)
    }
}
