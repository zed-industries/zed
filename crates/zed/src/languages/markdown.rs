use language::InteractionProvider;

pub struct MarkdownInteractions;

impl InteractionProvider for MarkdownInteractions {
    fn on_click(&self, text: &str) -> Option<String> {
        if text == "[ ]" {
            return Some("[x]".to_string());
        } else if text == "[x]" {
            return Some("[ ]".to_string());
        } else {
            return None;
        }
    }
}
