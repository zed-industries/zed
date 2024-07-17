use zed_extension_api::{self as zed, Result};

pub struct Rubocop {}

impl Rubocop {
    pub const LANGUAGE_SERVER_ID: &'static str = "rubocop";

    pub fn new() -> Self {
        Self {}
    }

    pub fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let path = worktree.which("rubocop").ok_or_else(|| {
            "rubocop must be installed manually. Install it with `gem install rubocop` or specify the 'binary' path to it via local settings.".to_string()
        })?;

        Ok(path)
    }
}
