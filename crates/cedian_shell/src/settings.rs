//! Settings: the user's `cedian.toml`, the one settings and policy file
//! (ADR-0018). Permissions, reviewer allow-list, update channel and the
//! workflow gate floor.
//!
//! The file belongs to the user, never to a workspace: `$CEDIAN_CONFIG`, else
//! `$XDG_CONFIG_HOME/cedian/cedian.toml`, else `~/.config/cedian/cedian.toml`.
//! Nothing inside a workspace is read for settings, so a repository cannot
//! loosen policy or opt itself in to anything (ADR-0035, §77). A settings
//! file found in the workspace is refused, never silently ignored.
//!
//! Unknown keys at any depth, a missing or wrong `schema`, and a bad floor
//! gate fail closed: a typo'd policy must never silently become permissive.

use cedian_workflow::{FloorRuleSpec, GateFloor};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Current settings schema (P3). Required in the file. Bump on any breaking
/// shape change.
pub const SETTINGS_SCHEMA: u32 = 1;

/// An OMP model role name (`modelRoles` key). Defaults to `review`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewRole(pub String);

impl Default for ReviewRole {
    fn default() -> Self {
        Self("review".to_string())
    }
}

impl ReviewRole {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub const SETTINGS_FILE: &str = "cedian.toml";
/// Env override for the settings path. Set but missing is an error.
pub const CONFIG_ENV: &str = "CEDIAN_CONFIG";
/// The pre-row-E headless file, refused wherever it is found.
const LEGACY_FILE: &str = "cedian.json";

/// Permission tier verdicts (canonical: Allow | Ask | Deny; Abstain is
/// system-generated, never persisted here).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Allow,
    Ask,
    Deny,
}

/// `[permissions]` tiers (§64: safe / project_write / dangerous).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Permissions {
    pub safe: Verdict,
    pub project_write: Verdict,
    pub dangerous: Verdict,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            safe: Verdict::Allow,
            project_write: Verdict::Allow,
            dangerous: Verdict::Ask,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateChannel {
    #[default]
    Stable,
    Beta,
}

/// Who decides approvals and `computer` in a workspace (ADR-0035).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Policy {
    /// The default spawn profile: cedian makes OMP's resolver strict.
    #[default]
    Cedian,
    /// The user's own OMP config: approval mode and `computer`.
    Omp,
}

/// Whether a person is there to see the run. Unattended runs (reviewers,
/// automations) never get `Policy::Omp` (ADR-0035 decision 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunKind {
    Interactive,
    Unattended,
}

/// The policy a run gets, and why a project key did not apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyChoice {
    pub policy: Policy,
    pub notes: Vec<String>,
}

/// Validated settings. Only [`parse_settings`] builds one from text, so a
/// `Settings` in hand passed the schema check and every floor gate passed
/// `Gate::register`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Settings {
    pub permissions: Permissions,
    /// Reviewer allow-list: shell commands reviewers may run (S3).
    pub reviewer_allow_list: Vec<String>,
    /// `[review] role`: the OMP model role reviewers run on (ADR-0039).
    /// cedian stores role names only; OMP's `modelRoles` maps them to
    /// models. Default `review`.
    pub review_role: ReviewRole,
    pub update_channel: UpdateChannel,
    /// Gates cedian requires per task kind × risk. Empty = fast lane
    /// (ADR-0026).
    pub floor: GateFloor,
    /// `[projects."<path>"]` as written; read through [`Settings::policy_for`].
    projects: BTreeMap<String, Policy>,
    /// The file these settings came from (canonical), if any.
    source: Option<PathBuf>,
}

impl Settings {
    /// The policy `workdir` (canonical) runs under. Fails closed to
    /// `Cedian` with a note on any doubt (ADR-0035 decision 6): a key that is
    /// not absolute, does not exist or is not canonical as written (a symlink
    /// in it could be repointed); two keys for this workspace that disagree;
    /// a settings file that lives inside the workspace itself.
    pub fn policy_for(&self, workdir: &Path, run: RunKind) -> PolicyChoice {
        let mut notes = Vec::new();
        let mut matched = Vec::new();
        for (key, value) in &self.projects {
            let path = Path::new(key);
            let why = match path.canonicalize() {
                _ if !path.is_absolute() => "not an absolute path".to_string(),
                Ok(canonical) if canonical == path => {
                    if canonical == workdir {
                        matched.push(*value);
                    }
                    continue;
                }
                Ok(canonical) => format!("not canonical; write it as {:?}", canonical.display()),
                Err(e) => e.to_string(),
            };
            notes.push(format!(
                "[projects.{key:?}] ignored: {why} (default policy)"
            ));
        }
        let mut policy = match matched.as_slice() {
            [] => Policy::Cedian,
            [first, rest @ ..] if rest.iter().all(|p| p == first) => *first,
            _ => {
                notes.push("two [projects] keys name this workspace with different policies: default policy".to_string());
                Policy::Cedian
            }
        };
        if policy == Policy::Omp {
            let inside = self
                .source
                .as_deref()
                .is_some_and(|src| src.starts_with(workdir));
            if inside {
                notes.push("the settings file is inside this workspace, so it cannot opt it in: default policy".to_string());
                policy = Policy::Cedian;
            } else if run == RunKind::Unattended {
                notes.push(
                    "policy = \"omp\" does not apply to unattended runs (ADR-0035): default policy"
                        .to_string(),
                );
                policy = Policy::Cedian;
            }
        }
        PolicyChoice { policy, notes }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSettings {
    schema: Option<u32>,
    #[serde(default)]
    permissions: Permissions,
    #[serde(default)]
    reviewer_allow_list: Vec<String>,
    #[serde(default)]
    update_channel: UpdateChannel,
    #[serde(default)]
    workflow: RawWorkflow,
    #[serde(default)]
    projects: BTreeMap<String, RawProject>,
    #[serde(default)]
    review: RawReview,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReview {
    role: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProject {
    policy: Policy,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWorkflow {
    #[serde(default)]
    floor: Vec<FloorRuleSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsError {
    /// TOML syntax, a wrong type, or an unknown key (the message names it).
    Parse(String),
    BadSchema(Option<u32>),
    BadFloor(String),
    /// A `cedian.json` from before ADR-0018.
    Legacy {
        found: PathBuf,
        user: Option<PathBuf>,
    },
    /// A settings file inside the workspace (§77).
    InWorkspace {
        found: PathBuf,
        user: Option<PathBuf>,
    },
    /// `CEDIAN_CONFIG` names a file that does not exist.
    Missing(PathBuf),
    Read {
        path: PathBuf,
        error: String,
    },
    /// A parse error in the file at `path`.
    InFile {
        path: PathBuf,
        error: Box<SettingsError>,
    },
}

fn user_path_hint(user: &Option<PathBuf>) -> String {
    match user {
        Some(path) => format!("{}", path.display()),
        None => format!("${CONFIG_ENV} or ~/.config/cedian/{SETTINGS_FILE}"),
    }
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "settings parse error: {e}"),
            Self::BadSchema(None) => write!(
                f,
                "settings file has no `schema` key — add `schema = {SETTINGS_SCHEMA}` as the first line"
            ),
            Self::BadSchema(Some(v)) => write!(
                f,
                "settings schema {v} not supported (want {SETTINGS_SCHEMA}) — fix or regenerate the file"
            ),
            Self::BadFloor(e) => write!(f, "bad [[workflow.floor]]: {e}"),
            Self::Legacy { found, user } => write!(
                f,
                "{} is no longer read: settings live in the user's {SETTINGS_FILE}. \
                 Move its keys to {} as TOML with `schema = {SETTINGS_SCHEMA}` on the first line, \
                 then delete {} (ADR-0018)",
                found.display(),
                user_path_hint(user),
                found.display()
            ),
            Self::InWorkspace { found, user } => write!(
                f,
                "{} is inside the workspace and is not read: settings live only in the user's \
                 {SETTINGS_FILE} ({}), so a repository cannot set policy (§77, ADR-0035). \
                 Move it there or delete it",
                found.display(),
                user_path_hint(user)
            ),
            Self::Missing(path) => {
                write!(
                    f,
                    "${CONFIG_ENV} names {}, which does not exist",
                    path.display()
                )
            }
            Self::Read { path, error } => write!(f, "cannot read {}: {error}", path.display()),
            Self::InFile { path, error } => write!(f, "{}: {error}", path.display()),
        }
    }
}

impl std::error::Error for SettingsError {}

/// Parse and validate a `cedian.toml` document. Pure.
pub fn parse_settings(toml_src: &str) -> Result<Settings, SettingsError> {
    let raw: RawSettings =
        toml::from_str(toml_src).map_err(|e| SettingsError::Parse(e.message().to_string()))?;
    if raw.schema != Some(SETTINGS_SCHEMA) {
        return Err(SettingsError::BadSchema(raw.schema));
    }
    Ok(Settings {
        permissions: raw.permissions,
        reviewer_allow_list: raw.reviewer_allow_list,
        review_role: raw
            .review
            .role
            .map(|r| r.trim().to_string())
            .filter(|r| !r.is_empty())
            .map(ReviewRole)
            .unwrap_or_default(),
        update_channel: raw.update_channel,
        floor: GateFloor::from_specs(raw.workflow.floor).map_err(SettingsError::BadFloor)?,
        projects: raw
            .projects
            .into_iter()
            .map(|(key, project)| (key, project.policy))
            .collect(),
        source: None,
    })
}

/// Where the user's settings file is: `(path, explicit)`. `explicit` means
/// `CEDIAN_CONFIG` named it. `None` when no location can be derived.
fn locate(env: impl Fn(&str) -> Option<String>) -> Option<(PathBuf, bool)> {
    let set = |key: &str| env(key).filter(|v| !v.is_empty()).map(PathBuf::from);
    if let Some(path) = set(CONFIG_ENV) {
        return Some((path, true));
    }
    let dir = set("XDG_CONFIG_HOME").or_else(|| set("HOME").map(|home| home.join(".config")))?;
    Some((dir.join("cedian").join(SETTINGS_FILE), false))
}

/// The settings `workdir` runs under: refuse workspace settings files, then
/// read the user's file. No file at the default location = defaults.
pub fn resolve_settings(workdir: &Path) -> Result<Settings, SettingsError> {
    resolve_with(workdir, |key| std::env::var(key).ok())
}

fn resolve_with(
    workdir: &Path,
    env: impl Fn(&str) -> Option<String>,
) -> Result<Settings, SettingsError> {
    let located = locate(env);
    let user = located.as_ref().map(|(path, _)| path.clone());
    let legacy = workdir.join(LEGACY_FILE);
    if legacy.exists() {
        return Err(SettingsError::Legacy {
            found: legacy,
            user,
        });
    }
    let local = workdir.join(SETTINGS_FILE);
    let same_file = |a: &Path, b: &Path| match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    };
    if local.exists() && !user.as_deref().is_some_and(|u| same_file(u, &local)) {
        return Err(SettingsError::InWorkspace { found: local, user });
    }
    let Some((path, explicit)) = located else {
        return Ok(Settings::default());
    };
    match std::fs::read_to_string(&path) {
        Ok(src) => match parse_settings(&src) {
            Ok(settings) => Ok(Settings {
                source: path.canonicalize().ok(),
                ..settings
            }),
            Err(error) => Err(SettingsError::InFile {
                path,
                error: Box::new(error),
            }),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && !explicit => Ok(Settings::default()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(SettingsError::Missing(path)),
        Err(e) => Err(SettingsError::Read {
            path,
            error: e.to_string(),
        }),
    }
}

/// Canonical default document (what onboarding writes on first run).
pub fn default_settings_toml() -> String {
    format!(
        "schema = {SETTINGS_SCHEMA}\n\
         update_channel = \"stable\"\n\
         reviewer_allow_list = []\n\
         \n\
         [permissions]\n\
         safe = \"allow\"\n\
         project_write = \"allow\"\n\
         dangerous = \"ask\"\n\
         \n\
         # The OMP model role reviewers run on (map it in OMP's modelRoles).\n\
         # [review]\n\
         # role = \"review\"\n\
         \n\
         # Gates cedian requires per task kind and risk. None = fast lane.\n\
         # [[workflow.floor]]\n\
         # kind = \"bug_fix\"\n\
         # min_risk = \"medium\"\n\
         # gates = [{{ id = \"tests\", gate_kind = \"test\", evidence_kinds = [\"test\"] }}]\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cedian_workflow::{Risk, TaskKind};
    use std::collections::HashMap;

    fn parse(src: &str) -> Result<Settings, SettingsError> {
        parse_settings(src)
    }

    #[test]
    fn default_document_round_trips() {
        assert_eq!(
            parse(&default_settings_toml()).unwrap(),
            Settings::default()
        );
    }

    #[test]
    fn review_role_defaults_to_review_and_a_model_id_is_refused() {
        assert_eq!(parse("schema = 1").unwrap().review_role.as_str(), "review");
        let s = parse("schema = 1\n[review]\nrole = \"review-alt\"\n").unwrap();
        assert_eq!(s.review_role.as_str(), "review-alt");
        assert!(
            parse("schema = 1\n[review]\nmodel = \"opencode-go/glm-5.3\"\n").is_err(),
            "cedian.toml names roles, never model ids (ADR-0039)"
        );
    }

    #[test]
    fn schema_is_required_and_checked() {
        assert_eq!(parse("").unwrap_err(), SettingsError::BadSchema(None));
        assert_eq!(
            parse("schema = 2").unwrap_err(),
            SettingsError::BadSchema(Some(2))
        );
        assert!(parse("schema = 1").is_ok());
    }

    #[test]
    fn defaults_are_safe() {
        let s = parse("schema = 1").unwrap();
        assert_eq!(s.permissions.dangerous, Verdict::Ask);
        assert_eq!(s.update_channel, UpdateChannel::Stable);
        assert!(s.floor.rules.is_empty());
    }

    #[test]
    fn unknown_keys_fail_closed_at_any_depth() {
        for src in [
            "schema = 1\ndangerous_stuff = true",
            "schema = 1\n[permissions]\nsafe = \"allow\"\nproject_write = \"allow\"\ndangerous = \"ask\"\ndangerus = \"allow\"",
            "schema = 1\n[workflow]\nflor = []",
            "schema = 1\n[projects.\"/x\"]\npolicy = \"omp\"\ncomputer = true",
        ] {
            match parse(src) {
                Err(SettingsError::Parse(e)) => assert!(e.contains("unknown field"), "{src}: {e}"),
                other => panic!("{src}: {other:?}"),
            }
        }
    }

    #[test]
    fn bad_channel_rejected() {
        assert!(matches!(
            parse("schema = 1\nupdate_channel = \"nightly\""),
            Err(SettingsError::Parse(_))
        ));
    }

    #[test]
    fn floor_loads_and_bad_floor_fails() {
        let s = parse(
            "schema = 1\n[[workflow.floor]]\nkind = \"bug_fix\"\nmin_risk = \"medium\"\n\
             gates = [{ id = \"tests\", gate_kind = \"test\", evidence_kinds = [\"test\"] }]",
        )
        .unwrap();
        let rule = &s.floor.rules[0];
        assert_eq!((rule.kind, rule.min_risk), (TaskKind::BugFix, Risk::Medium));
        assert!(rule.gates[0].required && rule.gates[0].predicate.fresh);
        assert!(matches!(
            parse(
                "schema = 1\n[[workflow.floor]]\nkind = \"bug_fix\"\nmin_risk = \"low\"\ngates = [{ id = \"t\", gate_kind = \"test\", fresh = false }]"
            ),
            Err(SettingsError::Parse(_))
        ));
    }

    #[test]
    fn floor_review_roles_and_max_reviewers_load() {
        let s = parse(
            "schema = 1\n[[workflow.floor]]\nkind = \"feature\"\nmin_risk = \"low\"\n\
             review_roles = [\"review\", \"review-alt\"]\nmax_reviewers = 2\n\
             gates = [{ id = \"tests\", gate_kind = \"test\" }]",
        )
        .unwrap();
        let rule = &s.floor.rules[0];
        assert_eq!(rule.review_roles, vec!["review", "review-alt"]);
        assert_eq!(rule.max_reviewers, Some(2));
        let err = parse(
            "schema = 1\n[[workflow.floor]]\nkind = \"feature\"\nmin_risk = \"low\"\n\
             review_roles = [\"opencode-go/muse\"]\n\
             gates = [{ id = \"tests\", gate_kind = \"test\" }]",
        )
        .unwrap_err();
        assert!(
            matches!(&err, SettingsError::BadFloor(e) if e.contains("model id")),
            "{err}"
        );
    }

    #[test]
    fn project_policy_must_be_cedian_or_omp() {
        assert!(matches!(
            parse("schema = 1\n[projects.\"/x\"]\npolicy = \"yolo\""),
            Err(SettingsError::Parse(_))
        ));
        assert!(matches!(
            parse("schema = 1\n[projects.\"/x\"]"),
            Err(SettingsError::Parse(_))
        ));
    }

    #[test]
    fn project_key_opts_in_only_its_canonical_workspace() {
        let t = Tmp::new("projects");
        let other = t.0.join("other");
        std::fs::create_dir_all(&other).unwrap();
        let s = parse(&format!(
            "schema = 1\n[projects.{:?}]\npolicy = \"omp\"\n[projects.\"relative/x\"]\npolicy = \"omp\"\n[projects.{:?}]\npolicy = \"omp\"\n[projects.{:?}]\npolicy = \"omp\"\n",
            t.ws().display().to_string(),
            t.0.join("gone").display().to_string(),
            t.0.join("other/../ws").display().to_string(),
        ))
        .unwrap();
        let ws = s.policy_for(&t.ws(), RunKind::Interactive);
        assert_eq!(ws.policy, Policy::Omp);
        assert_eq!(ws.notes.len(), 3, "{:?}", ws.notes);
        assert!(
            ws.notes.iter().any(|n| n.contains("not canonical")),
            "{:?}",
            ws.notes
        );
        assert_eq!(
            s.policy_for(&other, RunKind::Interactive).policy,
            Policy::Cedian
        );
        let unattended = s.policy_for(&t.ws(), RunKind::Unattended);
        assert_eq!(unattended.policy, Policy::Cedian);
        assert!(unattended.notes.iter().any(|n| n.contains("unattended")));
        assert_eq!(
            Settings::default()
                .policy_for(&t.ws(), RunKind::Interactive)
                .policy,
            Policy::Cedian
        );
    }

    #[test]
    fn opt_in_fails_closed_on_doubt() {
        let t = Tmp::new("doubt");
        let ws = t.ws().display().to_string();
        let link = t.0.join("link");
        std::os::unix::fs::symlink(t.ws(), &link).unwrap();
        let conflicting = parse(&format!(
            "schema = 1\n[projects.{ws:?}]\npolicy = \"omp\"\n[projects.{:?}]\npolicy = \"cedian\"\n",
            format!("{ws}/"),
        ))
        .unwrap();
        let choice = conflicting.policy_for(&t.ws(), RunKind::Interactive);
        assert_eq!(choice.policy, Policy::Cedian, "{:?}", choice.notes);
        assert!(
            choice
                .notes
                .iter()
                .any(|n| n.contains("different policies"))
        );

        let via_link = parse(&format!(
            "schema = 1\n[projects.{:?}]\npolicy = \"omp\"\n",
            link.display().to_string()
        ))
        .unwrap();
        assert_eq!(
            via_link.policy_for(&t.ws(), RunKind::Interactive).policy,
            Policy::Cedian
        );

        let inside = t.ws().join("cedian.toml");
        std::fs::write(
            &inside,
            format!("schema = 1\n[projects.{ws:?}]\npolicy = \"omp\"\n"),
        )
        .unwrap();
        let s = resolve_with(&t.ws(), env(&[("CEDIAN_CONFIG", &inside)])).unwrap();
        let choice = s.policy_for(&t.ws(), RunKind::Interactive);
        assert_eq!(choice.policy, Policy::Cedian);
        assert!(
            choice
                .notes
                .iter()
                .any(|n| n.contains("inside this workspace"))
        );
    }

    struct Tmp(PathBuf);
    impl Tmp {
        fn new(name: &str) -> Self {
            let dir =
                std::env::temp_dir().join(format!("cedian-settings-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(dir.join("ws")).unwrap();
            Self(dir.canonicalize().unwrap())
        }
        fn ws(&self) -> PathBuf {
            self.0.join("ws")
        }
    }
    impl Drop for Tmp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn env(pairs: &[(&str, &Path)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.display().to_string()))
            .collect();
        move |key| map.get(key).cloned()
    }

    #[test]
    fn location_prefers_env_then_xdg_then_home() {
        let p = Path::new("/c/cedian.toml");
        assert_eq!(locate(env(&[("CEDIAN_CONFIG", p)])), Some((p.into(), true)));
        assert_eq!(
            locate(env(&[
                ("XDG_CONFIG_HOME", Path::new("/x")),
                ("HOME", Path::new("/h"))
            ])),
            Some((PathBuf::from("/x/cedian/cedian.toml"), false))
        );
        assert_eq!(
            locate(env(&[("HOME", Path::new("/h"))])),
            Some((PathBuf::from("/h/.config/cedian/cedian.toml"), false))
        );
        assert_eq!(locate(env(&[])), None);
    }

    #[test]
    fn user_file_is_read_and_missing_default_means_defaults() {
        let t = Tmp::new("read");
        let user = t.0.join("cedian.toml");
        std::fs::write(&user, "schema = 1\n[permissions]\nsafe = \"allow\"\nproject_write = \"deny\"\ndangerous = \"deny\"\n").unwrap();
        let s = resolve_with(&t.ws(), env(&[("CEDIAN_CONFIG", &user)])).unwrap();
        assert_eq!(s.permissions.dangerous, Verdict::Deny);
        let s = resolve_with(&t.ws(), env(&[("XDG_CONFIG_HOME", &t.0.join("none"))])).unwrap();
        assert_eq!(s, Settings::default());
    }

    #[test]
    fn explicit_missing_file_is_an_error() {
        let t = Tmp::new("missing");
        let gone = t.0.join("typo.toml");
        assert_eq!(
            resolve_with(&t.ws(), env(&[("CEDIAN_CONFIG", &gone)])).unwrap_err(),
            SettingsError::Missing(gone)
        );
    }

    #[test]
    fn bad_user_file_names_its_path() {
        let t = Tmp::new("bad");
        let user = t.0.join("cedian.toml");
        std::fs::write(&user, "schema = 3").unwrap();
        let err = resolve_with(&t.ws(), env(&[("CEDIAN_CONFIG", &user)])).unwrap_err();
        assert!(
            err.to_string().starts_with(&user.display().to_string()),
            "{err}"
        );
    }

    #[test]
    fn leftover_json_is_refused_naming_the_move() {
        let t = Tmp::new("legacy");
        let user = t.0.join("cedian.toml");
        std::fs::write(&user, "schema = 1").unwrap();
        std::fs::write(t.ws().join("cedian.json"), "{}").unwrap();
        let err = resolve_with(&t.ws(), env(&[("CEDIAN_CONFIG", &user)])).unwrap_err();
        let text = err.to_string();
        assert!(matches!(err, SettingsError::Legacy { .. }));
        assert!(
            text.contains("cedian.json is no longer read")
                && text.contains(&user.display().to_string())
                && text.contains("schema = 1"),
            "{text}"
        );
    }

    #[test]
    fn workspace_toml_is_refused_unless_it_is_the_user_file() {
        let t = Tmp::new("local");
        let local = t.ws().join("cedian.toml");
        std::fs::write(&local, "schema = 1\n[permissions]\nsafe = \"allow\"\nproject_write = \"allow\"\ndangerous = \"allow\"\n").unwrap();
        let user = t.0.join("cedian.toml");
        std::fs::write(&user, "schema = 1").unwrap();
        assert!(matches!(
            resolve_with(&t.ws(), env(&[("CEDIAN_CONFIG", &user)])),
            Err(SettingsError::InWorkspace { .. })
        ));
        assert!(resolve_with(&t.ws(), env(&[("CEDIAN_CONFIG", &local)])).is_ok());
    }
}
