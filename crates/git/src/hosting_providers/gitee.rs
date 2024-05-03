use url::Url;

use crate::hosting_provider::GitHostingProvider;

pub struct Gitee;

impl GitHostingProvider for Gitee {
    fn name(&self) -> String {
        "Gitee".to_string()
    }

    fn base_url(&self) -> Url {
        Url::parse("https://gitee.com").unwrap()
    }

    fn supports_avatars(&self) -> bool {
        false
    }

    fn format_line_number(&self, line: u32) -> String {
        format!("L{line}")
    }

    fn format_line_numbers(&self, start_line: u32, end_line: u32) -> String {
        format!("L{start_line}-{end_line}")
    }
}
