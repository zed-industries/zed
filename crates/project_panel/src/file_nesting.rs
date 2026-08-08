use collections::HashMap;
use regex::Regex;
use util::ResultExt as _;

pub struct FileNestingPatterns {
    /// Sorted by parent pattern so that the nesting assignment does not depend
    /// on the iteration order of the settings map.
    patterns: Vec<FileNestingPattern>,
}

struct FileNestingPattern {
    parent_regex: Regex,
    /// Child templates in `regex::Captures::expand` syntax, so `${capture}`
    /// from the settings appears here as `${1}`.
    child_templates: Vec<String>,
}

impl FileNestingPatterns {
    pub fn new(patterns: &HashMap<String, String>) -> Self {
        let mut compiled = patterns
            .iter()
            .filter_map(|(parent_pattern, child_patterns)| {
                let parent_regex = compile_wildcard_pattern(parent_pattern, true)?;
                let child_templates = child_patterns
                    .split(',')
                    .map(|child| child.trim().replace("${capture}", "${1}"))
                    .filter(|child| !child.is_empty())
                    .collect::<Vec<_>>();
                if child_templates.is_empty() {
                    return None;
                }
                Some((
                    parent_pattern.clone(),
                    FileNestingPattern {
                        parent_regex,
                        child_templates,
                    },
                ))
            })
            .collect::<Vec<_>>();
        compiled.sort_by(|(left, _), (right, _)| left.cmp(right));
        Self {
            patterns: compiled.into_iter().map(|(_, pattern)| pattern).collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Given the file names of one directory's children, returns for each name
    /// the index of the name it nests under, if any.
    ///
    /// Names are considered as parents in lexicographic order. A name that is
    /// already nested cannot become a parent, and a name that already has
    /// children cannot become nested, so chains cannot form.
    pub fn nesting_parents(&self, names: &[&str]) -> Vec<Option<usize>> {
        let mut parent_of = vec![None; names.len()];
        if self.patterns.is_empty() {
            return parent_of;
        }

        let name_to_index: HashMap<&str, usize> = names
            .iter()
            .enumerate()
            .map(|(index, name)| (*name, index))
            .collect();
        let mut sorted_indices: Vec<usize> = (0..names.len()).collect();
        sorted_indices.sort_by_key(|index| names[*index]);
        let mut has_children = vec![false; names.len()];

        for &parent_index in &sorted_indices {
            if parent_of[parent_index].is_some() {
                continue;
            }
            for pattern in &self.patterns {
                let Some(captures) = pattern.parent_regex.captures(names[parent_index]) else {
                    continue;
                };
                for child_template in &pattern.child_templates {
                    let mut child_pattern = String::new();
                    captures.expand(child_template, &mut child_pattern);

                    let mut claim = |child_index: usize| {
                        if child_index != parent_index
                            && parent_of[child_index].is_none()
                            && !has_children[child_index]
                        {
                            parent_of[child_index] = Some(parent_index);
                            has_children[parent_index] = true;
                        }
                    };

                    if child_pattern.contains('*') {
                        let Some(child_regex) = compile_wildcard_pattern(&child_pattern, false)
                        else {
                            continue;
                        };
                        for &child_index in &sorted_indices {
                            if child_regex.is_match(names[child_index]) {
                                claim(child_index);
                            }
                        }
                    } else if let Some(&child_index) = name_to_index.get(child_pattern.as_str()) {
                        claim(child_index);
                    }
                }
            }
        }

        parent_of
    }
}

fn compile_wildcard_pattern(pattern: &str, capture: bool) -> Option<Regex> {
    let wildcard = if capture { "(.*)" } else { ".*" };
    let regex_pattern = pattern
        .split('*')
        .map(regex::escape)
        .collect::<Vec<_>>()
        .join(wildcard);
    Regex::new(&format!("^{regex_pattern}$")).log_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterns(entries: &[(&str, &str)]) -> FileNestingPatterns {
        FileNestingPatterns::new(
            &entries
                .iter()
                .map(|(parent, children)| (parent.to_string(), children.to_string()))
                .collect(),
        )
    }

    #[track_caller]
    fn assert_nesting(patterns: &FileNestingPatterns, names: &[&str], expected: &[(&str, &str)]) {
        let parents = patterns.nesting_parents(names);
        let actual: Vec<(&str, &str)> = parents
            .iter()
            .enumerate()
            .filter_map(|(child, parent)| parent.map(|parent| (names[child], names[parent])))
            .collect();
        assert_eq!(actual, expected, "for names {names:?}");
    }

    #[test]
    fn test_exact_child_names() {
        let patterns = patterns(&[("package.json", "package-lock.json, .npmrc")]);
        assert_nesting(
            &patterns,
            &["package.json", "package-lock.json", ".npmrc", "index.js"],
            &[
                ("package-lock.json", "package.json"),
                (".npmrc", "package.json"),
            ],
        );
    }

    #[test]
    fn test_capture_expansion() {
        let patterns = patterns(&[("*.ts", "${capture}.js, ${capture}.d.ts")]);
        assert_nesting(
            &patterns,
            &["foo.ts", "foo.js", "foo.d.ts", "bar.js"],
            &[("foo.js", "foo.ts"), ("foo.d.ts", "foo.ts")],
        );
    }

    #[test]
    fn test_no_chains() {
        // `foo.d.ts` nests `foo.d.js`, so it cannot also become a child of
        // `foo.ts`: the result is two flat groups instead of a chain.
        let patterns = patterns(&[("*.ts", "${capture}.js, ${capture}.d.ts")]);
        assert_nesting(
            &patterns,
            &["foo.ts", "foo.js", "foo.d.ts", "foo.d.js"],
            &[("foo.js", "foo.ts"), ("foo.d.js", "foo.d.ts")],
        );
    }

    #[test]
    fn test_glob_child_pattern() {
        let patterns = patterns(&[("*.go", "${capture}_test.go, ${capture}.*.go")]);
        assert_nesting(
            &patterns,
            &["main.go", "main_test.go", "main.helper.go", "other.go"],
            &[("main_test.go", "main.go"), ("main.helper.go", "main.go")],
        );
    }

    #[test]
    fn test_parent_with_children_cannot_be_claimed() {
        // `a.go` would claim `a.b.go` via the glob, but `a.b.go` already has
        // children of its own by the time `a.go` is considered.
        let patterns = patterns(&[("*.go", "${capture}.*.go")]);
        assert_nesting(
            &patterns,
            &["a.go", "a.b.go", "a.b.c.go"],
            &[("a.b.c.go", "a.b.go")],
        );
    }

    #[test]
    fn test_file_does_not_nest_under_itself() {
        let patterns = patterns(&[("*.js", "${capture}.js, *.js")]);
        assert_nesting(&patterns, &["foo.js", "bar.js"], &[("foo.js", "bar.js")]);
    }

    #[test]
    fn test_invalid_and_empty_patterns_are_ignored() {
        let patterns = patterns(&[("*.rs", ""), ("*.ts", "${capture}.js")]);
        assert_nesting(
            &patterns,
            &["foo.rs", "foo.ts", "foo.js"],
            &[("foo.js", "foo.ts")],
        );
    }
}
