//!
//! Default settings types for GitHub workflows.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::concurrency::Concurrency;

/// Represents default settings for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Defaults {
    /// Default settings for running jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunDefaults>,

    /// Default retry settings for jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryDefaults>,

    /// Default concurrency settings for jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
}

/// Represents default settings for running commands.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct RunDefaults {
    /// The shell to use for running commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    /// The working directory for running commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// Represents default settings for retries.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RetryDefaults {
    /// The maximum number of retry attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

/// Represents a strategy for retrying jobs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RetryStrategy {
    /// The maximum number of retry attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}
