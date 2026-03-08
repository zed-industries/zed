use regex::Regex;

#[derive(Debug, Clone)]
pub struct FileNestingPattern {
    regex: Regex,
    targets: Vec<String>,
}

impl FileNestingPattern {
    pub fn new(pattern: &str, replacement: &str) -> Option<Self> {
        let regex = compile_pattern(pattern, PatternMode::CaptureWildcard).ok()?;

        let targets = replacement
            .split(',')
            .map(|target| target.trim().replace("$(capture)", "${1}"))
            .collect();

        Some(Self { regex, targets })
    }

    /// Returns a list of patterns/filenames that should be nested under `text` if `text` matches this pattern.
    /// The returned strings may be exact filenames (if the target didn't contain wildcards)
    /// or glob patterns (if the target did contain wildcards, like `*.env`).
    /// Note that `*` in the target is NOT replaced by the regex engine, it remains a literal `*`.
    pub fn match_and_replace(&self, text: &str) -> Option<Vec<String>> {
        let captures = self.regex.captures(text)?;
        let mut result = Vec::with_capacity(self.targets.len());
        for target in &self.targets {
            let mut replaced = String::new();
            captures.expand(target, &mut replaced);
            result.push(replaced);
        }
        Some(result)
    }

    /// Compiles a glob pattern like `*.env` or `foo.*` into a Regex.
    pub fn compile_glob(pattern: &str) -> Option<Regex> {
        compile_pattern(pattern, PatternMode::MatchWildcard).ok()
    }
}

enum PatternMode {
    CaptureWildcard,
    MatchWildcard,
}

fn compile_pattern(pattern: &str, mode: PatternMode) -> Result<Regex, regex::Error> {
    let has_wildcard = pattern.contains('*');
    let mut regex_pattern = String::new();
    let mut parts = pattern.split('*');

    if let Some(first_part) = parts.next() {
        if matches!(mode, PatternMode::CaptureWildcard) && !has_wildcard {
            regex_pattern.push('(');
        }
        regex_pattern.push_str(&regex::escape(first_part));
        if matches!(mode, PatternMode::CaptureWildcard) && !has_wildcard {
            regex_pattern.push(')');
        }
    }

    for part in parts {
        regex_pattern.push_str(match mode {
            PatternMode::CaptureWildcard => "(.*)",
            PatternMode::MatchWildcard => ".*",
        });
        regex_pattern.push_str(&regex::escape(part));
    }

    Regex::new(&format!("^{regex_pattern}$"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nesting_patterns() {
        let pattern = FileNestingPattern::new("*.ts", "$(capture).js").unwrap();
        assert_eq!(
            pattern.match_and_replace("foo.ts"),
            Some(vec!["foo.js".to_string()])
        );
        assert_eq!(
            pattern.match_and_replace("foo.bar.ts"),
            Some(vec!["foo.bar.js".to_string()])
        );
        assert_eq!(pattern.match_and_replace("foo.rs"), None);

        let pattern = FileNestingPattern::new("foo-*.js", "bar-$(capture).ts").unwrap();
        assert_eq!(
            pattern.match_and_replace("foo-baz.js"),
            Some(vec!["bar-baz.ts".to_string()])
        );

        let pattern = FileNestingPattern::new("package.json", "$(capture).lock").unwrap();
        assert_eq!(
            pattern.match_and_replace("package.json"),
            Some(vec!["package.json.lock".to_string()])
        );

        let pattern = FileNestingPattern::new("*.ts", "$(capture).js, $(capture).d.ts").unwrap();
        assert_eq!(
            pattern.match_and_replace("foo.ts"),
            Some(vec!["foo.js".to_string(), "foo.d.ts".to_string()])
        );

        let pattern = FileNestingPattern::new(".env", "*.env, .env.*").unwrap();
        assert_eq!(
            pattern.match_and_replace(".env"),
            Some(vec!["*.env".to_string(), ".env.*".to_string()])
        );

        let pattern = FileNestingPattern::new("*.cs", "$(capture).*.cs, $(capture).cs.uid").unwrap();
        assert_eq!(
            pattern.match_and_replace("foo.cs"),
            Some(vec!["foo.*.cs".to_string(), "foo.cs.uid".to_string()])
        );
        assert_eq!(
            pattern.match_and_replace("foo.bar.cs"),
            Some(vec!["foo.bar.*.cs".to_string(), "foo.bar.cs.uid".to_string()])
        );
    }

    #[test]
    fn test_compile_glob() {
        let wildcard_glob = FileNestingPattern::compile_glob("*.env").unwrap();
        assert!(wildcard_glob.is_match(".env"));
        assert!(wildcard_glob.is_match("foo.env"));
        assert!(!wildcard_glob.is_match("foo.txt"));

        let exact_glob = FileNestingPattern::compile_glob("package.json").unwrap();
        assert!(exact_glob.is_match("package.json"));
        assert!(!exact_glob.is_match("package-lock.json"));
    }
}
