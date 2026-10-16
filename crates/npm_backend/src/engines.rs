//! Which Node.js (and npm) a project requires, and whether the active one
//! fits.
//!
//! Two things used to go wrong here. npm writes ranges with spaces and `||`
//! (`>=18.17.1 <19.0.0`, `^18 || ^20`), which the `semver` crate (Cargo's
//! dialect, comma-separated) can't read — and a range it couldn't read was
//! counted as satisfied, so every SPFx project showed "engines OK" whatever
//! Node.js was active. And the npm manager read only `engines.node`, while
//! the Node panel also reads `.nvmrc` and `.node-version`, so the two could
//! disagree about what a project requires.
//!
//! [`npm_range_matches`] reads npm's own range syntax, and answers `None`
//! rather than guess when it can't. [`node_requirement`] finds the
//! requirement the same way version managers do, and names the file it came
//! from. [`EngineCompat`] puts the two together into a verdict a panel can
//! show as it is.

use std::path::Path;

use semver::{Version, VersionReq};

/// Whether `version` falls in the npm range `range`: `Some(true)` or
/// `Some(false)`, or `None` when either can't be read.
///
/// Understands npm's syntax: `||` between alternatives, spaces between the
/// conditions of one alternative (also after an operator: `< 23.0.0`), hyphen
/// ranges (`1.2.3 - 2.3.4`), `x`/`*` wildcards and partial versions (`18`,
/// `18.x`, `18.17`), `^`, `~`, a leading `v`, and an empty range or `*` for
/// "any".
pub fn npm_range_matches(version: &str, range: &str) -> Option<bool> {
    let version = Version::parse(version.trim().trim_start_matches('v')).ok()?;
    let mut any_alternative = false;
    for alternative in range.split("||") {
        let requirement = alternative_to_semver(alternative)?;
        any_alternative = true;
        if requirement.matches(&version) {
            return Some(true);
        }
    }
    any_alternative.then_some(false)
}

/// One `||` alternative of an npm range as a `semver` requirement.
fn alternative_to_semver(alternative: &str) -> Option<VersionReq> {
    let alternative = alternative.trim();
    if alternative.is_empty() || alternative == "*" || alternative.eq_ignore_ascii_case("x") {
        return Some(VersionReq::STAR);
    }

    // `1.2.3 - 2.3.4`: inclusive at both ends.
    if let Some((low, high)) = alternative.split_once(" - ") {
        let low = bare_version(low.trim())?;
        let high = bare_version(high.trim())?;
        return VersionReq::parse(&format!(">={low}, <={high}")).ok();
    }

    // Glue an operator written apart from its version (`< 23.0.0`) back on.
    let mut conditions: Vec<String> = Vec::new();
    let mut pending_operator: Option<&str> = None;
    for token in alternative.split_whitespace() {
        if is_operator(token) {
            pending_operator = Some(token);
            continue;
        }
        let token = match pending_operator.take() {
            Some(operator) => format!("{operator}{token}"),
            None => token.to_string(),
        };
        conditions.push(condition_to_semver(&token)?);
    }
    if pending_operator.is_some() || conditions.is_empty() {
        return None;
    }
    VersionReq::parse(&conditions.join(", ")).ok()
}

fn is_operator(token: &str) -> bool {
    matches!(token, ">=" | "<=" | ">" | "<" | "=" | "^" | "~")
}

/// One condition (`>=18.17.1`, `^18`, `18.x`, `1.2.3`) in `semver`'s syntax.
/// The differences that matter: in npm a bare full version is exact while in
/// `semver` it means `^`, and a bare partial version (`18.17`) means
/// `18.17.x` in npm but `^18.17` in `semver`.
fn condition_to_semver(condition: &str) -> Option<String> {
    let split_at = condition
        .find(|c: char| c.is_ascii_digit() || c == 'v' || c == 'x' || c == 'X' || c == '*')
        .unwrap_or(condition.len());
    let (operator, version) = condition.split_at(split_at);
    // Anything else in front (`lts/` of `lts/*`) isn't a range at all.
    if !matches!(operator, "" | "=" | ">=" | "<=" | ">" | "<" | "^" | "~") {
        return None;
    }
    let version = version.trim_start_matches('v');
    if version.is_empty() {
        return None;
    }
    let parts: Vec<&str> = version.split('.').collect();
    let wildcard = |part: &str| matches!(part, "x" | "X" | "*");
    let numeric = |part: &&str| part.chars().next().is_some_and(|c| c.is_ascii_digit());
    if parts.len() > 3 || parts.is_empty() {
        return None;
    }

    // `18`, `18.17`, `18.x`, `18.17.*`: everything below the given parts.
    let given = parts.iter().take_while(|part| !wildcard(part)).count();
    if given < 3 {
        if !parts[..given].iter().all(numeric) || parts[given..].iter().any(|part| !wildcard(part))
        {
            return None;
        }
        if given == 0 {
            return Some("*".to_string());
        }
        let stem = parts[..given].join(".");
        return Some(match operator {
            "" | "=" => format!("{stem}.*"),
            // `^18.x`, `~18`, `>=18` read the missing parts as zeros.
            _ => format!("{operator}{stem}"),
        });
    }

    Version::parse(version).ok()?;
    Some(match operator {
        "" => format!("={version}"),
        _ => format!("{operator}{version}"),
    })
}

/// A full version written plainly (`18.17.1`, `v18.17.1`), as the ends of a
/// hyphen range must be.
fn bare_version(text: &str) -> Option<String> {
    let text = text.trim_start_matches('v');
    Version::parse(text).ok().map(|_| text.to_string())
}

/// What a project requires of Node.js, and where it says so.
#[derive(serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct NodeRequirement {
    /// As written: `>=18.17.1 <19.0.0`, `20`, `lts/iron`.
    pub range: String,
    /// The file it came from, relative to the project: `.nvmrc`,
    /// `package.json engines`, or a parent folder's (`../.nvmrc`).
    pub source: String,
}

/// The Node.js `project` requires, read the way version managers read it:
/// in the project's folder, then each folder above it, the first of
/// `.nvmrc`, `.node-version` or `engines.node` in `package.json`.
pub fn node_requirement(project: &Path) -> Option<NodeRequirement> {
    for (depth, dir) in project.ancestors().enumerate() {
        let prefix = "../".repeat(depth);
        for file in [".nvmrc", ".node-version"] {
            let Ok(text) = std::fs::read_to_string(dir.join(file)) else {
                continue;
            };
            // The first line that isn't a comment, as nvm reads it.
            if let Some(line) = text
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty() && !line.starts_with('#'))
            {
                return Some(NodeRequirement {
                    range: line.to_string(),
                    source: format!("{prefix}{file}"),
                });
            }
        }
        let engines_node = std::fs::read_to_string(dir.join("package.json"))
            .ok()
            .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
            .and_then(|json| {
                json.pointer("/engines/node")
                    .and_then(|node| node.as_str())
                    .map(str::to_string)
            });
        if let Some(range) = engines_node {
            return Some(NodeRequirement {
                range,
                source: format!("{prefix}package.json engines"),
            });
        }
    }
    None
}

/// How the active version compares with what is required.
#[derive(serde::Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequirementCheck {
    /// Nothing is required.
    NotDeclared,
    /// The active version fits.
    Matches,
    /// The active version doesn't fit.
    Mismatch,
    /// The requirement couldn't be read (a version manager alias such as
    /// `lts/iron`, or something malformed), so nothing is claimed either way.
    Unreadable,
}

fn check(version: &str, range: Option<&str>) -> RequirementCheck {
    let Some(range) = range.map(str::trim).filter(|range| !range.is_empty()) else {
        return RequirementCheck::NotDeclared;
    };
    match npm_range_matches(version, range) {
        Some(true) => RequirementCheck::Matches,
        Some(false) => RequirementCheck::Mismatch,
        None => RequirementCheck::Unreadable,
    }
}

/// The active Node.js and npm against what the project requires.
#[derive(serde::Serialize, Clone, Debug)]
pub struct EngineCompat {
    pub node: RequirementCheck,
    pub npm: RequirementCheck,
    pub node_requirement: Option<NodeRequirement>,
    /// `engines.npm` from `package.json`, when there is one.
    pub npm_range: Option<String>,
    pub node_version: String,
    pub npm_version: String,
}

/// The check for `project` with Node.js `node_version` and npm `npm_version`
/// active.
pub fn engine_compat(project: &Path, node_version: &str, npm_version: &str) -> EngineCompat {
    let node_requirement = node_requirement(project);
    let npm_range = std::fs::read_to_string(project.join("package.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|json| {
            json.pointer("/engines/npm")
                .and_then(|npm| npm.as_str())
                .map(str::to_string)
        });
    EngineCompat {
        node: check(
            node_version,
            node_requirement
                .as_ref()
                .map(|requirement| requirement.range.as_str()),
        ),
        npm: check(npm_version, npm_range.as_deref()),
        node_requirement,
        npm_range,
        node_version: node_version.to_string(),
        npm_version: npm_version.to_string(),
    }
}

/// How serious a verdict is, for its colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Fine,
    Unknown,
    Problem,
}

impl EngineCompat {
    /// One line for the panel: what is active, and how it compares.
    pub fn summary(&self) -> (Verdict, String) {
        let node = &self.node_version;
        let requirement = self.node_requirement.as_ref();
        let node_part = match (self.node, requirement) {
            (RequirementCheck::Matches, Some(req)) => {
                format!("Node.js {node} matches {} ({})", req.range, req.source)
            }
            (RequirementCheck::Mismatch, Some(req)) => format!(
                "Node.js {node} isn't supported: this project requires {} ({})",
                req.range, req.source
            ),
            (RequirementCheck::Unreadable, Some(req)) => format!(
                "Node.js {node}: couldn't check the required version {} ({})",
                req.range, req.source
            ),
            _ => format!("Node.js {node}: no required version declared"),
        };
        let npm_part = match (self.npm, self.npm_range.as_deref()) {
            (RequirementCheck::Mismatch, Some(range)) => Some(format!(
                "npm {} isn't supported: requires {range}",
                self.npm_version
            )),
            (RequirementCheck::Unreadable, Some(range)) => Some(format!(
                "npm {}: couldn't check the required version {range}",
                self.npm_version
            )),
            _ => None,
        };
        let verdict =
            if self.node == RequirementCheck::Mismatch || self.npm == RequirementCheck::Mismatch {
                Verdict::Problem
            } else if self.node == RequirementCheck::Unreadable
                || self.npm == RequirementCheck::Unreadable
            {
                Verdict::Unknown
            } else {
                Verdict::Fine
            };
        let text = match npm_part {
            Some(npm) => format!("{node_part} \u{2014} {npm}"),
            None => format!("{node_part}  |  npm {}", self.npm_version),
        };
        (verdict, text)
    }

    /// Why installing now is risky, when it is: the active Node.js or npm
    /// doesn't fit what the project requires. `None` when it fits, or when
    /// that can't be told.
    pub fn install_warning(&self) -> Option<String> {
        let mut problems = Vec::new();
        if self.node == RequirementCheck::Mismatch
            && let Some(req) = &self.node_requirement
        {
            problems.push(format!(
                "The active Node.js is {}, but this project requires {} ({}).",
                self.node_version, req.range, req.source
            ));
        }
        if self.npm == RequirementCheck::Mismatch
            && let Some(range) = &self.npm_range
        {
            problems.push(format!(
                "The active npm is {}, but this project requires {range} (package.json engines).",
                self.npm_version
            ));
        }
        (!problems.is_empty()).then(|| {
            problems.push(
                "Packages installed now may not match what the project builds with. \
                 Pick a matching version in the Node panel first, or continue anyway."
                    .to_string(),
            );
            problems.join("\n\n")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npm_ranges_are_read_as_npm_writes_them() {
        let cases: &[(&str, &str, Option<bool>)] = &[
            // The two ranges every SPFx project declares.
            ("18.20.4", ">=18.17.1 <19.0.0", Some(true)),
            ("22.22.3", ">=18.17.1 <19.0.0", Some(false)),
            ("22.22.3", ">=22.14.0 < 23.0.0", Some(true)),
            ("26.11.1", ">=22.14.0 < 23.0.0", Some(false)),
            ("22.10.0", ">=22.14.0 < 23.0.0", Some(false)),
            // Alternatives.
            ("20.11.0", "^18 || ^20", Some(true)),
            ("22.0.0", "^18 || ^20", Some(false)),
            // Partial versions and wildcards.
            ("18.20.4", "18", Some(true)),
            ("19.0.0", "18", Some(false)),
            ("18.17.9", "18.17", Some(true)),
            ("18.18.0", "18.17", Some(false)),
            ("18.20.4", "18.x", Some(true)),
            ("20.0.0", "*", Some(true)),
            ("20.0.0", "", Some(true)),
            // A full version on its own is exact in npm.
            ("18.17.1", "18.17.1", Some(true)),
            ("18.17.2", "18.17.1", Some(false)),
            ("18.17.1", "v18.17.1", Some(true)),
            // Hyphen ranges are inclusive.
            ("20.0.0", "18.0.0 - 20.0.0", Some(true)),
            ("20.0.1", "18.0.0 - 20.0.0", Some(false)),
            // Caret, tilde and plain comparisons.
            ("18.99.0", "^18.17.0", Some(true)),
            ("18.17.5", "~18.17.0", Some(true)),
            ("18.18.0", "~18.17.0", Some(false)),
            ("20.11.0", ">=18", Some(true)),
            ("16.20.0", ">=18", Some(false)),
            // Never a guess.
            ("20.11.0", "lts/iron", None),
            ("20.11.0", "lts/*", None),
            ("20.11.0", "node", None),
            ("20.11.0", ">= banana", None),
            ("20.11.0", ">=", None),
            ("v20", ">=18", None),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                npm_range_matches(version, range),
                *expected,
                "{version} in {range:?}"
            );
        }
    }

    fn project(files: &[(&str, &str)]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for (name, content) in files {
            let path = dir.path().join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        }
        dir
    }

    #[test]
    fn the_requirement_comes_from_the_first_file_that_states_one() {
        let dir = project(&[
            (
                "package.json",
                r#"{"engines": {"node": ">=22.14.0 < 23.0.0"}}"#,
            ),
            (".nvmrc", "# pinned for the team\n\n20\n"),
        ]);
        let requirement = node_requirement(dir.path()).unwrap();
        assert_eq!(requirement.range, "20");
        assert_eq!(requirement.source, ".nvmrc");

        std::fs::remove_file(dir.path().join(".nvmrc")).unwrap();
        let requirement = node_requirement(dir.path()).unwrap();
        assert_eq!(requirement.range, ">=22.14.0 < 23.0.0");
        assert_eq!(requirement.source, "package.json engines");
    }

    #[test]
    fn a_parent_folders_requirement_applies_and_says_so() {
        let dir = project(&[
            (".node-version", "18.20.4\n"),
            ("packages/app/package.json", "{}"),
        ]);
        let requirement = node_requirement(&dir.path().join("packages").join("app")).unwrap();
        assert_eq!(requirement.range, "18.20.4");
        assert_eq!(requirement.source, "../../.node-version");
    }

    #[test]
    fn verdicts_say_what_they_know_and_no_more() {
        let spfx_120 = project(&[(
            "package.json",
            r#"{"engines": {"node": ">=18.17.1 <19.0.0"}}"#,
        )]);
        let check = engine_compat(spfx_120.path(), "22.22.3", "10.9.9");
        assert_eq!(check.node, RequirementCheck::Mismatch);
        let (verdict, text) = check.summary();
        assert_eq!(verdict, Verdict::Problem);
        assert_eq!(
            text,
            "Node.js 22.22.3 isn't supported: this project requires >=18.17.1 <19.0.0 \
             (package.json engines)  |  npm 10.9.9"
        );
        assert!(
            check
                .install_warning()
                .unwrap()
                .contains("requires >=18.17.1 <19.0.0")
        );

        let fits = engine_compat(spfx_120.path(), "18.20.4", "10.8.2");
        assert_eq!(fits.summary().0, Verdict::Fine);
        assert!(fits.install_warning().is_none());

        let undeclared = project(&[("package.json", "{}")]);
        let check = engine_compat(undeclared.path(), "22.22.3", "10.9.9");
        assert_eq!(check.node, RequirementCheck::NotDeclared);
        assert_eq!(
            check.summary(),
            (
                Verdict::Fine,
                "Node.js 22.22.3: no required version declared  |  npm 10.9.9".to_string()
            )
        );
        assert!(check.install_warning().is_none());

        let alias = project(&[(".nvmrc", "lts/iron\n")]);
        let check = engine_compat(alias.path(), "22.22.3", "10.9.9");
        assert_eq!(check.node, RequirementCheck::Unreadable);
        assert_eq!(check.summary().0, Verdict::Unknown);
        assert!(check.install_warning().is_none(), "no warning on a guess");

        let old_npm = project(&[("package.json", r#"{"engines": {"npm": ">=11"}}"#)]);
        let check = engine_compat(old_npm.path(), "22.22.3", "10.9.9");
        assert_eq!(check.npm, RequirementCheck::Mismatch);
        assert_eq!(check.summary().0, Verdict::Problem);
        assert!(
            check
                .install_warning()
                .unwrap()
                .contains("The active npm is 10.9.9")
        );
    }
}
