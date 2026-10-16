//! §5 of `DOCS/workflow-schema/execution-engine-design.md` — container
//! semantics. Each container type gets its own executor here, which calls
//! back into `scheduler::run_level` for its own nested body/bodies — the
//! recursion §3's own doc comment describes ("a container's own body is
//! scheduled by a *separate* recursive call, not by [`run_level`] reaching
//! into it").
//!
//! All four container types (`If`, `Foreach`, `Until`, `Try`/`Catch`) are
//! implemented, built in that order per the design doc's suggested build
//! order (`If` simplest — no loop/limit concerns — through `Try`/`Catch`
//! last, once per-leaf failure reporting was proven correct).

use std::path::Path;

use futures::future::BoxFuture;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, StreamExt};
use serde_json::{Map, Value};

use super::engine::RunContext;
use super::executors::execute_leaf;
use super::registry;
use super::scheduler::run_level;
use super::schema::{Action, RunOutcome};
use super::status::{IterationScope, RunStatus, StatusSink};

/// Scans a just-finished `LevelResult` for `Skipped` outcomes and reports
/// them — the only outcome `execute_action` itself never sees, since a
/// skipped action is never actually dispatched (`run_level` decides it's
/// skipped before calling `execute`).
fn report_skipped(result: &super::scheduler::LevelResult, status: &StatusSink) {
    for (id, outcome) in &result.outcomes {
        if *outcome == RunOutcome::Skipped {
            status.report(id, RunStatus::Skipped, None);
        }
    }
}

/// Dispatches one action to either a leaf executor (§4) or a container
/// executor (§5 below) — the actual `execute` callback every `run_level`
/// call should be given, at any nesting depth (including the top level).
/// Returns a boxed future because container execution recurses back into
/// this very function through `run_level`, which an ordinary `async fn`
/// can't do (the future's size would be unbounded).
// `solution_root` gets its own lifetime `'r` (only required to outlive
// `'a`, not to equal it) rather than sharing `'a` with `action_id`/
// `action`/`ctx`. Those three are always tied to whatever `run_level`
// call is driving this (its own borrow of an `ActionMap`/`RunContext`),
// which needs to stay higher-ranked (`for<'a> Fn(&'a str, ...) -> ...`)
// for the closure passed into `run_level` to type-check; `solution_root`
// is captured from further out (the top-level call) and outlives every
// such 'a trivially, but forcing it into the *same* named lifetime as the
// other three breaks that higher-ranked inference.
pub fn execute_action<'a, 'r: 'a>(
    action_id: &'a str,
    action: &'a Action,
    ctx: &'a RunContext,
    solution_root: &'r Path,
    status: &'r StatusSink,
) -> BoxFuture<'a, Result<Map<String, Value>, String>> {
    async move {
        status.report(action_id, RunStatus::Running, None);
        let outcome = if registry::is_container(&action.type_id) {
            execute_container(action_id, action, ctx, solution_root, status).await
        } else {
            execute_leaf(action_id, action, ctx, solution_root).await
        };
        let (phase, detail) = match &outcome {
            Ok(outputs) => (
                RunStatus::Succeeded,
                (!outputs.is_empty()).then(|| serde_json::to_string(outputs).unwrap_or_default()),
            ),
            Err(message) => (RunStatus::Failed, Some(message.clone())),
        };
        status.report(action_id, phase, detail);
        outcome
    }
    .boxed()
}

async fn execute_container(
    action_id: &str,
    action: &Action,
    ctx: &RunContext,
    solution_root: &Path,
    status: &StatusSink,
) -> Result<Map<String, Value>, String> {
    match action.type_id.as_str() {
        "If" => execute_if(action_id, action, ctx, solution_root, status).await,
        "Foreach" => execute_foreach(action_id, action, ctx, solution_root, status).await,
        "Until" => execute_until(action_id, action, ctx, solution_root, status).await,
        "Try" => execute_try(action_id, action, ctx, solution_root, status).await,
        other => Err(format!("container type \"{other}\" isn't implemented yet")),
    }
}

/// §5/§3a: run `actions` (the try body) via the scheduler, scoped to just
/// that body, on the *same* `ctx` as the caller (no fork — matches
/// `If`/`Until`, see §2's note). If every leaf resolves `Succeeded` (or
/// `Skipped` via its own internal `runAfter` gating — not a failure), the
/// `Try` itself is `Succeeded` and `catch` never runs. If any leaf
/// resolves `Failed`, populate `<tryId>.outputs.error` as
/// `{actionId, message}` for the first one encountered (in the body's own
/// authoring order, for determinism — `ActionMap` is an `IndexMap`) and
/// run `catch` if present; `catch` succeeding makes the overall `Try`
/// `Succeeded` (handled), `catch` failing or being absent makes it
/// `Failed`.
async fn execute_try(
    action_id: &str,
    action: &Action,
    ctx: &RunContext,
    solution_root: &Path,
    status: &StatusSink,
) -> Result<Map<String, Value>, String> {
    let body = action
        .actions
        .as_ref()
        .ok_or_else(|| format!("{action_id}: Try action is missing \"actions\""))?;

    let result = run_level(body, ctx, |id, a, c| {
        execute_action(id, a, c, solution_root, status)
    })
    .await;
    report_skipped(&result, status);
    if result.all_ok() {
        return Ok(Map::new());
    }

    let (failed_action_id, message) = body
        .keys()
        .find_map(|id| {
            result
                .errors
                .get(id)
                .map(|message| (id.clone(), message.clone()))
        })
        .unwrap_or_else(|| {
            (
                "unknown".to_string(),
                "one or more actions failed".to_string(),
            )
        });

    let mut error = Map::new();
    error.insert("actionId".to_string(), Value::String(failed_action_id));
    error.insert("message".to_string(), Value::String(message));
    let mut error_outputs = Map::new();
    error_outputs.insert("error".to_string(), Value::Object(error));
    // Needed *now*, before `catch` runs, so `catch`'s own body can
    // reference `{"var": "<tryId>.outputs.error..."}` (exactly what
    // `.forge/order-processing.flow.json`'s `logChargeFailure` does). The
    // caller's own `run_level` will overwrite `ctx`'s entry for this
    // action with whatever this function finally returns, so the success
    // path below must re-include it, or it would be silently erased.
    ctx.record_outputs(action_id, error_outputs.clone());

    let Some(catch) = action.catch.as_ref() else {
        return Err(format!(
            "{action_id}: try body failed and no catch is present"
        ));
    };

    let catch_result = run_level(&catch.actions, ctx, |id, a, c| {
        execute_action(id, a, c, solution_root, status)
    })
    .await;
    report_skipped(&catch_result, status);
    if catch_result.all_ok() {
        Ok(error_outputs)
    } else {
        let first_error = catch_result
            .errors
            .values()
            .next()
            .cloned()
            .unwrap_or_else(|| "catch handler failed".to_string());
        Err(format!("{action_id}: catch handler failed — {first_error}"))
    }
}

/// §5, the one piece of container semantics the design doc was explicit
/// and emphatic about ("a real operational hazard if an infinite loop is
/// possible"): loop the body until `until` evaluates truthy, **or**
/// `limit.count` iterations have run, **or** `limit.timeout` has elapsed —
/// whichever comes first, both enforced as hard stops. A do-while shape
/// (run the body, then check `until`), not check-then-run — matches
/// ordinary retry-loop semantics ("try, then see if we're done").
///
/// Runs the body against `ctx` directly, **not** `ctx.child()` — unlike
/// `Foreach`, an `Until` body typically reuses the *same* action ids every
/// iteration on purpose (there's no per-iteration loop variable the way
/// `Foreach` has `$item` to key outputs by), and last-write-wins on a
/// shared context is exactly the desired behavior: the `until` condition
/// and whatever runs after `Until` succeeds should see the *latest*
/// iteration's outputs, not be unable to see the loop at all.
async fn execute_until(
    action_id: &str,
    action: &Action,
    ctx: &RunContext,
    solution_root: &Path,
    status: &StatusSink,
) -> Result<Map<String, Value>, String> {
    let expr = action
        .until
        .as_ref()
        .ok_or_else(|| format!("{action_id}: Until action is missing \"until\""))?;
    let limit = action
        .limit
        .as_ref()
        .ok_or_else(|| format!("{action_id}: Until action is missing \"limit\""))?;
    let body = action
        .actions
        .as_ref()
        .ok_or_else(|| format!("{action_id}: Until action is missing \"actions\""))?;

    let timeout = limit
        .timeout
        .as_deref()
        .map(parse_iso8601_duration)
        .transpose()?;
    let deadline = timeout.map(|d| tokio::time::Instant::now() + d);

    for iteration in 0..limit.count {
        if let Some(deadline) = deadline
            && tokio::time::Instant::now() >= deadline
        {
            return Err(format!(
                "{action_id}: timed out after {iteration} iteration(s) without \"until\" becoming truthy"
            ));
        }

        let result = run_level(body, ctx, |id, a, c| {
            execute_action(id, a, c, solution_root, status)
        })
        .await;
        report_skipped(&result, status);
        if !result.all_ok() {
            let first_error = result
                .errors
                .values()
                .next()
                .cloned()
                .unwrap_or_else(|| "one or more actions failed".to_string());
            return Err(format!(
                "{action_id}: iteration {iteration} failed — {first_error}"
            ));
        }

        if is_truthy(&ctx.evaluate(expr)?) {
            return Ok(Map::new());
        }
    }

    Err(format!(
        "{action_id}: exceeded limit.count ({}) without \"until\" becoming truthy",
        limit.count
    ))
}

/// Parses the `PnDTnHnMnS` subset of ISO 8601 durations relevant to a
/// loop timeout. Calendar `Y` (year) and date-part `M` (month) are
/// deliberately rejected — their real length varies (leap years, 28–31
/// day months), meaningless for a fixed timeout — while `D` (day), `H`
/// (hour), time-part `M` (minute), and `S` (second, fractional allowed)
/// are all fixed-length and supported.
fn parse_iso8601_duration(s: &str) -> Result<std::time::Duration, String> {
    let rest = s
        .trim()
        .strip_prefix('P')
        .ok_or_else(|| format!("\"{s}\": ISO 8601 durations must start with \"P\""))?;
    let (date_part, time_part) = match rest.split_once('T') {
        Some((d, t)) => (d, Some(t)),
        None => (rest, None),
    };

    let mut seconds = 0.0;
    let mut accumulate = |part: &str, in_time: bool| -> Result<(), String> {
        let mut num = String::new();
        for c in part.chars() {
            if c.is_ascii_digit() || c == '.' {
                num.push(c);
                continue;
            }
            let n: f64 = num
                .parse()
                .map_err(|_| format!("\"{s}\": invalid number before \"{c}\""))?;
            num.clear();
            seconds += match (in_time, c) {
                (false, 'D') => n * 86_400.0,
                (false, 'Y' | 'M') => {
                    return Err(format!(
                        "\"{s}\": calendar Y/M components aren't supported for a timeout — use D/H/M/S"
                    ));
                }
                (true, 'H') => n * 3_600.0,
                (true, 'M') => n * 60.0,
                (true, 'S') => n,
                (_, other) => return Err(format!("\"{s}\": unexpected \"{other}\"")),
            };
        }
        if !num.is_empty() {
            return Err(format!("\"{s}\": trailing number with no unit"));
        }
        Ok(())
    };
    accumulate(date_part, false)?;
    if let Some(time_part) = time_part {
        accumulate(time_part, true)?;
    }

    if seconds < 0.0 {
        return Err(format!("\"{s}\": duration can't be negative"));
    }
    Ok(std::time::Duration::from_secs_f64(seconds))
}

/// §5: "evaluate `foreach` (JSONLogic) once against the current
/// `RunContext` to get a JSON array." **Locked (revised 2026-09-12):
/// iterations run concurrently**, not sequentially — the original v1
/// design doc chose sequential as "the simpler, safer default for a first
/// implementation," explicitly flagging concurrent fan-out as a future
/// enhancement once the context-scoping question (§2) was settled; §2 is
/// now settled (each iteration already gets its own `ctx.child()` layer,
/// which was always going to be a prerequisite for concurrency, not just
/// sequential correctness) and Try/Catch (the mechanism a per-item failure
/// would realistically be handled through, as `.forge/order-processing.flow.json`'s
/// `tryCharge` demonstrates) is implemented and proven, so there's no
/// remaining reason to serialize independent iterations. Unbounded — no
/// artificial concurrency cap, matching `scheduler::run_level`'s own
/// fan-out of a level's independent actions, which has never been capped
/// either. All iterations run to completion regardless of any single
/// iteration's failure (no cancellation-on-first-failure — futures already
/// in flight aren't aborted, matching `run_level`'s own "run everything
/// eligible, then report" shape rather than a sequential fail-fast); if
/// any iteration didn't resolve cleanly, the *first one encountered while
/// draining results* (not necessarily the first to have started — indices
/// are recorded specifically so the error message is still traceable to a
/// real item) is reported as this action's own failure.
async fn execute_foreach(
    action_id: &str,
    action: &Action,
    ctx: &RunContext,
    solution_root: &Path,
    status: &StatusSink,
) -> Result<Map<String, Value>, String> {
    let expr = action
        .foreach
        .as_ref()
        .ok_or_else(|| format!("{action_id}: Foreach action is missing \"foreach\""))?;
    let items = ctx.evaluate(expr)?;
    let items = items
        .as_array()
        .cloned()
        .ok_or_else(|| format!("{action_id}: \"foreach\" must evaluate to an array"))?;

    let Some(body) = action.actions.as_ref() else {
        return Ok(Map::new());
    };

    let total = items.len();
    let mut running = FuturesUnordered::new();
    for (index, item) in items.into_iter().enumerate() {
        // Every iteration reports under the same action ids, so each gets a
        // sink that says which one it is.
        let iteration_status = status.in_iteration(IterationScope {
            container_id: action_id.to_string(),
            index,
            total,
            item: item.clone(),
        });
        let iteration_ctx = ctx.child().with_item(item);
        running.push(async move {
            let result = run_level(body, &iteration_ctx, |id, a, c| {
                execute_action(id, a, c, solution_root, &iteration_status)
            })
            .await;
            report_skipped(&result, &iteration_status);
            (index, result)
        });
    }

    let mut first_error: Option<String> = None;
    while let Some((index, result)) = running.next().await {
        if !result.all_ok() && first_error.is_none() {
            let message = result
                .errors
                .values()
                .next()
                .cloned()
                .unwrap_or_else(|| "one or more actions failed".to_string());
            first_error = Some(format!("{action_id}: iteration {index} failed — {message}"));
        }
    }

    match first_error {
        Some(err) => Err(err),
        None => Ok(Map::new()),
    }
}

/// §5: "evaluate `expression` once; run `actions` (true branch) if
/// truthy, else `else.actions` if present, else no-op (the `If` action
/// itself just resolves `Succeeded` with no work done)." The chosen
/// branch runs through the *same* `RunContext` as the caller (§2: `If`
/// doesn't fork a child context — its body runs at most once per
/// execution, no iteration-identity problem the way `Foreach` has).
async fn execute_if(
    action_id: &str,
    action: &Action,
    ctx: &RunContext,
    solution_root: &Path,
    status: &StatusSink,
) -> Result<Map<String, Value>, String> {
    let expr = action
        .expression
        .as_ref()
        .ok_or_else(|| format!("{action_id}: If action is missing \"expression\""))?;
    let condition = ctx.evaluate(expr)?;

    let body = if is_truthy(&condition) {
        action.actions.as_ref()
    } else {
        action.else_branch.as_ref().map(|branch| &branch.actions)
    };

    let Some(body) = body else {
        return Ok(Map::new());
    };

    let result = run_level(body, ctx, |id, a, c| {
        execute_action(id, a, c, solution_root, status)
    })
    .await;
    report_skipped(&result, status);
    if result.all_ok() {
        Ok(Map::new())
    } else {
        let first_error = result
            .errors
            .values()
            .next()
            .cloned()
            .unwrap_or_else(|| "one or more actions failed".to_string());
        Err(format!("{action_id}: branch failed — {first_error}"))
    }
}

/// JSONLogic truthiness (<https://jsonlogic.com/truthy.html>): falsy is
/// `false`/`0`/`""`/`null`/`[]`, everything else (including `{}`,
/// non-empty strings/arrays, and any nonzero number) is truthy — not
/// Rust's own `bool`, since `expression` can legally resolve to any JSON
/// type.
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(true),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::schema::{ActionMap, Branch, Limit};

    fn root() -> &'static Path {
        Path::new(".")
    }

    #[test]
    fn truthiness_matches_the_jsonlogic_spec() {
        assert!(!is_truthy(&Value::Null));
        assert!(!is_truthy(&Value::Bool(false)));
        assert!(!is_truthy(&Value::from(0)));
        assert!(!is_truthy(&Value::from("")));
        assert!(!is_truthy(&Value::Array(vec![])));
        assert!(is_truthy(&Value::Bool(true)));
        assert!(is_truthy(&Value::from(1)));
        assert!(is_truthy(&Value::from(-1)));
        assert!(is_truthy(&Value::from("x")));
        assert!(is_truthy(&serde_json::json!([1])));
        assert!(is_truthy(&serde_json::json!({})));
    }

    fn log_action(id: &str, message: &str) -> (String, Action) {
        let mut action = Action::new("log");
        action.inputs.insert("level".into(), Value::from("info"));
        action.inputs.insert("message".into(), Value::from(message));
        (id.to_string(), action)
    }

    #[tokio::test]
    async fn runs_the_true_branch_and_records_its_outputs() {
        let mut if_action = Action::new("If");
        if_action.expression = Some(serde_json::json!(true));
        let mut then_actions = ActionMap::new();
        let (id, action) = log_action("t1", "hi");
        then_actions.insert(id, action);
        if_action.actions = Some(then_actions);

        let ctx = RunContext::new();
        let outputs = execute_action("myIf", &if_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        assert!(outputs.is_empty());
        // "t1" actually ran (its outputs got recorded in the shared context
        // — no fork, per §2).
        let seen = ctx
            .evaluate(&serde_json::json!({"var": "t1.outputs"}))
            .unwrap();
        assert!(seen.is_object());
    }

    #[tokio::test]
    async fn runs_the_else_branch_when_falsy() {
        let mut if_action = Action::new("If");
        if_action.expression = Some(serde_json::json!(false));
        let mut then_actions = ActionMap::new();
        let (id, action) = log_action("thenOnly", "should not run");
        then_actions.insert(id, action);
        if_action.actions = Some(then_actions);

        let mut else_actions = ActionMap::new();
        let (id, action) = log_action("elseOnly", "should run");
        else_actions.insert(id, action);
        if_action.else_branch = Some(Branch {
            actions: else_actions,
        });

        let ctx = RunContext::new();
        execute_action("myIf", &if_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();

        let then_ran = ctx
            .evaluate(&serde_json::json!({"var": "thenOnly.outputs"}))
            .unwrap();
        assert!(then_ran.is_null());
        let else_ran = ctx
            .evaluate(&serde_json::json!({"var": "elseOnly.outputs"}))
            .unwrap();
        assert!(else_ran.is_object());
    }

    #[tokio::test]
    async fn falsy_with_no_else_branch_is_a_no_op_success() {
        let mut if_action = Action::new("If");
        if_action.expression = Some(serde_json::json!(false));
        let mut then_actions = ActionMap::new();
        let (id, action) = log_action("thenOnly", "should not run");
        then_actions.insert(id, action);
        if_action.actions = Some(then_actions);
        // No else_branch set.

        let ctx = RunContext::new();
        let outputs = execute_action("myIf", &if_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        assert!(outputs.is_empty());
    }

    #[tokio::test]
    async fn expression_reads_outputs_from_ancestor_actions() {
        let ctx = RunContext::new();
        ctx.record_outputs(
            "fetchCustomer",
            serde_json::json!({"tier": "vip"})
                .as_object()
                .unwrap()
                .clone(),
        );

        let mut if_action = Action::new("If");
        if_action.expression =
            Some(serde_json::json!({"==": [{"var": "fetchCustomer.outputs.tier"}, "vip"]}));
        let mut then_actions = ActionMap::new();
        let (id, action) = log_action("notifyVip", "vip!");
        then_actions.insert(id, action);
        if_action.actions = Some(then_actions);

        execute_action("ifVip", &if_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        let ran = ctx
            .evaluate(&serde_json::json!({"var": "notifyVip.outputs"}))
            .unwrap();
        assert!(ran.is_object());
    }

    #[tokio::test]
    async fn a_failed_action_inside_the_branch_fails_the_if_itself() {
        let mut if_action = Action::new("If");
        if_action.expression = Some(serde_json::json!(true));
        let mut then_actions = ActionMap::new();
        then_actions.insert("bad".to_string(), Action::new("TotallyMadeUp"));
        if_action.actions = Some(then_actions);

        let ctx = RunContext::new();
        let err = execute_action("myIf", &if_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("myIf"));
    }

    fn transform_action(id: &str, expr: Value) -> (String, Action) {
        let mut action = Action::new("transform");
        action.inputs.insert("expression".into(), expr);
        (id.to_string(), action)
    }

    #[tokio::test]
    async fn foreach_binds_item_per_iteration() {
        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!([10, 20, 30]));
        let mut body = ActionMap::new();
        let (id, action) =
            transform_action("double", serde_json::json!({"*": [{"var": "$item"}, 2]}));
        body.insert(id, action);
        foreach.actions = Some(body);

        let ctx = RunContext::new();
        let outputs = execute_action("loop", &foreach, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        assert!(outputs.is_empty());
        // Each iteration's child context is discarded after it finishes —
        // the outer context never sees "double"'s outputs at all (§2).
        let leaked = ctx
            .evaluate(&serde_json::json!({"var": "double.outputs.result"}))
            .unwrap();
        assert!(leaked.is_null());
    }

    /// The body runs under the same action id every time, so each report
    /// has to say which iteration it came from and what its item was.
    #[tokio::test]
    async fn foreach_reports_which_iteration_a_body_action_ran_in() {
        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!([10, 20, 30]));
        let mut body = ActionMap::new();
        let (id, action) =
            transform_action("double", serde_json::json!({"*": [{"var": "$item"}, 2]}));
        body.insert(id, action);
        foreach.actions = Some(body);

        let seen = Arc::new(Mutex::new(Vec::new()));
        let sink_seen = seen.clone();
        let status = StatusSink::with_scope(move |scope, id, status, detail| {
            if status == RunStatus::Succeeded {
                sink_seen
                    .lock()
                    .unwrap()
                    .push((scope.to_vec(), id.to_string(), detail));
            }
        });
        execute_action("loop", &foreach, &RunContext::new(), root(), &status)
            .await
            .unwrap();

        let mut seen = seen.lock().unwrap().clone();
        seen.sort_by_key(|(scope, _, _)| scope.first().map(|s| s.index));
        // The loop itself reports outside any iteration; its body inside one.
        let (scope, id, _) = &seen[0];
        assert!(scope.is_empty());
        assert_eq!(id, "loop");
        for (position, item) in [10, 20, 30].into_iter().enumerate() {
            let (scope, id, detail) = &seen[position + 1];
            assert_eq!(id, "double");
            assert_eq!(
                scope.as_slice(),
                [IterationScope {
                    container_id: "loop".to_string(),
                    index: position,
                    total: 3,
                    item: serde_json::json!(item),
                }]
            );
            let outputs: Value = serde_json::from_str(detail.as_deref().unwrap()).unwrap();
            assert_eq!(outputs["result"], serde_json::json!(item * 2));
        }
    }

    #[tokio::test]
    async fn foreach_runs_cleanly_with_the_same_action_id_every_iteration() {
        // A same-id action every iteration is exactly the case §2 exists
        // for (re-running the body per iteration must not let iteration 2
        // silently clash with iteration 1's leftover output under the
        // same id in one shared map). The per-iteration isolation itself
        // is already proven directly against `RunContext` in
        // `engine::tests::child_sees_parent_data_but_writes_stay_local`;
        // this just confirms the container executor doesn't error or
        // behave oddly running the same body repeatedly.
        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!([1, 2, 3]));
        let mut body = ActionMap::new();
        let (id, action) = transform_action("accumulate", serde_json::json!({"var": "$item"}));
        body.insert(id, action);
        foreach.actions = Some(body);

        let ctx = RunContext::new();
        let outputs = execute_action("loop", &foreach, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        assert!(outputs.is_empty());
    }

    #[tokio::test]
    async fn foreach_over_empty_array_is_a_no_op_success() {
        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!([]));
        let mut body = ActionMap::new();
        body.insert("shouldNotRun".to_string(), Action::new("TotallyMadeUp"));
        foreach.actions = Some(body);

        let ctx = RunContext::new();
        let outputs = execute_action("loop", &foreach, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        assert!(outputs.is_empty());
    }

    #[tokio::test]
    async fn foreach_reports_a_failure_from_any_iteration() {
        // Iterations run concurrently (§5, revised 2026-09-12) — every
        // iteration still runs to completion even though `bad` fails in
        // all of them, so this only asserts *an* iteration was reported
        // failing, not specifically "iteration 0" (`FuturesUnordered`
        // doesn't guarantee completion order for already-ready futures).
        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!([1, 2, 3]));
        let mut body = ActionMap::new();
        body.insert("bad".to_string(), Action::new("TotallyMadeUp"));
        foreach.actions = Some(body);

        let ctx = RunContext::new();
        let err = execute_action("loop", &foreach, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(
            err.contains("iteration"),
            "expected an iteration-failure message, got: {err}"
        );
        assert!(
            err.contains("unknown action type"),
            "expected the underlying error, got: {err}"
        );
    }

    #[tokio::test]
    async fn foreach_iterations_actually_run_concurrently() {
        // Each iteration's body sleeps via `delay` — if iterations ran
        // sequentially (the pre-2026-09-12 behavior), 5 items * 400ms
        // would take >=2000ms. `delay`/`http`/`StartProcess`/etc. all
        // share one deliberately single-worker `on_tokio` runtime
        // (`reqwest_client::runtime()`, kept small on purpose) — when the
        // *whole suite* runs with cargo test's default parallelism, many
        // other tests hammer that same shared runtime at once, and this
        // test can see real queueing delay from that (confirmed: reliably
        // fast alone or with `--test-threads=1`, occasionally slow only
        // under full-suite contention). That's suite-level noise, not
        // evidence iterations stopped running concurrently — a large gap
        // between the two timings, and a generous threshold, means this
        // still catches a real regression (which would blow *far* past
        // 2000ms, not linger near the boundary) without chasing
        // contention-driven flakiness that has nothing to do with what
        // this test verifies.
        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!([1, 2, 3, 4, 5]));
        let mut body = ActionMap::new();
        let mut delay_action = Action::new("delay");
        delay_action.inputs.insert("ms".into(), Value::from(400));
        body.insert("wait".to_string(), delay_action);
        foreach.actions = Some(body);

        let ctx = RunContext::new();
        let started = std::time::Instant::now();
        execute_action("loop", &foreach, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        let elapsed = started.elapsed();
        assert!(
            elapsed < std::time::Duration::from_millis(1500),
            "took {elapsed:?} — iterations ran sequentially, not concurrently"
        );
    }

    #[tokio::test]
    async fn foreach_rejects_a_non_array_expression() {
        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!("not an array"));
        foreach.actions = Some(ActionMap::new());

        let ctx = RunContext::new();
        let err = execute_action("loop", &foreach, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("array"));
    }

    #[tokio::test]
    async fn foreach_can_read_outputs_from_outside_the_loop() {
        let ctx = RunContext::new();
        ctx.record_outputs(
            "fetchOrder",
            serde_json::json!({"multiplier": 3})
                .as_object()
                .unwrap()
                .clone(),
        );

        let mut foreach = Action::new("Foreach");
        foreach.foreach = Some(serde_json::json!([1, 2]));
        let mut body = ActionMap::new();
        let (id, action) = transform_action(
            "scaled",
            serde_json::json!({"*": [{"var": "$item"}, {"var": "fetchOrder.outputs.multiplier"}]}),
        );
        body.insert(id, action);
        foreach.actions = Some(body);

        execute_action("loop", &foreach, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
    }

    fn limit(count: u32, timeout: Option<&str>) -> Limit {
        Limit {
            count,
            timeout: timeout.map(str::to_string),
        }
    }

    #[test]
    fn iso8601_duration_parsing() {
        assert_eq!(
            parse_iso8601_duration("PT30S").unwrap(),
            std::time::Duration::from_secs(30)
        );
        assert_eq!(
            parse_iso8601_duration("PT1M").unwrap(),
            std::time::Duration::from_secs(60)
        );
        assert_eq!(
            parse_iso8601_duration("PT1H30M").unwrap(),
            std::time::Duration::from_secs(5_400)
        );
        assert_eq!(
            parse_iso8601_duration("P1D").unwrap(),
            std::time::Duration::from_secs(86_400)
        );
        assert_eq!(
            parse_iso8601_duration("PT0.5S").unwrap(),
            std::time::Duration::from_secs_f64(0.5)
        );

        assert!(parse_iso8601_duration("30S").is_err(), "must start with P");
        assert!(
            parse_iso8601_duration("P1Y").is_err(),
            "calendar Y unsupported"
        );
        assert!(
            parse_iso8601_duration("P1M").is_err(),
            "calendar (date-part) M unsupported"
        );
        assert!(parse_iso8601_duration("PTXS").is_err(), "garbage number");
    }

    #[tokio::test]
    async fn until_stops_as_soon_as_the_condition_is_truthy() {
        let mut until = Action::new("Until");
        until.until = Some(serde_json::json!(true)); // truthy immediately
        until.limit = Some(limit(5, None));
        let mut body = ActionMap::new();
        let (id, action) = log_action("mark", "ran once");
        body.insert(id, action);
        until.actions = Some(body);

        let ctx = RunContext::new();
        let outputs = execute_action("poll", &until, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        assert!(outputs.is_empty());
        // Do-while: the body ran exactly once before the condition was
        // even checked.
        let ran = ctx
            .evaluate(&serde_json::json!({"var": "mark.outputs"}))
            .unwrap();
        assert!(ran.is_object());
    }

    #[tokio::test]
    async fn until_stops_at_limit_count_when_never_truthy() {
        let mut until = Action::new("Until");
        until.until = Some(serde_json::json!(false)); // never satisfied
        until.limit = Some(limit(3, None));
        let mut body = ActionMap::new();
        let (id, action) = log_action("mark", "ran");
        body.insert(id, action);
        until.actions = Some(body);

        let ctx = RunContext::new();
        let err = execute_action("poll", &until, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("limit.count"));
        // The body still ran at least once before giving up.
        let ran = ctx
            .evaluate(&serde_json::json!({"var": "mark.outputs"}))
            .unwrap();
        assert!(ran.is_object());
    }

    #[tokio::test]
    async fn until_stops_at_timeout_well_before_limit_count() {
        let mut until = Action::new("Until");
        until.until = Some(serde_json::json!(false)); // never satisfied
        until.limit = Some(limit(10_000, Some("PT0.05S"))); // 50ms
        let mut delay_action = Action::new("delay");
        delay_action.inputs.insert("ms".into(), Value::from(20));
        let mut body = ActionMap::new();
        body.insert("wait".to_string(), delay_action);
        until.actions = Some(body);

        let ctx = RunContext::new();
        let started = std::time::Instant::now();
        let err = execute_action("poll", &until, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        let elapsed = started.elapsed();

        assert!(err.contains("timed out"));
        // Would take 10_000 * 20ms (200s) if the timeout weren't enforced
        // — 5s is still a huge margin below that while tolerating real
        // scheduling contention when the whole suite runs in parallel
        // (see `foreach_iterations_actually_run_concurrently`'s comment on
        // why `on_tokio`'s single-worker runtime makes tight timing
        // thresholds flaky under full-suite load, not just this test).
        assert!(
            elapsed < std::time::Duration::from_secs(5),
            "took {elapsed:?} — timeout wasn't enforced"
        );
    }

    #[tokio::test]
    async fn until_fails_fast_when_the_body_fails() {
        let mut until = Action::new("Until");
        until.until = Some(serde_json::json!(false));
        until.limit = Some(limit(5, None));
        let mut body = ActionMap::new();
        body.insert("bad".to_string(), Action::new("TotallyMadeUp"));
        until.actions = Some(body);

        let ctx = RunContext::new();
        let err = execute_action("poll", &until, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("iteration 0"));
    }

    #[tokio::test]
    async fn until_requires_until_limit_and_actions() {
        let ctx = RunContext::new();

        let mut missing_until = Action::new("Until");
        missing_until.limit = Some(limit(1, None));
        missing_until.actions = Some(ActionMap::new());
        let err = execute_action("poll", &missing_until, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("until"));

        let mut missing_limit = Action::new("Until");
        missing_limit.until = Some(serde_json::json!(true));
        missing_limit.actions = Some(ActionMap::new());
        let err = execute_action("poll", &missing_limit, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("limit"));

        let mut missing_actions = Action::new("Until");
        missing_actions.until = Some(serde_json::json!(true));
        missing_actions.limit = Some(limit(1, None));
        let err = execute_action("poll", &missing_actions, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("actions"));
    }

    fn unknown_type_action(id: &str) -> (String, Action) {
        (id.to_string(), Action::new("TotallyMadeUp"))
    }

    #[tokio::test]
    async fn try_with_no_failures_never_runs_catch() {
        let mut try_action = Action::new("Try");
        let mut body = ActionMap::new();
        let (id, action) = log_action("ok", "fine");
        body.insert(id, action);
        try_action.actions = Some(body);
        let mut catch_body = ActionMap::new();
        let (id, action) = log_action("shouldNotRun", "catch ran");
        catch_body.insert(id, action);
        try_action.catch = Some(Branch {
            actions: catch_body,
        });

        let ctx = RunContext::new();
        let outputs = execute_action("tryCharge", &try_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();
        assert!(outputs.is_empty());
        let catch_ran = ctx
            .evaluate(&serde_json::json!({"var": "shouldNotRun.outputs"}))
            .unwrap();
        assert!(catch_ran.is_null());
    }

    #[tokio::test]
    async fn try_failure_is_recovered_by_a_succeeding_catch() {
        let mut try_action = Action::new("Try");
        let mut body = ActionMap::new();
        let (id, action) = unknown_type_action("chargeItem");
        body.insert(id, action);
        try_action.actions = Some(body);

        // Mirrors `.forge/order-processing.flow.json`'s `logChargeFailure`:
        // reads `<tryId>.outputs.error.message` from inside `catch`.
        let mut catch_body = ActionMap::new();
        let (id, action) = transform_action(
            "logChargeFailure",
            serde_json::json!({"var": "tryCharge.outputs.error.message"}),
        );
        catch_body.insert(id, action);
        try_action.catch = Some(Branch {
            actions: catch_body,
        });

        let ctx = RunContext::new();
        let outputs = execute_action("tryCharge", &try_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap();

        let error = outputs
            .get("error")
            .and_then(Value::as_object)
            .expect("error object");
        assert_eq!(error.get("actionId"), Some(&Value::from("chargeItem")));
        assert!(error.get("message").is_some());

        let logged = ctx
            .evaluate(&serde_json::json!({"var": "logChargeFailure.outputs.result"}))
            .unwrap();
        assert!(
            logged
                .as_str()
                .is_some_and(|s| s.contains("unknown action type"))
        );
    }

    #[tokio::test]
    async fn try_failure_with_no_catch_fails_the_try() {
        let mut try_action = Action::new("Try");
        let mut body = ActionMap::new();
        let (id, action) = unknown_type_action("chargeItem");
        body.insert(id, action);
        try_action.actions = Some(body);
        // No catch set.

        let ctx = RunContext::new();
        let err = execute_action("tryCharge", &try_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("no catch is present"));
        // The error is still recorded even though the Try itself failed —
        // a downstream `runAfter: {"tryCharge": ["Failed"]}` action could
        // still want to inspect it.
        let error = ctx
            .evaluate(&serde_json::json!({"var": "tryCharge.outputs.error.actionId"}))
            .unwrap();
        assert_eq!(error, Value::from("chargeItem"));
    }

    #[tokio::test]
    async fn try_failure_with_a_failing_catch_fails_the_try() {
        let mut try_action = Action::new("Try");
        let mut body = ActionMap::new();
        let (id, action) = unknown_type_action("chargeItem");
        body.insert(id, action);
        try_action.actions = Some(body);
        let mut catch_body = ActionMap::new();
        let (id, action) = unknown_type_action("alsoBad");
        catch_body.insert(id, action);
        try_action.catch = Some(Branch {
            actions: catch_body,
        });

        let ctx = RunContext::new();
        let err = execute_action("tryCharge", &try_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("catch handler failed"));
    }

    #[tokio::test]
    async fn try_picks_the_first_failure_in_authoring_order() {
        let mut try_action = Action::new("Try");
        let mut body = ActionMap::new();
        let (id, action) = unknown_type_action("firstBad");
        body.insert(id, action);
        let (id, action) = unknown_type_action("secondBad");
        body.insert(id, action);
        try_action.actions = Some(body);

        let ctx = RunContext::new();
        let err = execute_action("tryCharge", &try_action, &ctx, root(), &StatusSink::none())
            .await
            .unwrap_err();
        assert!(err.contains("no catch is present"));
        let error_id = ctx
            .evaluate(&serde_json::json!({"var": "tryCharge.outputs.error.actionId"}))
            .unwrap();
        assert_eq!(error_id, Value::from("firstBad"));
    }

    #[tokio::test]
    async fn status_reports_running_then_succeeded_for_a_leaf() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let sink_events = events.clone();
        let status = StatusSink::new(move |id, s, _detail| {
            sink_events.lock().unwrap().push((id.to_string(), s))
        });

        let (id, action) = log_action("mark", "hi");
        let ctx = RunContext::new();
        execute_action(&id, &action, &ctx, root(), &status)
            .await
            .unwrap();

        let recorded = events.lock().unwrap().clone();
        assert_eq!(
            recorded,
            vec![
                ("mark".to_string(), RunStatus::Running),
                ("mark".to_string(), RunStatus::Succeeded)
            ]
        );
    }

    #[tokio::test]
    async fn status_reports_failed_for_a_leaf_that_errors() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let sink_events = events.clone();
        let status = StatusSink::new(move |id, s, _detail| {
            sink_events.lock().unwrap().push((id.to_string(), s))
        });

        let action = Action::new("TotallyMadeUp");
        let ctx = RunContext::new();
        let _ = execute_action("bad", &action, &ctx, root(), &status).await;

        let recorded = events.lock().unwrap().clone();
        assert_eq!(
            recorded,
            vec![
                ("bad".to_string(), RunStatus::Running),
                ("bad".to_string(), RunStatus::Failed)
            ]
        );
    }

    #[tokio::test]
    async fn status_reports_for_every_nested_action_inside_a_container() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let sink_events = events.clone();
        let status = StatusSink::new(move |id, s, _detail| {
            sink_events.lock().unwrap().push((id.to_string(), s))
        });

        let mut if_action = Action::new("If");
        if_action.expression = Some(serde_json::json!(true));
        let mut body = ActionMap::new();
        let (id, action) = log_action("inner", "hi");
        body.insert(id, action);
        if_action.actions = Some(body);

        let ctx = RunContext::new();
        execute_action("myIf", &if_action, &ctx, root(), &status)
            .await
            .unwrap();

        let recorded = events.lock().unwrap().clone();
        // Both the container itself and its nested child reported their
        // own Running/Succeeded transitions.
        assert!(recorded.contains(&("myIf".to_string(), RunStatus::Running)));
        assert!(recorded.contains(&("myIf".to_string(), RunStatus::Succeeded)));
        assert!(recorded.contains(&("inner".to_string(), RunStatus::Running)));
        assert!(recorded.contains(&("inner".to_string(), RunStatus::Succeeded)));
        // The container's own transitions bracket its child's.
        let my_if_start = recorded
            .iter()
            .position(|e| e.0 == "myIf" && e.1 == RunStatus::Running)
            .unwrap();
        let inner_start = recorded
            .iter()
            .position(|e| e.0 == "inner" && e.1 == RunStatus::Running)
            .unwrap();
        let my_if_end = recorded
            .iter()
            .position(|e| e.0 == "myIf" && e.1 == RunStatus::Succeeded)
            .unwrap();
        assert!(my_if_start < inner_start && inner_start < my_if_end);
    }

    #[tokio::test]
    async fn status_reports_skipped_for_an_action_whose_dependency_outcome_doesnt_match() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let sink_events = events.clone();
        let status = StatusSink::new(move |id, s, _detail| {
            sink_events.lock().unwrap().push((id.to_string(), s))
        });

        let mut if_action = Action::new("If");
        if_action.expression = Some(serde_json::json!(true));
        let mut body = ActionMap::new();
        body.insert("a".to_string(), Action::new("log")); // missing required "message" -> fails
        let mut skip_me = Action::new("log");
        skip_me
            .run_after
            .insert("a".to_string(), vec![RunOutcome::Succeeded]);
        body.insert("b".to_string(), skip_me);
        if_action.actions = Some(body);

        let ctx = RunContext::new();
        let _ = execute_action("myIf", &if_action, &ctx, root(), &status).await;

        let recorded = events.lock().unwrap().clone();
        assert!(recorded.contains(&("b".to_string(), RunStatus::Skipped)));
        // A skipped action was never dispatched, so it never got a
        // Running event.
        assert!(!recorded.contains(&("b".to_string(), RunStatus::Running)));
    }
}
