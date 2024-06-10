use crate::types::{DebuggerCapabilities, Source, ThreadId};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event", content = "body")]
// seq is omitted as unused and is not sent by some implementations
pub enum Event {
    Initialized(Option<DebuggerCapabilities>),
    Stopped(Stopped),
    Continued(Continued),
    Exited(Exited),
    Terminated(Option<Terminated>),
    Thread(Thread),
    Output(Output),
    Breakpoint(Breakpoint),
    Module(Module),
    LoadedSource(LoadedSource),
    Process(Process),
    Capabilities(Capabilities),
    // ProgressStart(),
    // ProgressUpdate(),
    // ProgressEnd(),
    // Invalidated(),
    Memory(Memory),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stopped {
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<ThreadId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_focus_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_stopped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_breakpoint_ids: Option<Vec<usize>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continued {
    pub thread_id: ThreadId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_continued: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Exited {
    pub exit_code: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Terminated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<Value>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub reason: String,
    pub thread_id: ThreadId,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoint {
    pub reason: String,
    pub breakpoint: crate::types::Breakpoint,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub reason: String,
    pub module: crate::types::Module,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedSource {
    pub reason: String,
    pub source: crate::types::Source,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_process_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_process: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_method: Option<String>, // TODO: use enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer_size: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub capabilities: crate::types::DebuggerCapabilities,
}

// #[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Invalidated {
// pub areas: Vec<InvalidatedArea>,
// pub thread_id: Option<ThreadId>,
// pub stack_frame_id: Option<usize>,
// }

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub memory_reference: String,
    pub offset: usize,
    pub count: usize,
}
