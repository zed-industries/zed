use dashmap::DashMap;
use gpui::{HighlightStyle, Hsla};
use theme::SyntaxTheme;

use crate::editor_settings::VariableColorMode;

#[derive(Debug)]
pub struct VariableColorCache {
    colors: DashMap<u64, Hsla>,
    pub mode: VariableColorMode,
    max_entries: usize,
}

impl VariableColorCache {
    pub fn new(mode: VariableColorMode) -> Self {
        Self {
            // Pre-allocate capacity to reduce realloc overhead
            colors: DashMap::with_capacity(256),
            mode,
            max_entries: 3000, // Increased limit for large codebases
        }
    }

    #[inline]
    pub fn get_or_insert(&self, identifier: &str, theme: &SyntaxTheme) -> HighlightStyle {
        let hash = hash_identifier(identifier);
        self.get_or_insert_by_hash(hash, theme)
    }

    /// Get or insert color from iterator without allocating String.
    /// Returns None if the iterator doesn't represent a valid identifier.
    /// This validates identifier format (starts with letter/_,  contains alphanumeric/_)
    /// in a single pass while computing the hash.
    #[inline]
    pub fn get_or_insert_validated<I>(&self, iter: I, theme: &SyntaxTheme) -> Option<HighlightStyle>
    where
        I: Iterator<Item = char>,
    {
        let hash = hash_and_validate_identifier(iter)?;
        Some(self.get_or_insert_by_hash(hash, theme))
    }

    #[inline]
    pub fn get_or_insert_by_hash(&self, hash: u64, theme: &SyntaxTheme) -> HighlightStyle {
        if let Some(entry) = self.colors.get(&hash) {
            return HighlightStyle {
                color: Some(*entry.value()),
                ..Default::default()
            };
        }

        if self.colors.len() >= self.max_entries {
            return self.generate_color_without_cache(hash, theme);
        }

        let style = self.generate_color_without_cache(hash, theme);
        if let Some(color) = style.color {
            self.colors.insert(hash, color);
        }
        style
    }

    pub fn clear(&self) {
        self.colors.clear();
    }

    pub fn mode(&self) -> VariableColorMode {
        self.mode
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    fn generate_color_without_cache(&self, hash: u64, theme: &SyntaxTheme) -> HighlightStyle {
        let color = match self.mode {
            VariableColorMode::ThemePalette => {
                const RAINBOW_PALETTE_SIZE: usize = 32;
                let index = fibonacci_hash(hash, RAINBOW_PALETTE_SIZE);
                theme
                    .rainbow_color(index)
                    .and_then(|style| style.color)
                    .unwrap_or_else(|| gpui::rgb(0xa6e3a1).into())
            }
            VariableColorMode::DynamicHSL => {
                let hue = hash_to_hue(hash);
                Hsla {
                    h: hue,
                    s: 0.70,
                    l: 0.65,
                    a: 1.0,
                }
            }
        };

        HighlightStyle {
            color: Some(color),
            ..Default::default()
        }
    }
}

const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME: u64 = 1099511628211;
const GOLDEN_RATIO_MULTIPLIER: u64 = 11400714819323198485u64;

#[inline]
pub fn hash_identifier(s: &str) -> u64 {
    s.bytes().fold(FNV_OFFSET, |hash, byte| {
        (hash ^ (byte as u64)).wrapping_mul(FNV_PRIME)
    })
}

/// Hash identifier from iterator WITH validation in a single pass.
/// Returns None if not a valid identifier (must start with letter/underscore,
/// contain only alphanumeric/underscore, non-empty, and ≤120 bytes).
#[inline]
pub fn hash_and_validate_identifier<I>(mut iter: I) -> Option<u64>
where
    I: Iterator<Item = char>,
{
    let mut hash = FNV_OFFSET;
    let mut len = 0;
    const MAX_LEN: usize = 120;

    // Check first character
    let first = iter.next()?;
    if !first.is_alphabetic() && first != '_' {
        return None;
    }

    // Hash first character
    for byte in first.encode_utf8(&mut [0; 4]).bytes() {
        hash = (hash ^ (byte as u64)).wrapping_mul(FNV_PRIME);
    }
    len += first.len_utf8();

    // Process remaining characters
    for ch in iter {
        if !ch.is_alphanumeric() && ch != '_' {
            return None;
        }

        len += ch.len_utf8();
        if len > MAX_LEN {
            return None;
        }

        for byte in ch.encode_utf8(&mut [0; 4]).bytes() {
            hash = (hash ^ (byte as u64)).wrapping_mul(FNV_PRIME);
        }
    }

    Some(hash)
}
pub fn validate_identifier_for_rainbow(text: &str) -> Option<&str> {
    let trimmed = text.trim();

    if trimmed.is_empty() || trimmed.len() > 120 {
        return None;
    }
    let mut chars = trimmed.chars();
    let first = chars.next()?;

    if !first.is_alphabetic() && first != '_' {
        return None;
    }

    if !chars.all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    Some(trimmed)
}

#[inline]
fn fibonacci_hash(hash: u64, palette_size: usize) -> usize {
    let distributed = hash.wrapping_mul(GOLDEN_RATIO_MULTIPLIER);
    (distributed as usize) % palette_size
}

#[inline]
#[cfg(test)]
pub fn hash_to_color_index(identifier: &str, palette_size: usize) -> usize {
    let hash = hash_identifier(identifier);
    fibonacci_hash(hash, palette_size)
}

#[inline]
fn hash_to_hue(hash: u64) -> f32 {
    let distributed = hash.wrapping_mul(GOLDEN_RATIO_MULTIPLIER);
    (distributed as f32) / (u64::MAX as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_hashing() {
        let hash1 = hash_identifier("my_variable");
        let hash2 = hash_identifier("my_variable");
        let hash3 = hash_identifier("other_variable");

        assert_eq!(hash1, hash2, "Same identifier should produce same hash");
        assert_ne!(
            hash1, hash3,
            "Different identifiers should produce different hashes"
        );
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
        assert!(
            coverage > 0.9,
            "Hue coverage {:.2} should span most of spectrum",
            coverage
        );
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
            assert!(
                *count > 50,
                "Bucket {} has poor distribution: {} items",
                i,
                count
            );
        }
    }

    #[test]
    fn test_hash_identifier_different_values() {
        let hash1 = hash_identifier("variable_a");
        let hash2 = hash_identifier("variable_b");
        let hash3 = hash_identifier("avariable_");

        assert_ne!(
            hash1, hash2,
            "Different strings should have different hashes"
        );
        assert_ne!(
            hash1, hash3,
            "Different strings should have different hashes"
        );
        assert_ne!(
            hash2, hash3,
            "Different strings should have different hashes"
        );
    }
}
