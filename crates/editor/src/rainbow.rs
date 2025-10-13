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

use gpui::{ HighlightStyle, Hsla, Rgba };
use language::rainbow_config;
use multi_buffer::MultiBufferSnapshot;
use std::ops::Range;
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
    #[allow(dead_code)]
    pub fn get(&self, identifier: &str) -> Option<HighlightStyle> {
        let hash = hash_identifier(identifier);
        self.cache.get(&hash).copied()
    }

    #[inline]
    pub fn get_by_hash(&self, hash: u64) -> Option<HighlightStyle> {
        self.cache.get(&hash).copied()
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, identifier: &str, style: HighlightStyle) {
        if self.cache.len() >= self.max_entries {
            self.cache.retain(|hash, _| hash % 2 == 0);
        }

        let hash = hash_identifier(identifier);
        self.cache.insert(hash, style);
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

/// Maps an identifier name to a color palette index.
#[inline]
#[allow(dead_code)]
pub fn hash_to_color_index(identifier: &str, palette_size: usize) -> usize {
    let hash = hash_identifier(identifier);
    fibonacci_hash(hash, palette_size)
}

// ============================================================================
// Golden Ratio HSL Color Generation
// ============================================================================

const GOLDEN_RATIO_CONJUGATE: f32 = 0.618033988749895;

/// Generates a color using golden ratio for optimal color distribution.
///
/// Based on Martin Ankerl's algorithm: https://martin.ankerl.com/2009/12/09/how-to-create-random-colors-programmatically/
///
/// The golden ratio ensures each new color is maximally different from all previous colors,
/// providing better visual distinction than palette-based approaches.
#[inline]
fn hash_to_hue_golden_ratio(hash: u64) -> f32 {
    let normalized = ((hash as f64) / (u64::MAX as f64)) as f32;
    (normalized * GOLDEN_RATIO_CONJUGATE) % 1.0
}

/// Converts HSL to RGB color.
///
/// # Arguments
/// * `h` - Hue (0.0 to 1.0)
/// * `s` - Saturation (0.0 to 1.0)
/// * `l` - Lightness (0.0 to 1.0)
#[inline]
#[allow(dead_code)]
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Rgba {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h * 6.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Rgba {
        r: r + m,
        g: g + m,
        b: b + m,
        a: 1.0,
    }
}

/// Generates a dynamic rainbow color using golden ratio distribution.
///
/// Fixed saturation and lightness values ensure consistent, readable colors
/// across all themes while the hue varies based on the identifier hash.
#[inline]
fn generate_dynamic_rainbow_color(hash: u64) -> HighlightStyle {
    let hue = hash_to_hue_golden_ratio(hash);
    // Fixed saturation and lightness for consistency and readability
    let saturation = 0.65; // Vibrant but not overwhelming
    let lightness = 0.7; // Bright enough for dark themes, visible on light themes

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
// Identifier Extraction
// ============================================================================

/// Extracts a complete identifier by walking backwards/forwards from the chunk.
///
/// Tree-sitter may split identifiers across multiple chunks (e.g., "base_profile" → "b" + "ase_profile").
/// We need to walk the buffer to find the full identifier boundaries.
///
/// # Performance
/// - O(n) where n is identifier length (typically small)
/// - Validates the final extracted string is a valid identifier
#[allow(dead_code)]
pub fn extract_complete_identifier(
    buffer: &MultiBufferSnapshot,
    chunk_range: Range<usize>
) -> Option<(Range<usize>, String)> {
    let total_len = buffer.len();
    if chunk_range.start >= total_len || chunk_range.is_empty() {
        return None;
    }

    // Walk backward from chunk start to find identifier start
    let mut start = chunk_range.start;
    if start > 0 {
        for ch in buffer.reversed_chars_at(start) {
            if ch.is_alphanumeric() || ch == '_' {
                start = start.saturating_sub(ch.len_utf8());
                if start == 0 {
                    break;
                }
            } else {
                break;
            }
        }
    }

    // Walk forward from chunk end to find identifier end
    let mut end = chunk_range.end.min(total_len);
    for ch in buffer.chars_at(end) {
        if ch.is_alphanumeric() || ch == '_' {
            end += ch.len_utf8();
            if end >= total_len {
                break;
            }
        } else {
            break;
        }
    }

    // Extract the full identifier
    if start >= end {
        return None;
    }

    let identifier: String = buffer.text_for_range(start..end).collect();

    // CRITICAL: Validate the extracted string is a valid identifier
    // This prevents extracting things like "iter()" or "&base_profile"
    if identifier.is_empty() || identifier.len() < 2 {
        return None;
    }

    // Must start with letter or underscore
    let first_char = identifier.chars().next()?;
    if !first_char.is_alphabetic() && first_char != '_' {
        return None;
    }

    // ALL characters must be alphanumeric or underscore (no parentheses, operators, whitespace)
    if !identifier.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    Some((start..end, identifier))
}

// ============================================================================
// Rainbow Highlighting Application
// ============================================================================

/// Applies variable color highlighting to an identifier.
///
/// # Arguments
/// * `identifier` - The variable name to color
/// * `is_variable_like` - Whether the token is a variable or parameter
/// * `rainbow_config` - Rainbow highlighting configuration (enabled and mode)
/// * `theme` - The syntax theme with rainbow palette (used for theme_palette mode)
/// * `cache` - Cache for computed styles
///
/// # Returns
/// An optional `HighlightStyle` if variable color highlighting should be applied.
#[inline]
pub fn apply_rainbow_highlighting(
    identifier: &str,
    is_variable_like: bool,
    rainbow_config: &RainbowConfig,
    theme: &SyntaxTheme,
    cache: &mut RainbowCache
) -> Option<HighlightStyle> {
    // Fast path: early returns
    if !rainbow_config.enabled || !is_variable_like {
        return None;
    }

    if !rainbow_config::is_valid_identifier(identifier) {
        return None;
    }

    // Compute hash once
    let hash = hash_identifier(identifier);

    // Check cache first using pre-computed hash
    if let Some(cached_style) = cache.get_by_hash(hash) {
        return Some(cached_style);
    }

    // Compute color based on mode
    let style = match rainbow_config.mode {
        VariableColorMode::DynamicHSL => {
            // Generate color using golden ratio for optimal distribution
            generate_dynamic_rainbow_color(hash)
        }
        VariableColorMode::ThemePalette => {
            // Use theme's rainbow palette
            let palette_size = theme.rainbow_palette_size();
            let hash_index = fibonacci_hash(hash, palette_size);
            theme.rainbow_color(hash_index)?
        }
    };

    cache.insert_by_hash(hash, style);
    Some(style)
}

/// Helper to determine if a token type should receive rainbow highlighting.
#[inline]
#[cfg(test)]
pub fn is_variable_like_token(token_type: &str) -> bool {
    matches!(token_type, "variable" | "parameter")
}

/// Helper to determine if a tree-sitter capture should receive rainbow highlighting.
#[inline]
#[allow(dead_code)]
pub fn is_variable_like_capture(capture_name: &str) -> bool {
    capture_name.starts_with("variable") &&
        !capture_name.contains("special") &&
        !capture_name.contains("builtin") &&
        !capture_name.contains("member")
}

// ============================================================================
// Tree-sitter Rendering Integration
// ============================================================================

/// Cached identifier state for rendering performance.
/// Caches by range since tree-sitter may split identifiers across chunks.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CachedIdentifier {
    pub range: Range<usize>,
    pub style: Option<HighlightStyle>,
}

/// Applies rainbow highlighting to a tree-sitter chunk during rendering.
///
/// This is the main entry point for `display_map.rs`.
///
/// # Performance
/// - Uses caching to avoid re-computing colors for the same identifier
/// - Only extracts identifiers when necessary
/// - Validates ranges before processing
///
/// # Arguments
/// * `chunk_range` - The byte range of the chunk in the buffer
/// * `capture_name` - The tree-sitter capture name (e.g., "variable")
/// * `cached_identifier` - Mutable cache state
/// * `buffer_snapshot` - The buffer to read from
/// * `rainbow_config` - Rainbow highlighting settings
/// * `theme` - The syntax theme
///
/// # Returns
/// An optional `HighlightStyle` if rainbow highlighting should be applied.
#[allow(dead_code)]
pub fn apply_to_chunk(
    chunk_range: Range<usize>,
    capture_name: Option<&str>,
    cached_identifier: &mut Option<CachedIdentifier>,
    buffer_snapshot: &MultiBufferSnapshot,
    rainbow_config: &RainbowConfig,
    theme: &SyntaxTheme
) -> Option<HighlightStyle> {
    // Check if we should apply rainbow
    let capture_name = capture_name?;
    if !is_variable_like_capture(capture_name) {
        return None;
    }

    // Check cache FIRST - if this chunk is part of a cached identifier, return cached color
    if let Some(cached) = cached_identifier {
        if chunk_range.start >= cached.range.start && chunk_range.end <= cached.range.end {
            return cached.style;
        }
    }

    // Extract full identifier by walking backwards/forwards
    let (extracted_range, identifier) = match extract_complete_identifier(buffer_snapshot, chunk_range) {
        Some(result) => result,
        None => {
            return None;
        }
    };

    // Compute rainbow color
    let style = with_rainbow_cache(|cache| {
        let result = apply_rainbow_highlighting(
            &identifier,
            true, // We already know it's variable-like
            rainbow_config,
            theme,
            cache
        );
        result
    });

    // Update cache with the FULL identifier range
    *cached_identifier = Some(CachedIdentifier {
        range: extracted_range,
        style,
    });

    style
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

        assert!(cache.get("test_var").is_none());

        let style = HighlightStyle::default();
        cache.insert("test_var", style);
        assert!(cache.get("test_var").is_some());

        assert!(cache.get("other_var").is_none());
    }

    #[test]
    fn test_eviction() {
        let mut cache = RainbowCache {
            cache: HashMap::default(),
            max_entries: 10,
        };

        let style = HighlightStyle::default();

        for i in 0..15 {
            cache.insert(&format!("var_{}", i), style);
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
    fn test_is_variable_like_capture() {
        assert!(is_variable_like_capture("variable"));
        assert!(is_variable_like_capture("variable.local"));
        assert!(!is_variable_like_capture("variable.special"));
        assert!(!is_variable_like_capture("variable.builtin"));
        assert!(!is_variable_like_capture("variable.member"));
        assert!(!is_variable_like_capture("function"));
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

        // Ensure reasonable distribution
        for count in &counts {
            assert!(*count > 0, "Poor distribution detected");
        }
    }
}
