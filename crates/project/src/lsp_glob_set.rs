use anyhow::{anyhow, Result};
use std::path::Path;

#[derive(Default)]
pub struct LspGlobSet {
    patterns: Vec<glob::Pattern>,
}

impl LspGlobSet {
    pub fn clear(&mut self) {
        self.patterns.clear();
    }

    /// Add a pattern to the glob set.
    ///
    /// LSP's glob syntax supports bash-style brace expansion. For example,
    /// the pattern '*.{js,ts}' would match all JavaScript or TypeScript files.
    /// This is not a part of the standard libc glob syntax, and isn't supported
    /// by the `glob` crate. So we pre-process the glob patterns, producing a
    /// separate glob `Pattern` object for each part of a brace expansion.
    pub fn add_pattern(&mut self, pattern: &str) -> Result<()> {
        // Find all of the ranges of `pattern` that contain matched curly braces.
        let mut expansion_ranges = Vec::new();
        let mut expansion_start_ix = None;
        for (ix, c) in pattern.match_indices(|c| ['{', '}'].contains(&c)) {
            match c {
                "{" => {
                    if expansion_start_ix.is_some() {
                        return Err(anyhow!("nested braces in glob patterns aren't supported"));
                    }
                    expansion_start_ix = Some(ix);
                }
                "}" => {
                    if let Some(start_ix) = expansion_start_ix {
                        expansion_ranges.push(start_ix..ix + 1);
                    }
                    expansion_start_ix = None;
                }
                _ => {}
            }
        }

        // Starting with a single pattern, process each brace expansion by cloning
        // the pattern once per element of the expansion.
        let mut unexpanded_patterns = vec![];
        let mut expanded_patterns = vec![pattern.to_string()];

        for outer_range in expansion_ranges.into_iter().rev() {
            let inner_range = (outer_range.start + 1)..(outer_range.end - 1);
            std::mem::swap(&mut unexpanded_patterns, &mut expanded_patterns);
            for unexpanded_pattern in unexpanded_patterns.drain(..) {
                for part in unexpanded_pattern[inner_range.clone()].split(',') {
                    let mut expanded_pattern = unexpanded_pattern.clone();
                    expanded_pattern.replace_range(outer_range.clone(), part);
                    expanded_patterns.push(expanded_pattern);
                }
            }
        }

        // Parse the final glob patterns and add them to the set.
        for pattern in expanded_patterns {
            let pattern = glob::Pattern::new(&pattern)?;
            self.patterns.push(pattern);
        }

        Ok(())
    }

    pub fn matches(&self, path: &Path) -> bool {
        self.patterns
            .iter()
            .any(|pattern| pattern.matches_path(path))
    }
}

impl std::fmt::Debug for LspGlobSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set()
            .entries(self.patterns.iter().map(|p| p.as_str()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_set() {
        let mut watch = LspGlobSet::default();
        watch.add_pattern("/a/**/*.rs").unwrap();
        watch.add_pattern("/a/**/Cargo.toml").unwrap();

        assert!(watch.matches("/a/b.rs".as_ref()));
        assert!(watch.matches("/a/b/c.rs".as_ref()));

        assert!(!watch.matches("/b/c.rs".as_ref()));
        assert!(!watch.matches("/a/b.ts".as_ref()));
    }

    #[test]
    fn test_brace_expansion() {
        let mut watch = LspGlobSet::default();
        watch.add_pattern("/a/*.{ts,js,tsx}").unwrap();

        assert!(watch.matches("/a/one.js".as_ref()));
        assert!(watch.matches("/a/two.ts".as_ref()));
        assert!(watch.matches("/a/three.tsx".as_ref()));

        assert!(!watch.matches("/a/one.j".as_ref()));
        assert!(!watch.matches("/a/two.s".as_ref()));
        assert!(!watch.matches("/a/three.t".as_ref()));
        assert!(!watch.matches("/a/four.t".as_ref()));
        assert!(!watch.matches("/a/five.xt".as_ref()));
    }

    #[test]
    fn test_multiple_brace_expansion() {
        let mut watch = LspGlobSet::default();
        watch.add_pattern("/a/{one,two,three}.{b*c,d*e}").unwrap();

        assert!(watch.matches("/a/one.bic".as_ref()));
        assert!(watch.matches("/a/two.dole".as_ref()));
        assert!(watch.matches("/a/three.deeee".as_ref()));

        assert!(!watch.matches("/a/four.bic".as_ref()));
        assert!(!watch.matches("/a/one.be".as_ref()));
    }
}
