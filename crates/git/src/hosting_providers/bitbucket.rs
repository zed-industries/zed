use url::Url;

use crate::hosting_provider::GitHostingProvider;

pub struct Bitbucket;

impl GitHostingProvider for Bitbucket {
    fn name(&self) -> String {
        "Bitbucket".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://bitbucket.org").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        false
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("lines-{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("lines-{start_line}:{end_line}")
    }
}
