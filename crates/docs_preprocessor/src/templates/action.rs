use crate::PreprocessorContext;
use regex::Regex;
use std::collections::HashMap;

use super::Template;

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
        Regex::new(&format!(
            "\\{{\\s*#{}\\s+name=\"(.*?)\"\\s*\\}}",
            self.key()
        ))
        .unwrap()
    }

    fn parse_args(&self, args: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), args.trim().to_string());
        map
    }

    fn render(&self, _context: &PreprocessorContext, args: &HashMap<String, String>) -> String {
        let name = args.get("name").map(|s| s.to_string()).unwrap_or_default();
        
        let formatted_name = name
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!(" {}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim()
            .to_string();
        
        format!("<code class=\"hljs\">{}</code>", formatted_name)
    }
}
