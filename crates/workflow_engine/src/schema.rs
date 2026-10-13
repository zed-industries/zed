//! `flow.json` — the persisted workflow definition. Mirrors
//! `DOCS/workflow-schema/flow.schema.json` field-for-field; if the two
//! disagree, that's a real bug, not a style choice — the JSON Schema is
//! meant to validate exactly what these types (de)serialize.
//!
//! Deliberately **not** an enum-per-action-type. See
//! `DOCS/forge-workflow-engine-design.md` §2's "Locked" note: adding an
//! action type must stay a pure `ActionDef` registry entry (`registry.rs`),
//! never a Rust code change — an enum would force every exhaustive `match`
//! over `Action` (this adapter, the execution engine, the canvas) to be
//! touched and recompiled per new type, the exact "changing a feature
//! touches the parser" shape this whole engine replaces ForgeFlow to avoid.
//! The same reasoning is why the four container constructs
//! (`Foreach`/`Until`/`If`/`Try`) are optional fields on one flat `Action`
//! rather than their own enum variants — the schema itself defines "action"
//! as one object type with conditional-but-optional fields (see its
//! `allOf`/`if`/`then`), not a discriminated union, and this type matches
//! that shape exactly.

use std::path::Path;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Directory, next to `<flow>.flow.json`, holding a flow's per-action request
/// body companions — the same relationship `.flow.layout.json` already has to
/// its flow file.
///
/// Lives here rather than in the Designer because two independent crates have
/// to agree on it: the Designer *writes* companions to these paths, and the
/// JSON language server's schema associations have to *recognize* them to
/// validate them. Duplicating the literal in both would let the writer and
/// the validator drift apart silently, which is worse than a slightly odd
/// home for the constant.
pub const HTTP_BODY_DIR_SUFFIX: &str = ".flow-http";

/// Suffix of a per-action request-body companion: `<action_id>.body.json`.
///
/// The trailing `.json` is what gets the file the JSON language server (and
/// `json.schemas` file associations) at all; `*.flow-http/**` alone would
/// match nothing that Zed has a language for.
pub const HTTP_BODY_FILE_SUFFIX: &str = ".body.json";

/// Suffix of the optional user-authored schema for a body companion:
/// `<action_id>.body.schema.json`, sitting next to the body it validates.
///
/// Optional by design: the body is always validated by the registry-derived
/// schema (see `registry::body_json_schema`) plus plain well-formedness. This
/// sidecar is how a user states the shape the registry can't know — the
/// actual API contract of the service being called.
pub const HTTP_BODY_SCHEMA_SUFFIX: &str = ".body.schema.json";

/// Whether `path` is a request-body companion file, judged purely on its
/// name. Used by the JSON language server wiring to decide which schemas to
/// apply to an opened file, and by the sidecar lookup to find the schema that
/// belongs to a given body.
///
/// Requires both the directory *and* the file suffix so a `.body.json` file
/// that isn't part of a `.flow-http` tree isn't mistaken for one.
pub fn is_http_body_companion(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let dir_matches = path.components().any(|component| {
        component
            .as_os_str()
            .to_string_lossy()
            .contains(HTTP_BODY_DIR_SUFFIX)
    });
    dir_matches && name.ends_with(HTTP_BODY_FILE_SUFFIX)
}

/// The sidecar schema path belonging to a body companion at `body_path`:
/// `foo.body.json` -> `foo.body.schema.json`, in the same directory.
///
/// Returns `None` if `body_path` isn't a body companion or has no parent, so
/// callers can't accidentally synthesize a path for an unrelated file.
pub fn http_body_sidecar_path(body_path: &Path) -> Option<std::path::PathBuf> {
    if !is_http_body_companion(body_path) {
        return None;
    }
    let name = body_path.file_name()?.to_str()?;
    let stem = name.strip_suffix(HTTP_BODY_FILE_SUFFIX)?;
    let sidecar = format!("{stem}{HTTP_BODY_SCHEMA_SUFFIX}");
    Some(body_path.with_file_name(sidecar))
}

/// A flow's `actions` map, and every nested `actions`/`else.actions`/
/// `catch.actions` body — same shape, fully recursive. `IndexMap` (not
/// `HashMap`) so a round-tripped file doesn't reorder actions the user
/// didn't touch, even though `runAfter` — not this map's key order — is
/// what actually determines execution order.
pub type ActionMap = IndexMap<String, Action>;

/// Any JSONLogic expression (<https://jsonlogic.com>), evaluated by
/// `datalogic-rs`. Deliberately opaque here — JSONLogic's own grammar is
/// the source of truth for what's inside, not this schema.
pub type JsonLogicExpr = Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub actions: ActionMap,
    #[serde(default)]
    pub outputs: IndexMap<String, JsonLogicExpr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub type_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Untyped, per-type shape defined only by the `ActionDef` registry
    /// entry for `type_id` (`registry.rs`) — see the module doc's "Locked"
    /// note. Not validated here; the canvas/execution engine consult the
    /// registry for that.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub inputs: serde_json::Map<String, Value>,
    /// Empty = no dependency (a parallel root, or the first action of a
    /// nested body). Each key is another action id *in the same
    /// `ActionMap`* (nesting levels don't cross) whose listed outcome(s)
    /// must occur before this action runs. `["Succeeded", "Failed"]` is
    /// how "continue on failure" is expressed — see the design doc's
    /// "Locked" note on why that's not a separate bool field.
    #[serde(
        default,
        rename = "runAfter",
        skip_serializing_if = "IndexMap::is_empty"
    )]
    pub run_after: IndexMap<String, Vec<RunOutcome>>,
    /// Canvas position `[x, y]` — flow-space, relative to this action's own
    /// parent scope for a nested action (same convention as
    /// `gpui_flow::FlowNode::parent`'s local-position rule), absolute for a
    /// top-level one. `None` for an action never placed on the canvas yet
    /// (or hand-written), in which case `workflow_json::load` falls back to
    /// its placeholder stacked layout for just that action.
    ///
    /// **Revises** the design doc's original "canvas position is not
    /// stored in `flow.json`" decision — that assumed a `flow.layout.json`
    /// sidecar would exist to hold it instead. It doesn't, and until it
    /// does, saving a flow with no position field silently discarded every
    /// manual arrangement on the next open (confirmed live: dragging nodes,
    /// saving, reopening reset them). Embedding `pos` directly is strictly
    /// worse for the "matches an external spec exactly" goal that
    /// motivated the sidecar, but strictly better than the actual observed
    /// behavior of losing user edits. Revisit if/when the sidecar gets
    /// built for real.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pos: Option<(f32, f32)>,

    // ── Foreach/Until only ──
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreach: Option<JsonLogicExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until: Option<JsonLogicExpr>,
    /// Mandatory on every real `Until` action (enforced by the JSON Schema's
    /// conditional `allOf`, not by this field being non-`Option` — the
    /// schema comment explains why the type system can't express
    /// "required only when type_id == Until" cleanly on one flat struct).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,

    /// Foreach/Until body, or If's true-branch body, or Try's try-body —
    /// same recursive `ActionMap` shape every time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<ActionMap>,

    // ── If only ──
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<JsonLogicExpr>,
    #[serde(default, rename = "else", skip_serializing_if = "Option::is_none")]
    pub else_branch: Option<Branch>,

    // ── Try only ──
    /// Optional — see the design doc §3a. Runs only if something inside
    /// `actions` (the try body) actually ends `Failed` at runtime, not a
    /// JSONLogic condition decided up front (that's the concrete reason
    /// `Try` needed its own construct instead of reusing `If`). Omitting
    /// `catch` is legal — an unhandled `Try` just fails openly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catch: Option<Branch>,
}

impl Action {
    pub fn new(type_id: impl Into<String>) -> Self {
        Self {
            type_id: type_id.into(),
            label: None,
            inputs: serde_json::Map::new(),
            run_after: IndexMap::new(),
            pos: None,
            foreach: None,
            until: None,
            limit: None,
            actions: None,
            expression: None,
            else_branch: None,
            catch: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub actions: ActionMap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    pub count: u32,
    /// ISO 8601 duration, e.g. `"PT30S"` — kept as the raw string (not
    /// parsed into `std::time::Duration`, which has no ISO 8601 `serde`
    /// support) since parsing it is an execution-engine concern, not this
    /// schema layer's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RunOutcome {
    Succeeded,
    Failed,
    Skipped,
}

/// `script` action `inputs.source` — §4's two source modes, discriminated
/// by `kind`. The one shape the JSON Schema constrains beyond "untyped
/// per-registry", since it's part of §4 itself, not registry-specific.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ScriptSource {
    Inline {
        /// `dotnet` deliberately excluded at the JSON Schema level (no
        /// equivalent to executing a loose code string) — not re-enforced
        /// by this Rust type, which accepts any `ScriptRuntime`; validate
        /// against `flow.schema.json` for that constraint, don't duplicate
        /// it silently and incompletely here.
        runtime: ScriptRuntime,
        code: String,
    },
    File {
        /// Always resolved against the solution root, never the running
        /// process's own working directory — direct lesson from
        /// ForgeFlow's `uri_to_path` bug (design doc §4).
        path: String,
        /// Override only — normally inferred from `path`'s extension.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        runtime: Option<ScriptRuntime>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScriptRuntime {
    Python,
    Node,
    Dotnet,
    Powershell,
    Bash,
}

impl ScriptRuntime {
    /// The file extension this runtime's scripts use: the suffix of the temp
    /// file inline code is spilled to, and of the companion file the designer
    /// writes beside the flow.
    ///
    /// This is the single source of truth for the runtime <-> extension
    /// mapping. The executors spill and spawn from it and
    /// `designer_panel::script_companion` names companion files from it, so a
    /// new runtime can't be added to one and forgotten in the other — the same
    /// "one definition per concept" rule the module doc states for action
    /// types.
    pub fn extension(self) -> &'static str {
        match self {
            ScriptRuntime::Python => "py",
            ScriptRuntime::Node => "js",
            ScriptRuntime::Powershell => "ps1",
            ScriptRuntime::Dotnet => "csproj",
            ScriptRuntime::Bash => "sh",
        }
    }

    /// The runtime a script at `path` should run under, inferred from its
    /// extension. `None` when the extension isn't recognized, in which case
    /// file mode requires an explicit `runtime` override.
    ///
    /// Note `.ts` maps to `Node`, not a TypeScript runtime: the executor
    /// spawns `node`, which runs the annotated source directly on current
    /// Node versions.
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
            "py" => Some(ScriptRuntime::Python),
            // `.mjs`/`.cjs` (and the TypeScript `.mts`/`.cts`) are the same
            // runtime; the extension only tells Node which module system the
            // file uses.
            "js" | "mjs" | "cjs" | "ts" | "mts" | "cts" => Some(ScriptRuntime::Node),
            "ps1" => Some(ScriptRuntime::Powershell),
            "csproj" => Some(ScriptRuntime::Dotnet),
            "sh" | "bash" => Some(ScriptRuntime::Bash),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_a_top_level_body_companion() {
        assert!(is_http_body_companion(Path::new(
            "/repo/checkout.flow-http/fetch_users.body.json"
        )));
    }

    #[test]
    fn recognizes_a_nested_body_companion() {
        // Nested inside an `If`/`Foreach`/`Try` body, so the scope directory
        // sits between the `.flow-http` dir and the file.
        assert!(is_http_body_companion(Path::new(
            "/repo/checkout.flow-http/loop/then/fetch_users.body.json"
        )));
    }

    #[test]
    fn rejects_files_that_are_not_body_companions() {
        // Right suffix, wrong directory: a `.body.json` the Designer didn't
        // write must not be captured by the schema association.
        assert!(!is_http_body_companion(Path::new(
            "/repo/other/fetch_users.body.json"
        )));
        // Right directory, wrong file: the `.flow-http` dir may later hold
        // the sidecar schema itself, which is a schema, not an instance.
        assert!(!is_http_body_companion(Path::new(
            "/repo/checkout.flow-http/fetch_users.body.schema.json"
        )));
        assert!(!is_http_body_companion(Path::new(
            "/repo/checkout.flow.json"
        )));
    }

    #[test]
    fn sidecar_path_sits_next_to_its_body() {
        let sidecar = http_body_sidecar_path(Path::new(
            "/repo/checkout.flow-http/loop/fetch_users.body.json",
        ))
        .expect("a body companion has a sidecar path");
        assert_eq!(
            sidecar,
            Path::new("/repo/checkout.flow-http/loop/fetch_users.body.schema.json")
        );
    }

    #[test]
    fn sidecar_path_is_none_for_non_bodies() {
        // Guards against synthesizing a path for an unrelated file, which
        // would then advertise a schema association for it.
        assert_eq!(
            http_body_sidecar_path(Path::new("/repo/other/thing.body.json")),
            None
        );
    }

    /// The actual worked example from `DOCS/workflow-schema/example-flow.json`
    /// must deserialize cleanly through these types — if the JSON Schema and
    /// this Rust type ever drift apart, this is where it shows up first.
    #[test]
    fn parses_the_reference_example_flow() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("DOCS/workflow-schema/example-flow.json");
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
        let def: WorkflowDefinition = serde_json::from_str(&text).expect("parse example-flow.json");
        assert_eq!(def.id, "order-processing");
        assert!(def.actions.contains_key("waitForApi"));
        assert!(def.actions.contains_key("foreach"));

        let foreach = &def.actions["foreach"];
        assert_eq!(foreach.type_id, "Foreach");
        assert!(foreach.foreach.is_some());
        let body = foreach.actions.as_ref().expect("foreach body");
        let try_charge = &body["tryCharge"];
        assert_eq!(try_charge.type_id, "Try");
        assert!(try_charge.catch.is_some());

        let until = &def.actions["until"];
        assert_eq!(until.type_id, "Until");
        assert!(until.limit.is_some());

        let if_vip = &def.actions["ifVip"];
        assert_eq!(if_vip.type_id, "If");
        assert!(if_vip.else_branch.is_some());
    }

    #[test]
    fn round_trips_through_serde() {
        let mut action = Action::new("http");
        action
            .inputs
            .insert("method".into(), Value::String("GET".into()));
        action.run_after.insert(
            "root".into(),
            vec![RunOutcome::Succeeded, RunOutcome::Failed],
        );

        let mut actions = ActionMap::new();
        actions.insert("fetch".into(), action);
        let def = WorkflowDefinition {
            id: "x".into(),
            name: "X".into(),
            actions,
            outputs: IndexMap::new(),
        };

        let json = serde_json::to_string(&def).unwrap();
        let back: WorkflowDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(def, back);
    }
}
