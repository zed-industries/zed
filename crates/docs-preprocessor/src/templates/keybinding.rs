use crate::PreprocessorContext;
use regex::Regex;
use std::collections::HashMap;

use super::Template;

pub struct KeybindingTemplate;

impl KeybindingTemplate {
    pub fn new() -> Self {
        KeybindingTemplate
    }
}

impl Template for KeybindingTemplate {
    fn key(&self) -> &'static str {
        "keybinding"
    }

    fn regex(&self) -> Regex {
        Regex::new(&format!(
            "\\{{\\s*#{}\\s+name=\"(.*?)\"\\s*\\}}",
            self.key()
        ))
        .unwrap()
    }

    fn parse_args(&self, args: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), args.to_string());
        map
    }

    fn render(&self, context: &PreprocessorContext, args: &HashMap<String, String>) -> String {
        let name = args.get("name").map(String::as_str).unwrap_or("");
        let macos_binding = context.find_binding("macos", name).unwrap_or_default();
        let linux_binding = context.find_binding("linux", name).unwrap_or_default();
        format!("<kbd class=\"keybinding\">{macos_binding}|{linux_binding}</kbd>")
    }
}
