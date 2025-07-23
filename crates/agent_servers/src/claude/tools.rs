use std::path::PathBuf;

use agentic_coding_protocol::{self as acp, PushToolCallParams, ToolCallLocation};
use itertools::Itertools;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::ResultExt;

pub enum ClaudeTool {
    Task(Option<TaskToolParams>),
    NotebookRead(Option<NotebookReadToolParams>),
    NotebookEdit(Option<NotebookEditToolParams>),
    Edit(Option<EditToolParams>),
    MultiEdit(Option<MultiEditToolParams>),
    ReadFile(Option<ReadToolParams>),
    Write(Option<WriteToolParams>),
    Ls(Option<LsToolParams>),
    Glob(Option<GlobToolParams>),
    Grep(Option<GrepToolParams>),
    Terminal(Option<BashToolParams>),
    WebFetch(Option<WebFetchToolParams>),
    WebSearch(Option<WebSearchToolParams>),
    TodoWrite(Option<TodoWriteToolParams>),
    ExitPlanMode(Option<ExitPlanModeToolParams>),
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
            "MultiEdit" => Self::MultiEdit(serde_json::from_value(input).log_err()),
            "Write" => Self::Write(serde_json::from_value(input).log_err()),
            "LS" => Self::Ls(serde_json::from_value(input).log_err()),
            "Glob" => Self::Glob(serde_json::from_value(input).log_err()),
            "Grep" => Self::Grep(serde_json::from_value(input).log_err()),
            "Bash" => Self::Terminal(serde_json::from_value(input).log_err()),
            "WebFetch" => Self::WebFetch(serde_json::from_value(input).log_err()),
            "WebSearch" => Self::WebSearch(serde_json::from_value(input).log_err()),
            "TodoWrite" => Self::TodoWrite(serde_json::from_value(input).log_err()),
            "exit_plan_mode" => Self::ExitPlanMode(serde_json::from_value(input).log_err()),
            "Task" => Self::Task(serde_json::from_value(input).log_err()),
            "NotebookRead" => Self::NotebookRead(serde_json::from_value(input).log_err()),
            "NotebookEdit" => Self::NotebookEdit(serde_json::from_value(input).log_err()),
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
            Self::Task(Some(params)) => params.description.clone(),
            Self::Task(None) => "Task".into(),
            Self::NotebookRead(Some(params)) => {
                format!("Read Notebook {}", params.notebook_path.display())
            }
            Self::NotebookRead(None) => "Read Notebook".into(),
            Self::NotebookEdit(Some(params)) => {
                format!("Edit Notebook {}", params.notebook_path.display())
            }
            Self::NotebookEdit(None) => "Edit Notebook".into(),
            Self::Terminal(Some(params)) => format!("`{}`", params.command),
            Self::Terminal(None) => "Terminal".into(),
            Self::ReadFile(_) => "Read File".into(),
            Self::Ls(Some(params)) => {
                format!("List Directory {}", params.path.display())
            }
            Self::Ls(None) => "List Directory".into(),
            Self::Edit(Some(params)) => {
                format!("Edit {}", params.abs_path.display())
            }
            Self::Edit(None) => "Edit".into(),
            Self::MultiEdit(Some(params)) => {
                format!("Multi Edit {}", params.file_path.display())
            }
            Self::MultiEdit(None) => "Multi Edit".into(),
            Self::Write(Some(params)) => {
                format!("Write {}", params.file_path.display())
            }
            Self::Write(None) => "Write".into(),
            Self::Glob(Some(params)) => {
                format!("Glob `{params}`")
            }
            Self::Glob(None) => "Glob".into(),
            Self::Grep(Some(params)) => format!("`{params}`"),
            Self::Grep(None) => "Grep".into(),
            Self::WebFetch(Some(params)) => format!("Fetch {}", params.url),
            Self::WebFetch(None) => "Fetch".into(),
            Self::WebSearch(Some(params)) => format!("Web Search: {}", params),
            Self::WebSearch(None) => "Web Search".into(),
            Self::TodoWrite(Some(params)) => format!(
                "Update TODOs: {}",
                params.todos.iter().map(|todo| &todo.content).join(", ")
            ),
            Self::TodoWrite(None) => "Update TODOs".into(),
            Self::ExitPlanMode(_) => "Exit Plan Mode".into(),
            Self::Other { name, .. } => name.clone(),
        }
    }

    pub fn content(&self) -> Option<acp::ToolCallContent> {
        match &self {
            Self::Other { input, .. } => Some(acp::ToolCallContent::Markdown {
                markdown: format!(
                    "```json\n{}```",
                    serde_json::to_string_pretty(&input).unwrap_or("{}".to_string())
                ),
            }),
            Self::Task(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.prompt.clone(),
            }),
            Self::NotebookRead(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.notebook_path.display().to_string(),
            }),
            Self::NotebookEdit(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.new_source.clone(),
            }),
            Self::Terminal(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: format!(
                    "`{}`\n\n{}",
                    params.command,
                    params.description.as_deref().unwrap_or_default()
                ),
            }),
            Self::ReadFile(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.abs_path.display().to_string(),
            }),
            Self::Ls(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.path.display().to_string(),
            }),
            Self::Glob(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.to_string(),
            }),
            Self::Grep(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: format!("`{params}`"),
            }),
            Self::WebFetch(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.prompt.clone(),
            }),
            Self::WebSearch(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.to_string(),
            }),
            Self::TodoWrite(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params
                    .todos
                    .iter()
                    .map(|todo| {
                        format!(
                            "- {} {}: {}",
                            match todo.status {
                                TodoStatus::Completed => "✅",
                                TodoStatus::InProgress => "🚧",
                                TodoStatus::Pending => "⬜",
                            },
                            todo.priority,
                            todo.content
                        )
                    })
                    .join("\n"),
            }),
            Self::ExitPlanMode(Some(params)) => Some(acp::ToolCallContent::Markdown {
                markdown: params.plan.clone(),
            }),
            Self::Edit(Some(params)) => Some(acp::ToolCallContent::Diff {
                diff: acp::Diff {
                    path: params.abs_path.clone(),
                    old_text: Some(params.old_text.clone()),
                    new_text: params.new_text.clone(),
                },
            }),
            Self::Write(Some(params)) => Some(acp::ToolCallContent::Diff {
                diff: acp::Diff {
                    path: params.file_path.clone(),
                    old_text: None,
                    new_text: params.content.clone(),
                },
            }),
            Self::MultiEdit(Some(params)) => {
                // todo: show multiple edits in a multibuffer?
                params.edits.first().map(|edit| acp::ToolCallContent::Diff {
                    diff: acp::Diff {
                        path: params.file_path.clone(),
                        old_text: Some(edit.old_string.clone()),
                        new_text: edit.new_string.clone(),
                    },
                })
            }
            Self::Task(None)
            | Self::NotebookRead(None)
            | Self::NotebookEdit(None)
            | Self::Terminal(None)
            | Self::ReadFile(None)
            | Self::Ls(None)
            | Self::Glob(None)
            | Self::Grep(None)
            | Self::WebFetch(None)
            | Self::WebSearch(None)
            | Self::TodoWrite(None)
            | Self::ExitPlanMode(None)
            | Self::Edit(None)
            | Self::Write(None)
            | Self::MultiEdit(None) => None,
        }
    }

    pub fn icon(&self) -> acp::Icon {
        match self {
            Self::Task(_) => acp::Icon::Hammer,
            Self::NotebookRead(_) => acp::Icon::FileSearch,
            Self::NotebookEdit(_) => acp::Icon::Pencil,
            Self::Edit(_) => acp::Icon::Pencil,
            Self::MultiEdit(_) => acp::Icon::Pencil,
            Self::Write(_) => acp::Icon::Pencil,
            Self::ReadFile(_) => acp::Icon::FileSearch,
            Self::Ls(_) => acp::Icon::Folder,
            Self::Glob(_) => acp::Icon::FileSearch,
            Self::Grep(_) => acp::Icon::Regex,
            Self::Terminal(_) => acp::Icon::Terminal,
            Self::WebSearch(_) => acp::Icon::Globe,
            Self::WebFetch(_) => acp::Icon::Globe,
            Self::TodoWrite(_) => acp::Icon::LightBulb,
            Self::ExitPlanMode(_) => acp::Icon::Hammer,
            Self::Other { .. } => acp::Icon::Hammer,
        }
    }

    pub fn confirmation(&self, description: Option<String>) -> acp::ToolCallConfirmation {
        match &self {
            Self::Edit(_) | Self::Write(_) | Self::NotebookEdit(_) | Self::MultiEdit(_) => {
                acp::ToolCallConfirmation::Edit { description }
            }
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
            Self::ExitPlanMode(Some(params)) => acp::ToolCallConfirmation::Other {
                description: if let Some(description) = description {
                    format!("{description} {}", params.plan)
                } else {
                    params.plan.clone()
                },
            },
            Self::Task(Some(params)) => acp::ToolCallConfirmation::Other {
                description: if let Some(description) = description {
                    format!("{description} {}", params.description)
                } else {
                    params.description.clone()
                },
            },
            Self::Ls(Some(LsToolParams { path, .. }))
            | Self::ReadFile(Some(ReadToolParams { abs_path: path, .. })) => {
                let path = path.display();
                acp::ToolCallConfirmation::Other {
                    description: if let Some(description) = description {
                        format!("{description} {path}")
                    } else {
                        path.to_string()
                    },
                }
            }
            Self::NotebookRead(Some(NotebookReadToolParams { notebook_path, .. })) => {
                let path = notebook_path.display();
                acp::ToolCallConfirmation::Other {
                    description: if let Some(description) = description {
                        format!("{description} {path}")
                    } else {
                        path.to_string()
                    },
                }
            }
            Self::Glob(Some(params)) => acp::ToolCallConfirmation::Other {
                description: if let Some(description) = description {
                    format!("{description} {params}")
                } else {
                    params.to_string()
                },
            },
            Self::Grep(Some(params)) => acp::ToolCallConfirmation::Other {
                description: if let Some(description) = description {
                    format!("{description} {params}")
                } else {
                    params.to_string()
                },
            },
            Self::WebSearch(Some(params)) => acp::ToolCallConfirmation::Other {
                description: if let Some(description) = description {
                    format!("{description} {params}")
                } else {
                    params.to_string()
                },
            },
            Self::TodoWrite(Some(params)) => {
                let params = params.todos.iter().map(|todo| &todo.content).join(", ");
                acp::ToolCallConfirmation::Other {
                    description: if let Some(description) = description {
                        format!("{description} {params}")
                    } else {
                        params
                    },
                }
            }
            Self::Terminal(None)
            | Self::Task(None)
            | Self::NotebookRead(None)
            | Self::ExitPlanMode(None)
            | Self::Ls(None)
            | Self::Glob(None)
            | Self::Grep(None)
            | Self::ReadFile(None)
            | Self::WebSearch(None)
            | Self::TodoWrite(None)
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
            Self::MultiEdit(Some(MultiEditToolParams { file_path, .. })) => {
                vec![ToolCallLocation {
                    path: file_path.clone(),
                    line: None,
                }]
            }
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
            Self::NotebookRead(Some(NotebookReadToolParams { notebook_path, .. })) => {
                vec![ToolCallLocation {
                    path: notebook_path.clone(),
                    line: None,
                }]
            }
            Self::NotebookEdit(Some(NotebookEditToolParams { notebook_path, .. })) => {
                vec![ToolCallLocation {
                    path: notebook_path.clone(),
                    line: None,
                }]
            }
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
            Self::Task(_)
            | Self::NotebookRead(None)
            | Self::NotebookEdit(None)
            | Self::Edit(None)
            | Self::MultiEdit(None)
            | Self::Write(None)
            | Self::ReadFile(None)
            | Self::Ls(None)
            | Self::Glob(_)
            | Self::Grep(_)
            | Self::Terminal(_)
            | Self::WebFetch(_)
            | Self::WebSearch(_)
            | Self::TodoWrite(_)
            | Self::ExitPlanMode(_)
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

impl std::fmt::Display for GlobToolParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.path {
            write!(f, "{}", path.display())?;
        }
        write!(f, "{}", self.pattern)
    }
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

#[derive(Deserialize, Serialize, JsonSchema, strum::Display, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TodoPriority {
    High,
    Medium,
    Low,
}

impl Into<acp::PlanEntryPriority> for TodoPriority {
    fn into(self) -> acp::PlanEntryPriority {
        match self {
            TodoPriority::High => acp::PlanEntryPriority::High,
            TodoPriority::Medium => acp::PlanEntryPriority::Medium,
            TodoPriority::Low => acp::PlanEntryPriority::Low,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

impl Into<acp::PlanEntryStatus> for TodoStatus {
    fn into(self) -> acp::PlanEntryStatus {
        match self {
            TodoStatus::Pending => acp::PlanEntryStatus::Pending,
            TodoStatus::InProgress => acp::PlanEntryStatus::InProgress,
            TodoStatus::Completed => acp::PlanEntryStatus::Completed,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct Todo {
    /// Unique identifier
    pub id: String,
    /// Task description
    pub content: String,
    /// Priority level of the todo
    pub priority: TodoPriority,
    /// Current status of the todo
    pub status: TodoStatus,
}

impl Into<acp::PlanEntry> for Todo {
    fn into(self) -> acp::PlanEntry {
        acp::PlanEntry {
            content: self.content,
            priority: self.priority.into(),
            status: self.status.into(),
        }
    }
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct TodoWriteToolParams {
    pub todos: Vec<Todo>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct ExitPlanModeToolParams {
    /// Implementation plan in markdown format
    pub plan: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct TaskToolParams {
    /// Short 3-5 word description of task
    pub description: String,
    /// Detailed task for agent to perform
    pub prompt: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct NotebookReadToolParams {
    /// Absolute path to .ipynb file
    pub notebook_path: PathBuf,
    /// Specific cell ID to read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CellType {
    Code,
    Markdown,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EditMode {
    Replace,
    Insert,
    Delete,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct NotebookEditToolParams {
    /// Absolute path to .ipynb file
    pub notebook_path: PathBuf,
    /// New cell content
    pub new_source: String,
    /// Cell ID to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<String>,
    /// Type of cell (code or markdown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_type: Option<CellType>,
    /// Edit operation mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_mode: Option<EditMode>,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct MultiEditItem {
    /// The text to search for and replace
    pub old_string: String,
    /// The replacement text
    pub new_string: String,
    /// Whether to replace all occurrences or just the first
    #[serde(default, skip_serializing_if = "is_false")]
    pub replace_all: bool,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct MultiEditToolParams {
    /// Absolute path to file
    pub file_path: PathBuf,
    /// List of edits to apply
    pub edits: Vec<MultiEditItem>,
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
