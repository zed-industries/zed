pub enum ContextType {
    CurrentFile,
    Snippet(usize),
}

pub fn collect_context(context_type: ContextType) -> String {
    match context_type {
        ContextType::CurrentFile => "Current File".to_string(),
        ContextType::Snippet(index) => format!("Snippet {}", index),
    }
}
