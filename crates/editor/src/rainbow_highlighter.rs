const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME: u64 = 1099511628211;
const GOLDEN_RATIO_MULTIPLIER: u64 = 11400714819323198485u64;

#[inline]
pub fn fnv1a_hash(s: &str) -> u64 {
    s.bytes().fold(FNV_OFFSET, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(FNV_PRIME)
    })
}

#[inline]
fn fibonacci_hash(hash: u64, palette_size: usize) -> usize {
    let distributed = hash.wrapping_mul(GOLDEN_RATIO_MULTIPLIER);
    (distributed as usize) % palette_size
}

#[allow(dead_code)]
pub struct RainbowHighlighter;

#[allow(dead_code)]
impl RainbowHighlighter {
    #[inline]
    pub fn hash_to_index(variable_name: &str, palette_size: usize) -> usize {
        let hash = fnv1a_hash(variable_name);
        fibonacci_hash(hash, palette_size)
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
        
        assert!(palette_size > 0, "Default theme should have colors");
        
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
        
        let names = ["var", "var1", "var2", "var_a", "var_b"];
        let indices: Vec<_> = names.iter()
            .map(|name| RainbowHighlighter::hash_to_index(name, palette_size))
            .collect();
        
        for i in 0..indices.len() {
            for j in (i + 1)..indices.len() {
                let diff = indices[i].abs_diff(indices[j]);
                
                assert!(
                    diff >= 1,
                    "Similar names '{}' and '{}' have indices too close: {} vs {}",
                    names[i], names[j], indices[i], indices[j]
                );
            }
        }
    }
    
    #[test]
    fn test_actual_agent_names_distribution() {
        let palette_size = 32;
        
        // Variables from the screenshot
        let names = vec![
            "devops_agent",
            "implementation_plan_agent",
            "implementation_plan_summary_agent",
            "initial_code_generation_agent",
            "solution_architect_agent",
            "root_agent",
        ];
        
        println!("\n=== Agent Variable Color Distribution (palette_size={}) ===", palette_size);
        let mut indices = Vec::new();
        for name in &names {
            let index = RainbowHighlighter::hash_to_index(name, palette_size);
            println!("{:<40} -> color index {:2}", name, index);
            indices.push(index);
        }
        
        // Check for collisions and close colors
        for i in 0..names.len() {
            for j in (i+1)..names.len() {
                if indices[i] == indices[j] {
                    println!("⚠️  COLLISION: '{}' and '{}' both map to color {}", names[i], names[j], indices[i]);
                }
                let diff = indices[i].abs_diff(indices[j]);
                if diff <= 2 && diff > 0 {
                    println!("⚠️  SIMILAR COLORS: '{}' (index {}) and '{}' (index {}) differ by only {}", 
                        names[i], indices[i], names[j], indices[j], diff);
                }
            }
        }
    }
}
