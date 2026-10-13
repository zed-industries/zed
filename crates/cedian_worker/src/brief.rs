//! The worker brief (ADR-0033 decision 1): the checked shape a worktree
//! request or `cedian worker spawn` must carry before anything is created.
//! One validator, so the agent and headless paths cannot disagree.
//!
//! [`WorkerScope`] compiles the briefs' globs into the turn's write scope
//! (ADR-0063): the rule lives here, so the panel never re-implements it.

use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// The required brief fields, in the order a refusal names them.
pub const REQUIRED_FIELDS: [&str; 5] =
    ["goal", "scope.write", "acceptance", "verify", "timebox_min"];

/// Where the worker may write, and where it must not (ADR-0033 decision 3).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Scope {
    pub write: Vec<String>,
    pub deny: Vec<String>,
}

/// One turn's write scope: the compiled globs of one or more briefs
/// (ADR-0063 decision 1–2). `src/**` covers the subtree (`src/lib.rs`,
/// `src/a/b.rs`); a path is worktree-relative with `/` separators. A path is
/// in scope when some brief's `scope.write` matches it and no brief's
/// `scope.deny` does.
#[derive(Debug)]
pub struct WorkerScope {
    briefs: Vec<BriefScope>,
}

#[derive(Debug)]
struct BriefScope {
    /// The running worker whose brief this is (ADR-0063 decision 4): the
    /// union cannot say which worker a path belongs to, so each is named.
    worker: String,
    write: GlobSet,
    deny: GlobSet,
}

impl WorkerScope {
    /// Compile `briefs` — each `(worker id, brief)` — into one scope. A glob
    /// that will not compile is an error naming it, never a silent non-match.
    pub fn new<'a>(
        briefs: impl IntoIterator<Item = (&'a str, &'a Brief)>,
    ) -> Result<Self, ScopeError> {
        let mut out = Vec::new();
        for (worker, brief) in briefs {
            out.push(BriefScope {
                worker: worker.to_string(),
                write: compile(&brief.scope.write)?,
                deny: compile(&brief.scope.deny)?,
            });
        }
        Ok(Self { briefs: out })
    }

    /// Whether `path` (worktree-relative, `/` separators) is writable.
    pub fn covers(&self, path: &str) -> bool {
        self.briefs.iter().any(|b| b.write.is_match(path))
            && !self.briefs.iter().any(|b| b.deny.is_match(path))
    }

    /// The running workers the scope was built from, named in the audit row
    /// a breach writes (ADR-0063 decision 4).
    pub fn workers(&self) -> impl Iterator<Item = &str> {
        self.briefs.iter().map(|b| b.worker.as_str())
    }
}

/// A glob in a brief will not compile; the error names the pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeError {
    /// The pattern that would not compile.
    pub pattern: String,
}

impl std::fmt::Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "brief scope: {:?} is not a valid glob (ADR-0033)",
            self.pattern
        )
    }
}

impl std::error::Error for ScopeError {}

/// Compile globs with `literal_separator`: `*` stays in one path segment,
/// `**` crosses them, so `src/**` is the whole subtree.
fn compile(patterns: &[String]) -> Result<GlobSet, ScopeError> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob = GlobBuilder::new(pattern)
            .literal_separator(true)
            .build()
            .map_err(|_| ScopeError {
                pattern: pattern.clone(),
            })?;
        builder.add(glob);
    }
    builder.build().map_err(|_| ScopeError {
        pattern: patterns.join(", "),
    })
}

/// A checked worker brief; [`Brief::to_json`] is the canonical stored form.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Brief {
    pub goal: String,
    pub scope: Scope,
    pub acceptance: Vec<String>,
    pub verify: Vec<String>,
    pub timebox_min: f64,
    pub context: Vec<String>,
    pub forbidden: Vec<String>,
}

/// Why a brief was refused: fields to name, or a shape that will not parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BriefError {
    /// Every required field that is absent or empty, in ADR order.
    Missing(Vec<String>),
    /// Not JSON, not an object, or a field of the wrong type.
    Malformed(String),
}

impl std::fmt::Display for BriefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(fields) => write!(
                f,
                "no worktree: the brief is missing {} (ADR-0033)",
                fields.join(", ")
            ),
            Self::Malformed(why) => {
                write!(f, "no worktree: the brief is malformed: {why} (ADR-0033)")
            }
        }
    }
}

impl std::error::Error for BriefError {}

impl Brief {
    /// Parse and check a brief from JSON. A refusal names every required
    /// field that is missing or empty (ADR-0033 decision 1).
    pub fn from_json(raw: &str) -> Result<Self, BriefError> {
        let value: Value =
            serde_json::from_str(raw).map_err(|e| BriefError::Malformed(e.to_string()))?;
        let obj = value
            .as_object()
            .ok_or_else(|| BriefError::Malformed("not a JSON object".to_string()))?;

        let goal = req_text(obj.get("goal"), "goal")?;
        let (write, deny) = match obj.get("scope") {
            None => (None, Vec::new()),
            Some(Value::Object(scope)) => (
                req_list(scope.get("write"), "scope.write")?,
                opt_list(scope.get("deny"), "scope.deny")?,
            ),
            Some(_) => return Err(BriefError::Malformed("scope must be an object".to_string())),
        };
        let acceptance = req_list(obj.get("acceptance"), "acceptance")?;
        let verify = req_list(obj.get("verify"), "verify")?;
        let timebox_min = req_positive(obj.get("timebox_min"), "timebox_min")?;
        let context = opt_list(obj.get("context"), "context")?;
        let forbidden = opt_list(obj.get("forbidden"), "forbidden")?;

        let missing: Vec<String> = REQUIRED_FIELDS
            .iter()
            .filter(|field| !match **field {
                "goal" => goal.is_some(),
                "scope.write" => write.is_some(),
                "acceptance" => acceptance.is_some(),
                "verify" => verify.is_some(),
                "timebox_min" => timebox_min.is_some(),
                other => unreachable!("unknown required field {other}"),
            })
            .map(|field| (*field).to_string())
            .collect();
        if !missing.is_empty() {
            return Err(BriefError::Missing(missing));
        }
        Ok(Self {
            goal: goal.expect("present unless named missing"),
            scope: Scope {
                write: write.expect("present unless named missing"),
                deny,
            },
            acceptance: acceptance.expect("present unless named missing"),
            verify: verify.expect("present unless named missing"),
            timebox_min: timebox_min.expect("present unless named missing"),
            context,
            forbidden,
        })
    }

    /// Read and check a brief file (`cedian worker spawn --brief <file>`).
    pub fn from_file(path: &Path) -> Result<Self, BriefError> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| BriefError::Malformed(format!("{}: {e}", path.display())))?;
        Self::from_json(&raw)
    }

    /// The canonical stored JSON for this brief.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("a brief has no unserializable field")
    }

    /// Write this revision to `<state>/briefs/<id>.<revision>.json`. `create_new`
    /// keeps a stored revision immutable — a change is a new revision beside it
    /// (ADR-0033 decision 2) — and returns the path so the caller can remove it
    /// when the spawn it was stored for fails.
    pub fn store(&self, state: &Path, id: &str, revision: u32) -> Result<PathBuf, String> {
        let dir = state.join("briefs");
        let path = dir.join(format!("{id}.{revision}.json"));
        let body = self.to_json();
        std::fs::create_dir_all(&dir)
            .and_then(|()| {
                std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path)
            })
            .and_then(|mut file| {
                std::io::Write::write_all(&mut file, body.as_bytes()).inspect_err(|_| {
                    let _ = std::fs::remove_file(&path);
                })
            })
            .map_err(|e| format!("brief {}: {e}", path.display()))?;
        Ok(path)
    }
}

/// A required non-empty string, or `None` (→ named missing) when absent/blank.
fn req_text(v: Option<&Value>, field: &str) -> Result<Option<String>, BriefError> {
    match v {
        None => Ok(None),
        Some(Value::String(s)) if !s.trim().is_empty() => Ok(Some(s.clone())),
        Some(Value::String(_)) => Ok(None),
        Some(_) => Err(BriefError::Malformed(format!(
            "{field} must be a non-empty string"
        ))),
    }
}

/// A required array of non-empty strings, or `None` when absent/empty/blank.
fn req_list(v: Option<&Value>, field: &str) -> Result<Option<Vec<String>>, BriefError> {
    let Some(v) = v else { return Ok(None) };
    let Value::Array(items) = v else {
        return Err(BriefError::Malformed(format!(
            "{field} must be an array of strings"
        )));
    };
    let strings = string_items(items, field)?;
    Ok((!strings.is_empty() && strings.iter().all(|s| !s.trim().is_empty())).then_some(strings))
}

/// An optional array of strings; absent means empty.
fn opt_list(v: Option<&Value>, field: &str) -> Result<Vec<String>, BriefError> {
    match v {
        None => Ok(Vec::new()),
        Some(Value::Array(items)) => string_items(items, field),
        Some(_) => Err(BriefError::Malformed(format!(
            "{field} must be an array of strings"
        ))),
    }
}

fn string_items(items: &[Value], field: &str) -> Result<Vec<String>, BriefError> {
    items
        .iter()
        .map(|item| {
            item.as_str().map(str::to_string).ok_or_else(|| {
                BriefError::Malformed(format!("{field} must be an array of strings"))
            })
        })
        .collect()
}

/// A required positive number, or `None` (→ named missing) when absent/≤ 0.
fn req_positive(v: Option<&Value>, field: &str) -> Result<Option<f64>, BriefError> {
    match v {
        None => Ok(None),
        Some(Value::Number(n)) => Ok(n.as_f64().filter(|m| *m > 0.0)),
        Some(_) => Err(BriefError::Malformed(format!(
            "{field} must be a positive number"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(v: Value) -> Result<Brief, BriefError> {
        Brief::from_json(&v.to_string())
    }

    fn full() -> Value {
        serde_json::json!({
            "goal": "fix the casing", "scope": {"write": ["notes.txt"]},
            "acceptance": ["notes.txt keeps its casing"], "verify": ["cat notes.txt"],
            "timebox_min": 15,
        })
    }

    #[test]
    fn a_whole_brief_parses_with_defaults() {
        let brief = parse(full()).unwrap();
        assert_eq!(brief.goal, "fix the casing");
        assert_eq!(brief.scope.write, ["notes.txt"]);
        assert_eq!(brief.timebox_min, 15.0);
        assert!(brief.scope.deny.is_empty());
        assert!(brief.context.is_empty());
        assert!(brief.forbidden.is_empty());
    }

    #[test]
    fn each_missing_field_is_named_alone_in_adr_order() {
        for field in REQUIRED_FIELDS {
            let mut v = full();
            let obj = v.as_object_mut().unwrap();
            match field {
                "goal" => obj.remove("goal"),
                "acceptance" => obj.remove("acceptance"),
                "verify" => obj.remove("verify"),
                "timebox_min" => obj.remove("timebox_min"),
                "scope.write" => obj.insert("scope".into(), serde_json::json!({})),
                other => panic!("unhandled field {other}"),
            };
            assert_eq!(
                parse(v).unwrap_err(),
                BriefError::Missing(vec![field.to_string()]),
                "{field}"
            );
        }
    }

    #[test]
    fn an_empty_brief_names_every_field_in_order() {
        assert_eq!(
            parse(serde_json::json!({})).unwrap_err(),
            BriefError::Missing(REQUIRED_FIELDS.iter().map(|f| f.to_string()).collect())
        );
    }

    #[test]
    fn a_wrong_type_is_malformed_and_an_empty_required_list_is_missing() {
        let mut v = full();
        v.as_object_mut()
            .unwrap()
            .insert("scope".into(), serde_json::json!({"write": "notes.txt"}));
        assert!(matches!(parse(v), Err(BriefError::Malformed(_))));

        let mut v = full();
        v.as_object_mut()
            .unwrap()
            .insert("timebox_min".into(), serde_json::json!("15"));
        assert!(matches!(parse(v), Err(BriefError::Malformed(_))));

        let mut v = full();
        v.as_object_mut()
            .unwrap()
            .insert("verify".into(), serde_json::json!([]));
        assert_eq!(
            parse(v).unwrap_err(),
            BriefError::Missing(vec!["verify".to_string()])
        );
    }

    #[test]
    fn optional_lists_parse_when_present() {
        let mut v = full();
        let obj = v.as_object_mut().unwrap();
        obj.insert("context".into(), serde_json::json!(["PR #12"]));
        obj.insert("forbidden".into(), serde_json::json!(["no new deps"]));
        obj.get_mut("scope")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("deny".into(), serde_json::json!(["vendor/**"]));
        let brief = parse(v).unwrap();
        assert_eq!(brief.context, ["PR #12"]);
        assert_eq!(brief.forbidden, ["no new deps"]);
        assert_eq!(brief.scope.deny, ["vendor/**"]);
    }

    fn scope_brief(write: &[&str], deny: &[&str]) -> Brief {
        Brief {
            goal: "g".to_string(),
            scope: Scope {
                write: write.iter().map(|s| (*s).to_string()).collect(),
                deny: deny.iter().map(|s| (*s).to_string()).collect(),
            },
            acceptance: vec!["a".to_string()],
            verify: vec!["v".to_string()],
            timebox_min: 1.0,
            context: Vec::new(),
            forbidden: Vec::new(),
        }
    }

    /// `src/**` covers the subtree; a sibling tree is not in scope.
    #[test]
    fn a_subtree_glob_covers_its_files() {
        let brief = scope_brief(&["src/**"], &[]);
        let scope = WorkerScope::new([("w1", &brief)]).unwrap();
        assert!(scope.covers("src/lib.rs"), "a file one level down");
        assert!(scope.covers("src/a/b.rs"), "a file deeper still");
        assert!(!scope.covers("docs/x.md"), "outside the subtree");
        assert!(!scope.covers("srcs/lib.rs"), "a prefix is not the subtree");
    }

    #[test]
    fn a_deny_glob_wins_over_a_write_glob() {
        let brief = scope_brief(&["src/**"], &["src/secret/**"]);
        let scope = WorkerScope::new([("w1", &brief)]).unwrap();
        assert!(scope.covers("src/lib.rs"));
        assert!(!scope.covers("src/secret/key.rs"), "the deny wins");
    }

    /// Several running briefs union (ADR-0063 decision 1): a path one allows
    /// is in scope, and a path any denies is out.
    #[test]
    fn several_briefs_union() {
        let a = scope_brief(&["src/**"], &[]);
        let b = scope_brief(&["docs/**"], &["docs/private.md"]);
        let scope = WorkerScope::new([("wa", &a), ("wb", &b)]).unwrap();
        assert!(scope.covers("src/lib.rs"), "the first brief allows it");
        assert!(scope.covers("docs/x.md"), "only the second allows it");
        assert!(!scope.covers("docs/private.md"), "the second denies it");
        assert!(!scope.covers("other.txt"), "no brief allows it");
        assert_eq!(
            scope.workers().collect::<Vec<_>>(),
            vec!["wa", "wb"],
            "the scope names the workers it was built from"
        );
    }

    /// A malformed glob is an error naming the pattern, never a non-match.
    #[test]
    fn a_malformed_glob_is_an_error_naming_it() {
        let brief = scope_brief(&["src/**", "src/["], &[]);
        let err = WorkerScope::new([("w1", &brief)]).unwrap_err();
        assert_eq!(err.pattern, "src/[");
        assert!(err.to_string().contains("src/["), "{err}");
        let deny_only = scope_brief(&["src/**"], &["["]);
        assert_eq!(
            WorkerScope::new([("w1", &deny_only)]).unwrap_err().pattern,
            "["
        );
    }

    #[test]
    fn a_brief_file_is_read() {
        let dir = std::env::temp_dir().join(format!("cedian-brief-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("b.json");
        std::fs::write(&path, full().to_string()).unwrap();
        assert_eq!(Brief::from_file(&path).unwrap().goal, "fix the casing");
        assert!(Brief::from_file(&dir.join("absent.json")).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_stored_revision_is_canonical_and_immutable() {
        let dir = std::env::temp_dir().join(format!("cedian-brief-store-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let brief = parse(full()).unwrap();
        let path = brief.store(&dir, "w1", 1).unwrap();
        let stored: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(stored, serde_json::to_value(&brief).unwrap());
        assert_eq!(stored.get("id"), None, "the brief carries no envelope");
        assert!(
            brief.store(&dir, "w1", 1).is_err(),
            "a revision is write-once"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
