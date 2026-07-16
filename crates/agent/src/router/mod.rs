//! Multi-agent router — inspects user prompts and dispatches to the
//! most appropriate model for the task type.
//!
//! ## How it works
//!
//! 1. Classify the user's prompt into a TaskType (Edit / Research / Terminal / General)
//! 2. Select the configured model profile for that task
//! 3. Spawn a sub-agent with the right model + tools
//! 4. Merge results back into the main thread
//!
//! The router lives ABOVE the Ask/Write/Minimal profiles. When routing
//! is enabled, the profile's model selection and tool allowlist are
//! overridden by the matched route profile.

use std::sync::Arc;

use anyhow::Result;
use gpui::{App, SharedString, Task};
use serde::{Deserialize, Serialize};

pub mod classifier;
pub mod dispatch;

pub use classifier::*;
pub use dispatch::*;

/// The type of task the router identifies from the user's prompt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    /// Code editing, generation, refactoring
    Edit,
    /// Web search, documentation, research
    Research,
    /// Terminal commands, build, test, deploy
    Terminal,
    /// Architecture, planning, task decomposition
    Planning,
    /// Image analysis, screenshots, visual UI
    Vision,
    /// Code review, audit, verify changes
    Review,
    /// General Q&A, planning, explanation
    General,
}

impl TaskType {
    pub fn label(&self) -> &'static str {
        match self {
            TaskType::Edit => "Edit",
            TaskType::Research => "Research",
            TaskType::Terminal => "Terminal",
            TaskType::Planning => "Planning",
            TaskType::Vision => "Vision",
            TaskType::Review => "Review",
            TaskType::General => "General",
        }
    }
}

/// A model profile for a specific task type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProfile {
    /// Display name (e.g. "Strong Coder")
    pub name: String,
    /// Provider ID (e.g. "anthropic", "openrouter", "google")
    pub provider: String,
    /// Model name (e.g. "claude-sonnet-4", "gemini-2.0-flash")
    pub model: String,
    /// Tool allowlist — empty means "all available tools"
    #[serde(default)]
    pub tools: Vec<String>,
}

impl Default for ModelProfile {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            provider: String::new(),
            model: String::new(),
            tools: vec![],
        }
    }
}

/// Router configuration, settable in `settings.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    /// Master switch (default: false — opt-in)
    pub enabled: bool,
    /// Model profiles per task type
    pub profiles: RoutingProfiles,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            profiles: RoutingProfiles::default(),
        }
    }
}

/// Per-task-type model profiles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingProfiles {
    pub edit: ModelProfile,
    pub research: ModelProfile,
    pub terminal: ModelProfile,
    pub planning: ModelProfile,
    pub vision: ModelProfile,
    pub review: ModelProfile,
    pub general: ModelProfile,
}

impl Default for RoutingProfiles {
    fn default() -> Self {
        Self {
            edit: ModelProfile {
                name: "Edit".into(),
                provider: "anthropic".into(),
                model: "claude-sonnet-4".into(),
                tools: vec![],
            },
            research: ModelProfile {
                name: "Research".into(),
                provider: "google".into(),
                model: "gemini-2.0-flash".into(),
                tools: vec!["search_web".into(), "fetch".into(), "read_file".into(), "grep".into()],
            },
            terminal: ModelProfile {
                name: "Terminal".into(),
                provider: "openrouter".into(),
                model: "deepseek/deepseek-chat".into(),
                tools: vec!["terminal".into(), "read_file".into()],
            },
            planning: ModelProfile {
                name: "Planning".into(),
                provider: "anthropic".into(),
                model: "claude-sonnet-4".into(),
                tools: vec![],
            },
            vision: ModelProfile {
                name: "Vision".into(),
                provider: "google".into(),
                model: "gemini-2.0-flash".into(),
                tools: vec!["read_file".into(), "fetch".into()],
            },
            review: ModelProfile {
                name: "Review".into(),
                provider: "anthropic".into(),
                model: "claude-sonnet-4".into(),
                tools: vec!["read_file".into(), "grep".into(), "diagnostics".into()],
            },
            general: ModelProfile {
                name: "General".into(),
                provider: String::new(),
                model: String::new(),
                tools: vec![],
            },
        }
    }
}

/// The router instance. Created per-thread with the current config.
pub struct Router {
    config: RouterConfig,
}

impl Router {
    pub fn new(config: RouterConfig) -> Self {
        Self { config }
    }

    /// Get the model profile for the given task type.
    pub fn profile_for(&self, task_type: TaskType) -> &ModelProfile {
        match task_type {
            TaskType::Edit => &self.config.profiles.edit,
            TaskType::Research => &self.config.profiles.research,
            TaskType::Terminal => &self.config.profiles.terminal,
            TaskType::Planning => &self.config.profiles.planning,
            TaskType::Vision => &self.config.profiles.vision,
            TaskType::Review => &self.config.profiles.review,
            TaskType::General => &self.config.profiles.general,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}
