# workflow_engine

The engine that validates and runs `.flow.json` workflows. Pure data and
tokio; it has no `gpui` dependency and no UI.

## Why it exists

Flows are this fork's way of describing multi-step automation — start a
service, wait for its port, call an API, run a script, branch on the result —
as a file you can commit. This crate is the part that gives those files
meaning: the schema, the list of action types, the scheduler that works out
what can run in parallel, and the executors that do the work.

Ported out of Forge's `backend/workflow`. Keeping it headless means it can be
driven by the Flows panel, the Designer, or a test, with identical behaviour.

| Crate | Role |
| --- | --- |
| `workflow_engine` (this) | Schema, registry, scheduler, executors |
| [`flows_panel`](../flows_panel/README.md) | Lists flows; create, open, run |
| [`designer_panel`](../designer_panel/README.md) | Visual canvas editor for one flow |

## The file format

A flow is a JSON object with an `id`, a `name` and a map of `actions`. Each
action has a `type`, its `inputs`, and a `runAfter` map naming the actions it
waits for and which outcomes allow it to proceed:

```json
{
  "id": "order-processing",
  "name": "Order Processing",
  "actions": {
    "waitForApi": {
      "type": "WaitForPort",
      "inputs": { "port": 4000, "timeout_ms": 30000 },
      "runAfter": {}
    },
    "fetchOrder": {
      "type": "http",
      "inputs": { "method": "GET", "url": "https://example.com/v1/order" },
      "runAfter": { "waitForApi": ["Succeeded"] }
    }
  }
}
```

Actions with an empty `runAfter` start immediately. Actions whose
dependencies are all satisfied run together as one level.

Complete examples: `DOCS/workflow-schema/example-flow.json` and
`fixtures/order-processing.flow.json`.

### Action types

| `type` | Label | Kind |
| --- | --- | --- |
| `http` | HTTP Request | Leaf |
| `transform` | Transform | Leaf |
| `InitVariable` | Initialize Variable | Leaf |
| `SetVariable` | Set Variable | Leaf |
| `script` | Script | Leaf |
| `StartProcess` | Start Process | Leaf |
| `WaitForPort` | Wait For Port | Leaf |
| `notify` | Notify | Leaf |
| `delay` | Delay | Leaf |
| `log` | Log | Leaf |
| `SetEnv` | Set Environment Variable | Leaf |
| `Foreach` | Foreach | Container |
| `Until` | Until | Container |
| `If` | If | Container |
| `Try` | Try/Catch | Container |

Containers hold their own nested `actions` map.

#### Variables

`InitVariable` creates a variable and `SetVariable` changes it. A variable
belongs to the whole run: any later expression reads it as
`{"var": "variables.<name>"}`, and a value set inside a loop body is still
set after the loop, which an action's own outputs are not.

| Action | Input | Notes |
| --- | --- | --- |
| `InitVariable` | `name` | No `.`, since it is read as a path segment |
| | `type` | `object`, `string`, `integer` or `array`; fixed from here on |
| | `value` | Optional starting value. Defaults to `{}`, `""`, `0` or `[]` |
| `SetVariable` | `name` | Must already be initialized |
| | `value` | The new value as written |
| | `valueExpression` | Or a JSONLogic expression for it. Use one, not both |

A value has to fit the variable's type. Scalars convert where that is
unambiguous (`5` for a string becomes `"5"`, `"5"` for an integer becomes
`5`); anything else fails the action. Both actions output `name` and `value`.

A `Foreach` runs its iterations concurrently, so iterations that all set the
same variable race: the variable ends up holding whichever wrote last.
`variables` can't be used as an action id.

#### `script`

A script gets its input as JSON on standard input, and returns its result by
printing a JSON object as its last line of output, which becomes the action's
outputs. `args` is piped as written; `argsExpression` is evaluated first, so
it is how a variable (`{"var": "variables.ids"}`), every variable
(`{"var": "variables"}`), the loop item (`{"var": "$item"}`) or an earlier
result reaches a script. Use one, not both. A script can't set a variable
itself: follow it with a `SetVariable` that reads its output.

Every script is also launched with the environment variable `FORGE_CONTEXT`
set to the path of a JSON file written for that one launch and deleted when
it exits:

```json
{ "variables": {}, "item": null, "args": null, "actions": { "fetch": { "status": 200 } } }
```

`variables` is every flow variable, `item` the enclosing `Foreach`'s current
item, `args` what is also on standard input, and `actions` each finished
action's outputs by id. It is a file per launch, not one shared file,
because a loop's iterations run at the same time. The Designer writes small
helpers that read it (see `designer_panel`'s README).

#### `http`

| Input | Notes |
| --- | --- |
| `method` | One of `GET`, `POST`, `PUT`, `PATCH`, `MERGE`, `DELETE`; case is ignored |
| `url` | A string, or a JSONLogic object that evaluates to one |
| `headers` | Optional object. A string value is sent as written; an object value is evaluated as JSONLogic |
| `body` | Optional, sent exactly as written. A string is sent as text, anything else as JSON |
| `bodyExpression` | Optional JSONLogic expression whose result is the body. Use it instead of `body`, not with it |

A response outside 2xx fails the action. To carry on after one, give the
next action a `["Failed"]` `runAfter` or wrap the request in a `Try`.
`Content-Type` defaults to `application/json` (`text/plain` for a string
body) unless `headers` sets it.

#### Future features

Three action types are commented out in `src/registry.rs` until the work
behind them exists. A flow that uses one fails validation as an unknown
action type.

| `type` | What it would do | What it needs first |
| --- | --- | --- |
| `sendEmail` | Send an email | A mail transport (SMTP or a provider API) and somewhere to keep its credentials. The old executor only logged "would send" |
| `DbMigration` | Run a `.sql` file against a Database panel connection | A way for this headless crate to reach a live connection, and a decision on transactions |
| `AiCheck` | Send a prompt to a language model and use the answer | A model backend reachable from this crate, and a defined result shape |

The `runnable: false` flag on `ActionDef` is still supported: an entry that
has it is listed in the catalog but disabled, and validation blocks a run
that contains it.

Expressions that refer to other actions' outputs are JSONLogic, evaluated
against the run's `RunContext`.

### Sidecar files

Presentation and bulky data are kept out of `<name>.flow.json` so that a git
diff of a flow shows behaviour changes only:

| Path | Holds |
| --- | --- |
| `<name>.flow.layout.json` | Canvas node positions (`layout` module) |
| `<name>.flow-http/<action>.body.json` | An `http` action's request body |
| `<name>.flow-http/<action>.body.schema.json` | Optional user schema for that body |

The `.flow-http` constants live in `schema.rs` because both the Designer
(which writes these files) and the JSON language-server wiring (which
validates them) must agree on the names.

## Using it

```rust
use workflow_engine::{WorkflowDefinition, run_workflow, validate_definition};

let def: WorkflowDefinition = serde_json::from_str(&text)?;

// Optional: collect problems to show the user before running.
let issues = validate_definition(&def);

// `status` is a workflow_engine::StatusSink: a wrapper around an optional
// Fn(&str, RunStatus, Option<String>) callback (action id, new status, detail).
// See `src/status.rs` for its constructors.
run_workflow(def, solution_root, status).await?;
```

`run_workflow` owns its arguments and returns a `Send + 'static` future, so
it can be handed to a tokio runtime. `solution_root` is only used by `script`
actions in file mode. The status callback is invoked from whichever thread is
driving the run — do not touch UI state from it directly (see
`designer_panel`'s `run_state` module for the pattern).

### API

| Area | Items |
| --- | --- |
| Schema | `WorkflowDefinition`, `Action`, `ActionMap`, `Branch`, `Limit`, `JsonLogicExpr`, `ScriptRuntime`, `ScriptSource`, `RunOutcome` |
| Registry | `ActionDef`, `ActionCategory`, `CatalogGroup`, `FieldKind`, `InputField`, `OutputField` |
| Running | `run_workflow`, `RunContext`, `execute_action`, `execute_leaf` |
| Scheduling | `compute_levels`, `validate`, `LevelResult` |
| Validation | `validate_definition`, `ValidationIssue`, `ValidationSeverity` |
| Status | `RunStatus`, `StatusSink` |
| History | `RunHistoryEntry`, `ActionHistoryRecord` |
| Layout | `WorkflowLayout` |
| Project scan | `scan_project`, `DetectedService`, `ServiceKind`, `ServiceSpec` |
| Processes | `ProcessInfo`, `ProcessStatus` |

`scan_project` detects runnable services in a workspace; `task_chain` turns a
list of `ServiceSpec`s into a flow. Together they back the Flows panel's
"Add Task Chain".

## Adding an action type

Add one `ActionDef` entry to `REGISTRY` in `src/registry.rs` and an executor
for it. That is the whole change.

`Action` is intentionally one flat struct with optional fields, not an enum
with a variant per type. The registry entry drives the canvas form, the
engine and expression completion at once, so a new type should never require
touching `schema::Action`, the load/save adapter or an exhaustive `match`. If
it seems to, the action is probably being modelled wrongly.

## Layout

| File | Contents |
| --- | --- |
| `src/schema.rs` | `flow.json` types and sidecar naming |
| `src/registry.rs` | Every action type's definition |
| `src/scheduler.rs` | Dependency levels and structural validation |
| `src/runner.rs` | `run_workflow` |
| `src/executors.rs` | Leaf executors (http, script, log, …) |
| `src/container.rs` | Container executors (Foreach, Until, If, Try) |
| `src/engine.rs` | `RunContext` and JSONLogic plumbing |
| `src/validation.rs` | User-facing validation issues |
| `src/status.rs` | Run status events |
| `src/history.rs` | Run history records |
| `src/layout.rs` | `.flow.layout.json` sidecar |
| `src/process_tracker.rs` | Tracking of started processes |
| `src/project_scan.rs`, `src/task_chain.rs` | Service detection and task-chain generation |

## Development

```sh
cargo check -p workflow_engine -j 8
cargo test -p workflow_engine -j 8
```

This crate is well covered by unit tests, most heavily the executors,
containers, validation and scheduler.

To run the tests of every fork crate at once: `script/test-fork-crates.ps1`.
