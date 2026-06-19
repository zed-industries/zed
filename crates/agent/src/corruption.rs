use std::time::Instant;

use serde::{Deserialize, Serialize};

/// Signals that indicate a particular type of output corruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorruptionSignal {
    /// The model is repeating the same text or pattern.
    Repetition,
    /// The model switched to a script or language it was not prompted for.
    ScriptSwitching,
    /// The structure of the output has broken down (e.g., malformed JSON, XML).
    StructureBreakdown,
    /// The output has lost semantic coherence.
    SemanticCollapse,
    /// The model is no longer following the task instructions.
    TaskIrrelevance,
    /// Non-printing or invalid character sequences dominate the output.
    CharacterClassChaos,
}

impl CorruptionSignal {
    /// Returns a human-readable label for the signal.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Repetition => "repetition",
            Self::ScriptSwitching => "script_switching",
            Self::StructureBreakdown => "structure_breakdown",
            Self::SemanticCollapse => "semantic_collapse",
            Self::TaskIrrelevance => "task_irrelevance",
            Self::CharacterClassChaos => "character_class_chaos",
        }
    }
}

/// A point-in-time snapshot of what was happening when corruption was detected.
///
/// This is captured to aid debugging. Snapshots can be disabled via settings
/// and may contain sensitive data so a redaction flag is included.
#[derive(Debug, Clone)]
pub struct CorruptionSnapshot {
    /// Telemetry ID of the model that produced the output.
    pub model_id: String,
    /// Provider ID (e.g. "openai", "anthropic").
    pub provider: String,
    /// A hash of the prompt that led to the corruption.
    pub prompt_hash: u64,
    /// Last `max_output_bytes` of model output before the trigger.
    pub last_output: String,
    /// Which signals triggered the corruption assessment.
    pub triggered_signals: Vec<CorruptionSignal>,
    /// Confidence level of the corruption assessment (0.0-1.0).
    pub confidence: f32,
    /// When the snapshot was taken.
    pub timestamp: Instant,
}

impl CorruptionSnapshot {
    /// The default maximum number of bytes of output to retain in a snapshot.
    pub const DEFAULT_MAX_OUTPUT_BYTES: usize = 4096;

    /// Create a new snapshot from the given context.
    pub fn new(
        model_id: String,
        provider: String,
        prompt_hash: u64,
        last_output: String,
        triggered_signals: Vec<CorruptionSignal>,
        confidence: f32,
    ) -> Self {
        Self {
            model_id,
            provider,
            prompt_hash,
            last_output,
            triggered_signals,
            confidence,
            timestamp: Instant::now(),
        }
    }

    /// Truncate the captured output to the given byte budget.
    pub fn truncate_output(&mut self, max_bytes: usize) {
        if self.last_output.len() > max_bytes {
            let remainder = self.last_output.len() - max_bytes;
            self.last_output = self.last_output.split_off(remainder);
        }
    }

    /// Redact the output content for privacy-sensitive contexts.
    ///
    /// When redaction is enabled, the captured output is replaced with a
    /// placeholder so no user data leaks through telemetry.
    pub fn redact(&mut self) {
        let byte_len = self.last_output.len();
        self.last_output = format!("[redacted: {} bytes of model output suppressed]", byte_len);
    }
}

/// A telemetry event emitted when corruption is detected and handled.
#[derive(Debug, Clone)]
pub struct CorruptionEvent {
    /// When the event occurred.
    pub timestamp: Instant,
    /// Which layer flagged the corruption (e.g. "output_quality",
    /// "missing_completion_tool", "scope_anomaly", "ast_validation").
    pub layer: &'static str,
    /// Telemetry ID of the model in use at the time.
    pub model_id: String,
    /// Provider ID at the time.
    pub provider: String,
    /// How many times a corruption retry has been attempted.
    pub retry_count: u8,
    /// Whether the corruption was ultimately resolved (e.g. by a model fallback).
    pub resolved: bool,
    /// An optional snapshot of the corrupted output (may be disabled or redacted).
    pub snapshot: Option<CorruptionSnapshot>,
}

impl CorruptionEvent {
    /// Create a new corruption event.
    pub fn new(
        layer: &'static str,
        model_id: String,
        provider: String,
        retry_count: u8,
        resolved: bool,
        snapshot: Option<CorruptionSnapshot>,
    ) -> Self {
        Self {
            timestamp: Instant::now(),
            layer,
            model_id,
            provider,
            retry_count,
            resolved,
            snapshot,
        }
    }
}

/// Settings controlling corruption snapshot behavior.
///
/// Nested under `agent.corruption_defense.snapshots` in the user's settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CorruptionSnapshotSettings {
    /// Whether corruption snapshots are captured at all.
    pub enabled: bool,
    /// Maximum number of output bytes to retain in a snapshot.
    pub max_output_bytes: usize,
    /// Whether to redact the captured output content.
    pub redact: bool,
}

impl CorruptionSnapshotSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn max_output_bytes(&self) -> usize {
        if self.max_output_bytes == 0 {
            CorruptionSnapshot::DEFAULT_MAX_OUTPUT_BYTES
        } else {
            self.max_output_bytes
        }
    }
}

impl Default for CorruptionSnapshotSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            max_output_bytes: CorruptionSnapshot::DEFAULT_MAX_OUTPUT_BYTES,
            redact: true,
        }
    }
}
