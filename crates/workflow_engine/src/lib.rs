//! The Forge workflow execution engine, ported out of Forge's
//! `backend/workflow` into a standalone crate: `flow.json` schema types,
//! the action registry, the level scheduler, per-action executors
//! (script/http/log/SendEmail/StartProcess/...), the container executors
//! (Branch/Foreach/If/Switch/Try/Until), the GUI-aware process tracker, the
//! project scanner, and the `RunContext` JSONLogic plumbing. Pure data +
//! tokio, no `gpui` dependency — the `FlowState` adapter that actually puts
//! this on the Designer canvas lives separately in `designer_panel`
//! (`workflow_json.rs`), same split Forge's `fdgn.rs` draws between grammar
//! and canvas.

mod async_rt;

pub mod container;
pub mod engine;
pub mod executors;
pub mod history;
pub mod layout;
pub mod process_tracker;
pub mod project_scan;
pub mod registry;
pub mod runner;
pub mod scheduler;
pub mod schema;
pub mod status;
pub mod task_chain;
pub mod validation;

pub use container::execute_action;
pub use engine::RunContext;
pub use executors::execute_leaf;
pub use history::{ActionHistoryRecord, RunHistoryEntry};
pub use layout::WorkflowLayout;
pub use process_tracker::{ProcessInfo, ProcessStatus};
pub use project_scan::{DetectedService, ServiceKind, scan as scan_project};
pub use registry::{ActionCategory, ActionDef, CatalogGroup, FieldKind, InputField, OutputField};
pub use runner::run_workflow;
pub use scheduler::{LevelResult, compute_levels, validate};
pub use schema::{
    Action, ActionMap, Branch, JsonLogicExpr, Limit, RunOutcome, ScriptRuntime, ScriptSource,
    WorkflowDefinition,
};
pub use status::{IterationScope, RunStatus, StatusSink};
pub use task_chain::ServiceSpec;
pub use validation::{ValidationIssue, ValidationSeverity, validate_definition};
