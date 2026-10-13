//! `cedian_worker`: parallel workers on isolated git worktrees (S5).
//!
//! Plan refs: §43 R1 (cedian owns worktrees, OMP requests; conflict → STALE,
//! never auto-merge; merge-back needs explicit accept).
//!
//! Phase 16 (mechanism; the app registers `cedian_worktree_request` since
//! S9 U8, ADR-0050). §18 R3 (hunk precedence surfaces STALE through
//! the existing tracker; the registry adds no new hunk states).
//!
//! S9 handoff: this crate is the headless mechanism (registry record +
//! blocking `git` subprocess calls); the GUI binding visualizes `Registry`
//! and stays out of orchestration (§88: OMP is the only orchestrator).
//!
//! One-shot per invocation: each CLI command shells out to
//! `git worktree`, persists the updated [`Registry`], and exits — no daemon
//! is ever held.
#![allow(
    clippy::disallowed_methods,
    reason = "headless, synchronous process control (OMP, LSP, DAP, Chrome, git, sandbox-exec): Zed's async spawn helpers do not apply"
)]

pub mod brief;
pub mod host_tool;
pub mod registry;
pub mod worktree;

pub use brief::{Brief, BriefError, Scope};
pub use host_tool::{WORKTREE_REQUEST_TOOL, request_worktree, worktree_request_tool};
pub use registry::{Registry, WorkerError, WorkerHead, WorkerStatus};
pub use worktree::{
    MergePlan, last_commit_ms, merge_back, merge_preview, remove, spawn, validate_id,
};
