use std::{fmt, future::Future, pin::Pin};

use anyhow::Result;
use futures::Stream;

/// Stable identifier for a REPL provider registered with Zed.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReplProviderId(String);

impl ReplProviderId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ReplProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Features a provider supports in addition to evaluating source code.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplCapabilities {
    pub interrupt: bool,
    pub namespaces: bool,
    pub load_file: bool,
    pub completions: bool,
    pub inspection: bool,
}

/// Describes a connection requested for a project.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplConnectRequest {
    pub project_path: String,
    pub session_kind: String,
}

/// Metadata for a live REPL session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplSession {
    pub id: String,
    pub provider_id: ReplProviderId,
    pub kind: String,
    pub namespace: Option<String>,
    pub capabilities: ReplCapabilities,
}

/// Source submitted to a REPL session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplEvaluation {
    pub code: String,
    pub namespace: Option<String>,
    pub file_path: Option<String>,
}

/// A single streamed event emitted while evaluating source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplOutputEvent {
    Value(String),
    Stdout(String),
    Stderr(String),
    Exception(String),
    Status(ReplEvaluationStatus),
}

/// Lifecycle state for an evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplEvaluationStatus {
    Running,
    Finished,
    Interrupted,
    Failed(String),
}

impl ReplEvaluationStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ReplEvaluationStatus::Finished
                | ReplEvaluationStatus::Interrupted
                | ReplEvaluationStatus::Failed(_)
        )
    }
}

/// Result stream returned by a REPL connection for one evaluation.
pub type ReplOutputStream = Pin<Box<dyn Stream<Item = Result<ReplOutputEvent>> + Send>>;

/// Pending connection attempt initiated by a provider.
pub type ReplConnectTask = Pin<Box<dyn Future<Output = Result<Box<dyn ReplConnection>>> + Send>>;

/// A uniquely interruptible evaluation and its output stream.
pub struct ReplExecution {
    pub id: String,
    pub output: ReplOutputStream,
}

/// A connected REPL transport.
pub trait ReplConnection: Send + Sync {
    fn session(&self) -> &ReplSession;
    fn evaluate(&self, evaluation: ReplEvaluation) -> Result<ReplExecution>;
    fn interrupt(&self, execution_id: &str) -> Result<()>;
    fn close(&self) -> Result<()>;
}

/// A language-specific implementation that can establish REPL connections.
///
/// Native integrations and extension-host adapters implement this trait. The
/// extension ABI will mirror this contract instead of exposing a transport
/// protocol such as nREPL directly to the UI.
pub trait ReplProvider: Send + Sync {
    fn id(&self) -> ReplProviderId;
    fn display_name(&self) -> &str;
    fn capabilities(&self) -> ReplCapabilities;
    fn connect(&self, request: ReplConnectRequest) -> ReplConnectTask;
}

/// Persistent in-memory model for a REPL transcript.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplTranscript {
    entries: Vec<ReplTranscriptEntry>,
}

impl ReplTranscript {
    pub fn entries(&self) -> &[ReplTranscriptEntry] {
        &self.entries
    }

    pub fn begin_evaluation(&mut self, evaluation: ReplEvaluation) -> usize {
        self.entries.push(ReplTranscriptEntry {
            evaluation,
            events: Vec::new(),
            status: ReplEvaluationStatus::Running,
        });
        self.entries.len() - 1
    }

    pub fn push_event(&mut self, entry_index: usize, event: ReplOutputEvent) -> bool {
        let Some(entry) = self.entries.get_mut(entry_index) else {
            return false;
        };
        if let ReplOutputEvent::Status(status) = &event {
            entry.status = status.clone();
        }
        entry.events.push(event);
        true
    }
}

/// One submitted expression and every event it emitted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplTranscriptEntry {
    pub evaluation: ReplEvaluation,
    pub events: Vec<ReplOutputEvent>,
    pub status: ReplEvaluationStatus,
}

/// State shared by a REPL transcript renderer and its editable input.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplView {
    transcript: ReplTranscript,
    input: String,
    namespace: Option<String>,
}

impl ReplView {
    pub fn transcript(&self) -> &ReplTranscript {
        &self.transcript
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn set_input(&mut self, input: impl Into<String>) {
        self.input = input.into();
    }

    pub fn set_namespace(&mut self, namespace: Option<String>) {
        self.namespace = namespace;
    }

    pub fn take_evaluation(&mut self) -> Option<ReplEvaluation> {
        let code = std::mem::take(&mut self.input);
        (!code.trim().is_empty()).then(|| ReplEvaluation {
            code,
            namespace: self.namespace.clone(),
            file_path: None,
        })
    }

    pub fn begin_evaluation(&mut self, evaluation: ReplEvaluation) -> usize {
        self.transcript.begin_evaluation(evaluation)
    }

    pub fn push_event(&mut self, entry_index: usize, event: ReplOutputEvent) -> bool {
        self.transcript.push_event(entry_index, event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transcript_keeps_events_and_terminal_status_together() {
        let mut transcript = ReplTranscript::default();
        let entry = transcript.begin_evaluation(ReplEvaluation {
            code: "(+ 1 2)".into(),
            namespace: Some("user".into()),
            file_path: None,
        });

        assert!(transcript.push_event(entry, ReplOutputEvent::Stdout("computing\n".into())));
        assert!(transcript.push_event(entry, ReplOutputEvent::Value("3".into())));
        assert!(transcript.push_event(
            entry,
            ReplOutputEvent::Status(ReplEvaluationStatus::Finished),
        ));

        let entry = &transcript.entries()[0];
        assert_eq!(entry.events.len(), 3);
        assert_eq!(entry.status, ReplEvaluationStatus::Finished);
        assert!(entry.status.is_terminal());
    }

    #[test]
    fn transcript_rejects_events_for_missing_entries() {
        let mut transcript = ReplTranscript::default();
        assert!(!transcript.push_event(0, ReplOutputEvent::Value("3".into())));
    }

    #[test]
    fn view_turns_editable_input_into_an_evaluation() {
        let mut view = ReplView::default();
        view.set_namespace(Some("my.app".into()));
        view.set_input("(inc 2)");

        let evaluation = view.take_evaluation().expect("input should evaluate");
        assert_eq!(evaluation.code, "(inc 2)");
        assert_eq!(evaluation.namespace.as_deref(), Some("my.app"));
        assert!(view.input().is_empty());
        assert!(view.take_evaluation().is_none());
    }
}
