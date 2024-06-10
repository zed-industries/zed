use crate::types::{
    Breakpoint, DebuggerCapabilities, Scope, Source, SourceBreakpoint, StackFrame,
    StackFrameFormat, Thread, ThreadId, ValueFormat, Variable, VariablePresentationHint,
};
use ::serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

pub trait Request {
    type Arguments: DeserializeOwned + Serialize;
    type Result: DeserializeOwned + Serialize;
    const COMMAND: &'static str;
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeArguments {
    #[serde(rename = "clientID", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(rename = "adapterID")]
    pub adapter_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "linesStartAt1", skip_serializing_if = "Option::is_none")]
    pub lines_start_at_one: Option<bool>,
    #[serde(rename = "columnsStartAt1", skip_serializing_if = "Option::is_none")]
    pub columns_start_at_one: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_variable_type: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_variable_paging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_run_in_terminal_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_memory_references: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_progress_reporting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_invalidated_event: Option<bool>,
}

#[derive(Debug)]
pub enum Initialize {}

impl Request for Initialize {
    type Arguments = InitializeArguments;
    type Result = DebuggerCapabilities;
    const COMMAND: &'static str = "initialize";
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchRequestArguments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debug: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub __restart: Option<Value>,
}

#[derive(Debug)]
pub enum Launch {}

impl Request for Launch {
    type Arguments = LaunchRequestArguments;
    type Result = ();
    const COMMAND: &'static str = "launch";
}

#[derive(Debug)]
pub enum Attach {}

impl Request for Attach {
    type Arguments = Value;
    type Result = ();
    const COMMAND: &'static str = "attach";
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectArguments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_debuggee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend_debuggee: Option<bool>,
}

#[derive(Debug)]
pub enum Restart {}

impl Request for Restart {
    type Arguments = Value;
    type Result = ();
    const COMMAND: &'static str = "restart";
}

#[derive(Debug)]
pub enum Disconnect {}

impl Request for Disconnect {
    type Arguments = Option<DisconnectArguments>;
    type Result = ();
    const COMMAND: &'static str = "disconnect";
}

#[derive(Debug)]
pub enum ConfigurationDone {}

impl Request for ConfigurationDone {
    type Arguments = ();
    type Result = ();
    const COMMAND: &'static str = "configurationDone";
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsArguments {
    pub source: Source,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<SourceBreakpoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_modified: Option<bool>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<Breakpoint>>,
}

#[derive(Debug)]
pub enum SetBreakpoints {}

impl Request for SetBreakpoints {
    type Arguments = SetBreakpointsArguments;
    type Result = SetBreakpointsResponse;
    const COMMAND: &'static str = "setBreakpoints";
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueArguments {
    pub thread_id: ThreadId,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_continued: Option<bool>,
}

#[derive(Debug)]
pub enum Continue {}

impl Request for Continue {
    type Arguments = ContinueArguments;
    type Result = ContinueResponse;
    const COMMAND: &'static str = "continue";
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceArguments {
    pub thread_id: ThreadId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StackFrameFormat>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_frames: Option<usize>,
    pub stack_frames: Vec<StackFrame>,
}

#[derive(Debug)]
pub enum StackTrace {}

impl Request for StackTrace {
    type Arguments = StackTraceArguments;
    type Result = StackTraceResponse;
    const COMMAND: &'static str = "stackTrace";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadsResponse {
    pub threads: Vec<Thread>,
}

#[derive(Debug)]
pub enum Threads {}

impl Request for Threads {
    type Arguments = ();
    type Result = ThreadsResponse;
    const COMMAND: &'static str = "threads";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesArguments {
    pub frame_id: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesResponse {
    pub scopes: Vec<Scope>,
}

#[derive(Debug)]
pub enum Scopes {}

impl Request for Scopes {
    type Arguments = ScopesArguments;
    type Result = ScopesResponse;
    const COMMAND: &'static str = "scopes";
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablesArguments {
    pub variables_reference: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablesResponse {
    pub variables: Vec<Variable>,
}

#[derive(Debug)]
pub enum Variables {}

impl Request for Variables {
    type Arguments = VariablesArguments;
    type Result = VariablesResponse;
    const COMMAND: &'static str = "variables";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInArguments {
    pub thread_id: ThreadId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
}

#[derive(Debug)]
pub enum StepIn {}

impl Request for StepIn {
    type Arguments = StepInArguments;
    type Result = ();
    const COMMAND: &'static str = "stepIn";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOutArguments {
    pub thread_id: ThreadId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
}

#[derive(Debug)]
pub enum StepOut {}

impl Request for StepOut {
    type Arguments = StepOutArguments;
    type Result = ();
    const COMMAND: &'static str = "stepOut";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextArguments {
    pub thread_id: ThreadId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
}

#[derive(Debug)]
pub enum Next {}

impl Request for Next {
    type Arguments = NextArguments;
    type Result = ();
    const COMMAND: &'static str = "next";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PauseArguments {
    pub thread_id: ThreadId,
}

#[derive(Debug)]
pub enum Pause {}

impl Request for Pause {
    type Arguments = PauseArguments;
    type Result = ();
    const COMMAND: &'static str = "pause";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateArguments {
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResponse {
    pub result: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,
    pub variables_reference: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reference: Option<String>,
}

#[derive(Debug)]
pub enum Evaluate {}

impl Request for Evaluate {
    type Arguments = EvaluateArguments;
    type Result = EvaluateResponse;
    const COMMAND: &'static str = "evaluate";
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExceptionBreakpointsArguments {
    pub filters: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExceptionBreakpointsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<Breakpoint>>,
}

#[derive(Debug)]
pub enum SetExceptionBreakpoints {}

impl Request for SetExceptionBreakpoints {
    type Arguments = SetExceptionBreakpointsArguments;
    type Result = SetExceptionBreakpointsResponse;
    const COMMAND: &'static str = "setExceptionBreakpoints";
}

// Reverse Requests

#[derive(Debug, Default, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_process_id: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalArguments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub cwd: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, Option<String>>>,
}

#[derive(Debug)]
pub enum RunInTerminal {}

impl Request for RunInTerminal {
    type Arguments = RunInTerminalArguments;
    type Result = RunInTerminalResponse;
    const COMMAND: &'static str = "runInTerminal";
}
