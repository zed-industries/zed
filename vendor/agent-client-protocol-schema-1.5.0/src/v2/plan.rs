//! Execution plans for complex tasks that require multiple steps.
//!
//! Plans are strategies that agents share with clients through session updates,
//! providing real-time visibility into their thinking and progress.
//!
//! See: [Agent Plan](https://agentclientprotocol.com/protocol/agent-plan)

use std::{collections::BTreeMap, sync::Arc};

use derive_more::{Display, From};
use schemars::JsonSchema;
use schemars::Schema;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnError, VecSkipError, serde_as, skip_serializing_none};

use super::Meta;
use crate::{IntoOption, SkipListener};

/// Unique identifier for a plan within a session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Display, From)]
#[serde(transparent)]
#[from(forward)]
#[non_exhaustive]
pub struct PlanId(pub Arc<str>);

impl PlanId {
    /// Wraps a protocol string as a typed [`PlanId`].
    #[must_use]
    pub fn new(id: impl Into<Self>) -> Self {
        id.into()
    }
}

/// A content update for a plan identified by ID.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlanUpdate {
    /// The updated plan content.
    pub plan: PlanUpdateContent,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl PlanUpdate {
    /// Builds [`PlanUpdate`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(plan: PlanUpdateContent) -> Self {
        Self { plan, meta: None }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Updated content for a plan.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum PlanUpdateContent {
    /// Structured plan entries.
    Items(PlanItems),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// A URI pointing to a file containing the plan.
    #[cfg(feature = "unstable_plan_operations")]
    File(PlanFile),
    /// **UNSTABLE**
    ///
    /// This capability is not part of the spec yet, and may be removed or changed at any point.
    ///
    /// Raw markdown content for the plan.
    #[cfg(feature = "unstable_plan_operations")]
    Markdown(PlanMarkdown),
    /// Custom or future plan update content.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    ///
    /// Receivers that do not understand this content type should preserve the
    /// raw payload when storing, replaying, proxying, or forwarding plans, and
    /// otherwise ignore it or display it generically.
    #[serde(untagged)]
    Other(OtherPlanUpdateContent),
}

/// Custom or future plan update content payload.
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq, Eq)]
#[schemars(inline)]
#[schemars(transform = other_plan_update_content_schema)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OtherPlanUpdateContent {
    /// Custom or future plan update content type.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(rename = "type")]
    pub type_: String,
    /// The plan ID to update.
    pub plan_id: PlanId,
    /// Additional fields from the unknown plan update content payload.
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl OtherPlanUpdateContent {
    /// Builds [`OtherPlanUpdateContent`] from an unknown discriminator and preserves the remaining extension fields.
    #[must_use]
    pub fn new(
        type_: impl Into<String>,
        plan_id: impl Into<PlanId>,
        mut fields: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        fields.remove("type");
        fields.remove("planId");
        Self {
            type_: type_.into(),
            plan_id: plan_id.into(),
            fields,
        }
    }
}

impl<'de> Deserialize<'de> for OtherPlanUpdateContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut fields = BTreeMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let type_ = fields
            .remove("type")
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;
        let serde_json::Value::String(type_) = type_ else {
            return Err(serde::de::Error::custom("`type` must be a string"));
        };
        let plan_id = fields
            .remove("planId")
            .ok_or_else(|| serde::de::Error::missing_field("planId"))?;
        let serde_json::Value::String(plan_id) = plan_id else {
            return Err(serde::de::Error::custom("`planId` must be a string"));
        };

        if is_known_plan_update_content_type(&type_) {
            return Err(serde::de::Error::custom(format!(
                "known plan update content `{type_}` did not match its schema"
            )));
        }

        Ok(Self {
            type_,
            plan_id: PlanId::new(plan_id),
            fields,
        })
    }
}

fn is_known_plan_update_content_type(type_: &str) -> bool {
    KNOWN_PLAN_UPDATE_CONTENT_TYPES.contains(&type_)
}

fn other_plan_update_content_schema(schema: &mut Schema) {
    super::schema_util::reject_known_string_discriminators(
        schema,
        "type",
        KNOWN_PLAN_UPDATE_CONTENT_TYPES,
    );
}

const KNOWN_PLAN_UPDATE_CONTENT_TYPES: &[&str] = &["items", "file", "markdown"];

impl PlanUpdateContent {
    /// Builds a plan update that replaces the itemized entries for a plan.
    #[must_use]
    pub fn items(plan_id: impl Into<PlanId>, entries: Vec<PlanEntry>) -> Self {
        Self::Items(PlanItems::new(plan_id, entries))
    }

    /// Builds a plan update that points clients at an external plan file URI.
    #[cfg(feature = "unstable_plan_operations")]
    #[must_use]
    pub fn file(plan_id: impl Into<PlanId>, uri: impl Into<String>) -> Self {
        Self::File(PlanFile::new(plan_id, uri))
    }

    /// Builds a plan update whose plan content is inline Markdown.
    #[cfg(feature = "unstable_plan_operations")]
    #[must_use]
    pub fn markdown(plan_id: impl Into<PlanId>, content: impl Into<String>) -> Self {
        Self::Markdown(PlanMarkdown::new(plan_id, content))
    }
}

/// A plan represented as structured entries.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlanItems {
    /// The plan ID to update.
    pub plan_id: PlanId,
    /// The list of tasks to be accomplished.
    ///
    /// When updating an item-based plan, the agent must send a complete list of all entries
    /// with their current status. The client replaces that plan with each update.
    #[serde_as(deserialize_as = "DefaultOnError<VecSkipError<_, SkipListener>>")]
    #[schemars(extend("x-deserialize-default-on-error" = true, "x-deserialize-skip-invalid-items" = true))]
    pub entries: Vec<PlanEntry>,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl PlanItems {
    /// Builds [`PlanItems`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(plan_id: impl Into<PlanId>, entries: Vec<PlanEntry>) -> Self {
        Self {
            plan_id: plan_id.into(),
            entries,
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// A plan represented by a file URI.
#[cfg(feature = "unstable_plan_operations")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlanFile {
    /// The plan ID to update.
    pub plan_id: PlanId,
    /// The URI of the file containing the plan.
    #[schemars(url)]
    pub uri: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

#[cfg(feature = "unstable_plan_operations")]
impl PlanFile {
    /// Builds [`PlanFile`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(plan_id: impl Into<PlanId>, uri: impl Into<String>) -> Self {
        Self {
            plan_id: plan_id.into(),
            uri: uri.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// A plan represented as raw markdown content.
#[cfg(feature = "unstable_plan_operations")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlanMarkdown {
    /// The plan ID to update.
    pub plan_id: PlanId,
    /// Markdown content for the plan.
    pub content: String,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

#[cfg(feature = "unstable_plan_operations")]
impl PlanMarkdown {
    /// Builds [`PlanMarkdown`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(plan_id: impl Into<PlanId>, content: impl Into<String>) -> Self {
        Self {
            plan_id: plan_id.into(),
            content: content.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// **UNSTABLE**
///
/// This capability is not part of the spec yet, and may be removed or changed at any point.
///
/// Removal notice for a plan identified by ID.
#[cfg(feature = "unstable_plan_operations")]
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlanRemoved {
    /// The plan ID to remove.
    pub plan_id: PlanId,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

#[cfg(feature = "unstable_plan_operations")]
impl PlanRemoved {
    /// Builds [`PlanRemoved`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(plan_id: impl Into<PlanId>) -> Self {
        Self {
            plan_id: plan_id.into(),
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// A single entry in the execution plan.
///
/// Represents a task or goal that the assistant intends to accomplish
/// as part of fulfilling the user's request.
/// See protocol docs: [Plan Entries](https://agentclientprotocol.com/protocol/agent-plan#plan-entries)
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlanEntry {
    /// Human-readable description of what this task aims to accomplish.
    pub content: String,
    /// The relative importance of this task.
    /// Used to indicate which tasks are most critical to the overall goal.
    pub priority: PlanEntryPriority,
    /// Current execution status of this task.
    pub status: PlanEntryStatus,
    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[schemars(extend("x-deserialize-default-on-error" = true))]
    #[serde(default)]
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
}

impl PlanEntry {
    /// Builds [`PlanEntry`] with the required fields set; optional fields start unset or empty.
    #[must_use]
    pub fn new(
        content: impl Into<String>,
        priority: PlanEntryPriority,
        status: PlanEntryStatus,
    ) -> Self {
        Self {
            content: content.into(),
            priority,
            status,
            meta: None,
        }
    }

    /// The _meta property is reserved by ACP to allow clients and agents to attach additional
    /// metadata to their interactions. Implementations MUST NOT make assumptions about values at
    /// these keys.
    ///
    /// See protocol docs: [Extensibility](https://agentclientprotocol.com/protocol/extensibility)
    #[must_use]
    pub fn meta(mut self, meta: impl IntoOption<Meta>) -> Self {
        self.meta = meta.into_option();
        self
    }
}

/// Priority levels for plan entries.
///
/// Used to indicate the relative importance or urgency of different
/// tasks in the execution plan.
/// See protocol docs: [Plan Entries](https://agentclientprotocol.com/protocol/agent-plan#plan-entries)
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PlanEntryPriority {
    /// High priority task - critical to the overall goal.
    High,
    /// Medium priority task - important but not critical.
    Medium,
    /// Low priority task - nice to have but not essential.
    Low,
    /// Custom or future plan entry priority.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

/// Status of a plan entry in the execution flow.
///
/// Tracks the lifecycle of each task from planning through completion.
/// See protocol docs: [Plan Entries](https://agentclientprotocol.com/protocol/agent-plan#plan-entries)
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PlanEntryStatus {
    /// The task has not started yet.
    Pending,
    /// The task is currently being worked on.
    InProgress,
    /// The task has been successfully completed.
    Completed,
    /// Custom or future plan entry status.
    ///
    /// Values beginning with `_` are reserved for implementation-specific
    /// extensions. Unknown values that do not begin with `_` are reserved for
    /// future ACP variants.
    #[serde(untagged)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_entry_priority_preserves_unknown_variant() {
        let priority: PlanEntryPriority = serde_json::from_str("\"urgent\"").unwrap();
        assert_eq!(priority, PlanEntryPriority::Other("urgent".to_string()));
        assert_eq!(serde_json::to_value(&priority).unwrap(), "urgent");
    }

    #[test]
    fn plan_entry_status_preserves_unknown_variant() {
        let status: PlanEntryStatus = serde_json::from_str("\"blocked\"").unwrap();
        assert_eq!(status, PlanEntryStatus::Other("blocked".to_string()));
        assert_eq!(serde_json::to_value(&status).unwrap(), "blocked");
    }

    #[test]
    fn plan_update_content_preserves_unknown_variant() {
        let content: PlanUpdateContent = serde_json::from_value(serde_json::json!({
            "type": "_timeline",
            "planId": "plan-1",
            "events": []
        }))
        .unwrap();

        let PlanUpdateContent::Other(unknown) = content else {
            panic!("expected unknown plan update content");
        };

        assert_eq!(unknown.type_, "_timeline");
        assert_eq!(unknown.plan_id.to_string(), "plan-1");
        assert!(!unknown.fields.contains_key("planId"));
        assert_eq!(
            serde_json::to_value(PlanUpdateContent::Other(unknown)).unwrap(),
            serde_json::json!({
                "type": "_timeline",
                "planId": "plan-1",
                "events": []
            })
        );
    }

    #[test]
    fn plan_update_content_does_not_hide_malformed_known_variant() {
        assert!(
            serde_json::from_value::<PlanUpdateContent>(serde_json::json!({
                "type": "items"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<PlanUpdateContent>(serde_json::json!({
                "type": "file",
                "planId": "plan-1"
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<PlanUpdateContent>(serde_json::json!({
                "type": "markdown",
                "planId": "plan-1"
            }))
            .is_err()
        );
    }

    #[test]
    fn plan_update_content_requires_id_for_unknown_variant() {
        assert!(
            serde_json::from_value::<PlanUpdateContent>(serde_json::json!({
                "type": "_timeline"
            }))
            .is_err()
        );
    }
}
