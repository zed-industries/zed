#[allow(dead_code)]
pub struct RainbowHighlighter;

#[allow(dead_code)]
impl RainbowHighlighter {
    /// Maps a variable name to a color index using Fibonacci hashing.
    /// This provides superior color distribution compared to simple modulo operations,
    /// preventing color clustering and ensuring variables have visually distinct colors.
    #[inline]
    pub fn hash_to_index(variable_name: &str, palette_size: usize) -> usize {
        let hash = Self::fnv1a_hash(variable_name);
        Self::fibonacci_hash(hash, palette_size)
    }
    
    /// FNV-1a hash: deterministic, collision-resistant string hashing
    #[inline]
    fn fnv1a_hash(s: &str) -> u64 {
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;
        
        s.bytes().fold(FNV_OFFSET, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(FNV_PRIME)
        })
    }
    
    /// Fibonacci hashing: Golden ratio multiplication for perfect distribution
    /// 
    /// Magic constant: 2^64 / φ (golden ratio) = 11400714819323198485
    /// This maps hash values evenly across the full 64-bit range without clustering,
    /// ensuring adjacent hashes produce well-distributed color indices.
    #[inline]
    fn fibonacci_hash(hash: u64, palette_size: usize) -> usize {
        const GOLDEN_RATIO_MULTIPLIER: u64 = 11400714819323198485u64;
        
        let distributed = hash.wrapping_mul(GOLDEN_RATIO_MULTIPLIER);
        (distributed as usize) % palette_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_deterministic() {
        let name = "my_variable";
        let palette_size = 12;
        
        let index1 = RainbowHighlighter::hash_to_index(name, palette_size);
        let index2 = RainbowHighlighter::hash_to_index(name, palette_size);
        
        assert_eq!(index1, index2, "Same name should produce same index");
    }
    
    #[test]
    fn test_hash_within_bounds() {
        let names = vec!["foo", "bar", "baz", "long_variable_name", "x"];
        let palette_size = 12;
        
        for name in names {
            let index = RainbowHighlighter::hash_to_index(name, palette_size);
            assert!(index < palette_size, "Index {} should be < {}", index, palette_size);
        }
    }
    
    #[test]
    fn test_hash_distribution() {
        let names: Vec<_> = (0..100).map(|i| format!("var_{}", i)).collect();
        let palette_size = 12;
        let mut counts = vec![0; palette_size];
        
        for name in &names {
            let index = RainbowHighlighter::hash_to_index(name, palette_size);
            counts[index] += 1;
        }
        
        for count in &counts {
            assert!(*count > 0, "Poor distribution detected");
        }
    }
    
    #[test]
    fn test_different_names_different_indices() {
        let palette_size = 12;
        let index1 = RainbowHighlighter::hash_to_index("foo", palette_size);
        let index2 = RainbowHighlighter::hash_to_index("bar", palette_size);
        
        assert_ne!(index1, index2, "Different names should produce different indices");
    }
    
    #[test]
    fn test_rainbow_end_to_end_with_theme() {
        use theme::SyntaxTheme;
        
        let theme = SyntaxTheme::default();
        let palette_size = theme.rainbow_palette_size();
        
        assert_eq!(palette_size, 12, "Default theme should have 12 colors");
        
        let var1_index = RainbowHighlighter::hash_to_index("foo", palette_size);
        let var2_index = RainbowHighlighter::hash_to_index("bar", palette_size);
        
        let style1 = theme.rainbow_color(var1_index);
        let style2 = theme.rainbow_color(var2_index);
        
        assert!(style1.is_some(), "Should get style for foo");
        assert!(style2.is_some(), "Should get style for bar");
        
        let var1_again = RainbowHighlighter::hash_to_index("foo", palette_size);
        assert_eq!(var1_index, var1_again, "Same variable should hash to same index");
    }
    
    #[test]
    fn test_identifier_length_boundaries() {
        let palette_size = 12;
        
        let very_long_name = "a".repeat(33);
        let max_valid_name = "a".repeat(32);
        let min_valid_name = "ab";
        let too_short = "a";
        
        let long_index = RainbowHighlighter::hash_to_index(&very_long_name, palette_size);
        let max_index = RainbowHighlighter::hash_to_index(&max_valid_name, palette_size);
        let min_index = RainbowHighlighter::hash_to_index(&min_valid_name, palette_size);
        let short_index = RainbowHighlighter::hash_to_index(&too_short, palette_size);
        
        assert!(long_index < palette_size, "Even long names should hash correctly");
        assert!(max_index < palette_size, "32-char names should hash correctly");
        assert!(min_index < palette_size, "2-char names should hash correctly");
        assert!(short_index < palette_size, "1-char names should hash correctly");
    }
    
    #[test]
    fn test_fibonacci_hashing_distribution() {
        let names: Vec<_> = (0..1000).map(|i| format!("variable_{}", i)).collect();
        let palette_size = 12;
        let mut counts = vec![0; palette_size];
        
        for name in &names {
            let index = RainbowHighlighter::hash_to_index(name, palette_size);
            counts[index] += 1;
        }
        
        let expected_per_bucket = 1000.0 / palette_size as f64;
        
        for count in &counts {
            let deviation = (*count as f64 - expected_per_bucket).abs() / expected_per_bucket;
            assert!(deviation < 0.3, "Distribution deviation too high: {} items in bucket (expected ~{})", count, expected_per_bucket);
        }
        
        let min_count = *counts.iter().min().unwrap();
        let max_count = *counts.iter().max().unwrap();
        assert!(max_count - min_count < 30, "Distribution range too wide: min={}, max={}", min_count, max_count);
    }
    
    #[test]
    fn test_similar_names_distinct_colors() {
        let palette_size = 12;
        
        let names = vec!["var", "var1", "var2", "var_a", "var_b"];
        let indices: Vec<_> = names.iter()
            .map(|name| RainbowHighlighter::hash_to_index(name, palette_size))
            .collect();
        
        for i in 0..indices.len() {
            for j in (i + 1)..indices.len() {
                let diff = if indices[i] > indices[j] {
                    indices[i] - indices[j]
                } else {
                    indices[j] - indices[i]
                };
                
                assert!(
                    diff >= 1,
                    "Similar names '{}' and '{}' have indices too close: {} vs {}",
                    names[i], names[j], indices[i], indices[j]
                );
            }
        }
    }
}
