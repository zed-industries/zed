use std::{cell::RefCell, path::PathBuf, rc::Rc};

use agentic_coding_protocol::{self as acp, Client, ReadTextFileParams, WriteTextFileParams};
use anyhow::{Context, Result};
use collections::HashMap;
use context_server::{
    listener::McpServer,
    types::{
        CallToolParams, CallToolResponse, Implementation, InitializeParams, InitializeResponse,
        ListToolsResponse, ProtocolVersion, ServerCapabilities, Tool, ToolAnnotations,
        ToolResponseContent, ToolsCapabilities, requests,
    },
};
use gpui::{App, AsyncApp, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AcpClientDelegate, claude::McpServerConfig};

pub struct PermissionMcpServer {
    server: McpServer,
}

pub const SERVER_NAME: &str = "zed";
pub const TOOL_NAME: &str = "request_confirmation";

#[derive(Deserialize, JsonSchema, Debug)]
struct PermissionToolParams {
    tool_name: String,
    input: serde_json::Value,
    tool_use_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PermissionToolResponse {
    behavior: PermissionToolBehavior,
    updated_input: serde_json::Value,
}

#[derive(Deserialize, JsonSchema, Debug)]
struct ReadToolParams {
    /// The absolute path to the file to read.
    abs_path: PathBuf,
    /// Which line to start reading from. Omit to start from the begining.
    offset: Option<u32>,
    /// How many lines to read. Omit for the whole file.
    limit: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadToolResponse {
    content: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
struct EditToolParams {
    /// The absolute path to the file to read.
    abs_path: PathBuf,
    /// The old text to replace (must be unique in the file)
    old_text: String,
    /// The new text.
    new_text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EditToolResponse {}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum PermissionToolBehavior {
    Allow,
    Deny,
}

impl PermissionMcpServer {
    pub fn new(
        cx: &App,
        delegate: AcpClientDelegate,
        tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
    ) -> Result<Self> {
        let mut mcp_server = McpServer::new(cx)?;
        mcp_server.handle_request::<requests::Initialize>(Self::handle_initialize);
        mcp_server.handle_request::<requests::ListTools>(Self::handle_list_tools);
        mcp_server.handle_request::<requests::CallTool>(move |request, cx| {
            Self::handle_call_tool(request, delegate.clone(), tool_id_map.clone(), cx)
        });

        Ok(Self { server: mcp_server })
    }

    pub fn server_config(&self) -> Result<McpServerConfig> {
        let zed_path = util::get_shell_safe_zed_path()?;

        Ok(McpServerConfig {
            command: zed_path,
            args: vec![
                "--nc".into(),
                self.server.socket_path().display().to_string(),
            ],
            env: None,
        })
    }

    fn handle_initialize(_: InitializeParams, cx: &App) -> Task<Result<InitializeResponse>> {
        cx.spawn(async move |_cx| {
            Ok(InitializeResponse {
                protocol_version: ProtocolVersion("2025-06-18".into()),
                capabilities: ServerCapabilities {
                    experimental: None,
                    logging: None,
                    completions: None,
                    prompts: None,
                    resources: None,
                    tools: Some(ToolsCapabilities {
                        list_changed: Some(false),
                    }),
                },
                server_info: Implementation {
                    name: SERVER_NAME.into(),
                    version: "0.1.0".into(),
                },
                meta: None,
            })
        })
    }

    fn handle_list_tools(_: (), cx: &App) -> Task<Result<ListToolsResponse>> {
        cx.spawn(async move |_cx| {
            Ok(ListToolsResponse {
                tools: vec![
                    Tool {
                        name: TOOL_NAME.into(),
                        input_schema: schemars::schema_for!(PermissionToolParams).into(),
                        description: None,
                        annotations: None,
                    },
                    Tool {
                        name: "Read".into(),
                        input_schema: schemars::schema_for!(ReadToolParams).into(),
                        description: Some("Read the contents of a file. In sessions with mcp__zed__Read always use it instead of Read as it contains the most up-to-date contents.".to_string()),
                        annotations: Some(ToolAnnotations {
                            title: Some("Read file".to_string()),
                            read_only_hint: Some(true),
                            destructive_hint: Some(false),
                            open_world_hint: Some(false),
                            // if time passes the contents might change, but it's not going to do anything different
                            // true or false seem too strong, let's try a none.
                            idempotent_hint: None,
                        }),
                    },
                    Tool {
                        name: "Edit".into(),
                        input_schema: schemars::schema_for!(EditToolParams).into(),
                        // todo!() do we need this?
                        // Performs exact string replacements in files.
                        //
                        // Usage:
                        // • You must use your Read tool at least once in the conversation before editing. This tool will error if you attempt an edit without reading the file.
                        // • When editing text from Read tool output, ensure you preserve the exact indentation (tabs/spaces) as it appears AFTER the line number prefix. The line number prefix format is: spaces + line number + tab. Everything after that tab is the actual file content to match. Never include any part of the line number prefix in the old_string or new_string.
                        // • ALWAYS prefer editing existing files in the codebase. NEVER write new files unless explicitly required.
                        // • Only use emojis if the user explicitly requests it. Avoid adding emojis to files unless asked.
                        // • The edit will FAIL if \old_string\ is not unique in the file. Either provide a larger string with more surrounding context to make it unique or use \replace_all\ to change every instance of \old_string\.
                        // • Use \replace_all\ for replacing and renaming strings across the file. This parameter is useful if you want to rename a variable for instance.`;
                        description: Some("Edits a file. In sessions with mcp__zed__Edit always use it instead of Edit as it will show the diff to the user better.".to_string()),
                        annotations: Some(ToolAnnotations {
                            title: Some("Edit file".to_string()),
                            read_only_hint: Some(false),
                            destructive_hint: Some(false),
                            open_world_hint: Some(false),
                            idempotent_hint: Some(false),
                        }),
                    },
                ],
                next_cursor: None,
                meta: None,
            })
        })
    }

    fn handle_call_tool(
        request: CallToolParams,
        delegate: AcpClientDelegate,
        tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
        cx: &App,
    ) -> Task<Result<CallToolResponse>> {
        cx.spawn(async move |cx| {
            if request.name.as_str() == TOOL_NAME {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                let result =
                    Self::handle_permissions_tool_call(input, delegate, tool_id_map, cx).await?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: serde_json::to_string(&result)?,
                    }],
                    is_error: None,
                    meta: None,
                })
            } else if request.name.as_str() == "Read" {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                let result = Self::handle_read_tool_call(input, delegate, cx).await?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: serde_json::to_string(&result)?,
                    }],
                    is_error: None,
                    meta: None,
                })
            } else if request.name.as_str() == "Edit" {
                let input =
                    serde_json::from_value(request.arguments.context("Arguments required")?)?;

                let result = Self::handle_edit_tool_call(input, delegate, cx).await?;
                Ok(CallToolResponse {
                    content: vec![ToolResponseContent::Text {
                        text: serde_json::to_string(&result)?,
                    }],
                    is_error: None,
                    meta: None,
                })
            } else {
                anyhow::bail!("Unsupported tool");
            }
        })
    }

    fn handle_read_tool_call(
        params: ReadToolParams,
        delegate: AcpClientDelegate,
        cx: &AsyncApp,
    ) -> Task<Result<ReadToolResponse>> {
        cx.spawn(async move |_cx| {
            let response = delegate
                .read_text_file(ReadTextFileParams {
                    path: params.abs_path,
                    line: params.offset,
                    limit: params.limit,
                })
                .await?;

            Ok(ReadToolResponse {
                content: response.content,
            })
        })
    }

    fn handle_edit_tool_call(
        params: EditToolParams,
        delegate: AcpClientDelegate,
        cx: &AsyncApp,
    ) -> Task<Result<EditToolResponse>> {
        cx.spawn(async move |_cx| {
            // todo!() use previous read...
            let response = delegate
                .read_text_file(ReadTextFileParams {
                    path: params.abs_path.clone(),
                    line: None,
                    limit: None,
                })
                .await?;

            let new_content = response.content.replace(&params.old_text, &params.new_text);
            if new_content == response.content {
                return Err(anyhow::anyhow!("The old_text was not found in the content"));
            }

            delegate
                .write_text_file(WriteTextFileParams {
                    path: params.abs_path,
                    content: new_content,
                })
                .await?;

            Ok(EditToolResponse {})
        })
    }

    fn handle_permissions_tool_call(
        params: PermissionToolParams,
        delegate: AcpClientDelegate,
        tool_id_map: Rc<RefCell<HashMap<String, acp::ToolCallId>>>,
        cx: &AsyncApp,
    ) -> Task<Result<PermissionToolResponse>> {
        match params.tool_use_id {
            Some(tool_use_id) => {
                let Some(tool_call_id) = tool_id_map.borrow().get(&tool_use_id).cloned() else {
                    // todo!
                    return Task::ready(Err(anyhow::anyhow!("Tool call ID not found")));
                };

                cx.spawn(async move |_cx| {
                    let outcome = delegate
                        .request_existing_tool_call_confirmation(
                            tool_call_id,
                            // todo!
                            acp::ToolCallConfirmation::Edit { description: None },
                        )
                        .await?;

                    match outcome {
                        acp::ToolCallConfirmationOutcome::Allow |
                        // todo! remove these from UI
                        acp::ToolCallConfirmationOutcome::AlwaysAllow |
                        acp::ToolCallConfirmationOutcome::AlwaysAllowMcpServer |
                        acp::ToolCallConfirmationOutcome::AlwaysAllowTool => {
                            Ok(PermissionToolResponse {
                                behavior: PermissionToolBehavior::Allow,
                                updated_input: params.input,
                            })
                        },
                        acp::ToolCallConfirmationOutcome::Reject|
                        acp::ToolCallConfirmationOutcome::Cancel =>{
                            Ok(PermissionToolResponse {
                                behavior: PermissionToolBehavior::Deny,
                                updated_input: params.input,
                            })
                        }
                    }
                })
            }
            None => {
                // todo!
                Task::ready(Ok(PermissionToolResponse {
                    behavior: PermissionToolBehavior::Allow,
                    updated_input: params.input,
                }))
            }
        }
    }
}
