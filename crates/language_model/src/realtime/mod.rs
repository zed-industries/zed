use std::sync::Arc;

use rodio::buffer::SamplesBuffer;

use crate::{
    CompletionRequestStatus, LanguageModelToolResult, LanguageModelToolUse, LanguageModelToolUseId,
    StopReason, TokenUsage,
};

#[derive(Clone, Debug)]
pub enum RealtimeResponse {
    StatusUpdate(CompletionRequestStatus),
    Stop(StopReason),
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking {
        data: String,
    },
    Audio(SamplesBuffer),
    SpeechInterrupt,
    ToolUse(LanguageModelToolUse),
    ToolUseJsonParseError {
        id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        raw_input: Arc<str>,
        json_parse_error: String,
    },
    StartMessage {
        message_id: String,
    },
    UsageUpdate(TokenUsage),
    AudioEnd,
}

#[derive(Clone, Debug)]
pub enum RealtimeRequest {
    Text(String),
    Audio(SamplesBuffer),
    ToolResult(LanguageModelToolResult),
}
