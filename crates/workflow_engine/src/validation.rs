//! Design-time validation of a [`WorkflowDefinition`] against the action
//! registry. Produces a flat list of [`ValidationIssue`]s each anchored to
//! a specific node by its layout path (the same `"parent/child"` key
//! convention [`WorkflowLayout`] uses) and optionally to a specific input
//! field on that node.
//!
//! This is purely a **data function** — no gpui, no async. `designer_panel`
//! calls it after every load/save/property edit and stores the result;
//! `flows_panel` can call it before offering a "Run" button. The same
//! issues are displayed both in the canvas (node-level badge) and in the
//! properties inspector (per-field hint).
//!
//! ## What is checked
//!
//! | Check | Severity |
//! |---|---|
//! | Action type not in REGISTRY | Error |
//! | Runnable = false (a registered stub; none exist right now) | Error |
//! | Required input field is blank/missing | Error |
//! | `FieldKind::Json` value is not valid JSON | Error |
//! | `FieldKind::Expression` value is not a JSON object | Warning |
//! | `script` source: `File` path is empty | Error |
//! | `script` source: `File` path extension unrecognised and no runtime override | Warning |
//! | `runAfter` names an id that isn't in the same scope | Error |
//! | Cycle in `runAfter` graph (delegated to `scheduler::validate`) | Error |
//!
//! Checks that require filesystem access (does the script file exist?) are
//! intentionally out of scope here — they belong in a separate "pre-run"
//! check that can be async and workspace-aware.

use std::path::Path;

use serde_json::Value;

use crate::registry::{FieldKind, find as registry_find};
use crate::scheduler::validate as scheduler_validate;
use crate::schema::{Action, ActionMap, ScriptRuntime, ScriptSource, WorkflowDefinition};

// ── Public types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationSeverity {
    /// Blocks Run; Save still allowed.
    Error,
    /// Does not block Run; surfaces as a hint in the inspector.
    Warning,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    /// Layout-path key for the node this issue belongs to, e.g.
    /// `"fetch"` or `"loop/inner/step"`. Matches the keys in
    /// [`WorkflowLayout::positions`].
    pub node_path: String,
    /// The input field name this issue is attached to, or `None` for a
    /// node-level issue (unregistered type, cycle, etc.).
    pub field: Option<String>,
    pub message: String,
}

impl ValidationIssue {
    fn error(node_path: impl Into<String>, field: Option<&str>, msg: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Error,
            node_path: node_path.into(),
            field: field.map(str::to_string),
            message: msg.into(),
        }
    }

    fn warning(node_path: impl Into<String>, field: Option<&str>, msg: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Warning,
            node_path: node_path.into(),
            field: field.map(str::to_string),
            message: msg.into(),
        }
    }
}

// ── Entry point ──────────────────────────────────────────────────────────

/// Validate every action in `def`, returning all issues found. The list is
/// ordered: top-level actions first (in definition order), then nested
/// bodies depth-first. An empty list means the definition is clean.
///
/// The caller is responsible for merging issues with
/// [`WorkflowLayout`]-keyed node ids — `node_path` here uses the same
/// prefix convention layout keys use, so the lookup is a direct map get.
pub fn validate_definition(def: &WorkflowDefinition) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // 1. Structural graph validation (runAfter cycles / dangling refs) —
    //    one pass over the whole definition, errors reported at the top
    //    level since `scheduler::validate` returns a single string.
    if let Err(msg) = scheduler_validate(def) {
        // Attribute to the definition root rather than a specific node;
        // the message already names the offending id(s).
        issues.push(ValidationIssue::error("", None, msg));
    }

    // 2. Per-action checks, recursively.
    validate_map(&def.actions, "", &mut issues);

    issues
}

// ── Internal recursion ───────────────────────────────────────────────────

fn validate_map(actions: &ActionMap, path_prefix: &str, issues: &mut Vec<ValidationIssue>) {
    for (id, action) in actions {
        let node_path = if path_prefix.is_empty() {
            id.clone()
        } else {
            format!("{path_prefix}{id}")
        };
        if id == crate::engine::VARIABLES_KEY {
            issues.push(ValidationIssue::error(
                &node_path,
                None,
                format!(
                    "\"{id}\" is reserved for the flow's variables and can't be an action id"
                ),
            ));
        }
        validate_action(&node_path, action, issues);

        // Recurse into container bodies with appropriate path prefixes —
        // same convention as `workflow_json::build_actions_map`.
        if let Some(body) = &action.actions {
            match action.type_id.as_str() {
                "Foreach" | "Until" => {
                    validate_map(body, &format!("{node_path}/"), issues);
                }
                "If" => {
                    validate_map(body, &format!("{node_path}/then/"), issues);
                }
                "Try" => {
                    validate_map(body, &format!("{node_path}/try/"), issues);
                }
                _ => {
                    // Unknown container — still recurse so nothing is missed.
                    validate_map(body, &format!("{node_path}/"), issues);
                }
            }
        }
        if let Some(branch) = &action.else_branch {
            validate_map(&branch.actions, &format!("{node_path}/else/"), issues);
        }
        if let Some(branch) = &action.catch {
            validate_map(&branch.actions, &format!("{node_path}/catch/"), issues);
        }
    }
}

fn validate_action(node_path: &str, action: &Action, issues: &mut Vec<ValidationIssue>) {
    // ── Registry membership ───────────────────────────────────────────────
    let def = match registry_find(&action.type_id) {
        Some(d) => d,
        None => {
            issues.push(ValidationIssue::error(
                node_path,
                None,
                format!(
                    "Action type \"{}\" is not registered — it cannot be executed",
                    action.type_id
                ),
            ));
            // No point checking fields for an unknown type.
            return;
        }
    };

    // ── Runnable check ────────────────────────────────────────────────────
    if !def.runnable {
        issues.push(ValidationIssue::error(
            node_path,
            None,
            format!("\"{}\" is not yet implemented and cannot be run", def.label),
        ));
        // Still validate inputs so the user can author the action
        // while waiting for the executor to land.
    }

    // ── Per-field checks ──────────────────────────────────────────────────
    for input_def in def.inputs {
        let raw_value: Option<&Value> = action.inputs.get(input_def.name);

        // For `Foreach`/`Until`/`If` the expression lives on a dedicated
        // struct field rather than `inputs` — retrieve it if so.
        let alt_value: Option<Value>;
        let effective_value: Option<&Value> = if raw_value.is_none() {
            alt_value = match input_def.name {
                "foreach" => action.foreach.clone(),
                "until" => action.until.clone(),
                "expression" => action.expression.clone(),
                "limit" => action
                    .limit
                    .as_ref()
                    .map(|l| serde_json::to_value(l).unwrap_or(Value::Null)),
                _ => None,
            };
            alt_value.as_ref()
        } else {
            raw_value
        };

        match effective_value {
            None if input_def.required => {
                issues.push(ValidationIssue::error(
                    node_path,
                    Some(input_def.name),
                    format!("\"{}\" is required", input_def.name),
                ));
            }
            None => {
                // Optional and absent — fine.
            }
            Some(v) => {
                let text = value_text(v);
                if input_def.required && text.trim().is_empty() {
                    issues.push(ValidationIssue::error(
                        node_path,
                        Some(input_def.name),
                        format!("\"{}\" is required and must not be blank", input_def.name),
                    ));
                    continue;
                }

                // A structured value is a JSONLogic expression resolved at
                // run time, so only literal text can be checked here.
                if !input_def.options.is_empty()
                    && v.is_string()
                    && !input_def
                        .options
                        .iter()
                        .any(|option| option.eq_ignore_ascii_case(text.trim()))
                {
                    issues.push(ValidationIssue::error(
                        node_path,
                        Some(input_def.name),
                        format!(
                            "\"{}\" must be one of {}",
                            input_def.name,
                            input_def.options.join(", ")
                        ),
                    ));
                    continue;
                }

                match input_def.kind {
                    FieldKind::Json => {
                        validate_json_field(node_path, input_def.name, &text, issues);
                    }
                    FieldKind::Expression => {
                        validate_expression_field(node_path, input_def.name, &text, issues);
                    }
                    FieldKind::Number => {
                        validate_number_field(node_path, input_def.name, v, issues);
                    }
                    FieldKind::String | FieldKind::Bool => {
                        // No structural validation beyond required/blank above.
                    }
                }
            }
        }
    }

    // ── HTTP-specific checks ──────────────────────────────────────────────
    if action.type_id == "http"
        && action.inputs.contains_key("body")
        && action.inputs.contains_key("bodyExpression")
    {
        issues.push(ValidationIssue::error(
            node_path,
            Some("bodyExpression"),
            "Set either \"body\" or \"bodyExpression\", not both",
        ));
    }

    // ── Variable-specific checks ──────────────────────────────────────────
    if action.type_id == "SetVariable" {
        let literal = action.inputs.contains_key("value");
        let expression = action.inputs.contains_key("valueExpression");
        if literal == expression {
            issues.push(ValidationIssue::error(
                node_path,
                Some("value"),
                if literal {
                    "Set either \"value\" or \"valueExpression\", not both"
                } else {
                    "Set Variable needs a \"value\" or a \"valueExpression\""
                },
            ));
        }
    }
    if action.type_id == "InitVariable"
        && let Some(kind) = action
            .inputs
            .get("type")
            .and_then(Value::as_str)
            .and_then(crate::engine::VariableType::parse)
        && let Some(value) = action.inputs.get("value")
        && let Err(problem) = kind.coerce(value.clone())
    {
        issues.push(ValidationIssue::error(
            node_path,
            Some("value"),
            format!("\"value\" doesn't fit the variable's type: {problem}"),
        ));
    }
    if matches!(action.type_id.as_str(), "InitVariable" | "SetVariable")
        && action
            .inputs
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| name.contains('.'))
    {
        issues.push(ValidationIssue::error(
            node_path,
            Some("name"),
            "\"name\" can't contain a \".\" — it would be read as a path",
        ));
    }

    // ── Script-specific checks ────────────────────────────────────────────
    if action.type_id == "script"
        && action.inputs.contains_key("args")
        && action.inputs.contains_key("argsExpression")
    {
        issues.push(ValidationIssue::error(
            node_path,
            Some("argsExpression"),
            "Set either \"args\" or \"argsExpression\", not both",
        ));
    }
    if action.type_id == "script" {
        validate_script_source(node_path, action, issues);
    }
}

// ── Field-kind validators ────────────────────────────────────────────────

/// A `Value` from `action.inputs` — either already parsed JSON (when
/// loaded from a `.flow.json`) or a raw string that the canvas stored as
/// `Value::String`. Extract the text to re-validate.
fn value_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn validate_json_field(
    node_path: &str,
    field_name: &str,
    text: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    if text.trim().is_empty() {
        return; // blank optional — blank-required was already caught above
    }
    if serde_json::from_str::<Value>(text).is_err() {
        issues.push(ValidationIssue::error(
            node_path,
            Some(field_name),
            format!("\"{}\" must be valid JSON", field_name),
        ));
    }
}

fn validate_expression_field(
    node_path: &str,
    field_name: &str,
    text: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    if text.trim().is_empty() {
        return;
    }
    match serde_json::from_str::<Value>(text) {
        Ok(Value::Object(_)) => {
            // A JSONLogic expression is always a JSON object.
        }
        Ok(_) => {
            issues.push(ValidationIssue::warning(
                node_path,
                Some(field_name),
                format!(
                    "\"{}\" should be a JSONLogic expression (a JSON object like {{\"var\": \"…\"}})",
                    field_name
                ),
            ));
        }
        Err(_) => {
            issues.push(ValidationIssue::error(
                node_path,
                Some(field_name),
                format!(
                    "\"{}\" must be valid JSON (JSONLogic expression)",
                    field_name
                ),
            ));
        }
    }
}

fn validate_number_field(
    node_path: &str,
    field_name: &str,
    v: &Value,
    issues: &mut Vec<ValidationIssue>,
) {
    // Accept an actual JSON number or a string that parses as a number.
    let ok = match v {
        Value::Number(_) => true,
        Value::String(s) => s.trim().parse::<f64>().is_ok(),
        _ => false,
    };
    if !ok {
        issues.push(ValidationIssue::error(
            node_path,
            Some(field_name),
            format!("\"{}\" must be a number", field_name),
        ));
    }
}

// ── Script source checks ─────────────────────────────────────────────────

fn validate_script_source(node_path: &str, action: &Action, issues: &mut Vec<ValidationIssue>) {
    let Some(raw) = action.inputs.get("source") else {
        // Missing entirely — the required-field check above already fired.
        return;
    };
    let text = value_text(raw);
    if text.trim().is_empty() {
        return; // also already caught
    }

    let source: ScriptSource = match serde_json::from_str(&text) {
        Ok(s) => s,
        Err(_) => {
            // The Json-kind check above already flagged this.
            return;
        }
    };

    match &source {
        ScriptSource::File { path, runtime } => {
            if path.trim().is_empty() {
                issues.push(ValidationIssue::error(
                    node_path,
                    Some("source"),
                    "Script file path must not be blank",
                ));
            } else {
                // Check extension recognisability when no override is set.
                if runtime.is_none() && ScriptRuntime::from_path(Path::new(path.as_str())).is_none()
                {
                    issues.push(ValidationIssue::warning(
                        node_path,
                        Some("source"),
                        format!(
                            "Cannot infer runtime from \"{path}\" — set a runtime override in the Script editor"
                        ),
                    ));
                }
            }
        }
        ScriptSource::Inline { code, .. } => {
            if code.trim().is_empty() {
                issues.push(ValidationIssue::warning(
                    node_path,
                    Some("source"),
                    "Inline script body is empty",
                ));
            }
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use serde_json::json;

    use super::*;
    use crate::schema::{Action, RunOutcome, WorkflowDefinition};

    fn def_with(actions: impl IntoIterator<Item = (&'static str, Action)>) -> WorkflowDefinition {
        WorkflowDefinition {
            id: "test".into(),
            name: "Test".into(),
            actions: actions
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            outputs: IndexMap::new(),
        }
    }

    fn http_action() -> Action {
        let mut a = Action::new("http");
        a.inputs.insert("method".into(), json!("GET"));
        a.inputs.insert("url".into(), json!("https://example.com"));
        a
    }

    // ── Happy path ────────────────────────────────────────────────────────

    #[test]
    fn clean_http_action_produces_no_issues() {
        let def = def_with([("fetch", http_action())]);
        let issues = validate_definition(&def);
        assert!(issues.is_empty(), "unexpected issues: {issues:#?}");
    }

    #[test]
    fn clean_delay_action_produces_no_issues() {
        let mut a = Action::new("delay");
        a.inputs.insert("ms".into(), json!(500));
        let def = def_with([("wait", a)]);
        assert!(validate_definition(&def).is_empty());
    }

    // ── Closed value set ──────────────────────────────────────────────────

    #[test]
    fn http_method_outside_the_set_is_an_error() {
        let mut a = http_action();
        a.inputs.insert("method".into(), json!("FETCH"));
        let def = def_with([("fetch", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues.iter().any(|i| i.node_path == "fetch"
                && i.field.as_deref() == Some("method")
                && i.severity == ValidationSeverity::Error),
            "an unknown method should be an error: {issues:#?}"
        );
    }

    #[test]
    fn http_method_is_matched_case_insensitively() {
        for method in ["get", "Merge", " PATCH "] {
            let mut a = http_action();
            a.inputs.insert("method".into(), json!(method));
            let def = def_with([("fetch", a)]);
            let issues = validate_definition(&def);
            assert!(issues.is_empty(), "{method:?} should be accepted: {issues:#?}");
        }
    }

    #[test]
    fn http_body_and_body_expression_together_is_an_error() {
        let mut a = http_action();
        a.inputs.insert("body".into(), json!({"a": 1}));
        a.inputs
            .insert("bodyExpression".into(), json!({"var": "x.outputs.y"}));
        let def = def_with([("fetch", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues.iter().any(|i| i.node_path == "fetch"
                && i.field.as_deref() == Some("bodyExpression")
                && i.severity == ValidationSeverity::Error),
            "both body inputs should be an error: {issues:#?}"
        );
    }

    // ── Required field ────────────────────────────────────────────────────

    #[test]
    fn missing_required_field_is_an_error() {
        let mut a = Action::new("http");
        // Missing both "method" and "url".
        let def = def_with([("fetch", a.clone())]);
        let issues = validate_definition(&def);
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .collect();
        assert!(
            errors.iter().any(|i| i.field.as_deref() == Some("method")),
            "expected error on 'method', got: {issues:#?}"
        );
        assert!(
            errors.iter().any(|i| i.field.as_deref() == Some("url")),
            "expected error on 'url', got: {issues:#?}"
        );

        // Providing only one required field should fix that one error.
        a.inputs.insert("method".into(), json!("GET"));
        let def = def_with([("fetch", a)]);
        let issues = validate_definition(&def);
        assert!(
            !issues.iter().any(|i| i.field.as_deref() == Some("method")),
            "method error should be gone"
        );
        assert!(
            issues.iter().any(|i| i.field.as_deref() == Some("url")),
            "url error should remain"
        );
    }

    #[test]
    fn blank_required_string_field_is_an_error() {
        let mut a = Action::new("http");
        a.inputs.insert("method".into(), json!(""));
        a.inputs.insert("url".into(), json!("https://example.com"));
        let def = def_with([("fetch", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues
                .iter()
                .any(|i| i.field.as_deref() == Some("method")
                    && i.severity == ValidationSeverity::Error),
            "blank method should be an error: {issues:#?}"
        );
    }

    // ── JSON field ────────────────────────────────────────────────────────

    #[test]
    fn invalid_json_body_is_an_error() {
        let mut a = http_action();
        a.inputs.insert("body".into(), json!("not { valid json"));
        let def = def_with([("fetch", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues
                .iter()
                .any(|i| i.field.as_deref() == Some("body")
                    && i.severity == ValidationSeverity::Error),
            "invalid JSON body should be an error: {issues:#?}"
        );
    }

    #[test]
    fn valid_json_body_has_no_issue() {
        let mut a = http_action();
        a.inputs.insert("body".into(), json!({"key": "value"}));
        let def = def_with([("fetch", a)]);
        assert!(validate_definition(&def).is_empty());
    }

    // ── Expression field ──────────────────────────────────────────────────

    #[test]
    fn non_object_expression_is_a_warning() {
        let mut a = Action::new("transform");
        // A JSON array is not a valid JSONLogic expression.
        a.inputs.insert("expression".into(), json!([1, 2, 3]));
        let def = def_with([("t", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues
                .iter()
                .any(|i| i.field.as_deref() == Some("expression")
                    && i.severity == ValidationSeverity::Warning),
            "array expression should be a warning: {issues:#?}"
        );
    }

    #[test]
    fn object_expression_is_valid() {
        let mut a = Action::new("transform");
        a.inputs
            .insert("expression".into(), json!({"var": "fetch.outputs.body"}));
        let def = def_with([("t", a)]);
        assert!(validate_definition(&def).is_empty());
    }

    // ── Unregistered type ─────────────────────────────────────────────────

    #[test]
    fn unknown_type_id_is_an_error() {
        let a = Action::new("NotARealType");
        let def = def_with([("x", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues.iter().any(|i| i.field.is_none()
                && i.severity == ValidationSeverity::Error
                && i.node_path == "x"),
            "unknown type should produce a node-level error: {issues:#?}"
        );
    }

    // ── Unrunnable type ───────────────────────────────────────────────────

    // FUTURE FEATURE — the registry has no `runnable: false` entry while
    // `DbMigration`/`AiCheck` are parked (see `registry::REGISTRY`), so the
    // runnable check has nothing to exercise it. Restore this with them.
    //
    // #[test]
    // fn unrunnable_type_is_an_error() {
    //     let mut a = Action::new("DbMigration");
    //     a.inputs
    //         .insert("sql_file".into(), json!("migrations/001.sql"));
    //     a.inputs.insert("connection_id".into(), json!("default"));
    //     let def = def_with([("migrate", a)]);
    //     let issues = validate_definition(&def);
    //     assert!(
    //         issues.iter().any(|i| i.node_path == "migrate"
    //             && i.field.is_none()
    //             && i.severity == ValidationSeverity::Error),
    //         "unrunnable DbMigration should be an error: {issues:#?}"
    //     );
    // }

    /// A flow written before the future-feature types were parked still
    /// blocks Run, now as an unknown type.
    #[test]
    fn parked_future_types_are_errors() {
        for type_id in ["sendEmail", "DbMigration", "AiCheck"] {
            let def = def_with([("parked", Action::new(type_id))]);
            let issues = validate_definition(&def);
            assert!(
                issues
                    .iter()
                    .any(|i| i.node_path == "parked" && i.severity == ValidationSeverity::Error),
                "{type_id} should be an error: {issues:#?}"
            );
        }
    }

    // ── runAfter / cycle ──────────────────────────────────────────────────

    #[test]
    fn dangling_run_after_is_an_error() {
        let mut a = http_action();
        a.run_after
            .insert("ghost".into(), vec![RunOutcome::Succeeded]);
        let def = def_with([("fetch", a)]);
        let issues = validate_definition(&def);
        // The scheduler_validate error is attached to the root ("") path.
        assert!(
            issues
                .iter()
                .any(|i| i.severity == ValidationSeverity::Error),
            "dangling runAfter should produce an error: {issues:#?}"
        );
    }

    #[test]
    fn cycle_in_run_after_is_an_error() {
        let mut a = Action::new("delay");
        a.inputs.insert("ms".into(), json!(10));
        a.run_after.insert("b".into(), vec![RunOutcome::Succeeded]);

        let mut b = Action::new("delay");
        b.inputs.insert("ms".into(), json!(10));
        b.run_after.insert("a".into(), vec![RunOutcome::Succeeded]);

        let def = def_with([("a", a), ("b", b)]);
        let issues = validate_definition(&def);
        assert!(
            issues
                .iter()
                .any(|i| i.severity == ValidationSeverity::Error),
            "cycle should produce an error: {issues:#?}"
        );
    }

    // ── Script source ─────────────────────────────────────────────────────

    #[test]
    fn script_with_blank_file_path_is_an_error() {
        let mut a = Action::new("script");
        let source = ScriptSource::File {
            path: String::new(),
            runtime: None,
        };
        a.inputs
            .insert("source".into(), serde_json::to_value(source).unwrap());
        let def = def_with([("s", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues
                .iter()
                .any(|i| i.field.as_deref() == Some("source")
                    && i.severity == ValidationSeverity::Error),
            "blank file path should be an error: {issues:#?}"
        );
    }

    #[test]
    fn script_with_unknown_extension_and_no_override_is_a_warning() {
        let mut a = Action::new("script");
        let source = ScriptSource::File {
            path: "scripts/thing.xyz".into(),
            runtime: None,
        };
        a.inputs
            .insert("source".into(), serde_json::to_value(source).unwrap());
        let def = def_with([("s", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues.iter().any(|i| i.field.as_deref() == Some("source")
                && i.severity == ValidationSeverity::Warning),
            "unknown extension should be a warning: {issues:#?}"
        );
    }

    #[test]
    fn script_with_known_extension_is_clean() {
        let mut a = Action::new("script");
        let source = ScriptSource::File {
            path: "scripts/thing.py".into(),
            runtime: None,
        };
        a.inputs
            .insert("source".into(), serde_json::to_value(source).unwrap());
        let def = def_with([("s", a)]);
        assert!(validate_definition(&def).is_empty());
    }

    #[test]
    fn inline_script_with_empty_code_is_a_warning() {
        let mut a = Action::new("script");
        let source = ScriptSource::Inline {
            runtime: crate::schema::ScriptRuntime::Python,
            code: String::new(),
        };
        a.inputs
            .insert("source".into(), serde_json::to_value(source).unwrap());
        let def = def_with([("s", a)]);
        let issues = validate_definition(&def);
        assert!(
            issues.iter().any(|i| i.field.as_deref() == Some("source")
                && i.severity == ValidationSeverity::Warning),
            "empty inline code should be a warning: {issues:#?}"
        );
    }

    // ── Nested containers ─────────────────────────────────────────────────

    #[test]
    fn issues_inside_foreach_body_carry_prefixed_paths() {
        let inner = Action::new("http");
        // Missing required "method" and "url"
        let mut outer = Action::new("Foreach");
        outer.foreach = Some(json!({"var": "items"}));
        outer.actions = Some(
            std::iter::once(("step".to_string(), inner)).collect::<crate::schema::ActionMap>(),
        );

        let def = def_with([("loop", outer)]);
        let issues = validate_definition(&def);

        // The inner action's path should be "loop/step".
        assert!(
            issues.iter().any(|i| i.node_path == "loop/step"),
            "inner issues should carry path 'loop/step': {issues:#?}"
        );
    }
}
