use url::Url;

use crate::hosting_provider::GitHostingProvider;

pub struct Codeberg;

impl GitHostingProvider for Codeberg {
    fn name(&self) -> String {
        "Codeberg".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://codeberg.org").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        true
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-L{end_line}")
    }
}
