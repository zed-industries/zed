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

use std::collections::HashMap;
use gpui::{ Hsla, HighlightStyle };
use theme::SyntaxTheme;

use crate::editor_settings::VariableColorMode;

// ============================================================================
// Variable Color Cache (Optimized for Hot Path)
// ============================================================================

pub struct VariableColorCache {
    colors: HashMap<u64, Hsla>,
    pub mode: VariableColorMode,
    max_entries: usize,
}

impl VariableColorCache {
    pub fn new(mode: VariableColorMode) -> Self {
        Self {
            colors: HashMap::default(),
            mode,
            max_entries: 1000,
        }
    }

    #[inline]
    pub fn get_or_insert(&mut self, identifier: &str, theme: &SyntaxTheme) -> HighlightStyle {
        let hash = hash_identifier(identifier);
        
        if let Some(&color) = self.colors.get(&hash) {
            return HighlightStyle {
                color: Some(color),
                ..Default::default()
            };
        }
        
        if self.colors.len() >= self.max_entries {
            self.colors.retain(|k, _| k % 2 == 0);
        }
        
        let color = match self.mode {
            VariableColorMode::ThemePalette => {
                const RAINBOW_PALETTE_SIZE: usize = 32;
                let index = fibonacci_hash(hash, RAINBOW_PALETTE_SIZE);
                theme.rainbow_color(index)
                    .and_then(|style| style.color)
                    .unwrap_or_else(|| gpui::rgb(0xa6e3a1).into())
            }
            VariableColorMode::DynamicHSL => {
                let hue = hash_to_hue(hash);
                Hsla { h: hue, s: 0.70, l: 0.65, a: 1.0 }
            }
        };
        
        self.colors.insert(hash, color);
        
        HighlightStyle {
            color: Some(color),
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.colors.clear();
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_hashing() {
        let hash1 = hash_identifier("my_variable");
        let hash2 = hash_identifier("my_variable");
        let hash3 = hash_identifier("other_variable");

        assert_eq!(hash1, hash2, "Same identifier should produce same hash");
        assert_ne!(hash1, hash3, "Different identifiers should produce different hashes");
    }

    #[test]
    fn test_hash_to_color_index() {
        let name = "my_variable";
        let palette_size = 12;

        let index1 = hash_to_color_index(name, palette_size);
        let index2 = hash_to_color_index(name, palette_size);

        assert_eq!(index1, index2);
        assert!(index1 < palette_size);
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
    fn test_hash_identifier_different_values() {
        let hash1 = hash_identifier("variable_a");
        let hash2 = hash_identifier("variable_b");
        let hash3 = hash_identifier("avariable_");
        
        assert_ne!(hash1, hash2, "Different strings should have different hashes");
        assert_ne!(hash1, hash3, "Different strings should have different hashes");
        assert_ne!(hash2, hash3, "Different strings should have different hashes");
    }
}
