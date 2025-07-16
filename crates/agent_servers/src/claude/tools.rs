use std::path::PathBuf;

use agentic_coding_protocol::{self as acp, PushToolCallParams, ToolCallLocation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::ResultExt;

pub enum ClaudeTool {
    Edit(Option<EditToolParams>),
    ReadFile(Option<ReadToolParams>),
    ListDirectory,
    Glob,
    Grep,
    Terminal,
    Web,
    Todo,
    Subagent,
    Other,
}

impl ClaudeTool {
    pub fn infer(tool_name: &str, input: serde_json::Value) -> Self {
        match tool_name {
            // Known tools
            "mcp__zed__Read" => Self::ReadFile(serde_json::from_value(input).log_err()),
            "mcp__zed__Edit" => Self::Edit(serde_json::from_value(input).log_err()),
            "MultiEdit" => Self::Edit(None),
            "Write" => Self::Edit(None),
            "LS" => Self::ListDirectory,
            "Glob" => Self::Glob,
            "Grep" => Self::Grep,
            "Bash" => Self::Terminal,
            "WebFetch" => Self::Web,
            "WebSearch" => Self::Web,
            "TodoWrite" => Self::Todo,
            "exit_plan_mode" => Self::Todo,
            "Task" => Self::Subagent,
            // Inferred from name
            _ => {
                let tool_name = tool_name.to_lowercase();

                if tool_name.contains("edit") || tool_name.contains("write") {
                    Self::Edit(None)
                } else if tool_name.contains("web") {
                    Self::Web
                } else if tool_name.contains("todo") {
                    Self::Todo
                } else if tool_name.contains("terminal") {
                    Self::Terminal
                } else {
                    Self::Other
                }
            }
        }
    }

    pub fn custom_label(tool_name: &str) -> Option<String> {
        if let Some(server_tool) = tool_name.strip_prefix("mcp__") {
            let mut split = server_tool.split("__");
            let server = split.next()?;
            let tool_name = split.next()?;
            Some(format!("{}: {}", server, tool_name))
        } else {
            None
        }
    }

    pub fn tool_call_params(tool_name: String, input: serde_json::Value) -> PushToolCallParams {
        let formatted = serde_json::to_string_pretty(&input).unwrap();
        let markdown = format!("```json\n{}\n```", formatted);
        let inferred_tool = Self::infer(&tool_name, input);

        PushToolCallParams {
            label: Self::custom_label(&tool_name).unwrap_or(tool_name),
            icon: inferred_tool.icon(),
            content: Some(acp::ToolCallContent::Markdown { markdown }),
            locations: inferred_tool.locations(),
        }
    }

    pub fn icon(&self) -> acp::Icon {
        match self {
            Self::Edit(_) => acp::Icon::Pencil,
            Self::ReadFile(_) => acp::Icon::FileSearch,
            Self::ListDirectory => acp::Icon::Folder,
            Self::Glob => acp::Icon::FileSearch,
            Self::Grep => acp::Icon::Regex,
            Self::Terminal => acp::Icon::Terminal,
            Self::Web => acp::Icon::Globe,
            Self::Todo => acp::Icon::LightBulb,
            Self::Subagent => acp::Icon::Hammer,
            Self::Other => acp::Icon::Hammer,
        }
    }

    pub fn confirmation(&self, description: Option<String>) -> acp::ToolCallConfirmation {
        match &self {
            Self::Edit(_) => acp::ToolCallConfirmation::Edit { description },
            Self::Web => acp::ToolCallConfirmation::Fetch {
                urls: vec![],
                description,
            },
            Self::Terminal
            | Self::ListDirectory
            | Self::Glob
            | Self::Grep
            | Self::Todo
            | Self::Subagent
            | Self::ReadFile(_)
            | Self::Other => acp::ToolCallConfirmation::Other {
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
            | Self::ListDirectory
            | Self::Glob
            | Self::Grep
            | Self::Terminal
            | Self::Web
            | Self::Todo
            | Self::Subagent
            | Self::Other => vec![],
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
    /// Which line to start reading from. Omit to start from the begining.
    pub offset: Option<u32>,
    /// How many lines to read. Omit for the whole file.
    pub limit: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadToolResponse {
    pub content: String,
}
