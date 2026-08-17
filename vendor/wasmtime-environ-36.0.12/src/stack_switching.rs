//! This module contains basic type definitions used by the implementation of
//! the stack switching proposal.

/// Discriminant of variant `Absent` in
/// `wasmtime::runtime::vm::VMStackChain`.
pub const STACK_CHAIN_ABSENT_DISCRIMINANT: usize = 0;
/// Discriminant of variant `InitialStack` in
/// `wasmtime::runtime::vm::VMStackChain`.
pub const STACK_CHAIN_INITIAL_STACK_DISCRIMINANT: usize = 1;
/// Discriminant of variant `Continiation` in
/// `wasmtime::runtime::vm::VMStackChain`.
pub const STACK_CHAIN_CONTINUATION_DISCRIMINANT: usize = 2;

/// Discriminant of variant `Fresh` in
/// `runtime::vm::VMStackState`.
pub const STACK_STATE_FRESH_DISCRIMINANT: u32 = 0;
/// Discriminant of variant `Running` in
/// `runtime::vm::VMStackState`.
pub const STACK_STATE_RUNNING_DISCRIMINANT: u32 = 1;
/// Discriminant of variant `Parent` in
/// `runtime::vm::VMStackState`.
pub const STACK_STATE_PARENT_DISCRIMINANT: u32 = 2;
/// Discriminant of variant `Suspended` in
/// `runtime::vm::VMStackState`.
pub const STACK_STATE_SUSPENDED_DISCRIMINANT: u32 = 3;
/// Discriminant of variant `Returned` in
/// `runtime::vm::VMStackState`.
pub const STACK_STATE_RETURNED_DISCRIMINANT: u32 = 4;

/// Discriminant of variant `Return` in
/// `runtime::vm::ControlEffect`.
pub const CONTROL_EFFECT_RETURN_DISCRIMINANT: u32 = 0;
/// Discriminant of variant `Resume` in
/// `runtime::vm::ControlEffect`.
pub const CONTROL_EFFECT_RESUME_DISCRIMINANT: u32 = 1;
/// Discriminant of variant `Suspend` in
/// `runtime::vm::ControlEffect`.
pub const CONTROL_EFFECT_SUSPEND_DISCRIMINANT: u32 = 2;
/// Discriminant of variant `Switch` in
/// `runtime::vm::ControlEffect`.
pub const CONTROL_EFFECT_SWITCH_DISCRIMINANT: u32 = 3;
