//! Load/save adapter between `flow.json` (`workflow_engine::schema`) and
//! `gpui_flow::FlowState`. Ported from
//! `E:\Forge_GPUI\src\forge_shell\panels\designer_panel\workflow_json.rs`,
//! cut down to the workflow-JSON half only (no `.fdgn`/ForgeFlow — never
//! vendored into this tree, see `gpui_flow`/`workflow_engine`'s own module
//! docs).
//!
//! **Position data does not live in `flow.json`.** `Action::pos` is a
//! legacy stopgap field (see `workflow_engine::schema`'s doc comment on
//! it) superseded by the `.flow.layout.json` sidecar
//! (`workflow_engine::layout`, already ported) — `save` here never sets
//! `Action::pos`, only returns a path-keyed `WorkflowLayout` for the
//! caller to persist via `workflow_engine::layout::save`.
//!
//! **`If`/`Try`'s two nested bodies need a visual grouping node that has
//! no equivalent in the JSON schema at all** — `action.actions` (the
//! true/try body) and `action.else_branch`/`action.catch` (the false/catch
//! body) are just fields on one `Action`, but the canvas needs two
//! separate `FlowNode` containers side by side. [`load`] synthesizes them
//! with reserved `node_type`s (`__branch_then`, `__branch_else`,
//! `__branch_try`, `__branch_catch`); [`save`] recognizes those exact
//! strings to unwrap them back into the right `Action` field instead of
//! emitting them as a real action.

use std::collections::HashMap;

use gpui_flow::{FlowEdge, FlowNode, FlowState, HandleDef, HandlePosition};
use serde_json::Value;
use workflow_engine::{
    Action, ActionMap, Branch, FieldKind, InputField, JsonLogicExpr, Limit, RunOutcome,
    WorkflowDefinition, WorkflowLayout,
};

use indexmap::IndexMap;

pub const BRANCH_THEN: &str = "__branch_then";
pub const BRANCH_ELSE: &str = "__branch_else";
pub const BRANCH_TRY: &str = "__branch_try";
pub const BRANCH_CATCH: &str = "__branch_catch";

/// Returns default node properties when adding a new action of `type_id`.
///
/// A `script` gets none here: its `source` names a file created for that
/// node, which takes the flow's path and the node's id to work out, so the
/// panel sets it (`DesignerPanel::push_action_node`).
pub fn default_properties_for_type(type_id: &str) -> Vec<(gpui::SharedString, gpui::SharedString)> {
    match type_id {
        "http" => vec![("method".into(), "GET".into())],
        "InitVariable" => vec![("type".into(), "string".into())],
        _ => Vec::new(),
    }
}

pub const LEAF_SIZE: (f32, f32) = (172.0, 46.0);
const Y_STEP: f32 = 130.0;
const ROOT_X: f32 = 60.0;
pub(crate) const CHILD_X: f32 = 20.0;
pub(crate) const CHILD_Y0: f32 = 20.0;
// ── Load: WorkflowDefinition -> FlowState ──────────────────────────────

/// Build a `FlowState` from a parsed `flow.json`. Every action is expected
/// to already have a real `pos` (the caller applies the saved sidecar,
/// then auto-layout for anything still unset — see `auto_layout`) — a
/// `None` still falls back to a simple one-column placeholder stack so
/// this never panics/produces an unusable graph on its own.
///
/// `edge_color`/`failed_edge_color` are `0xRRGGBB` — the caller's job to
/// derive from the active theme (`designer_panel::hex(cx.theme()...)`),
/// so every edge (including a plain "Succeeded" one, and a continue-on-
/// failure "Always" one) gets an explicit color instead of falling
/// through to `gpui_flow`'s own hardcoded default gray — that default
/// doesn't move with the theme, since `gpui_flow` itself is vendored
/// as-is (see its own module doc: zero source rewrites).
pub fn load(def: &WorkflowDefinition, edge_color: u32, failed_edge_color: u32) -> FlowState {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut next_y = CHILD_Y0;
    walk_actions(
        &def.actions,
        None,
        ROOT_X,
        &mut next_y,
        &mut nodes,
        &mut edges,
        edge_color,
        failed_edge_color,
    );

    let mut state = FlowState::new(nodes, edges);
    state.refit_all_containers();
    state
}

fn walk_actions(
    actions: &ActionMap,
    parent: Option<&str>,
    x: f32,
    next_y: &mut f32,
    nodes: &mut Vec<FlowNode>,
    edges: &mut Vec<FlowEdge>,
    edge_color: u32,
    failed_edge_color: u32,
) {
    for (id, action) in actions {
        let (px, py) = match action.pos {
            Some((px, py)) => (px, py),
            None => {
                let y = *next_y;
                *next_y += Y_STEP;
                (x, y)
            }
        };

        let mut node = FlowNode::new(id.clone(), px, py)
            .node_type(action.type_id.clone())
            .label(action.label.clone().unwrap_or_else(|| id.clone()))
            .context_menu(true)
            .handles(vec![
                HandleDef::target(HandlePosition::Top),
                HandleDef::source(HandlePosition::Bottom),
            ]);
        if let Some(p) = parent {
            node = node.parent(p.to_string());
        }
        node.properties = flatten_action_meta(action);

        let is_container = matches!(action.type_id.as_str(), "Foreach" | "Until" | "If" | "Try");
        if !is_container {
            node = node.size(LEAF_SIZE.0, LEAF_SIZE.1);
        }
        nodes.push(node);

        for (dep_id, outcomes) in &action.run_after {
            let mut edge = FlowEdge::new(format!("e_{dep_id}_{id}"), dep_id.clone(), id.clone())
                .color(edge_color);
            let has_succeeded = outcomes.contains(&RunOutcome::Succeeded);
            let has_failed = outcomes.contains(&RunOutcome::Failed);
            if has_failed && !has_succeeded {
                edge = edge.color(failed_edge_color).label("Failed");
            } else if has_failed && has_succeeded {
                edge = edge.label("Always");
            }
            edges.push(edge);
        }

        match action.type_id.as_str() {
            "Foreach" | "Until" => {
                if let Some(body) = &action.actions {
                    let mut y = CHILD_Y0;
                    walk_actions(
                        body,
                        Some(id),
                        CHILD_X,
                        &mut y,
                        nodes,
                        edges,
                        edge_color,
                        failed_edge_color,
                    );
                }
            }
            // Both wrappers are always created, whether or not the file has
            // the branch: a wrapper is the only thing on the canvas an action
            // can be dropped into, so a missing one could never be filled.
            "If" => {
                let branches = [
                    (BRANCH_THEN, action.actions.as_ref()),
                    (
                        BRANCH_ELSE,
                        action.else_branch.as_ref().map(|branch| &branch.actions),
                    ),
                ];
                for (branch_type, body) in branches {
                    push_branch(
                        id,
                        branch_type,
                        body,
                        nodes,
                        edges,
                        edge_color,
                        failed_edge_color,
                    );
                }
            }
            "Try" => {
                let branches = [
                    (BRANCH_TRY, action.actions.as_ref()),
                    (
                        BRANCH_CATCH,
                        action.catch.as_ref().map(|branch| &branch.actions),
                    ),
                ];
                for (branch_type, body) in branches {
                    push_branch(
                        id,
                        branch_type,
                        body,
                        nodes,
                        edges,
                        edge_color,
                        failed_edge_color,
                    );
                }
            }
            _ => {}
        }
    }
}

/// Marks a branch wrapper whose branch exists in the flow file, as opposed to
/// one created only to give the canvas somewhere to drop actions.
///
/// [`save`] needs the difference for an empty branch. An empty `catch` that
/// was written on purpose swallows the failure, while no `catch` lets the
/// `Try` fail, so save must not invent one or drop one.
const BRANCH_DECLARED: &str = "__declared";

/// The two branch wrapper types an `If` or `Try` owns, or `None` for any
/// other action type. The first is the body (`actions`), the second the
/// alternative (`else`/`catch`).
pub fn branch_types(type_id: &str) -> Option<[&'static str; 2]> {
    match type_id {
        "If" => Some([BRANCH_THEN, BRANCH_ELSE]),
        "Try" => Some([BRANCH_TRY, BRANCH_CATCH]),
        _ => None,
    }
}

/// The synthetic wrapper node for one branch of `owner_id`. `declared` is
/// whether the flow file has the branch (see [`BRANCH_DECLARED`]).
///
/// Not deletable: it has no `Action` of its own, so removing it could only
/// orphan the real actions inside it.
pub fn branch_node(owner_id: &str, branch_type: &'static str, declared: bool) -> FlowNode {
    let (label, x) = match branch_type {
        BRANCH_THEN => ("True", CHILD_X),
        BRANCH_ELSE => ("False", CHILD_X + 260.0),
        BRANCH_TRY => ("Try", CHILD_X),
        _ => ("Catch", CHILD_X + 260.0),
    };
    let mut node = FlowNode::new(format!("{owner_id}{branch_type}"), x, CHILD_Y0)
        .node_type(branch_type)
        .label(label)
        .parent(owner_id.to_string())
        // So Add Action from a right-click on the branch targets the branch.
        .context_menu(true)
        .handles(vec![HandleDef::source(HandlePosition::Bottom)]);
    node.deletable = false;
    if declared {
        node.properties
            .push((BRANCH_DECLARED.into(), "true".into()));
    }
    node
}

fn push_branch(
    owner_id: &str,
    branch_type: &'static str,
    body: Option<&ActionMap>,
    nodes: &mut Vec<FlowNode>,
    edges: &mut Vec<FlowEdge>,
    edge_color: u32,
    failed_edge_color: u32,
) {
    let node = branch_node(owner_id, branch_type, body.is_some());
    let branch_id = node.id.to_string();
    nodes.push(node);
    let Some(body) = body else {
        return;
    };
    let mut y = CHILD_Y0;
    walk_actions(
        body,
        Some(&branch_id),
        CHILD_X,
        &mut y,
        nodes,
        edges,
        edge_color,
        failed_edge_color,
    );
}

/// Whether the branch behind the wrapper `wrapper_id` should be written: it
/// was in the file, or it has gained actions since.
fn branch_is_written(state: &FlowState, wrapper_id: &str) -> bool {
    let wrapper_id: gpui_flow::NodeId = wrapper_id.to_string().into();
    let Some(wrapper) = state.get_node(&wrapper_id) else {
        return false;
    };
    wrapper
        .properties
        .iter()
        .any(|(key, _)| key.as_ref() == BRANCH_DECLARED)
        || state.is_container(&wrapper_id)
}

/// Flatten `Action.inputs` plus the four struct-level fields that have no
/// place in `inputs` (`foreach`/`until`/`limit`/`expression`) into
/// `FlowNode.properties`. The four struct-level fields use a `__` prefix
/// reserved from real input names (`ActionDef.inputs` entries are always
/// plain identifiers — see `registry.rs`) so [`unflatten_action_meta`] can
/// tell them apart on the way back.
fn flatten_action_meta(action: &Action) -> Vec<(gpui::SharedString, gpui::SharedString)> {
    let mut props: Vec<(gpui::SharedString, gpui::SharedString)> = action
        .inputs
        .iter()
        .map(|(k, v)| (k.clone().into(), value_to_string(v).into()))
        .collect();
    if let Some(expr) = &action.foreach {
        props.push(("__foreach".into(), value_to_string(expr).into()));
    }
    if let Some(expr) = &action.until {
        props.push(("__until".into(), value_to_string(expr).into()));
    }
    if let Some(limit) = &action.limit {
        props.push(("__limit_count".into(), limit.count.to_string().into()));
        if let Some(t) = &limit.timeout {
            props.push(("__limit_timeout".into(), t.clone().into()));
        }
    }
    if let Some(expr) = &action.expression {
        props.push(("__expression".into(), value_to_string(expr).into()));
    }
    props
}

/// Renders an input `Value` back into the property text the inspector shows.
/// The inverse of [`string_to_value`], and `pub(crate)` because
/// `script_companion` reuses it to keep a companion-hydrated `source` row
/// spelled exactly like a freshly loaded one.
pub(crate) fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// `pub(crate)` because `http_companion` reuses it when a companion-hydrated
/// `body` is written back into the flow's `inputs`, so it lands as the same
/// `Value` a freshly loaded body would (`{"a": 1}` as JSON, not as the string
/// `"{\"a\": 1}"`).
pub(crate) fn string_to_value(s: &str) -> Value {
    serde_json::from_str(s).unwrap_or_else(|_| Value::String(s.to_string()))
}

/// Same as [`string_to_value`], but decides *how* to parse from the registry's
/// declared [`FieldKind`] instead of always trying full JSON.
///
/// The property layer is stringly-typed ([`flatten_action_meta`] reduces every
/// value to text), so without this a `FieldKind::String` field holding the
/// literal text `true` would round-trip into the JSON boolean `true` — the
/// file's *type* silently changes on every open/save cycle. That matters most
/// for `notify.message` / `log.message`, which the registry documents as
/// "String, but a JSONLogic expression is also evaluated": retyping the string
/// `true` as a boolean makes the runtime resolve a value the author meant as
/// literal text.
///
/// For `String` the rule is **scalar text is literal text, but a structured
/// value is deliberate**: `true`, `42`, `null` and unparseable text all stay
/// strings, while an object/array is kept structured. That second half matters
/// because those same String fields accept a JSONLogic expression — the
/// registry's own `help` text says so — and `resolve_string_field` only
/// evaluates a non-string value. Collapsing `{"var": "x.y"}` to a string would
/// silently stop resolving it.
///
/// `Json` and `Expression` are unguarded — those fields are JSON by
/// definition (an HTTP body, a script `source` enum). A value that doesn't
/// parse degrades to `Value::String` so a half-typed edit keeps the author's
/// keystrokes.
pub(crate) fn string_to_value_as(kind: FieldKind, s: &str) -> Value {
    match kind {
        FieldKind::String => match serde_json::from_str::<Value>(s) {
            Ok(value) if value.is_object() || value.is_array() => value,
            _ => Value::String(s.to_string()),
        },
        FieldKind::Number => serde_json::from_str(s)
            .ok()
            .filter(Value::is_number)
            .unwrap_or_else(|| Value::String(s.to_string())),
        FieldKind::Bool => serde_json::from_str(s)
            .ok()
            .filter(Value::is_boolean)
            .unwrap_or_else(|| Value::String(s.to_string())),
        FieldKind::Json | FieldKind::Expression => string_to_value(s),
    }
}

/// The registry-declared field for `type_id`'s `name`, or `None` for an
/// undeclared extra. Drives both the coercion in [`string_to_value_as`] and
/// the known-vs-extra distinction in [`unflatten_action_meta`].
pub(crate) fn declared_field(type_id: &str, name: &str) -> Option<InputField> {
    let def = workflow_engine::registry::find(type_id)?;
    def.inputs.iter().copied().find(|field| field.name == name)
}

/// The field an inspector row was flattened *from*, for container meta rows
/// that don't correspond to a registry input name one-to-one.
///
/// The container-only fields (`foreach`, `until`, `expression`, `limit`) are
/// typed in the registry under their real input names, while the canvas stores
/// them under reserved `__`-prefixed keys so they can't collide with a real
/// `inputs` key (see [`flatten_action_meta`]). This maps back so a `limit`
/// row still finds its declared `FieldKind`.
pub(crate) fn declared_field_for_meta(type_id: &str, key: &str) -> Option<InputField> {
    const META_TO_INPUT: &[(&str, &str)] = &[
        ("__expression", "expression"),
        ("__foreach", "foreach"),
        ("__until", "until"),
        ("__limit_count", "limit"),
    ];
    match key {
        "__limit_timeout" => {
            // `limit` is a single `Json` object in the registry but two rows on
            // the canvas, so only the count half is Number-typed here; the
            // timeout is free text (see `Limit::timeout`).
            let _ = type_id;
            None
        }
        other => {
            let input = META_TO_INPUT
                .iter()
                .find(|(meta, _)| *meta == other)
                .map(|(_, input)| *input)?;
            declared_field(type_id, input)
        }
    }
}

// ── Save: FlowState -> WorkflowDefinition ──────────────────────────────

/// Flow-level content the canvas has no node for and therefore cannot
/// regenerate on save.
///
/// [`load`] folds a `WorkflowDefinition` down to *nodes*, so anything that
/// isn't an action — today just the flow-level `outputs` map — has nowhere to
/// live in `FlowState`. Dropping it would silently delete a hand-authored
/// `outputs` on the first save, so [`load`] stashes it here and [`save`]
/// writes it back untouched. It is deliberately *not* editable in the panel:
/// an `outputs` entry is a JSONLogic expression, which needs the expression
/// editor in 4f before it can be edited honestly.
#[derive(Debug, Clone, Default)]
pub struct FlowPreserved {
    /// `WorkflowDefinition::outputs`, verbatim.
    pub outputs: IndexMap<String, JsonLogicExpr>,
}

/// Reads the flow-level fields [`save`] can't reconstruct back out of a
/// `FlowState`. Call once per load, hand the result to [`save`].
pub fn preserved_from(def: &WorkflowDefinition) -> FlowPreserved {
    FlowPreserved {
        outputs: def.outputs.clone(),
    }
}

/// Builds both the executable `WorkflowDefinition` (no `pos` fields — see
/// the module doc) and a `WorkflowLayout` capturing every node's current
/// canvas position, path-keyed so the same id at different nesting scopes
/// can't collide. No file I/O — the caller persists both.
pub fn save(
    state: &FlowState,
    id: impl Into<String>,
    name: impl Into<String>,
    preserved: &FlowPreserved,
) -> (WorkflowDefinition, WorkflowLayout) {
    let mut positions = HashMap::new();
    let actions = build_actions_map(state, None, "", &mut positions);
    (
        WorkflowDefinition {
            id: id.into(),
            name: name.into(),
            actions,
            outputs: preserved.outputs.clone(),
        },
        WorkflowLayout { positions },
    )
}

fn build_actions_map(
    state: &FlowState,
    parent: Option<&str>,
    path_prefix: &str,
    positions: &mut HashMap<String, (f32, f32)>,
) -> ActionMap {
    let mut map = ActionMap::new();
    for node in &state.nodes {
        let node_parent = node.parent_id.as_deref();
        if node_parent != parent {
            continue;
        }
        let type_id = node.node_type.as_deref().unwrap_or("transform");
        if is_branch_wrapper(type_id) {
            continue;
        }

        let id = node.id.to_string();
        let mut action = Action::new(type_id);
        action.label = Some(node.label.to_string());
        positions.insert(
            format!("{path_prefix}{id}"),
            (node.position.x, node.position.y),
        );
        let (inputs, foreach, until, limit, expression) =
            unflatten_action_meta(type_id, &node.properties);
        action.inputs = inputs;
        action.foreach = foreach;
        action.until = until;
        action.limit = limit;
        action.expression = expression;
        action.run_after = build_run_after(state, &node.id);

        match type_id {
            "Foreach" | "Until" => {
                let child_prefix = format!("{path_prefix}{id}/");
                action.actions = Some(build_actions_map(
                    state,
                    Some(&id),
                    &child_prefix,
                    positions,
                ));
            }
            "If" => {
                let then_id = format!("{id}{BRANCH_THEN}");
                let else_id = format!("{id}{BRANCH_ELSE}");
                let then_prefix = format!("{path_prefix}{id}/then/");
                action.actions = Some(build_actions_map(
                    state,
                    Some(&then_id),
                    &then_prefix,
                    positions,
                ));
                if branch_is_written(state, &else_id) {
                    let else_prefix = format!("{path_prefix}{id}/else/");
                    action.else_branch = Some(Branch {
                        actions: build_actions_map(state, Some(&else_id), &else_prefix, positions),
                    });
                }
            }
            "Try" => {
                let try_id = format!("{id}{BRANCH_TRY}");
                let catch_id = format!("{id}{BRANCH_CATCH}");
                let try_prefix = format!("{path_prefix}{id}/try/");
                action.actions = Some(build_actions_map(
                    state,
                    Some(&try_id),
                    &try_prefix,
                    positions,
                ));
                if branch_is_written(state, &catch_id) {
                    let catch_prefix = format!("{path_prefix}{id}/catch/");
                    action.catch = Some(Branch {
                        actions: build_actions_map(
                            state,
                            Some(&catch_id),
                            &catch_prefix,
                            positions,
                        ),
                    });
                }
            }
            _ => {}
        }

        map.insert(id, action);
    }
    map
}

/// True for the four reserved `node_type`s [`load`] synthesizes for `If`/`Try`
/// bodies. Those have no `Action` of their own, so the panel treats them as
/// canvas-only scaffolding: no properties to edit, and not deletable on their
/// own (deleting one would destroy the real actions it groups).
pub fn is_branch_wrapper(type_id: &str) -> bool {
    matches!(
        type_id,
        BRANCH_THEN | BRANCH_ELSE | BRANCH_TRY | BRANCH_CATCH
    )
}

/// Reconstruct `runAfter` for `node_id` from its *incoming* edges.
///
/// **Known lossy edge**: an edge labeled `"Failed"` always round-trips to
/// `["Failed"]`, `"Always"` to `["Succeeded","Failed"]`, anything else to
/// `["Succeeded"]` — this recovers exactly the three shapes `load` ever
/// produces, but a hand-edited `flow.json` using some other outcome
/// combination (e.g. `["Skipped"]` alone) won't round-trip through the
/// canvas unchanged.
fn build_run_after(
    state: &FlowState,
    node_id: &gpui_flow::NodeId,
) -> indexmap::IndexMap<String, Vec<RunOutcome>> {
    let mut run_after = indexmap::IndexMap::new();
    for edge in &state.edges {
        if edge.target != *node_id {
            continue;
        }
        let outcomes = match edge.label.as_deref() {
            Some("Failed") => vec![RunOutcome::Failed],
            Some("Always") => vec![RunOutcome::Succeeded, RunOutcome::Failed],
            _ => vec![RunOutcome::Succeeded],
        };
        run_after.insert(edge.source.to_string(), outcomes);
    }
    run_after
}

fn unflatten_action_meta(
    type_id: &str,
    properties: &[(gpui::SharedString, gpui::SharedString)],
) -> (
    serde_json::Map<String, Value>,
    Option<Value>,
    Option<Value>,
    Option<Limit>,
    Option<Value>,
) {
    let mut inputs = serde_json::Map::new();
    let mut foreach = None;
    let mut until = None;
    let mut limit_count: Option<u32> = None;
    let mut limit_timeout: Option<String> = None;
    let mut expression = None;

    for (k, v) in properties {
        match k.as_ref() {
            "__foreach" => foreach = Some(meta_to_value(type_id, k.as_ref(), v)),
            "__until" => until = Some(meta_to_value(type_id, k.as_ref(), v)),
            "__limit_count" => limit_count = v.parse().ok(),
            "__limit_timeout" => limit_timeout = Some(v.to_string()),
            "__expression" => expression = Some(meta_to_value(type_id, k.as_ref(), v)),
            key => {
                // A declared optional input left blank means "not used". The
                // inspector can hold an empty row for one, and writing it out
                // would send it: an empty `body` goes over the wire as `""`.
                if v.trim().is_empty()
                    && declared_field(type_id, key).is_some_and(|field| !field.required)
                {
                    continue;
                }
                inputs.insert(key.to_string(), input_to_value(type_id, key, v));
            }
        }
    }

    let limit = limit_count.map(|count| Limit {
        count,
        timeout: limit_timeout,
    });
    (inputs, foreach, until, limit, expression)
}

/// Coerces a real `inputs` row, using the registry's `FieldKind` when
/// `key` is a declared field of `type_id`.
///
/// An **undeclared** key is an extra the panel has no schema for (e.g. a
/// `strategy` key left on an old transform flow). Those get the
/// best-effort [`string_to_value`] treatment — try JSON, fall back to text —
/// which is lossy for a value like the string `true`, but unavoidable without
/// a declared type and no worse than the pre-existing behavior. Declared
/// fields get the exact round-trip [`string_to_value_as`] guarantees.
fn input_to_value(type_id: &str, key: &str, text: &str) -> Value {
    match declared_field(type_id, key) {
        Some(field) => string_to_value_as(field.kind, text),
        None => string_to_value(text),
    }
}

/// Same as [`input_to_value`] for a reserved `__`-prefixed container row.
/// `__limit_timeout` deliberately has no declared field (see
/// [`declared_field_for_meta`]) so it falls back to plain text, matching the
/// `Option<String>` it is deserialized into.
fn meta_to_value(type_id: &str, key: &str, text: &str) -> Value {
    match declared_field_for_meta(type_id, key) {
        Some(field) => string_to_value_as(field.kind, text),
        None => string_to_value(text),
    }
}

// ── The `.flow.layout.json` sidecar bridge ─────────────────────────────

/// Applies a loaded sidecar's positions onto `def`, path-keyed the same
/// way `save` produces them.
pub fn apply_saved_layout(def: &mut WorkflowDefinition, layout: &WorkflowLayout) {
    apply_layout_to_map(&mut def.actions, "", layout);
}

fn apply_layout_to_map(actions: &mut ActionMap, path_prefix: &str, layout: &WorkflowLayout) {
    for (id, action) in actions.iter_mut() {
        if let Some(&pos) = layout.positions.get(&format!("{path_prefix}{id}")) {
            action.pos = Some(pos);
        }
        match action.type_id.as_str() {
            "Foreach" | "Until" => {
                if let Some(body) = &mut action.actions {
                    apply_layout_to_map(body, &format!("{path_prefix}{id}/"), layout);
                }
            }
            "If" => {
                if let Some(body) = &mut action.actions {
                    apply_layout_to_map(body, &format!("{path_prefix}{id}/then/"), layout);
                }
                if let Some(branch) = &mut action.else_branch {
                    apply_layout_to_map(
                        &mut branch.actions,
                        &format!("{path_prefix}{id}/else/"),
                        layout,
                    );
                }
            }
            "Try" => {
                if let Some(body) = &mut action.actions {
                    apply_layout_to_map(body, &format!("{path_prefix}{id}/try/"), layout);
                }
                if let Some(branch) = &mut action.catch {
                    apply_layout_to_map(
                        &mut branch.actions,
                        &format!("{path_prefix}{id}/catch/"),
                        layout,
                    );
                }
            }
            _ => {}
        }
    }
}

/// Computes a layered/topological placement (column = dependency depth via
/// `workflow_engine::compute_levels`, row = position within that depth)
/// for every action in `def` that has no position yet after whatever
/// sidecar lookup already ran — a brand-new hand-written flow, or one the
/// Task Chain wizard just generated. Falls back to a one-column stack for
/// just the one scope a cycle/dangling-ref is detected in. Returns a
/// `WorkflowLayout` covering only the actions this pass actually placed,
/// for the caller to merge into what gets persisted.
pub fn auto_layout(def: &mut WorkflowDefinition) -> WorkflowLayout {
    let mut positions = HashMap::new();
    auto_layout_map(&mut def.actions, "", &mut positions);
    WorkflowLayout { positions }
}

const LEVEL_X_STEP: f32 = 220.0;

fn auto_layout_map(
    actions: &mut ActionMap,
    path_prefix: &str,
    positions: &mut HashMap<String, (f32, f32)>,
) {
    match workflow_engine::compute_levels(actions) {
        Ok(levels) => {
            for (level_ix, ids) in levels.iter().enumerate() {
                for (row_ix, id) in ids.iter().enumerate() {
                    let Some(action) = actions.get_mut(id) else {
                        continue;
                    };
                    if action.pos.is_none() {
                        let pos = (
                            CHILD_X + level_ix as f32 * LEVEL_X_STEP,
                            CHILD_Y0 + row_ix as f32 * Y_STEP,
                        );
                        action.pos = Some(pos);
                        positions.insert(format!("{path_prefix}{id}"), pos);
                    }
                }
            }
        }
        Err(_) => {
            let mut y = CHILD_Y0;
            for (id, action) in actions.iter_mut() {
                if action.pos.is_none() {
                    let pos = (CHILD_X, y);
                    action.pos = Some(pos);
                    positions.insert(format!("{path_prefix}{id}"), pos);
                    y += Y_STEP;
                }
            }
        }
    }

    for (id, action) in actions.iter_mut() {
        match action.type_id.as_str() {
            "Foreach" | "Until" => {
                if let Some(body) = &mut action.actions {
                    auto_layout_map(body, &format!("{path_prefix}{id}/"), positions);
                }
            }
            "If" => {
                if let Some(body) = &mut action.actions {
                    auto_layout_map(body, &format!("{path_prefix}{id}/then/"), positions);
                }
                if let Some(branch) = &mut action.else_branch {
                    auto_layout_map(
                        &mut branch.actions,
                        &format!("{path_prefix}{id}/else/"),
                        positions,
                    );
                }
            }
            "Try" => {
                if let Some(body) = &mut action.actions {
                    auto_layout_map(body, &format!("{path_prefix}{id}/try/"), positions);
                }
                if let Some(branch) = &mut action.catch {
                    auto_layout_map(
                        &mut branch.actions,
                        &format!("{path_prefix}{id}/catch/"),
                        positions,
                    );
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_def() -> WorkflowDefinition {
        WorkflowDefinition {
            id: "f".into(),
            name: "F".into(),
            actions: ActionMap::default(),
            outputs: IndexMap::default(),
        }
    }

    fn round_trip(def: &WorkflowDefinition) -> WorkflowDefinition {
        let state = load(def, 0x808080, 0xff0000);
        let (saved, _layout) = save(&state, &def.id, &def.name, &preserved_from(def));
        saved
    }

    /// The regression that motivated `FlowPreserved`: `save` used to hardcode
    /// `outputs: Default::default()`, so opening a flow with a hand-authored
    /// `outputs` map and pressing Save deleted it outright.
    #[test]
    fn flow_level_outputs_survive_a_save() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "a1": { "type": "log", "inputs": { "level": "info", "message": "hi" } }
              },
              "outputs": { "echo": { "var": "a1.message" } }
            }"#,
        )
        .unwrap();

        let saved = round_trip(&def);
        assert_eq!(saved.outputs, def.outputs);
        assert_eq!(
            saved.outputs.get("echo"),
            Some(&serde_json::json!({"var": "a1.message"})),
            "the expression should survive unchanged, not be re-serialized"
        );
    }

    #[test]
    fn a_flow_without_outputs_saves_an_empty_map() {
        assert!(round_trip(&empty_def()).outputs.is_empty());
    }

    /// `string_to_value` unconditionally tried to parse inspector text as
    /// JSON, so a `FieldKind::String` field holding the literal text `true`
    /// came back out of a save as the JSON boolean `true` — the file's type
    /// changed on every open/save cycle.
    #[test]
    fn string_fields_do_not_retype_on_round_trip() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "a1": { "type": "log", "inputs": { "level": "info", "message": "true" } }
              }
            }"#,
        )
        .unwrap();

        let saved = round_trip(&def);
        assert_eq!(
            saved.actions["a1"].inputs["message"],
            Value::String("true".into()),
            "a String field's literal text must not be re-parsed as a boolean"
        );
    }

    /// Same coercion, in the other direction: a numeric-looking string stays a
    /// string, but a declared `Number` field really does get parsed.
    #[test]
    fn declared_kinds_coerce_per_the_registry() {
        assert_eq!(
            string_to_value_as(FieldKind::String, "42"),
            Value::String("42".into())
        );
        assert_eq!(string_to_value_as(FieldKind::Number, "42"), Value::from(42));
        assert_eq!(
            string_to_value_as(FieldKind::Bool, "true"),
            Value::Bool(true)
        );
        // A half-typed edit keeps the author's keystrokes instead of vanishing.
        assert_eq!(
            string_to_value_as(FieldKind::Number, "4."),
            Value::String("4.".into())
        );
        assert_eq!(
            string_to_value_as(FieldKind::Bool, "tru"),
            Value::String("tru".into())
        );
        // JSON fields still accept structured values.
        assert_eq!(
            string_to_value_as(FieldKind::Json, r#"{"a": 1}"#),
            serde_json::json!({"a": 1})
        );
    }

    /// `notify.message` and `log.message` are `FieldKind::String` but accept a
    /// JSONLogic expression, which the runtime resolves. A *structured* value
    /// must therefore still survive even though the field is String-typed.
    #[test]
    fn string_fields_still_round_trip_structured_values() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "a1": { "type": "log", "inputs": { "level": "info", "message": {"var": "x.y"} } }
              }
            }"#,
        )
        .unwrap();

        let saved = round_trip(&def);
        assert_eq!(
            saved.actions["a1"].inputs["message"],
            serde_json::json!({"var": "x.y"})
        );
    }

    /// An input the registry doesn't declare (a leftover `strategy` key on an
    /// old transform flow) has no type to coerce by, so it round-trips on a
    /// best-effort JSON-or-text basis. Pinned so the fallback is a deliberate
    /// choice rather than drift.
    #[test]
    fn a_blank_optional_input_is_not_written() {
        let props: Vec<(gpui::SharedString, gpui::SharedString)> = vec![
            ("method".into(), "GET".into()),
            ("url".into(), "".into()),
            ("body".into(), "  ".into()),
        ];
        let (inputs, ..) = unflatten_action_meta("http", &props);
        assert!(!inputs.contains_key("body"), "blank optional is omitted");
        // A blank *required* input stays, so validation can report it.
        assert!(inputs.contains_key("url"));
    }

    #[test]
    fn undeclared_input_keys_round_trip() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "a1": { "type": "log", "inputs": { "level": "info", "message": "hi", "strategy": "fast" } }
              }
            }"#,
        )
        .unwrap();

        assert!(declared_field("log", "strategy").is_none());
        let saved = round_trip(&def);
        assert_eq!(
            saved.actions["a1"].inputs["strategy"],
            Value::String("fast".into())
        );
    }

    /// `limit` is one `Json` object in the registry but two canvas rows, so
    /// only the count row is Number-typed; the timeout stays free text.
    #[test]
    fn container_meta_rows_map_back_to_their_registry_field() {
        assert_eq!(
            declared_field_for_meta("Foreach", "__foreach").map(|f| f.kind),
            Some(FieldKind::Expression)
        );
        assert_eq!(
            declared_field_for_meta("Until", "__until").map(|f| f.kind),
            Some(FieldKind::Expression)
        );
        assert!(declared_field_for_meta("Foreach", "__limit_timeout").is_none());
    }

    /// Positions must be captured path-keyed, and the synthesized `If` branch
    /// wrappers must be unwrapped back into `actions`/`else_branch` rather
    /// than emitted as real actions.
    #[test]
    fn if_branches_round_trip_through_synthetic_wrappers() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "cond": {
                  "type": "If",
                  "expression": {"var": "x"},
                  "actions": { "t1": { "type": "log", "inputs": { "level": "info", "message": "yes" } } },
                  "else": { "actions": { "e1": { "type": "log", "inputs": { "level": "warn", "message": "no" } } } }
                }
              }
            }"#,
        )
        .unwrap();

        let state = load(&def, 0x808080, 0xff0000);
        // The wrappers are canvas-only scaffolding.
        assert!(
            state
                .get_node(&format!("cond{BRANCH_THEN}").into())
                .is_some()
        );

        let (saved, _layout) = save(&state, "f", "F", &FlowPreserved::default());
        let cond = &saved.actions["cond"];
        assert_eq!(cond.type_id, "If");
        assert!(cond.actions.as_ref().unwrap().contains_key("t1"));
        assert_eq!(
            cond.else_branch.as_ref().unwrap().actions["e1"].type_id,
            "log"
        );
        assert_eq!(
            cond.actions.as_ref().unwrap()["t1"].inputs["message"],
            Value::String("yes".into())
        );
        // No wrapper leaked into the saved actions map at any nesting level.
        assert!(!saved.actions.contains_key(BRANCH_THEN));
        assert_eq!(saved.actions.len(), 1);
    }

    /// A branch the file doesn't have still gets a wrapper to drop actions
    /// into, but saving it empty must not invent the branch: an empty `catch`
    /// would turn a `Try` that fails into one that swallows the failure.
    #[test]
    fn an_absent_branch_gets_a_wrapper_but_is_not_written_while_empty() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "cond": { "type": "If", "expression": {"var": "x"}, "actions": {} },
                "guard": { "type": "Try", "actions": {} }
              }
            }"#,
        )
        .unwrap();

        let mut state = load(&def, 0x808080, 0xff0000);
        for wrapper in [
            format!("cond{BRANCH_ELSE}"),
            format!("guard{BRANCH_CATCH}"),
        ] {
            let node = state.get_node(&wrapper.clone().into()).expect("wrapper");
            assert!(!node.deletable, "{wrapper} must not be deletable");
        }

        let (saved, _) = save(&state, "f", "F", &FlowPreserved::default());
        assert!(saved.actions["cond"].else_branch.is_none());
        assert!(saved.actions["guard"].catch.is_none());

        // Dropping an action into the wrapper is what creates the branch.
        state.nodes.push(
            FlowNode::new("recover", 0.0, 0.0)
                .node_type("log")
                .parent(format!("guard{BRANCH_CATCH}")),
        );
        state.rebuild_lookup();
        let (saved, _) = save(&state, "f", "F", &FlowPreserved::default());
        assert!(
            saved.actions["guard"]
                .catch
                .as_ref()
                .unwrap()
                .actions
                .contains_key("recover")
        );
    }

    /// An empty branch that *is* in the file stays in the file.
    #[test]
    fn a_declared_empty_branch_round_trips() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "guard": { "type": "Try", "actions": {}, "catch": { "actions": {} } }
              }
            }"#,
        )
        .unwrap();
        let state = load(&def, 0x808080, 0xff0000);
        let (saved, _) = save(&state, "f", "F", &FlowPreserved::default());
        assert!(saved.actions["guard"].catch.as_ref().unwrap().actions.is_empty());
    }

    #[test]
    fn run_after_is_rebuilt_from_incoming_edges() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "a1": { "type": "log", "inputs": { "level": "info", "message": "1" } },
                "a2": {
                  "type": "log",
                  "inputs": { "level": "info", "message": "2" },
                  "runAfter": { "a1": ["Failed"] }
                }
              }
            }"#,
        )
        .unwrap();

        let saved = round_trip(&def);
        assert_eq!(
            saved.actions["a2"].run_after.get("a1").map(Vec::as_slice),
            Some(&[RunOutcome::Failed][..])
        );
    }

    #[test]
    fn nested_container_bodies_round_trip_with_scoped_positions() {
        let def: WorkflowDefinition = serde_json::from_str(
            r#"{
              "id": "f", "name": "F",
              "actions": {
                "loop": {
                  "type": "Foreach",
                  "foreach": "items",
                  "actions": {
                    "inner": { "type": "log", "inputs": { "level": "info", "message": "x" } }
                  }
                }
              }
            }"#,
        )
        .unwrap();

        let state = load(&def, 0x808080, 0xff0000);
        let inner = state.get_node(&"inner".into()).expect("child node");
        assert_eq!(inner.parent_id.as_deref(), Some("loop"));

        let (_saved, layout) = save(&state, "f", "F", &FlowPreserved::default());
        // Same id at two nesting levels must not collide in the layout map.
        assert!(layout.positions.contains_key("loop"));
        assert!(layout.positions.contains_key("loop/inner"));
    }

    #[test]
    fn node_positions_are_captured_absolute() {
        let mut state = load(&empty_def(), 0x808080, 0xff0000);
        state.set_nodes(vec![FlowNode::new("a1", 12.0, 34.0).node_type("log")]);

        let (_saved, layout) = save(&state, "f", "F", &FlowPreserved::default());
        assert_eq!(layout.positions.get("a1"), Some(&(12.0, 34.0)));
    }
}
