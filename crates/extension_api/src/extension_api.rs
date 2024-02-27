use std::sync::OnceLock;

pub struct Guest;
pub use wit::*;

pub type Result<T, E = String> = core::result::Result<T, E>;

pub trait Extension: Send + Sync {
    fn get_language_server_command(
        &self,
        config: wit::LanguageServerConfig,
        worktree: &wit::Worktree,
    ) -> Result<Command>;
}

#[macro_export]
macro_rules! register_extension {
    ($extension:path) => {
        pub extern "C" fn __zed_extension_init() {
            zed_extension_api::register_extension($extension);
        }
    };
}

#[doc(hidden)]
pub fn register_extension(extension: impl Extension + 'static) {
    EXTENSION.get_or_init(|| Box::new(extension));
}

static EXTENSION: OnceLock<Box<dyn Extension>> = OnceLock::new();

mod wit {
    wit_bindgen::generate!({
        exports: { world: super::Component },
    });
}

struct Component;

impl wit::Guest for Component {
    fn get_language_server_command(
        config: wit::LanguageServerConfig,
        worktree: &wit::Worktree,
    ) -> Result<wit::Command> {
        EXTENSION
            .get()
            .unwrap()
            .get_language_server_command(config, worktree)
    }
}
