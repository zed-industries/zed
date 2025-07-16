use std::path::PathBuf;

use agentic_coding_protocol::{self as acp, PushToolCallParams, ToolCallLocation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::ResultExt;

pub enum ClaudeTool {
    Edit(Option<EditToolParams>),
    ReadFile(Option<ReadToolParams>),
    Ls,
    Glob,
    Grep,
    Terminal(Option<BashToolParams>),
    WebFetch,
    WebSearch,
    TodoWrite,
    ExitPlanMode,
    Other {
        name: String,
        input: serde_json::Value,
    },
}

impl ClaudeTool {
    pub fn infer(tool_name: &str, input: serde_json::Value) -> Self {
        match tool_name {
            // Known tools
            "mcp__zed__Read" => Self::ReadFile(serde_json::from_value(input).log_err()),
            "mcp__zed__Edit" => Self::Edit(serde_json::from_value(input).log_err()),
            "MultiEdit" => Self::Edit(None),
            "Write" => Self::Edit(None),
            "LS" => Self::Ls,
            "Glob" => Self::Glob,
            "Grep" => Self::Grep,
            "Bash" => Self::Terminal(serde_json::from_value(input).log_err()),
            "WebFetch" => Self::WebFetch,
            "WebSearch" => Self::WebSearch,
            "TodoWrite" => Self::TodoWrite,
            "exit_plan_mode" => Self::ExitPlanMode,
            "Task" => Self::ExitPlanMode,
            // Inferred from name
            _ => {
                let tool_name = tool_name.to_lowercase();

                if tool_name.contains("edit") || tool_name.contains("write") {
                    Self::Edit(None)
                } else if tool_name.contains("terminal") {
                    Self::Terminal(None)
                } else {
                    Self::Other {
                        name: tool_name.to_string(),
                        input,
                    }
                }
            }
        }
    }

    pub fn label(&self) -> String {
        match &self {
            ClaudeTool::Terminal(Some(params)) => format!("`{}`", params.command),
            ClaudeTool::Terminal(None) => "Terminal".into(),
            ClaudeTool::ReadFile(_) => "Read File".into(),
            ClaudeTool::Ls => "List Directory".into(),
            ClaudeTool::Edit(_) => "Edit".into(),
            ClaudeTool::Glob => "Glob".into(),
            ClaudeTool::Grep => "Grep".into(),
            ClaudeTool::WebFetch => "Fetch".into(),
            ClaudeTool::WebSearch => "Web Search".into(),
            ClaudeTool::TodoWrite => "Update TODOs".into(),
            ClaudeTool::ExitPlanMode => "Exit Plan Mode".into(),
            ClaudeTool::Other { name, .. } => name.clone(),
        }
    }

    pub fn content(&self) -> Option<acp::ToolCallContent> {
        match &self {
            ClaudeTool::Other { input, .. } => Some(acp::ToolCallContent::Markdown {
                markdown: format!(
                    "```json\n{}```",
                    serde_json::to_string_pretty(&input).unwrap_or("{}".to_string())
                ),
            }),
            _ => None,
        }
    }

    pub fn icon(&self) -> acp::Icon {
        match self {
            Self::Edit(_) => acp::Icon::Pencil,
            Self::ReadFile(_) => acp::Icon::FileSearch,
            Self::Ls => acp::Icon::Folder,
            Self::Glob => acp::Icon::FileSearch,
            Self::Grep => acp::Icon::Regex,
            Self::Terminal(_) => acp::Icon::Terminal,
            Self::WebSearch => acp::Icon::Globe,
            Self::WebFetch => acp::Icon::Globe,
            Self::TodoWrite => acp::Icon::LightBulb,
            Self::ExitPlanMode => acp::Icon::Hammer,
            Self::Other { .. } => acp::Icon::Hammer,
        }
    }

    pub fn confirmation(&self, description: Option<String>) -> acp::ToolCallConfirmation {
        match &self {
            Self::Edit(_) => acp::ToolCallConfirmation::Edit { description },
            Self::WebFetch => acp::ToolCallConfirmation::Fetch {
                urls: vec![],
                description,
            },
            Self::Terminal(Some(BashToolParams {
                description,
                command,
            })) => acp::ToolCallConfirmation::Execute {
                command: command.clone(),
                root_command: command.clone(),
                description: Some(description.clone()),
            },
            Self::Terminal(None)
            | Self::Ls
            | Self::Glob
            | Self::Grep
            | Self::TodoWrite
            | Self::WebSearch
            | Self::ExitPlanMode
            | Self::ReadFile(_)
            | Self::Other { .. } => acp::ToolCallConfirmation::Other {
                description: description.unwrap_or("".to_string()),
            },
        }
    }

    pub fn locations(&self) -> Vec<acp::ToolCallLocation> {
        match &self {
            Self::Edit(Some(EditToolParams { abs_path, .. })) => vec![ToolCallLocation {
                path: abs_path.clone(),
                line: None,
            }],
            Self::ReadFile(Some(ReadToolParams {
                abs_path, offset, ..
            })) => vec![ToolCallLocation {
                path: abs_path.clone(),
                line: *offset,
            }],
            Self::Edit(None)
            | Self::ReadFile(None)
            | Self::Ls
            | Self::Glob
            | Self::Grep
            | Self::Terminal(_)
            | Self::WebFetch
            | Self::WebSearch
            | Self::TodoWrite
            | Self::ExitPlanMode
            | Self::Other { .. } => vec![],
        }
    }

    pub fn as_acp(&self) -> PushToolCallParams {
        PushToolCallParams {
            label: self.label(),
            content: self.content(),
            icon: self.icon(),
            locations: self.locations(),
        }
    }
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct EditToolParams {
    /// The absolute path to the file to read.
    pub abs_path: PathBuf,
    /// The old text to replace (must be unique in the file)
    pub old_text: String,
    /// The new text.
    pub new_text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditToolResponse;

#[derive(Deserialize, JsonSchema, Debug)]
pub struct ReadToolParams {
    /// The absolute path to the file to read.
    pub abs_path: PathBuf,
    /// Which line to start reading from. Omit to start from the beginning.
    pub offset: Option<u32>,
    /// How many lines to read. Omit for the whole file.
    pub limit: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadToolResponse {
    pub content: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct BashToolParams {
    pub description: String,
    pub command: String,
}
