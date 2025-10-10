use gpui::HighlightStyle;
use language::rainbow_config;
use theme::SyntaxTheme;

use crate::rainbow_highlighter::RainbowHighlighter;

/// Shared rainbow highlighting utilities for both tree-sitter and LSP semantic tokens.
/// This module consolidates the common logic to avoid duplication.

/// Attempts to apply rainbow highlighting to an identifier if rainbow highlighting is enabled.
/// Returns Some(HighlightStyle) if rainbow highlighting should be applied, None otherwise.
///
/// This is used by both:
/// 1. Tree-sitter syntax highlighting (display_map.rs)
/// 2. LSP semantic tokens (SemanticTokenStylizer)
#[inline]
pub fn try_rainbow_highlight(
    identifier: &str,
    is_variable_like: bool,
    rainbow_enabled: bool,
    theme: &SyntaxTheme,
) -> Option<HighlightStyle> {
    if !rainbow_enabled || !is_variable_like {
        return None;
    }
    
    if !rainbow_config::is_valid_identifier(identifier) {
        return None;
    }
    
    let palette_size = theme.rainbow_palette_size();
    let hash_index = RainbowHighlighter::hash_to_index(identifier, palette_size);
    theme.rainbow_color(hash_index)
}

/// Checks if a token type name represents a variable-like token.
/// This includes variables, parameters, and properties.
#[inline]
pub fn is_variable_like_token(token_type: &str) -> bool {
    matches!(token_type, "variable" | "parameter" | "property")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_variable_like_token() {
        assert!(is_variable_like_token("variable"));
        assert!(is_variable_like_token("parameter"));
        assert!(is_variable_like_token("property"));
        assert!(!is_variable_like_token("function"));
        assert!(!is_variable_like_token("class"));
        assert!(!is_variable_like_token("keyword"));
    }
    
    #[test]
    fn test_try_rainbow_highlight_disabled() {
        let theme = SyntaxTheme::default();
        
        let result = try_rainbow_highlight(
            "my_var",
            true,
            false,  // disabled
            &theme,
        );
        
        assert!(result.is_none(), "Should return None when rainbow highlighting is disabled");
    }
    
    #[test]
    fn test_try_rainbow_highlight_not_variable_like() {
        let theme = SyntaxTheme::default();
        
        let result = try_rainbow_highlight(
            "my_var",
            false,  // not variable-like
            true,
            &theme,
        );
        
        assert!(result.is_none(), "Should return None for non-variable tokens");
    }
    
    #[test]
    fn test_try_rainbow_highlight_invalid_identifier() {
        let theme = SyntaxTheme::default();
        
        let result = try_rainbow_highlight(
            "a",  // too short
            true,
            true,
            &theme,
        );
        
        assert!(result.is_none(), "Should return None for invalid identifiers");
    }
    
    #[test]
    fn test_try_rainbow_highlight_valid() {
        let theme = SyntaxTheme::default();
        
        let result = try_rainbow_highlight(
            "my_variable",
            true,
            true,
            &theme,
        );
        
        assert!(result.is_some(), "Should return Some for valid variable");
    }
    
    #[test]
    fn test_rainbow_highlight_deterministic() {
        let theme = SyntaxTheme::default();
        
        let result1 = try_rainbow_highlight("foo", true, true, &theme);
        let result2 = try_rainbow_highlight("foo", true, true, &theme);
        
        assert_eq!(result1, result2, "Same variable should produce same color");
    }
}
