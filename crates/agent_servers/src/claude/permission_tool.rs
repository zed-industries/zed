use std::sync::Arc;

use acp_thread::AcpThread;
use agent_client_protocol as acp;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result};
use context_server::{
    listener::{McpServerTool, ToolResponse},
    types::ToolResponseContent,
};
use gpui::{AsyncApp, WeakEntity};
use project::Fs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings as _, update_settings_file};
use util::debug_panic;

use crate::tools::ClaudeTool;

#[derive(Clone)]
pub struct PermissionTool {
    fs: Arc<dyn Fs>,
    thread_rx: watch::Receiver<WeakEntity<AcpThread>>,
}

/// Request permission for tool calls
#[derive(Deserialize, JsonSchema, Debug)]
pub struct PermissionToolParams {
    tool_name: String,
    input: serde_json::Value,
    tool_use_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionToolResponse {
    behavior: PermissionToolBehavior,
    updated_input: serde_json::Value,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum PermissionToolBehavior {
    Allow,
    Deny,
}

impl PermissionTool {
    pub fn new(fs: Arc<dyn Fs>, thread_rx: watch::Receiver<WeakEntity<AcpThread>>) -> Self {
        Self { fs, thread_rx }
    }
}

impl McpServerTool for PermissionTool {
    type Input = PermissionToolParams;
    type Output = ();

    const NAME: &'static str = "Confirmation";

    async fn run(
        &self,
        input: Self::Input,
        cx: &mut AsyncApp,
    ) -> Result<ToolResponse<Self::Output>> {
        if agent_settings::AgentSettings::try_read_global(cx, |settings| {
            settings.always_allow_tool_actions
        })
        .unwrap_or(false)
        {
            let response = PermissionToolResponse {
                behavior: PermissionToolBehavior::Allow,
                updated_input: input.input,
            };

            return Ok(ToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: serde_json::to_string(&response)?,
                }],
                structured_content: (),
            });
        }

        let mut thread_rx = self.thread_rx.clone();
        let Some(thread) = thread_rx.recv().await?.upgrade() else {
            anyhow::bail!("Thread closed");
        };

        let claude_tool = ClaudeTool::infer(&input.tool_name, input.input.clone());
        let tool_call_id = acp::ToolCallId(input.tool_use_id.context("Tool ID required")?.into());

        const ALWAYS_ALLOW: &str = "always_allow";
        const ALLOW: &str = "allow";
        const REJECT: &str = "reject";

        let chosen_option = thread
            .update(cx, |thread, cx| {
                thread.request_tool_call_authorization(
                    claude_tool.as_acp(tool_call_id).into(),
                    vec![
                        acp::PermissionOption {
                            id: acp::PermissionOptionId(ALWAYS_ALLOW.into()),
                            name: "Always Allow".into(),
                            kind: acp::PermissionOptionKind::AllowAlways,
                        },
                        acp::PermissionOption {
                            id: acp::PermissionOptionId(ALLOW.into()),
                            name: "Allow".into(),
                            kind: acp::PermissionOptionKind::AllowOnce,
                        },
                        acp::PermissionOption {
                            id: acp::PermissionOptionId(REJECT.into()),
                            name: "Reject".into(),
                            kind: acp::PermissionOptionKind::RejectOnce,
                        },
                    ],
                    cx,
                )
            })??
            .await?;

        let response = match chosen_option.0.as_ref() {
            ALWAYS_ALLOW => {
                cx.update(|cx| {
                    update_settings_file::<AgentSettings>(self.fs.clone(), cx, |settings, _| {
                        settings.set_always_allow_tool_actions(true);
                    });
                })?;

                PermissionToolResponse {
                    behavior: PermissionToolBehavior::Allow,
                    updated_input: input.input,
                }
            }
            ALLOW => PermissionToolResponse {
                behavior: PermissionToolBehavior::Allow,
                updated_input: input.input,
            },
            REJECT => PermissionToolResponse {
                behavior: PermissionToolBehavior::Deny,
                updated_input: input.input,
            },
            opt => {
                debug_panic!("Unexpected option: {}", opt);
                PermissionToolResponse {
                    behavior: PermissionToolBehavior::Deny,
                    updated_input: input.input,
                }
            }
        };

        Ok(ToolResponse {
            content: vec![ToolResponseContent::Text {
                text: serde_json::to_string(&response)?,
            }],
            structured_content: (),
        })
    }
}
