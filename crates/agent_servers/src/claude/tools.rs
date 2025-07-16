use agentic_coding_protocol as acp;

pub enum ClaudeTool {
    Edit,
    ReadFile,
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
    pub fn infer(tool_name: &str) -> Self {
        match tool_name {
            // Known tools
            "mcp__zed__Read" => Self::ReadFile,
            "mcp__zed__Edit" => Self::Edit,
            "MultiEdit" => Self::Edit,
            "Write" => Self::Edit,
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
                    Self::Edit
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

    pub fn icon(&self) -> acp::Icon {
        match self {
            Self::Edit => acp::Icon::Pencil,
            Self::ReadFile => acp::Icon::FileSearch,
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
            Self::Edit => acp::ToolCallConfirmation::Edit { description },
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
            | Self::ReadFile
            | Self::Other => acp::ToolCallConfirmation::Other {
                description: description.unwrap_or("".to_string()),
            },
        }
    }
}
