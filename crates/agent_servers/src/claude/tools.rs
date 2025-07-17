use std::path::PathBuf;

use agentic_coding_protocol::{self as acp, PushToolCallParams, ToolCallLocation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::ResultExt;

pub enum ClaudeTool {
    // Task,
    Edit(Option<EditToolParams>),
    ReadFile(Option<ReadToolParams>),
    Write(Option<WriteToolParams>),
    Ls(Option<LsToolParams>),
    Glob(Option<GlobToolParams>),
    Grep(Option<GrepToolParams>),
    Terminal(Option<BashToolParams>),
    WebFetch(Option<WebFetchToolParams>),
    WebSearch(Option<WebSearchToolParams>),
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
            "Write" => Self::Write(serde_json::from_value(input).log_err()),
            "LS" => Self::Ls(serde_json::from_value(input).log_err()),
            "Glob" => Self::Glob(serde_json::from_value(input).log_err()),
            "Grep" => Self::Grep(serde_json::from_value(input).log_err()),
            "Bash" => Self::Terminal(serde_json::from_value(input).log_err()),
            "WebFetch" => Self::WebFetch(serde_json::from_value(input).log_err()),
            "WebSearch" => Self::WebSearch(serde_json::from_value(input).log_err()),
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
            ClaudeTool::Ls(Some(params)) => {
                format!("List Directory {}", params.path.to_string_lossy())
            }
            ClaudeTool::Ls(None) => "List Directory".into(),
            ClaudeTool::Edit(Some(params)) => {
                format!("Edit {}", params.abs_path.to_string_lossy())
            }
            ClaudeTool::Edit(None) => "Edit".into(),
            ClaudeTool::Write(Some(params)) => {
                format!("Write {}", params.file_path.to_string_lossy())
            }
            ClaudeTool::Write(None) => "Write".into(),
            ClaudeTool::Glob(Some(GlobToolParams { path, pattern })) => {
                if let Some(path) = path {
                    format!("Glob {}{pattern}", path.to_string_lossy())
                } else {
                    format!("Glob {pattern}")
                }
            }
            ClaudeTool::Glob(None) => "Glob".into(),
            ClaudeTool::Grep(Some(params)) => params.to_string(),
            ClaudeTool::Grep(None) => "Grep".into(),
            ClaudeTool::WebFetch(Some(params)) => format!("Fetch {}", params.url),
            ClaudeTool::WebFetch(None) => "Fetch".into(),
            ClaudeTool::WebSearch(Some(params)) => format!("Web Seach: {}", params),
            ClaudeTool::WebSearch(None) => "Web Search".into(),
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
            Self::Write(_) => acp::Icon::Pencil,
            Self::ReadFile(_) => acp::Icon::FileSearch,
            Self::Ls(_) => acp::Icon::Folder,
            Self::Glob(_) => acp::Icon::FileSearch,
            Self::Grep(_) => acp::Icon::Regex,
            Self::Terminal(_) => acp::Icon::Terminal,
            Self::WebSearch(_) => acp::Icon::Globe,
            Self::WebFetch(_) => acp::Icon::Globe,
            Self::TodoWrite => acp::Icon::LightBulb,
            Self::ExitPlanMode => acp::Icon::Hammer,
            Self::Other { .. } => acp::Icon::Hammer,
        }
    }

    pub fn confirmation(&self, description: Option<String>) -> acp::ToolCallConfirmation {
        match &self {
            Self::Edit(_) | Self::Write(_) => acp::ToolCallConfirmation::Edit { description },
            Self::WebFetch(params) => acp::ToolCallConfirmation::Fetch {
                urls: params
                    .as_ref()
                    .map(|p| vec![p.url.clone()])
                    .unwrap_or_default(),
                description,
            },
            Self::Terminal(Some(BashToolParams {
                description,
                command,
                ..
            })) => acp::ToolCallConfirmation::Execute {
                command: command.clone(),
                root_command: command.clone(),
                description: description.clone(),
            },
            Self::Terminal(None)
            | Self::Ls(_)
            | Self::Glob(_)
            | Self::Grep(_)
            | Self::TodoWrite
            | Self::WebSearch(_)
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
            Self::Write(Some(WriteToolParams { file_path, .. })) => vec![ToolCallLocation {
                path: file_path.clone(),
                line: None,
            }],
            Self::ReadFile(Some(ReadToolParams {
                abs_path, offset, ..
            })) => vec![ToolCallLocation {
                path: abs_path.clone(),
                line: *offset,
            }],
            Self::Glob(Some(GlobToolParams {
                path: Some(path), ..
            })) => vec![ToolCallLocation {
                path: path.clone(),
                line: None,
            }],
            Self::Ls(Some(LsToolParams { path, .. })) => vec![ToolCallLocation {
                path: path.clone(),
                line: None,
            }],
            Self::Grep(Some(GrepToolParams {
                path: Some(path), ..
            })) => vec![ToolCallLocation {
                path: PathBuf::from(path),
                line: None,
            }],
            Self::Edit(None)
            | Self::Write(None)
            | Self::ReadFile(None)
            | Self::Ls(None)
            | Self::Glob(_)
            | Self::Grep(_)
            | Self::Terminal(_)
            | Self::WebFetch(_)
            | Self::WebSearch(_)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// How many lines to read. Omit for the whole file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadToolResponse {
    pub content: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct WriteToolParams {
    /// Absolute path for new file
    pub file_path: PathBuf,
    /// File content
    pub content: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct BashToolParams {
    /// Shell command to execute
    pub command: String,
    /// 5-10 word description of what command does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Timeout in ms (max 600000ms/10min, default 120000ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GlobToolParams {
    /// Glob pattern like **/*.js or src/**/*.ts
    pub pattern: String,
    /// Directory to search in (omit for current directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct LsToolParams {
    /// Absolute path to directory
    pub path: PathBuf,
    /// Array of glob patterns to ignore
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ignore: Vec<String>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GrepToolParams {
    /// Regex pattern to search for
    pub pattern: String,
    /// File/directory to search (defaults to current directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// "content" (shows lines), "files_with_matches" (default), "count"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_mode: Option<GrepOutputMode>,
    /// Filter files with glob pattern like "*.js"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glob: Option<String>,
    /// File type filter like "js", "py", "rust"
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// Case insensitive search
    #[serde(rename = "-i", default, skip_serializing_if = "is_false")]
    pub case_insensitive: bool,
    /// Show line numbers (content mode only)
    #[serde(rename = "-n", default, skip_serializing_if = "is_false")]
    pub line_numbers: bool,
    /// Lines after match (content mode only)
    #[serde(rename = "-A", skip_serializing_if = "Option::is_none")]
    pub after_context: Option<u32>,
    /// Lines before match (content mode only)
    #[serde(rename = "-B", skip_serializing_if = "Option::is_none")]
    pub before_context: Option<u32>,
    /// Lines before and after match (content mode only)
    #[serde(rename = "-C", skip_serializing_if = "Option::is_none")]
    pub context: Option<u32>,
    /// Enable multiline/cross-line matching
    #[serde(default, skip_serializing_if = "is_false")]
    pub multiline: bool,
    /// Limit output to first N results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_limit: Option<u32>,
}

impl std::fmt::Display for GrepToolParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "grep")?;

        // Boolean flags
        if self.case_insensitive {
            write!(f, " -i")?;
        }
        if self.line_numbers {
            write!(f, " -n")?;
        }

        // Context options
        if let Some(after) = self.after_context {
            write!(f, " -A {}", after)?;
        }
        if let Some(before) = self.before_context {
            write!(f, " -B {}", before)?;
        }
        if let Some(context) = self.context {
            write!(f, " -C {}", context)?;
        }

        // Output mode
        if let Some(mode) = &self.output_mode {
            match mode {
                GrepOutputMode::FilesWithMatches => write!(f, " -l")?,
                GrepOutputMode::Count => write!(f, " -c")?,
                GrepOutputMode::Content => {} // Default mode
            }
        }

        // Head limit
        if let Some(limit) = self.head_limit {
            write!(f, " | head -{}", limit)?;
        }

        // Glob pattern
        if let Some(glob) = &self.glob {
            write!(f, " --include=\"{}\"", glob)?;
        }

        // File type
        if let Some(file_type) = &self.file_type {
            write!(f, " --type={}", file_type)?;
        }

        // Multiline
        if self.multiline {
            write!(f, " -P")?; // Perl-compatible regex for multiline
        }

        // Pattern (escaped if contains special characters)
        write!(f, " \"{}\"", self.pattern)?;

        // Path
        if let Some(path) = &self.path {
            write!(f, " {}", path)?;
        }

        Ok(())
    }
}

fn is_false(v: &bool) -> bool {
    !*v
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrepOutputMode {
    Content,
    FilesWithMatches,
    Count,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct WebFetchToolParams {
    /// Valid URL to fetch
    #[serde(rename = "url")]
    pub url: String,
    /// What to extract from content
    pub prompt: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct WebSearchToolParams {
    /// Search query (min 2 chars)
    pub query: String,
    /// Only include these domains
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_domains: Vec<String>,
    /// Exclude these domains
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_domains: Vec<String>,
}

impl std::fmt::Display for WebSearchToolParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.query)?;

        if !self.allowed_domains.is_empty() {
            write!(f, " (allowed: {})", self.allowed_domains.join(", "))?;
        }

        if !self.blocked_domains.is_empty() {
            write!(f, " (blocked: {})", self.blocked_domains.join(", "))?;
        }

        Ok(())
    }
}
