use std::{
    cell::Cell,
    fmt,
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use acp_thread::{AcpThread, AcpThreadEvent, MentionUri, ThreadStatus, line_range_suffix};
use agent::{ContextServerRegistry, SharedThread, ThreadStore};
use agent_client_protocol::schema as acp;
use agent_servers::AgentServer;
use agent_settings::UserAgentsMd;
use collections::HashSet;
use db::kvp::{Dismissable, KeyValueStore};
use itertools::Itertools;
use project::{AgentId, ProjectItem};
use serde::{Deserialize, Serialize};
use settings::{LanguageModelProviderSetting, LanguageModelSelection};

use zed_actions::{
    DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize,
    agent::{
        AddSelectionToThread, ConflictContent, LogoutAgent, OpenSettings, ReauthenticateAgent,
        ResetAgentZoom, ResetOnboarding, ResolveConflictedFilesWithAgent,
        ResolveConflictsWithAgent, ReviewBranchDiff,
    },
    assistant::{
        FocusAgent, ManageSkills, OpenGlobalAgentsMdRules, OpenProjectAgentsMdRules, Toggle,
        ToggleFocus,
    },
};

use crate::ExpandMessageEditor;
use crate::ManageProfiles;
use crate::agent_connection_store::AgentConnectionStore;
use crate::completion_provider::{AgentContextSelection, AgentContextSource};
use crate::terminal_thread_metadata_store::{
    TerminalThreadMetadata, TerminalThreadMetadataStore, compose_terminal_thread_title,
    terminal_title_without_prefix,
};
use crate::thread_metadata_store::{ThreadId, ThreadMetadataStore, ThreadMetadataStoreEvent};
use crate::{
    AddContextServer, AgentDiffPane, ConversationView, CopyThreadToClipboard, Follow,
    LoadThreadFromClipboard, NewTerminalThread, NewThread, OpenActiveThreadAsMarkdown,
    OpenAgentDiff, ResetFastModeWarnings, ResetTrialEndUpsell, ResetTrialUpsell,
    ShowAllSidebarThreadMetadata, ShowThreadMetadata, ToggleNewThreadMenu, ToggleOptionsMenu,
    agent_configuration::{AgentConfiguration, AssistantConfigurationEvent},
    conversation_view::{
        AcpThreadViewEvent, RootThreadUpdated, ThreadView, reset_fast_mode_warnings,
    },
    ui::{AgentNotification, AgentNotificationEvent, EndTrialUpsell},
};
use crate::{
    Agent, AgentInitialContent, AgentThreadSource, ExternalSourcePrompt, NewExternalAgentThread,
    NewNativeAgentThreadFromSummary,
};
use agent_settings::AgentSettings;
use ai_onboarding::AgentPanelOnboarding;
use anyhow::{Context as _, Result, anyhow};
#[cfg(feature = "audio")]
use audio::{Audio, Sound};
use chrono::{DateTime, Utc};
use client::UserStore;
use cloud_api_types::Plan;
use collections::HashMap;
use editor::{Editor, MultiBuffer};
use extension_host::ExtensionStore;
use feature_flags::{CreateThreadToolFeatureFlag, FeatureFlagAppExt as _};

use fs::Fs;
use gpui::{
    Action, Anchor, Animation, AnimationExt, AnyElement, App, AsyncWindowContext, ClipboardItem,
    Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable, KeyContext, Pixels,
    PlatformDisplay, Subscription, Task, TaskExt, WeakEntity, WindowHandle, prelude::*,
    pulsating_between,
};
use language::LanguageRegistry;
use language_model::LanguageModelRegistry;
use notifications::status_toast::StatusToast;
use project::{Project, ProjectPath, Worktree};
use settings::TerminalDockPosition;
use settings::{NotifyWhenAgentWaiting, Settings, update_settings_file};

use terminal::{Event as TerminalEvent, terminal_settings::TerminalSettings};
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use text::OffsetRangeExt;
use theme_settings::ThemeSettings;
use ui::{
    ContextMenu, ContextMenuEntry, GradientFade, IconButton, KeyBinding, PopoverMenu,
    PopoverMenuHandle, ProjectEmptyState, Tab, Tooltip, prelude::*, utils::WithRemSize,
};
use util::ResultExt as _;
use workspace::{
    CollaboratorId, DraggedSelection, DraggedTab, MultiWorkspace, PathList, SerializedPathList,
    ToggleWorkspaceSidebar, ToggleZoom, Workspace, WorkspaceId,
    dock::{DockPosition, Panel, PanelEvent},
    item::ItemEvent,
};

const AGENT_PANEL_KEY: &str = "agent_panel";
const MIN_PANEL_WIDTH: Pixels = px(300.);
const LAST_USED_AGENT_KEY: &str = "agent_panel__last_used_external_agent";
const LAST_CREATED_ENTRY_KIND_KEY: &str = "agent_panel__last_created_entry_kind";
const TERMINAL_AGENT_TELEMETRY_ID: &str = "terminal";
const KNOWN_TERMINAL_AGENT_COMMANDS: &[&str] = &[
    "agent", // Unfortunately, both Cursor cli + grok
    "agy",
    "aider",
    "amp",
    "claude",
    "codex",
    "copilot",
    "crush",
    "devin",
    "droid",
    "gemini",
    "goose",
    "grok",
    "openhands",
    "opencode",
    "pi",
    "qwen",
];

fn is_known_terminal_agent_command(command: &str) -> bool {
    KNOWN_TERMINAL_AGENT_COMMANDS.contains(&command)
}

fn terminal_program_to_report(
    last_observed_program: &mut Option<String>,
    current_program: Option<String>,
) -> Option<String> {
    let current_program =
        current_program.filter(|program| is_known_terminal_agent_command(program));
    let program_to_report =
        if current_program.is_some() && current_program != *last_observed_program {
            current_program.clone()
        } else {
            None
        };
    *last_observed_program = current_program;
    program_to_report
}

/// Maximum number of idle threads kept in the agent panel's retained list.
/// Set as a GPUI global to override; otherwise defaults to 5.
pub struct MaxIdleRetainedThreads(pub usize);
impl gpui::Global for MaxIdleRetainedThreads {}

impl MaxIdleRetainedThreads {
    pub fn global(cx: &App) -> usize {
        cx.try_global::<Self>().map_or(5, |g| g.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TerminalId(uuid::Uuid);

impl TerminalId {
    pub(crate) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub(crate) fn to_key_string(self) -> String {
        self.0.hyphenated().to_string()
    }

    pub(crate) fn from_key_string(key: &str) -> anyhow::Result<Self> {
        Ok(Self(uuid::Uuid::parse_str(key)?))
    }
}

impl fmt::Display for TerminalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Debug)]
pub struct AgentPanelTerminalInfo {
    pub id: TerminalId,
    pub title: SharedString,
    pub created_at: DateTime<Utc>,
    pub has_notification: bool,
    pub custom_title: Option<SharedString>,
    pub working_directory: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct LastUsedAgent {
    agent: Agent,
}

#[derive(Serialize, Deserialize)]
struct LastCreatedEntryKind {
    entry_kind: AgentPanelEntryKind,
}

struct SourcePanelInitialization {
    agent: Agent,
    initial_content: Option<AgentInitialContent>,
}

/// Reads the most recently used agent across all workspaces. Used as a fallback
/// when opening a workspace that has no per-workspace agent preference yet.
fn read_global_last_used_agent(kvp: &KeyValueStore) -> Option<Agent> {
    kvp.read_kvp(LAST_USED_AGENT_KEY)
        .log_err()
        .flatten()
        .and_then(|json| serde_json::from_str::<LastUsedAgent>(&json).log_err())
        .map(|entry| entry.agent)
}

async fn write_global_last_used_agent(kvp: KeyValueStore, agent: Agent) {
    if let Some(json) = serde_json::to_string(&LastUsedAgent { agent }).log_err() {
        kvp.write_kvp(LAST_USED_AGENT_KEY.to_string(), json)
            .await
            .log_err();
    }
}

fn read_global_last_created_entry_kind(kvp: &KeyValueStore) -> Option<AgentPanelEntryKind> {
    kvp.read_kvp(LAST_CREATED_ENTRY_KIND_KEY)
        .log_err()
        .flatten()
        .and_then(|json| serde_json::from_str::<LastCreatedEntryKind>(&json).log_err())
        .map(|entry| entry.entry_kind)
}

fn project_agents_md_path(
    project: &Entity<Project>,
    require_existing_file: bool,
    cx: &App,
) -> Option<PathBuf> {
    let rel_path = util::rel_path::RelPath::unix("AGENTS.md").ok()?;
    project
        .read(cx)
        .visible_worktrees(cx)
        .next()
        .and_then(|worktree| {
            let worktree = worktree.read(cx);

            if require_existing_file {
                let entry = worktree.entry_for_path(rel_path)?;
                if !entry.is_file() {
                    return None;
                }
            }

            Some(worktree.absolutize(rel_path))
        })
}

fn open_global_rules(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    workspace
        .open_abs_path(
            paths::agents_file().clone(),
            workspace::OpenOptions {
                focus: Some(true),
                ..Default::default()
            },
            window,
            cx,
        )
        .detach_and_log_err(cx);
}

fn open_project_rules(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    if let Some(path) = project_agents_md_path(workspace.project(), false, cx) {
        workspace
            .open_abs_path(
                path,
                workspace::OpenOptions {
                    focus: Some(true),
                    ..Default::default()
                },
                window,
                cx,
            )
            .detach_and_log_err(cx);
    }
}

async fn write_global_last_created_entry_kind(kvp: KeyValueStore, entry_kind: AgentPanelEntryKind) {
    if let Some(json) = serde_json::to_string(&LastCreatedEntryKind { entry_kind }).log_err() {
        kvp.write_kvp(LAST_CREATED_ENTRY_KIND_KEY.to_string(), json)
            .await
            .log_err();
    }
}

fn read_serialized_panel(
    workspace_id: workspace::WorkspaceId,
    kvp: &KeyValueStore,
) -> Option<SerializedAgentPanel> {
    let scope = kvp.scoped(AGENT_PANEL_KEY);
    let key = i64::from(workspace_id).to_string();
    scope
        .read(&key)
        .log_err()
        .flatten()
        .and_then(|json| serde_json::from_str::<SerializedAgentPanel>(&json).log_err())
}

async fn save_serialized_panel(
    workspace_id: workspace::WorkspaceId,
    panel: SerializedAgentPanel,
    kvp: KeyValueStore,
) -> Result<()> {
    let scope = kvp.scoped(AGENT_PANEL_KEY);
    let key = i64::from(workspace_id).to_string();
    scope.write(key, serde_json::to_string(&panel)?).await?;
    Ok(())
}

/// Migration: reads the original single-panel format stored under the
/// `"agent_panel"` KVP key before per-workspace keying was introduced.
fn read_legacy_serialized_panel(kvp: &KeyValueStore) -> Option<SerializedAgentPanel> {
    kvp.read_kvp(AGENT_PANEL_KEY)
        .log_err()
        .flatten()
        .and_then(|json| serde_json::from_str::<SerializedAgentPanel>(&json).log_err())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadTitleRegenerationResult {
    NotOpen,
    Started,
    NoModel,
    AlreadyGenerating,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
enum AgentPanelEntryKind {
    #[default]
    Thread,
    Terminal,
}

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentPanel {
    selected_agent: Option<Agent>,
    #[serde(default)]
    last_created_entry_kind: AgentPanelEntryKind,
    #[serde(default)]
    last_active_thread: Option<SerializedActiveThread>,
    #[serde(default)]
    last_active_terminal_id: Option<String>,
    #[serde(default)]
    new_draft_thread_id: Option<ThreadId>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SerializedActiveThread {
    /// For drafts this is `None`; use `thread_id` to address them instead.
    session_id: Option<String>,
    /// Optional for back-compat with older serialized payloads that only carried `session_id`.
    #[serde(default)]
    thread_id: Option<ThreadId>,
    agent_type: Agent,
    title: Option<String>,
    work_dirs: Option<SerializedPathList>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, _: &NewThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.new_thread_with_workspace(Some(workspace), window, cx)
                        });
                        workspace.focus_panel::<AgentPanel>(window, cx);
                    }
                })
                .register_action(|workspace, _: &NewTerminalThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.new_terminal(
                                Some(workspace),
                                AgentThreadSource::AgentPanel,
                                window,
                                cx,
                            )
                        });
                        workspace.focus_panel::<AgentPanel>(window, cx);
                    }
                })
                .register_action(
                    |workspace, action: &NewNativeAgentThreadFromSummary, window, cx| {
                        if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                            panel.update(cx, |panel, cx| {
                                panel.new_native_agent_thread_from_summary(action, window, cx)
                            });
                            workspace.focus_panel::<AgentPanel>(window, cx);
                        }
                    },
                )
                .register_action(|workspace, _: &ExpandMessageEditor, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.expand_message_editor(window, cx));
                    }
                })
                .register_action(|workspace, _: &OpenSettings, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_configuration(window, cx));
                    }
                })
                .register_action(|workspace, action: &NewExternalAgentThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.new_external_agent_thread(action, window, cx);
                        });
                    }
                })
                .register_action(|workspace, action: &ManageSkills, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.manage_skills(action, window, cx));
                    }
                })
                .register_action(|workspace, _: &OpenGlobalAgentsMdRules, window, cx| {
                    open_global_rules(workspace, window, cx);
                })
                .register_action(|workspace, _: &OpenProjectAgentsMdRules, window, cx| {
                    open_project_rules(workspace, window, cx);
                })
                .register_action(|workspace, _: &Follow, window, cx| {
                    workspace.follow(CollaboratorId::Agent, window, cx);
                })
                .register_action(|workspace, _: &OpenAgentDiff, window, cx| {
                    let thread = workspace
                        .panel::<AgentPanel>(cx)
                        .and_then(|panel| panel.read(cx).active_conversation_view().cloned())
                        .and_then(|conversation| {
                            conversation
                                .read(cx)
                                .root_thread_view()
                                .map(|r| r.read(cx).thread.clone())
                        });

                    if let Some(thread) = thread {
                        AgentDiffPane::deploy_in_workspace(thread, workspace, window, cx);
                    }
                })
                .register_action(|workspace, _: &ToggleOptionsMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_options_menu(&ToggleOptionsMenu, window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &ToggleNewThreadMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_new_thread_menu(&ToggleNewThreadMenu, window, cx);
                        });
                    }
                })
                .register_action(|_workspace, _: &ResetOnboarding, window, cx| {
                    window.dispatch_action(workspace::RestoreBanner.boxed_clone(), cx);
                    window.refresh();
                })
                .register_action(|workspace, _: &ResetTrialUpsell, _window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, _| {
                            panel
                                .new_user_onboarding_upsell_dismissed
                                .store(false, Ordering::Release);
                        });
                    }
                    OnboardingUpsell::set_dismissed(false, cx);
                })
                .register_action(|_workspace, _: &ResetTrialEndUpsell, _window, cx| {
                    TrialEndUpsell::set_dismissed(false, cx);
                })
                .register_action(|_workspace, _: &ResetFastModeWarnings, _window, cx| {
                    reset_fast_mode_warnings(cx);
                })
                .register_action(|workspace, _: &ResetAgentZoom, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.reset_agent_zoom(window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &CopyThreadToClipboard, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.copy_thread_to_clipboard(window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &LoadThreadFromClipboard, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.load_thread_from_clipboard(window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &ShowThreadMetadata, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.show_thread_metadata(&ShowThreadMetadata, window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &ShowAllSidebarThreadMetadata, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.show_all_sidebar_thread_metadata(
                                &ShowAllSidebarThreadMetadata,
                                window,
                                cx,
                            );
                        });
                    }
                })
                .register_action(|workspace, action: &ReviewBranchDiff, window, cx| {
                    let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                        return;
                    };

                    let mention_uri = MentionUri::GitDiff {
                        base_ref: action.base_ref.to_string(),
                    };
                    let diff_uri = mention_uri.to_uri().to_string();

                    let content_blocks = vec![
                        acp::ContentBlock::Text(acp::TextContent::new(
                            "Please review this branch diff carefully. Point out any issues, \
                             potential bugs, or improvement opportunities you find.\n\n"
                                .to_string(),
                        )),
                        acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                            acp::EmbeddedResourceResource::TextResourceContents(
                                acp::TextResourceContents::new(
                                    action.diff_text.to_string(),
                                    diff_uri,
                                ),
                            ),
                        )),
                    ];

                    workspace.focus_panel::<AgentPanel>(window, cx);

                    panel.update(cx, |panel, cx| {
                        panel.external_thread(
                            None,
                            None,
                            None,
                            None,
                            Some(AgentInitialContent::ContentBlock {
                                blocks: content_blocks,
                                auto_submit: true,
                            }),
                            true,
                            AgentThreadSource::GitPanel,
                            window,
                            cx,
                        );
                    });
                })
                .register_action(
                    |workspace, action: &ResolveConflictsWithAgent, window, cx| {
                        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                            return;
                        };

                        let content_blocks = build_conflict_resolution_prompt(&action.conflicts);

                        workspace.focus_panel::<AgentPanel>(window, cx);

                        panel.update(cx, |panel, cx| {
                            panel.external_thread(
                                None,
                                None,
                                None,
                                None,
                                Some(AgentInitialContent::ContentBlock {
                                    blocks: content_blocks,
                                    auto_submit: true,
                                }),
                                true,
                                AgentThreadSource::GitPanel,
                                window,
                                cx,
                            );
                        });
                    },
                )
                .register_action(
                    |workspace, action: &ResolveConflictedFilesWithAgent, window, cx| {
                        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                            return;
                        };

                        let content_blocks =
                            build_conflicted_files_resolution_prompt(&action.conflicted_file_paths);

                        workspace.focus_panel::<AgentPanel>(window, cx);

                        panel.update(cx, |panel, cx| {
                            panel.external_thread(
                                None,
                                None,
                                None,
                                None,
                                Some(AgentInitialContent::ContentBlock {
                                    blocks: content_blocks,
                                    auto_submit: true,
                                }),
                                true,
                                AgentThreadSource::GitPanel,
                                window,
                                cx,
                            );
                        });
                    },
                )
                .register_action(
                    |workspace: &mut Workspace, _: &AddSelectionToThread, window, cx| {
                        let active_editor = workspace
                            .active_item(cx)
                            .and_then(|item| item.act_as::<Editor>(cx));
                        let has_editor_selection = active_editor.is_some_and(|editor| {
                            editor.update(cx, |editor, cx| {
                                editor.has_non_empty_selection(&editor.display_snapshot(cx))
                            })
                        });

                        let has_terminal_selection = workspace
                            .active_item(cx)
                            .and_then(|item| item.act_as::<TerminalView>(cx))
                            .is_some_and(|terminal_view| {
                                terminal_view
                                    .read(cx)
                                    .terminal()
                                    .read(cx)
                                    .last_content
                                    .selection_text
                                    .as_ref()
                                    .is_some_and(|text| !text.is_empty())
                            });

                        let has_terminal_panel_selection =
                            workspace.panel::<TerminalPanel>(cx).is_some_and(|panel| {
                                let position = match TerminalSettings::get_global(cx).dock {
                                    TerminalDockPosition::Left => DockPosition::Left,
                                    TerminalDockPosition::Bottom => DockPosition::Bottom,
                                    TerminalDockPosition::Right => DockPosition::Right,
                                };
                                let dock_is_open =
                                    workspace.dock_at_position(position).read(cx).is_open();
                                dock_is_open && !panel.read(cx).terminal_selections(cx).is_empty()
                            });

                        if !has_editor_selection
                            && !has_terminal_selection
                            && !has_terminal_panel_selection
                        {
                            return;
                        }

                        let Some(agent_panel) = workspace.panel::<AgentPanel>(cx) else {
                            return;
                        };

                        let source = AgentContextSource::from_focused(workspace, window, cx);
                        let source = source.or_else(|| {
                            let cached = agent_panel.read(cx).last_context_source.clone()?;
                            cached.exists(workspace, cx).then_some(cached)
                        });
                        let source =
                            source.or_else(|| AgentContextSource::from_active(workspace, cx));

                        let Some(source) = source else {
                            return;
                        };

                        let Some(selection) = source.read_selection(workspace, true, cx) else {
                            return;
                        };

                        if !agent_panel.focus_handle(cx).contains_focused(window, cx) {
                            workspace.toggle_panel_focus::<AgentPanel>(window, cx);
                        }

                        agent_panel.update(cx, |panel, cx| {
                            panel.last_context_source = Some(source);
                            cx.defer_in(window, move |panel, window, cx| {
                                if let Some(conversation_view) = panel.active_conversation_view() {
                                    conversation_view.update(cx, |conversation_view, cx| {
                                        conversation_view.insert_selection(selection, window, cx);
                                    });
                                } else if let Some(terminal_id) = panel.active_terminal_id()
                                    && let Some(agent_terminal) = panel.terminals.get(&terminal_id)
                                {
                                    // Resolve mentions against the cwd: live cwd, else spawn dir.
                                    let working_directory = agent_terminal
                                        .view
                                        .read(cx)
                                        .terminal()
                                        .read(cx)
                                        .working_directory()
                                        .or_else(|| agent_terminal.working_directory.clone());
                                    let text = format_selection_for_terminal(
                                        &selection,
                                        &panel.project,
                                        working_directory.as_deref(),
                                        cx,
                                    );
                                    if !text.is_empty() {
                                        let view = agent_terminal.view.clone();
                                        view.update(cx, |view, cx| {
                                            view.terminal().update(cx, |terminal, _| {
                                                terminal.paste(&text);
                                            });
                                            window.focus(&view.focus_handle(cx), cx);
                                        });
                                    }
                                }
                            });
                        });
                    },
                );
        },
    )
    .detach();
}

fn format_selection_for_terminal(
    selection: &AgentContextSelection,
    project: &Entity<Project>,
    working_directory: Option<&std::path::Path>,
    cx: &App,
) -> String {
    match selection {
        AgentContextSelection::Editor(ranges) => {
            let path_style = project.read(cx).path_style(cx);
            let mut parts: Vec<String> = Vec::new();
            for (buffer, range) in ranges {
                let buffer = buffer.read(cx);
                let Some(project_path) = buffer.project_path(cx) else {
                    continue;
                };
                let snapshot = buffer.snapshot();
                let point_range = range.to_point(&snapshot);
                let line_range = point_range.start.row..=point_range.end.row;
                let path = mention_path_for_terminal(
                    project,
                    &project_path,
                    working_directory,
                    path_style,
                    cx,
                );
                parts.push(format!("{path}{}", line_range_suffix(&line_range)));
            }
            if parts.is_empty() {
                String::new()
            } else {
                // Trailing space so the mention doesn't fuse with the next input.
                format!("{} ", parts.join(" "))
            }
        }
        AgentContextSelection::Terminal(texts) => texts.join("\n"),
    }
}

/// Path for a terminal mention: relative to the terminal cwd if possible, else absolute.
fn mention_path_for_terminal(
    project: &Entity<Project>,
    project_path: &ProjectPath,
    working_directory: Option<&std::path::Path>,
    path_style: util::paths::PathStyle,
    cx: &App,
) -> String {
    let abs_path = project.read(cx).absolute_path(project_path, cx);
    match (abs_path, working_directory) {
        (Some(abs_path), Some(working_directory)) => path_style
            .strip_prefix(&abs_path, working_directory)
            .map(|relative| relative.display(path_style).into_owned())
            .unwrap_or_else(|| abs_path.to_string_lossy().into_owned()),
        (Some(abs_path), None) => abs_path.to_string_lossy().into_owned(),
        (None, _) => project_path.path.display(path_style).into_owned(),
    }
}

fn conflict_resource_block(conflict: &ConflictContent) -> acp::ContentBlock {
    let mention_uri = MentionUri::MergeConflict {
        file_path: conflict.file_path.clone(),
    };
    acp::ContentBlock::Resource(acp::EmbeddedResource::new(
        acp::EmbeddedResourceResource::TextResourceContents(acp::TextResourceContents::new(
            conflict.conflict_text.clone(),
            mention_uri.to_uri().to_string(),
        )),
    ))
}

fn build_conflict_resolution_prompt(conflicts: &[ConflictContent]) -> Vec<acp::ContentBlock> {
    if conflicts.is_empty() {
        return Vec::new();
    }

    let mut blocks = Vec::new();

    if conflicts.len() == 1 {
        let conflict = &conflicts[0];

        blocks.push(acp::ContentBlock::Text(acp::TextContent::new(
            "Please resolve the following merge conflict in ",
        )));
        let mention = MentionUri::File {
            abs_path: PathBuf::from(conflict.file_path.clone()),
        };
        blocks.push(acp::ContentBlock::ResourceLink(acp::ResourceLink::new(
            mention.name(),
            mention.to_uri(),
        )));

        blocks.push(acp::ContentBlock::Text(acp::TextContent::new(
            indoc::formatdoc!(
                "\nThe conflict is between branch `{ours}` (ours) and `{theirs}` (theirs).

                Analyze both versions carefully and resolve the conflict by editing \
                the file directly. Choose the resolution that best preserves the intent \
                of both changes, or combine them if appropriate.

                ",
                ours = conflict.ours_branch_name,
                theirs = conflict.theirs_branch_name,
            ),
        )));
    } else {
        let n = conflicts.len();
        let unique_files: HashSet<&str> = conflicts.iter().map(|c| c.file_path.as_str()).collect();
        let ours = &conflicts[0].ours_branch_name;
        let theirs = &conflicts[0].theirs_branch_name;
        blocks.push(acp::ContentBlock::Text(acp::TextContent::new(
            indoc::formatdoc!(
                "Please resolve all {n} merge conflicts below.

                The conflicts are between branch `{ours}` (ours) and `{theirs}` (theirs).

                For each conflict, analyze both versions carefully and resolve them \
                by editing the file{suffix} directly. Choose resolutions that best preserve \
                the intent of both changes, or combine them if appropriate.

                ",
                suffix = if unique_files.len() > 1 { "s" } else { "" },
            ),
        )));
    }

    for conflict in conflicts {
        blocks.push(conflict_resource_block(conflict));
    }

    blocks
}

fn build_conflicted_files_resolution_prompt(
    conflicted_file_paths: &[String],
) -> Vec<acp::ContentBlock> {
    if conflicted_file_paths.is_empty() {
        return Vec::new();
    }

    let instruction = indoc::indoc!(
        "The following files have unresolved merge conflicts. Please open each \
         file, find the conflict markers (`<<<<<<<` / `=======` / `>>>>>>>`), \
         and resolve every conflict by editing the files directly.

         Choose resolutions that best preserve the intent of both changes, \
         or combine them if appropriate.

         Files with conflicts:
         ",
    );

    let mut content = vec![acp::ContentBlock::Text(acp::TextContent::new(instruction))];
    for path in conflicted_file_paths {
        let mention = MentionUri::File {
            abs_path: PathBuf::from(path),
        };
        content.push(acp::ContentBlock::ResourceLink(acp::ResourceLink::new(
            mention.name(),
            mention.to_uri(),
        )));
        content.push(acp::ContentBlock::Text(acp::TextContent::new("\n")));
    }
    content
}

fn format_timestamp_human(dt: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*dt);

    let relative = if duration.num_seconds() < 0 {
        "in the future".to_string()
    } else if duration.num_seconds() < 60 {
        let seconds = duration.num_seconds();
        format!("{seconds} seconds ago")
    } else if duration.num_minutes() < 60 {
        let minutes = duration.num_minutes();
        format!("{minutes} minutes ago")
    } else if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        format!("{hours} hours ago")
    } else {
        let days = duration.num_days();
        format!("{days} days ago")
    };

    format!("{} ({})", dt.to_rfc3339(), relative)
}

/// Used for `dev: show thread metadata` action
fn thread_metadata_to_debug_json(
    metadata: &crate::thread_metadata_store::ThreadMetadata,
) -> serde_json::Value {
    serde_json::json!({
        "thread_id": metadata.thread_id,
        "session_id": metadata.session_id.as_ref().map(|s| s.0.to_string()),
        "agent_id": metadata.agent_id.0.to_string(),
        "title": metadata.title.as_ref().map(|t| t.to_string()),
        "title_override": metadata.title_override.as_ref().map(|t| t.to_string()),
        "updated_at": format_timestamp_human(&metadata.updated_at),
        "created_at": metadata.created_at.as_ref().map(format_timestamp_human),
        "interacted_at": metadata.interacted_at.as_ref().map(format_timestamp_human),
        "worktree_paths": format!("{:?}", metadata.worktree_paths),
        "archived": metadata.archived,
    })
}

/// Optional parameters for `AgentPanel::create_thread_with_options`. All
/// fields default to the panel's current selection so the agent tool only
/// needs to override what it actually cares about.
#[derive(Default)]
pub struct CreateThreadOptions {
    /// Title to assign to the new thread up front.
    pub title: Option<SharedString>,
    /// Initial content to populate in the thread (optionally auto-submitted).
    pub initial_content: Option<AgentInitialContent>,
    /// Agent to use. Defaults to the panel's selected agent.
    pub agent: Option<Agent>,
    /// Model override, as `provider/model-id`. Only applied when the thread
    /// uses the native Zed agent.
    pub model: Option<String>,
    /// Working directories to attach to the new thread (e.g., the path of a
    /// freshly-created sibling worktree). When `None`, the thread inherits
    /// the project's default path list.
    pub work_dirs: Option<PathList>,
}

pub(crate) struct AgentThread {
    conversation_view: Entity<ConversationView>,
}

struct AgentTerminal {
    view: Entity<TerminalView>,
    title_editor: Option<Entity<Editor>>,
    title_editor_initial_title: Option<String>,
    title_editor_subscription: Option<Subscription>,
    last_known_title: String,
    last_known_terminal_title: String,
    last_observed_program: Option<String>,
    working_directory: Option<PathBuf>,
    created_at: DateTime<Utc>,
    has_notification: bool,
    notification_windows: Vec<WindowHandle<AgentNotification>>,
    notification_subscriptions: Vec<Subscription>,
    _subscriptions: Vec<Subscription>,
}

impl AgentTerminal {
    fn terminal_title_for_view(view: &TerminalView, cx: &App) -> SharedString {
        let terminal = view.terminal().read(cx);
        if terminal.breadcrumb_text.is_empty() {
            let title = terminal.title(true);
            if title == "Terminal" {
                SharedString::from("")
            } else {
                title.into()
            }
        } else {
            terminal.breadcrumb_text.clone().into()
        }
    }

    fn current_terminal_title(&self, cx: &App) -> SharedString {
        let view = self.view.read(cx);
        Self::terminal_title_for_view(view, cx)
    }

    fn terminal_title(&self, cx: &App) -> SharedString {
        let title = self.current_terminal_title(cx);
        if title.is_empty() && !self.last_known_terminal_title.is_empty() {
            SharedString::from(self.last_known_terminal_title.clone())
        } else {
            title
        }
    }

    fn title(&self, cx: &App) -> SharedString {
        let terminal_title = self.terminal_title(cx);
        let custom_title = self.custom_title(cx);
        compose_terminal_thread_title(
            terminal_title.as_ref(),
            custom_title.as_ref().map(|title| title.as_ref()),
        )
    }

    fn editable_title(&self, cx: &App) -> SharedString {
        if let Some(custom_title) = self.custom_title(cx) {
            custom_title
        } else {
            let terminal_title = self.terminal_title(cx);
            SharedString::from(terminal_title_without_prefix(terminal_title.as_ref()).to_string())
        }
    }

    fn refresh_title(&mut self, cx: &mut App) -> bool {
        let terminal_title = self.current_terminal_title(cx);
        if !terminal_title.is_empty() {
            self.last_known_terminal_title = terminal_title.to_string();
        }

        let title = self.title(cx);
        let changed = self.last_known_title != title.as_ref();
        if changed {
            self.last_known_title = title.to_string();
        }
        changed
    }

    fn refresh_metadata(&mut self, cx: &mut App) -> bool {
        let title_changed = self.refresh_title(cx);
        let current_working_directory = self.view.read(cx).terminal().read(cx).working_directory();
        let working_directory_changed = current_working_directory
            .as_ref()
            .is_some_and(|current| self.working_directory.as_ref() != Some(current));
        if working_directory_changed {
            self.working_directory = current_working_directory;
        }
        title_changed || working_directory_changed
    }

    fn custom_title(&self, cx: &App) -> Option<SharedString> {
        self.view.read(cx).custom_title().map(SharedString::from)
    }

    fn report_started_terminal_program(
        &mut self,
        terminal_id: TerminalId,
        source: AgentThreadSource,
        cx: &App,
    ) {
        let current_program = self
            .view
            .read(cx)
            .terminal()
            .read(cx)
            .foreground_process_command_name();

        if let Some(program) =
            terminal_program_to_report(&mut self.last_observed_program, current_program)
        {
            telemetry::event!(
                "Agent Terminal Program Started",
                agent = TERMINAL_AGENT_TELEMETRY_ID,
                terminal_id = terminal_id.to_key_string(),
                program = program,
                source = source.as_str(),
                side = crate::agent_sidebar_side(cx),
                thread_location = "current_worktree",
            );
        }
    }
}

enum BaseView {
    Uninitialized,
    AgentThread {
        conversation_view: Entity<ConversationView>,
    },
    Terminal {
        terminal_id: TerminalId,
    },
}

impl From<AgentThread> for BaseView {
    fn from(thread: AgentThread) -> Self {
        BaseView::AgentThread {
            conversation_view: thread.conversation_view,
        }
    }
}

enum OverlayView {
    Configuration,
}

enum VisibleSurface<'a> {
    Uninitialized,
    AgentThread(&'a Entity<ConversationView>),
    Terminal(&'a Entity<TerminalView>),
    Configuration(Option<&'a Entity<AgentConfiguration>>),
}

enum WhichFontSize {
    AgentFont,
    None,
}

impl BaseView {
    pub fn which_font_size_used(&self) -> WhichFontSize {
        match self {
            BaseView::AgentThread { .. } => WhichFontSize::AgentFont,
            BaseView::Terminal { .. } | BaseView::Uninitialized => WhichFontSize::None,
        }
    }
}

impl OverlayView {
    pub fn which_font_size_used(&self) -> WhichFontSize {
        match self {
            OverlayView::Configuration => WhichFontSize::None,
        }
    }
}

pub struct AgentPanel {
    workspace: WeakEntity<Workspace>,
    /// Workspace id is used as a database key
    workspace_id: Option<WorkspaceId>,
    user_store: Entity<UserStore>,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    connection_store: Entity<AgentConnectionStore>,
    context_server_registry: Entity<ContextServerRegistry>,
    configuration: Option<Entity<AgentConfiguration>>,
    configuration_subscription: Option<Subscription>,
    focus_handle: FocusHandle,
    base_view: BaseView,
    last_created_entry_kind: AgentPanelEntryKind,
    overlay_view: Option<OverlayView>,
    draft_thread: Option<Entity<ConversationView>>,
    retained_threads: HashMap<ThreadId, Entity<ConversationView>>,
    terminals: HashMap<TerminalId, AgentTerminal>,
    pending_terminal_spawn: Option<TerminalId>,
    new_thread_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_panel_menu_handle: PopoverMenuHandle<ContextMenu>,
    _extension_subscription: Option<Subscription>,
    _project_subscription: Subscription,
    zoomed: bool,
    pending_serialization: Option<Task<Result<()>>>,
    new_user_onboarding: Entity<AgentPanelOnboarding>,
    new_user_onboarding_upsell_dismissed: AtomicBool,
    selected_agent: Agent,
    _thread_view_subscription: Option<Subscription>,
    _active_thread_focus_subscription: Option<Subscription>,
    _base_view_observation: Option<Subscription>,
    _draft_editor_observation: Option<Subscription>,
    _active_draft_reclaim_observation: Option<Subscription>,
    _thread_metadata_store_subscription: Subscription,
    last_context_source: Option<AgentContextSource>,

    is_active: bool,
}

impl AgentPanel {
    fn serialize(&mut self, cx: &mut App) {
        let Some(workspace_id) = self.workspace_id else {
            return;
        };

        let selected_agent = self.selected_agent.clone();
        let last_created_entry_kind = self.last_created_entry_kind;
        let last_active_terminal_id = self
            .active_terminal_id()
            .map(|terminal_id| terminal_id.to_key_string());

        let last_active_thread = if last_active_terminal_id.is_some() {
            None
        } else {
            let is_draft_active = self.active_thread_is_draft(cx);
            let active_thread_id = self.active_thread_id(cx);
            let active_thread_agent = self
                .active_conversation_view()
                .map(|cv| cv.read(cx).agent_key().clone())
                .unwrap_or_else(|| self.selected_agent.clone());
            self.active_agent_thread(cx)
                .map(|thread| {
                    let thread = thread.read(cx);

                    let title = thread.title();
                    let work_dirs = thread.work_dirs().cloned();
                    SerializedActiveThread {
                        session_id: (!is_draft_active).then(|| thread.session_id().0.to_string()),
                        thread_id: active_thread_id,
                        agent_type: active_thread_agent.clone(),
                        title: title.map(|t| t.to_string()),
                        work_dirs: work_dirs.map(|dirs| dirs.serialize()),
                    }
                })
                .or_else(|| {
                    // The active view may be in `Loading` or `LoadError` — for
                    // example, while a restored thread is waiting for a custom
                    // agent to finish registering. Without this fallback, a
                    // stray `serialize()` triggered during that window would
                    // write `session_id=None` and wipe the restored session
                    if is_draft_active {
                        return None;
                    }
                    let conversation_view = self.active_conversation_view()?;
                    let session_id = conversation_view.read(cx).root_session_id.clone()?;
                    let metadata = ThreadMetadataStore::try_global(cx)
                        .and_then(|store| store.read(cx).entry_by_session(&session_id).cloned());
                    Some(SerializedActiveThread {
                        session_id: Some(session_id.0.to_string()),
                        thread_id: active_thread_id,
                        agent_type: active_thread_agent.clone(),
                        title: metadata
                            .as_ref()
                            .and_then(|m| m.title.as_ref())
                            .map(|t| t.to_string()),
                        work_dirs: metadata.map(|m| m.folder_paths().serialize()),
                    })
                })
        };

        let new_draft_thread_id = self
            .draft_thread
            .as_ref()
            .map(|draft| draft.read(cx).thread_id);

        let kvp = KeyValueStore::global(cx);
        self.pending_serialization = Some(cx.background_spawn(async move {
            save_serialized_panel(
                workspace_id,
                SerializedAgentPanel {
                    selected_agent: Some(selected_agent),
                    last_created_entry_kind,
                    last_active_thread,
                    last_active_terminal_id,
                    new_draft_thread_id,
                },
                kvp,
            )
            .await?;
            anyhow::Ok(())
        }));
    }

    pub fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        let kvp = cx.update(|_window, cx| KeyValueStore::global(cx)).ok();
        cx.spawn(async move |cx| {
            let workspace_id = workspace
                .read_with(cx, |workspace, _| workspace.database_id())
                .ok()
                .flatten();

            let (serialized_panel, global_last_used_agent, global_last_created_entry_kind) = cx
                .background_spawn(async move {
                    match kvp {
                        Some(kvp) => {
                            let panel = workspace_id
                                .and_then(|id| read_serialized_panel(id, &kvp))
                                .or_else(|| read_legacy_serialized_panel(&kvp));
                            let global_agent = read_global_last_used_agent(&kvp);
                            let global_entry_kind = read_global_last_created_entry_kind(&kvp);
                            (panel, global_agent, global_entry_kind)
                        }
                        None => (None, None, None),
                    }
                })
                .await;

            let has_open_project = workspace
                .read_with(cx, |workspace, cx| !workspace.root_paths(cx).is_empty())
                .unwrap_or(false);
            let terminal_id_to_restore = if has_open_project {
                serialized_panel
                    .as_ref()
                    .and_then(|panel| panel.last_active_terminal_id.as_deref())
                    .and_then(|terminal_id| {
                        match TerminalId::from_key_string(terminal_id) {
                            Ok(terminal_id) => Some(terminal_id),
                            Err(error) => {
                                log::warn!("failed to parse last active terminal id: {error}");
                                None
                            }
                        }
                    })
            } else {
                None
            };
            let terminal_to_restore = if let Some(terminal_id) = terminal_id_to_restore {
                match cx.update(|_window, cx| {
                    TerminalThreadMetadataStore::try_global(cx).map(|store| {
                        let reload_task = store.read(cx).reload_task();
                        (store, reload_task)
                    })
                }) {
                    Ok(Some((store, reload_task))) => {
                        reload_task.await;
                        match store
                            .read_with(cx, |store, _cx| store.entry(terminal_id).cloned())
                        {
                            Some(metadata) => Some(metadata),
                            None => {
                                log::info!(
                                    "last active terminal is missing, skipping restoration"
                                );
                                None
                            }
                        }
                    }
                    Ok(None) => {
                        log::warn!("failed to restore active terminal: metadata store missing");
                        None
                    }
                    Err(err) => {
                        log::warn!("failed to access terminal metadata store: {err}");
                        None
                    }
                }
            } else {
                None
            };

            let thread_to_restore = if has_open_project && terminal_to_restore.is_none() {
                if let Some(info) = serialized_panel
                    .as_ref()
                    .and_then(|panel| panel.last_active_thread.as_ref())
                {
                    match cx.update(|_window, cx| {
                        ThreadMetadataStore::try_global(cx).map(|store| {
                            let reload_task = store.read(cx).reload_task();
                            (store, reload_task)
                        })
                    }) {
                        Ok(Some((store, reload_task))) => {
                            reload_task.await;
                            let thread_id = store.read_with(cx, |store, _cx| {
                                let primary = info.thread_id.and_then(|tid| store.entry(tid));
                                let fallback = info.session_id.as_ref().and_then(|sid| {
                                    store.entry_by_session(&acp::SessionId::new(sid.clone()))
                                });
                                primary
                                    .or(fallback)
                                    .filter(|entry| !entry.archived)
                                    .map(|entry| entry.thread_id)
                            });
                            match thread_id {
                                Some(thread_id) => Some((info, thread_id)),
                                None => {
                                    log::info!(
                                        "last active thread is archived or missing, skipping restoration"
                                    );
                                    None
                                }
                            }
                        }
                        Ok(None) => {
                            log::warn!("failed to restore active thread: metadata store missing");
                            None
                        }
                        Err(err) => {
                            log::warn!("failed to access thread metadata store: {err}");
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let panel = workspace.update_in(cx, |workspace, window, cx| {
                let panel = cx.new(|cx| Self::new(workspace, window, cx));

                panel.update(cx, |panel, cx| {
                    let is_via_collab = panel.project.read(cx).is_via_collab();
                    // Collab workspaces only support NativeAgent; clamp any
                    // non-native choice so `set_active` can't bypass the
                    // collab guard in `external_thread`.
                    let clamp = |agent: Agent| {
                        if is_via_collab && !agent.is_native() {
                            Agent::NativeAgent
                        } else {
                            agent
                        }
                    };
                    let global_fallback =
                        global_last_used_agent.filter(|agent| !is_via_collab || agent.is_native());

                    if let Some(serialized_panel) = &serialized_panel {
                        panel.last_created_entry_kind = serialized_panel.last_created_entry_kind;
                    } else if let Some(entry_kind) = global_last_created_entry_kind {
                        panel.last_created_entry_kind = entry_kind;
                    }

                    // The thread being restored may have been bound to an
                    // agent different from the panel's last selected one
                    // (e.g. a draft created while a different agent was
                    // active). When restoring a thread, prefer its agent
                    // so the draft survives reload bound to the right
                    // backend; otherwise fall back to the serialized
                    // selection, then the global last-used agent.
                    let initial_agent = match &thread_to_restore {
                        Some((info, _)) => Some(clamp(info.agent_type.clone())),
                        None => serialized_panel
                            .as_ref()
                            .and_then(|p| p.selected_agent.clone())
                            .map(clamp)
                            .or(global_fallback),
                    };
                    if let Some(agent) = initial_agent {
                        panel.selected_agent = agent;
                    }

                    if let Some(metadata) = terminal_to_restore {
                        panel.restore_terminal_for_panel_load(
                            metadata,
                            false,
                            AgentThreadSource::AgentPanel,
                            Some(workspace),
                            window,
                            cx,
                        );
                    } else if let Some((info, thread_id)) = thread_to_restore {
                        let agent = panel.selected_agent.clone();
                        panel.load_agent_thread(
                            agent,
                            thread_id,
                            info.work_dirs.as_ref().map(PathList::deserialize),
                            info.title.clone().map(Into::into),
                            false,
                            AgentThreadSource::AgentPanel,
                            window,
                            cx,
                        );
                    }
                    if let Some(new_draft_thread_id) = serialized_panel
                        .as_ref()
                        .and_then(|p| p.new_draft_thread_id)
                    {
                        panel.restore_new_draft(new_draft_thread_id, window, cx);
                    }
                    cx.notify();
                });

                panel
            })?;

            Ok(panel)
        })
    }

    pub(crate) fn new(workspace: &Workspace, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fs = workspace.app_state().fs.clone();
        let user_store = workspace.app_state().user_store.clone();
        let project = workspace.project();
        let language_registry = project.read(cx).languages().clone();
        let client = workspace.client().clone();
        let workspace_id = workspace.database_id();
        let workspace = workspace.weak_handle();

        let context_server_registry =
            cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));

        let thread_store = ThreadStore::global(cx);

        let base_view = BaseView::Uninitialized;

        let weak_panel = cx.entity().downgrade();
        let onboarding = cx.new(|cx| {
            AgentPanelOnboarding::new(
                user_store.clone(),
                client,
                move |_window, cx| {
                    weak_panel
                        .update(cx, |panel, cx| {
                            panel.dismiss_ai_onboarding(cx);
                        })
                        .ok();
                },
                cx,
            )
        });

        // Subscribe to extension events to sync agent servers when extensions change
        let extension_subscription = ExtensionStore::try_global(cx).map(|store| {
            cx.subscribe(&store, |this, _source, event, cx| match event {
                extension_host::Event::ExtensionUninstalled(id) => {
                    this.migrate_agent_server_from_extensions(id.clone(), cx);
                }
                _ => {}
            })
        });

        let connection_store = cx.new(|cx| AgentConnectionStore::new(project.clone(), cx));
        let _project_subscription =
            cx.subscribe(&project, |this, _project, event, cx| match event {
                project::Event::WorktreeAdded(_)
                | project::Event::WorktreeRemoved(_)
                | project::Event::WorktreeOrderChanged
                | project::Event::WorktreePathsChanged { .. } => {
                    this.ensure_native_agent_connection(cx);
                    this.update_thread_work_dirs(cx);
                    this.persist_all_terminal_metadata(cx);
                    cx.notify();
                }
                _ => {}
            });

        let _thread_metadata_store_subscription = cx.subscribe(
            &ThreadMetadataStore::global(cx),
            |this, _store, event, cx| {
                let ThreadMetadataStoreEvent::ThreadArchived(thread_id) = event;
                if this.retained_threads.remove(thread_id).is_some() {
                    cx.notify();
                }
            },
        );

        cx.on_release(|this, cx| {
            this.dismiss_all_terminal_notifications(cx);
        })
        .detach();

        let panel = Self {
            workspace_id,
            base_view,
            last_created_entry_kind: AgentPanelEntryKind::Thread,
            overlay_view: None,
            workspace,
            user_store,
            project: project.clone(),
            fs: fs.clone(),
            language_registry,
            connection_store,
            configuration: None,
            configuration_subscription: None,
            focus_handle: cx.focus_handle(),
            context_server_registry,
            draft_thread: None,
            retained_threads: HashMap::default(),
            terminals: HashMap::default(),
            pending_terminal_spawn: None,
            new_thread_menu_handle: PopoverMenuHandle::default(),
            agent_panel_menu_handle: PopoverMenuHandle::default(),

            _extension_subscription: extension_subscription,
            _project_subscription,
            zoomed: false,
            pending_serialization: None,
            new_user_onboarding: onboarding,
            thread_store,
            selected_agent: Agent::default(),
            _thread_view_subscription: None,
            _active_thread_focus_subscription: None,
            new_user_onboarding_upsell_dismissed: AtomicBool::new(OnboardingUpsell::dismissed(cx)),
            _base_view_observation: None,
            _draft_editor_observation: None,
            _active_draft_reclaim_observation: None,
            _thread_metadata_store_subscription,
            last_context_source: None,
            is_active: false,
        };

        panel.ensure_native_agent_connection(cx);
        panel
    }

    pub fn toggle_focus(
        workspace: &mut Workspace,
        _: &ToggleFocus,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if workspace
            .panel::<Self>(cx)
            .is_some_and(|panel| panel.read(cx).enabled(cx))
        {
            workspace.toggle_panel_focus::<Self>(window, cx);
        }
    }

    pub fn focus(
        workspace: &mut Workspace,
        _: &FocusAgent,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if workspace
            .panel::<Self>(cx)
            .is_some_and(|panel| panel.read(cx).enabled(cx))
        {
            workspace.focus_panel::<Self>(window, cx);
        }
    }

    pub fn toggle(
        workspace: &mut Workspace,
        _: &Toggle,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if workspace
            .panel::<Self>(cx)
            .is_some_and(|panel| panel.read(cx).enabled(cx))
        {
            if !workspace.toggle_panel_focus::<Self>(window, cx) {
                workspace.close_panel::<Self>(window, cx);
            }
        }
    }

    pub fn thread_store(&self) -> &Entity<ThreadStore> {
        &self.thread_store
    }

    pub fn connection_store(&self) -> &Entity<AgentConnectionStore> {
        &self.connection_store
    }

    pub fn selected_agent(&self, cx: &App) -> Agent {
        if self.project.read(cx).is_via_collab() {
            Agent::NativeAgent
        } else {
            self.selected_agent.clone()
        }
    }

    pub fn open_thread(
        &mut self,
        session_id: acp::SessionId,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Share links / clipboard imports enter with only a session id. If
        // this machine already has a metadata row for the session, route
        // through the normal thread-id path.
        let existing_thread_id = ThreadMetadataStore::try_global(cx).and_then(|store| {
            store
                .read(cx)
                .entry_by_session(&session_id)
                .map(|m| m.thread_id)
        });
        if let Some(thread_id) = existing_thread_id {
            self.load_agent_thread(
                crate::Agent::NativeAgent,
                thread_id,
                work_dirs,
                title,
                true,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        } else {
            self.external_thread_by_session(
                crate::Agent::NativeAgent,
                session_id,
                work_dirs,
                title,
                true,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        }
    }

    fn external_thread_by_session(
        &mut self,
        agent: Agent,
        session_id: acp::SessionId,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let thread = self.create_agent_thread_with_server_for_external_session(
            agent, None, session_id, work_dirs, title, None, source, window, cx,
        );
        self.set_base_view(thread.into(), focus, window, cx);
    }

    pub(crate) fn context_server_registry(&self) -> &Entity<ContextServerRegistry> {
        &self.context_server_registry
    }

    pub fn is_visible(workspace: &Entity<Workspace>, cx: &App) -> bool {
        let workspace_read = workspace.read(cx);

        workspace_read
            .panel::<AgentPanel>(cx)
            .map(|panel| {
                let panel_id = Entity::entity_id(&panel);

                workspace_read.all_docks().iter().any(|dock| {
                    dock.read(cx)
                        .visible_panel()
                        .is_some_and(|visible_panel| visible_panel.panel_id() == panel_id)
                })
            })
            .unwrap_or(false)
    }

    /// Clear the active view, retaining any running thread in the background.
    pub fn clear_base_view(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let old_view = std::mem::replace(&mut self.base_view, BaseView::Uninitialized);
        self.retain_running_thread(old_view, cx);
        self.clear_overlay_state();
        self.activate_draft(false, AgentThreadSource::AgentPanel, window, cx);
        self.serialize(cx);
        cx.emit(AgentPanelEvent::ActiveViewChanged);
        cx.notify();
    }

    pub fn new_thread(&mut self, _action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        if !self.has_open_project(cx) {
            return;
        }

        self.new_thread_with_workspace(None, window, cx);
    }

    fn new_thread_with_workspace(
        &mut self,
        workspace: Option<&Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.should_create_terminal_for_new_entry(cx) {
            self.new_terminal(workspace, AgentThreadSource::AgentPanel, window, cx);
        } else {
            self.activate_new_thread(true, AgentThreadSource::AgentPanel, window, cx);
        }
    }

    pub fn activate_new_thread(
        &mut self,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_open_project(cx) {
            return;
        }

        self.set_last_created_entry_kind_from_user_action(AgentPanelEntryKind::Thread, cx);

        // If the user is viewing a *parked* draft and the ephemeral
        // new-draft slot is occupied, pressing `+` should just focus the
        // ephemeral draft — not park it and create yet another empty one.
        // This matches the mental model of `+` as "go to my new-thread
        // slot". The parked draft will be put back into `retained_threads`
        // by `set_base_view`'s `retain_running_thread` call.
        if let Some(draft) = self.draft_thread.clone()
            && self.active_thread_is_draft(cx)
            && !self.active_view_is_new_draft(cx)
            && *draft.read(cx).agent_key() == self.selected_agent
        {
            self.set_base_view(
                BaseView::AgentThread {
                    conversation_view: draft,
                },
                focus,
                window,
                cx,
            );
            return;
        }

        if let Some(draft) = self.draft_thread.clone() {
            if self.draft_has_content(&draft, cx) {
                let draft_id = draft.read(cx).thread_id;
                self.draft_thread = None;
                self._draft_editor_observation = None;
                self.retained_threads.insert(draft_id, draft);
            } else if *draft.read(cx).agent_key() != self.selected_agent {
                let old_draft_id = draft.read(cx).thread_id;
                ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                    store.delete(old_draft_id, cx);
                });
                self.draft_thread = None;
                self._draft_editor_observation = None;
            }
        }
        self.activate_draft(focus, source, window, cx);
    }

    fn draft_has_content(&self, draft: &Entity<ConversationView>, cx: &App) -> bool {
        let cv = draft.read(cx);
        if let Some(thread_view) = cv.active_thread() {
            let text = thread_view.read(cx).message_editor.read(cx).text(cx);
            if !text.trim().is_empty() {
                return true;
            }
        }
        if let Some(acp_thread) = cv.root_thread(cx) {
            let thread = acp_thread.read(cx);
            if !thread.is_draft_thread() {
                return true;
            }
            if thread
                .draft_prompt()
                .is_some_and(|blocks| !blocks.is_empty())
            {
                return true;
            }
        }
        false
    }

    /// Reattaches the panel's new-draft slot to the persisted `thread_id`,
    /// seeding the editor with any prompt text from the draft-prompt kvp
    /// store.
    ///
    /// If the active view already holds this thread — because the user's
    /// last-active thread was the new-draft itself — we reuse that
    /// ConversationView instead of building a second one.
    fn restore_new_draft(
        &mut self,
        thread_id: ThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_open_project(cx) {
            return;
        }

        let active_matching = match &self.base_view {
            BaseView::AgentThread { conversation_view }
                if conversation_view.read(cx).thread_id == thread_id =>
            {
                Some(conversation_view.clone())
            }
            _ => None,
        };
        if let Some(conversation_view) = active_matching {
            self.observe_draft_editor(&conversation_view, cx);
            self.draft_thread = Some(conversation_view);
            return;
        }

        let Some(metadata) = ThreadMetadataStore::try_global(cx)
            .and_then(|store| store.read(cx).entry(thread_id).cloned())
            .filter(|m| m.is_draft())
        else {
            return;
        };

        let agent = if self.project.read(cx).is_via_collab() {
            Agent::NativeAgent
        } else {
            Agent::from(metadata.agent_id.clone())
        };
        let initial_content = crate::draft_prompt_store::read(thread_id, cx).map(|blocks| {
            AgentInitialContent::ContentBlock {
                blocks,
                auto_submit: false,
            }
        });
        let thread = self.create_agent_thread_with_server(
            agent,
            None,
            Some(thread_id),
            Some(metadata.folder_paths().clone()),
            metadata.title.clone(),
            initial_content,
            None,
            AgentThreadSource::AgentPanel,
            window,
            cx,
        );
        self.observe_draft_editor(&thread.conversation_view, cx);
        self.draft_thread = Some(thread.conversation_view);
    }

    pub fn new_external_agent_thread(
        &mut self,
        action: &NewExternalAgentThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_open_project(cx) {
            return;
        }

        self.selected_agent = action.agent.clone().into();
        self.activate_new_thread(true, AgentThreadSource::AgentPanel, window, cx);
    }

    pub fn new_terminal(
        &mut self,
        workspace: Option<&Workspace>,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.supports_terminal(cx) {
            return;
        }
        self.set_last_created_entry_kind_from_user_action(AgentPanelEntryKind::Terminal, cx);
        let working_directory = self.terminal_working_directory(workspace, cx);
        self.spawn_terminal(
            TerminalId::new(),
            working_directory,
            None,
            None,
            None,
            true,
            true,
            source,
            window,
            cx,
        );
    }

    fn terminal_working_directory(
        &self,
        workspace: Option<&Workspace>,
        cx: &App,
    ) -> Option<PathBuf> {
        workspace
            .map(|workspace| terminal_view::default_working_directory(workspace, cx))
            .unwrap_or_else(|| self.default_terminal_working_directory(cx))
    }

    pub fn supports_terminal(&self, cx: &App) -> bool {
        self.has_open_project(cx) && self.project.read(cx).supports_terminal(cx)
    }

    pub fn should_create_terminal_for_new_entry(&self, cx: &App) -> bool {
        self.last_created_entry_kind == AgentPanelEntryKind::Terminal
            && self.project.read(cx).supports_terminal(cx)
    }

    fn set_last_created_entry_kind_from_user_action(
        &mut self,
        entry_kind: AgentPanelEntryKind,
        cx: &mut Context<Self>,
    ) {
        if self.last_created_entry_kind != entry_kind {
            self.last_created_entry_kind = entry_kind;
            self.serialize(cx);
        }

        cx.background_spawn({
            let kvp = KeyValueStore::global(cx);
            async move {
                write_global_last_created_entry_kind(kvp, entry_kind).await;
            }
        })
        .detach();
    }

    fn spawn_terminal(
        &mut self,
        terminal_id: TerminalId,
        working_directory: Option<PathBuf>,
        custom_title: Option<SharedString>,
        initial_title: Option<SharedString>,
        created_at: Option<DateTime<Utc>>,
        select: bool,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let terminal_working_directory = working_directory.clone();
        let terminal_task = self.project.update(cx, |project, cx| {
            project.create_terminal_shell(working_directory, cx)
        });
        let workspace = self.workspace.clone();
        let workspace_id = self.workspace_id;
        let project = self.project.downgrade();

        cx.spawn_in(window, async move |this, cx| {
            let terminal = match terminal_task.await {
                Ok(terminal) => terminal,
                Err(error) => {
                    log::error!("failed to spawn agent panel terminal: {error:#}");
                    workspace
                        .update(cx, |workspace, cx| workspace.show_error(error, cx))
                        .log_err();
                    this.update(cx, |this, cx| {
                        if this.pending_terminal_spawn == Some(terminal_id) {
                            this.pending_terminal_spawn = None;
                            cx.notify();
                        }
                    })
                    .log_err();
                    return anyhow::Ok(());
                }
            };
            this.update_in(cx, |this, window, cx| {
                let terminal_view = cx.new(|cx| {
                    TerminalView::new(terminal, workspace, workspace_id, project, window, cx)
                });
                this.insert_terminal(
                    terminal_id,
                    terminal_view,
                    terminal_working_directory,
                    custom_title,
                    initial_title,
                    created_at,
                    select,
                    focus,
                    source,
                    window,
                    cx,
                );
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn insert_terminal(
        &mut self,
        terminal_id: TerminalId,
        terminal_view: Entity<TerminalView>,
        working_directory: Option<PathBuf>,
        custom_title: Option<SharedString>,
        initial_title: Option<SharedString>,
        created_at: Option<DateTime<Utc>>,
        select: bool,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(custom_title) = custom_title {
            terminal_view.update(cx, |terminal_view, cx| {
                terminal_view.set_custom_title(Some(custom_title.to_string()), cx);
            });
        }
        let terminal_entity = terminal_view.read(cx).terminal().clone();
        let view_subscription = cx.subscribe(
            &terminal_view,
            move |this, _terminal_view, event: &ItemEvent, cx| match event {
                ItemEvent::UpdateTab | ItemEvent::UpdateBreadcrumbs => {
                    this.refresh_terminal_metadata(terminal_id, cx);
                }
                ItemEvent::CloseItem | ItemEvent::Edit => {}
            },
        );
        // Listen on the underlying `Terminal` entity for shell-driven metadata
        // changes and bell.
        let terminal_subscription = cx.subscribe_in(
            &terminal_entity,
            window,
            move |this, _terminal, event: &TerminalEvent, window, cx| match event {
                TerminalEvent::TitleChanged
                | TerminalEvent::Wakeup
                | TerminalEvent::BreadcrumbsChanged => {
                    this.refresh_terminal_metadata(terminal_id, cx);
                    this.report_terminal_program(terminal_id, source, cx);
                }
                TerminalEvent::Bell => this.mark_terminal_notification(terminal_id, window, cx),
                TerminalEvent::CloseTerminal => {
                    this.close_terminal_from_terminal_event(terminal_id, window, cx);
                }
                TerminalEvent::BlinkChanged(_)
                | TerminalEvent::SelectionsChanged
                | TerminalEvent::NewNavigationTarget(_)
                | TerminalEvent::Open(_) => {}
            },
        );

        let last_known_terminal_title = initial_title
            .map(|title| title.to_string())
            .unwrap_or_default();
        let mut terminal = AgentTerminal {
            view: terminal_view,
            title_editor: None,
            title_editor_initial_title: None,
            title_editor_subscription: None,
            last_known_title: last_known_terminal_title.clone(),
            last_known_terminal_title,
            last_observed_program: None,
            working_directory,
            created_at: created_at.unwrap_or_else(Utc::now),
            has_notification: false,
            notification_windows: Vec::new(),
            notification_subscriptions: Vec::new(),
            _subscriptions: vec![view_subscription, terminal_subscription],
        };
        if self.pending_terminal_spawn == Some(terminal_id) {
            self.pending_terminal_spawn = None;
        }
        terminal.refresh_metadata(cx);
        terminal.report_started_terminal_program(terminal_id, source, cx);
        self.terminals.insert(terminal_id, terminal);
        self.persist_terminal_metadata(terminal_id, cx);
        self.emit_terminal_thread_started(terminal_id, source, cx);
        if select {
            self.set_base_view(BaseView::Terminal { terminal_id }, focus, window, cx);
        }
        cx.emit(AgentPanelEvent::EntryChanged);
        cx.notify();
    }

    pub fn activate_terminal(
        &mut self,
        terminal_id: TerminalId,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(terminal) = self.terminals.get_mut(&terminal_id) else {
            return;
        };
        let had_notification = terminal.has_notification;
        terminal.has_notification = false;
        if had_notification {
            self.dismiss_terminal_notifications(terminal_id, cx);
        }
        self.set_base_view(BaseView::Terminal { terminal_id }, focus, window, cx);
        if had_notification {
            cx.emit(AgentPanelEvent::EntryChanged);
            cx.notify();
        }
    }

    pub fn close_terminal(
        &mut self,
        terminal_id: TerminalId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_terminal_internal(terminal_id, true, None, window, cx);
    }

    pub fn close_terminal_without_activating_draft(
        &mut self,
        terminal_id: TerminalId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_terminal_internal(terminal_id, false, None, window, cx);
    }

    fn close_terminal_internal(
        &mut self,
        terminal_id: TerminalId,
        activate_draft_after_close: bool,
        terminal_closed_metadata: Option<TerminalThreadMetadata>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let was_active = self.active_terminal_id() == Some(terminal_id);

        if self.pending_terminal_spawn == Some(terminal_id) {
            self.pending_terminal_spawn = None;
        }
        self.dismiss_terminal_notifications(terminal_id, cx);
        if self.terminals.remove(&terminal_id).is_none() {
            return;
        }
        if let Some(store) = TerminalThreadMetadataStore::try_global(cx) {
            store.update(cx, |store, cx| {
                store.delete(terminal_id, cx);
            });
        }
        if was_active {
            self.base_view = BaseView::Uninitialized;
            self.refresh_base_view_subscriptions(window, cx);
            if activate_draft_after_close {
                self.activate_draft(false, AgentThreadSource::AgentPanel, window, cx);
            }
        }

        if let Some(metadata) = terminal_closed_metadata {
            cx.emit(AgentPanelEvent::TerminalClosed { metadata });
        }
        cx.emit(AgentPanelEvent::EntryChanged);
        cx.notify();
    }

    fn close_terminal_from_terminal_event(
        &mut self,
        terminal_id: TerminalId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let metadata = self.terminal_metadata(terminal_id, cx);
        self.close_terminal_internal(terminal_id, false, metadata, window, cx);
    }

    fn emit_terminal_thread_started(
        &self,
        terminal_id: TerminalId,
        source: AgentThreadSource,
        cx: &App,
    ) {
        telemetry::event!(
            "Agent Thread Started",
            agent = TERMINAL_AGENT_TELEMETRY_ID,
            terminal_id = terminal_id.to_key_string(),
            source = source.as_str(),
            side = crate::agent_sidebar_side(cx),
            thread_location = "current_worktree",
        );
    }

    fn refresh_terminal_metadata(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        if let Some(terminal) = self.terminals.get_mut(&terminal_id)
            && terminal.refresh_metadata(cx)
        {
            self.persist_terminal_metadata(terminal_id, cx);
            cx.emit(AgentPanelEvent::EntryChanged);
            cx.notify();
        }
    }

    fn report_terminal_program(
        &mut self,
        terminal_id: TerminalId,
        source: AgentThreadSource,
        cx: &mut Context<Self>,
    ) {
        if let Some(terminal) = self.terminals.get_mut(&terminal_id) {
            terminal.report_started_terminal_program(terminal_id, source, cx);
        }
    }

    fn persist_all_terminal_metadata(&self, cx: &mut Context<Self>) {
        let terminal_ids = self.terminals.keys().copied().collect::<Vec<_>>();
        for terminal_id in terminal_ids {
            self.persist_terminal_metadata(terminal_id, cx);
        }
    }

    fn persist_terminal_metadata(&self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        let Some(store) = TerminalThreadMetadataStore::try_global(cx) else {
            return;
        };
        let Some(metadata) = self.terminal_metadata(terminal_id, cx) else {
            return;
        };
        store.update(cx, |store, cx| {
            store.save(metadata, cx);
        });
    }

    fn terminal_metadata(
        &self,
        terminal_id: TerminalId,
        cx: &App,
    ) -> Option<TerminalThreadMetadata> {
        let terminal = self.terminals.get(&terminal_id)?;
        let project = self.project.read(cx);
        Some(TerminalThreadMetadata {
            terminal_id,
            title: terminal.terminal_title(cx),
            custom_title: terminal.custom_title(cx),
            created_at: terminal.created_at,
            worktree_paths: project.worktree_paths(cx),
            remote_connection: project.remote_connection_options(cx),
            working_directory: terminal.working_directory.clone(),
        })
    }

    pub fn restore_terminal(
        &mut self,
        metadata: TerminalThreadMetadata,
        focus: bool,
        source: AgentThreadSource,
        workspace: Option<&Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.has_terminal(metadata.terminal_id) {
            self.activate_terminal(metadata.terminal_id, focus, window, cx);
            return;
        }

        if !self.supports_terminal(cx) {
            return;
        }

        self.pending_terminal_spawn = Some(metadata.terminal_id);
        let working_directory = self.terminal_restore_working_directory(&metadata, workspace, cx);
        let initial_title = Self::terminal_restore_initial_title(&metadata);
        self.spawn_terminal(
            metadata.terminal_id,
            working_directory,
            metadata.custom_title.clone(),
            initial_title,
            Some(metadata.created_at),
            true,
            focus,
            source,
            window,
            cx,
        );
    }

    fn restore_terminal_for_panel_load(
        &mut self,
        metadata: TerminalThreadMetadata,
        focus: bool,
        source: AgentThreadSource,
        workspace: Option<&Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        #[cfg(test)]
        self.restore_test_terminal(metadata, focus, source, workspace, window, cx)
            .log_err();

        #[cfg(not(test))]
        self.restore_terminal(metadata, focus, source, workspace, window, cx);
    }

    fn terminal_restore_working_directory(
        &self,
        metadata: &TerminalThreadMetadata,
        workspace: Option<&Workspace>,
        cx: &App,
    ) -> Option<PathBuf> {
        if let Some(working_directory) = metadata.working_directory.clone() {
            return Some(working_directory);
        }

        if let Some(workspace) = workspace {
            return terminal_view::default_working_directory(workspace, cx);
        }

        self.default_terminal_working_directory(cx)
    }

    fn terminal_restore_initial_title(metadata: &TerminalThreadMetadata) -> Option<SharedString> {
        (!metadata.title.is_empty()).then(|| metadata.title.clone())
    }

    fn edit_terminal_title(
        &mut self,
        terminal_id: TerminalId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(terminal) = self.terminals.get_mut(&terminal_id) else {
            return;
        };

        if let Some(title_editor) = terminal.title_editor.as_ref() {
            title_editor.focus_handle(cx).focus(window, cx);
            return;
        }

        let title = terminal.editable_title(cx).to_string();
        let title_editor_initial_title = title.clone();
        let title_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(title, window, cx);
            editor
        });
        let title_editor_subscription = cx.subscribe_in(
            &title_editor,
            window,
            move |this, title_editor, event: &editor::EditorEvent, window, cx| {
                this.handle_terminal_title_editor_event(
                    terminal_id,
                    title_editor,
                    event,
                    window,
                    cx,
                );
            },
        );
        title_editor.update(cx, |editor, cx| {
            editor.select_all(&editor::actions::SelectAll, window, cx);
            editor.focus_handle(cx).focus(window, cx);
        });
        terminal.title_editor = Some(title_editor);
        terminal.title_editor_initial_title = Some(title_editor_initial_title);
        terminal.title_editor_subscription = Some(title_editor_subscription);
        cx.notify();
    }

    fn stop_editing_terminal_title(
        &mut self,
        terminal_id: TerminalId,
        focus_terminal: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(terminal) = self.terminals.get_mut(&terminal_id) else {
            return;
        };
        let terminal_view = terminal.view.clone();
        terminal.title_editor = None;
        terminal.title_editor_initial_title = None;
        terminal.title_editor_subscription = None;
        let title_changed = terminal.refresh_title(cx);

        if focus_terminal {
            terminal_view.focus_handle(cx).focus(window, cx);
        }
        if title_changed {
            cx.emit(AgentPanelEvent::EntryChanged);
        }
        cx.notify();
    }

    fn handle_terminal_title_editor_event(
        &mut self,
        terminal_id: TerminalId,
        title_editor: &Entity<Editor>,
        event: &editor::EditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            editor::EditorEvent::BufferEdited => {
                if !title_editor.read(cx).is_focused(window) {
                    return;
                }
                let Some((terminal_view, initial_title, terminal_title)) =
                    self.terminals.get(&terminal_id).and_then(|terminal| {
                        terminal
                            .title_editor
                            .as_ref()
                            .is_some_and(|current_editor| current_editor == title_editor)
                            .then(|| {
                                (
                                    terminal.view.clone(),
                                    terminal.title_editor_initial_title.clone(),
                                    terminal.terminal_title(cx),
                                )
                            })
                    })
                else {
                    return;
                };
                let new_title = title_editor.read(cx).text(cx);
                if initial_title.as_deref() == Some(new_title.as_str()) {
                    return;
                }
                let label = if new_title.trim().is_empty()
                    || new_title == terminal_title_without_prefix(terminal_title.as_ref())
                {
                    None
                } else {
                    Some(new_title)
                };

                cx.defer(move |cx| {
                    terminal_view.update(cx, |terminal_view, cx| {
                        terminal_view.set_custom_title(label, cx);
                    });
                });
            }
            editor::EditorEvent::Blurred => {
                if self
                    .terminals
                    .get(&terminal_id)
                    .and_then(|terminal| terminal.title_editor.as_ref())
                    .is_some_and(|current_editor| current_editor == title_editor)
                {
                    self.stop_editing_terminal_title(terminal_id, false, window, cx);
                }
            }
            _ => {}
        }
    }

    fn mark_terminal_notification(
        &mut self,
        terminal_id: TerminalId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_terminal_visible(terminal_id, window, cx) {
            return;
        }
        let newly_notified = {
            let Some(terminal) = self.terminals.get_mut(&terminal_id) else {
                return;
            };
            if terminal.has_notification {
                false
            } else {
                terminal.has_notification = true;
                true
            }
        };
        if newly_notified {
            cx.emit(AgentPanelEvent::EntryChanged);
            cx.notify();
            #[cfg(feature = "audio")]
            self.play_terminal_notification_sound(
                self.terminal_status_visible(terminal_id, window, cx),
                cx,
            );
            self.show_terminal_notification(terminal_id, window, cx);
        }
    }

    fn show_terminal_notification(
        &mut self,
        terminal_id: TerminalId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(terminal) = self.terminals.get(&terminal_id) else {
            return;
        };
        if !terminal.notification_windows.is_empty() {
            return;
        }
        let title = terminal.title(cx);
        if self.terminal_status_visible(terminal_id, window, cx) {
            return;
        }
        let settings = AgentSettings::get_global(cx);
        match settings.notify_when_agent_waiting {
            NotifyWhenAgentWaiting::PrimaryScreen => {
                if let Some(primary) = cx.primary_display() {
                    self.pop_up_terminal_notification(terminal_id, &title, primary, window, cx);
                }
            }
            NotifyWhenAgentWaiting::AllScreens => {
                for screen in cx.displays() {
                    self.pop_up_terminal_notification(terminal_id, &title, screen, window, cx);
                }
            }
            NotifyWhenAgentWaiting::Never => {}
        }
    }

    fn pop_up_terminal_notification(
        &mut self,
        terminal_id: TerminalId,
        title: &SharedString,
        screen: Rc<dyn PlatformDisplay>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let options = AgentNotification::window_options(screen, cx);
        let project_name = self.workspace.upgrade().and_then(|workspace| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .visible_worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).root_name_str().to_string())
        });
        let title = title.clone();
        let Ok(screen_window) = cx.open_window(options, |_window, cx| {
            cx.new(|_cx| AgentNotification::new(title, None, IconName::Terminal, project_name))
        }) else {
            return;
        };
        let Ok(pop_up) = screen_window.entity(cx) else {
            return;
        };

        let event_subscription = cx.subscribe_in(&pop_up, window, {
            move |this, _, event: &AgentNotificationEvent, window, cx| match event {
                AgentNotificationEvent::Accepted => {
                    let Some(handle) = window.window_handle().downcast::<MultiWorkspace>() else {
                        log::error!("root view should be a MultiWorkspace");
                        return;
                    };
                    cx.activate(true);

                    let workspace = this.workspace.clone();
                    cx.defer(move |cx| {
                        handle
                            .update(cx, |multi_workspace, window, cx| {
                                window.activate_window();

                                let Some(workspace) = workspace.upgrade() else {
                                    return;
                                };
                                multi_workspace.activate(workspace.clone(), None, window, cx);

                                workspace.update(cx, |workspace, cx| {
                                    workspace.reveal_panel::<AgentPanel>(window, cx);
                                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                                        panel.update(cx, |panel, cx| {
                                            panel.activate_terminal(terminal_id, true, window, cx);
                                        });
                                    }
                                    workspace.focus_panel::<AgentPanel>(window, cx);
                                });
                            })
                            .log_err();
                    });

                    this.dismiss_terminal_notifications(terminal_id, cx);
                }
                AgentNotificationEvent::Dismissed => {
                    this.dismiss_terminal_notifications(terminal_id, cx);
                }
            }
        });

        let pop_up_weak = pop_up.downgrade();
        let window_activation_subscription = cx.observe_window_activation(window, {
            let pop_up_weak = pop_up_weak.clone();
            move |this, window, cx| {
                this.dismiss_terminal_pop_up_if_visible(terminal_id, &pop_up_weak, window, cx);
            }
        });

        let multi_workspace_subscription = {
            let pop_up_weak = pop_up_weak.clone();
            window.root::<MultiWorkspace>().flatten().map(|mw| {
                cx.observe_in(&mw, window, move |this, _, window, cx| {
                    this.dismiss_terminal_pop_up_if_visible(terminal_id, &pop_up_weak, window, cx);
                })
            })
        };

        let this_panel = cx.entity();
        let agent_panel_subscription = cx.subscribe_in(&this_panel, window, {
            move |this, _, event: &AgentPanelEvent, window, cx| match event {
                AgentPanelEvent::ActiveViewChanged | AgentPanelEvent::ActiveViewFocused => {
                    this.dismiss_terminal_pop_up_if_visible(terminal_id, &pop_up_weak, window, cx);
                }
                AgentPanelEvent::EntryChanged
                | AgentPanelEvent::TerminalClosed { .. }
                | AgentPanelEvent::ThreadInteracted { .. } => {}
            }
        });

        let Some(terminal) = self.terminals.get_mut(&terminal_id) else {
            screen_window
                .update(cx, |_, window, _| window.remove_window())
                .ok();
            return;
        };
        terminal.notification_windows.push(screen_window);
        terminal.notification_subscriptions.push(event_subscription);
        terminal
            .notification_subscriptions
            .push(window_activation_subscription);
        terminal
            .notification_subscriptions
            .push(agent_panel_subscription);
        if let Some(subscription) = multi_workspace_subscription {
            terminal.notification_subscriptions.push(subscription);
        }
    }

    fn dismiss_terminal_notifications(&mut self, terminal_id: TerminalId, cx: &mut App) {
        let Some(terminal) = self.terminals.get_mut(&terminal_id) else {
            return;
        };
        let windows = std::mem::take(&mut terminal.notification_windows);
        terminal.notification_subscriptions.clear();
        for window in windows {
            window
                .update(cx, |_, window, _| {
                    window.remove_window();
                })
                .ok();
        }
    }

    fn dismiss_all_terminal_notifications(&mut self, cx: &mut App) {
        let terminal_ids = self.terminals.keys().copied().collect::<Vec<_>>();
        for terminal_id in terminal_ids {
            self.dismiss_terminal_notifications(terminal_id, cx);
        }
    }

    fn active_terminal_visible(&self, terminal_id: TerminalId, window: &Window, cx: &App) -> bool {
        if !window.is_window_active() {
            return false;
        }
        if !self.terminal_surface_visible(terminal_id) {
            return false;
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return false;
        };
        if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten() {
            let multi_workspace = multi_workspace.read(cx);
            if multi_workspace.workspace() != &workspace {
                return false;
            }
        }
        AgentPanel::is_visible(&workspace, cx)
    }

    fn terminal_surface_visible(&self, terminal_id: TerminalId) -> bool {
        self.active_terminal_id() == Some(terminal_id)
            && matches!(self.visible_surface(), VisibleSurface::Terminal(_))
    }

    fn terminal_status_visible(&self, terminal_id: TerminalId, window: &Window, cx: &App) -> bool {
        if !window.is_window_active() {
            return false;
        }

        if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten() {
            let multi_workspace = multi_workspace.read(cx);
            if multi_workspace.sidebar_open() && multi_workspace.is_threads_list_view_active(cx) {
                return true;
            }

            let Some(workspace) = self.workspace.upgrade() else {
                return false;
            };

            return multi_workspace.workspace() == &workspace
                && self.terminal_surface_visible(terminal_id)
                && AgentPanel::is_visible(&workspace, cx);
        }

        self.workspace.upgrade().is_some_and(|workspace| {
            self.terminal_surface_visible(terminal_id) && AgentPanel::is_visible(&workspace, cx)
        })
    }

    fn dismiss_terminal_pop_up_if_visible(
        &mut self,
        terminal_id: TerminalId,
        pop_up: &WeakEntity<AgentNotification>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.terminal_status_visible(terminal_id, window, cx) {
            return;
        }
        if self.active_terminal_visible(terminal_id, window, cx)
            && let Some(terminal) = self.terminals.get_mut(&terminal_id)
            && terminal.has_notification
        {
            terminal.has_notification = false;
            cx.emit(AgentPanelEvent::EntryChanged);
            cx.notify();
        }
        if let Some(pop_up) = pop_up.upgrade() {
            pop_up.update(cx, |notification, cx| {
                notification.dismiss(cx);
            });
        }
    }

    #[cfg(feature = "audio")]
    fn play_terminal_notification_sound(&self, visible: bool, cx: &mut App) {
        let settings = AgentSettings::get_global(cx);
        if settings.play_sound_when_agent_done.should_play(visible) {
            Audio::play_sound(Sound::AgentDone, cx);
        }
    }

    fn default_terminal_working_directory(&self, cx: &App) -> Option<PathBuf> {
        // Reuse the workspace-based helper so behavior matches the regular
        // terminal panel (e.g. `WorkingDirectory::FirstProjectDirectory` falling
        // back to a file's parent directory when the worktree root is a file).
        self.workspace
            .upgrade()
            .and_then(|workspace| terminal_view::default_working_directory(workspace.read(cx), cx))
    }

    fn has_open_project(&self, cx: &App) -> bool {
        self.project.read(cx).visible_worktrees(cx).next().is_some()
    }

    fn ensure_native_agent_connection(&self, cx: &mut Context<Self>) {
        if !self.has_open_project(cx) {
            return;
        }

        let fs = self.fs.clone();
        let thread_store = self.thread_store.clone();
        self.connection_store.update(cx, |store, cx| {
            store.request_connection(
                Agent::NativeAgent,
                Agent::NativeAgent.server(fs, thread_store),
                cx,
            );
        });
    }

    pub fn activate_draft(
        &mut self,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_open_project(cx) {
            return;
        }

        let draft = self.ensure_draft(source, window, cx);
        if let BaseView::AgentThread { conversation_view } = &self.base_view {
            if conversation_view.entity_id() == draft.entity_id() {
                // If we're already viewing the draft as the base view but an
                // overlay (e.g. Settings) is covering it, clear the overlay
                // so the user actually sees the draft they asked for.
                // Otherwise pressing "New Thread" from the Settings panel is
                // a silent no-op because the early return below would leave
                // the overlay on top of the draft.
                if self.overlay_view.is_some() {
                    self.clear_overlay(focus, window, cx);
                } else if focus {
                    self.focus_handle(cx).focus(window, cx);
                }
                return;
            }
        }
        self.set_base_view(
            BaseView::AgentThread {
                conversation_view: draft,
            },
            focus,
            window,
            cx,
        );
    }

    fn ensure_draft(
        &mut self,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ConversationView> {
        let desired_agent = self.selected_agent(cx);
        if let Some(draft) = &self.draft_thread {
            let draft_entity = draft.entity_id();
            let agent_matches = *draft.read(cx).agent_key() == desired_agent;
            let has_editor_content = draft.read(cx).root_thread_view().is_some_and(|tv| {
                !tv.read(cx)
                    .message_editor
                    .read(cx)
                    .text(cx)
                    .trim()
                    .is_empty()
            });
            // Only retarget the empty draft when the user is actively
            // viewing it — that's the case where switching agents in the
            // toolbar should replace the draft with one bound to the
            // newly-selected agent. When the draft is parked in its slot
            // while the user is viewing a real thread, `selected_agent`
            // reflects that real thread's agent and must not be allowed
            // to silently rebuild the draft.
            let draft_is_active = matches!(
                &self.base_view,
                BaseView::AgentThread { conversation_view }
                    if conversation_view.entity_id() == draft_entity
            );

            if agent_matches || has_editor_content || !draft_is_active {
                return draft.clone();
            }

            // Clean up the old empty draft's metadata so it doesn't
            // linger as a ghost entry in the sidebar.
            let old_draft_id = draft.read(cx).thread_id;
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.delete(old_draft_id, cx);
            });

            self.draft_thread = None;
            self._draft_editor_observation = None;
        }

        let thread = self.create_agent_thread_with_server(
            desired_agent,
            None,
            None,
            None,
            None,
            None,
            None,
            source,
            window,
            cx,
        );

        self.draft_thread = Some(thread.conversation_view.clone());
        self.observe_draft_editor(&thread.conversation_view, cx);
        thread.conversation_view
    }

    fn observe_draft_editor(
        &mut self,
        conversation_view: &Entity<ConversationView>,
        cx: &mut Context<Self>,
    ) {
        if let Some(acp_thread) = conversation_view.read(cx).root_thread(cx) {
            self._draft_editor_observation = Some(cx.subscribe(
                &acp_thread,
                |this, acp_thread, event: &AcpThreadEvent, cx| {
                    if !acp_thread.read(cx).is_draft_thread()
                        && this.draft_thread.as_ref().is_some_and(|draft| {
                            draft
                                .read(cx)
                                .root_thread(cx)
                                .is_some_and(|thread| thread.entity_id() == acp_thread.entity_id())
                        })
                    {
                        this.draft_thread = None;
                        this._draft_editor_observation = None;
                        this.serialize(cx);
                        return;
                    }

                    if let AcpThreadEvent::PromptUpdated = event {
                        this.serialize(cx);
                    }
                },
            ));
        } else {
            let cv = conversation_view.clone();
            self._draft_editor_observation = Some(cx.observe(&cv, |this, cv, cx| {
                if cv.read(cx).root_thread(cx).is_some() {
                    this.observe_draft_editor(&cv, cx);
                }
            }));
        }
    }

    /// Sets up an editor observation on the active view that reclaims
    /// it as ephemeral when the editor becomes empty. Only activates
    /// for non-ephemeral draft threads.
    fn observe_active_draft_for_empty_editor(
        &mut self,
        conversation_view: &Entity<ConversationView>,
        cx: &mut Context<Self>,
    ) {
        let thread_id = conversation_view.read(cx).thread_id;
        let is_ephemeral = self
            .draft_thread
            .as_ref()
            .is_some_and(|d| d.read(cx).thread_id == thread_id);
        if is_ephemeral {
            self._active_draft_reclaim_observation = None;
            return;
        }
        let is_draft = conversation_view
            .read(cx)
            .root_thread(cx)
            .is_some_and(|t| t.read(cx).is_draft_thread());
        if !is_draft {
            self._active_draft_reclaim_observation = None;
            return;
        }
        let Some(editor) = conversation_view
            .read(cx)
            .active_thread()
            .map(|tv| tv.read(cx).message_editor.clone())
        else {
            self._active_draft_reclaim_observation = None;
            return;
        };
        let cv = conversation_view.clone();
        self._active_draft_reclaim_observation =
            Some(cx.observe(&editor, move |this, _editor, cx| {
                let editor_has_text = cv.read(cx).active_thread().is_some_and(|tv| {
                    !tv.read(cx)
                        .message_editor
                        .read(cx)
                        .text(cx)
                        .trim()
                        .is_empty()
                });
                if editor_has_text {
                    return;
                }
                if this.ephemeral_draft_thread_id(cx) == Some(thread_id) {
                    return;
                }
                if this.active_thread_id(cx) != Some(thread_id) {
                    return;
                }
                if this.try_make_empty_draft_ephemeral(cv.clone(), cx) {
                    this._active_draft_reclaim_observation = None;
                    cx.emit(AgentPanelEvent::EntryChanged);
                    cx.notify();
                }
            }));
    }

    fn try_make_empty_draft_ephemeral(
        &mut self,
        conversation_view: Entity<ConversationView>,
        cx: &mut Context<Self>,
    ) -> bool {
        let (thread_id, is_draft, is_empty) = {
            let conversation = conversation_view.read(cx);
            let thread_id = conversation.thread_id;
            let is_draft = conversation
                .root_thread(cx)
                .is_some_and(|thread| thread.read(cx).is_draft_thread());
            let is_empty = if let Some(thread_view) = conversation.active_thread() {
                thread_view
                    .read(cx)
                    .message_editor
                    .read(cx)
                    .text(cx)
                    .trim()
                    .is_empty()
            } else {
                !self.draft_has_content(&conversation_view, cx)
            };

            (thread_id, is_draft, is_empty)
        };

        if !is_draft || !is_empty {
            return false;
        }

        self.retained_threads.remove(&thread_id);
        self.set_ephemeral_draft(conversation_view, cx);
        true
    }

    /// Moves a conversation view into the ephemeral `draft_thread` slot,
    /// cleaning up any previous ephemeral draft and deleting the thread's
    /// metadata so it no longer appears in the sidebar.
    fn set_ephemeral_draft(
        &mut self,
        conversation_view: Entity<ConversationView>,
        cx: &mut Context<Self>,
    ) {
        if let Some(old_draft) = self.draft_thread.take() {
            let old_id = old_draft.read(cx).thread_id;
            let new_id = conversation_view.read(cx).thread_id;
            if old_id != new_id {
                ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                    store.delete(old_id, cx);
                });
            }
            self._draft_editor_observation = None;
        }
        self.draft_thread = Some(conversation_view.clone());
        self.observe_draft_editor(&conversation_view, cx);
        self.serialize(cx);
    }

    /// Creates a new retained thread and inserts it into the sidebar without
    /// switching the active view to it. Used by the `create_thread` agent tool,
    /// which passes an initial prompt, and optionally an agent and model
    /// override.
    pub fn create_thread_with_options(
        &mut self,
        options: CreateThreadOptions,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ThreadId {
        let (agent, override_used) = if self.project.read(cx).is_via_collab() {
            (Agent::NativeAgent, false)
        } else if let Some(override_agent) = options.agent {
            (override_agent, true)
        } else {
            (self.selected_agent.clone(), false)
        };
        // If the caller explicitly overrode the agent (e.g., the `create_thread`
        // tool wants to spawn a sibling thread using a specific agent), we
        // shouldn't let that change the panel's selected_agent or the
        // last-used-agent preference. Snapshot and restore both.
        let saved_selected_agent = override_used.then(|| self.selected_agent.clone());
        let thread = self.create_agent_thread_with_server(
            agent,
            None,
            None,
            options.work_dirs,
            options.title.clone(),
            options.initial_content,
            options.model,
            source,
            window,
            cx,
        );
        if let Some(original) = saved_selected_agent {
            if self.selected_agent != original {
                self.selected_agent = original.clone();
                self.serialize(cx);
                // Restore the last-used-agent in persistent storage as well.
                cx.background_spawn({
                    let kvp = KeyValueStore::global(cx);
                    async move {
                        write_global_last_used_agent(kvp, original).await;
                    }
                })
                .detach();
            }
        }
        let thread_id = thread.conversation_view.read(cx).thread_id;
        self.retained_threads
            .insert(thread_id, thread.conversation_view);
        thread_id
    }

    pub fn activate_retained_thread(
        &mut self,
        id: ThreadId,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let conversation_view = if let Some(view) = self.retained_threads.remove(&id) {
            self.try_make_empty_draft_ephemeral(view.clone(), cx);
            view
        } else if let Some(draft) = &self.draft_thread {
            if draft.read(cx).thread_id == id {
                draft.clone()
            } else {
                return;
            }
        } else {
            return;
        };
        self.set_base_view(
            BaseView::AgentThread { conversation_view },
            focus,
            window,
            cx,
        );
    }

    pub fn active_thread_id(&self, cx: &App) -> Option<ThreadId> {
        match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                Some(conversation_view.read(cx).thread_id)
            }
            _ => None,
        }
    }

    /// Drops a thread — retained or the active ephemeral draft — from
    /// the panel and deletes its metadata row. Used by the sidebar when
    /// the user dismisses a parked draft.
    pub fn remove_thread(&mut self, id: ThreadId, window: &mut Window, cx: &mut Context<Self>) {
        self.remove_thread_internal(id, true, window, cx);
    }

    pub fn remove_thread_without_activating_draft(
        &mut self,
        id: ThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.remove_thread_internal(id, false, window, cx);
    }

    fn remove_thread_internal(
        &mut self,
        id: ThreadId,
        activate_draft_after_remove: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.retained_threads.remove(&id);
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.delete(id, cx);
        });

        if self
            .draft_thread
            .as_ref()
            .is_some_and(|d| d.read(cx).thread_id == id)
        {
            self.draft_thread = None;
            self._draft_editor_observation = None;
        }

        if self.active_thread_id(cx) == Some(id) {
            self.clear_overlay_state();
            if activate_draft_after_remove {
                self.activate_draft(false, AgentThreadSource::AgentPanel, window, cx);
            } else {
                self.base_view = BaseView::Uninitialized;
                self.refresh_base_view_subscriptions(window, cx);
            }
            self.serialize(cx);
            cx.emit(AgentPanelEvent::ActiveViewChanged);
            cx.notify();
        }
    }

    pub fn ephemeral_draft_thread_id(&self, cx: &App) -> Option<ThreadId> {
        let draft = self.draft_thread.as_ref()?;
        let draft = draft.read(cx);
        draft
            .root_thread(cx)
            .is_some_and(|thread| thread.read(cx).is_draft_thread())
            .then_some(draft.thread_id)
    }

    pub fn active_terminal_id(&self) -> Option<TerminalId> {
        match &self.base_view {
            BaseView::Terminal { terminal_id } => Some(*terminal_id),
            _ => None,
        }
    }

    pub fn has_terminal(&self, terminal_id: TerminalId) -> bool {
        self.terminals.contains_key(&terminal_id)
    }

    pub fn terminals(&self, cx: &App) -> Vec<AgentPanelTerminalInfo> {
        self.terminals
            .iter()
            .map(|(id, terminal)| AgentPanelTerminalInfo {
                id: *id,
                title: terminal.title(cx),
                created_at: terminal.created_at,
                has_notification: terminal.has_notification,
                custom_title: terminal.custom_title(cx),
                working_directory: terminal.working_directory.clone(),
            })
            .collect()
    }

    pub fn editor_text(&self, id: ThreadId, cx: &App) -> Option<String> {
        self.editor_text_if_in_memory(id, cx).flatten()
    }

    pub fn editor_text_if_in_memory(&self, id: ThreadId, cx: &App) -> Option<Option<String>> {
        let cv = self
            .retained_threads
            .get(&id)
            .or_else(|| {
                self.draft_thread
                    .as_ref()
                    .filter(|draft| draft.read(cx).thread_id == id)
            })
            .or_else(|| match &self.base_view {
                BaseView::AgentThread { conversation_view }
                    if conversation_view.read(cx).thread_id == id =>
                {
                    Some(conversation_view)
                }
                _ => None,
            })?;
        let tv = cv.read(cx).root_thread_view()?;
        let text = tv.read(cx).message_editor.read(cx).text(cx);
        if text.trim().is_empty() {
            Some(None)
        } else {
            Some(Some(text))
        }
    }

    pub fn draft_prompt_blocks_if_in_memory(
        &self,
        id: ThreadId,
        cx: &App,
    ) -> Option<Vec<acp::ContentBlock>> {
        let cv = self
            .retained_threads
            .get(&id)
            .or_else(|| {
                self.draft_thread
                    .as_ref()
                    .filter(|draft| draft.read(cx).thread_id == id)
            })
            .or_else(|| match &self.base_view {
                BaseView::AgentThread { conversation_view }
                    if conversation_view.read(cx).thread_id == id =>
                {
                    Some(conversation_view)
                }
                _ => None,
            })?;
        let thread_view = cv.read(cx).root_thread_view()?;
        let thread_view = thread_view.read(cx);
        Some(
            thread_view
                .message_editor
                .read(cx)
                .draft_content_blocks_snapshot(cx),
        )
    }

    fn new_native_agent_thread_from_summary(
        &mut self,
        action: &NewNativeAgentThreadFromSummary,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let session_id = action.from_session_id.clone();

        let Some(content) = Self::initial_content_for_thread_summary(session_id.clone(), cx) else {
            log::error!("No session found for summarization with id {}", session_id);
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            this.update_in(cx, |this, window, cx| {
                this.external_thread(
                    Some(Agent::NativeAgent),
                    None,
                    None,
                    None,
                    Some(content),
                    true,
                    AgentThreadSource::AgentPanel,
                    window,
                    cx,
                );
                anyhow::Ok(())
            })
        })
        .detach_and_log_err(cx);
    }

    fn initial_content_for_thread_summary(
        session_id: acp::SessionId,
        cx: &App,
    ) -> Option<AgentInitialContent> {
        let thread = ThreadStore::global(cx)
            .read(cx)
            .entries()
            .find(|t| t.id == session_id)?;

        Some(AgentInitialContent::ThreadSummary {
            session_id: thread.id,
            title: Some(thread.title),
        })
    }

    fn external_thread(
        &mut self,
        agent_choice: Option<crate::Agent>,
        resume_thread_id: Option<ThreadId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if resume_thread_id.is_none() && !self.has_open_project(cx) {
            return;
        }

        let agent = agent_choice.unwrap_or_else(|| self.selected_agent(cx));
        let thread = self.create_agent_thread_with_server(
            agent,
            None,
            resume_thread_id,
            work_dirs,
            title,
            initial_content,
            None,
            source,
            window,
            cx,
        );
        self.set_base_view(thread.into(), focus, window, cx);
    }

    fn manage_skills(
        &mut self,
        _action: &ManageSkills,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.dispatch_action(
            Box::new(zed_actions::OpenSettingsAt {
                path: zed_actions::AGENT_SKILLS_SETTINGS_PATH.to_string(),
                target: None,
            }),
            cx,
        );
    }

    /// Refresh the native agent's view of available skills
    pub fn refresh_skills(&mut self, cx: &mut Context<Self>) {
        if !self.has_open_project(cx) {
            return;
        }

        self.ensure_native_agent_connection(cx);
        let Some(connect_task) = self.connection_store.update(cx, |store, cx| {
            store
                .entry(&Agent::NativeAgent)
                .map(|entry| entry.read(cx).wait_for_connection())
        }) else {
            return;
        };
        let project = self.project.clone();
        cx.spawn(async move |_this, cx| -> Result<()> {
            let connected = connect_task.await?;
            if let Some(native_connection) = connected
                .connection
                .downcast::<agent::NativeAgentConnection>()
            {
                cx.update(|cx| native_connection.refresh_skills_for_project(project, cx));
            }
            Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn expand_message_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(conversation_view) = self.active_conversation_view() else {
            return;
        };

        let Some(active_thread) = conversation_view.read(cx).root_thread_view() else {
            return;
        };

        active_thread.update(cx, |active_thread, cx| {
            active_thread.expand_message_editor(&ExpandMessageEditor, window, cx);
            active_thread.focus_handle(cx).focus(window, cx);
        })
    }

    pub fn go_back(&mut self, _: &workspace::GoBack, window: &mut Window, cx: &mut Context<Self>) {
        if self.overlay_view.is_some() {
            self.clear_overlay(true, window, cx);
            cx.notify();
        }
    }

    pub fn toggle_options_menu(
        &mut self,
        _: &ToggleOptionsMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        self.agent_panel_menu_handle.toggle(window, cx);
    }

    pub fn toggle_new_thread_menu(
        &mut self,
        _: &ToggleNewThreadMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_open_project(cx) {
            return;
        }

        self.new_thread_menu_handle.toggle(window, cx);
    }

    pub fn increase_font_size(
        &mut self,
        action: &IncreaseBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_font_size_action(action.persist, px(1.0), cx);
    }

    pub fn decrease_font_size(
        &mut self,
        action: &DecreaseBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.handle_font_size_action(action.persist, px(-1.0), cx);
    }

    fn handle_font_size_action(&mut self, persist: bool, delta: Pixels, cx: &mut Context<Self>) {
        match self.visible_font_size() {
            WhichFontSize::AgentFont => {
                if persist {
                    update_settings_file(self.fs.clone(), cx, move |settings, cx| {
                        let agent_ui_font_size =
                            ThemeSettings::get_global(cx).agent_ui_font_size(cx) + delta;
                        let agent_buffer_font_size =
                            ThemeSettings::get_global(cx).agent_buffer_font_size(cx) + delta;

                        let _ = settings.theme.agent_ui_font_size.insert(
                            f32::from(theme_settings::clamp_font_size(agent_ui_font_size)).into(),
                        );
                        let _ = settings.theme.agent_buffer_font_size.insert(
                            f32::from(theme_settings::clamp_font_size(agent_buffer_font_size))
                                .into(),
                        );
                    });
                } else {
                    theme_settings::adjust_agent_ui_font_size(cx, |size| size + delta);
                    theme_settings::adjust_agent_buffer_font_size(cx, |size| size + delta);
                }
            }
            WhichFontSize::None => {
                // The agent panel does not own this font size (e.g. when a
                // terminal is the visible surface). Let the action bubble up
                // to the workspace handler so the global buffer font size is
                // adjusted instead.
                cx.propagate();
            }
        }
    }

    pub fn reset_font_size(
        &mut self,
        action: &ResetBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.visible_font_size() {
            WhichFontSize::AgentFont => {
                if action.persist {
                    update_settings_file(self.fs.clone(), cx, move |settings, _| {
                        settings.theme.agent_ui_font_size = None;
                        settings.theme.agent_buffer_font_size = None;
                    });
                } else {
                    theme_settings::reset_agent_ui_font_size(cx);
                    theme_settings::reset_agent_buffer_font_size(cx);
                }
            }
            WhichFontSize::None => {
                // Let the workspace handler reset the global buffer font size
                // that the terminal uses.
                cx.propagate();
            }
        }
    }

    pub fn reset_agent_zoom(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        theme_settings::reset_agent_ui_font_size(cx);
        theme_settings::reset_agent_buffer_font_size(cx);
    }

    pub fn toggle_zoom(&mut self, _: &ToggleZoom, window: &mut Window, cx: &mut Context<Self>) {
        if self.zoomed {
            cx.emit(PanelEvent::ZoomOut);
        } else {
            if !self.focus_handle(cx).contains_focused(window, cx) {
                cx.focus_self(window);
            }
            cx.emit(PanelEvent::ZoomIn);
        }
    }

    pub(crate) fn open_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.overlay_view, Some(OverlayView::Configuration)) {
            self.clear_overlay(true, window, cx);
            return;
        }

        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let context_server_store = self.project.read(cx).context_server_store();
        let fs = self.fs.clone();

        self.configuration = Some(cx.new(|cx| {
            AgentConfiguration::new(
                fs,
                agent_server_store,
                self.connection_store.clone(),
                context_server_store,
                self.context_server_registry.clone(),
                self.language_registry.clone(),
                self.workspace.clone(),
                window,
                cx,
            )
        }));

        if let Some(configuration) = self.configuration.as_ref() {
            self.configuration_subscription = Some(cx.subscribe_in(
                configuration,
                window,
                Self::handle_agent_configuration_event,
            ));
        }

        self.set_overlay(OverlayView::Configuration, true, window, cx);

        if let Some(configuration) = self.configuration.as_ref() {
            configuration.focus_handle(cx).focus(window, cx);
        }
    }

    pub(crate) fn open_active_thread_as_markdown(
        &mut self,
        _: &OpenActiveThreadAsMarkdown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(workspace) = self.workspace.upgrade()
            && let Some(conversation_view) = self.active_conversation_view()
            && let Some(active_thread) = conversation_view.read(cx).active_thread().cloned()
        {
            active_thread.update(cx, |thread, cx| {
                thread
                    .open_thread_as_markdown(workspace, window, cx)
                    .detach_and_log_err(cx);
            });
        }
    }

    pub fn open_thread_as_markdown(
        &mut self,
        thread_id: ThreadId,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(conversation_view) = self.conversation_view_for_id(&thread_id, cx).cloned() else {
            return false;
        };
        let Some(thread_view) = conversation_view.read(cx).root_thread_view() else {
            return false;
        };
        thread_view.update(cx, |thread, cx| {
            thread
                .open_thread_as_markdown(workspace, window, cx)
                .detach_and_log_err(cx);
        });
        true
    }

    fn copy_thread_to_clipboard(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(thread) = self.active_native_agent_thread(cx) else {
            Self::show_deferred_toast(&self.workspace, "No active native thread to copy", cx);
            return;
        };

        let workspace = self.workspace.clone();
        let load_task = thread.read(cx).to_db(cx);

        cx.spawn_in(window, async move |_this, cx| {
            let db_thread = load_task.await;
            let shared_thread = SharedThread::from_db_thread(&db_thread);
            let thread_data = shared_thread.to_bytes()?;
            let encoded = base64::Engine::encode(&base64::prelude::BASE64_STANDARD, &thread_data);

            cx.update(|_window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(encoded));
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        struct ThreadCopiedToast;
                        workspace.show_toast(
                            workspace::Toast::new(
                                workspace::notifications::NotificationId::unique::<ThreadCopiedToast>(),
                                "Thread copied to clipboard (base64 encoded)",
                            )
                            .autohide(),
                            cx,
                        );
                    });
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn show_deferred_toast(
        workspace: &WeakEntity<workspace::Workspace>,
        message: &'static str,
        cx: &mut App,
    ) {
        let workspace = workspace.clone();
        cx.defer(move |cx| {
            if let Some(workspace) = workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    struct ClipboardToast;
                    workspace.show_toast(
                        workspace::Toast::new(
                            workspace::notifications::NotificationId::unique::<ClipboardToast>(),
                            message,
                        )
                        .autohide(),
                        cx,
                    );
                });
            }
        });
    }

    fn load_thread_from_clipboard(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.has_open_project(cx) {
            Self::show_deferred_toast(&self.workspace, "Open a project to load a thread", cx);
            return;
        }

        let Some(clipboard) = cx.read_from_clipboard() else {
            Self::show_deferred_toast(&self.workspace, "No clipboard content available", cx);
            return;
        };

        let Some(encoded) = clipboard.text() else {
            Self::show_deferred_toast(&self.workspace, "Clipboard does not contain text", cx);
            return;
        };

        let thread_data = match base64::Engine::decode(&base64::prelude::BASE64_STANDARD, &encoded)
        {
            Ok(data) => data,
            Err(_) => {
                Self::show_deferred_toast(
                    &self.workspace,
                    "Failed to decode clipboard content (expected base64)",
                    cx,
                );
                return;
            }
        };

        let shared_thread = match SharedThread::from_bytes(&thread_data) {
            Ok(thread) => thread,
            Err(_) => {
                Self::show_deferred_toast(
                    &self.workspace,
                    "Failed to parse thread data from clipboard",
                    cx,
                );
                return;
            }
        };

        let db_thread = shared_thread.to_db_thread();
        let session_id = acp::SessionId::new(uuid::Uuid::new_v4().to_string());
        let thread_store = self.thread_store.clone();
        let title = db_thread.title.clone();
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |this, cx| {
            thread_store
                .update(&mut cx.clone(), |store, cx| {
                    store.save_thread(session_id.clone(), db_thread, Default::default(), cx)
                })
                .await?;

            this.update_in(cx, |this, window, cx| {
                this.open_thread(session_id, None, Some(title), window, cx);
            })?;

            this.update_in(cx, |_, _window, cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        struct ThreadLoadedToast;
                        workspace.show_toast(
                            workspace::Toast::new(
                                workspace::notifications::NotificationId::unique::<ThreadLoadedToast>(),
                                "Thread loaded from clipboard",
                            )
                            .autohide(),
                            cx,
                        );
                    });
                }
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn show_thread_metadata(
        &mut self,
        _: &ShowThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(thread_id) = self.active_thread_id(cx) else {
            Self::show_deferred_toast(&self.workspace, "No active thread", cx);
            return;
        };

        let Some(store) = ThreadMetadataStore::try_global(cx) else {
            Self::show_deferred_toast(&self.workspace, "Thread metadata store not available", cx);
            return;
        };

        let Some(metadata) = store.read(cx).entry(thread_id).cloned() else {
            Self::show_deferred_toast(&self.workspace, "No metadata found for active thread", cx);
            return;
        };

        let json = thread_metadata_to_debug_json(&metadata);
        let text = serde_json::to_string_pretty(&json).unwrap_or_default();
        let title = format!("Thread Metadata: {}", metadata.display_title());

        self.open_json_buffer(title, text, window, cx);
    }

    fn show_all_sidebar_thread_metadata(
        &mut self,
        _: &ShowAllSidebarThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(store) = ThreadMetadataStore::try_global(cx) else {
            Self::show_deferred_toast(&self.workspace, "Thread metadata store not available", cx);
            return;
        };

        let entries: Vec<serde_json::Value> = store
            .read(cx)
            .entries()
            .filter(|t| !t.archived)
            .map(thread_metadata_to_debug_json)
            .collect();

        let json = serde_json::Value::Array(entries);
        let text = serde_json::to_string_pretty(&json).unwrap_or_default();

        self.open_json_buffer("All Sidebar Thread Metadata".to_string(), text, window, cx);
    }

    fn open_json_buffer(
        &self,
        title: String,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let json_language = self.language_registry.language_for_name("JSON");
        let project = self.project.clone();
        let workspace = self.workspace.clone();

        window
            .spawn(cx, async move |cx| {
                let json_language = json_language.await.ok();

                let buffer = project
                    .update(cx, |project, cx| {
                        project.create_buffer(json_language, false, cx)
                    })
                    .await?;

                buffer.update(cx, |buffer, cx| {
                    buffer.set_text(text, cx);
                    buffer.set_capability(language::Capability::ReadWrite, cx);
                });

                workspace.update_in(cx, |workspace, window, cx| {
                    let buffer =
                        cx.new(|cx| MultiBuffer::singleton(buffer, cx).with_title(title.clone()));

                    workspace.add_item_to_active_pane(
                        Box::new(cx.new(|cx| {
                            let mut editor =
                                Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                            editor.set_breadcrumb_header(title);
                            editor.disable_mouse_wheel_zoom();
                            editor
                        })),
                        None,
                        true,
                        window,
                        cx,
                    );
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
    }

    fn handle_agent_configuration_event(
        &mut self,
        _entity: &Entity<AgentConfiguration>,
        event: &AssistantConfigurationEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            AssistantConfigurationEvent::NewThread(provider) => {
                if LanguageModelRegistry::read_global(cx)
                    .default_model()
                    .is_none_or(|model| model.provider.id() != provider.id())
                    && let Some(model) = provider.default_model(cx)
                {
                    update_settings_file(self.fs.clone(), cx, move |settings, _| {
                        let provider = model.provider_id().0.to_string();
                        let enable_thinking = model.supports_thinking();
                        let effort = model
                            .default_effort_level()
                            .map(|effort| effort.value.to_string());
                        let model = model.id().0.to_string();
                        settings
                            .agent
                            .get_or_insert_default()
                            .set_model(LanguageModelSelection {
                                provider: LanguageModelProviderSetting(provider),
                                model,
                                enable_thinking,
                                effort,
                                speed: None,
                            })
                    });
                }

                self.activate_new_thread(true, AgentThreadSource::AgentPanel, window, cx);
                if let Some((thread, model)) = self
                    .active_native_agent_thread(cx)
                    .zip(provider.default_model(cx))
                {
                    thread.update(cx, |thread, cx| {
                        thread.set_model(model, cx);
                    });
                }
            }
        }
    }

    pub fn workspace_id(&self) -> Option<WorkspaceId> {
        self.workspace_id
    }

    pub fn retained_threads(&self) -> &HashMap<ThreadId, Entity<ConversationView>> {
        &self.retained_threads
    }

    pub fn active_conversation_view(&self) -> Option<&Entity<ConversationView>> {
        match &self.base_view {
            BaseView::AgentThread { conversation_view } => Some(conversation_view),
            _ => None,
        }
    }

    pub(crate) fn visible_conversation_view(&self) -> Option<&Entity<ConversationView>> {
        match self.visible_surface() {
            VisibleSurface::AgentThread(conversation_view) => Some(conversation_view),
            _ => None,
        }
    }

    pub fn conversation_view_for_id(
        &self,
        thread_id: &ThreadId,
        cx: &App,
    ) -> Option<&Entity<ConversationView>> {
        self.retained_threads.get(thread_id).or_else(|| {
            if let Some(view) = self.active_conversation_view()
                && view.read(cx).thread_id == *thread_id
            {
                Some(view)
            } else {
                None
            }
        })
    }

    pub fn regenerate_thread_title(
        &mut self,
        thread_id: ThreadId,
        cx: &mut Context<Self>,
    ) -> ThreadTitleRegenerationResult {
        let Some(conversation_view) = self.conversation_view_for_id(&thread_id, cx).cloned() else {
            return ThreadTitleRegenerationResult::NotOpen;
        };
        Self::regenerate_conversation_thread_title(conversation_view, cx)
    }

    fn regenerate_conversation_thread_title(
        conversation_view: Entity<ConversationView>,
        cx: &mut App,
    ) -> ThreadTitleRegenerationResult {
        let Some(thread) = conversation_view.read(cx).as_native_thread(cx) else {
            return ThreadTitleRegenerationResult::NotOpen;
        };
        let thread_id = conversation_view.read(cx).parent_id();
        thread.update(cx, |thread, cx| {
            if thread.is_generating_title() {
                ThreadTitleRegenerationResult::AlreadyGenerating
            } else if thread.summarization_model().is_none() {
                ThreadTitleRegenerationResult::NoModel
            } else if thread.regenerate_title_with_callback(cx, move |title, cx| {
                ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                    store.set_generated_title(thread_id, title, cx);
                });
            }) {
                ThreadTitleRegenerationResult::Started
            } else {
                ThreadTitleRegenerationResult::AlreadyGenerating
            }
        })
    }

    pub fn conversation_views(&self) -> Vec<Entity<ConversationView>> {
        self.active_conversation_view()
            .into_iter()
            .cloned()
            .chain(self.retained_threads.values().cloned())
            .collect()
    }

    pub fn active_thread_view(&self, cx: &App) -> Option<Entity<ThreadView>> {
        let server_view = self.active_conversation_view()?;
        server_view.read(cx).root_thread_view()
    }

    pub fn active_agent_thread(&self, cx: &App) -> Option<Entity<AcpThread>> {
        match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                conversation_view.read(cx).root_thread(cx)
            }
            _ => None,
        }
    }

    pub fn is_retained_thread(&self, id: &ThreadId) -> bool {
        self.retained_threads.contains_key(id)
    }

    pub fn cancel_thread(&self, thread_id: &ThreadId, cx: &mut Context<Self>) -> bool {
        let conversation_views = self
            .active_conversation_view()
            .into_iter()
            .chain(self.retained_threads.values());

        for conversation_view in conversation_views {
            if *thread_id == conversation_view.read(cx).thread_id {
                if let Some(thread_view) = conversation_view.read(cx).root_thread_view() {
                    thread_view.update(cx, |view, cx| view.cancel_generation(cx));
                    return true;
                }
            }
        }
        false
    }

    fn update_thread_work_dirs(&self, cx: &mut Context<Self>) {
        let new_work_dirs = self.project.read(cx).default_path_list(cx);
        let new_worktree_paths = self.project.read(cx).worktree_paths(cx);

        if let Some(conversation_view) = self.active_conversation_view() {
            conversation_view.update(cx, |conversation_view, cx| {
                conversation_view.set_work_dirs(new_work_dirs.clone(), cx);
            });
        }

        for conversation_view in self.retained_threads.values() {
            conversation_view.update(cx, |conversation_view, cx| {
                conversation_view.set_work_dirs(new_work_dirs.clone(), cx);
            });
        }

        if self.project.read(cx).is_via_collab() {
            return;
        }

        // Update metadata store so threads' path lists stay in sync with
        // the project's current worktrees. Without this, threads saved
        // before a worktree was added would have stale paths and not
        // appear under the correct sidebar group.
        let mut thread_ids: Vec<ThreadId> = self.retained_threads.keys().copied().collect();
        if let Some(active_id) = self.active_thread_id(cx) {
            thread_ids.push(active_id);
        }
        if !thread_ids.is_empty() {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.update_worktree_paths(&thread_ids, new_worktree_paths, cx);
            });
        }
    }

    fn retain_running_thread(&mut self, old_view: BaseView, cx: &mut Context<Self>) {
        let BaseView::AgentThread { conversation_view } = old_view else {
            return;
        };

        if self
            .draft_thread
            .as_ref()
            .is_some_and(|d| d.entity_id() == conversation_view.entity_id())
        {
            if self.draft_has_content(&conversation_view, cx) {
                let thread_id = conversation_view.read(cx).thread_id;
                self.draft_thread = None;
                self._draft_editor_observation = None;
                self.retained_threads.insert(thread_id, conversation_view);
                self.cleanup_retained_threads(cx);
            }
            return;
        }

        let thread_id = conversation_view.read(cx).thread_id;

        if self.retained_threads.contains_key(&thread_id) {
            return;
        }

        self.retained_threads.insert(thread_id, conversation_view);
        self.cleanup_retained_threads(cx);
    }

    fn cleanup_retained_threads(&mut self, cx: &App) {
        let mut potential_removals = self
            .retained_threads
            .iter()
            .filter(|(_id, view)| {
                let Some(thread_view) = view.read(cx).root_thread_view() else {
                    return true;
                };
                let thread = thread_view.read(cx).thread.read(cx);
                thread.connection().supports_load_session() && thread.status() == ThreadStatus::Idle
            })
            .collect::<Vec<_>>();

        let max_idle = MaxIdleRetainedThreads::global(cx);

        potential_removals.sort_unstable_by_key(|(_, view)| view.read(cx).updated_at(cx));
        let n = potential_removals.len().saturating_sub(max_idle);
        let to_remove = potential_removals
            .into_iter()
            .map(|(id, _)| *id)
            .take(n)
            .collect::<Vec<_>>();
        for id in to_remove {
            self.retained_threads.remove(&id);
        }
    }

    pub(crate) fn active_native_agent_thread(&self, cx: &App) -> Option<Entity<agent::Thread>> {
        match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                conversation_view.read(cx).as_native_thread(cx)
            }
            _ => None,
        }
    }

    fn set_base_view(
        &mut self,
        new_view: BaseView,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear_overlay_state();

        let old_view = std::mem::replace(&mut self.base_view, new_view);
        self.retain_running_thread(old_view, cx);

        if let BaseView::AgentThread { conversation_view } = &self.base_view {
            let conversation_view = conversation_view.read(cx);
            let thread_agent = conversation_view.agent_key().clone();
            if self.selected_agent != thread_agent {
                self.selected_agent = thread_agent;
                self.serialize(cx);
            }
        }

        self.refresh_base_view_subscriptions(window, cx);

        if focus {
            self.focus_handle(cx).focus(window, cx);
        }
        cx.emit(AgentPanelEvent::ActiveViewChanged);
    }

    fn set_overlay(
        &mut self,
        overlay: OverlayView,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.overlay_view = Some(overlay);
        if focus {
            self.focus_handle(cx).focus(window, cx);
        }
        cx.emit(AgentPanelEvent::ActiveViewChanged);
    }

    fn clear_overlay(&mut self, focus: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_overlay_state();

        if focus {
            self.focus_handle(cx).focus(window, cx);
        }
        cx.emit(AgentPanelEvent::ActiveViewChanged);
    }

    fn clear_overlay_state(&mut self) {
        self.overlay_view = None;
        self.configuration_subscription = None;
        self.configuration = None;
    }

    fn refresh_base_view_subscriptions(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self._base_view_observation = match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                self._thread_view_subscription =
                    Self::subscribe_to_active_thread_view(conversation_view, window, cx);
                let focus_handle = conversation_view.focus_handle(cx);
                self._active_thread_focus_subscription =
                    Some(cx.on_focus_in(&focus_handle, window, |_this, _window, cx| {
                        cx.emit(AgentPanelEvent::ActiveViewFocused);
                        cx.notify();
                    }));
                let cv = conversation_view.clone();
                self.observe_active_draft_for_empty_editor(&cv, cx);
                Some(cx.observe_in(&cv, window, |this, server_view, window, cx| {
                    this._thread_view_subscription =
                        Self::subscribe_to_active_thread_view(&server_view, window, cx);
                    this.observe_active_draft_for_empty_editor(&server_view, cx);
                    cx.emit(AgentPanelEvent::ActiveViewChanged);
                    this.serialize(cx);
                    cx.notify();
                }))
            }
            BaseView::Terminal { terminal_id } => {
                self._thread_view_subscription = None;
                if let Some(terminal) = self.terminals.get(terminal_id) {
                    let terminal_id = *terminal_id;
                    let focus_handle = terminal.view.focus_handle(cx);
                    self._active_thread_focus_subscription =
                        Some(
                            cx.on_focus_in(&focus_handle, window, move |this, _window, cx| {
                                if let Some(terminal) = this.terminals.get_mut(&terminal_id) {
                                    terminal.has_notification = false;
                                }
                                cx.emit(AgentPanelEvent::ActiveViewFocused);
                                cx.notify();
                            }),
                        );
                } else {
                    self._active_thread_focus_subscription = None;
                }
                None
            }
            BaseView::Uninitialized => {
                self._thread_view_subscription = None;
                self._active_thread_focus_subscription = None;
                None
            }
        };
        self.serialize(cx);
    }

    fn visible_surface(&self) -> VisibleSurface<'_> {
        if let Some(overlay_view) = &self.overlay_view {
            return match overlay_view {
                OverlayView::Configuration => {
                    VisibleSurface::Configuration(self.configuration.as_ref())
                }
            };
        }

        match &self.base_view {
            BaseView::Uninitialized => VisibleSurface::Uninitialized,
            BaseView::AgentThread { conversation_view } => {
                VisibleSurface::AgentThread(conversation_view)
            }
            BaseView::Terminal { terminal_id } => self
                .terminals
                .get(terminal_id)
                .map(|terminal| VisibleSurface::Terminal(&terminal.view))
                .unwrap_or(VisibleSurface::Uninitialized),
        }
    }

    fn is_overlay_open(&self) -> bool {
        self.overlay_view.is_some()
    }

    fn visible_font_size(&self) -> WhichFontSize {
        self.overlay_view.as_ref().map_or_else(
            || self.base_view.which_font_size_used(),
            OverlayView::which_font_size_used,
        )
    }

    fn subscribe_to_active_thread_view(
        server_view: &Entity<ConversationView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Subscription> {
        server_view.read(cx).root_thread_view().map(|tv| {
            cx.subscribe_in(
                &tv,
                window,
                |this, _view, event: &AcpThreadViewEvent, _window, cx| match event {
                    AcpThreadViewEvent::Interacted => {
                        let Some(thread_id) = this.active_thread_id(cx) else {
                            return;
                        };
                        // If the draft was the active thread, it has now been
                        // promoted to a real thread. Clear the ephemeral
                        // pointer; the ConversationView itself stays put as
                        // the active base view.
                        if this
                            .draft_thread
                            .as_ref()
                            .is_some_and(|draft| draft.read(cx).thread_id == thread_id)
                        {
                            this.draft_thread = None;
                            this._draft_editor_observation = None;
                        }
                        this.retained_threads.remove(&thread_id);
                        cx.emit(AgentPanelEvent::ThreadInteracted { thread_id });
                    }
                },
            )
        })
    }

    fn migrate_agent_server_from_extensions(&mut self, id: Arc<str>, cx: &mut Context<Self>) {
        self.project.update(cx, |project, cx| {
            project.agent_server_store().update(cx, |store, cx| {
                store.migrate_agent_server_from_extensions(id, project.fs().clone(), cx);
            });
        });
    }

    pub fn new_agent_thread_with_external_source_prompt(
        &mut self,
        external_source_prompt: Option<ExternalSourcePrompt>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.external_thread(
            None,
            None,
            None,
            None,
            external_source_prompt.map(AgentInitialContent::from),
            true,
            AgentThreadSource::AgentPanel,
            window,
            cx,
        );
    }

    pub fn load_agent_thread(
        &mut self,
        agent: Agent,
        thread_id: ThreadId,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(store) = ThreadMetadataStore::try_global(cx) {
            store.update(cx, |store, cx| {
                store.unarchive(thread_id, cx);
            });
        }

        // Check if the active view already holds this thread.
        if let BaseView::AgentThread { conversation_view } = &self.base_view {
            if conversation_view.read(cx).thread_id == thread_id {
                self.clear_overlay_state();
                cx.emit(AgentPanelEvent::ActiveViewChanged);
                return;
            }
        }

        // Check if the thread is already in memory — either as the
        // ephemeral draft pointer or in retained_threads. Either way we
        // can just reactivate without touching storage.
        if let Some(draft) = self.draft_thread.clone()
            && draft.read(cx).thread_id == thread_id
        {
            self.set_base_view(
                BaseView::AgentThread {
                    conversation_view: draft,
                },
                focus,
                window,
                cx,
            );
            return;
        }
        if let Some(conversation_view) = self.retained_threads.remove(&thread_id) {
            self.try_make_empty_draft_ephemeral(conversation_view.clone(), cx);
            self.set_base_view(
                BaseView::AgentThread { conversation_view },
                focus,
                window,
                cx,
            );
            return;
        }

        // Not in memory. Build a fresh ConversationView. For drafts we
        // also seed the message editor with any prompt text the user had
        // typed before closing the window (persisted in the scoped kvp
        // draft-prompt store).
        let is_draft = ThreadMetadataStore::try_global(cx)
            .and_then(|store| store.read(cx).entry(thread_id).map(|m| m.is_draft()))
            .unwrap_or(false);
        let initial_content = is_draft
            .then(|| crate::draft_prompt_store::read(thread_id, cx))
            .flatten()
            .map(|blocks| AgentInitialContent::ContentBlock {
                blocks,
                auto_submit: false,
            });

        self.external_thread(
            Some(agent),
            Some(thread_id),
            work_dirs,
            title,
            initial_content,
            focus,
            source,
            window,
            cx,
        );
    }

    pub(crate) fn create_agent_thread_with_server(
        &mut self,
        agent: Agent,
        server_override: Option<Rc<dyn AgentServer>>,
        resume_thread_id: Option<ThreadId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        model_override: Option<String>,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AgentThread {
        let resume_session_id = resume_thread_id.and_then(|tid| {
            ThreadMetadataStore::try_global(cx)
                .and_then(|store| store.read(cx).entry(tid).and_then(|m| m.session_id.clone()))
        });
        self.create_agent_thread_inner(
            agent,
            server_override,
            resume_thread_id,
            resume_session_id,
            work_dirs,
            title,
            initial_content,
            model_override,
            source,
            window,
            cx,
        )
    }

    /// Legacy entry that resumes a thread by raw ACP session id when no
    /// local [`ThreadMetadata`] row exists yet (share-link imports and
    /// clipboard imports).
    ///
    /// TODO(legacy-session-id): migrate remaining callers (share-link
    /// handler, clipboard import) to mint a [`ThreadId`] + seed metadata
    /// so they can route through [`create_agent_thread_with_server`] and
    /// this entry can be deleted.
    fn create_agent_thread_with_server_for_external_session(
        &mut self,
        agent: Agent,
        server_override: Option<Rc<dyn AgentServer>>,
        resume_session_id: acp::SessionId,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AgentThread {
        self.create_agent_thread_inner(
            agent,
            server_override,
            None,
            Some(resume_session_id),
            work_dirs,
            title,
            initial_content,
            None,
            source,
            window,
            cx,
        )
    }

    fn create_agent_thread_inner(
        &mut self,
        agent: Agent,
        server_override: Option<Rc<dyn AgentServer>>,
        resume_thread_id: Option<ThreadId>,
        resume_session_id: Option<acp::SessionId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        model_override: Option<String>,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AgentThread {
        let thread_id = resume_thread_id.unwrap_or_else(ThreadId::new);
        let workspace = self.workspace.clone();
        let project = self.project.clone();

        if self.selected_agent != agent {
            self.selected_agent = agent.clone();
            self.serialize(cx);
        }

        cx.background_spawn({
            let kvp = KeyValueStore::global(cx);
            let agent = agent.clone();
            async move {
                write_global_last_used_agent(kvp, agent).await;
            }
        })
        .detach();

        let server = server_override
            .unwrap_or_else(|| agent.server(self.fs.clone(), self.thread_store.clone()));
        let thread_store = server
            .clone()
            .downcast::<agent::NativeAgentServer>()
            .is_some()
            .then(|| self.thread_store.clone());

        let connection_store = self.connection_store.clone();

        let conversation_view = cx.new(|cx| {
            crate::ConversationView::new(
                server,
                connection_store,
                agent,
                resume_session_id,
                Some(thread_id),
                work_dirs,
                title,
                initial_content,
                workspace.clone(),
                project,
                thread_store,
                source,
                window,
                cx,
            )
        });

        cx.observe_in(
            &conversation_view,
            window,
            |this, server_view, window, cx| {
                let is_active = this
                    .active_conversation_view()
                    .is_some_and(|active| active.entity_id() == server_view.entity_id());
                if is_active {
                    cx.emit(AgentPanelEvent::ActiveViewChanged);
                    this.serialize(cx);
                } else {
                    cx.emit(AgentPanelEvent::EntryChanged);
                }
                this.ensure_sibling_host_installed(&server_view, window, cx);
                cx.notify();
            },
        )
        .detach();

        // Try installing the host eagerly as well, in case the connection is
        // already established by the time the observe fires.
        self.ensure_sibling_host_installed(&conversation_view, window, cx);

        if let Some(model) = model_override {
            // The native thread is constructed asynchronously after the
            // connection establishes. Wait for the first `RootThreadUpdated`
            // event that yields a native thread, then apply the override once.
            let applied = Cell::new(false);
            cx.subscribe(
                &conversation_view,
                move |_this, view, _event: &RootThreadUpdated, cx| {
                    if applied.get() {
                        return;
                    }
                    let Some(native_thread) = view.read(cx).as_native_thread(cx) else {
                        return;
                    };
                    apply_native_model_override(&native_thread, &model, cx);
                    applied.set(true);
                },
            )
            .detach();
        }

        AgentThread { conversation_view }
    }

    fn ensure_sibling_host_installed(
        &self,
        conversation_view: &Entity<ConversationView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !cx.has_flag::<CreateThreadToolFeatureFlag>() {
            return;
        }
        let Some(native_connection) = conversation_view.read(cx).as_native_connection(cx) else {
            return;
        };
        let host = Rc::new(AgentPanelSiblingHost::new(
            cx.weak_entity(),
            window.window_handle(),
        )) as Rc<dyn agent::SiblingThreadHost>;
        native_connection.0.update(cx, |native_agent, _cx| {
            native_agent.set_sibling_thread_host(host);
        });
    }

    fn active_thread_has_messages(&self, cx: &App) -> bool {
        self.active_agent_thread(cx)
            .is_some_and(|thread| !thread.read(cx).entries().is_empty())
    }

    /// Whether the active view is in the **ephemeral** new-draft slot
    pub fn active_view_is_new_draft(&self, cx: &App) -> bool {
        self.draft_thread.as_ref().is_some_and(|draft| {
            draft
                .read(cx)
                .root_thread(cx)
                .is_some_and(|thread| thread.read(cx).is_draft_thread())
                && self
                    .active_conversation_view()
                    .is_some_and(|active| active.entity_id() == draft.entity_id())
        })
    }
    /// Whether the active thread is any kind of draft
    pub fn active_thread_is_draft(&self, cx: &App) -> bool {
        self.active_agent_thread(cx)
            .is_some_and(|thread| thread.read(cx).is_draft_thread())
    }
}

/// Apply a `provider/model-id` model override to a freshly-created native thread.
/// Best-effort: logs an error and leaves the default model in place if the
/// string can't be parsed or the model isn't registered.
pub(crate) fn apply_native_model_override(
    thread: &Entity<agent::Thread>,
    model_id: &str,
    cx: &mut App,
) {
    let Some(selected) = parse_provider_slash_model(model_id) else {
        log::warn!(
            "create_thread: could not parse model override {model_id:?}; expected `provider/model-id`"
        );
        return;
    };
    let configured = LanguageModelRegistry::global(cx)
        .update(cx, |registry, cx| registry.select_model(&selected, cx));
    let Some(configured) = configured else {
        log::warn!(
            "create_thread: no model registered for {model_id:?}; using thread's default model"
        );
        return;
    };
    thread.update(cx, |thread, cx| {
        thread.set_model(configured.model, cx);
    });
}

fn parse_provider_slash_model(input: &str) -> Option<language_model::SelectedModel> {
    let (provider, model) = input.split_once('/')?;
    if provider.is_empty() || model.is_empty() {
        return None;
    }
    Some(language_model::SelectedModel {
        provider: language_model::LanguageModelProviderId::from(provider.to_string()),
        model: language_model::LanguageModelId::from(model.to_string()),
    })
}

/// Bridges agent-side `SiblingThreadHost` calls to `AgentPanel`. Constructed
/// and installed on a `NativeAgent` by the agent panel when a native-agent
/// thread is created.
pub(crate) struct AgentPanelSiblingHost {
    panel: WeakEntity<AgentPanel>,
    window: gpui::AnyWindowHandle,
}

impl AgentPanelSiblingHost {
    pub(crate) fn new(panel: WeakEntity<AgentPanel>, window: gpui::AnyWindowHandle) -> Self {
        Self { panel, window }
    }
}

impl agent::SiblingThreadHost for AgentPanelSiblingHost {
    fn create_sibling_thread(
        &self,
        request: agent::SiblingThreadRequest,
        cx: &mut gpui::AsyncApp,
    ) -> Task<Result<agent::SiblingThreadInfo>> {
        let panel = self.panel.clone();
        let window = self.window;
        cx.spawn(async move |cx| {
            let agent_choice = match request.agent_id.as_deref() {
                None => None,
                Some(id) if id == agent::ZED_AGENT_ID.as_ref() => Some(Agent::NativeAgent),
                Some(id) => {
                    // Reject unknown agent ids up front so the model gets a
                    // structured error pointing at `list_agents_and_models`,
                    // rather than a thread that silently fails to launch in
                    // the user's sidebar.
                    let known = panel
                        .read_with(cx, |panel, cx| {
                            let store = panel.project.read(cx).agent_server_store().clone();
                            store
                                .read(cx)
                                .external_agents()
                                .any(|known_id| known_id.0.as_ref() == id)
                        })
                        .unwrap_or(false);
                    if !known {
                        return Err(anyhow!(
                            "Unknown agent id {id:?}. Call `list_agents_and_models` \
                             to see the agents available for `create_thread`."
                        ));
                    }
                    Some(Agent::Custom {
                        id: project::AgentId(id.to_string().into()),
                    })
                }
            };

            let initial_content = AgentInitialContent::ContentBlock {
                blocks: vec![acp::ContentBlock::Text(acp::TextContent::new(
                    request.prompt.clone(),
                ))],
                auto_submit: true,
            };

            let title: SharedString = request.title.clone();
            let options = CreateThreadOptions {
                title: Some(title.clone()),
                initial_content: Some(initial_content),
                agent: agent_choice.clone(),
                model: request.model.clone(),
                work_dirs: None,
            };

            // If the caller asked for a fresh worktree, open a new workspace
            // backed by a linked git worktree of each git repo in the parent
            // project — the same flow the user gets when they pick "Create
            // worktree" from the worktree picker. The sibling thread is then
            // created inside the new workspace's agent panel, so it lives
            // alongside any threads the user would create there manually.
            let mut worktree_warning: Option<String> = None;
            let target_panel = if request.use_new_worktree {
                let workspace = panel.read_with(cx, |panel, _cx| panel.workspace.clone())?;
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow!("Source workspace is no longer available"))?;
                // The branch target follows the existing UI semantics: when
                // `base_ref` is set, treat it as the ref to base off of
                // (resolved like `git switch --detach <ref>`); otherwise base
                // off the current HEAD. Either way the new worktrees are in
                // detached HEAD state — the agent can attach to a branch via
                // git afterwards.
                let branch_target = match request.base_ref.as_ref() {
                    Some(ref_name) => zed_actions::NewWorktreeBranchTarget::ExistingBranch {
                        name: ref_name.clone(),
                    },
                    None => zed_actions::NewWorktreeBranchTarget::CurrentBranch,
                };
                let action = zed_actions::CreateWorktree {
                    worktree_name: request.worktree_name.clone(),
                    branch_target,
                };
                let creation = window.update(cx, |_root, window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        git_ui::worktree_service::create_worktree_workspace(
                            workspace, &action, window, None, cx,
                        )
                    })
                })?;
                let created = creation
                    .await
                    .context("failed to create worktree workspace")?;
                // The creation flow tells us when the project had multiple
                // worktrees of the same underlying repo, which it consolidates
                // into one new worktree — flag it so the calling agent knows
                // the result may not reflect every source worktree's state.
                if created.consolidated_worktrees {
                    worktree_warning = Some(
                        "The project contained multiple worktrees backed by the same git \
                         repository, so they were consolidated into a single new worktree. \
                         The new thread's worktree is based on one of them and may not \
                         reflect the exact state of the others."
                            .to_string(),
                    );
                }
                // Locate the agent panel on the new workspace. We rely on
                // the panel having registered by the time
                // `create_worktree_workspace` returns — `open_worktree_workspace`
                // explicitly awaits `take_panels_task` and the initial scan.
                created
                    .workspace
                    .read_with(cx, |workspace, cx| workspace.panel::<AgentPanel>(cx))
                    .ok_or_else(|| anyhow!("new workspace did not register an agent panel"))?
                    .downgrade()
            } else {
                panel.clone()
            };
            // Both the source panel and any newly-opened worktree workspace
            // live in the same OS window (the new workspace is a tab on the
            // existing MultiWorkspace), so the original window handle is
            // still the right context for the `create_thread_with_options`
            // call regardless of which panel ends up the target.
            let target_window = window;

            // We deliberately don't wait for the new thread's session to
            // become available here: there are currently no agent tools that
            // operate on sibling threads by session ID, so requiring one would
            // just introduce a race for no benefit.
            let resolved_agent_id = target_window.update(cx, |_root, window, cx| {
                target_panel.update(cx, |panel, cx| {
                    panel.create_thread_with_options(
                        options,
                        AgentThreadSource::AgentPanel,
                        window,
                        cx,
                    );
                    let resolved_agent = agent_choice
                        .clone()
                        .unwrap_or_else(|| panel.selected_agent.clone());
                    resolved_agent.id()
                })
            })??;

            Ok(agent::SiblingThreadInfo {
                title,
                agent_id: resolved_agent_id.0.to_string(),
                model: request.model,
                warning: worktree_warning,
            })
        })
    }

    fn list_available_agents(&self, cx: &mut App) -> Result<agent::AvailableAgents> {
        let panel = self
            .panel
            .upgrade()
            .ok_or_else(|| anyhow!("Agent panel is no longer available"))?;

        let mut agents = Vec::new();

        // Native Zed agent — always available, and we can enumerate models
        // directly from the language model registry.
        let native_models = {
            let registry = LanguageModelRegistry::read_global(cx);
            let default = registry.default_model();
            let mut models = Vec::new();
            for provider in registry.providers() {
                if !provider.is_authenticated(cx) {
                    continue;
                }
                let provider_id = provider.id();
                for model in provider.provided_models(cx) {
                    let id = format!("{}/{}", provider_id.0, model.id().0);
                    let is_default = default
                        .as_ref()
                        .map(|cm| cm.provider.id() == provider_id && cm.model.id() == model.id())
                        .unwrap_or(false);
                    models.push(agent::AvailableModel {
                        id,
                        name: model.name().0,
                        is_default,
                    });
                }
            }
            models
        };
        agents.push(agent::AvailableAgent {
            id: agent::ZED_AGENT_ID.to_string(),
            name: Agent::NativeAgent.label(),
            is_native: true,
            models: native_models,
        });

        let project = panel.read(cx).project.clone();
        let agent_server_store = project.read(cx).agent_server_store().clone();
        let store = agent_server_store.read(cx);
        for agent_id in store.external_agents() {
            let display = store
                .agent_display_name(agent_id)
                .unwrap_or_else(|| agent_id.0.clone());
            agents.push(agent::AvailableAgent {
                id: agent_id.0.to_string(),
                name: display,
                is_native: false,
                // External agents pick their own models dynamically; we don't
                // try to enumerate them ahead of time.
                models: Vec::new(),
            });
        }

        Ok(agent::AvailableAgents { agents })
    }
}

impl Focusable for AgentPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self.visible_surface() {
            VisibleSurface::Uninitialized => self.focus_handle.clone(),
            VisibleSurface::AgentThread(conversation_view) => conversation_view.focus_handle(cx),
            VisibleSurface::Terminal(terminal_view) => terminal_view.focus_handle(cx),
            VisibleSurface::Configuration(configuration) => {
                if let Some(configuration) = configuration {
                    configuration.focus_handle(cx)
                } else {
                    self.focus_handle.clone()
                }
            }
        }
    }
}

fn agent_panel_dock_position(cx: &App) -> DockPosition {
    AgentSettings::get_global(cx).dock.into()
}

pub enum AgentPanelEvent {
    ActiveViewChanged,
    ActiveViewFocused,
    EntryChanged,
    TerminalClosed { metadata: TerminalThreadMetadata },
    ThreadInteracted { thread_id: ThreadId },
}

impl EventEmitter<PanelEvent> for AgentPanel {}
impl EventEmitter<AgentPanelEvent> for AgentPanel {}

impl Panel for AgentPanel {
    fn persistent_name() -> &'static str {
        "AgentPanel"
    }

    fn panel_key() -> &'static str {
        AGENT_PANEL_KEY
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        agent_panel_dock_position(cx)
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        position != DockPosition::Bottom
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        let side = match position {
            DockPosition::Left => "left",
            DockPosition::Right | DockPosition::Bottom => "right",
        };
        telemetry::event!("Agent Panel Side Changed", side = side);
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings
                .agent
                .get_or_insert_default()
                .set_dock(position.into());
        });
    }

    fn default_size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = AgentSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => settings.default_width,
            DockPosition::Bottom => settings.default_height,
        }
    }

    fn min_size(&self, window: &Window, cx: &App) -> Option<Pixels> {
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => Some(MIN_PANEL_WIDTH),
            DockPosition::Bottom => None,
        }
    }

    fn supports_flexible_size(&self) -> bool {
        true
    }

    fn has_flexible_size(&self, _window: &Window, cx: &App) -> bool {
        AgentSettings::get_global(cx).flexible
    }

    fn set_flexible_size(&mut self, flexible: bool, _window: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings
                .agent
                .get_or_insert_default()
                .set_flexible_size(flexible);
        });
    }

    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.is_active = active;
        if active {
            self.ensure_thread_initialized(window, cx);
        }
    }

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        (self.enabled(cx) && AgentSettings::get_global(cx).button).then_some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Agent Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        0
    }

    fn enabled(&self, cx: &App) -> bool {
        AgentSettings::get_global(cx).enabled(cx)
    }

    fn is_agent_panel(&self) -> bool {
        true
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.agent.get_or_insert_default().button = Some(false);
        }))
    }

    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        self.zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomed = zoomed;
        cx.notify();
    }
}

impl AgentPanel {
    fn ensure_thread_initialized(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.base_view, BaseView::Uninitialized) {
            if self.pending_terminal_spawn.is_some() {
                return;
            }
            if self.should_create_terminal_for_new_entry(cx) {
                let terminal_id = TerminalId::new();
                self.pending_terminal_spawn = Some(terminal_id);
                cx.defer_in(window, move |this, window, cx| {
                    if matches!(this.base_view, BaseView::Uninitialized)
                        && this.pending_terminal_spawn == Some(terminal_id)
                        && this.should_create_terminal_for_new_entry(cx)
                    {
                        this.create_initial_terminal(
                            terminal_id,
                            AgentThreadSource::AgentPanel,
                            window,
                            cx,
                        );
                    } else if this.pending_terminal_spawn == Some(terminal_id) {
                        this.pending_terminal_spawn = None;
                    }
                });
            } else {
                self.activate_draft(false, AgentThreadSource::AgentPanel, window, cx);
            }
        }
    }

    fn create_initial_terminal(
        &mut self,
        terminal_id: TerminalId,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.supports_terminal(cx) {
            if self.pending_terminal_spawn == Some(terminal_id) {
                self.pending_terminal_spawn = None;
            }
            return;
        }
        let working_directory = self.terminal_working_directory(None, cx);
        self.spawn_initial_terminal(terminal_id, working_directory, source, window, cx);
    }

    #[cfg(not(test))]
    fn spawn_initial_terminal(
        &mut self,
        terminal_id: TerminalId,
        working_directory: Option<PathBuf>,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.spawn_terminal(
            terminal_id,
            working_directory,
            None,
            None,
            None,
            true,
            false,
            source,
            window,
            cx,
        );
    }

    #[cfg(test)]
    fn spawn_initial_terminal(
        &mut self,
        terminal_id: TerminalId,
        working_directory: Option<PathBuf>,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Err(error) = self.insert_display_only_terminal(
            terminal_id,
            working_directory,
            None,
            None,
            None,
            true,
            false,
            source,
            window,
            cx,
        ) {
            log::error!("failed to spawn test agent panel terminal: {error:#}");
            if self.pending_terminal_spawn == Some(terminal_id) {
                self.pending_terminal_spawn = None;
                cx.notify();
            }
        }
    }

    fn destination_has_meaningful_state(&self, cx: &App) -> bool {
        if self.overlay_view.is_some()
            || !self.retained_threads.is_empty()
            || !self.terminals.is_empty()
        {
            return true;
        }

        match &self.base_view {
            BaseView::Uninitialized => false,
            BaseView::Terminal { .. } => true,
            BaseView::AgentThread { conversation_view } => {
                let has_entries = conversation_view
                    .read(cx)
                    .root_thread_view()
                    .is_some_and(|tv| !tv.read(cx).thread.read(cx).entries().is_empty());
                if has_entries {
                    return true;
                }

                conversation_view
                    .read(cx)
                    .root_thread_view()
                    .is_some_and(|thread_view| {
                        let thread_view = thread_view.read(cx);
                        thread_view
                            .thread
                            .read(cx)
                            .draft_prompt()
                            .is_some_and(|draft| !draft.is_empty())
                            || !thread_view
                                .message_editor
                                .read(cx)
                                .text(cx)
                                .trim()
                                .is_empty()
                    })
            }
        }
    }

    fn active_initial_content(&self, cx: &App) -> Option<AgentInitialContent> {
        let thread_view = self.active_thread_view(cx)?;
        let thread_view = thread_view.read(cx);
        let saved = thread_view
            .thread
            .read(cx)
            .draft_prompt()
            .map(|blocks| blocks.to_vec())
            .filter(|blocks| !blocks.is_empty());
        let blocks = saved.unwrap_or_else(|| {
            thread_view
                .message_editor
                .read(cx)
                .draft_content_blocks_snapshot(cx)
        });
        if blocks.is_empty() {
            return None;
        }
        Some(AgentInitialContent::ContentBlock {
            blocks,
            auto_submit: false,
        })
    }

    fn source_panel_initialization(
        source_workspace: &WeakEntity<Workspace>,
        cx: &App,
    ) -> Option<SourcePanelInitialization> {
        let source_workspace = source_workspace.upgrade()?;
        let source_panel = source_workspace.read(cx).panel::<AgentPanel>(cx)?;
        let source_panel = source_panel.read(cx);
        Some(SourcePanelInitialization {
            agent: source_panel.selected_agent(cx),
            initial_content: source_panel.active_initial_content(cx),
        })
    }

    pub fn initialize_from_source_workspace_if_needed(
        &mut self,
        source_workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.has_open_project(cx) {
            return false;
        }

        if self.destination_has_meaningful_state(cx) {
            return false;
        }

        let Some(initialization) = Self::source_panel_initialization(&source_workspace, cx) else {
            return false;
        };

        let mut initialized = false;
        if self.selected_agent != initialization.agent {
            self.selected_agent = initialization.agent.clone();
            self.serialize(cx);
            initialized = true;
        }

        if let Some(initial_content) = initialization.initial_content {
            let thread = self.create_agent_thread_with_server(
                initialization.agent,
                None,
                None,
                None,
                None,
                Some(initial_content),
                None,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
            self.draft_thread = Some(thread.conversation_view.clone());
            self.observe_draft_editor(&thread.conversation_view, cx);
            self.set_base_view(thread.into(), false, window, cx);
            true
        } else {
            if initialized
                && matches!(
                    &self.base_view,
                    BaseView::AgentThread { conversation_view }
                        if self.draft_thread.as_ref().is_some_and(|draft| {
                            draft.entity_id() == conversation_view.entity_id()
                        })
                )
            {
                self.activate_draft(false, AgentThreadSource::AgentPanel, window, cx);
            } else if initialized {
                cx.notify();
            }
            initialized
        }
    }

    fn is_title_editor_focused(&self, window: &Window, cx: &Context<Self>) -> bool {
        match self.visible_surface() {
            VisibleSurface::AgentThread(conversation_view) => conversation_view
                .read(cx)
                .root_thread_view()
                .is_some_and(|view| view.read(cx).title_editor.read(cx).is_focused(window)),
            VisibleSurface::Terminal(_) => self
                .active_terminal_id()
                .and_then(|id| self.terminals.get(&id))
                .and_then(|terminal| terminal.title_editor.as_ref())
                .is_some_and(|editor| editor.read(cx).is_focused(window)),
            _ => false,
        }
    }

    fn should_show_title_edit(&self, window: &Window, cx: &Context<Self>) -> bool {
        matches!(
            self.visible_surface(),
            VisibleSurface::AgentThread(_) | VisibleSurface::Terminal(_)
        ) && self.has_open_project(cx)
            && !self.is_title_editor_focused(window, cx)
    }

    fn render_title_view(&self, window: &mut Window, cx: &Context<Self>) -> AnyElement {
        let content = match self.visible_surface() {
            VisibleSurface::AgentThread(conversation_view) => {
                let server_view_ref = conversation_view.read(cx);
                let native_thread = server_view_ref.as_native_thread(cx);
                let is_generating_title = native_thread
                    .as_ref()
                    .is_some_and(|thread| thread.read(cx).is_generating_title());
                let title_generation_failed = native_thread
                    .as_ref()
                    .is_some_and(|thread| thread.read(cx).has_failed_title_generation());

                if let Some(title_editor) = server_view_ref
                    .root_thread_view()
                    .map(|r| r.read(cx).title_editor.clone())
                {
                    if is_generating_title {
                        Label::new(server_view_ref.title(cx))
                            .color(Color::Muted)
                            .truncate()
                            .with_animation(
                                "generating_title",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 0.8)),
                                |label, delta| label.alpha(delta),
                            )
                            .into_any_element()
                    } else {
                        let editable_title = div()
                            .flex_1()
                            .on_action({
                                let conversation_view = conversation_view.downgrade();
                                move |_: &menu::Confirm, window, cx| {
                                    if let Some(conversation_view) = conversation_view.upgrade() {
                                        conversation_view.focus_handle(cx).focus(window, cx);
                                    }
                                }
                            })
                            .on_action({
                                let conversation_view = conversation_view.downgrade();
                                move |_: &editor::actions::Cancel, window, cx| {
                                    if let Some(conversation_view) = conversation_view.upgrade() {
                                        conversation_view.focus_handle(cx).focus(window, cx);
                                    }
                                }
                            })
                            .child(title_editor);

                        if title_generation_failed {
                            h_flex()
                                .w_full()
                                .gap_1()
                                .child(editable_title)
                                .child(
                                    IconButton::new("retry-thread-title", IconName::XCircle)
                                        .icon_color(Color::Error)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text("Title generation failed. Retry"))
                                        .on_click({
                                            let conversation_view = conversation_view.clone();
                                            let workspace = self.workspace.clone();
                                            move |_event, _window, cx| {
                                                Self::handle_regenerate_thread_title(
                                                    conversation_view.clone(),
                                                    workspace.clone(),
                                                    cx,
                                                );
                                            }
                                        }),
                                )
                                .into_any_element()
                        } else {
                            editable_title.w_full().into_any_element()
                        }
                    }
                } else {
                    Label::new(conversation_view.read(cx).title(cx))
                        .color(Color::Muted)
                        .truncate()
                        .into_any_element()
                }
            }
            VisibleSurface::Terminal(_) => {
                if let Some((terminal_id, title_editor, title)) =
                    self.active_terminal_id().and_then(|terminal_id| {
                        self.terminals.get(&terminal_id).map(|terminal| {
                            (
                                terminal_id,
                                terminal.title_editor.clone(),
                                terminal.title(cx),
                            )
                        })
                    })
                {
                    if let Some(title_editor) = title_editor {
                        div()
                            .flex_1()
                            .on_action(cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                this.stop_editing_terminal_title(terminal_id, true, window, cx);
                            }))
                            .on_action(cx.listener(
                                move |this, _: &editor::actions::Cancel, window, cx| {
                                    this.stop_editing_terminal_title(terminal_id, true, window, cx);
                                },
                            ))
                            .child(title_editor)
                            .into_any_element()
                    } else {
                        div()
                            .id("terminal-title")
                            .flex_1()
                            .cursor_text()
                            .overflow_x_scroll()
                            .child(Label::new(title).color(Color::Muted).single_line())
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.edit_terminal_title(terminal_id, window, cx);
                            }))
                            .into_any_element()
                    }
                } else {
                    Label::new("Terminal").into_any_element()
                }
            }
            VisibleSurface::Configuration(_) => {
                Label::new("Settings").truncate().into_any_element()
            }
            VisibleSurface::Uninitialized => Label::new("Agent").truncate().into_any_element(),
        };

        let toolbar_bg = cx.theme().colors().tab_bar_background;
        let gradient_overlay = GradientFade::new(toolbar_bg, toolbar_bg, toolbar_bg)
            .width(px(64.0))
            .right(px(0.0))
            .gradient_stop(0.75);
        // The fade gradient renders as a visible patch on transparent windows
        // (the title already truncates).
        let opaque_window =
            cx.theme().window_background_appearance() == gpui::WindowBackgroundAppearance::Opaque;

        h_flex()
            .key_context("TitleEditor")
            .group("title_editor")
            .flex_grow_1()
            .w_full()
            .min_w_0()
            .max_w_full()
            .overflow_x_hidden()
            .child(content)
            .when(self.should_show_title_edit(window, cx), |this| {
                this.when(opaque_window, |this| this.child(gradient_overlay))
                    .child(
                        h_flex()
                            .visible_on_hover("title_editor")
                            .absolute()
                            .right_0()
                            .h_full()
                            .bg(cx.theme().colors().tab_bar_background)
                            .child(
                                IconButton::new("edit_tile", IconName::Pencil)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Edit Thread Title")),
                            ),
                    )
            })
            .into_any()
    }

    fn show_no_thread_summary_model_toast(workspace: Entity<Workspace>, cx: &mut App) {
        workspace.update(cx, |workspace, cx| {
            let toast = StatusToast::new(
                "No model is configured for summarizing thread titles.",
                cx,
                |this, _cx| {
                    this.icon(
                        Icon::new(IconName::Warning)
                            .size(IconSize::Small)
                            .color(Color::Warning),
                    )
                    .dismiss_button(true)
                },
            );
            workspace.toggle_status_toast(toast, cx);
        });
    }

    fn handle_regenerate_thread_title(
        conversation_view: Entity<ConversationView>,
        workspace: WeakEntity<Workspace>,
        cx: &mut App,
    ) {
        match Self::regenerate_conversation_thread_title(conversation_view, cx) {
            ThreadTitleRegenerationResult::NoModel => {
                if let Some(workspace) = workspace.upgrade() {
                    Self::show_no_thread_summary_model_toast(workspace, cx);
                }
            }
            ThreadTitleRegenerationResult::NotOpen
            | ThreadTitleRegenerationResult::Started
            | ThreadTitleRegenerationResult::AlreadyGenerating => {}
        }
    }

    fn render_panel_options_menu(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);
        // Resolve menu shortcuts at the thread root; the active editor can
        // shadow panel-level commands such as ManageSkills.
        let menu_action_context = match &self.base_view {
            BaseView::AgentThread { conversation_view } => conversation_view
                .read(cx)
                .active_thread()
                .map(|thread| thread.read(cx).focus_handle.clone())
                .unwrap_or_else(|| focus_handle.clone()),
            _ => focus_handle.clone(),
        };
        let showing_terminal = matches!(self.visible_surface(), VisibleSurface::Terminal(_));

        let conversation_view = match &self.base_view {
            BaseView::AgentThread { conversation_view } => Some(conversation_view.clone()),
            _ => None,
        };

        let can_regenerate_thread_title =
            conversation_view.as_ref().is_some_and(|conversation_view| {
                let conversation_view = conversation_view.read(cx);
                conversation_view.has_user_submitted_prompt(cx)
                    && conversation_view
                        .as_native_thread(cx)
                        .is_some_and(|thread| !thread.read(cx).is_generating_title())
            });

        let has_auth_methods = match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                conversation_view.read(cx).has_auth_methods()
            }
            _ => false,
        };
        let supports_logout = self
            .active_conversation_view()
            .is_some_and(|conversation_view| conversation_view.read(cx).supports_logout());

        let project_agents_md_path = project_agents_md_path(&self.project, true, cx);

        let global_agents_md_loaded = UserAgentsMd::global(cx)
            .and_then(|md| md.content())
            .is_some();

        let workspace = self.workspace.clone();

        PopoverMenu::new("agent-options-menu")
            .trigger_with_tooltip(
                IconButton::new("agent-options-menu", IconName::Ellipsis)
                    .icon_size(IconSize::Small),
                move |_window, cx| {
                    Tooltip::for_action_in(
                        "Toggle Agent Menu",
                        &ToggleOptionsMenu,
                        &focus_handle,
                        cx,
                    )
                },
            )
            .anchor(Anchor::TopRight)
            .with_handle(self.agent_panel_menu_handle.clone())
            .menu({
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |mut menu, _window, _| {
                        menu = menu.context(menu_action_context.clone());

                        if can_regenerate_thread_title {
                            menu = menu.header("Current Thread");

                            if let Some(conversation_view) = conversation_view.as_ref() {
                                menu = menu
                                    .entry("Regenerate Thread Title", None, {
                                        let conversation_view = conversation_view.clone();
                                        let workspace = workspace.clone();
                                        move |_, cx| {
                                            Self::handle_regenerate_thread_title(
                                                conversation_view.clone(),
                                                workspace.clone(),
                                                cx,
                                            );
                                        }
                                    })
                                    .separator();
                            }
                        }

                        if !showing_terminal {
                            menu = menu
                                .header("MCP Servers")
                                .action("Add Custom Server…", Box::new(AddContextServer))
                                .action(
                                    "Install New Servers…",
                                    Box::new(zed_actions::Extensions {
                                        category_filter: Some(
                                            zed_actions::ExtensionCategoryFilter::ContextServers,
                                        ),
                                        id: None,
                                    }),
                                )
                                .separator()
                                .header("Context")
                                .action("Skills", Box::new(ManageSkills));

                            if project_agents_md_path.is_some() || global_agents_md_loaded {
                                if global_agents_md_loaded {
                                    let workspace = workspace.clone();

                                    menu = menu.custom_entry(
                                        |_window, _cx| {
                                            h_flex()
                                                .w_full()
                                                .gap_1()
                                                .child(Label::new("Open Global Rules"))
                                                .child(
                                                    Label::new("(AGENTS.md)")
                                                        .color(Color::Muted)
                                                        .size(LabelSize::Small),
                                                )
                                                .into_any_element()
                                        },
                                        move |window, cx| {
                                            workspace
                                                .update(cx, |workspace, cx| {
                                                    open_global_rules(workspace, window, cx);
                                                })
                                                .log_err();
                                        },
                                    );
                                }

                                if project_agents_md_path.is_some() {
                                    let workspace = workspace.clone();
                                    menu = menu.custom_entry(
                                        |_window, _cx| {
                                            h_flex()
                                                .w_full()
                                                .gap_1()
                                                .child(Label::new("Open Project Rules"))
                                                .child(
                                                    Label::new("(AGENTS.md)")
                                                        .color(Color::Muted)
                                                        .size(LabelSize::Small),
                                                )
                                                .into_any_element()
                                        },
                                        move |window, cx| {
                                            workspace
                                                .update(cx, |workspace, cx| {
                                                    open_project_rules(workspace, window, cx);
                                                })
                                                .log_err();
                                        },
                                    );
                                }

                                menu = menu.separator();
                            }

                            menu = menu.action("Profiles", Box::new(ManageProfiles::default()));
                        }

                        menu = menu
                            .action("Settings", Box::new(OpenSettings))
                            .separator()
                            .action("Toggle Threads Sidebar", Box::new(ToggleWorkspaceSidebar));

                        if has_auth_methods || supports_logout {
                            menu = menu.separator()
                        }
                        if has_auth_methods {
                            menu = menu.action("Reauthenticate", Box::new(ReauthenticateAgent))
                        }
                        if supports_logout {
                            menu = menu.action("Log Out", Box::new(LogoutAgent))
                        }

                        menu
                    }))
                }
            })
    }

    fn render_toolbar_back_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        IconButton::new("go-back", IconName::ArrowLeft)
            .icon_size(IconSize::Small)
            .on_click(cx.listener(|this, _, window, cx| {
                this.go_back(&workspace::GoBack, window, cx);
            }))
            .tooltip({
                move |_window, cx| {
                    Tooltip::for_action_in("Go Back", &workspace::GoBack, &focus_handle, cx)
                }
            })
    }

    fn render_no_project_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        ProjectEmptyState::new(
            "Agent Panel",
            focus_handle.clone(),
            KeyBinding::for_action_in(&workspace::Open::default(), &focus_handle, cx),
        )
        .on_open_project(|_, window, cx| {
            telemetry::event!("Agent Panel Add Project Clicked");
            window.dispatch_action(workspace::Open::default().boxed_clone(), cx);
        })
        .on_clone_repo(|_, window, cx| {
            telemetry::event!("Agent Panel Clone Repo Clicked");
            window.dispatch_action(git::Clone.boxed_clone(), cx);
        })
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_server_store = self.project.read(cx).agent_server_store().clone();

        let focus_handle = self.focus_handle(cx);

        let can_create_entries = self.has_open_project(cx);
        let supports_terminal = self.supports_terminal(cx);
        let showing_terminal = matches!(self.visible_surface(), VisibleSurface::Terminal(_));

        let (selected_agent_custom_icon, selected_agent_label) = if showing_terminal {
            (None, SharedString::from("Terminal"))
        } else if let Agent::Custom { id, .. } = &self.selected_agent {
            let store = agent_server_store.read(cx);
            let icon = store.agent_icon(&id);

            let label = store
                .agent_display_name(&id)
                .unwrap_or_else(|| self.selected_agent.label());
            (icon, label)
        } else {
            (None, self.selected_agent.label())
        };

        let active_thread = match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                conversation_view.read(cx).as_native_thread(cx)
            }
            BaseView::Terminal { .. } | BaseView::Uninitialized => None,
        };

        let new_thread_menu_builder: Rc<
            dyn Fn(&mut Window, &mut App) -> Option<Entity<ContextMenu>>,
        > = {
            let selected_agent = self.selected_agent.clone();
            let is_agent_selected = move |agent: Agent| selected_agent == agent;

            let workspace = self.workspace.clone();
            let is_via_collab = workspace
                .update(cx, |workspace, cx| {
                    workspace.project().read(cx).is_via_collab()
                })
                .unwrap_or_default();

            let focus_handle = focus_handle.clone();
            let agent_server_store = agent_server_store;

            Rc::new(move |window, cx| {
                let active_thread = active_thread.clone();
                Some(ContextMenu::build(window, cx, |menu, _window, cx| {
                    menu.context(focus_handle.clone())
                        .when_some(active_thread, |this, active_thread| {
                            let thread = active_thread.read(cx);

                            if !thread.is_empty() {
                                let session_id = thread.id().clone();
                                this.item(
                                    ContextMenuEntry::new("New From Summary")
                                        .icon(IconName::ThreadFromSummary)
                                        .icon_color(Color::Muted)
                                        .handler(move |window, cx| {
                                            window.dispatch_action(
                                                Box::new(NewNativeAgentThreadFromSummary {
                                                    from_session_id: session_id.clone(),
                                                }),
                                                cx,
                                            );
                                        }),
                                )
                            } else {
                                this
                            }
                        })
                        .item(
                            ContextMenuEntry::new("Zed Agent")
                                .when(
                                    !showing_terminal && is_agent_selected(Agent::NativeAgent),
                                    |this| this.action(Box::new(NewThread)),
                                )
                                .icon(IconName::ZedAgent)
                                .icon_color(Color::Muted)
                                .handler({
                                    let workspace = workspace.clone();
                                    move |window, cx| {
                                        if let Some(workspace) = workspace.upgrade() {
                                            workspace.update(cx, |workspace, cx| {
                                                if let Some(panel) =
                                                    workspace.panel::<AgentPanel>(cx)
                                                {
                                                    panel.update(cx, |panel, cx| {
                                                        panel.selected_agent = Agent::NativeAgent;
                                                        panel.activate_new_thread(
                                                            true,
                                                            AgentThreadSource::AgentPanel,
                                                            window,
                                                            cx,
                                                        );
                                                    });
                                                }
                                            });
                                        }
                                    }
                                }),
                        )
                        .when(supports_terminal, |menu| {
                            menu.item(
                                ContextMenuEntry::new("Terminal")
                                    .when(showing_terminal, |this| this.action(Box::new(NewThread)))
                                    .when(!showing_terminal, |this| {
                                        this.action(Box::new(NewTerminalThread))
                                    })
                                    .icon(IconName::Terminal)
                                    .icon_color(Color::Muted)
                                    .handler({
                                        let workspace = workspace.clone();
                                        move |window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                workspace.update(cx, |workspace, cx| {
                                                    if let Some(panel) =
                                                        workspace.panel::<AgentPanel>(cx)
                                                    {
                                                        panel.update(cx, |panel, cx| {
                                                            panel.new_terminal(
                                                                Some(workspace),
                                                                AgentThreadSource::AgentPanel,
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                });
                                            }
                                        }
                                    }),
                            )
                        })
                        .map(|mut menu| {
                            let agent_server_store = agent_server_store.read(cx);
                            let registry_store = project::AgentRegistryStore::try_global(cx);
                            let registry_store_ref = registry_store.as_ref().map(|s| s.read(cx));

                            struct AgentMenuItem {
                                id: AgentId,
                                display_name: SharedString,
                            }

                            let agent_items = agent_server_store
                                .external_agents()
                                .map(|agent_id| {
                                    let display_name = agent_server_store
                                        .agent_display_name(agent_id)
                                        .or_else(|| {
                                            registry_store_ref
                                                .as_ref()
                                                .and_then(|store| store.agent(agent_id))
                                                .map(|a| a.name().clone())
                                        })
                                        .unwrap_or_else(|| agent_id.0.clone());
                                    AgentMenuItem {
                                        id: agent_id.clone(),
                                        display_name,
                                    }
                                })
                                .sorted_unstable_by_key(|e| e.display_name.to_lowercase())
                                .collect::<Vec<_>>();

                            if !agent_items.is_empty() {
                                menu = menu.separator().header("External Agents");
                            }
                            for item in &agent_items {
                                let mut entry = ContextMenuEntry::new(item.display_name.clone());

                                let icon_path =
                                    agent_server_store.agent_icon(&item.id).or_else(|| {
                                        registry_store_ref
                                            .as_ref()
                                            .and_then(|store| store.agent(&item.id))
                                            .and_then(|a| a.icon_path().cloned())
                                    });

                                if let Some(icon_path) = icon_path {
                                    entry = entry.custom_icon_svg(icon_path);
                                } else {
                                    entry = entry.icon(IconName::Sparkle);
                                }

                                entry = entry
                                    .when(
                                        !showing_terminal
                                            && is_agent_selected(Agent::Custom {
                                                id: item.id.clone(),
                                            }),
                                        |this| this.action(Box::new(NewThread)),
                                    )
                                    .icon_color(Color::Muted)
                                    .disabled(is_via_collab)
                                    .handler({
                                        let workspace = workspace.clone();
                                        let agent_id = item.id.clone();
                                        move |window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                workspace.update(cx, |workspace, cx| {
                                                    if let Some(panel) =
                                                        workspace.panel::<AgentPanel>(cx)
                                                    {
                                                        panel.update(cx, |panel, cx| {
                                                            panel.new_external_agent_thread(
                                                                &NewExternalAgentThread {
                                                                    agent: agent_id.clone(),
                                                                },
                                                                window,
                                                                cx,
                                                            );
                                                        });
                                                    }
                                                });
                                            }
                                        }
                                    });

                                menu = menu.item(entry);
                            }

                            menu
                        })
                        .separator()
                        .item(
                            ContextMenuEntry::new("Add More Agents")
                                .icon(IconName::Plus)
                                .icon_color(Color::Muted)
                                .handler({
                                    move |window, cx| {
                                        window
                                            .dispatch_action(Box::new(zed_actions::AcpRegistry), cx)
                                    }
                                }),
                        )
                }))
            })
        };

        let is_thread_loading = self
            .active_conversation_view()
            .map(|thread| thread.read(cx).is_loading())
            .unwrap_or(false);

        let has_custom_icon = selected_agent_custom_icon.is_some();
        let selected_agent_builtin_icon = if showing_terminal {
            Some(IconName::Terminal)
        } else {
            self.selected_agent.icon()
        };
        let selected_agent_label_for_tooltip = selected_agent_label.clone();

        let selected_agent = div()
            .id("selected_agent_icon")
            .px_0p5()
            .when_some(selected_agent_custom_icon, |this, icon_path| {
                this.child(
                    Icon::from_external_svg(icon_path)
                        .color(Color::Muted)
                        .size(IconSize::Small),
                )
            })
            .when(!has_custom_icon, |this| {
                this.when_some(selected_agent_builtin_icon, |this, icon| {
                    this.child(Icon::new(icon).color(Color::Muted))
                })
            })
            .tooltip(move |_, cx| {
                Tooltip::with_meta(
                    selected_agent_label_for_tooltip.clone(),
                    None,
                    "Selected Agent",
                    cx,
                )
            });

        let selected_agent = if is_thread_loading {
            selected_agent
                .with_animation(
                    "pulsating-icon",
                    Animation::new(Duration::from_secs(1))
                        .repeat()
                        .with_easing(pulsating_between(0.2, 0.6)),
                    |icon, delta| icon.opacity(delta),
                )
                .into_any_element()
        } else {
            selected_agent.into_any_element()
        };

        enum ToolbarMode {
            Overlay,
            Terminal,
            EmptyThread,
            ActiveThread,
        }

        let mode = if self.is_overlay_open() {
            ToolbarMode::Overlay
        } else if matches!(self.base_view, BaseView::Terminal { .. }) {
            ToolbarMode::Terminal
        } else if self.active_thread_has_messages(cx) {
            ToolbarMode::ActiveThread
        } else {
            ToolbarMode::EmptyThread
        };

        let is_full_screen = self.is_zoomed(window, cx);
        let (icon_id, icon_name, tooltip_text) = if is_full_screen {
            (
                "disable-full-screen",
                IconName::Minimize,
                "Disable Full Screen",
            )
        } else {
            (
                "enable-full-screen",
                IconName::Maximize,
                "Enable Full Screen",
            )
        };
        let full_screen_button = IconButton::new(icon_id, icon_name)
            .icon_size(IconSize::Small)
            .tooltip(move |_, cx| Tooltip::for_action(tooltip_text, &ToggleZoom, cx))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.toggle_zoom(&ToggleZoom, window, cx);
            }));

        let max_content_width = AgentSettings::get_global(cx).max_content_width;

        let base_container = h_flex()
            .size_full()
            .when(
                matches!(mode, ToolbarMode::EmptyThread | ToolbarMode::ActiveThread),
                |this| this.when_some(max_content_width, |this, max_w| this.max_w(max_w).mx_auto()),
            )
            .flex_none()
            .justify_between();

        let empty_thread_title = matches!(mode, ToolbarMode::EmptyThread).then(|| {
            Label::new(format!("New {} Thread", selected_agent_label))
                .color(Color::Muted)
                .truncate()
                .into_any_element()
        });

        let toolbar_content = {
            let new_thread_menu = PopoverMenu::new("new_thread_menu")
                .trigger_with_tooltip(
                    IconButton::new("new_thread_menu_btn", IconName::Plus)
                        .icon_size(IconSize::Small),
                    {
                        move |_window, cx| {
                            Tooltip::for_action_in(
                                "New Thread\u{2026}",
                                &ToggleNewThreadMenu,
                                &focus_handle,
                                cx,
                            )
                        }
                    },
                )
                .anchor(Anchor::TopRight)
                .with_handle(self.new_thread_menu_handle.clone())
                .menu(move |window, cx| new_thread_menu_builder(window, cx));

            base_container
                .child(
                    h_flex()
                        .relative()
                        .h_full()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .pl(DynamicSpacing::Base04.rems(cx))
                        .child(if matches!(mode, ToolbarMode::Overlay) {
                            self.render_toolbar_back_button(cx).into_any_element()
                        } else {
                            selected_agent.into_any_element()
                        })
                        .child(match empty_thread_title {
                            Some(title) => title,
                            None => self.render_title_view(window, cx),
                        }),
                )
                .child(
                    h_flex()
                        .h_full()
                        .flex_none()
                        .gap_1()
                        .pl_1()
                        .pr_1()
                        .when(can_create_entries, |this| this.child(new_thread_menu))
                        .child(full_screen_button)
                        .child(self.render_panel_options_menu(window, cx)),
                )
                .into_any_element()
        };

        h_flex()
            .id("agent-panel-toolbar")
            .h(Tab::container_height(cx))
            .flex_shrink_0()
            .max_w_full()
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(toolbar_content)
    }

    fn should_render_trial_end_upsell(&self, cx: &mut Context<Self>) -> bool {
        if TrialEndUpsell::dismissed(cx) {
            return false;
        }

        match &self.base_view {
            BaseView::AgentThread { .. } => {
                if LanguageModelRegistry::global(cx)
                    .read(cx)
                    .default_model()
                    .is_some_and(|model| {
                        model.provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
                    })
                {
                    return false;
                }
            }
            BaseView::Terminal { .. } | BaseView::Uninitialized => {
                return false;
            }
        }

        let plan = self.user_store.read(cx).plan();
        let has_previous_trial = self.user_store.read(cx).trial_started_at().is_some();

        plan.is_some_and(|plan| plan == Plan::ZedFree) && has_previous_trial
    }

    fn dismiss_ai_onboarding(&mut self, cx: &mut Context<Self>) {
        self.new_user_onboarding_upsell_dismissed
            .store(true, Ordering::Release);
        OnboardingUpsell::set_dismissed(true, cx);
        cx.notify();
    }

    fn should_render_new_user_onboarding(&mut self, cx: &mut Context<Self>) -> bool {
        if self
            .new_user_onboarding_upsell_dismissed
            .load(Ordering::Acquire)
        {
            return false;
        }

        let user_store = self.user_store.read(cx);

        if user_store.plan().is_some_and(|plan| plan == Plan::ZedPro)
            && user_store
                .subscription_period()
                .and_then(|period| period.0.checked_add_days(chrono::Days::new(1)))
                .is_some_and(|date| date < chrono::Utc::now())
        {
            if !self
                .new_user_onboarding_upsell_dismissed
                .load(Ordering::Acquire)
            {
                self.dismiss_ai_onboarding(cx);
            }
            return false;
        }

        let has_configured_non_zed_providers = LanguageModelRegistry::read_global(cx)
            .visible_providers()
            .iter()
            .any(|provider| {
                provider.is_authenticated(cx)
                    && provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
            });

        match &self.base_view {
            BaseView::Uninitialized | BaseView::Terminal { .. } => false,
            BaseView::AgentThread { conversation_view } => {
                if conversation_view.read(cx).as_native_thread(cx).is_some() {
                    let history_is_empty = ThreadStore::global(cx).read(cx).is_empty();
                    history_is_empty || !has_configured_non_zed_providers
                } else {
                    false
                }
            }
        }
    }

    fn render_new_user_onboarding(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_new_user_onboarding(cx) {
            return None;
        }

        Some(
            div()
                .bg(cx.theme().colors().editor_background)
                .child(self.new_user_onboarding.clone()),
        )
    }

    fn render_trial_end_upsell(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_trial_end_upsell(cx) {
            return None;
        }

        Some(
            v_flex()
                .absolute()
                .inset_0()
                .size_full()
                .bg(cx.theme().colors().panel_background)
                .opacity(0.85)
                .block_mouse_except_scroll()
                .child(EndTrialUpsell::new(Arc::new({
                    let this = cx.entity();
                    move |_, cx| {
                        this.update(cx, |_this, cx| {
                            TrialEndUpsell::set_dismissed(true, cx);
                            cx.notify();
                        });
                    }
                }))),
        )
    }

    fn render_drag_target(&self, cx: &Context<Self>) -> Div {
        let is_local = self.project.read(cx).is_local();
        div()
            .invisible()
            .absolute()
            .top_0()
            .right_0()
            .bottom_0()
            .left_0()
            .bg(cx.theme().colors().drop_target_background)
            .drag_over::<DraggedTab>(|this, _, _, _| this.visible())
            .drag_over::<DraggedSelection>(|this, _, _, _| this.visible())
            .when(is_local, |this| {
                this.drag_over::<ExternalPaths>(|this, _, _, _| this.visible())
            })
            .on_drop(cx.listener(move |this, tab: &DraggedTab, window, cx| {
                let item = tab.pane.read(cx).item_for_index(tab.ix);
                let project_paths = item
                    .and_then(|item| item.project_path(cx))
                    .into_iter()
                    .collect::<Vec<_>>();
                this.handle_drop(project_paths, vec![], window, cx);
            }))
            .on_drop(
                cx.listener(move |this, selection: &DraggedSelection, window, cx| {
                    let project_paths = selection
                        .items()
                        .filter_map(|item| this.project.read(cx).path_for_entry(item.entry_id, cx))
                        .collect::<Vec<_>>();
                    this.handle_drop(project_paths, vec![], window, cx);
                }),
            )
            .on_drop(cx.listener(move |this, paths: &ExternalPaths, window, cx| {
                this.handle_external_paths_drop(paths, window, cx);
            }))
    }

    fn handle_external_paths_drop(
        &mut self,
        paths: &ExternalPaths,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(&self.base_view, BaseView::Terminal { .. }) {
            // Terminal drops should match normal terminal views by pasting raw OS paths.
            // The agent-thread path below converts paths to project paths, which can add
            // worktrees and is only needed when attaching files to a conversation.
            self.paste_external_paths_into_active_terminal(paths, window, cx);
            return;
        }

        let BaseView::AgentThread { conversation_view } = &self.base_view else {
            return;
        };
        let conversation_view = conversation_view.clone();
        let tasks = paths
            .paths()
            .iter()
            .map(|path| Workspace::project_path_for_path(self.project.clone(), path, false, cx))
            .collect::<Vec<_>>();
        cx.spawn_in(window, async move |_this, cx| {
            let mut paths = vec![];
            let mut added_worktrees = vec![];
            let opened_paths = futures::future::join_all(tasks).await;
            for entry in opened_paths {
                if let Some((worktree, project_path)) = entry.log_err() {
                    added_worktrees.push(worktree);
                    paths.push(project_path);
                }
            }
            conversation_view
                .update_in(cx, |conversation_view, window, cx| {
                    conversation_view.insert_dragged_files(paths, added_worktrees, window, cx);
                })
                .log_err();
        })
        .detach();
    }

    fn paste_external_paths_into_active_terminal(
        &mut self,
        paths: &ExternalPaths,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let BaseView::Terminal { terminal_id } = &self.base_view else {
            return;
        };

        if !self.project.read(cx).is_local() {
            return;
        }

        let Some(terminal_view) = self
            .terminals
            .get(terminal_id)
            .map(|terminal| terminal.view.clone())
        else {
            return;
        };

        terminal_view.update(cx, |terminal_view, cx| {
            terminal_view.add_paths_to_terminal(paths.paths(), window, cx);
        });
    }

    fn handle_drop(
        &mut self,
        paths: Vec<ProjectPath>,
        added_worktrees: Vec<Entity<Worktree>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                conversation_view.update(cx, |conversation_view, cx| {
                    conversation_view.insert_dragged_files(paths, added_worktrees, window, cx);
                });
            }
            BaseView::Terminal { terminal_id } => {
                let paths = {
                    let project = self.project.read(cx);
                    paths
                        .iter()
                        .filter_map(|project_path| project.absolute_path(project_path, cx))
                        .collect::<Vec<_>>()
                };

                if paths.is_empty() {
                    return;
                }

                if let Some(terminal_view) = self
                    .terminals
                    .get(terminal_id)
                    .map(|terminal| terminal.view.clone())
                {
                    terminal_view.update(cx, |terminal_view, cx| {
                        terminal_view.add_paths_to_terminal(&paths, window, cx);
                    });
                }
            }
            BaseView::Uninitialized => {}
        }
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AgentPanel");
        key_context
    }
}

impl Render for AgentPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // WARNING: Changes to this element hierarchy can have
        // non-obvious implications to the layout of children.
        //
        // If you need to change it, please confirm:
        // - The message editor expands (cmd-option-esc) correctly
        // - When expanded, the buttons at the bottom of the panel are displayed correctly
        // - Font size works as expected and can be changed with cmd-+/cmd-
        // - Scrolling in all views works as expected
        // - Files can be dropped into the panel
        let content = v_flex()
            .key_context(self.key_context())
            .relative()
            .size_full()
            .justify_between()
            .bg(cx.theme().colors().panel_background)
            .on_action(cx.listener(|this, action: &NewThread, window, cx| {
                this.new_thread(action, window, cx);
            }))
            .on_action(cx.listener(|this, _: &NewTerminalThread, window, cx| {
                cx.stop_propagation();
                this.new_terminal(None, AgentThreadSource::AgentPanel, window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenSettings, window, cx| {
                this.open_configuration(window, cx);
            }))
            .on_action(cx.listener(Self::open_active_thread_as_markdown))
            .on_action(cx.listener(Self::manage_skills))
            .on_action(cx.listener(Self::go_back))
            .on_action(cx.listener(Self::toggle_options_menu))
            .on_action(cx.listener(Self::increase_font_size))
            .on_action(cx.listener(Self::decrease_font_size))
            .on_action(cx.listener(Self::reset_font_size))
            .on_action(cx.listener(Self::toggle_zoom))
            .on_action(cx.listener(|this, _: &ReauthenticateAgent, window, cx| {
                if let Some(conversation_view) = this.active_conversation_view() {
                    conversation_view.update(cx, |conversation_view, cx| {
                        conversation_view.reauthenticate(window, cx)
                    })
                }
            }))
            .on_action(cx.listener(|this, _: &LogoutAgent, window, cx| {
                if let Some(conversation_view) = this.active_conversation_view() {
                    conversation_view.update(cx, |conversation_view, cx| {
                        conversation_view.logout(window, cx)
                    })
                }
            }))
            .child(self.render_toolbar(window, cx))
            .children(self.render_new_user_onboarding(window, cx))
            .map(|parent| match self.visible_surface() {
                VisibleSurface::Uninitialized if !self.has_open_project(cx) => {
                    parent.child(self.render_no_project_state(cx))
                }
                VisibleSurface::Uninitialized => parent,
                VisibleSurface::AgentThread(conversation_view) => parent
                    .child(conversation_view.clone())
                    .child(self.render_drag_target(cx)),
                VisibleSurface::Terminal(terminal_view) => parent
                    .child(terminal_view.clone())
                    .child(self.render_drag_target(cx)),
                VisibleSurface::Configuration(configuration) => {
                    parent.children(configuration.cloned())
                }
            })
            .children(self.render_trial_end_upsell(window, cx));

        match self.visible_font_size() {
            WhichFontSize::AgentFont => {
                WithRemSize::new(ThemeSettings::get_global(cx).agent_ui_font_size(cx))
                    .size_full()
                    .child(content)
                    .into_any()
            }
            _ => content.into_any(),
        }
    }
}

struct OnboardingUpsell;

impl Dismissable for OnboardingUpsell {
    const KEY: &'static str = "dismissed-trial-upsell";
}

struct TrialEndUpsell;

impl Dismissable for TrialEndUpsell {
    const KEY: &'static str = "dismissed-trial-end-upsell";
}

/// Test-only helper methods
#[cfg(any(test, feature = "test-support"))]
impl AgentPanel {
    pub fn test_new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::new(workspace, window, cx)
    }

    /// Drops a thread's `ConversationView` from `retained_threads` without
    /// deleting its metadata or kvp state. Simulates the post-restart
    pub fn test_unload_retained_thread(&mut self, id: ThreadId) -> bool {
        self.retained_threads.remove(&id).is_some()
    }

    /// Opens an external thread using an arbitrary AgentServer.
    ///
    /// This is a test-only helper that allows visual tests and integration tests
    /// to inject a stub server without modifying production code paths.
    /// Not compiled into production builds.
    pub fn open_external_thread_with_server(
        &mut self,
        server: Rc<dyn AgentServer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ext_agent = Agent::Custom {
            id: server.agent_id(),
        };

        let thread = self.create_agent_thread_with_server(
            ext_agent,
            Some(server),
            None,
            None,
            None,
            None,
            None,
            AgentThreadSource::AgentPanel,
            window,
            cx,
        );
        self.set_base_view(thread.into(), true, window, cx);
    }

    /// Opens a restored external thread with an arbitrary AgentServer and
    /// a specific `resume_session_id` — as if we just restored from the KVP.
    ///
    /// Test-only helper. Not compiled into production builds.
    pub fn open_restored_thread_with_server(
        &mut self,
        server: Rc<dyn AgentServer>,
        resume_session_id: acp::SessionId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ext_agent = Agent::Custom {
            id: server.agent_id(),
        };

        // The panel addresses threads by `ThreadId` after the draft work;
        // map the test-provided `session_id` back through the metadata
        // store so this helper still resumes the right thread.
        let resume_thread_id = ThreadMetadataStore::try_global(cx).and_then(|store| {
            store
                .read(cx)
                .entry_by_session(&resume_session_id)
                .map(|m| m.thread_id)
        });

        let thread = self.create_agent_thread_with_server(
            ext_agent,
            Some(server),
            resume_thread_id,
            None,
            None,
            None,
            None,
            AgentThreadSource::AgentPanel,
            window,
            cx,
        );
        self.set_base_view(thread.into(), true, window, cx);
    }

    /// Returns the currently active thread view, if any.
    ///
    /// This is a test-only accessor that exposes the private `active_thread_view()`
    /// method for test assertions. Not compiled into production builds.
    pub fn active_thread_view_for_tests(&self) -> Option<&Entity<ConversationView>> {
        self.active_conversation_view()
    }

    /// Creates a draft thread using a stub server and sets it as the active view.
    #[cfg(any(test, feature = "test-support"))]
    pub fn open_draft_with_server(
        &mut self,
        server: Rc<dyn AgentServer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ext_agent = Agent::Custom {
            id: server.agent_id(),
        };
        let thread = self.create_agent_thread_with_server(
            ext_agent,
            Some(server),
            None,
            None,
            None,
            None,
            None,
            AgentThreadSource::AgentPanel,
            window,
            cx,
        );
        self.draft_thread = Some(thread.conversation_view.clone());
        self.set_base_view(thread.into(), true, window, cx);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn insert_test_terminal(
        &mut self,
        title: impl Into<String>,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<TerminalId> {
        let terminal_id = TerminalId::new();
        self.set_last_created_entry_kind_from_user_action(AgentPanelEntryKind::Terminal, cx);
        self.insert_display_only_terminal(
            terminal_id,
            None,
            Some(SharedString::from(title.into())),
            None,
            None,
            focus,
            focus,
            AgentThreadSource::AgentPanel,
            window,
            cx,
        )?;
        Ok(terminal_id)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn restore_test_terminal(
        &mut self,
        metadata: TerminalThreadMetadata,
        focus: bool,
        source: AgentThreadSource,
        workspace: Option<&Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if self.has_terminal(metadata.terminal_id) {
            self.activate_terminal(metadata.terminal_id, focus, window, cx);
            return Ok(());
        }

        if !self.supports_terminal(cx) {
            return Ok(());
        }

        let working_directory = self.terminal_restore_working_directory(&metadata, workspace, cx);
        let initial_title = Self::terminal_restore_initial_title(&metadata);
        self.insert_display_only_terminal(
            metadata.terminal_id,
            working_directory,
            metadata.custom_title.clone(),
            initial_title,
            Some(metadata.created_at),
            true,
            focus,
            source,
            window,
            cx,
        )
    }

    #[cfg(any(test, feature = "test-support"))]
    fn insert_display_only_terminal(
        &mut self,
        terminal_id: TerminalId,
        working_directory: Option<PathBuf>,
        custom_title: Option<SharedString>,
        initial_title: Option<SharedString>,
        created_at: Option<DateTime<Utc>>,
        select: bool,
        focus: bool,
        source: AgentThreadSource,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let settings = TerminalSettings::get_global(cx).clone();
        let path_style = self.project.read(cx).path_style(cx);
        let builder = terminal::TerminalBuilder::new_display_only(
            settings.cursor_shape,
            settings.alternate_scroll,
            settings.max_scroll_history_lines,
            cx.entity_id().as_u64(),
            cx.background_executor(),
            path_style,
        );
        let terminal = cx.new(|cx| builder.subscribe(cx));
        let terminal_view = cx.new(|cx| {
            TerminalView::new(
                terminal,
                self.workspace.clone(),
                self.workspace_id,
                self.project.downgrade(),
                window,
                cx,
            )
        });
        self.insert_terminal(
            terminal_id,
            terminal_view,
            working_directory,
            custom_title,
            initial_title,
            created_at,
            select,
            focus,
            source,
            window,
            cx,
        );
        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn emit_test_terminal_bell(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        let Some(terminal_entity) = self
            .terminals
            .get(&terminal_id)
            .map(|terminal| terminal.view.read(cx).terminal().clone())
        else {
            return;
        };
        terminal_entity.update(cx, |_terminal, cx| {
            cx.emit(TerminalEvent::Bell);
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn emit_test_terminal_close(&mut self, terminal_id: TerminalId, cx: &mut Context<Self>) {
        let Some(terminal_entity) = self
            .terminals
            .get(&terminal_id)
            .map(|terminal| terminal.view.read(cx).terminal().clone())
        else {
            return;
        };
        terminal_entity.update(cx, |_terminal, cx| {
            cx.emit(TerminalEvent::CloseTerminal);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NewWorktreeBranchTarget;
    use crate::conversation_view::tests::{StubAgentServer, init_test};
    use crate::test_support::{
        active_session_id, active_thread_id, open_thread_with_connection,
        open_thread_with_custom_connection, register_test_sidebar, send_message,
    };
    use acp_thread::{AgentConnection, StubAgentConnection, ThreadStatus, UserMessageId};
    use action_log::ActionLog;
    use anyhow::{Result, anyhow};
    use feature_flags::FeatureFlagAppExt;
    use fs::FakeFs;
    use gpui::{App, TestAppContext, UpdateGlobal, VisualTestContext};
    use parking_lot::Mutex;
    use project::{Project, WorktreePaths};
    use settings::{SettingsStore, WorkingDirectory};
    use std::any::Any;

    use serde_json::json;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use std::time::Instant;

    #[test]
    fn test_is_known_terminal_agent_command() {
        assert!(is_known_terminal_agent_command("claude"));
        assert!(is_known_terminal_agent_command("codex"));
        assert!(!is_known_terminal_agent_command("cargo"));
        assert!(!is_known_terminal_agent_command("internal-agent"));
    }

    #[test]
    fn test_terminal_program_reports_known_agent_transitions() {
        let mut last_observed_program = None;

        assert_eq!(
            terminal_program_to_report(&mut last_observed_program, Some("codex".to_string())),
            Some("codex".to_string())
        );
        assert_eq!(
            terminal_program_to_report(&mut last_observed_program, Some("codex".to_string())),
            None
        );
        assert_eq!(
            terminal_program_to_report(&mut last_observed_program, Some("zsh".to_string())),
            None
        );
        assert_eq!(
            terminal_program_to_report(
                &mut last_observed_program,
                Some("customer-data-export".to_string())
            ),
            None
        );
        assert_eq!(
            terminal_program_to_report(&mut last_observed_program, Some("codex".to_string())),
            Some("codex".to_string())
        );
        assert_eq!(
            terminal_program_to_report(&mut last_observed_program, None),
            None
        );
        assert_eq!(
            terminal_program_to_report(&mut last_observed_program, Some("codex".to_string())),
            Some("codex".to_string())
        );
    }

    #[derive(Clone, Default)]
    struct SessionTrackingConnection {
        next_session_number: Arc<Mutex<usize>>,
        sessions: Arc<Mutex<HashSet<acp::SessionId>>>,
    }

    impl SessionTrackingConnection {
        fn new() -> Self {
            Self::default()
        }

        fn create_session(
            self: Rc<Self>,
            session_id: acp::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Entity<AcpThread> {
            self.sessions.lock().insert(session_id.clone());

            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            cx.new(|cx| {
                AcpThread::new(
                    None,
                    title,
                    Some(work_dirs),
                    self,
                    project,
                    action_log,
                    session_id,
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            })
        }
    }

    impl AgentConnection for SessionTrackingConnection {
        fn agent_id(&self) -> AgentId {
            agent::ZED_AGENT_ID.clone()
        }

        fn telemetry_id(&self) -> SharedString {
            "session-tracking-test".into()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            let session_id = {
                let mut next_session_number = self.next_session_number.lock();
                let session_id = acp::SessionId::new(format!(
                    "session-tracking-session-{}",
                    *next_session_number
                ));
                *next_session_number += 1;
                session_id
            };
            let thread = self.create_session(session_id, project, work_dirs, None, cx);
            Task::ready(Ok(thread))
        }

        fn supports_load_session(&self) -> bool {
            true
        }

        fn load_session(
            self: Rc<Self>,
            session_id: acp::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            let thread = self.create_session(session_id, project, work_dirs, title, cx);
            thread.update(cx, |thread, cx| {
                thread
                    .handle_session_update(
                        acp::SessionUpdate::UserMessageChunk(acp::ContentChunk::new(
                            "Restored user message".into(),
                        )),
                        cx,
                    )
                    .expect("restored user message should be applied");
                thread
                    .handle_session_update(
                        acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                            "Restored assistant message".into(),
                        )),
                        cx,
                    )
                    .expect("restored assistant message should be applied");
            });
            Task::ready(Ok(thread))
        }

        fn supports_close_session(&self) -> bool {
            true
        }

        fn close_session(
            self: Rc<Self>,
            session_id: &acp::SessionId,
            _cx: &mut App,
        ) -> Task<Result<()>> {
            self.sessions.lock().remove(session_id);
            Task::ready(Ok(()))
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
        }

        fn authenticate(&self, _method_id: acp::AuthMethodId, _cx: &mut App) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }

        fn prompt(
            &self,
            _id: UserMessageId,
            params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<Result<acp::PromptResponse>> {
            if !self.sessions.lock().contains(&params.session_id) {
                return Task::ready(Err(anyhow!("Session not found")));
            }

            Task::ready(Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {}

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    #[gpui::test]
    async fn test_active_thread_serialize_and_load_round_trip(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        // Create a MultiWorkspace window with two workspaces.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        let project_a = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;
        let project_b = Project::test(fs, [], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        let workspace_a = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        workspace_a.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });
        workspace_b.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        // Set up workspace A: with an active thread.
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        panel_a.update_in(cx, |panel, window, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });

        cx.run_until_parked();

        panel_a.read_with(cx, |panel, cx| {
            assert!(
                panel.active_agent_thread(cx).is_some(),
                "workspace A should have an active thread after connection"
            );
        });

        send_message(&panel_a, cx);

        let agent_type_a = panel_a.read_with(cx, |panel, _cx| panel.selected_agent.clone());

        // Set up workspace B: ClaudeCode, no active thread.
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        panel_b.update(cx, |panel, _cx| {
            panel.selected_agent = Agent::Custom {
                id: "claude-acp".into(),
            };
        });

        // Serialize both panels.
        panel_a.update(cx, |panel, cx| panel.serialize(cx));
        panel_b.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        let workspace_a_id = workspace_a
            .read_with(cx, |workspace, _cx| workspace.database_id())
            .expect("workspace A should have a database id");
        let kvp = cx.update(|_window, cx| KeyValueStore::global(cx));
        let serialized_a: SerializedAgentPanel = cx
            .background_spawn(async move { read_serialized_panel(workspace_a_id, &kvp) })
            .await
            .expect("workspace A should serialize panel state");
        assert!(
            serialized_a.last_active_thread.is_some(),
            "active thread should be the thread restore target"
        );
        assert!(
            serialized_a.last_active_terminal_id.is_none(),
            "active thread serialization should not also include a terminal restore target"
        );

        cx.update(|_window, cx| {
            ThreadMetadataStore::init_global(cx);
        });

        // Load fresh panels for each workspace and verify independent state.
        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_a = AgentPanel::load(workspace_a.downgrade(), async_cx)
            .await
            .expect("panel A load should succeed");
        cx.run_until_parked();

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_b = AgentPanel::load(workspace_b.downgrade(), async_cx)
            .await
            .expect("panel B load should succeed");
        cx.run_until_parked();

        // Workspace A should restore its thread and agent type
        loaded_a.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent, agent_type_a,
                "workspace A agent type should be restored"
            );
            assert!(
                panel.active_conversation_view().is_some(),
                "workspace A should have its active thread restored"
            );
        });

        // Workspace B should restore its own agent type but have no active thread.
        loaded_b.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent,
                Agent::Custom {
                    id: "claude-acp".into()
                },
                "workspace B agent type should be restored"
            );
            assert!(
                panel.active_conversation_view().is_none(),
                "workspace B should have no active thread when it had no prior conversation"
            );
        });
    }

    #[gpui::test]
    async fn test_active_terminal_serialize_and_load_round_trip(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            TerminalThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.activate_new_thread(false, AgentThreadSource::AgentPanel, window, cx);
        });
        let terminal_id = panel
            .update_in(cx, |panel, window, cx| {
                panel.insert_test_terminal("Dev Server", true, window, cx)
            })
            .expect("test terminal should be inserted");
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        let workspace_id = workspace
            .read_with(cx, |workspace, _cx| workspace.database_id())
            .expect("workspace should have a database id");
        let kvp = cx.update(|_window, cx| KeyValueStore::global(cx));
        let serialized: SerializedAgentPanel = cx
            .background_spawn(async move { read_serialized_panel(workspace_id, &kvp) })
            .await
            .expect("workspace should serialize panel state");
        assert_eq!(
            serialized.last_active_terminal_id,
            Some(terminal_id.to_key_string())
        );
        assert!(
            serialized.last_active_thread.is_none(),
            "active terminal serialization should not also include a thread restore target"
        );

        cx.update(|_window, cx| {
            TerminalThreadMetadataStore::init_global(cx);
        });
        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        for _ in 0..8 {
            cx.run_until_parked();
        }

        loaded.read_with(cx, |panel, cx| {
            assert_eq!(panel.active_terminal_id(), Some(terminal_id));
            assert!(
                panel.active_conversation_view().is_none(),
                "the restored terminal should remain active instead of falling back to a draft"
            );
            assert!(
                panel
                    .terminals(cx)
                    .into_iter()
                    .any(|terminal| terminal.id == terminal_id),
                "active terminal metadata should be restored into the loaded panel"
            );
        });
    }

    #[gpui::test]
    async fn test_terminal_restore_working_directory_does_not_read_leased_workspace(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);

            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .terminal
                        .get_or_insert_default()
                        .project
                        .working_directory = Some(WorkingDirectory::AlwaysHome);
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        project.update(cx, |project, _cx| {
            project.mark_as_collab_for_testing();
        });
        project.read_with(cx, |project, _cx| {
            assert!(project.is_remote());
        });

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .expect("multi workspace should have an active workspace");
        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        assert_eq!(
            workspace.read_with(cx, |workspace, cx| {
                terminal_view::default_working_directory(workspace, cx)
            }),
            None
        );

        let metadata = TerminalThreadMetadata {
            terminal_id: TerminalId::new(),
            title: "Dev Server".into(),
            custom_title: None,
            created_at: Utc::now(),
            worktree_paths: project.read_with(cx, |project, cx| project.worktree_paths(cx)),
            remote_connection: None,
            working_directory: None,
        };
        assert_eq!(metadata.working_directory, None);

        let working_directory = workspace.update_in(cx, |workspace, _window, cx| {
            panel
                .read(cx)
                .terminal_restore_working_directory(&metadata, Some(workspace), cx)
        });

        assert_eq!(working_directory, None);
    }

    #[gpui::test]
    async fn test_pending_terminal_restore_prevents_initial_terminal_creation(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.last_created_entry_kind = AgentPanelEntryKind::Terminal;
            panel.pending_terminal_spawn = Some(TerminalId::new());
            panel.set_active(true, window, cx);
        });
        for _ in 0..4 {
            cx.run_until_parked();
        }

        panel.read_with(&cx, |panel, cx| {
            assert!(
                panel.terminals(cx).is_empty(),
                "activation while a terminal restore is pending should not create a second terminal"
            );
            assert!(
                panel.active_conversation_view().is_none(),
                "activation while a terminal restore is pending should not fall back to a draft"
            );
        });
    }

    #[gpui::test]
    async fn test_repeated_activation_only_creates_one_initial_terminal(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.last_created_entry_kind = AgentPanelEntryKind::Terminal;
            panel.set_active(true, window, cx);
            panel.set_active(true, window, cx);
        });
        for _ in 0..8 {
            cx.run_until_parked();
        }

        panel.read_with(&cx, |panel, cx| {
            assert_eq!(
                panel.terminals(cx).len(),
                1,
                "repeated activation should only enqueue one initial terminal"
            );
            assert!(
                panel.active_terminal_id().is_some(),
                "the single initial terminal should become active"
            );
        });
    }

    #[gpui::test]
    async fn test_restored_terminal_does_not_update_global_entry_kind(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        cx.update(|_, cx| {
            TerminalThreadMetadataStore::init_global(cx);
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.activate_new_thread(false, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert_eq!(
                read_global_last_created_entry_kind(&KeyValueStore::global(cx)),
                Some(AgentPanelEntryKind::Thread)
            );
        });

        let metadata = TerminalThreadMetadata {
            terminal_id: TerminalId::new(),
            title: "Restored Terminal".into(),
            custom_title: None,
            created_at: Utc::now(),
            worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                "/project",
            )])),
            remote_connection: None,
            working_directory: None,
        };
        panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.restore_test_terminal(
                    metadata,
                    true,
                    AgentThreadSource::AgentPanel,
                    None,
                    window,
                    cx,
                )
            })
            .expect("test terminal should be restored");
        cx.run_until_parked();

        cx.update(|_, cx| {
            assert_eq!(
                read_global_last_created_entry_kind(&KeyValueStore::global(cx)),
                Some(AgentPanelEntryKind::Thread),
                "restoring a terminal should not change the global new-entry default"
            );
        });
    }

    #[gpui::test]
    async fn test_new_workspace_load_uses_global_terminal_entry_kind(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            TerminalThreadMetadataStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project-a", json!({ "file.txt": "" }))
            .await;
        fs.insert_tree("/project-b", json!({ "file.txt": "" }))
            .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let project_a = Project::test(fs.clone(), [Path::new("/project-a")], cx).await;
        let project_b = Project::test(fs.clone(), [Path::new("/project-b")], cx).await;
        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
        let multi_workspace_entity = multi_workspace.root(cx).unwrap();
        let workspace_a = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        workspace_a.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });
        panel_a
            .update_in(cx, |panel, window, cx| {
                panel.insert_test_terminal("Dev Server", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        cx.update(|_window, cx| {
            assert_eq!(
                read_global_last_created_entry_kind(&KeyValueStore::global(cx)),
                Some(AgentPanelEntryKind::Terminal)
            );
        });

        let workspace_b = multi_workspace_entity.update_in(cx, |multi_workspace, window, cx| {
            multi_workspace.test_add_workspace(project_b.clone(), window, cx)
        });
        workspace_b.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded = AgentPanel::load(workspace_b.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        workspace_b.update_in(cx, |workspace, window, cx| {
            workspace.add_panel(loaded.clone(), window, cx);
        });
        loaded.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });
        for _ in 0..8 {
            cx.run_until_parked();
        }

        loaded.read_with(cx, |panel, cx| {
            assert!(
                panel.active_terminal_id().is_some(),
                "new workspace should initialize to a terminal when terminal was the globally last used entry kind"
            );
            assert!(
                panel.active_conversation_view().is_none(),
                "new workspace should not initialize to a draft when terminal is the global entry kind"
            );
            assert!(panel.should_create_terminal_for_new_entry(cx));
        });
    }

    #[gpui::test]
    async fn test_non_native_thread_without_metadata_is_not_restored(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });

        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.active_agent_thread(cx).is_some(),
                "should have an active thread after connection"
            );
        });

        // Serialize without ever sending a message, so no thread metadata exists.
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        cx.run_until_parked();

        loaded.read_with(cx, |panel, _cx| {
            assert!(
                panel.active_conversation_view().is_none(),
                "thread without metadata should not be restored; the panel should have no active thread"
            );
        });
    }

    #[gpui::test]
    async fn test_serialize_preserves_session_id_in_load_error(cx: &mut TestAppContext) {
        use crate::conversation_view::tests::FlakyAgentServer;
        use crate::thread_metadata_store::{ThreadId, ThreadMetadata};
        use chrono::Utc;
        use project::{AgentId as ProjectAgentId, WorktreePaths};

        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();
        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });
        let workspace_id = workspace
            .read_with(cx, |workspace, _cx| workspace.database_id())
            .expect("workspace should have a database id");

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        // Simulate a previous run that persisted metadata for this session.
        let resume_session_id = acp::SessionId::new("persistent-session");
        cx.update(|_window, cx| {
            ThreadMetadataStore::global(cx).update(cx, |store, cx| {
                store.save(
                    ThreadMetadata {
                        thread_id: ThreadId::new(),
                        session_id: Some(resume_session_id.clone()),
                        agent_id: ProjectAgentId::new("Flaky"),
                        title: Some("Persistent chat".into()),
                        title_override: None,
                        updated_at: Utc::now(),
                        created_at: Some(Utc::now()),
                        interacted_at: None,
                        worktree_paths: WorktreePaths::from_folder_paths(&PathList::default()),
                        remote_connection: None,
                        archived: false,
                    },
                    cx,
                );
            });
        });

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        // Open a restored thread using a flaky server so the initial connect
        // fails and the view lands in LoadError — mirroring the cold-start
        // race against a custom agent over SSH.
        let (server, _fail) =
            FlakyAgentServer::new(StubAgentConnection::new().with_supports_load_session(true));
        panel.update_in(cx, |panel, window, cx| {
            panel.open_restored_thread_with_server(
                Rc::new(server),
                resume_session_id.clone(),
                window,
                cx,
            );
        });
        cx.run_until_parked();

        // Sanity: the view couldn't connect, so no live AcpThread exists.
        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.active_agent_thread(cx).is_none(),
                "active_agent_thread should be None while the flaky server is failing"
            );
            let conversation_view = panel
                .active_conversation_view()
                .expect("panel should still have an active ConversationView");
            assert_eq!(
                conversation_view.read(cx).root_session_id.as_ref(),
                Some(&resume_session_id),
                "ConversationView should still hold the restored session id"
            );
        });

        // Serialize while in LoadError. Before the fix this wrote
        // `session_id=None` to the KVP and permanently lost the session.
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        let kvp = cx.update(|_window, cx| KeyValueStore::global(cx));
        let serialized: Option<SerializedAgentPanel> = cx
            .background_spawn(async move { read_serialized_panel(workspace_id, &kvp) })
            .await;
        let serialized_session_id = serialized
            .as_ref()
            .and_then(|p| p.last_active_thread.as_ref())
            .and_then(|t| t.session_id.clone());
        assert_eq!(
            serialized_session_id,
            Some(resume_session_id.0.to_string()),
            "serialize() must preserve the restored session id even while the \
             ConversationView is in LoadError; otherwise the bug survives a \
             restart because the KVP has been wiped"
        );
    }

    /// Extracts the text from a Text content block, panicking if it's not Text.
    fn expect_text_block(block: &acp::ContentBlock) -> &str {
        match block {
            acp::ContentBlock::Text(t) => t.text.as_str(),
            other => panic!("expected Text block, got {:?}", other),
        }
    }

    /// Extracts the (text_content, uri) from a Resource content block, panicking
    /// if it's not a TextResourceContents resource.
    fn expect_resource_block(block: &acp::ContentBlock) -> (&str, &str) {
        match block {
            acp::ContentBlock::Resource(r) => match &r.resource {
                acp::EmbeddedResourceResource::TextResourceContents(t) => {
                    (t.text.as_str(), t.uri.as_str())
                }
                other => panic!("expected TextResourceContents, got {:?}", other),
            },
            other => panic!("expected Resource block, got {:?}", other),
        }
    }

    #[gpui::test]
    async fn test_draft_prompt_blocks_use_current_editor_snapshot(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        let _stub_connection =
            crate::test_support::set_stub_agent_connection(StubAgentConnection::new());
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        let thread_id = active_thread_id(&panel, cx);
        let thread = panel.read_with(cx, |panel, cx| {
            panel
                .active_agent_thread(cx)
                .expect("draft thread should be active")
        });
        let message_editor = panel.read_with(cx, |panel, cx| {
            panel
                .active_thread_view(cx)
                .expect("draft thread view should be active")
                .read(cx)
                .message_editor
                .clone()
        });

        thread.update(cx, |thread, cx| {
            thread.set_draft_prompt(
                Some(vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "stale prompt",
                ))]),
                cx,
            );
        });
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("fresh prompt", window, cx);
        });
        let blocks = panel.read_with(cx, |panel, cx| {
            panel
                .draft_prompt_blocks_if_in_memory(thread_id, cx)
                .expect("draft should be in memory")
        });
        assert_eq!(blocks.len(), 1);
        assert_eq!(expect_text_block(&blocks[0]), "fresh prompt");

        thread.update(cx, |thread, cx| {
            thread.set_draft_prompt(
                Some(vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "stale prompt after clear",
                ))]),
                cx,
            );
        });
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("", window, cx);
        });
        let blocks = panel.read_with(cx, |panel, cx| {
            panel
                .draft_prompt_blocks_if_in_memory(thread_id, cx)
                .expect("draft should be in memory")
        });
        assert!(
            blocks.is_empty(),
            "cleared editor snapshot should override stale saved draft prompt"
        );
    }

    #[gpui::test]
    async fn test_draft_has_user_content_checks_all_live_copies(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        fs.insert_tree("/project_b", json!({ "file.txt": "" }))
            .await;
        let project_a = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;
        let project_b = Project::test(fs.clone(), [Path::new("/project_b")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
        let workspace_a = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        let _stub_connection =
            crate::test_support::set_stub_agent_connection(StubAgentConnection::new());
        panel_a.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        let thread_id = active_thread_id(&panel_a, cx);

        panel_b.update_in(cx, |panel, window, cx| {
            panel.load_agent_thread(
                Agent::Stub,
                thread_id,
                Some(PathList::new(&[PathBuf::from("/project_b")])),
                None,
                false,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        crate::test_support::type_draft_prompt(&panel_b, "content in second panel", cx);
        let panel_a_blocks = panel_a.read_with(cx, |panel, cx| {
            panel
                .draft_prompt_blocks_if_in_memory(thread_id, cx)
                .expect("draft should be live in first panel")
        });
        assert!(
            panel_a_blocks.is_empty(),
            "first live draft copy should be empty"
        );

        let has_user_content = cx.update(|_, cx| {
            crate::draft_prompt_store::draft_has_user_content(
                thread_id,
                [&workspace_a, &workspace_b],
                cx,
            )
        });
        assert!(
            has_user_content,
            "a later live draft copy with content should keep the draft"
        );
    }

    #[test]
    fn test_build_conflict_resolution_prompt_single_conflict() {
        let conflicts = vec![ConflictContent {
            file_path: "src/main.rs".to_string(),
            conflict_text: "<<<<<<< HEAD\nlet x = 1;\n=======\nlet x = 2;\n>>>>>>> feature"
                .to_string(),
            ours_branch_name: "HEAD".to_string(),
            theirs_branch_name: "feature".to_string(),
        }];

        let blocks = build_conflict_resolution_prompt(&conflicts);
        // 2 Text blocks + 1 ResourceLink + 1 Resource for the conflict
        assert_eq!(
            blocks.len(),
            4,
            "expected 2 text + 1 resource link + 1 resource block"
        );

        let intro_text = expect_text_block(&blocks[0]);
        assert!(
            intro_text.contains("Please resolve the following merge conflict in"),
            "prompt should include single-conflict intro text"
        );

        match &blocks[1] {
            acp::ContentBlock::ResourceLink(link) => {
                assert!(
                    link.uri.contains("file://"),
                    "resource link URI should use file scheme"
                );
                assert!(
                    link.uri.contains("main.rs"),
                    "resource link URI should reference file path"
                );
            }
            other => panic!("expected ResourceLink block, got {:?}", other),
        }

        let body_text = expect_text_block(&blocks[2]);
        assert!(
            body_text.contains("`HEAD` (ours)"),
            "prompt should mention ours branch"
        );
        assert!(
            body_text.contains("`feature` (theirs)"),
            "prompt should mention theirs branch"
        );
        assert!(
            body_text.contains("editing the file directly"),
            "prompt should instruct the agent to edit the file"
        );

        let (resource_text, resource_uri) = expect_resource_block(&blocks[3]);
        assert!(
            resource_text.contains("<<<<<<< HEAD"),
            "resource should contain the conflict text"
        );
        assert!(
            resource_uri.contains("merge-conflict"),
            "resource URI should use the merge-conflict scheme"
        );
        assert!(
            resource_uri.contains("main.rs"),
            "resource URI should reference the file path"
        );
    }

    #[test]
    fn test_build_conflict_resolution_prompt_multiple_conflicts_same_file() {
        let conflicts = vec![
            ConflictContent {
                file_path: "src/lib.rs".to_string(),
                conflict_text: "<<<<<<< main\nfn a() {}\n=======\nfn a_v2() {}\n>>>>>>> dev"
                    .to_string(),
                ours_branch_name: "main".to_string(),
                theirs_branch_name: "dev".to_string(),
            },
            ConflictContent {
                file_path: "src/lib.rs".to_string(),
                conflict_text: "<<<<<<< main\nfn b() {}\n=======\nfn b_v2() {}\n>>>>>>> dev"
                    .to_string(),
                ours_branch_name: "main".to_string(),
                theirs_branch_name: "dev".to_string(),
            },
        ];

        let blocks = build_conflict_resolution_prompt(&conflicts);
        // 1 Text instruction + 2 Resource blocks
        assert_eq!(blocks.len(), 3, "expected 1 text + 2 resource blocks");

        let text = expect_text_block(&blocks[0]);
        assert!(
            text.contains("all 2 merge conflicts"),
            "prompt should mention the total count"
        );
        assert!(
            text.contains("`main` (ours)"),
            "prompt should mention ours branch"
        );
        assert!(
            text.contains("`dev` (theirs)"),
            "prompt should mention theirs branch"
        );
        // Single file, so "file" not "files"
        assert!(
            text.contains("file directly"),
            "single file should use singular 'file'"
        );

        let (resource_a, _) = expect_resource_block(&blocks[1]);
        let (resource_b, _) = expect_resource_block(&blocks[2]);
        assert!(
            resource_a.contains("fn a()"),
            "first resource should contain first conflict"
        );
        assert!(
            resource_b.contains("fn b()"),
            "second resource should contain second conflict"
        );
    }

    #[test]
    fn test_build_conflict_resolution_prompt_multiple_conflicts_different_files() {
        let conflicts = vec![
            ConflictContent {
                file_path: "src/a.rs".to_string(),
                conflict_text: "<<<<<<< main\nA\n=======\nB\n>>>>>>> dev".to_string(),
                ours_branch_name: "main".to_string(),
                theirs_branch_name: "dev".to_string(),
            },
            ConflictContent {
                file_path: "src/b.rs".to_string(),
                conflict_text: "<<<<<<< main\nC\n=======\nD\n>>>>>>> dev".to_string(),
                ours_branch_name: "main".to_string(),
                theirs_branch_name: "dev".to_string(),
            },
        ];

        let blocks = build_conflict_resolution_prompt(&conflicts);
        // 1 Text instruction + 2 Resource blocks
        assert_eq!(blocks.len(), 3, "expected 1 text + 2 resource blocks");

        let text = expect_text_block(&blocks[0]);
        assert!(
            text.contains("files directly"),
            "multiple files should use plural 'files'"
        );

        let (_, uri_a) = expect_resource_block(&blocks[1]);
        let (_, uri_b) = expect_resource_block(&blocks[2]);
        assert!(
            uri_a.contains("a.rs"),
            "first resource URI should reference a.rs"
        );
        assert!(
            uri_b.contains("b.rs"),
            "second resource URI should reference b.rs"
        );
    }

    #[test]
    fn test_build_conflicted_files_resolution_prompt_file_paths_only() {
        let file_paths = vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "tests/integration.rs".to_string(),
        ];

        let blocks = build_conflicted_files_resolution_prompt(&file_paths);
        // 1 instruction Text block + (ResourceLink + newline Text) per file
        assert_eq!(
            blocks.len(),
            1 + (file_paths.len() * 2),
            "expected instruction text plus resource links and separators"
        );

        let text = expect_text_block(&blocks[0]);
        assert!(
            text.contains("unresolved merge conflicts"),
            "prompt should describe the task"
        );
        assert!(
            text.contains("conflict markers"),
            "prompt should mention conflict markers"
        );

        for (index, path) in file_paths.iter().enumerate() {
            let link_index = 1 + (index * 2);
            let newline_index = link_index + 1;

            match &blocks[link_index] {
                acp::ContentBlock::ResourceLink(link) => {
                    assert!(
                        link.uri.contains("file://"),
                        "resource link URI should use file scheme"
                    );
                    assert!(
                        link.uri.contains(path),
                        "resource link URI should reference file path: {path}"
                    );
                }
                other => panic!(
                    "expected ResourceLink block at index {}, got {:?}",
                    link_index, other
                ),
            }

            let separator = expect_text_block(&blocks[newline_index]);
            assert_eq!(
                separator, "\n",
                "expected newline separator after each file"
            );
        }
    }

    #[test]
    fn test_build_conflict_resolution_prompt_empty_conflicts() {
        let blocks = build_conflict_resolution_prompt(&[]);
        assert!(
            blocks.is_empty(),
            "empty conflicts should produce no blocks, got {} blocks",
            blocks.len()
        );
    }

    #[test]
    fn test_build_conflicted_files_resolution_prompt_empty_paths() {
        let blocks = build_conflicted_files_resolution_prompt(&[]);
        assert!(
            blocks.is_empty(),
            "empty paths should produce no blocks, got {} blocks",
            blocks.len()
        );
    }

    #[test]
    fn test_conflict_resource_block_structure() {
        let conflict = ConflictContent {
            file_path: "src/utils.rs".to_string(),
            conflict_text: "<<<<<<< HEAD\nold code\n=======\nnew code\n>>>>>>> branch".to_string(),
            ours_branch_name: "HEAD".to_string(),
            theirs_branch_name: "branch".to_string(),
        };

        let block = conflict_resource_block(&conflict);
        let (text, uri) = expect_resource_block(&block);

        assert_eq!(
            text, conflict.conflict_text,
            "resource text should be the raw conflict"
        );
        assert!(
            uri.starts_with("zed:///agent/merge-conflict"),
            "URI should use the zed merge-conflict scheme, got: {uri}"
        );
        assert!(uri.contains("utils.rs"), "URI should encode the file path");
    }

    fn open_generating_thread_with_loadable_connection(
        panel: &Entity<AgentPanel>,
        connection: &StubAgentConnection,
        cx: &mut VisualTestContext,
    ) -> (acp::SessionId, ThreadId) {
        open_thread_with_custom_connection(panel, connection.clone(), cx);
        let session_id = active_session_id(panel, cx);
        let thread_id = active_thread_id(panel, cx);
        send_message(panel, cx);
        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("done".into())),
                cx,
            );
        });
        cx.run_until_parked();
        (session_id, thread_id)
    }

    fn open_idle_thread_with_non_loadable_connection(
        panel: &Entity<AgentPanel>,
        connection: &StubAgentConnection,
        cx: &mut VisualTestContext,
    ) -> (acp::SessionId, ThreadId) {
        open_thread_with_custom_connection(panel, connection.clone(), cx);
        let session_id = active_session_id(panel, cx);
        let thread_id = active_thread_id(panel, cx);

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("done".into()),
        )]);
        send_message(panel, cx);

        (session_id, thread_id)
    }

    #[gpui::test]
    async fn test_draft_promotion_creates_metadata_and_new_session_on_reload(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Register a shared stub connection and use Agent::Stub so the draft
        // (and any reloaded draft) uses it.
        let stub_connection =
            crate::test_support::set_stub_agent_connection(StubAgentConnection::new());
        stub_connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response".into()),
        )]);
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        // Verify the thread is considered a draft.
        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.active_thread_is_draft(cx),
                "thread should be a draft before any message is sent"
            );
            assert!(
                panel.draft_thread.is_some(),
                "draft_thread field should be set"
            );
        });
        let draft_session_id = active_session_id(&panel, cx);
        let thread_id = active_thread_id(&panel, cx);

        // A draft thread is persisted with session_id: None.
        cx.update(|_window, cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            let entry = store
                .entry(thread_id)
                .expect("draft thread should have a metadata row");
            assert!(
                entry.is_draft(),
                "draft thread metadata should have session_id=None, got {:?}",
                entry.session_id,
            );
        });

        // Type into the message editor; the editor observer pushes the text
        // into `AcpThread.draft_prompt`, which emits `PromptUpdated` and
        // persists the prompt to the kvp store.
        crate::test_support::type_draft_prompt(&panel, "Hello from draft", cx);
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let reloaded_panel = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load with draft should succeed");
        cx.run_until_parked();

        reloaded_panel.read_with(cx, |panel, cx| {
            assert!(
                panel.active_thread_is_draft(cx),
                "reloaded panel should still show the draft as active"
            );
            assert!(
                panel.active_view_is_new_draft(cx),
                "reloaded draft should still occupy the new-draft slot: \
                 what's in the new-draft slot stays there across restarts, \
                 regardless of whether it's also the active view"
            );
            let active_entity = panel.active_conversation_view().map(|v| v.entity_id());
            let draft_entity = panel.draft_thread.as_ref().map(|v| v.entity_id());
            assert!(
                active_entity.is_some() && active_entity == draft_entity,
                "active view and draft slot should share a single ConversationView entity \
                 (active={active_entity:?}, draft={draft_entity:?})"
            );
        });

        // Thread identity is stable across reload — the metadata row we wrote
        // pre-reload maps back to the same ConversationView.
        let reloaded_thread_id = active_thread_id(&reloaded_panel, cx);
        assert_eq!(
            reloaded_thread_id, thread_id,
            "reloaded draft should preserve its ThreadId"
        );

        // ACP session_id is NOT preserved: drafts don't persist a session id,
        // so the reloaded ConversationView opens a fresh ACP session.
        let reloaded_session_id = active_session_id(&reloaded_panel, cx);
        assert_ne!(
            reloaded_session_id, draft_session_id,
            "reloaded draft should have a fresh ACP session ID"
        );

        let restored_text =
            reloaded_panel.read_with(cx, |panel, cx| panel.editor_text(reloaded_thread_id, cx));
        assert_eq!(
            restored_text.as_deref(),
            Some("Hello from draft"),
            "draft prompt text should be restored from the draft-prompt kvp store"
        );

        // Send a message on the reloaded panel — this promotes the draft to a
        // real thread. `ThreadId` stays the same; `session_id` is populated.
        let panel = reloaded_panel;
        let promoted_session_id = reloaded_session_id;
        send_message(&panel, cx);

        panel.read_with(cx, |panel, cx| {
            assert!(
                !panel.active_thread_is_draft(cx),
                "thread should no longer be a draft after sending a message"
            );
            assert!(
                panel.draft_thread.is_none(),
                "draft_thread should be None after promotion"
            );
            assert_eq!(
                panel.active_thread_id(cx),
                Some(thread_id),
                "same ThreadId should remain active after promotion"
            );
        });

        cx.update(|_window, cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            let metadata = store
                .entry(thread_id)
                .expect("promoted thread should have metadata");
            assert!(
                !metadata.is_draft(),
                "promoted thread metadata should no longer be a draft"
            );
            assert_eq!(
                metadata.session_id.as_ref(),
                Some(&promoted_session_id),
                "metadata session_id should match the thread's ACP session"
            );
        });

        // Serialize the panel, then reload it again.
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_panel = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        cx.run_until_parked();

        // The second load should restore the promoted real thread, keyed by
        // its session_id.
        loaded_panel.read_with(cx, |panel, cx| {
            let active_id = panel.active_thread_id(cx);
            assert_eq!(
                active_id,
                Some(thread_id),
                "loaded panel should restore the promoted thread"
            );
            assert!(
                !panel.active_thread_is_draft(cx),
                "restored thread should not be a draft"
            );
        });
    }

    #[gpui::test]
    async fn test_new_draft_survives_reload_when_real_thread_is_active(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();
        workspace.update(cx, |workspace, _cx| workspace.set_random_database_id());

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Register a shared stub connection under `Agent::Stub` so every
        // ConversationView the panel creates in this test (including any
        // post-reload rehydrations) reaches Connected synchronously.
        let stub_connection =
            crate::test_support::set_stub_agent_connection(StubAgentConnection::new());
        stub_connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("ok".into()),
        )]);

        // 1. Create a real thread by sending a message.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        crate::test_support::send_message(&panel, cx);
        let real_thread_id = crate::test_support::active_thread_id(&panel, cx);
        let real_session_id = crate::test_support::active_session_id(&panel, cx);
        cx.run_until_parked();

        // 2. Open a draft, type into it, then press Cmd-N again to
        //    park it into retained_threads as a *retained* draft.
        panel.update_in(cx, |panel, window, cx| {
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        let retained_draft_id = crate::test_support::active_thread_id(&panel, cx);
        crate::test_support::type_draft_prompt(&panel, "retained draft text", cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });
        cx.run_until_parked();

        // The pre-existing draft is now in retained_threads (parked),
        // and a fresh empty ephemeral new-draft is active.
        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.retained_threads.contains_key(&retained_draft_id),
                "first draft with content should be parked into retained_threads"
            );
            assert_ne!(
                panel.active_thread_id(cx),
                Some(retained_draft_id),
                "active view should be a fresh ephemeral draft, not the retained one"
            );
        });

        // 3. Type into the new ephemeral draft.
        let draft_thread_id = crate::test_support::active_thread_id(&panel, cx);
        crate::test_support::type_draft_prompt(&panel, "in-flight draft text", cx);

        // Sanity-check: both drafts' text has been persisted to the kvp
        // store via the editor observer / PromptUpdated chain.
        let (ephemeral_kvp, retained_kvp) = cx.update(|_, cx| {
            (
                crate::draft_prompt_store::read(draft_thread_id, cx),
                crate::draft_prompt_store::read(retained_draft_id, cx),
            )
        });
        assert!(
            ephemeral_kvp.is_some(),
            "ephemeral draft's prompt should be in the kvp store"
        );
        assert!(
            retained_kvp.is_some(),
            "retained draft's prompt should be in the kvp store"
        );

        assert_ne!(real_thread_id, draft_thread_id);
        assert_ne!(retained_draft_id, draft_thread_id);
        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.active_view_is_new_draft(cx),
                "draft should currently occupy the new-draft slot"
            );
        });

        // 4. Switch the active view back to the real thread. The ephemeral
        //    draft has content, so it gets parked into `retained_threads`
        //    immediately (the `draft_thread` slot is cleared).
        panel.update_in(cx, |panel, window, cx| {
            panel.load_agent_thread(
                Agent::Stub,
                real_thread_id,
                None,
                None,
                false,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            assert_eq!(panel.active_thread_id(cx), Some(real_thread_id));
            assert!(!panel.active_view_is_new_draft(cx));
        });

        // 5. Serialize + reload.
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();
        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_panel = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        cx.run_until_parked();

        // 6. The real thread is the active view on reload. The draft
        //    was parked when the user navigated away, so the draft_thread
        //    slot is empty.
        loaded_panel.read_with(cx, |panel, cx| {
            assert_eq!(
                panel.active_thread_id(cx),
                Some(real_thread_id),
                "real thread should be the active view after reload"
            );
            assert!(
                !panel.active_thread_is_draft(cx),
                "real thread is not a draft"
            );
            assert!(
                panel.draft_thread.is_none(),
                "draft_thread slot should be empty since the draft was parked on navigate-away"
            );
        });

        // 7. All three threads' metadata rows survive the reload.
        cx.update(|_window, cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            let ephemeral_row = store
                .entry(draft_thread_id)
                .expect("ephemeral draft metadata row should survive reload");
            assert!(
                ephemeral_row.is_draft(),
                "ephemeral draft row should still be a draft"
            );
            let retained_row = store
                .entry(retained_draft_id)
                .expect("retained draft metadata row should survive reload");
            assert!(
                retained_row.is_draft(),
                "retained draft row should still be a draft"
            );
            let real_row = store
                .entry(real_thread_id)
                .expect("real thread metadata row should survive reload");
            assert_eq!(real_row.session_id.as_ref(), Some(&real_session_id));
        });

        // 8. Opening the parked draft via load_agent_thread activates
        //    a fresh ConversationView and exposes its kvp-seeded prompt
        //    text in the editor.
        loaded_panel.update_in(cx, |panel, window, cx| {
            panel.load_agent_thread(
                Agent::Stub,
                draft_thread_id,
                None,
                None,
                false,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        let restored_ephemeral_text =
            loaded_panel.read_with(cx, |panel, cx| panel.editor_text(draft_thread_id, cx));
        assert_eq!(
            restored_ephemeral_text.as_deref(),
            Some("in-flight draft text"),
            "ephemeral draft prompt text should be restored from the kvp store"
        );

        // 9. Opening the retained draft via load_agent_thread builds a
        //    fresh ConversationView (since retained_threads was not
        //    carried across the reload) and seeds its editor from the
        //    kvp store.
        loaded_panel.update_in(cx, |panel, window, cx| {
            panel.load_agent_thread(
                Agent::Stub,
                retained_draft_id,
                None,
                None,
                false,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        let restored_retained_text =
            loaded_panel.read_with(cx, |panel, cx| panel.editor_text(retained_draft_id, cx));
        assert_eq!(
            restored_retained_text.as_deref(),
            Some("retained draft text"),
            "retained draft prompt text should be restored from the kvp store"
        );
    }

    #[gpui::test]
    async fn test_reloaded_ephemeral_draft_preserves_original_agent(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();
        workspace.update(cx, |workspace, _cx| workspace.set_random_database_id());

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        let _stub_connection =
            crate::test_support::set_stub_agent_connection(StubAgentConnection::new());
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        let draft_thread_id = crate::test_support::active_thread_id(&panel, cx);
        crate::test_support::type_draft_prompt(&panel, "pinned to stub", cx);

        // Diverge `selected_agent` from the draft's bound agent before
        // serialize.
        let other_agent = Agent::Custom {
            id: "other-agent".into(),
        };
        panel.update(cx, |panel, _cx| {
            panel.selected_agent = other_agent.clone();
        });
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        // Sanity-check: the draft's metadata row has agent_id="stub",
        // not "other-agent".
        cx.update(|_, cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            let row = store
                .entry(draft_thread_id)
                .expect("draft metadata row should exist");
            assert_eq!(
                row.agent_id.as_ref(),
                "stub",
                "draft metadata should retain its original agent binding"
            );
        });

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let reloaded_panel = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        cx.run_until_parked();

        reloaded_panel.read_with(cx, |panel, cx| {
            let draft_view = panel
                .draft_thread
                .as_ref()
                .expect("draft slot should be repopulated");
            assert_eq!(
                draft_view.read(cx).thread_id,
                draft_thread_id,
                "restored draft should have the same ThreadId"
            );
            assert_eq!(
                draft_view.read(cx).agent_key(),
                &Agent::Stub,
                "restored draft should still be bound to its original Agent::Stub, \
                 not the panel's current `selected_agent`"
            );
        });
    }

    #[gpui::test]
    async fn test_empty_workspace_does_not_create_agent_entries(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;
        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        panel.read_with(cx, |panel, cx| {
            assert_eq!(
                panel
                    .connection_store()
                    .read(cx)
                    .connection_status(&Agent::NativeAgent, cx),
                crate::agent_connection_store::AgentConnectionStatus::Disconnected,
                "empty workspaces should not start the native agent connection"
            );
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
            panel.new_external_agent_thread(
                &NewExternalAgentThread {
                    agent: AgentId::new("external-agent"),
                },
                window,
                cx,
            );
        });
        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.active_conversation_view().is_none(),
                "empty workspaces should not create agent threads"
            );
            assert!(
                panel.draft_thread.is_none(),
                "empty workspaces should not create draft threads"
            );
            assert!(
                panel.terminals(cx).is_empty(),
                "empty workspaces should not create agent panel terminals"
            );
        });

        cx.update(|_, cx| {
            cx.update_flags(true, vec!["agent-panel-terminal".to_string()]);
        });
        panel.update_in(cx, |panel, window, cx| {
            panel.new_terminal(None, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.terminals(cx).is_empty(),
                "empty workspaces should not create terminals after the terminal feature is enabled"
            );
            assert_eq!(
                panel
                    .connection_store()
                    .read(cx)
                    .connection_status(&Agent::NativeAgent, cx),
                crate::agent_connection_store::AgentConnectionStatus::Disconnected,
                "empty workspace actions should not start the native agent connection"
            );
        });
    }

    #[gpui::test]
    async fn test_add_selection_to_terminal_thread_pastes_mention(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({ "file.rs": "line one\nline two\nline three\n" }),
        )
        .await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Make a terminal thread the active conversation. A display-only terminal
        // avoids spawning a real shell; its working directory is supplied directly
        // so the mention resolves relative to it. No agent is started inside it.
        let terminal_id = TerminalId::new();
        panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_display_only_terminal(
                    terminal_id,
                    Some(PathBuf::from("/project")),
                    Some("Terminal".into()),
                    None,
                    None,
                    true,
                    true,
                    AgentThreadSource::AgentPanel,
                    window,
                    cx,
                )
            })
            .expect("display-only terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.active_terminal_id(), Some(terminal_id));
            assert!(panel.active_conversation_view().is_none());
        });

        // Open the file in the center pane so the selection comes from a
        // worktree-backed editor (with a project path).
        workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![PathBuf::from("/project/file.rs")],
                    workspace::OpenOptions::default(),
                    None,
                    window,
                    cx,
                )
            })
            .await;
        cx.run_until_parked();

        let editor = workspace.update(&mut cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
                .expect("opened file should be an editor")
        });

        cx.focus(&editor);
        cx.run_until_parked();

        let terminal = panel.read_with(&cx, |panel, cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should exist")
                .view
                .read(cx)
                .terminal()
                .clone()
        });
        // Drop any input the terminal may have received during setup.
        terminal.update(&mut cx, |terminal, _| {
            terminal.take_input_log();
        });

        // With only a cursor and nothing highlighted, the action is a no-op and
        // must not paste anything into the terminal.
        workspace.update_in(&mut cx, |_, window, cx| {
            window.dispatch_action(AddSelectionToThread.boxed_clone(), cx);
        });
        cx.run_until_parked();
        let pasted_without_selection =
            terminal.update(&mut cx, |terminal, _| terminal.take_input_log());
        assert!(
            pasted_without_selection.is_empty(),
            "no selection should paste nothing, got {pasted_without_selection:?}"
        );

        // Now highlight a portion of the file: from the start of line 2 into line 3.
        editor.update_in(&mut cx, |editor, window, cx| {
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges([text::Point::new(1, 0)..text::Point::new(2, 4)]);
            });
        });
        cx.run_until_parked();

        workspace.update_in(&mut cx, |_, window, cx| {
            window.dispatch_action(AddSelectionToThread.boxed_clone(), cx);
        });
        cx.run_until_parked();

        let pasted: String = terminal
            .update(&mut cx, |terminal, _| terminal.take_input_log())
            .into_iter()
            .map(|bytes| String::from_utf8(bytes).expect("pasted bytes should be valid UTF-8"))
            .collect();

        // Lines are 1-based and inclusive; the path is presented as
        // `<rel-path>:<start>-<end>`, with a trailing space.
        assert_eq!(pasted, "file.rs:2-3 ");
    }

    async fn setup_panel(cx: &mut TestAppContext) -> (Entity<AgentPanel>, VisualTestContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        (panel, cx)
    }

    async fn setup_visible_panel(
        cx: &mut TestAppContext,
    ) -> (Entity<AgentPanel>, VisualTestContext) {
        setup_visible_panel_with_sidebar(cx, true).await
    }

    async fn setup_visible_panel_with_sidebar(
        cx: &mut TestAppContext,
        threads_list_active: bool,
    ) -> (Entity<AgentPanel>, VisualTestContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::PrimaryScreen,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);
        register_test_sidebar(threads_list_active, &mut cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            workspace.focus_panel::<AgentPanel>(window, cx);
            panel
        });

        (panel, cx)
    }

    fn expected_terminal_drop_text(paths: &[PathBuf]) -> String {
        let mut text = String::new();
        for path in paths {
            text.push(' ');
            text.push_str(&format!("{path:?}"));
        }
        text.push(' ');
        text
    }

    #[gpui::test]
    async fn test_terminal_external_image_drop_writes_path(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        cx.update(|_, cx| {
            cx.update_flags(true, vec!["agent-panel-terminal".to_string()]);
        });

        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Image Upload", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        let terminal = panel.read_with(&cx, |panel, cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel")
                .view
                .read(cx)
                .terminal()
                .clone()
        });
        terminal.update(&mut cx, |terminal, _cx| terminal.take_input_log());

        let image_path = PathBuf::from("/tmp/dropped-image.png");
        panel.update_in(&mut cx, |panel, window, cx| {
            let external_paths = ExternalPaths(vec![image_path.clone()].into());
            panel.paste_external_paths_into_active_terminal(&external_paths, window, cx);
        });

        let mut input_log = terminal.update(&mut cx, |terminal, _cx| terminal.take_input_log());
        assert_eq!(input_log.len(), 1, "expected one write to the terminal");
        let written =
            String::from_utf8(input_log.remove(0)).expect("terminal write should be valid UTF-8");
        assert_eq!(
            written,
            expected_terminal_drop_text(std::slice::from_ref(&image_path))
        );
    }

    #[gpui::test]
    async fn test_terminal_external_paths_drop_handler_writes_image_path(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        cx.update(|_, cx| {
            cx.update_flags(true, vec!["agent-panel-terminal".to_string()]);
        });

        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Image Upload", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        let terminal = panel.read_with(&cx, |panel, cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel")
                .view
                .read(cx)
                .terminal()
                .clone()
        });
        terminal.update(&mut cx, |terminal, _cx| terminal.take_input_log());

        let image_path = PathBuf::from("/tmp/dropped-image.png");
        panel.update_in(&mut cx, |panel, window, cx| {
            let external_paths = ExternalPaths(vec![image_path.clone()].into());
            panel.handle_external_paths_drop(&external_paths, window, cx);
        });

        let mut input_log = terminal.update(&mut cx, |terminal, _cx| terminal.take_input_log());
        assert_eq!(input_log.len(), 1, "expected one write to the terminal");
        let written =
            String::from_utf8(input_log.remove(0)).expect("terminal write should be valid UTF-8");
        assert_eq!(
            written,
            expected_terminal_drop_text(std::slice::from_ref(&image_path))
        );
    }

    #[gpui::test]
    async fn test_external_file_drop_on_thread_does_not_paste_into_later_terminal(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            cx.update_flags(true, vec!["agent-panel-terminal".to_string()]);
        });

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        fs.insert_tree("/project", json!({ "file.txt": "content" }))
            .await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        open_thread_with_connection(&panel, StubAgentConnection::new(), &mut cx);
        let thread_id = active_thread_id(&panel, &cx);

        let file_path = PathBuf::from("/project/file.txt");
        panel.update_in(&mut cx, |panel, window, cx| {
            let external_paths = ExternalPaths(vec![file_path.clone()].into());
            panel.handle_external_paths_drop(&external_paths, window, cx);
        });

        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Drop Target", true, window, cx)
            })
            .expect("test terminal should be inserted");
        let terminal = panel.read_with(&cx, |panel, cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel")
                .view
                .read(cx)
                .terminal()
                .clone()
        });
        terminal.update(&mut cx, |terminal, _cx| terminal.take_input_log());

        cx.run_until_parked();

        let input_log = terminal.update(&mut cx, |terminal, _cx| terminal.take_input_log());
        assert!(
            input_log.is_empty(),
            "thread drop completion should not write to the active terminal"
        );

        let expected_uri = MentionUri::File {
            abs_path: file_path,
        }
        .to_uri()
        .to_string();
        let expected_text = format!("[@file.txt]({expected_uri}) ");
        let actual_text = panel.read_with(&cx, |panel, cx| panel.editor_text(thread_id, cx));
        assert_eq!(actual_text.as_deref(), Some(expected_text.as_str()));
    }

    #[gpui::test]
    async fn test_terminal_entry_kind_controls_new_entry(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        panel.read_with(&cx, |panel, cx| {
            assert!(panel.project.read(cx).supports_terminal(cx));
            assert!(!panel.should_create_terminal_for_new_entry(cx));
        });

        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Dev Server", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            assert_eq!(panel.active_terminal_id(), Some(terminal_id));
            assert!(panel.has_terminal(terminal_id));
            assert!(panel.should_create_terminal_for_new_entry(cx));
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "Dev Server");
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.activate_new_thread(false, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            assert_eq!(panel.active_terminal_id(), None);
            assert!(panel.has_terminal(terminal_id));
            assert!(!panel.should_create_terminal_for_new_entry(cx));
        });
    }

    #[gpui::test]
    async fn test_skills_menu_entry_shows_manage_skills_shortcut(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            let default_key_bindings = settings::KeymapFile::load_asset_allow_partial_failure(
                "keymaps/default-macos.json",
                cx,
            )
            .unwrap();
            cx.bind_keys(default_key_bindings);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        open_thread_with_connection(&panel, StubAgentConnection::new(), &mut cx);
        workspace.update_in(&mut cx, |workspace, window, cx| {
            workspace.focus_panel::<AgentPanel>(window, cx);
        });
        cx.run_until_parked();

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.toggle_options_menu(&ToggleOptionsMenu, window, cx);
        });
        cx.run_until_parked();

        assert!(
            cx.debug_bounds("MENU_ITEM-Skills").is_some(),
            "Skills menu item should be visible"
        );
        assert!(
            cx.debug_bounds("KEY_BINDING-l").is_some(),
            "Skills menu item should show the ManageSkills shortcut"
        );
    }

    #[gpui::test]
    async fn test_terminal_close_event_closes_without_sidebar(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        cx.update(|_, cx| {
            TerminalThreadMetadataStore::init_global(cx);
        });

        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Dev Server", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_close(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert!(!panel.has_terminal(terminal_id));
        });
        cx.update(|_, cx| {
            assert!(
                TerminalThreadMetadataStore::global(cx)
                    .read(cx)
                    .entry(terminal_id)
                    .is_none(),
                "terminal metadata should be deleted by the fallback close"
            );
        });
    }

    #[gpui::test]
    async fn test_new_thread_dismisses_settings_overlay(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        // Put the panel on its ephemeral new-draft view so the base view
        // already contains the draft that `NewThread` would activate.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.activate_new_thread(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            assert!(
                panel.active_view_is_new_draft(cx),
                "precondition: base view should be the ephemeral draft"
            );
            assert!(!panel.is_overlay_open());
        });

        // Simulate the Settings overlay being open on top of the draft.
        // We don't go through `open_configuration` here because it would
        // build provider configuration views, which call into
        // `LanguageModelProvider::configuration_view` — unimplemented for
        // the fake provider used in tests. The bug being exercised lives
        // entirely in the overlay/base-view bookkeeping, so toggling the
        // overlay flag directly is sufficient.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.set_overlay(OverlayView::Configuration, true, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert!(
                panel.is_overlay_open(),
                "precondition: Settings overlay should be open"
            );
        });

        // Dispatching `NewThread` while Settings is open must dismiss the
        // overlay so the user actually sees the new thread. Previously
        // this was a silent no-op: `activate_draft` early-returned without
        // clearing the overlay because the base view already held the
        // draft.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            assert!(
                !panel.is_overlay_open(),
                "Settings overlay should be dismissed when invoking NewThread"
            );
            assert!(panel.active_view_is_new_draft(cx));
        });
    }

    #[gpui::test]
    async fn test_terminal_title_omits_placeholder_title(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "");
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            assert_eq!(terminal.title(cx).as_ref(), "");
        });

        let terminal_view = panel.read_with(&cx, |panel, _cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel")
                .view
                .clone()
        });
        let terminal_entity =
            terminal_view.read_with(&cx, |terminal_view, _cx| terminal_view.terminal().clone());
        terminal_entity.update(&mut cx, |_terminal, cx| {
            cx.emit(TerminalEvent::TitleChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "");
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            assert_eq!(terminal.title(cx).as_ref(), "");
        });

        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "Shell Breadcrumb".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "Shell Breadcrumb");
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            assert_eq!(terminal.title(cx).as_ref(), "Shell Breadcrumb");
        });
    }

    #[gpui::test]
    async fn test_title_edit_affordance_matches_threads_and_terminals(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.activate_draft(false, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        panel.update_in(&mut cx, |panel, window, cx| {
            assert!(matches!(
                panel.visible_surface(),
                VisibleSurface::AgentThread(_)
            ));
            assert!(panel.should_show_title_edit(window, cx));
        });

        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Dev Server", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.update_in(&mut cx, |panel, window, cx| {
            assert!(matches!(
                panel.visible_surface(),
                VisibleSurface::Terminal(_)
            ));
            assert!(panel.should_show_title_edit(window, cx));

            panel.edit_terminal_title(terminal_id, window, cx);
            assert!(!panel.should_show_title_edit(window, cx));
        });
    }

    #[gpui::test]
    async fn test_restored_terminal_uses_metadata_title_until_shell_title_arrives(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;
        let terminal_id = TerminalId::new();
        let now = Utc::now();
        let metadata = TerminalThreadMetadata {
            terminal_id,
            title: "Persisted Shell Title".into(),
            custom_title: None,
            created_at: now,
            worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                "/project",
            )])),
            remote_connection: None,
            working_directory: None,
        };

        panel.update_in(&mut cx, |panel, window, cx| {
            panel
                .restore_test_terminal(metadata, true, AgentThreadSource::Sidebar, None, window, cx)
                .expect("test terminal should be restored");
        });
        cx.run_until_parked();

        let terminal_view = panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "Persisted Shell Title");
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should be restored")
                .view
                .clone()
        });

        let terminal_entity =
            terminal_view.read_with(&cx, |terminal_view, _cx| terminal_view.terminal().clone());
        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "Fresh Shell Title".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "Fresh Shell Title");
        });
    }

    #[gpui::test]
    async fn test_restored_terminal_selects_without_focusing(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        let terminal_id = TerminalId::new();
        let now = Utc::now();
        let metadata = TerminalThreadMetadata {
            terminal_id,
            title: "Persisted Shell Title".into(),
            custom_title: None,
            created_at: now,
            worktree_paths: WorktreePaths::from_folder_paths(&PathList::new(&[PathBuf::from(
                "/project",
            )])),
            remote_connection: None,
            working_directory: None,
        };

        panel.update_in(&mut cx, |panel, window, cx| {
            panel
                .restore_test_terminal(
                    metadata,
                    false,
                    AgentThreadSource::Sidebar,
                    None,
                    window,
                    cx,
                )
                .expect("test terminal should be restored");
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.active_terminal_id(), Some(terminal_id));
        });
    }

    #[gpui::test]
    async fn test_terminal_working_directory_uses_active_workspace_while_workspace_is_updating(
        cx: &mut TestAppContext,
    ) {
        let (workspace, panel, mut cx) = setup_workspace_panel(cx).await;
        panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Dev Server", false, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            assert_eq!(panel.last_created_entry_kind, AgentPanelEntryKind::Terminal);
            assert!(panel.should_create_terminal_for_new_entry(cx));
        });

        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = workspace
                .panel::<AgentPanel>(cx)
                .expect("agent panel should be registered in workspace");
            panel.read_with(cx, |panel, cx| {
                panel.terminal_working_directory(Some(workspace), cx);
            });
            workspace.focus_panel::<AgentPanel>(window, cx);
        });

        panel.read_with(&cx, |panel, cx| {
            assert_eq!(panel.last_created_entry_kind, AgentPanelEntryKind::Terminal);
            assert!(panel.should_create_terminal_for_new_entry(cx));
        });
    }

    #[gpui::test]
    async fn test_terminal_title_editor_is_created_only_while_editing(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Dev Server", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            assert!(terminal.title_editor.is_none());
        });

        panel.update(&mut cx, |panel, cx| {
            panel.refresh_terminal_metadata(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            assert!(terminal.title_editor.is_none());
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.edit_terminal_title(terminal_id, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            let title_editor = terminal
                .title_editor
                .as_ref()
                .expect("terminal title editor should be active while editing");
            assert_eq!(title_editor.read(cx).text(cx), "Dev Server");
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.stop_editing_terminal_title(terminal_id, false, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            assert!(terminal.title_editor.is_none());
        });
    }

    #[gpui::test]
    async fn test_terminal_title_editor_does_not_set_custom_title_when_unchanged(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Initial Custom Title", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        let terminal_view = panel.read_with(&cx, |panel, _cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel")
                .view
                .clone()
        });
        terminal_view.update(&mut cx, |terminal_view, cx| {
            terminal_view.set_custom_title(None, cx);
        });
        let terminal_entity =
            terminal_view.read_with(&cx, |terminal_view, _cx| terminal_view.terminal().clone());
        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "Shell Breadcrumb".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "Shell Breadcrumb");
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.edit_terminal_title(terminal_id, window, cx);
        });
        cx.run_until_parked();

        let title_editor = panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            let title_editor = terminal
                .title_editor
                .as_ref()
                .expect("terminal title editor should be active while editing")
                .clone();
            assert_eq!(title_editor.read(cx).text(cx), "Shell Breadcrumb");
            title_editor
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.handle_terminal_title_editor_event(
                terminal_id,
                &title_editor,
                &editor::EditorEvent::BufferEdited,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        terminal_view.read_with(&cx, |terminal_view, _cx| {
            assert!(terminal_view.custom_title().is_none());
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.stop_editing_terminal_title(terminal_id, false, window, cx);
        });
        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "Updated Shell Breadcrumb".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "Updated Shell Breadcrumb");
        });
    }

    #[gpui::test]
    async fn test_terminal_custom_title_recomposes_with_live_spinner(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Fix bug", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        let terminal_entity = panel.read_with(&cx, |panel, _cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel")
                .view
                .clone()
        });
        let terminal_entity =
            terminal_entity.read_with(&cx, |terminal_view, _cx| terminal_view.terminal().clone());

        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "⠋ Thinking".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "⠋ Fix bug");
            let metadata = panel
                .terminal_metadata(terminal_id, cx)
                .expect("terminal metadata should be available");
            assert_eq!(metadata.title.as_ref(), "⠋ Thinking");
            assert_eq!(
                metadata.custom_title.as_ref().map(|title| title.as_ref()),
                Some("Fix bug")
            );
            assert_eq!(metadata.display_title().as_ref(), "⠋ Fix bug");
        });

        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "⠙ Thinking".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "⠙ Fix bug");
            let metadata = panel
                .terminal_metadata(terminal_id, cx)
                .expect("terminal metadata should be available");
            assert_eq!(metadata.title.as_ref(), "⠙ Thinking");
            assert_eq!(metadata.display_title().as_ref(), "⠙ Fix bug");
        });

        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "Thinking".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "Fix bug");
            let metadata = panel
                .terminal_metadata(terminal_id, cx)
                .expect("terminal metadata should be available");
            assert_eq!(metadata.title.as_ref(), "Thinking");
            assert_eq!(metadata.display_title().as_ref(), "Fix bug");
        });
    }

    #[gpui::test]
    async fn test_terminal_title_editor_excludes_spinner_prefix(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Initial Custom Title", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        let terminal_view = panel.read_with(&cx, |panel, _cx| {
            panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel")
                .view
                .clone()
        });
        terminal_view.update(&mut cx, |terminal_view, cx| {
            terminal_view.set_custom_title(None, cx);
        });
        let terminal_entity =
            terminal_view.read_with(&cx, |terminal_view, _cx| terminal_view.terminal().clone());
        terminal_entity.update(&mut cx, |terminal, cx| {
            terminal.breadcrumb_text = "⠋ Thinking".to_string();
            cx.emit(TerminalEvent::BreadcrumbsChanged);
        });
        cx.run_until_parked();

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.edit_terminal_title(terminal_id, window, cx);
        });
        cx.run_until_parked();

        let title_editor = panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            let title_editor = terminal
                .title_editor
                .as_ref()
                .expect("terminal title editor should be active while editing")
                .clone();
            assert_eq!(title_editor.read(cx).text(cx), "Thinking");
            title_editor
        });

        title_editor.update_in(&mut cx, |editor, window, cx| {
            editor.set_text("Fix bug", window, cx);
            editor.focus_handle(cx).focus(window, cx);
        });
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.handle_terminal_title_editor_event(
                terminal_id,
                &title_editor,
                &editor::EditorEvent::BufferEdited,
                window,
                cx,
            );
        });
        cx.run_until_parked();

        terminal_view.read_with(&cx, |terminal_view, _cx| {
            assert_eq!(terminal_view.custom_title(), Some("Fix bug"));
        });
        panel.read_with(&cx, |panel, cx| {
            let terminals = panel.terminals(cx);
            assert_eq!(terminals.len(), 1);
            assert_eq!(terminals[0].title.as_ref(), "⠋ Fix bug");
            let metadata = panel
                .terminal_metadata(terminal_id, cx)
                .expect("terminal metadata should be available");
            assert_eq!(metadata.title.as_ref(), "⠋ Thinking");
            assert_eq!(
                metadata.custom_title.as_ref().map(|title| title.as_ref()),
                Some("Fix bug")
            );
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.stop_editing_terminal_title(terminal_id, false, window, cx);
            panel.edit_terminal_title(terminal_id, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals
                .get(&terminal_id)
                .expect("terminal should remain in the panel");
            let title_editor = terminal
                .title_editor
                .as_ref()
                .expect("terminal title editor should be active while editing");
            assert_eq!(title_editor.read(cx).text(cx), "Fix bug");
        });
    }

    #[gpui::test]
    async fn test_terminal_bell_marks_and_activation_clears_notification(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        let first_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Build", true, window, cx)
            })
            .expect("first test terminal should be inserted");
        let second_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Server", true, window, cx)
            })
            .expect("second test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.active_terminal_id(), Some(second_terminal_id));
        });

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(first_terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let first_terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == first_terminal_id)
                .expect("first terminal should remain in the panel");
            assert!(first_terminal.has_notification);
        });

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.activate_terminal(first_terminal_id, true, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let first_terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == first_terminal_id)
                .expect("first terminal should remain in the panel");
            assert!(!first_terminal.has_notification);
        });
    }

    #[gpui::test]
    async fn test_visible_terminal_bell_is_suppressed(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Claude", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        cx.update(|window, cx| {
            assert!(window.is_window_active());
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(!terminal.has_notification);
        });
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
    }

    #[gpui::test]
    async fn test_visible_terminal_bell_is_suppressed_without_focus(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Claude", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        let workspace = cx.update(|window, cx| {
            window
                .root::<MultiWorkspace>()
                .flatten()
                .expect("test window should have a MultiWorkspace root")
                .read(cx)
                .workspace()
                .clone()
        });
        workspace.update_in(&mut cx, |workspace, window, cx| {
            workspace.focus_handle(cx).focus(window, cx);
        });
        cx.update(|window, cx| {
            assert!(window.is_window_active());
            assert!(workspace.read(cx).focus_handle(cx).is_focused(window));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(!terminal.has_notification);
        });
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
    }

    #[gpui::test]
    async fn test_terminal_bell_notifies_when_configuration_overlay_covers_terminal(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Claude", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.set_overlay(OverlayView::Configuration, true, window, cx);
        });
        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(terminal.has_notification);
        });
        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("covered terminal bell should show a notification");
    }

    #[gpui::test]
    async fn test_thread_notification_shows_when_configuration_overlay_covers_thread(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let connection = StubAgentConnection::new();
        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Default response".into()),
        )]);
        open_thread_with_connection(&panel, connection, &mut cx);

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.set_overlay(OverlayView::Configuration, true, window, cx);
        });
        send_message(&panel, &mut cx);

        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("covered thread should show a notification");
    }

    #[gpui::test]
    async fn test_terminal_bell_marks_without_popup_when_sidebar_open(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let first_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Build", true, window, cx)
            })
            .expect("first test terminal should be inserted");
        let second_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Server", true, window, cx)
            })
            .expect("second test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.active_terminal_id(), Some(second_terminal_id));
        });
        cx.update(|window, cx| {
            let multi_workspace = window
                .root::<MultiWorkspace>()
                .flatten()
                .expect("test window should have a MultiWorkspace root");
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.open_sidebar(cx);
            });
        });
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(first_terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let first_terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == first_terminal_id)
                .expect("first terminal should remain in the panel");
            assert!(first_terminal.has_notification);
        });
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
    }

    #[gpui::test]
    async fn test_terminal_bell_notifies_when_sidebar_history_open(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_visible_panel_with_sidebar(cx, false).await;
        let first_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Build", true, window, cx)
            })
            .expect("first test terminal should be inserted");
        let second_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Server", true, window, cx)
            })
            .expect("second test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.active_terminal_id(), Some(second_terminal_id));
        });
        cx.update(|window, cx| {
            let multi_workspace = window
                .root::<MultiWorkspace>()
                .flatten()
                .expect("test window should have a MultiWorkspace root");
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.open_sidebar(cx);
            });
        });
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(first_terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let first_terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == first_terminal_id)
                .expect("first terminal should remain in the panel");
            assert!(first_terminal.has_notification);
        });
        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("terminal bell should notify when the sidebar thread list is hidden");
    }

    #[gpui::test]
    async fn test_terminal_notification_dismissed_when_sidebar_opens(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let first_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Build", true, window, cx)
            })
            .expect("first test terminal should be inserted");
        let second_terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Server", true, window, cx)
            })
            .expect("second test terminal should be inserted");
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.active_terminal_id(), Some(second_terminal_id));
        });
        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(first_terminal_id, cx);
        });
        cx.run_until_parked();

        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("inactive terminal bell should show a notification");

        cx.update(|window, cx| {
            let multi_workspace = window
                .root::<MultiWorkspace>()
                .flatten()
                .expect("test window should have a MultiWorkspace root");
            multi_workspace.update(cx, |multi_workspace, cx| {
                multi_workspace.open_sidebar(cx);
            });
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let first_terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == first_terminal_id)
                .expect("first terminal should remain in the panel");
            assert!(first_terminal.has_notification);
        });
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
    }

    #[gpui::test]
    async fn test_focused_terminal_bell_notifies_when_window_inactive(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Claude", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        cx.update(|window, cx| {
            assert!(window.is_window_active());
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });
        cx.deactivate_window();
        cx.update(|window, _cx| {
            assert!(!window.is_window_active());
        });

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(terminal.has_notification);
        });
        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("background terminal bell should show a notification");
    }

    #[gpui::test]
    async fn test_active_terminal_notification_clears_when_window_reactivates(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_visible_panel(cx).await;
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Claude", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        cx.deactivate_window();
        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(terminal.has_notification);
        });
        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("background terminal bell should show a notification");

        cx.update(|window, _cx| {
            window.activate_window();
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(!terminal.has_notification);
        });
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
    }

    #[gpui::test]
    async fn test_terminal_notification_dismissed_when_active_terminal_becomes_visible(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;
        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::PrimaryScreen,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Claude", true, window, cx)
            })
            .expect("test terminal should be inserted");
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(terminal_id, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(terminal.has_notification);
        });
        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("hidden terminal bell should show a notification");

        let workspace = cx.update(|window, cx| {
            window
                .root::<MultiWorkspace>()
                .flatten()
                .expect("test window should have a MultiWorkspace root")
                .read(cx)
                .workspace()
                .clone()
        });
        workspace.update_in(&mut cx, |workspace, window, cx| {
            workspace.add_panel(panel.clone(), window, cx);
            workspace.focus_panel::<AgentPanel>(window, cx);
        });
        cx.run_until_parked();

        panel.read_with(&cx, |panel, cx| {
            let terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == terminal_id)
                .expect("terminal should remain in the panel");
            assert!(!terminal.has_notification);
        });
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
    }

    #[gpui::test]
    async fn test_terminal_notification_closed_when_panel_dropped(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        cx.update(|_window, cx| {
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::PrimaryScreen,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });
        let terminal_id = panel
            .update_in(&mut cx, |panel, window, cx| {
                panel.insert_test_terminal("Claude", true, window, cx)
            })
            .expect("test terminal should be inserted");
        let weak_panel = panel.downgrade();
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            panel.emit_test_terminal_bell(terminal_id, cx);
        });
        cx.run_until_parked();

        cx.windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("hidden terminal bell should show a notification");

        drop(panel);
        cx.update(|_window, _cx| {});
        cx.run_until_parked();

        assert!(
            !weak_panel.is_upgradable(),
            "agent panel should be released after dropping the last handle"
        );
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
    }

    #[gpui::test]
    async fn test_terminal_notification_view_activates_terminal_workspace(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            cx.update_flags(true, vec!["agent-panel-terminal".to_string()]);
            AgentSettings::override_global(
                AgentSettings {
                    notify_when_agent_waiting: NotifyWhenAgentWaiting::PrimaryScreen,
                    ..AgentSettings::get_global(cx).clone()
                },
                cx,
            );
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        fs.insert_tree("/project_b", json!({ "file.txt": "" }))
            .await;
        let project_a = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;
        let project_b = Project::test(fs, [Path::new("/project_b")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));
        let workspace_a = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        let first_terminal_id = panel_a
            .update_in(cx, |panel, window, cx| {
                panel.insert_test_terminal("Build", true, window, cx)
            })
            .expect("first test terminal should be inserted");
        let second_terminal_id = panel_a
            .update_in(cx, |panel, window, cx| {
                panel.insert_test_terminal("Server", true, window, cx)
            })
            .expect("second test terminal should be inserted");
        cx.run_until_parked();

        multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                assert_eq!(multi_workspace.workspace(), &workspace_b);
            })
            .unwrap();
        panel_a.read_with(cx, |panel, _cx| {
            assert_eq!(panel.active_terminal_id(), Some(second_terminal_id));
        });

        panel_a.update(cx, |panel, cx| {
            panel.emit_test_terminal_bell(first_terminal_id, cx);
        });
        cx.run_until_parked();

        let notification = cx
            .windows()
            .iter()
            .find_map(|window| window.downcast::<AgentNotification>())
            .expect("terminal bell should show a notification");
        notification
            .update(cx, |notification, _window, cx| notification.accept(cx))
            .unwrap();
        assert!(
            cx.windows()
                .iter()
                .all(|window| window.downcast::<AgentNotification>().is_none())
        );
        cx.run_until_parked();

        multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                assert_eq!(multi_workspace.workspace(), &workspace_a);
            })
            .unwrap();
        panel_a.read_with(cx, |panel, cx| {
            assert_eq!(panel.active_terminal_id(), Some(first_terminal_id));
            let first_terminal = panel
                .terminals(cx)
                .into_iter()
                .find(|terminal| terminal.id == first_terminal_id)
                .expect("first terminal should remain in the panel");
            assert!(!first_terminal.has_notification);
        });
    }

    #[gpui::test]
    async fn test_running_thread_retained_when_navigating_away(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        let connection_a = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_a.clone(), &mut cx);
        send_message(&panel, &mut cx);

        let session_id_a = active_session_id(&panel, &cx);
        let thread_id_a = active_thread_id(&panel, &cx);

        // Send a chunk to keep thread A generating (don't end the turn).
        cx.update(|_, cx| {
            connection_a.send_update(
                session_id_a.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("chunk".into())),
                cx,
            );
        });
        cx.run_until_parked();

        // Verify thread A is generating.
        panel.read_with(&cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            assert_eq!(thread.read(cx).status(), ThreadStatus::Generating);
            assert!(panel.retained_threads.is_empty());
        });

        // Open a new thread B — thread A should be retained in background.
        let connection_b = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_b, &mut cx);

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.retained_threads.len(),
                1,
                "Running thread A should be retained in retained_threads"
            );
            assert!(
                panel.retained_threads.contains_key(&thread_id_a),
                "Retained thread should be keyed by thread A's thread ID"
            );
        });
    }

    #[gpui::test]
    async fn test_idle_non_loadable_thread_retained_when_navigating_away(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        let connection_a = StubAgentConnection::new();
        connection_a.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("Response".into()),
        )]);
        open_thread_with_connection(&panel, connection_a, &mut cx);
        send_message(&panel, &mut cx);

        let weak_view_a = panel.read_with(&cx, |panel, _cx| {
            panel.active_conversation_view().unwrap().downgrade()
        });
        let thread_id_a = active_thread_id(&panel, &cx);

        // Thread A should be idle (auto-completed via set_next_prompt_updates).
        panel.read_with(&cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            assert_eq!(thread.read(cx).status(), ThreadStatus::Idle);
        });

        // Open a new thread B — thread A should be retained because it is not loadable.
        let connection_b = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_b, &mut cx);

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.retained_threads.len(),
                1,
                "Idle non-loadable thread A should be retained in retained_threads"
            );
            assert!(
                panel.retained_threads.contains_key(&thread_id_a),
                "Retained thread should be keyed by thread A's thread ID"
            );
        });

        assert!(
            weak_view_a.upgrade().is_some(),
            "Idle non-loadable ConnectionView should still be retained"
        );
    }

    #[gpui::test]
    async fn test_background_thread_promoted_via_load(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        let connection_a = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_a.clone(), &mut cx);
        send_message(&panel, &mut cx);

        let session_id_a = active_session_id(&panel, &cx);
        let thread_id_a = active_thread_id(&panel, &cx);

        // Keep thread A generating.
        cx.update(|_, cx| {
            connection_a.send_update(
                session_id_a.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("chunk".into())),
                cx,
            );
        });
        cx.run_until_parked();

        // Open thread B — thread A goes to background.
        let connection_b = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_b, &mut cx);
        send_message(&panel, &mut cx);

        let thread_id_b = active_thread_id(&panel, &cx);

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.retained_threads.len(), 1);
            assert!(panel.retained_threads.contains_key(&thread_id_a));
        });

        // Load thread A back via load_agent_thread — should promote from background.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.load_agent_thread(
                panel.selected_agent(cx),
                thread_id_a,
                None,
                None,
                true,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });

        // Thread A should now be the active view, promoted from background.
        let active_session = active_session_id(&panel, &cx);
        assert_eq!(
            active_session, session_id_a,
            "Thread A should be the active thread after promotion"
        );

        panel.read_with(&cx, |panel, _cx| {
            assert!(
                !panel.retained_threads.contains_key(&thread_id_a),
                "Promoted thread A should no longer be in retained_threads"
            );
            assert!(
                panel.retained_threads.contains_key(&thread_id_b),
                "Thread B (idle, non-loadable) should remain retained in retained_threads"
            );
        });
    }

    #[gpui::test]
    async fn test_reopening_visible_thread_keeps_thread_usable(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            panel.connection_store.update(cx, |store, cx| {
                store.restart_connection(
                    Agent::NativeAgent,
                    Rc::new(StubAgentServer::new(SessionTrackingConnection::new())),
                    cx,
                );
            });
        });
        cx.run_until_parked();

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.external_thread(
                Some(Agent::NativeAgent),
                None,
                None,
                None,
                None,
                true,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        send_message(&panel, &mut cx);

        let session_id = active_session_id(&panel, &cx);

        panel.update_in(&mut cx, |panel, window, cx| {
            panel.open_thread(session_id.clone(), None, None, window, cx);
        });
        cx.run_until_parked();

        send_message(&panel, &mut cx);

        panel.read_with(&cx, |panel, cx| {
            let active_view = panel
                .active_conversation_view()
                .expect("visible conversation should remain open after reopening");
            let connected = active_view
                .read(cx)
                .as_connected()
                .expect("visible conversation should still be connected in the UI");
            assert!(
                !connected.has_thread_error(cx),
                "reopening an already-visible session should keep the thread usable"
            );
        });
    }

    #[gpui::test]
    async fn test_initial_content_for_thread_summary_uses_own_session_id(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let source_session_id = acp::SessionId::new("source-thread-session");
        let source_title: SharedString = "Source Thread Title".into();
        let db_thread = agent::DbThread {
            title: source_title.clone(),
            messages: Vec::new(),
            updated_at: Utc::now(),
            detailed_summary: None,
            initial_project_snapshot: None,
            cumulative_token_usage: Default::default(),
            request_token_usage: HashMap::default(),
            model: None,
            profile: None,
            imported: false,
            subagent_context: None,
            speed: None,
            thinking_enabled: false,
            thinking_effort: None,
            draft_prompt: None,
            ui_scroll_position: None,
            sandboxed_terminal_temp_dir: None,
        };

        let thread_store = cx.update(|cx| ThreadStore::global(cx));
        thread_store
            .update(cx, |store, cx| {
                store.save_thread(
                    source_session_id.clone(),
                    db_thread,
                    PathList::default(),
                    cx,
                )
            })
            .await
            .expect("saving source thread should succeed");
        cx.run_until_parked();

        thread_store.read_with(cx, |store, _cx| {
            let entry = store
                .thread_from_session_id(&source_session_id)
                .expect("saved thread should be listed in the store");
            assert!(
                entry.parent_session_id.is_none(),
                "saved thread is a root thread with no parent session"
            );
        });

        let content = cx
            .update(|cx| {
                AgentPanel::initial_content_for_thread_summary(source_session_id.clone(), cx)
            })
            .expect("initial content should be produced for a root thread");

        match content {
            AgentInitialContent::ThreadSummary { session_id, title } => {
                assert_eq!(
                    session_id, source_session_id,
                    "thread-summary mention should use the source thread's own session id"
                );
                assert_eq!(title, Some(source_title.clone()));
            }
            _ => panic!("expected AgentInitialContent::ThreadSummary"),
        }

        // Unknown session ids should still produce no content.
        let missing = cx.update(|cx| {
            AgentPanel::initial_content_for_thread_summary(
                acp::SessionId::new("does-not-exist"),
                cx,
            )
        });
        assert!(
            missing.is_none(),
            "unknown session ids should not produce initial content"
        );
    }

    #[gpui::test]
    async fn test_cleanup_retained_threads_keeps_five_most_recent_idle_loadable_threads(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;
        let connection = StubAgentConnection::new()
            .with_supports_load_session(true)
            .with_agent_id("loadable-stub".into())
            .with_telemetry_id("loadable-stub".into());
        let mut session_ids = Vec::new();
        let mut thread_ids = Vec::new();

        for _ in 0..7 {
            let (session_id, thread_id) =
                open_generating_thread_with_loadable_connection(&panel, &connection, &mut cx);
            session_ids.push(session_id);
            thread_ids.push(thread_id);
        }

        let base_time = Instant::now();

        for session_id in session_ids.iter().take(6) {
            connection.end_turn(session_id.clone(), acp::StopReason::EndTurn);
        }
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            for (index, thread_id) in thread_ids.iter().take(6).enumerate() {
                let conversation_view = panel
                    .retained_threads
                    .get(thread_id)
                    .expect("retained thread should exist")
                    .clone();
                conversation_view.update(cx, |view, cx| {
                    view.set_updated_at(base_time + Duration::from_secs(index as u64), cx);
                });
            }
            panel.cleanup_retained_threads(cx);
        });

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.retained_threads.len(),
                5,
                "cleanup should keep at most five idle loadable retained threads"
            );
            assert!(
                !panel.retained_threads.contains_key(&thread_ids[0]),
                "oldest idle loadable retained thread should be removed"
            );
            for thread_id in &thread_ids[1..6] {
                assert!(
                    panel.retained_threads.contains_key(thread_id),
                    "more recent idle loadable retained threads should be retained"
                );
            }
            assert!(
                !panel.retained_threads.contains_key(&thread_ids[6]),
                "the active thread should not also be stored as a retained thread"
            );
        });
    }

    #[gpui::test]
    async fn test_cleanup_retained_threads_preserves_idle_non_loadable_threads(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;

        let non_loadable_connection = StubAgentConnection::new();
        let (_non_loadable_session_id, non_loadable_thread_id) =
            open_idle_thread_with_non_loadable_connection(
                &panel,
                &non_loadable_connection,
                &mut cx,
            );

        let loadable_connection = StubAgentConnection::new()
            .with_supports_load_session(true)
            .with_agent_id("loadable-stub".into())
            .with_telemetry_id("loadable-stub".into());
        let mut loadable_session_ids = Vec::new();
        let mut loadable_thread_ids = Vec::new();

        for _ in 0..7 {
            let (session_id, thread_id) = open_generating_thread_with_loadable_connection(
                &panel,
                &loadable_connection,
                &mut cx,
            );
            loadable_session_ids.push(session_id);
            loadable_thread_ids.push(thread_id);
        }

        let base_time = Instant::now();

        for session_id in loadable_session_ids.iter().take(6) {
            loadable_connection.end_turn(session_id.clone(), acp::StopReason::EndTurn);
        }
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            for (index, thread_id) in loadable_thread_ids.iter().take(6).enumerate() {
                let conversation_view = panel
                    .retained_threads
                    .get(thread_id)
                    .expect("retained thread should exist")
                    .clone();
                conversation_view.update(cx, |view, cx| {
                    view.set_updated_at(base_time + Duration::from_secs(index as u64), cx);
                });
            }
            panel.cleanup_retained_threads(cx);
        });

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.retained_threads.len(),
                6,
                "cleanup should keep the non-loadable idle thread in addition to five loadable ones"
            );
            assert!(
                panel.retained_threads.contains_key(&non_loadable_thread_id),
                "idle non-loadable retained threads should not be cleanup candidates"
            );
            assert!(
                !panel.retained_threads.contains_key(&loadable_thread_ids[0]),
                "oldest idle loadable retained thread should still be removed"
            );
            for thread_id in &loadable_thread_ids[1..6] {
                assert!(
                    panel.retained_threads.contains_key(thread_id),
                    "more recent idle loadable retained threads should be retained"
                );
            }
            assert!(
                !panel.retained_threads.contains_key(&loadable_thread_ids[6]),
                "the active loadable thread should not also be stored as a retained thread"
            );
        });
    }

    #[test]
    fn test_deserialize_agent_variants() {
        // PascalCase (legacy AgentType format, persisted in panel state)
        assert_eq!(
            serde_json::from_str::<Agent>(r#""NativeAgent""#).unwrap(),
            Agent::NativeAgent,
        );
        assert_eq!(
            serde_json::from_str::<Agent>(r#"{"Custom":{"name":"my-agent"}}"#).unwrap(),
            Agent::Custom {
                id: "my-agent".into(),
            },
        );

        // Legacy TextThread variant deserializes to NativeAgent
        assert_eq!(
            serde_json::from_str::<Agent>(r#""TextThread""#).unwrap(),
            Agent::NativeAgent,
        );

        // snake_case (canonical format)
        assert_eq!(
            serde_json::from_str::<Agent>(r#""native_agent""#).unwrap(),
            Agent::NativeAgent,
        );
        assert_eq!(
            serde_json::from_str::<Agent>(r#"{"custom":{"name":"my-agent"}}"#).unwrap(),
            Agent::Custom {
                id: "my-agent".into(),
            },
        );

        // Serialization uses snake_case
        assert_eq!(
            serde_json::to_string(&Agent::NativeAgent).unwrap(),
            r#""native_agent""#,
        );
        assert_eq!(
            serde_json::to_string(&Agent::Custom {
                id: "my-agent".into()
            })
            .unwrap(),
            r#"{"custom":{"name":"my-agent"}}"#,
        );
    }

    #[gpui::test]
    fn test_resolve_worktree_branch_target() {
        let resolved = git_ui::worktree_service::resolve_worktree_branch_target(
            &NewWorktreeBranchTarget::ExistingBranch {
                name: "feature".to_string(),
            },
        );
        assert_eq!(resolved, Some("feature".to_string()));

        let resolved = git_ui::worktree_service::resolve_worktree_branch_target(
            &NewWorktreeBranchTarget::CurrentBranch,
        );
        assert_eq!(resolved, None);

        let resolved = git_ui::worktree_service::resolve_worktree_branch_target(
            &NewWorktreeBranchTarget::RemoteBranch {
                remote_name: "origin".to_string(),
                branch_name: "main".to_string(),
            },
        );
        assert_eq!(resolved, Some("refs/remotes/origin/main".to_string()));
    }

    #[gpui::test]
    async fn test_work_dirs_update_when_worktrees_change(cx: &mut TestAppContext) {
        use crate::thread_metadata_store::ThreadMetadataStore;

        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        // Set up a project with one worktree.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        let project = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        // Open thread A and send a message. With empty next_prompt_updates it
        // stays generating, so opening B will move A to retained_threads.
        let connection_a = StubAgentConnection::new().with_agent_id("agent-a".into());
        open_thread_with_custom_connection(&panel, connection_a.clone(), &mut cx);
        send_message(&panel, &mut cx);
        let session_id_a = active_session_id(&panel, &cx);
        let thread_id_a = active_thread_id(&panel, &cx);

        // Open thread C — thread A (generating) moves to background.
        // Thread C completes immediately (idle), then opening B moves C to background too.
        let connection_c = StubAgentConnection::new().with_agent_id("agent-c".into());
        connection_c.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("done".into()),
        )]);
        open_thread_with_custom_connection(&panel, connection_c.clone(), &mut cx);
        send_message(&panel, &mut cx);
        let thread_id_c = active_thread_id(&panel, &cx);

        // Open thread B — thread C (idle, non-loadable) is retained in background.
        let connection_b = StubAgentConnection::new().with_agent_id("agent-b".into());
        open_thread_with_custom_connection(&panel, connection_b.clone(), &mut cx);
        send_message(&panel, &mut cx);
        let session_id_b = active_session_id(&panel, &cx);
        let _thread_id_b = active_thread_id(&panel, &cx);

        let metadata_store = cx.update(|_, cx| ThreadMetadataStore::global(cx));

        panel.read_with(&cx, |panel, _cx| {
            assert!(
                panel.retained_threads.contains_key(&thread_id_a),
                "Thread A should be in retained_threads"
            );
            assert!(
                panel.retained_threads.contains_key(&thread_id_c),
                "Thread C should be in retained_threads"
            );
        });

        // Verify initial work_dirs for thread B contain only /project_a.
        let initial_b_paths = panel.read_with(&cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            thread.read(cx).work_dirs().cloned().unwrap()
        });
        assert_eq!(
            initial_b_paths.ordered_paths().collect::<Vec<_>>(),
            vec![&PathBuf::from("/project_a")],
            "Thread B should initially have only /project_a"
        );

        // Now add a second worktree to the project.
        fs.insert_tree("/project_b", json!({ "other.txt": "" }))
            .await;
        let (new_tree, _) = project
            .update(&mut cx, |project, cx| {
                project.find_or_create_worktree("/project_b", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| new_tree.read(cx).as_local().unwrap().scan_complete())
            .await;
        cx.run_until_parked();

        // Verify thread B's (active) work_dirs now include both worktrees.
        let updated_b_paths = panel.read_with(&cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            thread.read(cx).work_dirs().cloned().unwrap()
        });
        let mut b_paths_sorted = updated_b_paths.ordered_paths().cloned().collect::<Vec<_>>();
        b_paths_sorted.sort();
        assert_eq!(
            b_paths_sorted,
            vec![PathBuf::from("/project_a"), PathBuf::from("/project_b")],
            "Thread B work_dirs should include both worktrees after adding /project_b"
        );

        // Verify thread A's (background) work_dirs are also updated.
        let updated_a_paths = panel.read_with(&cx, |panel, cx| {
            let bg_view = panel.retained_threads.get(&thread_id_a).unwrap();
            let root_thread = bg_view.read(cx).root_thread_view().unwrap();
            root_thread
                .read(cx)
                .thread
                .read(cx)
                .work_dirs()
                .cloned()
                .unwrap()
        });
        let mut a_paths_sorted = updated_a_paths.ordered_paths().cloned().collect::<Vec<_>>();
        a_paths_sorted.sort();
        assert_eq!(
            a_paths_sorted,
            vec![PathBuf::from("/project_a"), PathBuf::from("/project_b")],
            "Thread A work_dirs should include both worktrees after adding /project_b"
        );

        // Verify thread idle C was also updated.
        let updated_c_paths = panel.read_with(&cx, |panel, cx| {
            let bg_view = panel.retained_threads.get(&thread_id_c).unwrap();
            let root_thread = bg_view.read(cx).root_thread_view().unwrap();
            root_thread
                .read(cx)
                .thread
                .read(cx)
                .work_dirs()
                .cloned()
                .unwrap()
        });
        let mut c_paths_sorted = updated_c_paths.ordered_paths().cloned().collect::<Vec<_>>();
        c_paths_sorted.sort();
        assert_eq!(
            c_paths_sorted,
            vec![PathBuf::from("/project_a"), PathBuf::from("/project_b")],
            "Thread C (idle background) work_dirs should include both worktrees after adding /project_b"
        );

        // Verify the metadata store reflects the new paths for running threads only.
        cx.run_until_parked();
        for (label, session_id) in [("thread B", &session_id_b), ("thread A", &session_id_a)] {
            let metadata_paths = metadata_store.read_with(&cx, |store, _cx| {
                let metadata = store
                    .entry_by_session(session_id)
                    .unwrap_or_else(|| panic!("{label} thread metadata should exist"));
                metadata.folder_paths().clone()
            });
            let mut sorted = metadata_paths.ordered_paths().cloned().collect::<Vec<_>>();
            sorted.sort();
            assert_eq!(
                sorted,
                vec![PathBuf::from("/project_a"), PathBuf::from("/project_b")],
                "{label} thread metadata folder_paths should include both worktrees"
            );
        }

        // Now remove a worktree and verify work_dirs shrink.
        let worktree_b_id = new_tree.read_with(&cx, |tree, _| tree.id());
        project.update(&mut cx, |project, cx| {
            project.remove_worktree(worktree_b_id, cx);
        });
        cx.run_until_parked();

        let after_remove_b = panel.read_with(&cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            thread.read(cx).work_dirs().cloned().unwrap()
        });
        assert_eq!(
            after_remove_b.ordered_paths().collect::<Vec<_>>(),
            vec![&PathBuf::from("/project_a")],
            "Thread B work_dirs should revert to only /project_a after removing /project_b"
        );

        let after_remove_a = panel.read_with(&cx, |panel, cx| {
            let bg_view = panel.retained_threads.get(&thread_id_a).unwrap();
            let root_thread = bg_view.read(cx).root_thread_view().unwrap();
            root_thread
                .read(cx)
                .thread
                .read(cx)
                .work_dirs()
                .cloned()
                .unwrap()
        });
        assert_eq!(
            after_remove_a.ordered_paths().collect::<Vec<_>>(),
            vec![&PathBuf::from("/project_a")],
            "Thread A work_dirs should revert to only /project_a after removing /project_b"
        );
    }

    #[gpui::test]
    async fn test_new_workspace_inherits_global_last_used_agent(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            // Use an isolated DB so parallel tests can't overwrite our global key.
            cx.set_global(db::AppDatabase::test_new());
        });

        let custom_agent = Agent::Custom {
            id: "my-preferred-agent".into(),
        };

        // Write a known agent to the global KVP to simulate a user who has
        // previously used this agent in another workspace.
        let kvp = cx.update(|cx| KeyValueStore::global(cx));
        write_global_last_used_agent(kvp, custom_agent.clone()).await;

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs.clone(), [], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        // Load the panel via `load()`, which reads the global fallback
        // asynchronously when no per-workspace state exists.
        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let panel = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        cx.run_until_parked();

        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent, custom_agent,
                "new workspace should inherit the global last-used agent"
            );
        });
    }

    #[gpui::test]
    async fn test_workspaces_maintain_independent_agent_selection(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project_a = Project::test(fs.clone(), [], cx).await;
        let project_b = Project::test(fs, [], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        let workspace_a = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        workspace_a.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });
        workspace_b.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let agent_a = Agent::Custom {
            id: "agent-alpha".into(),
        };
        let agent_b = Agent::Custom {
            id: "agent-beta".into(),
        };

        // Set up workspace A with agent_a
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });
        panel_a.update(cx, |panel, _cx| {
            panel.selected_agent = agent_a.clone();
        });

        // Set up workspace B with agent_b
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });
        panel_b.update(cx, |panel, _cx| {
            panel.selected_agent = agent_b.clone();
        });

        // Serialize both panels
        panel_a.update(cx, |panel, cx| panel.serialize(cx));
        panel_b.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        // Load fresh panels from serialized state and verify independence
        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_a = AgentPanel::load(workspace_a.downgrade(), async_cx)
            .await
            .expect("panel A load should succeed");
        cx.run_until_parked();

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_b = AgentPanel::load(workspace_b.downgrade(), async_cx)
            .await
            .expect("panel B load should succeed");
        cx.run_until_parked();

        loaded_a.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent, agent_a,
                "workspace A should restore agent-alpha, not agent-beta"
            );
        });

        loaded_b.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent, agent_b,
                "workspace B should restore agent-beta, not agent-alpha"
            );
        });
    }

    #[gpui::test]
    async fn test_new_thread_uses_workspace_selected_agent(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let custom_agent = Agent::Custom {
            id: "my-custom-agent".into(),
        };

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Set selected_agent to a custom agent
        panel.update(cx, |panel, _cx| {
            panel.selected_agent = custom_agent.clone();
        });

        // Call new_thread, which internally calls external_thread(None, ...)
        // This resolves the agent from self.selected_agent
        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });

        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent, custom_agent,
                "selected_agent should remain the custom agent after new_thread"
            );
            assert!(
                panel.active_conversation_view().is_some(),
                "a thread should have been created"
            );
        });
    }

    #[gpui::test]
    async fn test_draft_replaced_when_selected_agent_changes(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Create a draft with the default NativeAgent.
        panel.update_in(cx, |panel, window, cx| {
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });

        let first_draft_id = panel.read_with(cx, |panel, cx| {
            assert!(panel.draft_thread.is_some());
            assert_eq!(panel.selected_agent, Agent::NativeAgent);
            let draft = panel.draft_thread.as_ref().unwrap();
            assert_eq!(*draft.read(cx).agent_key(), Agent::NativeAgent);
            draft.entity_id()
        });

        // Switch selected_agent to a custom agent, then activate_draft again.
        // The stale NativeAgent draft should be replaced.
        let custom_agent = Agent::Custom {
            id: "my-custom-agent".into(),
        };
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = custom_agent.clone();
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });

        panel.read_with(cx, |panel, cx| {
            let draft = panel.draft_thread.as_ref().expect("draft should exist");
            assert_ne!(
                draft.entity_id(),
                first_draft_id,
                "a new draft should have been created"
            );
            assert_eq!(
                *draft.read(cx).agent_key(),
                custom_agent,
                "the new draft should use the custom agent"
            );
        });

        // Calling activate_draft again with the same agent should return the
        // cached draft (no replacement).
        let second_draft_id = panel.read_with(cx, |panel, _cx| {
            panel.draft_thread.as_ref().unwrap().entity_id()
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });

        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.draft_thread.as_ref().unwrap().entity_id(),
                second_draft_id,
                "draft should be reused when the agent has not changed"
            );
        });
    }

    #[gpui::test]
    async fn test_activate_draft_preserves_typed_content(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Create a draft using the Stub agent, which connects synchronously.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        let initial_draft_id = panel.read_with(cx, |panel, _cx| {
            panel.draft_thread.as_ref().unwrap().entity_id()
        });
        let initial_thread_id =
            panel.read_with(cx, |panel, cx| panel.active_thread_id(cx).unwrap());

        // Type some text into the draft editor.
        let thread_view = panel.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
        let message_editor = thread_view.read_with(cx, |view, _cx| view.message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Don't lose me!", window, cx);
        });

        // Press cmd-n on a typed draft — the draft is parked into
        // `retained_threads` so the user can return to it from the
        // sidebar, and a fresh, *empty* ephemeral draft becomes active.
        // The parked draft retains the prompt; the new one is a blank
        // slate.
        cx.dispatch_action(NewThread);
        cx.run_until_parked();

        panel.read_with(cx, |panel, _cx| {
            assert!(
                panel.retained_threads.contains_key(&initial_thread_id),
                "typed draft should have been parked into retained_threads"
            );
            let active_draft_id = panel.draft_thread.as_ref().unwrap().entity_id();
            assert_ne!(
                active_draft_id, initial_draft_id,
                "cmd-n should produce a fresh ephemeral draft"
            );
        });

        // The parked draft still holds the typed prompt.
        let parked_text = panel.read_with(cx, |panel, cx| panel.editor_text(initial_thread_id, cx));
        assert_eq!(
            parked_text.as_deref(),
            Some("Don't lose me!"),
            "parked draft should retain the typed prompt"
        );

        // The new active draft starts empty — no carry-over.
        let active_thread_id = panel.read_with(cx, |panel, cx| panel.active_thread_id(cx).unwrap());
        let active_text = panel.read_with(cx, |panel, cx| panel.editor_text(active_thread_id, cx));
        assert_eq!(
            active_text, None,
            "fresh ephemeral draft should start empty, not carry the parked draft's prompt"
        );
    }

    /// When the user is viewing a *parked* draft (selected from the
    /// sidebar) and presses `+`, the panel should just focus the
    /// ephemeral new-draft slot — not park it and create yet another
    /// empty draft. `+` is "go to my new-thread slot", not "reset state".
    #[gpui::test]
    async fn test_plus_with_parked_draft_active_focuses_ephemeral(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        workspace.update(cx, |workspace, _cx| workspace.set_random_database_id());
        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Open an initial draft, type into it, then press `+` to park it
        // and create a fresh ephemeral. The fresh ephemeral is what we'll
        // expect to refocus later.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        let parked_thread_id = crate::test_support::active_thread_id(&panel, cx);
        crate::test_support::type_draft_prompt(&panel, "parked draft prompt", cx);
        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });
        cx.run_until_parked();

        let ephemeral_thread_id = crate::test_support::active_thread_id(&panel, cx);
        let ephemeral_entity_id = panel.read_with(cx, |panel, _cx| {
            panel.draft_thread.as_ref().unwrap().entity_id()
        });
        assert_ne!(
            ephemeral_thread_id, parked_thread_id,
            "sanity: parking should have produced a fresh ephemeral draft"
        );

        // Activate the parked draft (simulates clicking it in the sidebar).
        panel.update_in(cx, |panel, window, cx| {
            panel.load_agent_thread(
                Agent::Stub,
                parked_thread_id,
                None,
                None,
                true,
                AgentThreadSource::Sidebar,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            crate::test_support::active_thread_id(&panel, cx),
            parked_thread_id,
            "sanity: parked draft should be the active view after load_agent_thread"
        );
        // The parked draft has content, so it was NOT reclaimed as
        // ephemeral. The previous ephemeral draft should still be in
        // the draft_thread slot.
        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.draft_thread.as_ref().unwrap().entity_id(),
                ephemeral_entity_id,
                "ephemeral draft slot should still hold the fresh draft"
            );
        });

        // Now press `+`. The ephemeral draft should become the active
        // view since it matches the selected agent.
        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            assert_eq!(
                panel.active_thread_id(cx),
                Some(ephemeral_thread_id),
                "`+` should have switched back to the existing ephemeral draft"
            );
            assert_eq!(
                panel.draft_thread.as_ref().unwrap().entity_id(),
                ephemeral_entity_id,
                "`+` should not have replaced the ephemeral draft"
            );
            assert!(
                panel.retained_threads.contains_key(&parked_thread_id),
                "parked draft should remain in `retained_threads`"
            );
        });
    }

    /// When viewing a parked draft (agent A) and selecting a different
    /// agent (B) from the dropdown menu, the panel should create a fresh
    /// draft for agent B — not reuse the existing ephemeral draft that
    /// was bound to agent A.
    #[gpui::test]
    async fn test_new_external_agent_replaces_mismatched_ephemeral_draft(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        workspace.update(cx, |workspace, _cx| workspace.set_random_database_id());
        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Create a draft with Stub agent, type into it, then press `+`
        // to park it — this also creates a fresh ephemeral draft (Stub).
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();
        let parked_thread_id = crate::test_support::active_thread_id(&panel, cx);
        crate::test_support::type_draft_prompt(&panel, "parked prompt", cx);
        panel.update_in(cx, |panel, window, cx| {
            panel.new_thread(&NewThread, window, cx);
        });
        cx.run_until_parked();

        let ephemeral_thread_id = crate::test_support::active_thread_id(&panel, cx);
        assert_ne!(ephemeral_thread_id, parked_thread_id);
        panel.read_with(cx, |panel, cx| {
            assert_eq!(
                panel.draft_thread.as_ref().unwrap().read(cx).agent_key(),
                &Agent::Stub,
                "ephemeral draft should be Stub agent"
            );
        });

        // Navigate back to the parked draft (simulates sidebar click).
        panel.update_in(cx, |panel, window, cx| {
            panel.load_agent_thread(
                Agent::Stub,
                parked_thread_id,
                None,
                None,
                true,
                AgentThreadSource::Sidebar,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            crate::test_support::active_thread_id(&panel, cx),
            parked_thread_id,
        );

        // Now switch to NativeAgent (simulates selecting a different
        // agent from the toolbar dropdown). This should NOT reuse the
        // Stub ephemeral draft — it should replace it with one bound to
        // NativeAgent.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::NativeAgent;
            panel.activate_new_thread(true, AgentThreadSource::AgentPanel, window, cx);
        });
        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            let draft = panel.draft_thread.as_ref().expect("draft should exist");
            assert_eq!(
                draft.read(cx).agent_key(),
                &Agent::NativeAgent,
                "ephemeral draft should be bound to NativeAgent, not Stub"
            );
            let active_id = panel.active_thread_id(cx).unwrap();
            assert_ne!(
                active_id, ephemeral_thread_id,
                "old Stub ephemeral draft should have been replaced"
            );
            assert!(
                panel.retained_threads.contains_key(&parked_thread_id),
                "parked draft should still be in retained_threads"
            );
        });
    }

    #[gpui::test]
    async fn test_typed_draft_is_parked_when_switching_agents(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        workspace.update(cx, |workspace, _cx| {
            workspace.set_random_database_id();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Create a draft with a custom stub server that connects synchronously.
        panel.update_in(cx, |panel, window, cx| {
            panel.open_draft_with_server(
                Rc::new(StubAgentServer::new(StubAgentConnection::new())),
                window,
                cx,
            );
        });
        cx.run_until_parked();

        let initial_draft_id = panel.read_with(cx, |panel, _cx| {
            panel.draft_thread.as_ref().unwrap().entity_id()
        });
        let initial_thread_id =
            panel.read_with(cx, |panel, cx| panel.active_thread_id(cx).unwrap());

        // Type text into the first draft's editor.
        let thread_view = panel.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
        let message_editor = thread_view.read_with(cx, |view, _cx| view.message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("saved prompt", window, cx);
        });

        // Switch to a different agent. The typed draft should be parked
        // into `retained_threads` (keeping the user's prompt accessible
        // from the sidebar) and a fresh empty draft on the new agent
        // should become active.
        cx.dispatch_action(NewExternalAgentThread {
            agent: Agent::Stub.id(),
        });
        cx.run_until_parked();

        // A new draft should have been created for the Stub agent.
        panel.read_with(cx, |panel, cx| {
            let draft = panel.draft_thread.as_ref().expect("draft should exist");
            assert_ne!(
                draft.entity_id(),
                initial_draft_id,
                "a new draft should have been created for the new agent"
            );
            assert_eq!(
                *draft.read(cx).agent_key(),
                Agent::Stub,
                "new draft should use the new agent"
            );
            assert!(
                panel.retained_threads.contains_key(&initial_thread_id),
                "typed draft should have been parked into retained_threads"
            );
        });

        // The parked draft retains the prompt.
        let parked_text = panel.read_with(cx, |panel, cx| panel.editor_text(initial_thread_id, cx));
        assert_eq!(
            parked_text.as_deref(),
            Some("saved prompt"),
            "parked draft should retain the user's prompt"
        );

        // The new draft on the new agent starts empty.
        let active_thread_id = panel.read_with(cx, |panel, cx| panel.active_thread_id(cx).unwrap());
        let active_text = panel.read_with(cx, |panel, cx| panel.editor_text(active_thread_id, cx));
        assert_eq!(
            active_text, None,
            "new draft on the new agent should start empty, not carry the parked draft's prompt"
        );
    }

    #[gpui::test]
    async fn test_rollback_all_succeed_returns_ok(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        cx.executor().run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project.repositories(cx).values().next().unwrap().clone()
        });

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let path_a = PathBuf::from("/worktrees/branch/project_a");
        let path_b = PathBuf::from("/worktrees/branch/project_b");

        let (sender_a, receiver_a) = futures::channel::oneshot::channel::<Result<()>>();
        let (sender_b, receiver_b) = futures::channel::oneshot::channel::<Result<()>>();
        sender_a.send(Ok(())).unwrap();
        sender_b.send(Ok(())).unwrap();

        let creation_infos = vec![
            (repository.clone(), path_a.clone(), receiver_a),
            (repository.clone(), path_b.clone(), receiver_b),
        ];

        let fs_clone = fs.clone();
        let result = multi_workspace
            .update(cx, |_, window, cx| {
                window.spawn(cx, async move |cx| {
                    git_ui::worktree_service::await_and_rollback_on_failure(
                        creation_infos,
                        fs_clone,
                        cx,
                    )
                    .await
                })
            })
            .unwrap()
            .await;

        let paths = result.expect("all succeed should return Ok");
        assert_eq!(paths, vec![path_a, path_b]);
    }

    #[gpui::test]
    async fn test_rollback_on_failure_attempts_all_worktrees(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        cx.executor().run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project.repositories(cx).values().next().unwrap().clone()
        });

        // Actually create a worktree so it exists in FakeFs for rollback to find.
        let success_path = PathBuf::from("/worktrees/branch/project");
        cx.update(|cx| {
            repository.update(cx, |repo, _| {
                repo.create_worktree(
                    git::repository::CreateWorktreeTarget::NewBranch {
                        branch_name: "branch".to_string(),
                        base_sha: None,
                    },
                    success_path.clone(),
                )
            })
        })
        .await
        .unwrap()
        .unwrap();
        cx.executor().run_until_parked();

        // Verify the worktree directory exists before rollback.
        assert!(
            fs.is_dir(&success_path).await,
            "worktree directory should exist before rollback"
        );

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        // Build creation_infos: one success, one failure.
        let failed_path = PathBuf::from("/worktrees/branch/failed_project");

        let (sender_ok, receiver_ok) = futures::channel::oneshot::channel::<Result<()>>();
        let (sender_err, receiver_err) = futures::channel::oneshot::channel::<Result<()>>();
        sender_ok.send(Ok(())).unwrap();
        sender_err
            .send(Err(anyhow!("branch already exists")))
            .unwrap();

        let creation_infos = vec![
            (repository.clone(), success_path.clone(), receiver_ok),
            (repository.clone(), failed_path.clone(), receiver_err),
        ];

        let fs_clone = fs.clone();
        let result = multi_workspace
            .update(cx, |_, window, cx| {
                window.spawn(cx, async move |cx| {
                    git_ui::worktree_service::await_and_rollback_on_failure(
                        creation_infos,
                        fs_clone,
                        cx,
                    )
                    .await
                })
            })
            .unwrap()
            .await;

        assert!(
            result.is_err(),
            "should return error when any creation fails"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("branch already exists"),
            "error should mention the original failure: {err_msg}"
        );

        // The successful worktree should have been rolled back by git.
        cx.executor().run_until_parked();
        assert!(
            !fs.is_dir(&success_path).await,
            "successful worktree directory should be removed by rollback"
        );
    }

    #[gpui::test]
    async fn test_rollback_on_canceled_receiver(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        cx.executor().run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project.repositories(cx).values().next().unwrap().clone()
        });

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let path = PathBuf::from("/worktrees/branch/project");

        // Drop the sender to simulate a canceled receiver.
        let (_sender, receiver) = futures::channel::oneshot::channel::<Result<()>>();
        drop(_sender);

        let creation_infos = vec![(repository.clone(), path.clone(), receiver)];

        let fs_clone = fs.clone();
        let result = multi_workspace
            .update(cx, |_, window, cx| {
                window.spawn(cx, async move |cx| {
                    git_ui::worktree_service::await_and_rollback_on_failure(
                        creation_infos,
                        fs_clone,
                        cx,
                    )
                    .await
                })
            })
            .unwrap()
            .await;

        assert!(
            result.is_err(),
            "should return error when receiver is canceled"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("canceled"),
            "error should mention cancellation: {err_msg}"
        );
    }

    #[gpui::test]
    async fn test_rollback_cleans_up_orphan_directories(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
        cx.executor().run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project.repositories(cx).values().next().unwrap().clone()
        });

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        // Simulate the orphan state: create_dir_all was called but git
        // worktree add failed, leaving a directory with leftover files.
        let orphan_path = PathBuf::from("/worktrees/branch/orphan_project");
        fs.insert_tree(
            "/worktrees/branch/orphan_project",
            json!({ "leftover.txt": "junk" }),
        )
        .await;

        assert!(
            fs.is_dir(&orphan_path).await,
            "orphan dir should exist before rollback"
        );

        let (sender, receiver) = futures::channel::oneshot::channel::<Result<()>>();
        sender.send(Err(anyhow!("hook failed"))).unwrap();

        let creation_infos = vec![(repository.clone(), orphan_path.clone(), receiver)];

        let fs_clone = fs.clone();
        let result = multi_workspace
            .update(cx, |_, window, cx| {
                window.spawn(cx, async move |cx| {
                    git_ui::worktree_service::await_and_rollback_on_failure(
                        creation_infos,
                        fs_clone,
                        cx,
                    )
                    .await
                })
            })
            .unwrap()
            .await;

        cx.executor().run_until_parked();

        assert!(result.is_err());
        assert!(
            !fs.is_dir(&orphan_path).await,
            "orphan worktree directory should be removed by filesystem cleanup"
        );
    }

    #[gpui::test]
    async fn test_selected_agent_syncs_when_navigating_between_threads(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        let stub_agent = Agent::Custom { id: "Test".into() };

        // Open thread A and send a message so it is retained.
        let connection_a = StubAgentConnection::new();
        connection_a.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("response a".into()),
        )]);
        open_thread_with_connection(&panel, connection_a, &mut cx);
        let _session_id_a = active_session_id(&panel, &cx);
        let thread_id_a = active_thread_id(&panel, &cx);
        send_message(&panel, &mut cx);
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.selected_agent, stub_agent);
        });

        // Open thread B with a different agent — thread A goes to retained.
        let custom_agent = Agent::Custom {
            id: "my-custom-agent".into(),
        };
        let connection_b = StubAgentConnection::new()
            .with_agent_id("my-custom-agent".into())
            .with_telemetry_id("my-custom-agent".into());
        connection_b.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("response b".into()),
        )]);
        open_thread_with_custom_connection(&panel, connection_b, &mut cx);
        send_message(&panel, &mut cx);
        cx.run_until_parked();

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent, custom_agent,
                "selected_agent should have changed to the custom agent"
            );
        });

        // Navigate back to thread A via load_agent_thread.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.load_agent_thread(
                stub_agent.clone(),
                thread_id_a,
                None,
                None,
                true,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent, stub_agent,
                "selected_agent should sync back to thread A's agent"
            );
        });
    }

    #[gpui::test]
    async fn test_classify_worktrees_skips_non_git_root_with_nested_repo(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/repo_a",
            json!({
                ".git": {},
                "src": { "main.rs": "" }
            }),
        )
        .await;
        fs.insert_tree(
            "/repo_b",
            json!({
                ".git": {},
                "src": { "lib.rs": "" }
            }),
        )
        .await;
        // `plain_dir` is NOT a git repo, but contains a nested git repo.
        fs.insert_tree(
            "/plain_dir",
            json!({
                "nested_repo": {
                    ".git": {},
                    "src": { "lib.rs": "" }
                }
            }),
        )
        .await;

        let project = Project::test(
            fs.clone(),
            [
                Path::new("/repo_a"),
                Path::new("/repo_b"),
                Path::new("/plain_dir"),
            ],
            cx,
        )
        .await;

        // Let the worktree scanner discover all `.git` directories.
        cx.executor().run_until_parked();

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, window, cx))
        });

        cx.run_until_parked();

        panel.read_with(cx, |panel, cx| {
            let (git_repos, non_git_paths) =
                git_ui::worktree_service::classify_worktrees(panel.project.read(cx), cx);

            let git_work_dirs: Vec<PathBuf> = git_repos
                .iter()
                .map(|repo| repo.read(cx).work_directory_abs_path.to_path_buf())
                .collect();

            assert_eq!(
                git_repos.len(),
                2,
                "only repo_a and repo_b should be classified as git repos, \
                 but got: {git_work_dirs:?}"
            );
            assert!(
                git_work_dirs.contains(&PathBuf::from("/repo_a")),
                "repo_a should be in git_repos: {git_work_dirs:?}"
            );
            assert!(
                git_work_dirs.contains(&PathBuf::from("/repo_b")),
                "repo_b should be in git_repos: {git_work_dirs:?}"
            );

            assert_eq!(
                non_git_paths,
                vec![PathBuf::from("/plain_dir")],
                "plain_dir should be classified as a non-git path \
                 (not matched to nested_repo inside it)"
            );
        });
    }
    #[gpui::test]
    async fn test_vim_search_does_not_steal_focus_from_agent_panel(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            vim::init(cx);
            search::init(cx);

            // Enable vim mode
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |s| s.vim_mode = Some(true));
            });

            // Load vim keybindings
            let mut vim_key_bindings =
                settings::KeymapFile::load_asset_allow_partial_failure("keymaps/vim.json", cx)
                    .unwrap();
            for key_binding in &mut vim_key_bindings {
                key_binding.set_meta(settings::KeybindSource::Vim.meta());
            }
            cx.bind_keys(vim_key_bindings);
        });

        // Create a project with a file so we have a buffer in the center pane.
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "hello world" }))
            .await;
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        // Open a file in the center pane.
        workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_paths(
                    vec![PathBuf::from("/project/file.txt")],
                    workspace::OpenOptions::default(),
                    None,
                    window,
                    cx,
                )
            })
            .await;
        cx.run_until_parked();

        // Add a BufferSearchBar to the center pane's toolbar, as a real
        // workspace would have.
        workspace.update_in(&mut cx, |workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.toolbar().update(cx, |toolbar, cx| {
                    let search_bar = cx.new(|cx| search::BufferSearchBar::new(None, window, cx));
                    toolbar.add_item(search_bar, window, cx);
                });
            });
        });

        // Create the agent panel and add it to the workspace.
        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Open a thread so the panel has an active editor.
        open_thread_with_connection(&panel, StubAgentConnection::new(), &mut cx);

        // Focus the agent panel.
        workspace.update_in(&mut cx, |workspace, window, cx| {
            workspace.focus_panel::<AgentPanel>(window, cx);
        });
        cx.run_until_parked();

        // Verify the agent panel has focus.
        workspace.update_in(&mut cx, |_, window, cx| {
            assert!(
                panel.read(cx).focus_handle(cx).contains_focused(window, cx),
                "Agent panel should be focused before pressing '/'"
            );
        });

        // Press '/' — the vim search keybinding.
        cx.simulate_keystrokes("/");

        // Focus should remain on the agent panel.
        workspace.update_in(&mut cx, |_, window, cx| {
            assert!(
                panel.read(cx).focus_handle(cx).contains_focused(window, cx),
                "Focus should remain on the agent panel after pressing '/'"
            );
        });
    }

    /// Connection that tracks closed sessions and detects prompts against
    /// sessions that no longer exist, used to reproduce session disassociation.
    #[derive(Clone, Default)]
    struct DisassociationTrackingConnection {
        next_session_number: Arc<Mutex<usize>>,
        sessions: Arc<Mutex<HashSet<acp::SessionId>>>,
        closed_sessions: Arc<Mutex<Vec<acp::SessionId>>>,
        missing_prompt_sessions: Arc<Mutex<Vec<acp::SessionId>>>,
    }

    impl DisassociationTrackingConnection {
        fn new() -> Self {
            Self::default()
        }

        fn create_session(
            self: Rc<Self>,
            session_id: acp::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Entity<AcpThread> {
            self.sessions.lock().insert(session_id.clone());

            let action_log = cx.new(|_| ActionLog::new(project.clone()));
            cx.new(|cx| {
                AcpThread::new(
                    None,
                    title,
                    Some(work_dirs),
                    self,
                    project,
                    action_log,
                    session_id,
                    watch::Receiver::constant(
                        acp::PromptCapabilities::new()
                            .image(true)
                            .audio(true)
                            .embedded_context(true),
                    ),
                    cx,
                )
            })
        }
    }

    impl AgentConnection for DisassociationTrackingConnection {
        fn agent_id(&self) -> AgentId {
            agent::ZED_AGENT_ID.clone()
        }

        fn telemetry_id(&self) -> SharedString {
            "disassociation-tracking-test".into()
        }

        fn new_session(
            self: Rc<Self>,
            project: Entity<Project>,
            work_dirs: PathList,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            let session_id = {
                let mut next_session_number = self.next_session_number.lock();
                let session_id = acp::SessionId::new(format!(
                    "disassociation-tracking-session-{}",
                    *next_session_number
                ));
                *next_session_number += 1;
                session_id
            };
            let thread = self.create_session(session_id, project, work_dirs, None, cx);
            Task::ready(Ok(thread))
        }

        fn supports_load_session(&self) -> bool {
            true
        }

        fn load_session(
            self: Rc<Self>,
            session_id: acp::SessionId,
            project: Entity<Project>,
            work_dirs: PathList,
            title: Option<SharedString>,
            cx: &mut App,
        ) -> Task<Result<Entity<AcpThread>>> {
            let thread = self.create_session(session_id, project, work_dirs, title, cx);
            thread.update(cx, |thread, cx| {
                thread
                    .handle_session_update(
                        acp::SessionUpdate::UserMessageChunk(acp::ContentChunk::new(
                            "Restored user message".into(),
                        )),
                        cx,
                    )
                    .expect("restored user message should be applied");
                thread
                    .handle_session_update(
                        acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                            "Restored assistant message".into(),
                        )),
                        cx,
                    )
                    .expect("restored assistant message should be applied");
            });
            Task::ready(Ok(thread))
        }

        fn supports_close_session(&self) -> bool {
            true
        }

        fn close_session(
            self: Rc<Self>,
            session_id: &acp::SessionId,
            _cx: &mut App,
        ) -> Task<Result<()>> {
            self.sessions.lock().remove(session_id);
            self.closed_sessions.lock().push(session_id.clone());
            Task::ready(Ok(()))
        }

        fn auth_methods(&self) -> &[acp::AuthMethod] {
            &[]
        }

        fn authenticate(&self, _method_id: acp::AuthMethodId, _cx: &mut App) -> Task<Result<()>> {
            Task::ready(Ok(()))
        }

        fn prompt(
            &self,
            _id: UserMessageId,
            params: acp::PromptRequest,
            _cx: &mut App,
        ) -> Task<Result<acp::PromptResponse>> {
            if !self.sessions.lock().contains(&params.session_id) {
                self.missing_prompt_sessions.lock().push(params.session_id);
                return Task::ready(Err(anyhow!("Session not found")));
            }

            Task::ready(Ok(acp::PromptResponse::new(acp::StopReason::EndTurn)))
        }

        fn cancel(&self, _session_id: &acp::SessionId, _cx: &mut App) {}

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
    }

    async fn setup_workspace_panel(
        cx: &mut TestAppContext,
    ) -> (Entity<Workspace>, Entity<AgentPanel>, VisualTestContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({ "file.txt": "" })).await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        (workspace, panel, cx)
    }

    /// Reproduces the retained-thread reset race:
    ///
    /// 1. Thread A is active and Connected.
    /// 2. User switches to thread B → A goes to retained_threads.
    /// 3. A thread_error is set on retained A's thread view.
    /// 4. AgentServersUpdated fires → retained A's handle_agent_servers_updated
    ///    sees has_thread_error=true → calls reset() → close_all_sessions →
    ///    session X removed, state = Loading.
    /// 5. User reopens thread X via open_thread → load_agent_thread checks
    ///    retained A's has_session → returns false (state is Loading) →
    ///    creates new ConversationView C.
    /// 6. Both A's reload task and C's load task complete → both call
    ///    load_session(X) → both get Connected with session X.
    /// 7. A is eventually cleaned up → on_release → close_all_sessions →
    ///    removes session X.
    /// 8. C sends → "Session not found".
    #[gpui::test]
    async fn test_retained_thread_reset_race_disassociates_session(cx: &mut TestAppContext) {
        let (_workspace, panel, mut cx) = setup_workspace_panel(cx).await;
        cx.run_until_parked();

        let connection = DisassociationTrackingConnection::new();
        panel.update(&mut cx, |panel, cx| {
            panel.connection_store.update(cx, |store, cx| {
                store.restart_connection(
                    Agent::Stub,
                    Rc::new(StubAgentServer::new(connection.clone())),
                    cx,
                );
            });
        });
        cx.run_until_parked();

        // Step 1: Open thread A and send a message.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.external_thread(
                Some(Agent::Stub),
                None,
                None,
                None,
                None,
                true,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        send_message(&panel, &mut cx);

        let session_id_a = active_session_id(&panel, &cx);
        let _thread_id_a = active_thread_id(&panel, &cx);

        // Step 2: Open thread B → A goes to retained_threads.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.external_thread(
                Some(Agent::Stub),
                None,
                None,
                None,
                None,
                true,
                AgentThreadSource::AgentPanel,
                window,
                cx,
            );
        });
        cx.run_until_parked();
        send_message(&panel, &mut cx);

        // Confirm A is retained.
        panel.read_with(&cx, |panel, _cx| {
            assert!(
                panel.retained_threads.contains_key(&_thread_id_a),
                "thread A should be in retained_threads after switching to B"
            );
        });

        // Step 3: Set a thread_error on retained A's active thread view.
        // This simulates an API error that occurred before the user switched
        // away, or a transient failure.
        let retained_conversation_a = panel.read_with(&cx, |panel, _cx| {
            panel
                .retained_threads
                .get(&_thread_id_a)
                .expect("thread A should be retained")
                .clone()
        });
        retained_conversation_a.update(&mut cx, |conversation, cx| {
            if let Some(thread_view) = conversation.active_thread() {
                thread_view.update(cx, |view, cx| {
                    view.handle_thread_error(
                        crate::conversation_view::ThreadError::Other {
                            message: "simulated error".into(),
                            acp_error_code: None,
                        },
                        cx,
                    );
                });
            }
        });

        // Confirm the thread error is set.
        retained_conversation_a.read_with(&cx, |conversation, cx| {
            let connected = conversation.as_connected().expect("should be connected");
            assert!(
                connected.has_thread_error(cx),
                "retained A should have a thread error"
            );
        });

        // Step 4: Emit AgentServersUpdated → retained A's
        // handle_agent_servers_updated sees has_thread_error=true,
        // calls reset(), which closes session X and sets state=Loading.
        //
        // Critically, we do NOT call run_until_parked between the emit
        // and open_thread. The emit's synchronous effects (event delivery
        // → reset() → close_all_sessions → state=Loading) happen during
        // the update's flush_effects. But the async reload task spawned
        // by initial_state has NOT been polled yet.
        panel.update(&mut cx, |panel, cx| {
            panel.project.update(cx, |project, cx| {
                project
                    .agent_server_store()
                    .update(cx, |_store, cx| cx.emit(project::AgentServersUpdated));
            });
        });
        // After this update returns, the retained ConversationView is in
        // Loading state (reset ran synchronously), but its async reload
        // task hasn't executed yet.

        // Step 5: Immediately open thread X via open_thread, BEFORE
        // the retained view's async reload completes. load_agent_thread
        // checks retained A's has_session → returns false (state is
        // Loading) → creates a NEW ConversationView C for session X.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.open_thread(session_id_a.clone(), None, None, window, cx);
        });

        // NOW settle everything: both async tasks (A's reload and C's load)
        // complete, both register session X.
        cx.run_until_parked();

        // Verify session A is the active session via C.
        panel.read_with(&cx, |panel, cx| {
            let active_session = panel
                .active_agent_thread(cx)
                .map(|t| t.read(cx).session_id().clone());
            assert_eq!(
                active_session,
                Some(session_id_a.clone()),
                "session A should be the active session after open_thread"
            );
        });

        // Step 6: Force the retained ConversationView A to be dropped
        // while the active view (C) still has the same session.
        // We can't use remove_thread because C shares the same ThreadId
        // and remove_thread would kill the active view too. Instead,
        // directly remove from retained_threads and drop the handle
        // so on_release → close_all_sessions fires only on A.
        drop(retained_conversation_a);
        panel.update(&mut cx, |panel, _cx| {
            panel.retained_threads.remove(&_thread_id_a);
        });
        cx.run_until_parked();

        // The key assertion: sending messages on the ACTIVE view (C)
        // must succeed. If the session was disassociated by A's cleanup,
        // this will fail with "Session not found".
        send_message(&panel, &mut cx);
        send_message(&panel, &mut cx);

        let missing = connection.missing_prompt_sessions.lock().clone();
        assert!(
            missing.is_empty(),
            "session should not be disassociated after retained thread reset race, \
             got missing prompt sessions: {:?}",
            missing
        );

        panel.read_with(&cx, |panel, cx| {
            let active_view = panel
                .active_conversation_view()
                .expect("conversation should remain open");
            let connected = active_view
                .read(cx)
                .as_connected()
                .expect("conversation should be connected");
            assert!(
                !connected.has_thread_error(cx),
                "conversation should not have a thread error"
            );
        });
    }

    #[gpui::test]
    async fn test_initialize_from_source_transfers_draft_to_fresh_panel(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        fs.insert_tree("/project_b", json!({ "file.txt": "" }))
            .await;
        let project_a = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;
        let project_b = Project::test(fs.clone(), [Path::new("/project_b")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        let workspace_a = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        // Set up panel_a with an active thread and type draft text.
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        cx.run_until_parked();

        panel_a.update_in(cx, |panel, window, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });
        cx.run_until_parked();

        let thread_view_a =
            panel_a.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
        let editor_a = thread_view_a.read_with(cx, |view, _cx| view.message_editor.clone());
        editor_a.update_in(cx, |editor, window, cx| {
            editor.set_text("Draft from workspace A", window, cx);
        });

        // Set up panel_b on workspace_b — starts as a fresh, empty panel.
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        cx.run_until_parked();

        // Initializing panel_b from workspace_a should transfer the draft,
        // even if panel_b already has an auto-created empty draft thread
        // (which set_active creates during add_panel).
        let transferred = panel_b.update_in(cx, |panel, window, cx| {
            panel.initialize_from_source_workspace_if_needed(workspace_a.downgrade(), window, cx)
        });
        assert!(
            transferred,
            "fresh destination panel should accept source content"
        );

        // Verify the panel was initialized: the base_view should now be an
        // AgentThread (not Uninitialized) and a draft_thread should be set.
        // We can't check the message editor text directly because the thread
        // needs a connected server session (not available in unit tests without
        // a stub server). The `transferred == true` return already proves that
        // source_panel_initialization read the content successfully.
        panel_b.read_with(cx, |panel, _cx| {
            assert!(
                panel.active_conversation_view().is_some(),
                "panel_b should have a conversation view after initialization"
            );
            assert!(
                panel.draft_thread.is_some(),
                "panel_b should have a draft_thread set after initialization"
            );
        });
    }

    #[gpui::test]
    async fn test_initialize_from_source_inherits_agent_without_draft_content(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        fs.insert_tree("/project_b", json!({ "file.txt": "" }))
            .await;
        let project_a = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;
        let project_b = Project::test(fs.clone(), [Path::new("/project_b")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        let workspace_a = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        panel_a.update(cx, |panel, _cx| {
            panel.selected_agent = Agent::Stub;
        });

        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        let initialized = panel_b.update_in(cx, |panel, window, cx| {
            panel.initialize_from_source_workspace_if_needed(workspace_a.downgrade(), window, cx)
        });
        assert!(
            initialized,
            "fresh destination panel should inherit the source agent"
        );

        panel_b.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent,
                Agent::Stub,
                "destination panel should inherit the source panel's selected agent"
            );
            assert!(
                panel.active_conversation_view().is_none(),
                "agent-only initialization should not create a draft thread"
            );
        });
    }

    #[gpui::test]
    async fn test_initialize_from_source_retargets_empty_destination_draft_agent(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        fs.insert_tree("/project_b", json!({ "file.txt": "" }))
            .await;
        let project_a = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;
        let project_b = Project::test(fs.clone(), [Path::new("/project_b")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        let workspace_a = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        panel_a.update(cx, |panel, _cx| {
            panel.selected_agent = Agent::Stub;
        });

        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        panel_b.update_in(cx, |panel, window, cx| {
            panel.activate_new_thread(false, AgentThreadSource::AgentPanel, window, cx);
        });

        let original_draft = panel_b.read_with(cx, |panel, cx| {
            let draft = panel.draft_thread.as_ref().expect("draft should exist");
            assert_eq!(
                *draft.read(cx).agent_key(),
                Agent::NativeAgent,
                "destination draft should start on the default agent"
            );
            draft.entity_id()
        });

        let initialized = panel_b.update_in(cx, |panel, window, cx| {
            panel.initialize_from_source_workspace_if_needed(workspace_a.downgrade(), window, cx)
        });
        assert!(
            initialized,
            "fresh destination draft should inherit the source agent"
        );

        panel_b.read_with(cx, |panel, cx| {
            let draft = panel.draft_thread.as_ref().expect("draft should exist");
            assert_ne!(
                draft.entity_id(),
                original_draft,
                "empty destination draft should be replaced when the inherited agent differs"
            );
            assert_eq!(
                *draft.read(cx).agent_key(),
                Agent::Stub,
                "empty destination draft should be rebound to the inherited agent"
            );
        });
    }

    #[gpui::test]
    async fn test_initialize_from_source_does_not_overwrite_existing_content(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project_a", json!({ "file.txt": "" }))
            .await;
        fs.insert_tree("/project_b", json!({ "file.txt": "" }))
            .await;
        let project_a = Project::test(fs.clone(), [Path::new("/project_a")], cx).await;
        let project_b = Project::test(fs.clone(), [Path::new("/project_b")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

        let workspace_a = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let workspace_b = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.test_add_workspace(project_b.clone(), window, cx)
            })
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        // Set up panel_a with draft text.
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        cx.run_until_parked();

        panel_a.update_in(cx, |panel, window, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });
        cx.run_until_parked();

        let thread_view_a =
            panel_a.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
        let editor_a = thread_view_a.read_with(cx, |view, _cx| view.message_editor.clone());
        editor_a.update_in(cx, |editor, window, cx| {
            editor.set_text("Draft from workspace A", window, cx);
        });

        // Set up panel_b with its OWN content — this is a non-fresh panel.
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        cx.run_until_parked();

        panel_b.update_in(cx, |panel, window, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });
        cx.run_until_parked();

        let thread_view_b =
            panel_b.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
        let editor_b = thread_view_b.read_with(cx, |view, _cx| view.message_editor.clone());
        editor_b.update_in(cx, |editor, window, cx| {
            editor.set_text("Existing work in workspace B", window, cx);
        });

        // Attempting to initialize panel_b from workspace_a should be rejected
        // because panel_b already has meaningful content.
        let transferred = panel_b.update_in(cx, |panel, window, cx| {
            panel.initialize_from_source_workspace_if_needed(workspace_a.downgrade(), window, cx)
        });
        assert!(
            !transferred,
            "destination panel with existing content should not be overwritten"
        );

        // Verify panel_b still has its original content.
        panel_b.read_with(cx, |panel, cx| {
            let thread_view = panel
                .active_thread_view(cx)
                .expect("panel_b should still have its thread view");
            let text = thread_view.read(cx).message_editor.read(cx).text(cx);
            assert_eq!(
                text, "Existing work in workspace B",
                "destination panel's content should be preserved"
            );
        });
    }

    #[gpui::test]
    async fn test_create_thread_with_options_retains_thread_and_restores_agent(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;
        let _stub_connection =
            crate::test_support::set_stub_agent_connection(StubAgentConnection::new());

        // Baseline: panel's selected_agent is the stub.
        panel.update(&mut cx, |panel, _cx| {
            panel.selected_agent = Agent::Stub;
        });

        // Case 1: no agent override. The new thread should land in
        // `retained_threads` and `selected_agent` should be unchanged.
        let no_override_id = panel.update_in(&mut cx, |panel, window, cx| {
            panel.create_thread_with_options(
                CreateThreadOptions::default(),
                AgentThreadSource::AgentPanel,
                window,
                cx,
            )
        });

        panel.read_with(&cx, |panel, _cx| {
            assert!(
                panel.retained_threads.contains_key(&no_override_id),
                "thread created via create_thread_with_options should be retained"
            );
            assert_eq!(
                panel.selected_agent,
                Agent::Stub,
                "selected_agent should be unchanged when no agent override is requested"
            );
        });

        // Case 2: an explicit agent override that differs from the panel's
        // selection. `create_agent_thread_inner` updates `selected_agent` as a
        // side effect; `create_thread_with_options` must restore it so the
        // user's last-used agent isn't silently flipped by an agent-initiated
        // call.
        let override_agent = Agent::Custom {
            id: "override-agent".into(),
        };
        let override_id = panel.update_in(&mut cx, |panel, window, cx| {
            panel.create_thread_with_options(
                CreateThreadOptions {
                    agent: Some(override_agent.clone()),
                    ..CreateThreadOptions::default()
                },
                AgentThreadSource::AgentPanel,
                window,
                cx,
            )
        });

        panel.read_with(&cx, |panel, _cx| {
            assert!(
                panel.retained_threads.contains_key(&override_id),
                "thread created with an agent override should also be retained"
            );
            assert_ne!(
                no_override_id, override_id,
                "each call should produce a distinct ThreadId"
            );
            assert_eq!(
                panel.selected_agent,
                Agent::Stub,
                "selected_agent should be restored to the original after an agent override"
            );
        });
    }
}
