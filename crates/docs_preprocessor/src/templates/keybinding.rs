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
        "kb"
    }

    fn regex(&self) -> Regex {
        Regex::new(&format!(r"\{{#{}(.*?)\}}", self.key())).unwrap()
    }

    fn parse_args(&self, args: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("action".to_string(), args.trim().to_string());
        map
    }

    fn render(&self, context: &PreprocessorContext, args: &HashMap<String, String>) -> String {
        let action = args.get("action").map(String::as_str).unwrap_or("");
        let macos_binding = context.find_binding("macos", action).unwrap_or_default();
        let linux_binding = context.find_binding("linux", action).unwrap_or_default();
        let windows_binding = context.find_binding("windows", action).unwrap_or_default();

        if macos_binding.is_empty() && linux_binding.is_empty() && windows_binding.is_empty() {
            return "<div>No default binding</div>".to_string();
        }

        format!("<kbd class=\"keybinding\">{macos_binding}|{linux_binding}|{windows_binding}</kbd>")
    }
}
