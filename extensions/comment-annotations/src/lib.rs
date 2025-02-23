use tree_sitter::Language;

extern "C" {
    fn tree_sitter_comment_annotations() -> Language;
}

#[no_mangle]
pub extern "C" fn register_highlights() -> bool {
    // Register the language and its highlighting rules
    unsafe {
        let language = tree_sitter_comment_annotations();
        let mut highlighter = tree_sitter_highlight::Highlighter::new();
        let config = tree_sitter_highlight::HighlightConfiguration::new(
            language,
            include_str!("../languages/highlights.scm"),
            "",
            "",
        ).unwrap();
        highlighter.add_configuration(&config).unwrap();
        true
    }
} 