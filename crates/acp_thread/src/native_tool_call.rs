use agent_client_protocol::schema::{IntoOption, v2 as acp_v2};

use crate::ToolCallLocation;

/// Application-owned tool data, not an ACP wire update.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeToolCall {
    pub tool_call_id: acp_v2::ToolCallId,
    pub title: String,
    pub name: Option<String>,
    pub kind: acp_v2::ToolKind,
    pub status: acp_v2::ToolCallStatus,
    pub content: Vec<acp_v2::ToolCallContent>,
    pub locations: Vec<ToolCallLocation>,
    pub raw_input: Option<serde_json::Value>,
    pub raw_output: Option<serde_json::Value>,
    pub meta: Option<acp_v2::Meta>,
}

impl NativeToolCall {
    pub fn new(tool_call_id: impl Into<acp_v2::ToolCallId>, title: impl Into<String>) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            title: title.into(),
            name: None,
            kind: acp_v2::ToolKind::Other,
            status: acp_v2::ToolCallStatus::Pending,
            content: Vec::new(),
            locations: Vec::new(),
            raw_input: None,
            raw_output: None,
            meta: None,
        }
    }

    pub fn name(mut self, name: impl IntoOption<String>) -> Self {
        self.name = name.into_option();
        self
    }

    pub fn kind(mut self, kind: acp_v2::ToolKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn status(mut self, status: acp_v2::ToolCallStatus) -> Self {
        self.status = status;
        self
    }

    pub fn content(mut self, content: impl Into<Vec<acp_v2::ToolCallContent>>) -> Self {
        self.content = content.into();
        self
    }

    pub fn locations(mut self, locations: impl Into<Vec<ToolCallLocation>>) -> Self {
        self.locations = locations.into();
        self
    }

    pub fn raw_input(mut self, raw_input: impl IntoOption<serde_json::Value>) -> Self {
        self.raw_input = raw_input.into_option();
        self
    }

    pub fn raw_output(mut self, raw_output: impl IntoOption<serde_json::Value>) -> Self {
        self.raw_output = raw_output.into_option();
        self
    }

    pub fn meta(mut self, meta: impl IntoOption<acp_v2::Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// `None` preserves a field; empty collections replace it with an empty collection.
/// A raw JSON null is data, not an instruction to clear a field.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NativeToolCallUpdateFields {
    pub kind: Option<acp_v2::ToolKind>,
    pub status: Option<acp_v2::ToolCallStatus>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub content: Option<Vec<acp_v2::ToolCallContent>>,
    pub locations: Option<Vec<ToolCallLocation>>,
    pub raw_input: Option<serde_json::Value>,
    pub raw_output: Option<serde_json::Value>,
}

impl NativeToolCallUpdateFields {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn kind(mut self, kind: impl IntoOption<acp_v2::ToolKind>) -> Self {
        self.kind = kind.into_option();
        self
    }

    pub fn status(mut self, status: impl IntoOption<acp_v2::ToolCallStatus>) -> Self {
        self.status = status.into_option();
        self
    }

    pub fn title(mut self, title: impl IntoOption<String>) -> Self {
        self.title = title.into_option();
        self
    }

    pub fn name(mut self, name: impl IntoOption<String>) -> Self {
        self.name = name.into_option();
        self
    }

    pub fn content(mut self, content: impl IntoOption<Vec<acp_v2::ToolCallContent>>) -> Self {
        self.content = content.into_option();
        self
    }

    pub fn locations(mut self, locations: impl IntoOption<Vec<ToolCallLocation>>) -> Self {
        self.locations = locations.into_option();
        self
    }

    pub fn raw_input(mut self, raw_input: impl IntoOption<serde_json::Value>) -> Self {
        self.raw_input = raw_input.into_option();
        self
    }

    pub fn raw_output(mut self, raw_output: impl IntoOption<serde_json::Value>) -> Self {
        self.raw_output = raw_output.into_option();
        self
    }
}

/// Native metadata follows the legacy hint policy: omission never clears prior hints.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeToolCallUpdate {
    pub tool_call_id: acp_v2::ToolCallId,
    pub fields: NativeToolCallUpdateFields,
    pub meta: Option<acp_v2::Meta>,
}

impl NativeToolCallUpdate {
    pub fn new(
        tool_call_id: impl Into<acp_v2::ToolCallId>,
        fields: NativeToolCallUpdateFields,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            fields,
            meta: None,
        }
    }

    pub fn meta(mut self, meta: impl IntoOption<acp_v2::Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

impl From<NativeToolCall> for NativeToolCallUpdate {
    fn from(call: NativeToolCall) -> Self {
        Self {
            tool_call_id: call.tool_call_id,
            fields: NativeToolCallUpdateFields {
                kind: Some(call.kind),
                status: Some(call.status),
                title: Some(call.title),
                name: call.name,
                content: Some(call.content),
                locations: Some(call.locations),
                raw_input: call.raw_input,
                raw_output: call.raw_output,
            },
            meta: call.meta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_defaults_are_explicit_updates() {
        let call = NativeToolCall::new("native", "Native tool");
        let update = NativeToolCallUpdate::from(call);
        assert_eq!(update.fields.kind, Some(acp_v2::ToolKind::Other));
        assert_eq!(update.fields.status, Some(acp_v2::ToolCallStatus::Pending));
        assert_eq!(update.fields.title.as_deref(), Some("Native tool"));
        assert_eq!(update.fields.content, Some(Vec::new()));
        assert_eq!(update.fields.locations, Some(Vec::new()));
        assert_eq!(update.fields.name, None);
        assert_eq!(update.fields.raw_input, None);
        assert_eq!(update.fields.raw_output, None);
        assert_eq!(update.meta, None);
        assert_ne!(update.fields, NativeToolCallUpdateFields::new());
    }

    #[test]
    fn native_setters_distinguish_omission_empty_and_json_null() {
        let fields = NativeToolCallUpdateFields::new()
            .kind(None)
            .status(None)
            .title(None::<String>)
            .name(None::<String>)
            .content(None)
            .locations(None)
            .raw_input(None)
            .raw_output(None);
        assert_eq!(fields, NativeToolCallUpdateFields::default());
        let fields = fields
            .content(Vec::new())
            .locations(Vec::new())
            .raw_input(serde_json::Value::Null)
            .raw_output(serde_json::Value::Null);
        assert_eq!(fields.content, Some(Vec::new()));
        assert_eq!(fields.locations, Some(Vec::new()));
        assert_eq!(fields.raw_input, Some(serde_json::Value::Null));
        assert_eq!(fields.raw_output, Some(serde_json::Value::Null));
    }
}
