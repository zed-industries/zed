//! The action registry — `DOCS/forge-workflow-engine-design.md` §2's "one
//! central abstraction." One `ActionDef` per `type_id` drives the canvas
//! form, the execution engine, and JSONLogic completion simultaneously.
//! Adding an action type is exactly one new entry in [`REGISTRY`] — never a
//! change to [`super::schema::Action`], the load/save adapter, or any
//! exhaustive `match`. If a future action type ever seems to need a Rust
//! code change beyond an entry here, that's a sign it's being modeled
//! wrong, not a sign this registry needs restructuring.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionCategory {
    /// A single unit of work — the common case.
    Leaf,
    /// Has its own nested `actions` map (`Foreach`/`Until`/`If`/`Try`) —
    /// the canvas renders these as `gpui-flow` container nodes
    /// (`FlowNode::parent`/`container_size`), not leaf boxes.
    Container,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    String,
    Number,
    Bool,
    /// Arbitrary JSON — an escape hatch for inputs no simpler kind fits
    /// (e.g. `script`'s `source`, `http`'s `body`).
    Json,
    /// A JSONLogic expression specifically — the canvas should offer
    /// expression-completion (referencing other actions' `outputs`) for a
    /// field of this kind, not a plain text/JSON editor.
    Expression,
}

#[derive(Debug, Clone, Copy)]
pub struct InputField {
    pub name: &'static str,
    pub kind: FieldKind,
    pub required: bool,
    /// One-line hint shown in the inspector below the field label.
    /// Empty string means no hint is shown.
    pub help: &'static str,
    /// The closed set of values this field accepts, or empty for free input.
    /// The inspector renders a non-empty set as a dropdown and validation
    /// rejects anything outside it (compared case-insensitively).
    pub options: &'static [&'static str],
}

/// The methods an `http` action can send, in the order the inspector's
/// dropdown lists them.
pub const HTTP_METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "MERGE", "DELETE"];

/// The types an `InitVariable` action can declare, in the order the
/// inspector's dropdown lists them.
pub const VARIABLE_TYPES: &[&str] = crate::engine::VariableType::NAMES;

#[derive(Debug, Clone, Copy)]
pub struct OutputField {
    pub name: &'static str,
    pub kind: FieldKind,
}

/// Catalog grouping label — the strings used here are what the Phase 3
/// action catalog renders as section headers. Kept as a plain `&str`
/// rather than an enum so adding a new group is still a one-entry
/// registry change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogGroup {
    /// Execution flow: loops, branches, error handling.
    FlowControl,
    /// Data retrieval and transformation.
    Data,
    /// External service calls (HTTP, email, webhooks).
    Integrations,
    /// Shell commands, scripts, long-running processes.
    Processes,
    /// Diagnostics and environment setup.
    Utilities,
}

impl CatalogGroup {
    pub fn label(self) -> &'static str {
        match self {
            CatalogGroup::FlowControl => "Flow Control",
            CatalogGroup::Data => "Data",
            CatalogGroup::Integrations => "Integrations",
            CatalogGroup::Processes => "Processes",
            CatalogGroup::Utilities => "Utilities",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActionDef {
    pub type_id: &'static str,
    pub label: &'static str,
    pub category: ActionCategory,
    /// Which section of the action catalog this action appears under.
    pub catalog_group: CatalogGroup,
    /// `false` for action types whose executor is an acknowledged stub — the
    /// catalog shows them disabled with a tooltip, and validation reports
    /// their use as a blocking error. Nothing in [`REGISTRY`] is a stub right
    /// now: the entries that were (`DbMigration`, `AiCheck`) are parked as
    /// future features, see the note at the end of the table.
    pub runnable: bool,
    pub inputs: &'static [InputField],
    pub outputs: &'static [OutputField],
}

// ── const helpers ────────────────────────────────────────────────────────

const fn field(
    name: &'static str,
    kind: FieldKind,
    required: bool,
    help: &'static str,
) -> InputField {
    InputField {
        name,
        kind,
        required,
        help,
        options: &[],
    }
}

/// Restricts `field` to a closed set of values.
const fn one_of(field: InputField, options: &'static [&'static str]) -> InputField {
    InputField { options, ..field }
}

/// Shorthand for an optional field with no help text.
///
/// The inspector renders `InputField::help` as the guidance line under a
/// field, and `designer_panel` asserts every registry field has one, so
/// prefer `opt_h`. This exists only for a field with genuinely nothing to say.
#[allow(dead_code)]
const fn opt(name: &'static str, kind: FieldKind) -> InputField {
    field(name, kind, false, "")
}

/// Shorthand for a required field with a help hint.
const fn req_h(name: &'static str, kind: FieldKind, help: &'static str) -> InputField {
    field(name, kind, true, help)
}

/// Shorthand for an optional field with a help hint.
const fn opt_h(name: &'static str, kind: FieldKind, help: &'static str) -> InputField {
    field(name, kind, false, help)
}

const fn output(name: &'static str, kind: FieldKind) -> OutputField {
    OutputField { name, kind }
}

/// Every action type this engine currently knows about. `type_id` here is
/// exactly the string that appears as `Action.type_id` (`"type"` in JSON) —
/// the schema keeps that field an open string on purpose (see
/// `schema.rs`'s module doc), this table is what actually closes it at the
/// application layer.
pub static REGISTRY: &[ActionDef] = &[
    // ── Data ─────────────────────────────────────────────────────────────
    ActionDef {
        type_id: "http",
        label: "HTTP Request",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Integrations,
        runnable: true,
        inputs: &[
            one_of(
                req_h("method", FieldKind::String, "HTTP method to send"),
                HTTP_METHODS,
            ),
            req_h(
                "url",
                FieldKind::String,
                "Absolute URL including scheme; a JSONLogic object is evaluated",
            ),
            opt_h(
                "headers",
                FieldKind::Json,
                r#"JSON object of request headers, e.g. {"Authorization": "Bearer abc"}"#,
            ),
            opt_h(
                "body",
                FieldKind::Json,
                "Request body sent as written — omit for GET/DELETE",
            ),
            // Separate from `body` because the JSONLogic engine reads every
            // object as an operator call, so one field can't hold both a
            // literal object and an expression.
            opt_h(
                "bodyExpression",
                FieldKind::Expression,
                "JSONLogic expression whose result is sent as the body — use instead of body",
            ),
        ],
        outputs: &[
            output("body", FieldKind::Json),
            output("status", FieldKind::Number),
        ],
    },
    ActionDef {
        type_id: "transform",
        label: "Transform",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Data,
        runnable: true,
        // Was a free-text `strategy: String` with no defined behavior
        // anywhere — every worked example (`"normalize"`/`"discount"`/etc.)
        // was authored purely to exercise the canvas renderer, before
        // execution was ever in scope. Execution-engine design doc §4
        // (decided 2026-09-10): evaluate `expression` against the current
        // `RunContext`, store the result as `outputs.result` — fully
        // generic, no Forge code needed per new transform. Existing flows
        // authored against the old `strategy` field keep that key as an
        // unrecognized extra `inputs` entry (harmless, just not shown by
        // the typed form) until hand-migrated to a real `expression`.
        inputs: &[req_h(
            "expression",
            FieldKind::Expression,
            "JSONLogic expression — result stored as outputs.result",
        )],
        outputs: &[output("result", FieldKind::Json)],
    },
    // ── Variables ────────────────────────────────────────────────────────
    //
    // A variable belongs to the whole run, not to the action that created
    // it: it is read as `variables.<name>` from any later expression, and a
    // `SetVariable` inside a loop body is still in effect after the loop
    // (an action's own outputs are not). `InitVariable` fixes the type;
    // `SetVariable` then has to supply a value that fits it.
    ActionDef {
        type_id: "InitVariable",
        label: "Initialize Variable",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Data,
        runnable: true,
        inputs: &[
            req_h(
                "name",
                FieldKind::String,
                "Variable name — read later as variables.<name>",
            ),
            one_of(
                req_h("type", FieldKind::String, "What the variable holds"),
                VARIABLE_TYPES,
            ),
            opt_h(
                "value",
                FieldKind::Json,
                "Starting value — defaults to the type's empty value",
            ),
        ],
        outputs: &[
            output("name", FieldKind::String),
            output("value", FieldKind::Json),
        ],
    },
    ActionDef {
        type_id: "SetVariable",
        label: "Set Variable",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Data,
        runnable: true,
        inputs: &[
            req_h(
                "name",
                FieldKind::String,
                "Name of a variable an Initialize Variable action created",
            ),
            opt_h(
                "value",
                FieldKind::Json,
                "New value, as written — must fit the variable's type",
            ),
            // Separate from `value` for the same reason as `http`'s
            // `bodyExpression`: one field can't hold both a literal object
            // and an expression.
            opt_h(
                "valueExpression",
                FieldKind::Expression,
                "JSONLogic expression whose result is the new value — use instead of value",
            ),
        ],
        outputs: &[
            output("name", FieldKind::String),
            output("value", FieldKind::Json),
        ],
    },
    // ── Processes ────────────────────────────────────────────────────────
    ActionDef {
        type_id: "script",
        label: "Script",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Processes,
        runnable: true,
        // `source` is a `ScriptSource` (inline/file, §4) — represented as
        // one JSON-kind field here since the registry's `FieldKind` enum
        // doesn't need its own "discriminated union" case just for this;
        // the canvas's script editor already knows to special-case it.
        // `args`, if present, is serialized to JSON and piped to the
        // process's stdin (§4's data contract) — optional, since plenty of
        // scripts don't need any input at all.
        inputs: &[
            req_h(
                "source",
                FieldKind::Json,
                "Inline code or workspace-file path — use the Script editor above",
            ),
            opt_h(
                "args",
                FieldKind::Json,
                "JSON value piped to stdin; scripts read it from stdin as JSON",
            ),
            // Separate from `args` for the same reason as `http`'s
            // `bodyExpression`.
            opt_h(
                "argsExpression",
                FieldKind::Expression,
                "JSONLogic expression whose result is piped to stdin — use instead of args",
            ),
        ],
        outputs: &[output("stdout_json", FieldKind::Json)],
    },
    ActionDef {
        type_id: "StartProcess",
        label: "Start Process",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Processes,
        runnable: true,
        // Unlike every other leaf, this resolves `Succeeded` the moment
        // the process *starts*, not when it exits — for a dev server
        // (`npm run dev`, `uvicorn ...`) that never exits on its own.
        // `args` is a JSON array of strings, not a single string (a shell
        // command line has quoting ambiguity this avoids entirely — see
        // `execute_start_process`'s doc comment). Pair with `WaitForPort`
        // (`runAfter` this action's own id) to confirm it actually came
        // up. The spawned child is tracked in a dedicated in-process
        // registry (`process_tracker`) so it can be listed/stopped from
        // the Designer's own "Processes" panel — deliberately *not* routed
        // through `ScriptRunner`, which is a different, single-concurrency
        // lifecycle model built around commands that are expected to exit.
        inputs: &[
            req_h(
                "command",
                FieldKind::String,
                "Executable name or absolute path",
            ),
            opt_h(
                "args",
                FieldKind::Json,
                r#"JSON array of argument strings, e.g. ["run", "dev"]"#,
            ),
            opt_h(
                "cwd",
                FieldKind::String,
                "Working directory; defaults to project root",
            ),
            opt_h(
                "url",
                FieldKind::String,
                "Dev-server URL shown in the Processes panel, e.g. http://localhost:5173",
            ),
        ],
        outputs: &[
            output("pid", FieldKind::Number),
            output("processId", FieldKind::String),
        ],
    },
    ActionDef {
        type_id: "WaitForPort",
        label: "Wait For Port",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Processes,
        runnable: true,
        inputs: &[
            req_h("port", FieldKind::Number, "TCP port number to poll"),
            opt_h(
                "timeout_ms",
                FieldKind::Number,
                "Max wait in milliseconds; default 30 000",
            ),
        ],
        outputs: &[output("ready", FieldKind::Bool)],
    },
    // ── Integrations ─────────────────────────────────────────────────────
    ActionDef {
        type_id: "notify",
        label: "Notify",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Integrations,
        runnable: true,
        // Webhook-based (decided 2026-09-12): POSTs a Slack/Mattermost-
        // compatible JSON body (`{"channel": ..., "text": ...}` — that
        // shape is also readable by a generic webhook receiver, not just
        // those two specifically) to `url`. `channel` is optional since
        // the webhook URL itself often already implies the destination;
        // when a real webhook supports Slack's channel-override
        // convention, this sets it.
        inputs: &[
            req_h(
                "url",
                FieldKind::String,
                "Webhook URL (Slack / Mattermost / generic)",
            ),
            opt_h(
                "channel",
                FieldKind::String,
                "Channel override, e.g. #alerts",
            ),
            req_h(
                "message",
                FieldKind::String,
                "Notification text; supports JSONLogic interpolation",
            ),
        ],
        outputs: &[output("status", FieldKind::Number)],
    },
    // FUTURE FEATURE — `sendEmail` is parked, not registered.
    //
    // It was listed as runnable, but its executor only wrote a "would send"
    // line to the log: there is no SMTP or mail-API transport in this tree,
    // so a flow reported `Succeeded` for an email that was never delivered.
    // To bring it back: pick a transport (SMTP settings or a provider API)
    // and a place to keep its credentials, implement the send in
    // `executors::execute_send_email`, give it a real output (a message id
    // or delivery status), then restore this entry and the dispatch arm in
    // `executors::execute_leaf`. Until then a flow that uses the type fails
    // validation as an unknown action type; `notify` covers webhook alerts.
    //
    // ActionDef {
    //     type_id: "sendEmail",
    //     label: "Send Email",
    //     category: ActionCategory::Leaf,
    //     catalog_group: CatalogGroup::Integrations,
    //     runnable: true,
    //     inputs: &[
    //         req_h(
    //             "to",
    //             FieldKind::String,
    //             "Recipient address or comma-separated list",
    //         ),
    //         req_h(
    //             "subject",
    //             FieldKind::String,
    //             "Subject line; supports JSONLogic interpolation",
    //         ),
    //         req_h(
    //             "body",
    //             FieldKind::String,
    //             "Plain-text body; supports JSONLogic interpolation",
    //         ),
    //     ],
    //     outputs: &[],
    // },
    // ── Utilities ────────────────────────────────────────────────────────
    ActionDef {
        type_id: "delay",
        label: "Delay",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Utilities,
        runnable: true,
        inputs: &[req_h(
            "ms",
            FieldKind::Number,
            "Sleep duration in milliseconds",
        )],
        outputs: &[],
    },
    ActionDef {
        type_id: "log",
        label: "Log",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Utilities,
        runnable: true,
        inputs: &[
            req_h("level", FieldKind::String, "info, warn, or error"),
            req_h(
                "message",
                FieldKind::String,
                "Log text; supports JSONLogic interpolation",
            ),
        ],
        outputs: &[
            output("level", FieldKind::String),
            output("message", FieldKind::String),
        ],
    },
    ActionDef {
        type_id: "SetEnv",
        label: "Set Environment Variable",
        category: ActionCategory::Leaf,
        catalog_group: CatalogGroup::Utilities,
        runnable: true,
        inputs: &[
            req_h("key", FieldKind::String, "Variable name, e.g. DATABASE_URL"),
            req_h(
                "value",
                FieldKind::String,
                "Value to set; supports JSONLogic interpolation",
            ),
        ],
        outputs: &[],
    },
    // ── Future features (parked, not registered) ─────────────────────────
    //
    // FUTURE FEATURE — `DbMigration` and `AiCheck` never had an executor.
    // They were registered with `runnable: false` so the catalog could show
    // them disabled, which advertised two actions nobody can use. They are
    // commented out until the work behind them exists; restoring an entry
    // here (plus its executor and dispatch arm) is the whole change, and the
    // `runnable: false` plumbing in the catalog and validator is still in
    // place for an entry that needs to ship disabled first.
    //
    // `DbMigration` — run a `.sql` file against a connection from the
    // Database panel. Needs: a way for this headless crate to resolve
    // `connection_id` to a live `database_panel` connection (it has no gpui
    // or panel dependency today), a decision on transactions and on what a
    // partial failure leaves behind, and outputs such as rows affected.
    //
    // ActionDef {
    //     type_id: "DbMigration",
    //     label: "DB Migration",
    //     category: ActionCategory::Leaf,
    //     catalog_group: CatalogGroup::Utilities,
    //     runnable: false,
    //     inputs: &[
    //         req_h(
    //             "sql_file",
    //             FieldKind::String,
    //             "Path to the .sql migration file",
    //         ),
    //         req_h(
    //             "connection_id",
    //             FieldKind::String,
    //             "Database connection defined in the DB panel",
    //         ),
    //     ],
    //     outputs: &[],
    // },
    //
    // `AiCheck` — send a prompt to a language model and use the answer as a
    // gate or as data for later actions. Needs: a model backend reachable
    // from this crate (Forge's AI Twin backend was never ported), a choice of
    // which configured provider and model a flow uses, and a defined result
    // shape (free text, or a pass/fail verdict a `runAfter` can branch on).
    //
    // ActionDef {
    //     type_id: "AiCheck",
    //     label: "AI Check",
    //     category: ActionCategory::Leaf,
    //     catalog_group: CatalogGroup::Utilities,
    //     runnable: false,
    //     inputs: &[req_h(
    //         "prompt",
    //         FieldKind::String,
    //         "Natural-language prompt sent to the AI model",
    //     )],
    //     outputs: &[output("result", FieldKind::String)],
    // },
    // ── Container constructs (§3/§3a) ────────────────────────────────────
    ActionDef {
        type_id: "Foreach",
        label: "Foreach",
        category: ActionCategory::Container,
        catalog_group: CatalogGroup::FlowControl,
        runnable: true,
        inputs: &[req_h(
            "foreach",
            FieldKind::Expression,
            "JSONLogic expression that evaluates to the array to iterate",
        )],
        outputs: &[],
    },
    ActionDef {
        type_id: "Until",
        label: "Until",
        category: ActionCategory::Container,
        catalog_group: CatalogGroup::FlowControl,
        runnable: true,
        inputs: &[
            req_h(
                "until",
                FieldKind::Expression,
                "JSONLogic condition — loop body repeats while this is falsy",
            ),
            req_h(
                "limit",
                FieldKind::Json,
                r#"Loop guard, e.g. {"count": 10} or {"count": 5, "timeout": "PT30S"}"#,
            ),
        ],
        outputs: &[],
    },
    ActionDef {
        type_id: "If",
        label: "If",
        category: ActionCategory::Container,
        catalog_group: CatalogGroup::FlowControl,
        runnable: true,
        inputs: &[req_h(
            "expression",
            FieldKind::Expression,
            "JSONLogic condition — true branch runs when truthy",
        )],
        outputs: &[],
    },
    ActionDef {
        type_id: "Try",
        label: "Try/Catch",
        category: ActionCategory::Container,
        catalog_group: CatalogGroup::FlowControl,
        runnable: true,
        inputs: &[],
        outputs: &[output("error", FieldKind::Json)], // {actionId, message} inside catch — §3a
    },
];

/// Look up a registered action type by id. `None` means the canvas/engine
/// don't recognize `type_id` — a real error condition for the loader to
/// surface, not something to silently no-op past.
pub fn find(type_id: &str) -> Option<&'static ActionDef> {
    REGISTRY.iter().find(|d| d.type_id == type_id)
}

pub fn is_container(type_id: &str) -> bool {
    matches!(find(type_id), Some(d) if d.category == ActionCategory::Container)
}

impl FieldKind {
    /// The JSON Schema `type` this kind validates as, or `None` when the kind
    /// is an escape hatch that accepts any JSON value.
    ///
    /// `Json` and `Expression` deliberately map to *no* type rather than to
    /// one: both are open content (`http`'s `body`, `transform`'s result), and
    /// asserting a single type would report a schema error on every
    /// legitimate array or scalar.
    pub fn json_schema_type(self) -> Option<&'static str> {
        match self {
            FieldKind::String => Some("string"),
            FieldKind::Number => Some("number"),
            FieldKind::Bool => Some("boolean"),
            FieldKind::Json | FieldKind::Expression => None,
        }
    }
}

/// The JSON Schema used to validate a request-body companion file for
/// `type_id`.
///
/// The root is deliberately unconstrained. `http`'s `body` is declared
/// `FieldKind::Json` — an escape hatch that by definition accepts any JSON
/// value — so asserting an object shape here would flag every legitimate
/// array or scalar body. That is the same deliberate-permissiveness trade-off
/// `json_schema_store`'s `flow.schema.json` documents for `inputs`.
///
/// What the registry *can* contribute is the field catalog: which inputs the
/// action accepts and which outputs it produces, with their kinds. That
/// rides along as the schema's `description` so the editor shows it as hover
/// text, rather than as a constraint that would invent a body shape nobody
/// declared. Real enforcement is the user's job via the sidecar schema file
/// described by [`crate::designer_panel`]'s HTTP body editor.
///
/// Returns `None` when `type_id` isn't registered or declares no
/// [`FieldKind::Json`] input, so callers don't associate a schema that
/// describes nothing.
pub fn body_json_schema(type_id: &str) -> Option<serde_json::Value> {
    let def = find(type_id)?;
    let _body = def
        .inputs
        .iter()
        .find(|input| input.kind == FieldKind::Json)?;

    let describe = |name: &'static str, kind: FieldKind, help: &'static str| -> String {
        let kind_name = kind.json_schema_type().unwrap_or("any JSON value");
        if help.is_empty() {
            format!("{name} ({kind_name})")
        } else {
            format!("{name} ({kind_name}) — {help}")
        }
    };
    let inputs = def
        .inputs
        .iter()
        .map(|input| {
            let required = if input.required {
                ", required"
            } else {
                ", optional"
            };
            let described = describe(input.name, input.kind, input.help);
            format!("{described}{required}")
        })
        .collect::<Vec<_>>()
        .join("; ");
    let outputs = def
        .outputs
        .iter()
        .map(|output| describe(output.name, output.kind, ""))
        .collect::<Vec<_>>()
        .join("; ");

    Some(serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": format!("{} — request body", def.label),
        "description": format!(
            "Registry-derived description of the `{}` action (type_id {:?}). \
             Inputs: {inputs}. Outputs: {outputs}. \
             The body itself is a FieldKind::Json escape hatch that accepts any \
             JSON value, so it is intentionally unconstrained here — a stricter \
             shape would flag legitimate array and scalar bodies. To actually \
             validate the body, add a sidecar `.body.schema.json` file next to \
             it; that schema is applied in addition to this one.",
            def.type_id, def.type_id,
        ),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_type_id_is_unique() {
        let mut seen = std::collections::HashSet::new();
        for def in REGISTRY {
            assert!(
                seen.insert(def.type_id),
                "duplicate type_id: {}",
                def.type_id
            );
        }
    }

    #[test]
    fn container_types_match_the_schema() {
        for t in ["Foreach", "Until", "If", "Try"] {
            assert!(is_container(t), "{t} should be a Container");
        }
        for t in ["http", "transform", "script", "log"] {
            assert!(!is_container(t), "{t} should be a Leaf");
        }
    }

    #[test]
    fn json_schema_type_maps_escape_hatches_to_none() {
        assert_eq!(FieldKind::String.json_schema_type(), Some("string"));
        assert_eq!(FieldKind::Number.json_schema_type(), Some("number"));
        assert_eq!(FieldKind::Bool.json_schema_type(), Some("boolean"));
        // The whole point: an unconstrained body must not claim a type.
        assert_eq!(FieldKind::Json.json_schema_type(), None);
        assert_eq!(FieldKind::Expression.json_schema_type(), None);
    }

    #[test]
    fn http_body_schema_is_generated_and_unconstrained() {
        let schema = body_json_schema("http").expect("http declares a Json body input");

        // No root `type`/`properties`: the body is arbitrary JSON, and
        // asserting a shape here would false-positive on array/scalar bodies.
        assert!(schema.get("type").is_none(), "root must stay unconstrained");
        assert!(schema.get("properties").is_none(), "no invented body shape");

        let description = schema["description"].as_str().expect("description");
        assert!(
            description.contains("method"),
            "documents inputs: {description}"
        );
        assert!(
            description.contains("url"),
            "documents inputs: {description}"
        );
        assert!(
            description.contains("status"),
            "documents outputs: {description}"
        );
    }

    #[test]
    fn body_schema_is_none_without_a_json_input() {
        // `log` has no Json input, so there is nothing to describe.
        assert!(body_json_schema("log").is_none());
        // And an unregistered type must not invent a schema.
        assert!(body_json_schema("NotARealType").is_none());
    }

    #[test]
    fn http_method_is_a_closed_set() {
        let method = find("http")
            .unwrap()
            .inputs
            .iter()
            .find(|input| input.name == "method")
            .unwrap();
        assert_eq!(method.options, HTTP_METHODS);
        // Every other field stays free input.
        let url = find("http")
            .unwrap()
            .inputs
            .iter()
            .find(|input| input.name == "url")
            .unwrap();
        assert!(url.options.is_empty());
    }

    #[test]
    fn find_returns_none_for_unknown_type() {
        assert!(find("NotARealType").is_none());
    }

    // FUTURE FEATURE — restore with the `DbMigration`/`AiCheck` entries.
    //
    // #[test]
    // fn stub_types_are_not_runnable() {
    //     for t in ["DbMigration", "AiCheck"] {
    //         let def = find(t).unwrap_or_else(|| panic!("{t} missing from REGISTRY"));
    //         assert!(!def.runnable, "{t} should have runnable: false");
    //     }
    // }

    #[test]
    fn parked_future_types_are_not_registered() {
        for t in ["sendEmail", "DbMigration", "AiCheck"] {
            assert!(find(t).is_none(), "{t} is parked and must not be registered");
        }
    }

    #[test]
    fn all_runnable_types_are_not_stubs() {
        // Every type that isn't explicitly a stub must be runnable.
        for def in REGISTRY {
            if !matches!(def.type_id, "DbMigration" | "AiCheck") {
                assert!(def.runnable, "{} should be runnable", def.type_id);
            }
        }
    }

    #[test]
    fn catalog_groups_are_assigned() {
        // Spot-check a few expected groupings.
        assert_eq!(
            find("http").unwrap().catalog_group,
            CatalogGroup::Integrations
        );
        assert_eq!(find("transform").unwrap().catalog_group, CatalogGroup::Data);
        assert_eq!(
            find("script").unwrap().catalog_group,
            CatalogGroup::Processes
        );
        assert_eq!(find("If").unwrap().catalog_group, CatalogGroup::FlowControl);
        assert_eq!(find("log").unwrap().catalog_group, CatalogGroup::Utilities);
    }
}
