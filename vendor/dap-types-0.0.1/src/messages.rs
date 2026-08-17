use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    BreakpointEvent, Capabilities, CapabilitiesEvent, ContinuedEvent, ExitedEvent,
    InvalidatedEvent, LoadedSourceEvent, MemoryEvent, ModuleEvent, OutputEvent, ProcessEvent,
    ProgressEndEvent, ProgressStartEvent, ProgressUpdateEvent, StoppedEvent, TerminatedEvent,
    ThreadEvent,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Message {
    Event(Box<Events>),
    Response(Response),
    Request(Request),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Request {
    pub seq: u64,
    pub command: String,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub arguments: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Response {
    pub seq: u64,
    pub request_seq: u64,
    pub success: bool,
    pub command: String,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub body: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct OtherEvent {
    pub event: String,
    pub body: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "event", content = "body")]
#[serde(rename_all = "camelCase")]
pub enum Events {
    Initialized(Option<Capabilities>),
    Stopped(StoppedEvent),
    Continued(ContinuedEvent),
    Exited(ExitedEvent),
    Terminated(Option<TerminatedEvent>),
    Thread(ThreadEvent),
    Output(OutputEvent),
    Breakpoint(BreakpointEvent),
    Module(ModuleEvent),
    LoadedSource(LoadedSourceEvent),
    Process(ProcessEvent),
    Capabilities(CapabilitiesEvent),
    ProgressStart(ProgressStartEvent),
    ProgressUpdate(ProgressUpdateEvent),
    ProgressEnd(ProgressEndEvent),
    Invalidated(InvalidatedEvent),
    Memory(MemoryEvent),
    #[serde(untagged)]
    Other(OtherEvent),
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Events::Initialized(_) => write!(f, "Initialized"),
            Events::Stopped(_) => write!(f, "Stopped"),
            Events::Continued(_) => write!(f, "Continued"),
            Events::Exited(_) => write!(f, "Exited"),
            Events::Terminated(_) => write!(f, "Terminated"),
            Events::Thread(_) => write!(f, "Thread"),
            Events::Output(_) => write!(f, "Output"),
            Events::Breakpoint(_) => write!(f, "Breakpoint"),
            Events::Module(_) => write!(f, "Module"),
            Events::LoadedSource(_) => write!(f, "LoadedSource"),
            Events::Process(_) => write!(f, "Process"),
            Events::Capabilities(_) => write!(f, "Capabilities"),
            Events::ProgressStart(_) => write!(f, "ProgressStart"),
            Events::ProgressUpdate(_) => write!(f, "ProgressUpdate"),
            Events::ProgressEnd(_) => write!(f, "ProgressEnd"),
            Events::Invalidated(_) => write!(f, "Invalidated"),
            Events::Memory(_) => write!(f, "Memory"),
            Events::Other(other) => write!(f, "{}", other.event.as_str()),
        }
    }
}

fn deserialize_empty_object<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value == Value::Object(serde_json::Map::new()) {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_events_deserialization() {
        // Test a known event type (Stopped)
        let stopped_json = json!({
            "event": "stopped",
            "body": {
                "reason": "breakpoint",
                "threadId": 1
            }
        });
        let stopped_event: Events = serde_json::from_value(stopped_json).unwrap();
        assert!(matches!(stopped_event, Events::Stopped(_)));

        // Test an unknown event type
        let unknown_json = json!({
            "event": "customEvent",
            "body": {
                "someField": "someValue",
                "anotherField": 42
            }
        });
        let unknown_event: Events = serde_json::from_value(unknown_json).unwrap();

        if let Events::Other(other) = unknown_event {
            assert_eq!(other.event, "customEvent");
            assert_eq!(
                other.body,
                json!({
                    "someField": "someValue",
                    "anotherField": 42
                })
            );
        } else {
            panic!("Expected Other variant for unknown event");
        }
    }
}
