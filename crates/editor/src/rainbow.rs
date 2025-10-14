// Rainbow Highlighting - Complete Implementation
//
// This module provides deterministic, hash-based color highlighting for variables
// to improve code readability. Each variable name gets a consistent color based
// on a hash of its identifier.
//
// ## Architecture
//
// Rainbow highlighting exists in two places:
// 1. LSP Semantic Tokens - Colors variables when LSP provides tokens
// 2. Tree-sitter Fallback - Colors variables when LSP doesn't provide tokens (especially in closures)
//
// This module provides the shared logic for both systems including caching.

use gpui::{ HighlightStyle, Hsla };
use std::cell::RefCell;
use theme::SyntaxTheme;

use crate::editor_settings::{ RainbowConfig, VariableColorMode };

use collections::HashMap;

thread_local! {
    static RAINBOW_CACHE: RefCell<RainbowCache> = RefCell::new(RainbowCache::new());
}

pub fn with_rainbow_cache<F, R>(f: F) -> R where F: FnOnce(&mut RainbowCache) -> R {
    RAINBOW_CACHE.with(|cache| f(&mut cache.borrow_mut()))
}

pub fn clear_rainbow_cache() {
    RAINBOW_CACHE.with(|cache| cache.borrow_mut().clear())
}

#[derive(Debug)]
pub struct RainbowCache {
    cache: HashMap<u64, HighlightStyle>,
    max_entries: usize,
}

impl RainbowCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::default(),
            max_entries: 1000,
        }
    }

    #[inline]
    pub fn get_by_hash(&self, hash: u64) -> Option<HighlightStyle> {
        self.cache.get(&hash).copied()
    }

    pub fn insert_by_hash(&mut self, hash: u64, style: HighlightStyle) {
        if self.cache.len() >= self.max_entries {
            self.cache.retain(|h, _| h % 2 == 0);
        }
        self.cache.insert(hash, style);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for RainbowCache {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Core Hashing Logic
// ============================================================================

const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME: u64 = 1099511628211;
const GOLDEN_RATIO_MULTIPLIER: u64 = 11400714819323198485u64;

/// FNV-1a hash function for identifier names.
/// Fast, simple, and provides good distribution for variable names.
#[inline]
pub fn hash_identifier(s: &str) -> u64 {
    s.bytes().fold(FNV_OFFSET, |hash, byte| { (hash ^ (byte as u64)).wrapping_mul(FNV_PRIME) })
}

/// Fibonacci hashing to distribute hash values into palette indices.
/// Provides better distribution than simple modulo.
#[inline]
fn fibonacci_hash(hash: u64, palette_size: usize) -> usize {
    let distributed = hash.wrapping_mul(GOLDEN_RATIO_MULTIPLIER);
    (distributed as usize) % palette_size
}

/// Maps an identifier name to a color palette index (test helper).
#[inline]
#[cfg(test)]
pub fn hash_to_color_index(identifier: &str, palette_size: usize) -> usize {
    let hash = hash_identifier(identifier);
    fibonacci_hash(hash, palette_size)
}

// ============================================================================
// Golden Ratio HSL Color Generation
// ============================================================================

const GOLDEN_RATIO_MULTIPLIER_F64: u64 = 11400714819323198485u64;

#[inline]
fn hash_to_hue(hash: u64) -> f32 {
    let distributed = hash.wrapping_mul(GOLDEN_RATIO_MULTIPLIER_F64);
    (distributed as f32) / (u64::MAX as f32)
}

/// Generates a dynamic rainbow color with full spectrum distribution.
#[inline]
fn generate_dynamic_rainbow_color(hash: u64) -> HighlightStyle {
    let hue = hash_to_hue(hash);
    let saturation = 0.70;
    let lightness = 0.65;

    let hsla = Hsla {
        h: hue,
        s: saturation,
        l: lightness,
        a: 1.0,
    };

    HighlightStyle {
        color: Some(hsla),
        ..Default::default()
    }
}

// ============================================================================
// Rainbow Highlighting Application
// ============================================================================

/// Applies variable color highlighting using a pre-computed hash.
#[inline]
pub fn apply_rainbow_by_hash(
    hash: u64,
    rainbow_config: &RainbowConfig,
    theme: &SyntaxTheme,
    cache: &mut RainbowCache
) -> Option<HighlightStyle> {
    if !rainbow_config.enabled {
        return None;
    }

    if let Some(cached_style) = cache.get_by_hash(hash) {
        return Some(cached_style);
    }

    let style = match rainbow_config.mode {
        VariableColorMode::DynamicHSL => generate_dynamic_rainbow_color(hash),
        VariableColorMode::ThemePalette => {
            let palette_size = theme.rainbow_palette_size();
            let hash_index = fibonacci_hash(hash, palette_size);
            theme.rainbow_color(hash_index)?
        }
    };

    cache.insert_by_hash(hash, style);
    Some(style)
}

/// Applies variable color highlighting to an identifier.
#[inline]
pub fn apply_rainbow_highlighting(
    identifier: &str,
    is_variable_like: bool,
    rainbow_config: &RainbowConfig,
    theme: &SyntaxTheme,
    cache: &mut RainbowCache
) -> Option<HighlightStyle> {
    if !rainbow_config.enabled || !is_variable_like {
        return None;
    }

    if identifier.is_empty() || identifier.len() < 2 {
        return None;
    }

    let hash = hash_identifier(identifier);
    apply_rainbow_by_hash(hash, rainbow_config, theme, cache)
}

/// Helper to determine if a token type should receive rainbow highlighting.
#[inline]
#[cfg(test)]
pub fn is_variable_like_token(token_type: &str) -> bool {
    matches!(token_type, "variable" | "parameter")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit_miss() {
        let mut cache = RainbowCache::new();
        let test_hash = hash_identifier("test_var");
        let other_hash = hash_identifier("other_var");

        assert!(cache.get_by_hash(test_hash).is_none());

        let style = HighlightStyle::default();
        cache.insert_by_hash(test_hash, style);
        assert!(cache.get_by_hash(test_hash).is_some());

        assert!(cache.get_by_hash(other_hash).is_none());
    }

    #[test]
    fn test_eviction() {
        let mut cache = RainbowCache {
            cache: HashMap::default(),
            max_entries: 10,
        };

        let style = HighlightStyle::default();

        for i in 0..15 {
            let hash = hash_identifier(&format!("var_{}", i));
            cache.insert_by_hash(hash, style);
        }

        assert!(cache.cache.len() < 15);
    }

    #[test]
    fn test_deterministic_hashing() {
        let hash1 = hash_identifier("my_variable");
        let hash2 = hash_identifier("my_variable");
        let hash3 = hash_identifier("other_variable");

        assert_eq!(hash1, hash2, "Same identifier should produce same hash");
        assert_ne!(hash1, hash3, "Different identifiers should produce different hashes");
    }

    #[test]
    fn test_hash_deterministic() {
        let name = "my_variable";
        let palette_size = 12;

        let index1 = hash_to_color_index(name, palette_size);
        let index2 = hash_to_color_index(name, palette_size);

        assert_eq!(index1, index2);
    }

    #[test]
    fn test_hash_within_bounds() {
        let names = vec!["foo", "bar", "baz", "long_variable_name"];
        let palette_size = 12;

        for name in names {
            let index = hash_to_color_index(name, palette_size);
            assert!(index < palette_size);
        }
    }

    #[test]
    fn test_is_variable_like_token() {
        assert!(is_variable_like_token("variable"));
        assert!(is_variable_like_token("parameter"));
        assert!(!is_variable_like_token("property"));
        assert!(!is_variable_like_token("function"));
    }

    #[test]
    fn test_hash_distribution() {
        let names: Vec<_> = (0..100).map(|i| format!("var_{}", i)).collect();
        let palette_size = 12;
        let mut counts = vec![0; palette_size];

        for name in &names {
            let index = hash_to_color_index(name, palette_size);
            counts[index] += 1;
        }

        for count in &counts {
            assert!(*count > 0, "Poor distribution detected");
        }
    }

    #[test]
    fn test_hue_full_spectrum_coverage() {
        let mut min_hue: f32 = 1.0;
        let mut max_hue: f32 = 0.0;
        
        for i in 0..1000 {
            let name = format!("variable_{}", i);
            let hash = hash_identifier(&name);
            let hue = hash_to_hue(hash);
            
            assert!(hue >= 0.0 && hue <= 1.0, "Hue {} out of range", hue);
            min_hue = min_hue.min(hue);
            max_hue = max_hue.max(hue);
        }
        
        let coverage = max_hue - min_hue;
        assert!(coverage > 0.9, "Hue coverage {:.2} should span most of spectrum", coverage);
    }

    #[test]
    fn test_similar_names_distribute_across_spectrum() {
        let names = ["var_1", "var_2", "var_3", "var_a", "var_b", "foo", "bar", "baz"];
        let mut hues = Vec::new();
        
        for name in &names {
            let hash = hash_identifier(name);
            let hue = hash_to_hue(hash);
            hues.push(hue);
        }
        
        let mut all_identical = true;
        for i in 1..hues.len() {
            if (hues[i] - hues[0]).abs() > 0.01 {
                all_identical = false;
                break;
            }
        }
        
        assert!(!all_identical, "All similar variable names should not get identical hues");
        
        let min = hues.iter().copied().fold(f32::INFINITY, f32::min);
        let max = hues.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        assert!(max - min > 0.3, "Hues should span at least 30% of spectrum, got {:.2}", max - min);
    }

    #[test]
    fn test_apply_rainbow_with_disabled_config() {
        use crate::editor_settings::{ RainbowConfig, VariableColorMode };
        
        let mut cache = RainbowCache::new();
        let config = RainbowConfig {
            enabled: false,
            mode: VariableColorMode::DynamicHSL,
        };
        let theme = SyntaxTheme::default();
        
        let result = apply_rainbow_highlighting("variable", true, &config, &theme, &mut cache);
        assert!(result.is_none(), "Disabled config should return None");
    }

    #[test]
    fn test_apply_rainbow_with_non_variable() {
        use crate::editor_settings::{ RainbowConfig, VariableColorMode };
        
        let mut cache = RainbowCache::new();
        let config = RainbowConfig {
            enabled: true,
            mode: VariableColorMode::DynamicHSL,
        };
        let theme = SyntaxTheme::default();
        
        let result = apply_rainbow_highlighting("function", false, &config, &theme, &mut cache);
        assert!(result.is_none(), "Non-variable should return None");
    }

    #[test]
    fn test_apply_rainbow_with_empty_identifier() {
        use crate::editor_settings::{ RainbowConfig, VariableColorMode };
        
        let mut cache = RainbowCache::new();
        let config = RainbowConfig {
            enabled: true,
            mode: VariableColorMode::DynamicHSL,
        };
        let theme = SyntaxTheme::default();
        
        let result = apply_rainbow_highlighting("", true, &config, &theme, &mut cache);
        assert!(result.is_none(), "Empty identifier should return None");
    }

    #[test]
    fn test_apply_rainbow_with_single_char() {
        use crate::editor_settings::{ RainbowConfig, VariableColorMode };
        
        let mut cache = RainbowCache::new();
        let config = RainbowConfig {
            enabled: true,
            mode: VariableColorMode::DynamicHSL,
        };
        let theme = SyntaxTheme::default();
        
        let result = apply_rainbow_highlighting("x", true, &config, &theme, &mut cache);
        assert!(result.is_none(), "Single char identifier should return None");
    }

    #[test]
    fn test_apply_rainbow_caches_result() {
        use crate::editor_settings::{ RainbowConfig, VariableColorMode };
        
        let mut cache = RainbowCache::new();
        let config = RainbowConfig {
            enabled: true,
            mode: VariableColorMode::DynamicHSL,
        };
        let theme = SyntaxTheme::default();
        
        let result1 = apply_rainbow_highlighting("variable", true, &config, &theme, &mut cache);
        assert!(result1.is_some(), "Should return style for valid variable");
        
        let hash = hash_identifier("variable");
        let cached = cache.get_by_hash(hash);
        assert!(cached.is_some(), "Result should be cached");
        assert_eq!(cached, result1, "Cached result should match");
    }

    #[test]
    fn test_fibonacci_hash_uniform_distribution() {
        let mut bucket_counts = [0; 12];
        
        for i in 0..1200 {
            let name = format!("identifier_{}", i);
            let hash = hash_identifier(&name);
            let bucket = fibonacci_hash(hash, 12);
            bucket_counts[bucket] += 1;
        }
        
        for (i, count) in bucket_counts.iter().enumerate() {
            assert!(*count > 50, "Bucket {} has poor distribution: {} items", i, count);
        }
    }

    #[test]
    fn test_hash_identifier_deterministic() {
        for _ in 0..100 {
            let hash1 = hash_identifier("test_variable");
            let hash2 = hash_identifier("test_variable");
            assert_eq!(hash1, hash2, "Hash function must be deterministic");
        }
    }

    #[test]
    fn test_hash_identifier_different_values() {
        let hash1 = hash_identifier("variable_a");
        let hash2 = hash_identifier("variable_b");
        let hash3 = hash_identifier("avariable_");
        
        assert_ne!(hash1, hash2, "Different strings should have different hashes");
        assert_ne!(hash1, hash3, "Different strings should have different hashes");
        assert_ne!(hash2, hash3, "Different strings should have different hashes");
    }

    #[test]
    fn test_apply_by_hash_with_disabled_config() {
        use crate::editor_settings::{ RainbowConfig, VariableColorMode };
        
        let mut cache = RainbowCache::new();
        let config = RainbowConfig {
            enabled: false,
            mode: VariableColorMode::DynamicHSL,
        };
        let theme = SyntaxTheme::default();
        let hash = hash_identifier("variable");
        
        let result = apply_rainbow_by_hash(hash, &config, &theme, &mut cache);
        assert!(result.is_none(), "Disabled config should return None for hash-based apply");
    }

    #[test]
    fn test_cache_eviction_occurs() {
        let mut cache = RainbowCache {
            cache: HashMap::default(),
            max_entries: 10,
        };
        let style = HighlightStyle::default();
        
        for i in 0..20 {
            let hash = i as u64;
            cache.insert_by_hash(hash, style);
        }
        
        assert!(cache.cache.len() < 20, "Cache should evict entries when exceeding max");
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = RainbowCache::new();
        let style = HighlightStyle::default();
        
        for i in 0..10 {
            let hash = hash_identifier(&format!("var_{}", i));
            cache.insert_by_hash(hash, style);
        }
        
        assert!(cache.cache.len() > 0, "Cache should have entries");
        
        cache.clear();
        assert_eq!(cache.cache.len(), 0, "Cache should be empty after clear");
    }
}
