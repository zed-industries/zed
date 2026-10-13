//! Request-body companions for `http` actions.
//!
//! An `http` action's `body` is a [`FieldKind::Json`] input — arbitrary JSON,
//! which is exactly the kind of thing that stops being editable in a
//! single-line inspector row. Script solved the same problem with a companion
//! file (see [`crate::script_companion`]); this module is the same mechanism
//! applied to bodies, so a body gets a real `.json` file with JSON language
//! server support, and the flow and the file stay in sync in both directions.
//!
//! Two things make bodies different from script code, and both are handled
//! here rather than by duplicating the script module:
//!
//! * **Comparison is semantic, not byte-wise.** A script body is text, so
//!   `script_companion` can compare bytes. JSON has insignificant
//!   whitespace and key order, so a file reformatted by the user's editor
//!   would look like a change and raise a phantom conflict. Everything is
//!   compared through [`canonical_body`] instead.
//! * **Malformed JSON must round-trip verbatim.** A body the user is
//!   mid-way through typing is not valid JSON. Rewriting it "canonically"
//!   would either fail and lose the text, or normalize away the mistake they
//!   were about to fix. An unparseable body is stored and compared as-is.

use std::path::{Path, PathBuf};

use serde_json::Value;
use workflow_engine::{
    ActionMap, WorkflowDefinition,
    schema::{HTTP_BODY_DIR_SUFFIX, HTTP_BODY_FILE_SUFFIX},
};

use crate::script_companion::{
    Sync, SyncState, action_key, companion_component, find_action_by_path, write_companion,
};

/// The `body` input name, per `registry.rs`'s `http` entry.
const BODY_INPUT: &str = "body";

/// The action type this module manages companions for.
const HTTP_TYPE_ID: &str = "http";

/// Path of the request-body companion for `action_path`: a sibling directory of
/// the flow file named after it, with the action's nesting scope preserved as
/// subdirectories, so `<flow>.flow-http/<scope>/<action_id>.body.json`.
///
/// Mirrors [`crate::script_companion::companion_path`] exactly, minus the
/// runtime-extension step, which only scripts need.
pub fn companion_path(flow_path: &Path, action_path: &[String]) -> Result<PathBuf, String> {
    let file_name = flow_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("invalid flow path: {}", flow_path.display()))?;
    let suffix = ".flow.json";
    let suffix_start = file_name
        .len()
        .checked_sub(suffix.len())
        .ok_or_else(|| format!("{} doesn't end in .flow.json", flow_path.display()))?;
    if !file_name[suffix_start..].eq_ignore_ascii_case(suffix) {
        return Err(format!("{} doesn't end in .flow.json", flow_path.display()));
    }
    let flow_name = &file_name[..suffix_start];
    let (action_id, scopes) = action_path
        .split_last()
        .ok_or_else(|| "http action path is empty".to_string())?;

    let directory = flow_path.with_file_name(format!("{flow_name}{HTTP_BODY_DIR_SUFFIX}"));
    let directory = scopes.iter().fold(directory, |path, component| {
        path.join(companion_component(component))
    });
    Ok(directory.join(format!(
        "{}{HTTP_BODY_FILE_SUFFIX}",
        companion_component(action_id)
    )))
}

/// The `body`-property text for `action_path` in `definition` — the exact
/// inverse of what `workflow_json` does when loading a flow, so a companion
/// hydrated by [`sync_inline_bodies`] shows up in the inspector spelled the
/// way a freshly loaded one is.
pub fn body_text_for_action_path(
    definition: &WorkflowDefinition,
    action_path: &[String],
) -> Option<String> {
    let action = find_action_by_path(&definition.actions, action_path)?;
    Some(crate::workflow_json::value_to_string(
        action.inputs.get(BODY_INPUT)?,
    ))
}

/// Renders `value` with object keys sorted, so two bodies differing only in key
/// order produce identical text.
///
/// Hand-rolled rather than `to_string_pretty` because the workspace enables
/// `serde_json`'s `preserve_order`: its `Value` maps are `IndexMap`s, so the
/// built-in serializer preserves insertion order and two key-reordered bodies
/// would compare unequal — precisely the phantom conflict this comparison
/// exists to prevent.
fn write_canonical(value: &Value, out: &mut String, indent: usize) {
    let pad = |out: &mut String, depth: usize| out.push_str(&"    ".repeat(depth));
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_unstable();
            out.push_str("{\n");
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                pad(out, indent + 1);
                // Round-trip through `Value::String` so keys with quotes or
                // newlines in them come out escaped, not raw.
                out.push_str(&Value::String(key.clone()).to_string());
                out.push_str(": ");
                write_canonical(&map[key], out, indent + 1);
            }
            out.push('\n');
            pad(out, indent);
            out.push('}');
        }
        Value::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(",\n");
                }
                pad(out, indent + 1);
                write_canonical(item, out, indent + 1);
            }
            out.push('\n');
            pad(out, indent);
            out.push(']');
        }
        scalar => out.push_str(&scalar.to_string()),
    }
}

/// Normalizes body text for comparison and for writing to a companion.
///
/// Returns pretty-printed, key-sorted JSON when `text` parses, and `text`
/// unchanged when it doesn't. The unparseable case is the point: a half-typed
/// body is still the user's work, and silently dropping or reformatting it
/// would lose it. Note this also means a malformed body round-trips
/// byte-for-byte, so the file and the inspector keep showing the same broken
/// text instead of two versions of it.
fn canonical_body(text: &str) -> String {
    match serde_json::from_str::<Value>(text) {
        Ok(value) => {
            let mut out = String::new();
            write_canonical(&value, &mut out, 0);
            out
        }
        Err(_) => text.to_string(),
    }
}

/// Whether two body texts are the same body, ignoring JSON-insignificant
/// differences: whitespace, indentation, and key order.
fn bodies_equivalent(left: &str, right: &str) -> bool {
    canonical_body(left) == canonical_body(right)
}

/// Reconciles every `http` body in `definition` with its companion file, in
/// both directions, and reports what happened to each action by
/// [`action_key`].
///
/// The rule, per action, is `script_companion::sync_inline_scripts`' — whichever
/// side changed since `state` last saw the pair wins, and if *both* changed
/// this returns `Err` rather than picking a loser. With no record for an
/// action (the first pass after opening the flow) a saved companion is
/// treated as the latest source, so it hydrates the flow.
///
/// Only actions that actually declare a `body` are touched. `body` is optional
/// in the registry (it's meaningless for `GET`/`DELETE`), so an `http` action
/// without one has no companion, and one is never invented for it — same
/// reasoning as a script's `File` source being left alone.
pub fn sync_inline_bodies(
    definition: &mut WorkflowDefinition,
    flow_path: &Path,
    state: &mut SyncState,
) -> Result<Vec<(String, Sync)>, String> {
    let mut report = Vec::new();
    sync_action_map(
        &mut definition.actions,
        flow_path,
        &mut Vec::new(),
        state,
        &mut report,
    )?;
    Ok(report)
}

/// Writes the flow's body for `action_path` through to its companion, for the
/// inspector's "Edit body" button.
///
/// Never hydrates, for the same reason [`crate::script_companion::write_through`]
/// doesn't: the caller wants to open the companion for the body the user can
/// see in the inspector, so pulling the file back into a throwaway definition
/// would record a sync for content the canvas doesn't have, and the next save
/// would write the stale inline body over the user's external edit.
pub fn write_through(
    definition: &WorkflowDefinition,
    flow_path: &Path,
    action_path: &[String],
    state: &mut SyncState,
) -> Result<Sync, String> {
    let key = action_key(action_path);
    let action = find_action_by_path(&definition.actions, action_path)
        .ok_or_else(|| format!("no http action at {key}"))?;
    let body = action
        .inputs
        .get(BODY_INPUT)
        .ok_or_else(|| format!("http action {key} has no body input"))?;
    let body = crate::workflow_json::value_to_string(body);

    let path = companion_path(flow_path, action_path)?;
    let file_body = match std::fs::read_to_string(&path) {
        Ok(file_body) => Some(file_body),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(format!("couldn't read {}: {error}", path.display())),
    };

    let canonical = canonical_body(&body);
    match file_body {
        // No companion yet: the flow's body becomes it.
        None => {
            write_companion(&path, &canonical)?;
            state.record(action_path, &canonical);
            Ok(Sync::Created)
        }
        Some(file_body) if bodies_equivalent(&file_body, &body) => {
            state.record(action_path, &canonical);
            Ok(Sync::InSync)
        }
        // The companion is what we last synced and the flow's body moved: the
        // user edited the body in the inspector, so it wins.
        Some(file_body)
            if state.last_synced(action_path) == Some(canonical_body(&file_body).as_str()) =>
        {
            write_companion(&path, &canonical)?;
            state.record(action_path, &canonical);
            Ok(Sync::Written)
        }
        _ => Err(conflict_error(&path, action_path)),
    }
}

/// The single message for "both sides of this body changed", worded to match
/// `script_companion`'s so the advice across the two can't drift apart.
fn conflict_error(path: &Path, action_path: &[String]) -> String {
    format!(
        "{} and the request body for http {} both changed since the last sync - \
         reconcile them by hand, or delete the companion to go back to inline-only",
        path.display(),
        action_path.join("/")
    )
}

/// Whether the flow file was modified after `companion`. `false` when either
/// time can't be read, which leaves the companion as the latest source.
fn flow_is_newer(flow_path: &Path, companion: &Path) -> bool {
    let modified = |path: &Path| std::fs::metadata(path).and_then(|meta| meta.modified()).ok();
    matches!(
        (modified(flow_path), modified(companion)),
        (Some(flow), Some(companion)) if flow > companion
    )
}

fn sync_action_map(
    actions: &mut ActionMap,
    flow_path: &Path,
    action_path: &mut Vec<String>,
    state: &mut SyncState,
    report: &mut Vec<(String, Sync)>,
) -> Result<(), String> {
    for (id, action) in actions {
        action_path.push(id.clone());

        if action.type_id == HTTP_TYPE_ID
            && let Some(body_value) = action.inputs.get(BODY_INPUT).cloned()
        {
            let body = crate::workflow_json::value_to_string(&body_value);
            let path = companion_path(flow_path, action_path)?;
            let file_body = match std::fs::read_to_string(&path) {
                Ok(file_body) => Some(file_body),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
                Err(error) => {
                    return Err(format!("couldn't read {}: {error}", path.display()));
                }
            };
            // Cloned rather than borrowed: `state` is recorded into below, and
            // a `&str` into it would still be live at that point.
            let previous = state.last_synced(action_path).map(canonical_body);
            let mut effective = body.clone();
            let outcome = match (previous, file_body.as_deref()) {
                // No companion yet: the flow's body becomes it.
                (_, None) => {
                    write_companion(&path, &canonical_body(&body))?;
                    Sync::Created
                }
                (_, Some(file_body)) if bodies_equivalent(file_body, &body) => Sync::InSync,
                // The companion is what we last synced and the flow's body
                // moved: the user edited the body, so it wins.
                (Some(previous), Some(file_body)) if bodies_equivalent(&previous, file_body) => {
                    write_companion(&path, &canonical_body(&body))?;
                    Sync::Written
                }
                // The flow's body is what we last synced and the companion
                // moved: the user edited the file, so it wins.
                (Some(previous), Some(file_body)) if bodies_equivalent(&previous, &body) => {
                    effective = file_body.to_string();
                    Sync::Hydrated
                }
                // No record for this action - the flow was just opened, so
                // nothing says which side moved. The file modified last is
                // the edit to keep: a flow changed since its companion (in
                // the raw JSON, or by a pull) wins over a stale companion.
                (None, Some(_)) if flow_is_newer(flow_path, &path) => {
                    write_companion(&path, &canonical_body(&body))?;
                    Sync::Written
                }
                // Otherwise a saved companion is the latest source.
                (None, Some(file_body)) => {
                    effective = file_body.to_string();
                    Sync::Hydrated
                }
                // Both sides moved since the last sync. Any winner destroys one
                // of the two edits, so refuse and let the user decide.
                (Some(_), Some(_)) => return Err(conflict_error(&path, action_path)),
            };
            if matches!(outcome, Sync::Hydrated) {
                action.inputs.insert(
                    BODY_INPUT.to_string(),
                    crate::workflow_json::string_to_value(&effective),
                );
            }
            state.record(action_path, &canonical_body(&effective));
            report.push((action_key(action_path), outcome));
        }

        match action.type_id.as_str() {
            "Foreach" | "Until" => {
                if let Some(body) = &mut action.actions {
                    sync_action_map(body, flow_path, action_path, state, report)?;
                }
            }
            "If" => {
                if let Some(body) = &mut action.actions {
                    action_path.push("then".to_string());
                    sync_action_map(body, flow_path, action_path, state, report)?;
                    action_path.pop();
                }
                if let Some(branch) = &mut action.else_branch {
                    action_path.push("else".to_string());
                    sync_action_map(&mut branch.actions, flow_path, action_path, state, report)?;
                    action_path.pop();
                }
            }
            "Try" => {
                if let Some(body) = &mut action.actions {
                    action_path.push("try".to_string());
                    sync_action_map(body, flow_path, action_path, state, report)?;
                    action_path.pop();
                }
                if let Some(branch) = &mut action.catch {
                    action_path.push("catch".to_string());
                    sync_action_map(&mut branch.actions, flow_path, action_path, state, report)?;
                    action_path.pop();
                }
            }
            _ => {}
        }
        action_path.pop();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::script_companion::action_path_for_node;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use workflow_engine::Action;

    static NEXT_TEMP_ID: AtomicUsize = AtomicUsize::new(0);

    fn temp_dir() -> PathBuf {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "designer-http-companion-{}-{id}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn flow_path(dir: &Path) -> PathBuf {
        dir.join("checkout.flow.json")
    }

    fn http_action(body: Option<&str>) -> Action {
        let mut action = Action::new("http");
        if let Some(body) = body {
            action.inputs.insert(
                BODY_INPUT.into(),
                crate::workflow_json::string_to_value(body),
            );
        }
        action
    }

    fn definition(actions: Vec<(&str, Action)>) -> WorkflowDefinition {
        WorkflowDefinition {
            id: "checkout".into(),
            name: "Checkout".into(),
            actions: actions
                .into_iter()
                .map(|(id, action)| (id.to_string(), action))
                .collect(),
            outputs: Default::default(),
        }
    }

    #[test]
    fn path_is_a_sibling_dir_preserving_scope() {
        let dir = temp_dir();
        let path = companion_path(
            &flow_path(&dir),
            &["loop".to_string(), "then".to_string(), "call".to_string()],
        )
        .expect("path builds");
        assert_eq!(
            path,
            dir.join("checkout.flow-http/loop/then/call.body.json")
        );
    }

    #[test]
    fn path_escapes_action_ids() {
        // An id with a path separator or `..` must land in a single escaped
        // segment, so it can neither create a nested directory nor escape the
        // companion directory.
        let dir = temp_dir();
        let path = companion_path(
            &flow_path(&dir),
            &["../../etc".to_string(), "passwd".to_string()],
        )
        .expect("path builds");

        // The scope stays inside the companion directory...
        let companion_dir = dir.join("checkout.flow-http");
        assert_eq!(
            path.parent().unwrap(),
            companion_dir.join("%2E%2E%2F%2E%2E%2Fetc")
        );
        // ...as one escaped segment, and the id is the only file name.
        assert_eq!(path.file_name().unwrap(), "passwd.body.json");
        assert!(path.starts_with(&companion_dir));
    }

    #[test]
    fn path_rejects_a_non_flow_file() {
        let dir = temp_dir();
        let err = companion_path(&dir.join("notes.txt"), &["a".to_string()])
            .expect_err("a non-flow file has no companion location");
        assert!(err.contains(".flow.json"), "{err}");
    }

    #[test]
    fn creates_a_companion_on_first_sync() {
        let dir = temp_dir();
        let mut def = definition(vec![("call", http_action(Some(r#"{"a":1}"#)))]);
        let mut state = SyncState::default();

        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(report, vec![("call".to_string(), Sync::Created)]);

        let written =
            std::fs::read_to_string(dir.join("checkout.flow-http/call.body.json")).unwrap();
        // Stored pretty-printed so the file is pleasant to edit by hand, and
        // key-sorted so two flows with the same body produce identical files.
        assert_eq!(written, canonical_body(r#"{"a":1}"#));
        assert_eq!(
            serde_json::from_str::<Value>(&written).unwrap(),
            serde_json::json!({"a": 1})
        );
    }

    #[test]
    fn does_not_invent_a_companion_for_a_body_less_action() {
        let dir = temp_dir();
        let mut def = definition(vec![("call", http_action(None))]);
        let mut state = SyncState::default();

        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert!(report.is_empty(), "no body, no companion: {report:?}");
        assert!(!dir.join("checkout.flow-http").exists());
    }

    #[test]
    fn reformatted_companion_is_not_a_conflict() {
        // The whole reason comparison is semantic: a user's editor
        // reindenting the file must not look like a change.
        let dir = temp_dir();
        std::fs::create_dir_all(dir.join("checkout.flow-http")).unwrap();
        std::fs::write(
            dir.join("checkout.flow-http/call.body.json"),
            "{\n    \"b\": 2,\n    \"a\": 1\n}",
        )
        .unwrap();

        let mut def = definition(vec![("call", http_action(Some(r#"{"a":1,"b":2}"#)))]);
        let mut state = SyncState::default();

        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(report, vec![("call".to_string(), Sync::InSync)]);
    }

    #[test]
    fn hydrates_the_flow_when_only_the_companion_changed() {
        let dir = temp_dir();
        std::fs::create_dir_all(dir.join("checkout.flow-http")).unwrap();
        std::fs::write(
            dir.join("checkout.flow-http/call.body.json"),
            r#"{"edited":true}"#,
        )
        .unwrap();

        let mut def = definition(vec![("call", http_action(Some(r#"{"edited":false}"#)))]);
        let mut state = SyncState::default();

        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(report, vec![("call".to_string(), Sync::Hydrated)]);
        assert_eq!(
            def.actions["call"].inputs[BODY_INPUT],
            serde_json::json!({"edited": true})
        );
    }

    /// Opening a flow whose file is newer than a differing companion (the
    /// body was edited in the raw JSON, or arrived in a pull) keeps the
    /// flow's body instead of reverting it to the stale companion.
    #[test]
    fn a_newer_flow_file_wins_over_a_stale_companion_on_open() {
        let dir = temp_dir();
        let companion = dir.join("checkout.flow-http/call.body.json");
        std::fs::create_dir_all(companion.parent().unwrap()).unwrap();
        std::fs::write(&companion, r#"{"v":1}"#).unwrap();
        std::fs::File::options()
            .write(true)
            .open(&companion)
            .unwrap()
            .set_modified(std::time::SystemTime::now() - std::time::Duration::from_secs(60))
            .unwrap();
        std::fs::write(flow_path(&dir), "{}").unwrap();

        let mut def = definition(vec![("call", http_action(Some(r#"{"v":2}"#)))]);
        let mut state = SyncState::default();

        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(report, vec![("call".to_string(), Sync::Written)]);
        assert_eq!(
            def.actions["call"].inputs[BODY_INPUT],
            serde_json::json!({"v": 2})
        );
        assert!(bodies_equivalent(
            &std::fs::read_to_string(&companion).unwrap(),
            r#"{"v":2}"#
        ));
    }

    /// The other way round: a companion edited after the flow was last
    /// written is the newer work, so it still hydrates.
    #[test]
    fn a_newer_companion_still_hydrates_on_open() {
        let dir = temp_dir();
        std::fs::write(flow_path(&dir), "{}").unwrap();
        std::fs::File::options()
            .write(true)
            .open(flow_path(&dir))
            .unwrap()
            .set_modified(std::time::SystemTime::now() - std::time::Duration::from_secs(60))
            .unwrap();
        let companion = dir.join("checkout.flow-http/call.body.json");
        std::fs::create_dir_all(companion.parent().unwrap()).unwrap();
        std::fs::write(&companion, r#"{"v":3}"#).unwrap();

        let mut def = definition(vec![("call", http_action(Some(r#"{"v":2}"#)))]);
        let mut state = SyncState::default();

        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(report, vec![("call".to_string(), Sync::Hydrated)]);
        assert_eq!(
            def.actions["call"].inputs[BODY_INPUT],
            serde_json::json!({"v": 3})
        );
    }

    #[test]
    fn writes_the_flow_through_when_only_the_flow_changed() {
        let dir = temp_dir();
        let mut def = definition(vec![("call", http_action(Some(r#"{"v":1}"#)))]);
        let mut state = SyncState::default();
        sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();

        // Simulate an inspector edit.
        def.actions["call"].inputs.insert(
            BODY_INPUT.into(),
            crate::workflow_json::string_to_value(r#"{"v":2}"#),
        );
        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(report, vec![("call".to_string(), Sync::Written)]);
        assert_eq!(
            std::fs::read_to_string(dir.join("checkout.flow-http/call.body.json")).unwrap(),
            canonical_body(r#"{"v":2}"#)
        );
    }

    #[test]
    fn refuses_when_both_sides_changed() {
        let dir = temp_dir();
        let mut def = definition(vec![("call", http_action(Some(r#"{"v":1}"#)))]);
        let mut state = SyncState::default();
        sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();

        // Inspector edit *and* an external edit to the same companion.
        def.actions["call"].inputs.insert(
            BODY_INPUT.into(),
            crate::workflow_json::string_to_value(r#"{"v":2}"#),
        );
        std::fs::write(dir.join("checkout.flow-http/call.body.json"), r#"{"v":3}"#).unwrap();

        let err = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state)
            .expect_err("both sides changed");
        assert!(err.contains("both changed"), "{err}");
    }

    #[test]
    fn malformed_bodies_round_trip_verbatim() {
        // A body mid-edit isn't valid JSON. It must not be normalized away,
        // and it must not collide with a genuinely different broken body.
        let dir = temp_dir();
        let mut def = definition(vec![("call", http_action(Some("{not json")))]);
        let mut state = SyncState::default();

        sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("checkout.flow-http/call.body.json")).unwrap(),
            "{not json"
        );

        // Still stable across a second pass rather than churning.
        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(report, vec![("call".to_string(), Sync::InSync)]);
    }

    #[test]
    fn syncs_nested_bodies() {
        let dir = temp_dir();
        let mut if_action = Action::new("If");
        if_action.actions = Some(
            [("inner".to_string(), http_action(Some(r#"{"n":1}"#)))]
                .into_iter()
                .collect(),
        );
        let mut def = definition(vec![("branch", if_action)]);
        let mut state = SyncState::default();

        let report = sync_inline_bodies(&mut def, &flow_path(&dir), &mut state).unwrap();
        assert_eq!(
            report,
            vec![("branch/then/inner".to_string(), Sync::Created)]
        );
        assert!(
            dir.join("checkout.flow-http/branch/then/inner.body.json")
                .is_file()
        );
    }

    #[test]
    fn write_through_creates_then_writes() {
        let dir = temp_dir();
        let mut def = definition(vec![("call", http_action(Some(r#"{"v":1}"#)))]);
        let mut state = SyncState::default();
        let action_path = vec!["call".to_string()];

        assert_eq!(
            write_through(&def, &flow_path(&dir), &action_path, &mut state).unwrap(),
            Sync::Created
        );

        // An inspector edit must be pushed through, not dropped.
        def.actions["call"].inputs.insert(
            BODY_INPUT.into(),
            crate::workflow_json::string_to_value(r#"{"v":9}"#),
        );
        assert_eq!(
            write_through(&def, &flow_path(&dir), &action_path, &mut state).unwrap(),
            Sync::Written
        );
        assert_eq!(
            std::fs::read_to_string(dir.join("checkout.flow-http/call.body.json")).unwrap(),
            canonical_body(r#"{"v":9}"#)
        );
    }

    #[test]
    fn write_through_reports_an_external_edit_as_a_conflict() {
        let dir = temp_dir();
        let def = definition(vec![("call", http_action(Some(r#"{"v":1}"#)))]);
        let mut state = SyncState::default();
        let action_path = vec!["call".to_string()];
        write_through(&def, &flow_path(&dir), &action_path, &mut state).unwrap();

        // The user edits the file out of band, then clicks "Edit body" again
        // without the canvas having noticed. Overwriting would destroy the
        // external edit.
        std::fs::write(dir.join("checkout.flow-http/call.body.json"), r#"{"v":7}"#).unwrap();
        let err = write_through(&def, &flow_path(&dir), &action_path, &mut state)
            .expect_err("external edit is a conflict");
        assert!(err.contains("both changed"), "{err}");
    }

    #[test]
    fn action_path_resolves_through_containers() {
        // Reuses the script module's traversal, so a body and a script in the
        // same nested scope resolve to the same action path shape.
        let mut if_action = Action::new("If");
        if_action.actions = Some(
            [("inner".to_string(), http_action(Some("{}")))]
                .into_iter()
                .collect(),
        );
        let def = definition(vec![("branch", if_action)]);

        let state = gpui_flow::FlowState::new(
            vec![
                gpui_flow::FlowNode::new("branch", 0.0, 0.0).node_type("If"),
                gpui_flow::FlowNode::new("branch__branch_then", 0.0, 0.0)
                    .node_type(crate::workflow_json::BRANCH_THEN)
                    .parent("branch"),
                gpui_flow::FlowNode::new("inner", 0.0, 0.0).parent("branch__branch_then"),
            ],
            Vec::new(),
        );

        let path = action_path_for_node(&def, &state, "inner").unwrap();
        assert_eq!(
            path,
            vec![
                "branch".to_string(),
                "then".to_string(),
                "inner".to_string()
            ]
        );
        assert_eq!(
            body_text_for_action_path(&def, &path).as_deref(),
            Some("{}")
        );
    }
}
