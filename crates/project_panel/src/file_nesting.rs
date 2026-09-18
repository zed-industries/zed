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
    child_patterns: Vec<ChildPattern>,
}

/// A child template split once on `*`, so matching never re-splits or
/// recompiles per parent. A single segment means an exact name; two segments
/// match by literal prefix/suffix without any regex; more segments fall back
/// to a compiled regex.
struct ChildPattern {
    segments: Vec<String>,
}

impl ChildPattern {
    fn has_wildcard(&self) -> bool {
        self.segments.len() > 1
    }
}

impl FileNestingPatterns {
    pub fn new(patterns: &HashMap<String, String>) -> Self {
        let mut compiled = patterns
            .iter()
            .filter_map(|(parent_pattern, child_patterns)| {
                let parent_regex = compile_wildcard_pattern(parent_pattern, true)?;
                let child_patterns = child_patterns
                    .split(',')
                    .map(|child| child.trim().to_string())
                    .filter(|child| !child.is_empty())
                    .map(|child| ChildPattern {
                        segments: child.split('*').map(str::to_string).collect(),
                    })
                    .collect::<Vec<_>>();
                if child_patterns.is_empty() {
                    return None;
                }
                Some((
                    parent_pattern.clone(),
                    FileNestingPattern {
                        parent_regex,
                        child_patterns,
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
    /// Like VS Code, the nesting relation is transitive and flattened to at
    /// most two levels: when `foo.js` nests under `foo.ts` and `foo.min.js`
    /// nests under `foo.js`, both files end up grouped directly under `foo.ts`.
    /// A file that is both nested and a parent of others is still displayed as
    /// a flat child of its root ancestor. Cycles (including mutual nesting)
    /// resolve to no nesting rather than hanging or panicking.
    pub fn nesting_parents(&self, names: &[&str], dirname: &str) -> Vec<Option<usize>> {
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

        // Direct nesting edges, child -> parents. A file may appear here both
        // as a child and as a parent; flattening to roots happens below.
        let mut direct_parents: Vec<Vec<usize>> = vec![Vec::new(); names.len()];
        let mut add_edge = |child_index: usize, parent_index: usize| {
            if child_index != parent_index && !direct_parents[child_index].contains(&parent_index) {
                direct_parents[child_index].push(parent_index);
            }
        };

        for &parent_index in &sorted_indices {
            for pattern in &self.patterns {
                let Some(captures) = pattern.parent_regex.captures(names[parent_index]) else {
                    continue;
                };
                for child_pattern in &pattern.child_patterns {
                    if !child_pattern.has_wildcard() {
                        let child_name = expand_child_template(
                            &child_pattern.segments[0],
                            &captures,
                            names[parent_index],
                            dirname,
                        );
                        if let Some(&child_index) = name_to_index.get(child_name.as_str()) {
                            add_edge(child_index, parent_index);
                        }
                    } else {
                        // The matcher is built once per parent; scanning every
                        // file must not re-expand or recompile per candidate.
                        let Some(matcher) = child_matcher(
                            &child_pattern.segments,
                            &captures,
                            names[parent_index],
                            dirname,
                        ) else {
                            continue;
                        };
                        for &child_index in &sorted_indices {
                            if matcher.matches(names[child_index]) {
                                add_edge(child_index, parent_index);
                            }
                        }
                    }
                }
            }
        }

        for child_index in 0..names.len() {
            let mut roots =
                Self::matrix_root_ancestors(child_index, &direct_parents, &mut Vec::new());
            roots.sort_unstable();
            roots.dedup();
            // A file never nests under itself; with no remaining root it is top-level.
            roots.retain(|root| *root != child_index);
            if roots.is_empty() {
                continue;
            }
            // A file can match several unrelated parents; VS Code shows it under
            // each of them. This panel only supports one parent per file, so pick
            // deterministically by name.
            roots.sort_by_key(|root| names[*root]);
            parent_of[child_index] = Some(roots[0]);
        }

        parent_of
    }

    /// All root ancestors of `file` from a precomputed direct-parent matrix.
    /// `stack` guards against cycles, mirroring VS Code's
    /// `findAllRootAncestors`.
    fn matrix_root_ancestors(
        file: usize,
        direct_parents: &[Vec<usize>],
        stack: &mut Vec<usize>,
    ) -> Vec<usize> {
        if stack.contains(&file) {
            return Vec::new();
        }
        if direct_parents[file].is_empty() {
            return vec![file];
        }
        stack.push(file);
        let mut roots = Vec::new();
        for &parent in &direct_parents[file] {
            roots.extend(Self::matrix_root_ancestors(parent, direct_parents, stack));
        }
        stack.pop();
        roots
    }

    /// Index of the file `names[file_index]` nests under, if any, resolving
    /// transitively to the root ancestor like [`nesting_parents`].
    ///
    /// Only the ancestors of one file are explored instead of mapping the
    /// whole directory, so revealing a file stays cheap enough for the UI
    /// thread even in large directories.
    pub fn nesting_parent_of(
        &self,
        names: &[&str],
        file_index: usize,
        dirname: &str,
    ) -> Option<usize> {
        if self.patterns.is_empty() || file_index >= names.len() {
            return None;
        }
        let mut roots = self.root_ancestors_of(names, file_index, dirname, &mut Vec::new());
        roots.sort_unstable();
        roots.dedup();
        roots.retain(|root| *root != file_index);
        if roots.is_empty() {
            return None;
        }
        roots.sort_by_key(|root| names[*root]);
        Some(roots[0])
    }

    /// Direct parents of one file: every other file whose patterns claim it.
    fn claiming_parents(&self, names: &[&str], child_index: usize, dirname: &str) -> Vec<usize> {
        let child_name = names[child_index];
        let mut parents = Vec::new();
        for (parent_index, parent_name) in names.iter().enumerate() {
            if parent_index == child_index {
                continue;
            }
            for pattern in &self.patterns {
                let Some(captures) = pattern.parent_regex.captures(parent_name) else {
                    continue;
                };
                let claimed = pattern.child_patterns.iter().any(|child_pattern| {
                    if !child_pattern.has_wildcard() {
                        expand_child_template(
                            &child_pattern.segments[0],
                            &captures,
                            parent_name,
                            dirname,
                        ) == child_name
                    } else {
                        child_matcher(&child_pattern.segments, &captures, parent_name, dirname)
                            .is_some_and(|matcher| matcher.matches(child_name))
                    }
                });
                if claimed {
                    parents.push(parent_index);
                    break;
                }
            }
        }
        parents
    }

    /// Root ancestors of `file`, resolving direct parents on demand.
    /// `stack` guards against cycles, mirroring VS Code's
    /// `findAllRootAncestors`.
    fn root_ancestors_of(
        &self,
        names: &[&str],
        file: usize,
        dirname: &str,
        stack: &mut Vec<usize>,
    ) -> Vec<usize> {
        if stack.contains(&file) {
            return Vec::new();
        }
        let direct = self.claiming_parents(names, file, dirname);
        if direct.is_empty() {
            return vec![file];
        }
        stack.push(file);
        let mut roots = Vec::new();
        for parent in direct {
            roots.extend(self.root_ancestors_of(names, parent, dirname, stack));
        }
        stack.pop();
        roots
    }
}

fn expand_child_template(
    template: &str,
    captures: &regex::Captures,
    parent_name: &str,
    dirname: &str,
) -> String {
    let capture = captures.get(1).map_or("", |capture| capture.as_str());
    let (basename, extname) = parent_name
        .rfind('.')
        .filter(|index| *index > 0)
        .map_or((parent_name, ""), |index| {
            (&parent_name[..index], &parent_name[index + 1..])
        });
    let substitutions = [
        ("${capture}", capture),
        ("$(capture)", capture),
        ("${basename}", basename),
        ("$(basename)", basename),
        ("${extname}", extname),
        ("$(extname)", extname),
        ("${dirname}", dirname),
        ("$(dirname)", dirname),
    ];
    let mut result = String::with_capacity(template.len());
    let mut offset = 0;
    while let Some(suffix) = template.get(offset..)
        && !suffix.is_empty()
    {
        if let Some((token, replacement)) = substitutions
            .iter()
            .find(|(token, _)| suffix.starts_with(token))
        {
            result.push_str(replacement);
            offset += token.len();
        } else if let Some(character) = suffix.chars().next() {
            result.push(character);
            offset += character.len_utf8();
        }
    }
    result
}

/// A child template expanded for one parent file, ready to test candidates.
/// Built once per parent so directory scans never re-expand or recompile per
/// candidate file. A single `*` matches by literal prefix/suffix without any
/// regex; multiple `*` fall back to a compiled regex. Substituted captures
/// stay literal in both cases.
enum ChildMatcher {
    Affix { prefix: String, suffix: String },
    Pattern(Regex),
}

impl ChildMatcher {
    fn matches(&self, child_name: &str) -> bool {
        match self {
            Self::Affix { prefix, suffix } => {
                child_name.len() >= prefix.len() + suffix.len()
                    && child_name.starts_with(prefix.as_str())
                    && child_name.ends_with(suffix.as_str())
            }
            Self::Pattern(regex) => regex.is_match(child_name),
        }
    }
}

fn child_matcher(
    segments: &[String],
    captures: &regex::Captures,
    parent_name: &str,
    dirname: &str,
) -> Option<ChildMatcher> {
    if segments.len() == 2 {
        Some(ChildMatcher::Affix {
            prefix: expand_child_template(segments[0].as_str(), captures, parent_name, dirname),
            suffix: expand_child_template(segments[1].as_str(), captures, parent_name, dirname),
        })
    } else {
        let template = segments.join("*");
        Some(ChildMatcher::Pattern(compile_child_pattern(
            &template,
            captures,
            parent_name,
            dirname,
        )?))
    }
}

fn compile_child_pattern(
    template: &str,
    captures: &regex::Captures,
    parent_name: &str,
    dirname: &str,
) -> Option<Regex> {
    let regex_pattern = template
        .split('*')
        .map(|segment| {
            regex::escape(&expand_child_template(
                segment,
                captures,
                parent_name,
                dirname,
            ))
        })
        .collect::<Vec<_>>()
        .join(".*");
    Regex::new(&format!("^{regex_pattern}$")).log_err()
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
        assert_nesting_in_dir(patterns, names, "", expected);
    }

    #[track_caller]
    fn assert_nesting_in_dir(
        patterns: &FileNestingPatterns,
        names: &[&str],
        dirname: &str,
        expected: &[(&str, &str)],
    ) {
        let parents = patterns.nesting_parents(names, dirname);
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
    fn test_file_attribute_expansion() {
        let patterns = patterns(&[("*.test.ts", "${basename}.js, ${extname}.config")]);
        assert_nesting(
            &patterns,
            &["widget.test.ts", "widget.test.js", "ts.config"],
            &[
                ("widget.test.js", "widget.test.ts"),
                ("ts.config", "widget.test.ts"),
            ],
        );
    }

    #[test]
    fn test_literal_dollar_in_pattern() {
        // `$schema.json` is a literal file name, not a substitution: only
        // `schema.json` nests, and a file named `.json` must not.
        let patterns = patterns(&[("$schema.json", "schema.json")]);
        assert_nesting(
            &patterns,
            &["$schema.json", "schema.json", ".json"],
            &[("schema.json", "$schema.json")],
        );
    }

    #[test]
    fn test_literal_star_in_filename() {
        // `a*.ts` contains a literal star; the substituted capture stays
        // literal and must not collect the unrelated `abc.js`.
        let patterns = patterns(&[("*.ts", "${capture}.js")]);
        assert_nesting(
            &patterns,
            &["a*.ts", "a*.js", "abc.js"],
            &[("a*.js", "a*.ts")],
        );
    }

    #[test]
    fn test_substituted_wildcard_is_literal() {
        let patterns = patterns(&[("index.ts", "${dirname}.config")]);
        assert_nesting_in_dir(
            &patterns,
            &["index.ts", "a*b.config", "axxb.config"],
            "a*b",
            &[("a*b.config", "index.ts")],
        );
    }

    #[test]
    fn test_transitive_nesting_flattens_to_root() {
        // Like VS Code, nesting is transitive and flattened: `foo.min.js`
        // nests under `foo.js`, which nests under `foo.ts`, so both generated
        // files end up grouped directly under `foo.ts`.
        let patterns = patterns(&[("*.ts", "${capture}.js"), ("*.js", "${capture}.min.js")]);
        assert_nesting(
            &patterns,
            &["foo.ts", "foo.js", "foo.min.js"],
            &[("foo.js", "foo.ts"), ("foo.min.js", "foo.ts")],
        );
    }

    #[test]
    fn test_no_chains() {
        // `foo.d.ts` nests under `foo.ts` and `foo.d.js` nests under `foo.d.ts`,
        // so transitively everything flattens under the root `foo.ts` instead
        // of forming a displayed chain.
        let patterns = patterns(&[("*.ts", "${capture}.js, ${capture}.d.ts")]);
        assert_nesting(
            &patterns,
            &["foo.ts", "foo.js", "foo.d.ts", "foo.d.js"],
            &[
                ("foo.js", "foo.ts"),
                ("foo.d.ts", "foo.ts"),
                ("foo.d.js", "foo.ts"),
            ],
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
    fn test_parent_with_children_nests_transitively() {
        // `a.b.go` nests under `a.go` and `a.b.c.go` nests under both, so
        // transitively everything flattens under the root `a.go`.
        let patterns = patterns(&[("*.go", "${capture}.*.go")]);
        assert_nesting(
            &patterns,
            &["a.go", "a.b.go", "a.b.c.go"],
            &[("a.b.go", "a.go"), ("a.b.c.go", "a.go")],
        );
    }

    #[test]
    fn test_file_does_not_nest_under_itself() {
        // Each file claims the other, forming a cycle. Cycles resolve to no
        // nesting rather than an arbitrary winner.
        let patterns = patterns(&[("*.js", "${capture}.js, *.js")]);
        assert_nesting(&patterns, &["foo.js", "bar.js"], &[]);
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
