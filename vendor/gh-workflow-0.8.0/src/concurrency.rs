use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::expression::Expression;

/// Represents concurrency settings for workflows.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Concurrency {
    /// The group name for concurrency.
    pub group: String,

    /// Whether to cancel in-progress jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_in_progress: Option<bool>,

    /// The limit on concurrent jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl Concurrency {
    pub fn new(group: impl Into<Expression>) -> Self {
        let expr: Expression = group.into();
        Self { group: expr.0, ..Default::default() }
    }
}
