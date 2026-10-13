//! `RunContext` — §2 of `DOCS/workflow-schema/execution-engine-design.md`.
//! The JSONLogic evaluation context threaded through the scheduler (§3,
//! `scheduler.rs`, fully built): a flat `{actionId: {"outputs": {...}}}` map
//! built up incrementally as actions complete, plus (§1) a reserved `$item`
//! slot bound only while a `Foreach` iteration's body is running.
//!
//! Deliberately doesn't know about `Action`/`ActionMap`/the scheduler at
//! all — same "data, not code" separation the registry already follows.
//! This is pure JSONLogic plumbing; scheduling and per-action-type
//! execution are separate, later pieces of the same design doc.

use std::sync::{Arc, RwLock};

use datalogic_rs::Engine;
use serde_json::{Map, Value};

/// Reserved data-context key for a `Foreach` iteration's loop variable —
/// see the design doc §1's "Locked" note. Not a valid action id (action ids
/// must start with a letter per the schema's `actionId` pattern), so this
/// can never collide with a real action's own entry.
const ITEM_KEY: &str = "$item";

/// Data-context key the flow's variables are read under:
/// `{"var": "variables.count"}`. An action can't use it as its id (see
/// `validation`), so it never hides an action's outputs.
pub const VARIABLES_KEY: &str = "variables";

/// What a flow variable holds. Fixed when the variable is initialized, and
/// every later `SetVariable` has to supply a value of the same type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Object,
    String,
    Integer,
    Array,
}

impl VariableType {
    /// The names used in `flow.json`, in the order the Designer lists them.
    pub const NAMES: &'static [&'static str] = &["object", "string", "integer", "array"];

    pub fn parse(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "object" => Some(Self::Object),
            "string" => Some(Self::String),
            "integer" => Some(Self::Integer),
            "array" => Some(Self::Array),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Object => "object",
            Self::String => "string",
            Self::Integer => "integer",
            Self::Array => "array",
        }
    }

    /// The value a variable of this type starts with when none is given.
    pub fn default_value(self) -> Value {
        match self {
            Self::Object => Value::Object(Map::new()),
            Self::String => Value::String(String::new()),
            Self::Integer => Value::from(0),
            Self::Array => Value::Array(Vec::new()),
        }
    }

    /// Fits `value` to this type, or says why it doesn't fit.
    ///
    /// Scalars convert where the meaning is unambiguous, because a value
    /// typed into a form arrives as whatever JSON it happens to parse as: `5`
    /// for a string variable becomes `"5"`, and `"5"` for an integer becomes
    /// `5`. Nothing converts to or from an object or an array.
    pub fn coerce(self, value: Value) -> Result<Value, String> {
        let mismatch = |value: &Value| format!("expected {} {}, got {value}", self.article(), self.name());
        match (self, value) {
            (Self::Object, value @ Value::Object(_)) => Ok(value),
            (Self::Array, value @ Value::Array(_)) => Ok(value),
            (Self::String, value @ Value::String(_)) => Ok(value),
            (Self::String, Value::Null) => Ok(Value::String(String::new())),
            (Self::String, value @ (Value::Number(_) | Value::Bool(_))) => {
                Ok(Value::String(value.to_string()))
            }
            (Self::Integer, Value::Number(number)) => {
                if number.is_i64() || number.is_u64() {
                    Ok(Value::Number(number))
                } else {
                    match number.as_f64() {
                        Some(float) if float.fract() == 0.0 => Ok(Value::from(float as i64)),
                        _ => Err(mismatch(&Value::Number(number))),
                    }
                }
            }
            (Self::Integer, Value::String(text)) => text
                .trim()
                .parse::<i64>()
                .map(Value::from)
                .map_err(|_| mismatch(&Value::String(text))),
            (_, value) => Err(mismatch(&value)),
        }
    }

    fn article(self) -> &'static str {
        match self {
            Self::Object | Self::Integer | Self::Array => "an",
            Self::String => "a",
        }
    }
}

/// One per flow execution, cheaply `Clone`d to produce nested-scope
/// variants (§2) — the `Arc<RwLock<_>>` data layer is what's actually
/// shared or forked, not this struct itself.
#[derive(Clone)]
pub struct RunContext {
    /// The flow's variables, by name. Unlike `data`, this is one map for the
    /// whole run: a `child()` context shares it, so a variable set inside a
    /// loop body is still set after the loop. That is what a variable is
    /// for, and it is also why two iterations writing the same variable
    /// race (a `Foreach` runs its iterations concurrently).
    variables: Arc<RwLock<Map<String, Value>>>,
    variable_types: Arc<RwLock<std::collections::HashMap<String, VariableType>>>,
    /// Action id -> `{"outputs": {...}}`, written once per action as it
    /// completes and read by every JSONLogic evaluation from this point
    /// on. `Arc<RwLock<_>>` so concurrently-running sibling actions (§3's
    /// scheduler runs everything runAfter-eligible at once) can all read
    /// it while only the action that just finished writes to it.
    data: Arc<RwLock<Map<String, Value>>>,
    /// This context's own `$item` binding, if any — see `with_item`.
    /// Deliberately *not* stored in `data` itself: `data` is shared with
    /// `child()`-derived contexts of the *previous* iteration, and if
    /// `$item` lived there, clearing/overwriting it for iteration N+1
    /// would need extra bookkeeping to undo. Keeping it as a plain field
    /// on `RunContext` (cheap to `Clone`) sidesteps that entirely.
    item: Option<Value>,
    engine: Arc<Engine>,
}

impl RunContext {
    pub fn new() -> Self {
        Self {
            variables: Arc::new(RwLock::new(Map::new())),
            variable_types: Arc::new(RwLock::new(std::collections::HashMap::new())),
            data: Arc::new(RwLock::new(Map::new())),
            item: None,
            engine: Arc::new(Engine::new()),
        }
    }

    /// A snapshot for one JSONLogic evaluation — a clone, not a live view,
    /// so an expression sees a consistent picture even if another action
    /// finishes mid-evaluation (design doc §2).
    pub fn snapshot(&self) -> Value {
        let mut map = self.data.read().unwrap().clone();
        if let Some(item) = &self.item {
            map.insert(ITEM_KEY.to_string(), item.clone());
        }
        map.insert(
            VARIABLES_KEY.to_string(),
            Value::Object(self.variables.read().unwrap().clone()),
        );
        Value::Object(map)
    }

    /// Everything a script launched now can be told about the run, as one
    /// JSON object:
    ///
    /// - `variables`: every flow variable by name;
    /// - `item`: the enclosing `Foreach`'s current item, or `null`;
    /// - `args`: what the action is also piping to stdin, or `null`;
    /// - `actions`: each finished action's outputs, by action id.
    ///
    /// The same data an expression sees, reshaped so a script reads
    /// `actions.fetch.body` instead of `fetch.outputs.body`.
    pub fn script_context(&self, args: Option<&Value>) -> Value {
        let mut actions = Map::new();
        for (id, entry) in self.data.read().unwrap().iter() {
            let outputs = entry.get("outputs").cloned().unwrap_or(Value::Null);
            actions.insert(id.clone(), outputs);
        }
        let mut context = Map::new();
        context.insert(
            VARIABLES_KEY.to_string(),
            Value::Object(self.variables.read().unwrap().clone()),
        );
        context.insert("item".to_string(), self.item.clone().unwrap_or(Value::Null));
        context.insert("args".to_string(), args.cloned().unwrap_or(Value::Null));
        context.insert("actions".to_string(), Value::Object(actions));
        Value::Object(context)
    }

    /// Creates the variable `name` with a fixed type and a starting value,
    /// readable from then on as `variables.<name>`. Initializing a name
    /// again replaces it, type included, so a loop body that initializes
    /// its own variable gets a fresh one each iteration. Returns the value
    /// stored, after [`VariableType::coerce`].
    pub fn init_variable(
        &self,
        name: &str,
        kind: VariableType,
        value: Value,
    ) -> Result<Value, String> {
        let value = kind
            .coerce(value)
            .map_err(|e| format!("variable \"{name}\": {e}"))?;
        self.variable_types
            .write()
            .unwrap()
            .insert(name.to_string(), kind);
        self.variables
            .write()
            .unwrap()
            .insert(name.to_string(), value.clone());
        Ok(value)
    }

    /// Replaces the value of an initialized variable. Fails for a name that
    /// was never initialized, which is nearly always a typo, and for a value
    /// that doesn't fit the variable's type. Returns the value stored.
    pub fn set_variable(&self, name: &str, value: Value) -> Result<Value, String> {
        let kind = self
            .variable_types
            .read()
            .unwrap()
            .get(name)
            .copied()
            .ok_or_else(|| {
                format!("variable \"{name}\" hasn't been initialized — add an Initialize Variable action before this one")
            })?;
        let value = kind
            .coerce(value)
            .map_err(|e| format!("variable \"{name}\": {e}"))?;
        self.variables
            .write()
            .unwrap()
            .insert(name.to_string(), value.clone());
        Ok(value)
    }

    /// Records a just-finished action's outputs, visible to every
    /// evaluation from this point on (including ones already in flight
    /// that haven't yet called `snapshot`, since sibling actions read
    /// through the same `Arc<RwLock<_>>`).
    pub fn record_outputs(&self, action_id: &str, outputs: Map<String, Value>) {
        let mut entry = Map::new();
        entry.insert("outputs".to_string(), Value::Object(outputs));
        self.data
            .write()
            .unwrap()
            .insert(action_id.to_string(), Value::Object(entry));
    }

    /// Evaluate one JSONLogic expression (`foreach`/`until`/`expression`,
    /// or any `inputs` field of `registry::FieldKind::Expression`) against
    /// the current snapshot.
    ///
    /// **Discovered empirically, not documented anywhere upstream**: this
    /// `datalogic-rs` build parses *any* JSON object appearing anywhere in
    /// the rule tree as an operator call — including one meant as a
    /// literal data value, and even wrapped in the nominal `"val"` escape
    /// operator. `{"sku": "ITEM-1"}` embedded as data (e.g. inside an
    /// `"or"` fallback array) fails with `"Invalid operator: sku"`, not a
    /// parse error naming the real problem. Numbers, strings, bools, and
    /// arrays of those are fine as literals; a bare JSON object never is.
    /// Build literal structured data by composing an object from `outputs`
    /// that already exist as real actions' results, not by writing an
    /// object literal directly into an expression.
    pub fn evaluate(&self, expr: &Value) -> Result<Value, String> {
        let data = self.snapshot();
        self.engine
            .eval_into::<Value, _, _>(expr, &data)
            .map_err(|e| e.to_string())
    }

    /// §2: a `Foreach` body's per-iteration child layer. Starts as a copy
    /// of `self`'s current data (so the body can read everything that's
    /// resolved so far, including ancestors' siblings), but writes made
    /// through the child (i.e. actions inside the loop body) land only in
    /// the child's own layer — never propagating back to `self` or to a
    /// sibling iteration's own `child()`. Without this, re-running the same
    /// body every iteration would let iteration 2's actions silently
    /// overwrite iteration 1's outputs under the same ids in one shared
    /// map.
    pub fn child(&self) -> Self {
        let snapshot = self.data.read().unwrap().clone();
        Self {
            variables: self.variables.clone(),
            variable_types: self.variable_types.clone(),
            data: Arc::new(RwLock::new(snapshot)),
            item: None,
            engine: self.engine.clone(),
        }
    }

    /// §1: bind `$item` for one `Foreach` iteration's body. Shares the same
    /// data layer as `self` (only the reserved `$item` slot differs), so
    /// this is cheap to call once per iteration — pair it with `child()`
    /// once per `Foreach` (not once per iteration) and call `with_item` on
    /// that child for each item in the evaluated `foreach` array.
    pub fn with_item(&self, item: Value) -> Self {
        Self {
            item: Some(item),
            ..self.clone()
        }
    }
}

impl Default for RunContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_variable_is_read_under_the_variables_key_and_keeps_its_type() {
        let ctx = RunContext::new();
        ctx.init_variable("count", VariableType::Integer, Value::from(1))
            .unwrap();
        let read = |ctx: &RunContext| {
            ctx.evaluate(&serde_json::json!({"var": "variables.count"}))
                .unwrap()
        };
        assert_eq!(read(&ctx), Value::from(1));

        ctx.set_variable("count", Value::from(2)).unwrap();
        assert_eq!(read(&ctx), Value::from(2));

        // The type was fixed at initialization.
        let err = ctx
            .set_variable("count", serde_json::json!(["x"]))
            .unwrap_err();
        assert!(err.contains("integer"), "{err}");
        assert_eq!(read(&ctx), Value::from(2));

        // An unknown name is an error, not a new variable.
        let err = ctx.set_variable("cuont", Value::from(3)).unwrap_err();
        assert!(err.contains("hasn't been initialized"), "{err}");
    }

    /// A loop body's outputs stay in its own layer, but a variable it sets
    /// is the flow's, so it is still set once the loop is done.
    #[test]
    fn a_variable_set_in_a_child_context_is_seen_by_the_parent() {
        let ctx = RunContext::new();
        ctx.init_variable("last", VariableType::String, Value::from(""))
            .unwrap();
        let iteration = ctx.child().with_item(Value::from("b"));
        iteration.set_variable("last", Value::from("b")).unwrap();
        assert_eq!(
            ctx.evaluate(&serde_json::json!({"var": "variables.last"}))
                .unwrap(),
            Value::from("b")
        );
    }

    #[test]
    fn script_context_gathers_what_a_script_can_be_told() {
        let ctx = RunContext::new();
        ctx.init_variable("ids", VariableType::Array, serde_json::json!([1]))
            .unwrap();
        ctx.record_outputs(
            "fetch",
            serde_json::json!({"status": 200})
                .as_object()
                .unwrap()
                .clone(),
        );
        assert_eq!(
            ctx.script_context(None),
            serde_json::json!({
                "variables": {"ids": [1]},
                "item": null,
                "args": null,
                "actions": {"fetch": {"status": 200}},
            })
        );

        let iteration = ctx.child().with_item(serde_json::json!({"id": 7}));
        let context = iteration.script_context(Some(&Value::from("x")));
        assert_eq!(context["item"], serde_json::json!({"id": 7}));
        assert_eq!(context["args"], Value::from("x"));
        assert_eq!(context["actions"]["fetch"]["status"], Value::from(200));
    }

    #[test]
    fn variable_values_are_fitted_to_their_type() {
        use VariableType::*;
        assert_eq!(String.coerce(Value::from(5)), Ok(Value::from("5")));
        assert_eq!(String.coerce(Value::Null), Ok(Value::from("")));
        assert_eq!(Integer.coerce(Value::from(" 12 ")), Ok(Value::from(12)));
        assert_eq!(Integer.coerce(Value::from(3.0)), Ok(Value::from(3)));
        assert!(Integer.coerce(Value::from(3.5)).is_err());
        assert!(Integer.coerce(Value::from("x")).is_err());
        assert!(Object.coerce(serde_json::json!([])).is_err());
        assert!(Array.coerce(serde_json::json!({})).is_err());
        assert_eq!(Array.coerce(serde_json::json!([1])), Ok(serde_json::json!([1])));
        assert!(String.coerce(serde_json::json!({})).is_err());

        for name in VariableType::NAMES {
            let kind = VariableType::parse(name).unwrap();
            assert_eq!(kind.name(), *name);
            assert_eq!(kind.coerce(kind.default_value()), Ok(kind.default_value()));
        }
        assert_eq!(VariableType::parse("Integer"), Some(Integer));
        assert_eq!(VariableType::parse("float"), None);
    }

    #[test]
    fn records_and_reads_back_outputs_by_action_id() {
        let ctx = RunContext::new();
        ctx.record_outputs(
            "fetchOrder",
            serde_json::json!({"total": 42})
                .as_object()
                .unwrap()
                .clone(),
        );

        let result = ctx
            .evaluate(&serde_json::json!({"var": "fetchOrder.outputs.total"}))
            .unwrap();
        assert_eq!(result, Value::from(42));
    }

    #[test]
    fn missing_var_evaluates_to_null_not_an_error() {
        let ctx = RunContext::new();
        let result = ctx
            .evaluate(&serde_json::json!({"var": "neverRan.outputs.x"}))
            .unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn item_is_only_visible_when_bound() {
        let ctx = RunContext::new();
        // Not bound yet — $item resolves to null, not an error.
        let before = ctx
            .evaluate(&serde_json::json!({"var": "$item.sku"}))
            .unwrap();
        assert_eq!(before, Value::Null);

        let bound = ctx.with_item(serde_json::json!({"sku": "ABC"}));
        let during = bound
            .evaluate(&serde_json::json!({"var": "$item.sku"}))
            .unwrap();
        assert_eq!(during, Value::from("ABC"));

        // The original context (sibling scope) was never mutated.
        let after = ctx
            .evaluate(&serde_json::json!({"var": "$item.sku"}))
            .unwrap();
        assert_eq!(after, Value::Null);
    }

    #[test]
    fn child_sees_parent_data_but_writes_stay_local() {
        let parent = RunContext::new();
        parent.record_outputs(
            "setup",
            serde_json::json!({"ready": true})
                .as_object()
                .unwrap()
                .clone(),
        );

        let child = parent.child();
        // Child can read what the parent already resolved before the fork.
        let seen = child
            .evaluate(&serde_json::json!({"var": "setup.outputs.ready"}))
            .unwrap();
        assert_eq!(seen, Value::from(true));

        // Child writes (an action inside the Foreach body finishing) don't
        // leak back to the parent or to a sibling iteration's own child.
        child.record_outputs(
            "processItem",
            serde_json::json!({"result": "ok"})
                .as_object()
                .unwrap()
                .clone(),
        );
        let sibling = parent.child();
        let leaked = sibling
            .evaluate(&serde_json::json!({"var": "processItem.outputs.result"}))
            .unwrap();
        assert_eq!(leaked, Value::Null);

        let parent_leaked = parent
            .evaluate(&serde_json::json!({"var": "processItem.outputs.result"}))
            .unwrap();
        assert_eq!(parent_leaked, Value::Null);
    }

    #[test]
    fn nested_containers_share_the_parent_context_directly() {
        // §2: `If`/`Until`/`Try` don't fork — they evaluate straight
        // against the same `RunContext` their caller holds (no `child()`
        // call in their executors, once those are written), so this test
        // just documents that `RunContext` itself imposes no such
        // isolation unless `child()` is explicitly used — the scheduler
        // is what decides per container type.
        let ctx = RunContext::new();
        ctx.record_outputs(
            "a",
            serde_json::json!({"x": 1}).as_object().unwrap().clone(),
        );
        let same_scope = ctx.clone();
        same_scope.record_outputs(
            "b",
            serde_json::json!({"y": 2}).as_object().unwrap().clone(),
        );

        let seen = ctx
            .evaluate(&serde_json::json!({"var": "b.outputs.y"}))
            .unwrap();
        assert_eq!(seen, Value::from(2));
    }
}
