use gpui::HighlightStyle;
use language::rainbow_config;
use theme::SyntaxTheme;

use crate::rainbow_cache::RainbowCache;
use crate::rainbow_highlighter::RainbowHighlighter;

/// Shared rainbow highlighting for tree-sitter and LSP semantic tokens.
/// Provides both uncached and cached variants for different use cases.
#[inline]
#[allow(dead_code)]
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

/// Cached version for better performance on repeated identifiers.
#[inline]
pub fn try_rainbow_highlight_cached(
    identifier: &str,
    is_variable_like: bool,
    rainbow_enabled: bool,
    theme: &SyntaxTheme,
    cache: &mut RainbowCache,
) -> Option<HighlightStyle> {
    if !rainbow_enabled || !is_variable_like {
        return None;
    }
    
    if !rainbow_config::is_valid_identifier(identifier) {
        return None;
    }
    
    if let Some(cached_style) = cache.get(identifier) {
        return Some(cached_style);
    }
    
    let palette_size = theme.rainbow_palette_size();
    let hash_index = RainbowHighlighter::hash_to_index(identifier, palette_size);
    if let Some(style) = theme.rainbow_color(hash_index) {
        cache.insert(identifier, style);
        Some(style)
    } else {
        None
    }
}

/// Determines if a token type should receive rainbow highlighting.
#[inline]
pub fn is_variable_like_token(token_type: &str) -> bool {
    matches!(token_type, "variable" | "parameter")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_variable_like_token() {
        assert!(is_variable_like_token("variable"));
        assert!(is_variable_like_token("parameter"));
        assert!(!is_variable_like_token("property"));
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
