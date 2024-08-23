use crate::PreprocessorContext;
use regex::Regex;
use std::collections::HashMap;

pub trait Template {
    fn key(&self) -> &'static str;
    fn regex(&self) -> Regex;
    fn parse_args(&self, args: &str) -> HashMap<String, String>;
    fn render(&self, context: &PreprocessorContext, args: &HashMap<String, String>) -> String;

    fn process(&self, context: &PreprocessorContext, content: &str) -> String {
        self.regex()
            .replace_all(content, |caps: &regex::Captures| {
                let args = self.parse_args(&caps[1]);
                self.render(context, &args)
            })
            .into_owned()
    }
}

pub struct ActionTemplate;

impl ActionTemplate {
    pub fn new() -> Self {
        ActionTemplate
    }
}

impl Template for ActionTemplate {
    fn key(&self) -> &'static str {
        "action"
    }

    fn regex(&self) -> Regex {
        Regex::new(&format!(r"\{{\s*#{}\s+(.+?)\s*}}", self.key())).unwrap()
    }

    fn parse_args(&self, args: &str) -> HashMap<String, String> {
        args.split_whitespace()
            .filter_map(|arg| {
                let mut parts = arg.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect()
    }

    fn render(&self, _context: &PreprocessorContext, args: &HashMap<String, String>) -> String {
        let name = args.get("name").map(String::as_str).unwrap_or("");
        format!("<code class=\"hljs\">{}</code>", name)
    }
}

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
        Regex::new(&format!(r"\{{\s*#{}\s+(.+?)\s*}}", self.key())).unwrap()
    }

    fn parse_args(&self, args: &str) -> HashMap<String, String> {
        args.split_whitespace()
            .filter_map(|arg| {
                let mut parts = arg.splitn(2, '=');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect()
    }

    fn render(&self, context: &PreprocessorContext, args: &HashMap<String, String>) -> String {
        let name = args.get("name").map(String::as_str).unwrap_or("");
        let macos_binding = context.find_binding("macos", name).unwrap_or_default();
        let linux_binding = context.find_binding("linux", name).unwrap_or_default();
        format!("<kbd class=\"keybinding\">{macos_binding}|{linux_binding}</kbd>")
    }
}
