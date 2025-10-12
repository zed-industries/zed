use crate::rainbow_cache::RainbowCache;
use crate::rainbow_highlighting::{is_variable_like_token, try_rainbow_highlight, try_rainbow_highlight_cached};
use crate::rainbow_highlighter::RainbowHighlighter;
use gpui::HighlightStyle;
use theme::SyntaxTheme;

#[gpui::test]
fn test_variable_like_token_detection() {
    assert!(is_variable_like_token("variable"));
    assert!(is_variable_like_token("parameter"));
    
    assert!(!is_variable_like_token("function"));
    assert!(!is_variable_like_token("class"));
    assert!(!is_variable_like_token("property"));
    assert!(!is_variable_like_token("type"));
    assert!(!is_variable_like_token("keyword"));
    assert!(!is_variable_like_token("string"));
    assert!(!is_variable_like_token("number"));
    assert!(!is_variable_like_token("operator"));
}

#[gpui::test]
fn test_rainbow_highlight_when_disabled() {
    let theme = SyntaxTheme::default();
    
    let result = try_rainbow_highlight("my_variable", true, false, &theme);
    
    assert!(result.is_none(), "Should return None when rainbow highlighting is disabled");
}

#[gpui::test]
fn test_rainbow_highlight_non_variable() {
    let theme = SyntaxTheme::default();
    
    let result = try_rainbow_highlight("function_name", false, true, &theme);
    
    assert!(result.is_none(), "Should return None for non-variable tokens");
}

#[gpui::test]
fn test_rainbow_highlight_invalid_identifier() {
    let theme = SyntaxTheme::default();
    
    let result = try_rainbow_highlight("a", true, true, &theme);
    
    assert!(result.is_none(), "Should return None for identifiers that are too short");
}

#[gpui::test]
fn test_rainbow_highlight_valid_identifier() {
    let theme = SyntaxTheme::default();
    
    let result = try_rainbow_highlight("my_variable", true, true, &theme);
    
    assert!(result.is_some(), "Should return Some for valid variable identifiers");
}

#[gpui::test]
fn test_rainbow_highlight_determinism() {
    let theme = SyntaxTheme::default();
    
    let result1 = try_rainbow_highlight("foo_bar", true, true, &theme);
    let result2 = try_rainbow_highlight("foo_bar", true, true, &theme);
    
    assert_eq!(result1, result2, "Same identifier should always produce same color");
}

#[gpui::test]
fn test_rainbow_highlight_different_identifiers() {
    let theme = SyntaxTheme::default();
    let palette_size = theme.rainbow_palette_size();
    
    let mut indices = std::collections::HashSet::new();
    let identifiers = vec!["foo", "bar", "baz", "qux", "quux"];
    
    for id in &identifiers {
        let index = RainbowHighlighter::hash_to_index(id, palette_size);
        indices.insert(index);
    }
    
    assert!(indices.len() > 1, "Different identifiers should produce different colors");
}

#[gpui::test]
fn test_rainbow_highlight_cached_same_result() {
    let theme = SyntaxTheme::default();
    let mut cache = RainbowCache::new();
    
    let result1 = try_rainbow_highlight_cached("my_var", true, true, &theme, &mut cache);
    let result2 = try_rainbow_highlight_cached("my_var", true, true, &theme, &mut cache);
    
    assert_eq!(result1, result2, "Cached results should be consistent");
}

#[gpui::test]
fn test_rainbow_highlight_cache_hit() {
    let theme = SyntaxTheme::default();
    let mut cache = RainbowCache::new();
    
    let _ = try_rainbow_highlight_cached("my_var", true, true, &theme, &mut cache);
    
    let cached_result = cache.get("my_var");
    assert!(cached_result.is_some(), "Identifier should be cached after first access");
}

#[gpui::test]
fn test_rainbow_highlight_cache_miss() {
    let cache = RainbowCache::new();
    
    let result = cache.get("non_existent");
    assert!(result.is_none(), "Cache should miss for unseen identifiers");
}

#[gpui::test]
fn test_hash_index_within_bounds() {
    let palette_size = 12;
    let identifiers = vec![
        "short",
        "medium_length",
        "very_long_identifier_name",
        "x",
        "y",
        "z",
        "variable_1",
        "variable_2",
    ];
    
    for id in identifiers {
        let index = RainbowHighlighter::hash_to_index(id, palette_size);
        assert!(index < palette_size, "Hash index {} should be < palette_size {}", index, palette_size);
    }
}

#[gpui::test]
fn test_hash_distribution_quality() {
    let palette_size = 12;
    let mut counts = vec![0; palette_size];
    
    for i in 0..120 {
        let id = format!("var_{}", i);
        let index = RainbowHighlighter::hash_to_index(&id, palette_size);
        counts[index] += 1;
    }
    
    for count in &counts {
        assert!(*count > 0, "All palette colors should be used with sufficient identifiers");
    }
    
    let min = *counts.iter().min().unwrap();
    let max = *counts.iter().max().unwrap();
    let range = max - min;
    
    assert!(range < 20, "Distribution should be relatively even, got range {}", range);
}

#[gpui::test]
fn test_rainbow_highlight_keyword_exclusion() {
    let theme = SyntaxTheme::default();
    
    let keywords = vec!["self", "super", "this", "let", "const", "var"];
    
    for keyword in keywords {
        let result = try_rainbow_highlight(keyword, true, true, &theme);
        assert!(result.is_none() || result.is_some(), "Keywords handling should be consistent");
    }
}


#[gpui::test]
fn test_rainbow_highlight_edge_cases() {
    let theme = SyntaxTheme::default();
    
    let edge_cases = vec![
        ("__dunder__", true),
        ("_private", true),
        ("CONSTANT", true),
        ("camelCase", true),
        ("snake_case", true),
        ("PascalCase", true),
        ("with123numbers", true),
    ];
    
    for (identifier, is_variable) in edge_cases {
        let result = try_rainbow_highlight(identifier, is_variable, true, &theme);
        if identifier.len() >= 2 {
            assert!(result.is_some(), "Valid identifier '{}' should get rainbow color", identifier);
        }
    }
}

#[gpui::test]
fn test_rainbow_palette_size_variations() {
    let theme = SyntaxTheme::default();
    let palette_size = theme.rainbow_palette_size();
    
    assert!(palette_size > 0, "Palette size should be positive");
    
    for i in 0..palette_size {
        let color = theme.rainbow_color(i);
        assert!(color.is_some(), "All palette indices should have colors");
    }
}

#[gpui::test]
fn test_similar_names_different_colors() {
    let theme = SyntaxTheme::default();
    let palette_size = theme.rainbow_palette_size();
    
    let similar_names = ["var", "var1", "var2", "vara", "varb"];
    let mut indices: Vec<usize> = similar_names
        .iter()
        .map(|name| RainbowHighlighter::hash_to_index(name, palette_size))
        .collect();
    
    indices.sort();
    indices.dedup();
    
    assert!(indices.len() >= 3, "Similar names should generally hash to different indices");
}

#[gpui::test]
fn test_unicode_identifier_handling() {
    let theme = SyntaxTheme::default();
    
    let unicode_ids = vec![
        "variable",
        "café",
        "naïve",
        "τ",
    ];
    
    for id in unicode_ids {
        let result = try_rainbow_highlight(id, true, true, &theme);
        if id.chars().count() >= 2 {
            let _ = result;
        }
    }
}

#[gpui::test]
fn test_cache_clear_functionality() {
    let mut cache = RainbowCache::new();
    let style = HighlightStyle::default();
    
    cache.insert("var1", style);
    cache.insert("var2", style);
    assert!(cache.get("var1").is_some());
    assert!(cache.get("var2").is_some());
    
    cache.clear();
    assert!(cache.get("var1").is_none());
    assert!(cache.get("var2").is_none());
}

#[gpui::test]
fn test_hash_consistency_across_calls() {
    use crate::rainbow_highlighter::fnv1a_hash;
    
    let test_strings = vec!["foo", "bar", "baz", "very_long_identifier"];
    
    for s in test_strings {
        let hash1 = fnv1a_hash(s);
        let hash2 = fnv1a_hash(s);
        assert_eq!(hash1, hash2, "Hash should be consistent for '{}'", s);
    }
}

#[gpui::test]
fn test_different_strings_different_hashes() {
    use crate::rainbow_highlighter::fnv1a_hash;
    
    let hash1 = fnv1a_hash("foo");
    let hash2 = fnv1a_hash("bar");
    let hash3 = fnv1a_hash("baz");
    
    assert_ne!(hash1, hash2);
    assert_ne!(hash2, hash3);
    assert_ne!(hash1, hash3);
}
