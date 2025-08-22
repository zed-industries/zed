use std::{collections::HashSet, fmt::Display, sync::Arc};

use agent_client_protocol as acp;
use gpui::{
    App, Empty, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment, ListState,
    StyleRefinement, TextStyleRefinement, Window, actions, list, prelude::*,
};
use language::LanguageRegistry;
use markdown::{CodeBlockRenderer, Markdown, MarkdownElement, MarkdownStyle};
use project::Project;
use serde::Serialize;
use settings::Settings;
use theme::ThemeSettings;
use ui::prelude::*;
use workspace::{Item, Workspace};

actions!(acp, [OpenDebugTools]);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &OpenDebugTools, window, cx| {
                let acp_tools =
                    Box::new(cx.new(|cx| AcpTools::new(workspace.project().clone(), cx)));
                workspace.add_item_to_active_pane(acp_tools, None, true, window, cx);
            });
        },
    )
    .detach();
}

struct AcpTools {
    project: Entity<Project>,
    focus_handle: FocusHandle,
    list_state: ListState,
    messages: Vec<AnyMessage>,
    expanded: HashSet<usize>,
}

impl AcpTools {
    fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let test_messages = vec![
            // Client Request -> Agent Response pairs
            Message::ClientRequest(acp::ClientRequest::InitializeRequest(
                acp::InitializeRequest {
                    protocol_version: acp::ProtocolVersion::default(),
                    client_capabilities: acp::ClientCapabilities {
                        fs: acp::FileSystemCapability {
                            read_text_file: true,
                            write_text_file: true,
                        },
                    },
                },
            )),
            Message::AgentResponse(acp::AgentResponse::InitializeResponse(
                acp::InitializeResponse {
                    protocol_version: acp::ProtocolVersion::default(),
                    agent_capabilities: acp::AgentCapabilities {
                        load_session: true,
                        prompt_capabilities: acp::PromptCapabilities {
                            image: true,
                            audio: false,
                            embedded_context: true,
                        },
                    },
                    auth_methods: vec![acp::AuthMethod {
                        id: acp::AuthMethodId("oauth".into()),
                        name: "OAuth 2.0".into(),
                        description: Some("OAuth 2.0 authentication".into()),
                    }],
                },
            )),
            Message::ClientRequest(acp::ClientRequest::AuthenticateRequest(
                acp::AuthenticateRequest {
                    method_id: acp::AuthMethodId("oauth".into()),
                },
            )),
            Message::AgentResponse(acp::AgentResponse::AuthenticateResponse),
            Message::ClientRequest(acp::ClientRequest::NewSessionRequest(
                acp::NewSessionRequest {
                    mcp_servers: vec![acp::McpServer {
                        name: "filesystem".into(),
                        command: "/usr/bin/mcp-server".into(),
                        args: vec!["--filesystem".into()],
                        env: vec![acp::EnvVariable {
                            name: "PATH".into(),
                            value: "/usr/bin".into(),
                        }],
                    }],
                    cwd: "/home/user/project".into(),
                },
            )),
            Message::AgentResponse(acp::AgentResponse::NewSessionResponse(
                acp::NewSessionResponse {
                    session_id: acp::SessionId("session-123".into()),
                },
            )),
            Message::ClientRequest(acp::ClientRequest::LoadSessionRequest(
                acp::LoadSessionRequest {
                    mcp_servers: vec![],
                    cwd: "/home/user/project".into(),
                    session_id: acp::SessionId("session-123".into()),
                },
            )),
            Message::AgentResponse(acp::AgentResponse::LoadSessionResponse),
            Message::ClientRequest(acp::ClientRequest::PromptRequest(acp::PromptRequest {
                session_id: acp::SessionId("session-123".into()),
                prompt: vec![
                    acp::ContentBlock::Text(acp::TextContent {
                        annotations: None,
                        text: "Hello, can you help me write some code?".into(),
                    }),
                    acp::ContentBlock::ResourceLink(acp::ResourceLink {
                        annotations: None,
                        description: Some("Main source file".into()),
                        mime_type: Some("text/rust".into()),
                        name: "main.rs".into(),
                        size: Some(1024),
                        title: Some("Main Rust File".into()),
                        uri: "file:///home/user/project/src/main.rs".into(),
                    }),
                ],
            })),
            Message::AgentResponse(acp::AgentResponse::PromptResponse(acp::PromptResponse {
                stop_reason: acp::StopReason::EndTurn,
            })),
            // Agent Request -> Client Response pairs
            Message::AgentRequest(acp::AgentRequest::RequestPermissionRequest(
                acp::RequestPermissionRequest {
                    session_id: acp::SessionId("session-123".into()),
                    tool_call: acp::ToolCallUpdate {
                        id: acp::ToolCallId("tool-call-456".into()),
                        fields: acp::ToolCallUpdateFields {
                            title: Some("Write File".into()),
                            kind: Some(acp::ToolKind::Edit),
                            status: Some(acp::ToolCallStatus::Pending),
                            content: Some(vec![acp::ToolCallContent::Content {
                                content: acp::ContentBlock::Text(acp::TextContent {
                                    annotations: None,
                                    text: "Writing to main.rs".into(),
                                }),
                            }]),
                            locations: Some(vec![acp::ToolCallLocation {
                                path: "src/main.rs".into(),
                                line: Some(10),
                            }]),
                            raw_input: None,
                            raw_output: None,
                        },
                    },
                    options: vec![
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("allow_once".into()),
                            name: "Allow Once".into(),
                            kind: acp::PermissionOptionKind::AllowOnce,
                        },
                        acp::PermissionOption {
                            id: acp::PermissionOptionId("reject_once".into()),
                            name: "Reject Once".into(),
                            kind: acp::PermissionOptionKind::RejectOnce,
                        },
                    ],
                },
            )),
            Message::ClientResponse(acp::ClientResponse::RequestPermissionResponse(
                acp::RequestPermissionResponse {
                    outcome: acp::RequestPermissionOutcome::Selected {
                        option_id: acp::PermissionOptionId("allow_once".into()),
                    },
                },
            )),
            Message::AgentRequest(acp::AgentRequest::WriteTextFileRequest(
                acp::WriteTextFileRequest {
                    session_id: acp::SessionId("session-123".into()),
                    path: "src/main.rs".into(),
                    content: "fn main() {\n    println!(\"Hello, world!\");\n}".into(),
                },
            )),
            Message::ClientResponse(acp::ClientResponse::WriteTextFileResponse),
            Message::AgentRequest(acp::AgentRequest::ReadTextFileRequest(
                acp::ReadTextFileRequest {
                    session_id: acp::SessionId("session-123".into()),
                    path: "src/main.rs".into(),
                    line: Some(1),
                    limit: Some(100),
                },
            )),
            Message::ClientResponse(acp::ClientResponse::ReadTextFileResponse(
                acp::ReadTextFileResponse {
                    content: "fn main() {\n    println!(\"Hello, world!\");\n}".into(),
                },
            )),
            // Notifications (no responses)
            Message::ClientNotification(acp::ClientNotification::CancelNotification(
                acp::CancelNotification {
                    session_id: acp::SessionId("session-123".into()),
                },
            )),
            Message::AgentNotification(acp::AgentNotification::SessionNotification(
                acp::SessionNotification {
                    session_id: acp::SessionId("session-123".into()),
                    update: acp::SessionUpdate::UserMessageChunk {
                        content: acp::ContentBlock::Text(acp::TextContent {
                            annotations: None,
                            text: "User is typing...".into(),
                        }),
                    },
                },
            )),
            Message::AgentNotification(acp::AgentNotification::SessionNotification(
                acp::SessionNotification {
                    session_id: acp::SessionId("session-123".into()),
                    update: acp::SessionUpdate::AgentMessageChunk {
                        content: acp::ContentBlock::Text(acp::TextContent {
                            annotations: None,
                            text: "I'll help you write that code...".into(),
                        }),
                    },
                },
            )),
            Message::AgentNotification(acp::AgentNotification::SessionNotification(
                acp::SessionNotification {
                    session_id: acp::SessionId("session-123".into()),
                    update: acp::SessionUpdate::AgentThoughtChunk {
                        content: acp::ContentBlock::Text(acp::TextContent {
                            annotations: None,
                            text: "Let me think about the best approach...".into(),
                        }),
                    },
                },
            )),
            Message::AgentNotification(acp::AgentNotification::SessionNotification(
                acp::SessionNotification {
                    session_id: acp::SessionId("session-123".into()),
                    update: acp::SessionUpdate::ToolCall(acp::ToolCall {
                        id: acp::ToolCallId("tool-call-789".into()),
                        title: "Read File".into(),
                        kind: acp::ToolKind::Read,
                        status: acp::ToolCallStatus::InProgress,
                        content: vec![acp::ToolCallContent::Content {
                            content: acp::ContentBlock::Text(acp::TextContent {
                                annotations: None,
                                text: "Reading src/main.rs".into(),
                            }),
                        }],
                        locations: vec![acp::ToolCallLocation {
                            path: "src/main.rs".into(),
                            line: None,
                        }],
                        raw_input: Some(serde_json::json!({"path": "src/main.rs"})),
                        raw_output: None,
                    }),
                },
            )),
            Message::AgentNotification(acp::AgentNotification::SessionNotification(
                acp::SessionNotification {
                    session_id: acp::SessionId("session-123".into()),
                    update: acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate {
                        id: acp::ToolCallId("tool-call-789".into()),
                        fields: acp::ToolCallUpdateFields {
                            status: Some(acp::ToolCallStatus::Completed),
                            raw_output: Some(
                                serde_json::json!({"content": "fn main() { println!(\"Hello!\"); }"}),
                            ),
                            ..Default::default()
                        },
                    }),
                },
            )),
            Message::AgentNotification(acp::AgentNotification::SessionNotification(
                acp::SessionNotification {
                    session_id: acp::SessionId("session-123".into()),
                    update: acp::SessionUpdate::Plan(acp::Plan {
                        entries: vec![
                            acp::PlanEntry {
                                content: "Read the existing code".into(),
                                priority: acp::PlanEntryPriority::High,
                                status: acp::PlanEntryStatus::Completed,
                            },
                            acp::PlanEntry {
                                content: "Write improved version".into(),
                                priority: acp::PlanEntryPriority::Medium,
                                status: acp::PlanEntryStatus::InProgress,
                            },
                            acp::PlanEntry {
                                content: "Test the changes".into(),
                                priority: acp::PlanEntryPriority::Low,
                                status: acp::PlanEntryStatus::Pending,
                            },
                        ],
                    }),
                },
            )),
        ];

        let language_registry = project.read(cx).languages().clone();

        Self {
            project,
            focus_handle: cx.focus_handle(),
            list_state: ListState::new(test_messages.len(), ListAlignment::Top, px(2048.)),
            messages: test_messages
                .into_iter()
                .map(|m| m.as_any(language_registry.clone(), cx))
                .collect(),
            expanded: HashSet::default(),
        }
    }

    fn render_message(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(message) = self.messages.get(index) else {
            return Empty.into_any();
        };

        let base_size = TextSize::Editor.rems(cx);

        let theme_settings = ThemeSettings::get_global(cx);
        let text_style = window.text_style();

        let colors = cx.theme().colors();

        v_flex()
            .w_full()
            .px_4()
            .py_3()
            .border_color(colors.border)
            .border_b_1()
            .gap_2()
            .items_start()
            .font_buffer(cx)
            .text_size(base_size)
            .id(index)
            .group("message")
            .hover(|this| this.bg(colors.element_background.opacity(0.5)))
            .on_click(cx.listener(move |this, _, _, cx| {
                if this.expanded.contains(&index) {
                    this.expanded.remove(&index);
                } else {
                    this.expanded.insert(index);
                    if let Some(message) = this.messages.get_mut(index) {
                        message.expanded(this.project.read(cx).languages().clone(), cx);
                    }
                    this.list_state.scroll_to_reveal_item(index);
                }
                cx.notify()
            }))
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .items_center()
                    .flex_shrink_0()
                    .child(message.direction.icon())
                    .child(div().child(message.name).text_color(colors.text_muted))
                    .child(div().flex_1())
                    .child(
                        div()
                            .child(ui::Chip::new(message.message_type.to_string()))
                            .visible_on_hover("message"),
                    ),
            )
            // probably shouldn't use markdown for this
            .when_some(
                if self.expanded.contains(&index) {
                    message.expanded_params_md.clone()
                } else {
                    message.collapsed_params_md.clone()
                },
                |this, params| {
                    this.child(
                        div().ml_6().flex_1().child(
                            MarkdownElement::new(
                                params,
                                MarkdownStyle {
                                    base_text_style: text_style,
                                    selection_background_color: colors.element_selection_background,
                                    syntax: cx.theme().syntax().clone(),
                                    code_block_overflow_x_scroll: true,
                                    code_block: StyleRefinement {
                                        text: Some(TextStyleRefinement {
                                            font_family: Some(
                                                theme_settings.buffer_font.family.clone(),
                                            ),
                                            font_size: Some((base_size * 0.8).into()),
                                            ..Default::default()
                                        }),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                            )
                            .code_block_renderer(
                                CodeBlockRenderer::Default {
                                    copy_button: false,
                                    copy_button_on_hover: false,
                                    border: false,
                                },
                            ),
                        ),
                    )
                },
            )
            .into_any()
    }
}

#[derive(Debug)]
pub enum Message {
    ClientRequest(acp::ClientRequest),
    ClientResponse(acp::ClientResponse),
    ClientNotification(acp::ClientNotification),
    AgentRequest(acp::AgentRequest),
    AgentResponse(acp::AgentResponse),
    AgentNotification(acp::AgentNotification),
}

impl Message {
    fn as_any(&self, language_registry: Arc<LanguageRegistry>, cx: &mut App) -> AnyMessage {
        match self {
            Message::ClientRequest(client_request) => {
                let name = match client_request {
                    acp::ClientRequest::InitializeRequest(_) => "initialize",
                    acp::ClientRequest::AuthenticateRequest(_) => "authenticate",
                    acp::ClientRequest::NewSessionRequest(_) => "session/new",
                    acp::ClientRequest::LoadSessionRequest(_) => "session/load",
                    acp::ClientRequest::PromptRequest(_) => "session/prompt",
                };
                AnyMessage::new(
                    name,
                    Direction::Outgoing,
                    MessageType::Request,
                    client_request,
                    &language_registry,
                    cx,
                )
            }
            Message::ClientResponse(client_response) => {
                let name = match client_response {
                    acp::ClientResponse::RequestPermissionResponse(_) => {
                        "session/request_permission"
                    }
                    acp::ClientResponse::WriteTextFileResponse => "fs/write_text_file",
                    acp::ClientResponse::ReadTextFileResponse(_) => "fs/read_text_file",
                };
                AnyMessage::new(
                    name,
                    Direction::Incoming,
                    MessageType::Response,
                    client_response,
                    &language_registry,
                    cx,
                )
            }
            Message::ClientNotification(client_notification) => {
                let name = match client_notification {
                    acp::ClientNotification::CancelNotification(_) => "session/cancel",
                };
                AnyMessage::new(
                    name,
                    Direction::Outgoing,
                    MessageType::Notification,
                    client_notification,
                    &language_registry,
                    cx,
                )
            }
            Message::AgentRequest(agent_request) => {
                let name = match agent_request {
                    acp::AgentRequest::RequestPermissionRequest(_) => "session/request_permission",
                    acp::AgentRequest::WriteTextFileRequest(_) => "fs/write_text_file",
                    acp::AgentRequest::ReadTextFileRequest(_) => "fs/read_text_file",
                };
                AnyMessage::new(
                    name,
                    Direction::Incoming,
                    MessageType::Request,
                    agent_request,
                    &language_registry,
                    cx,
                )
            }
            Message::AgentResponse(agent_response) => {
                let name = match agent_response {
                    acp::AgentResponse::InitializeResponse(_) => "initialize",
                    acp::AgentResponse::AuthenticateResponse => "authenticate",
                    acp::AgentResponse::NewSessionResponse(_) => "session/new",
                    acp::AgentResponse::LoadSessionResponse => "session/load",
                    acp::AgentResponse::PromptResponse(_) => "session/prompt",
                };
                AnyMessage::new(
                    name,
                    Direction::Incoming,
                    MessageType::Response,
                    agent_response,
                    &language_registry,
                    cx,
                )
            }
            Message::AgentNotification(agent_notification) => {
                let name = match agent_notification {
                    acp::AgentNotification::SessionNotification(_) => "session/update",
                };
                AnyMessage::new(
                    name,
                    Direction::Incoming,
                    MessageType::Notification,
                    agent_notification,
                    &language_registry,
                    cx,
                )
            }
        }
    }
}

struct AnyMessage {
    name: &'static str,
    direction: Direction,
    message_type: MessageType,
    params: Option<serde_json::Value>,
    collapsed_params_md: Option<Entity<Markdown>>,
    expanded_params_md: Option<Entity<Markdown>>,
}

impl AnyMessage {
    fn new(
        name: &'static str,
        direction: Direction,
        message_type: MessageType,
        params: &impl Serialize,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let params = serde_json::to_value(params).unwrap_or_default();
        // hide empty responses
        let params = if params.is_null() { None } else { Some(params) };

        Self {
            message_type,
            direction,
            name,
            collapsed_params_md: params
                .as_ref()
                .map(|params| collapsed_params_md(params, language_registry, cx)),
            expanded_params_md: None,
            params,
        }
    }

    fn expanded(&mut self, language_registry: Arc<LanguageRegistry>, cx: &mut App) {
        if let Some(params) = &self.params {
            let params_json = serde_json::to_string_pretty(params).unwrap_or_default();
            let params_md = format!("```json\n{}\n```", params_json);
            self.expanded_params_md = Some(cx.new(|cx| {
                Markdown::new(params_md.into(), Some(language_registry.clone()), None, cx)
            }));
        }
    }
}

fn collapsed_params_md(
    params: &serde_json::Value,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut App,
) -> Entity<Markdown> {
    let params_json = serde_json::to_string(params)
        .unwrap_or_default()
        .replace("{", "{ ")
        .replace("}", " }")
        .replace(":", ": ")
        .replace(",", ", ");

    let params_md = format!("```json\n{}\n```", params_json);
    cx.new(|cx| Markdown::new(params_md.into(), Some(language_registry.clone()), None, cx))
}

enum Direction {
    Incoming,
    Outgoing,
}

enum MessageType {
    Request,
    Response,
    Notification,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Request => write!(f, "Request"),
            MessageType::Response => write!(f, "Response"),
            MessageType::Notification => write!(f, "Notification"),
        }
    }
}

impl Direction {
    fn icon(&self) -> ui::Icon {
        match self {
            Direction::Incoming => ui::Icon::new(ui::IconName::ArrowDown).color(Color::Error),
            Direction::Outgoing => ui::Icon::new(ui::IconName::ArrowUp).color(Color::Success),
        }
    }
}

enum AcpToolsEvent {}

impl EventEmitter<AcpToolsEvent> for AcpTools {}

impl Item for AcpTools {
    type Event = AcpToolsEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> ui::SharedString {
        "ACP Stream".into()
    }
}

impl Focusable for AcpTools {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AcpTools {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                list(self.list_state.clone(), cx.processor(Self::render_message))
                    .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                    .flex_grow(),
            )
    }
}
