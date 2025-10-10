#[allow(dead_code)]
pub struct RainbowHighlighter;

#[allow(dead_code)]
impl RainbowHighlighter {
    #[inline]
    pub fn hash_to_index(variable_name: &str, palette_size: usize) -> usize {
        Self::fnv1a_hash(variable_name) as usize % palette_size
    }
    
    #[inline]
    fn fnv1a_hash(s: &str) -> u64 {
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;
        
        s.bytes().fold(FNV_OFFSET, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(FNV_PRIME)
        })
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
}
