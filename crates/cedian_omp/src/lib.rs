//! `cedian_omp`: process + protocol boundary to the bundled OMP sidecar.
//!
//! Plan §§4–6: `cedian.app` spawns `omp --mode rpc-ui` over stdio, negotiates
//! RPC v2 through the vendored upstream client (`vendor/omp-rpc`), and fans
//! session events out through [`EventRouter`]. Blocking client calls live on
//! caller threads off-UI — never the UI thread (plan §5).
#![allow(
    clippy::disallowed_methods,
    reason = "headless, synchronous process control (OMP, LSP, DAP, Chrome, git, sandbox-exec): Zed's async spawn helpers do not apply"
)]

pub mod dialog;
pub mod driver;
pub mod errors;
pub mod event_router;
pub mod history;
pub mod omp_config;
pub mod protected;
pub mod runtime;
pub mod sandbox;
pub mod session;
pub mod sessions;
pub mod spawn_profile;

pub use dialog::{
    Answerer, DialogRecord, GateDecision, UserAnswer, abandoned, abstained, headless_answer,
    now_ms, user_answer,
};
pub use errors::OmpError;
pub use event_router::{
    CallOutcome, DeltaKind, EventRouter, FinishedToolCall, LogEntry, PromptStatus, RouterEvent,
    SubagentStatus, TextBefore, Toast, ToastLevel, texts_before,
};
pub use omp_config::{Layer, OmpConfig, Setting, WriteOutcome};
pub use protected::{
    METADATA_DIR_NAMES, ProtectedKind, ProtectedRoots, approval_targets_protected,
    default_protected_bash_rules,
};
pub use runtime::{
    APP_PROMPT_TIMEOUT, HEADLESS_PROMPT_TIMEOUT, NewSession, OmpBinary, OmpRuntime, RuntimeConfig,
    RuntimeControl, RuntimeState, image_content, last_text,
};
pub use session::{
    ResumeState, SNAPSHOT_VERSION, SessionBinding, SnapshotVersionMismatch, validate_binding,
};
pub use sessions::SessionEntry;
pub use spawn_profile::{
    ApprovalMode, Approvals, BashRule, Sessions, SpawnPlan, SpawnPolicy, SpawnProfile, ToolPolicy,
    omp_config_get, omp_version, resolve_on_path, scrub_env,
};
