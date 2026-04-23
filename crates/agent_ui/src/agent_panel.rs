use std::{
    path::PathBuf,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use acp_thread::{AcpThread, AcpThreadEvent, MentionUri, ThreadStatus};
use agent::{ContextServerRegistry, SharedThread, ThreadStore};
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use collections::HashSet;
use db::kvp::{Dismissable, KeyValueStore};
use itertools::Itertools;
use project::AgentId;
use serde::{Deserialize, Serialize};
use settings::{LanguageModelProviderSetting, LanguageModelSelection};

use zed_actions::{
    DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize,
    agent::{
        AddSelectionToThread, ConflictContent, OpenSettings, ReauthenticateAgent, ResetAgentZoom,
        ResetOnboarding, ResolveConflictedFilesWithAgent, ResolveConflictsWithAgent,
        ReviewBranchDiff,
    },
    assistant::{FocusAgent, OpenRulesLibrary, Toggle, ToggleFocus},
};

use crate::DEFAULT_THREAD_TITLE;
use crate::ExpandMessageEditor;
use crate::ManageProfiles;
use crate::agent_connection_store::AgentConnectionStore;
use crate::thread_metadata_store::{ThreadId, ThreadMetadataStore};
use crate::{
    AddContextServer, AgentDiffPane, ConversationView, CopyThreadToClipboard, Follow,
    InlineAssistant, LoadThreadFromClipboard, NewThread, OpenActiveThreadAsMarkdown, OpenAgentDiff,
    ResetTrialEndUpsell, ResetTrialUpsell, ShowAllSidebarThreadMetadata, ShowThreadMetadata,
    ToggleNewThreadMenu, ToggleOptionsMenu,
    agent_configuration::{AgentConfiguration, AssistantConfigurationEvent},
    conversation_view::{AcpThreadViewEvent, ThreadView},
    ui::EndTrialUpsell,
};
use crate::{
    Agent, AgentInitialContent, ExternalSourcePrompt, NewExternalAgentThread,
    NewNativeAgentThreadFromSummary,
};
use agent_settings::AgentSettings;
use ai_onboarding::AgentPanelOnboarding;
use anyhow::Result;
use chrono::{DateTime, Utc};
use client::UserStore;
use cloud_api_types::Plan;
use collections::HashMap;
use editor::{Editor, MultiBuffer};
use extension::ExtensionEvents;
use extension_host::ExtensionStore;
use fs::Fs;
use gpui::{
    Action, Anchor, Animation, AnimationExt, AnyElement, App, AsyncWindowContext, ClipboardItem,
    Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable, KeyContext, Pixels, Subscription,
    Task, UpdateGlobal, WeakEntity, prelude::*, pulsating_between,
};
use language::LanguageRegistry;
use language_model::LanguageModelRegistry;
use project::{Project, ProjectPath, Worktree};
use prompt_store::{PromptStore, UserPromptId};
use rules_library::{RulesLibrary, open_rules_library};
use settings::TerminalDockPosition;
use settings::{Settings, update_settings_file};
use terminal::terminal_settings::TerminalSettings;
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use theme_settings::ThemeSettings;
use ui::{
    Button, Callout, ContextMenu, ContextMenuEntry, IconButton, PopoverMenu, PopoverMenuHandle,
    Tab, Tooltip, prelude::*, utils::WithRemSize,
};
use util::ResultExt as _;
use workspace::{
    CollaboratorId, DraggedSelection, DraggedTab, PathList, SerializedPathList,
    ToggleWorkspaceSidebar, ToggleZoom, Workspace, WorkspaceId,
    dock::{DockPosition, Panel, PanelEvent},
};

const AGENT_PANEL_KEY: &str = "agent_panel";
const MIN_PANEL_WIDTH: Pixels = px(300.);
const LAST_USED_AGENT_KEY: &str = "agent_panel__last_used_external_agent";

/// Maximum number of idle threads kept in the agent panel's retained list.
/// Set as a GPUI global to override; otherwise defaults to 5.
pub struct MaxIdleRetainedThreads(pub usize);
impl gpui::Global for MaxIdleRetainedThreads {}

impl MaxIdleRetainedThreads {
    pub fn global(cx: &App) -> usize {
        cx.try_global::<Self>().map_or(5, |g| g.0)
    }
}

#[derive(Serialize, Deserialize)]
struct LastUsedAgent {
    agent: Agent,
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

#[derive(Serialize, Deserialize, Debug)]
struct SerializedAgentPanel {
    selected_agent: Option<Agent>,
    #[serde(default)]
    last_active_thread: Option<SerializedActiveThread>,
    draft_thread_prompt: Option<Vec<acp::ContentBlock>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SerializedActiveThread {
    session_id: Option<String>,
    agent_type: Agent,
    title: Option<String>,
    work_dirs: Option<SerializedPathList>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, action: &NewThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.new_thread(action, window, cx));
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
                .register_action(|workspace, action: &OpenRulesLibrary, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.deploy_rules_library(action, window, cx)
                        });
                    }
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
                            "git_panel",
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
                                "git_panel",
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
                                "git_panel",
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

                        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                            return;
                        };

                        if !panel.focus_handle(cx).contains_focused(window, cx) {
                            workspace.toggle_panel_focus::<AgentPanel>(window, cx);
                        }

                        panel.update(cx, |_, cx| {
                            cx.defer_in(window, move |panel, window, cx| {
                                if let Some(conversation_view) = panel.active_conversation_view() {
                                    conversation_view.update(cx, |conversation_view, cx| {
                                        conversation_view.insert_selections(window, cx);
                                    });
                                }
                            });
                        });
                    },
                );
        },
    )
    .detach();
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
        "updated_at": format_timestamp_human(&metadata.updated_at),
        "created_at": metadata.created_at.as_ref().map(format_timestamp_human),
        "interacted_at": metadata.interacted_at.as_ref().map(format_timestamp_human),
        "worktree_paths": format!("{:?}", metadata.worktree_paths),
        "archived": metadata.archived,
    })
}

pub(crate) struct AgentThread {
    conversation_view: Entity<ConversationView>,
}

enum BaseView {
    Uninitialized,
    AgentThread {
        conversation_view: Entity<ConversationView>,
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
    Configuration(Option<&'a Entity<AgentConfiguration>>),
}

enum WhichFontSize {
    AgentFont,
    None,
}

impl BaseView {
    pub fn which_font_size_used(&self) -> WhichFontSize {
        WhichFontSize::AgentFont
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
    prompt_store: Option<Entity<PromptStore>>,
    connection_store: Entity<AgentConnectionStore>,
    context_server_registry: Entity<ContextServerRegistry>,
    configuration: Option<Entity<AgentConfiguration>>,
    configuration_subscription: Option<Subscription>,
    focus_handle: FocusHandle,
    base_view: BaseView,
    overlay_view: Option<OverlayView>,
    draft_thread: Option<Entity<ConversationView>>,
    retained_threads: HashMap<ThreadId, Entity<ConversationView>>,
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
    show_trust_workspace_message: bool,
    _base_view_observation: Option<Subscription>,
    _draft_editor_observation: Option<Subscription>,
}

impl AgentPanel {
    fn serialize(&mut self, cx: &mut App) {
        let Some(workspace_id) = self.workspace_id else {
            return;
        };

        let selected_agent = self.selected_agent.clone();

        let is_draft_active = self.active_thread_is_draft(cx);
        let last_active_thread = self
            .active_agent_thread(cx)
            .map(|thread| {
                let thread = thread.read(cx);

                let title = thread.title();
                let work_dirs = thread.work_dirs().cloned();
                SerializedActiveThread {
                    session_id: (!is_draft_active).then(|| thread.session_id().0.to_string()),
                    agent_type: self.selected_agent.clone(),
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
                    agent_type: self.selected_agent.clone(),
                    title: metadata
                        .as_ref()
                        .and_then(|m| m.title.as_ref())
                        .map(|t| t.to_string()),
                    work_dirs: metadata.map(|m| m.folder_paths().serialize()),
                })
            });

        let kvp = KeyValueStore::global(cx);
        let draft_thread_prompt = self.draft_thread.as_ref().and_then(|conversation| {
            Some(
                conversation
                    .read(cx)
                    .root_thread_view()?
                    .read(cx)
                    .thread
                    .read(cx)
                    .draft_prompt()?
                    .to_vec(),
            )
        });
        self.pending_serialization = Some(cx.background_spawn(async move {
            save_serialized_panel(
                workspace_id,
                SerializedAgentPanel {
                    selected_agent: Some(selected_agent),
                    last_active_thread,
                    draft_thread_prompt,
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
        let prompt_store = cx.update(|_window, cx| PromptStore::global(cx));
        let kvp = cx.update(|_window, cx| KeyValueStore::global(cx)).ok();
        cx.spawn(async move |cx| {
            let prompt_store = match prompt_store {
                Ok(prompt_store) => prompt_store.await.ok(),
                Err(_) => None,
            };
            let workspace_id = workspace
                .read_with(cx, |workspace, _| workspace.database_id())
                .ok()
                .flatten();

            let (serialized_panel, global_last_used_agent) = cx
                .background_spawn(async move {
                    match kvp {
                        Some(kvp) => {
                            let panel = workspace_id
                                .and_then(|id| read_serialized_panel(id, &kvp))
                                .or_else(|| read_legacy_serialized_panel(&kvp));
                            let global_agent = read_global_last_used_agent(&kvp);
                            (panel, global_agent)
                        }
                        None => (None, None),
                    }
                })
                .await;

            let was_draft_active = serialized_panel
                .as_ref()
                .and_then(|p| p.last_active_thread.as_ref())
                .is_some_and(|t| t.session_id.is_none());

            let last_active_thread = if let Some(thread_info) = serialized_panel
                .as_ref()
                .and_then(|p| p.last_active_thread.as_ref())
            {
                match &thread_info.session_id {
                    Some(session_id_str) => {
                        let session_id = acp::SessionId::new(session_id_str.clone());
                        let is_restorable = cx
                            .update(|_window, cx| {
                                let store = ThreadMetadataStore::global(cx);
                                store
                                    .read(cx)
                                    .entry_by_session(&session_id)
                                    .is_some_and(|entry| !entry.archived)
                            })
                            .unwrap_or(false);
                        if is_restorable {
                            Some(thread_info)
                        } else {
                            log::info!(
                                "last active thread {} is archived or missing, skipping restoration",
                                session_id_str
                            );
                            None
                        }
                    }
                    None => None,
                }
            } else {
                None
            };

            let panel = workspace.update_in(cx, |workspace, window, cx| {
                let panel = cx.new(|cx| Self::new(workspace, prompt_store, window, cx));

                panel.update(cx, |panel, cx| {
                    let is_via_collab = panel.project.read(cx).is_via_collab();

                    // Only apply a non-native global fallback to local projects.
                    // Collab workspaces only support NativeAgent, so inheriting a
                    // custom agent would cause set_active → new_agent_thread_inner
                    // to bypass the collab guard in external_thread.
                    let global_fallback =
                        global_last_used_agent.filter(|agent| !is_via_collab || agent.is_native());

                    if let Some(serialized_panel) = &serialized_panel {
                        if let Some(selected_agent) = serialized_panel.selected_agent.clone() {
                            panel.selected_agent = selected_agent;
                        } else if let Some(agent) = global_fallback {
                            panel.selected_agent = agent;
                        }
                    } else if let Some(agent) = global_fallback {
                        panel.selected_agent = agent;
                    }
                    cx.notify();
                });

                if let Some(thread_info) = last_active_thread {
                    if let Some(session_id_str) = &thread_info.session_id {
                        let agent = thread_info.agent_type.clone();
                        let session_id: acp::SessionId = session_id_str.clone().into();
                        panel.update(cx, |panel, cx| {
                            panel.selected_agent = agent.clone();
                            panel.load_agent_thread(
                                agent,
                                session_id,
                                thread_info.work_dirs.as_ref().map(|dirs| PathList::deserialize(dirs)),
                                thread_info.title.as_ref().map(|t| t.clone().into()),
                                false,
                                "agent_panel",
                                window,
                                cx,
                            );
                        });
                    }
                }

                let draft_prompt = serialized_panel
                    .as_ref()
                    .and_then(|p| p.draft_thread_prompt.clone());

                if draft_prompt.is_some() || was_draft_active {
                    panel.update(cx, |panel, cx| {
                        let agent = if panel.project.read(cx).is_via_collab() {
                            Agent::NativeAgent
                        } else {
                            panel.selected_agent.clone()
                        };
                        let initial_content = draft_prompt.map(|blocks| {
                            AgentInitialContent::ContentBlock {
                                blocks,
                                auto_submit: false,
                            }
                        });
                        let thread = panel.create_agent_thread(
                            agent,
                            None,
                            None,
                            None,
                            initial_content,
                            "agent_panel",
                            window,
                            cx,
                        );
                        panel.draft_thread = Some(thread.conversation_view.clone());
                        panel.observe_draft_editor(&thread.conversation_view, cx);

                        if was_draft_active && last_active_thread.is_none() {
                            panel.set_base_view(
                                BaseView::AgentThread {
                                    conversation_view: thread.conversation_view,
                                },
                                false,
                                window,
                                cx,
                            );
                        }
                    });
                }

                panel
            })?;

            Ok(panel)
        })
    }

    pub(crate) fn new(
        workspace: &Workspace,
        prompt_store: Option<Entity<PromptStore>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
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
        let extension_subscription = if let Some(extension_events) = ExtensionEvents::try_global(cx)
        {
            Some(
                cx.subscribe(&extension_events, |this, _source, event, cx| match event {
                    extension::Event::ExtensionInstalled(_)
                    | extension::Event::ExtensionUninstalled(_)
                    | extension::Event::ExtensionsInstalledChanged => {
                        this.sync_agent_servers_from_extensions(cx);
                    }
                    _ => {}
                }),
            )
        } else {
            None
        };

        let connection_store = cx.new(|cx| {
            let mut store = AgentConnectionStore::new(project.clone(), cx);
            // Register the native agent right away, so that it is available for
            // the inline assistant etc.
            store.request_connection(
                Agent::NativeAgent,
                Agent::NativeAgent.server(fs.clone(), thread_store.clone()),
                cx,
            );
            store
        });
        let _project_subscription =
            cx.subscribe(&project, |this, _project, event, cx| match event {
                project::Event::WorktreeAdded(_)
                | project::Event::WorktreeRemoved(_)
                | project::Event::WorktreeOrderChanged => {
                    this.update_thread_work_dirs(cx);
                }
                _ => {}
            });
        let mut panel = Self {
            workspace_id,
            base_view,
            overlay_view: None,
            workspace,
            user_store,
            project: project.clone(),
            fs: fs.clone(),
            language_registry,
            prompt_store,
            connection_store,
            configuration: None,
            configuration_subscription: None,
            focus_handle: cx.focus_handle(),
            context_server_registry,
            draft_thread: None,
            retained_threads: HashMap::default(),
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
            show_trust_workspace_message: false,
            new_user_onboarding_upsell_dismissed: AtomicBool::new(OnboardingUpsell::dismissed(cx)),
            _base_view_observation: None,
            _draft_editor_observation: None,
        };

        // Initial sync of agent servers from extensions
        panel.sync_agent_servers_from_extensions(cx);
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

    pub(crate) fn prompt_store(&self) -> &Option<Entity<PromptStore>> {
        &self.prompt_store
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
        self.load_agent_thread(
            crate::Agent::NativeAgent,
            session_id,
            work_dirs,
            title,
            true,
            "agent_panel",
            window,
            cx,
        );
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
        self.activate_draft(false, "agent_panel", window, cx);
        self.serialize(cx);
        cx.emit(AgentPanelEvent::ActiveViewChanged);
        cx.notify();
    }

    pub fn new_thread(&mut self, _action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        self.activate_draft(true, "agent_panel", window, cx);
    }

    pub fn new_external_agent_thread(
        &mut self,
        action: &NewExternalAgentThread,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(agent) = action.agent.clone() {
            self.selected_agent = agent;
        }
        self.activate_draft(true, "agent_panel", window, cx);
    }

    pub fn activate_draft(
        &mut self,
        focus: bool,
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let draft = self.ensure_draft(source, window, cx);
        if let BaseView::AgentThread { conversation_view } = &self.base_view {
            if conversation_view.entity_id() == draft.entity_id() {
                if focus {
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
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ConversationView> {
        let desired_agent = self.selected_agent(cx);
        if let Some(draft) = &self.draft_thread {
            let agent_matches = *draft.read(cx).agent_key() == desired_agent;
            if agent_matches {
                return draft.clone();
            }
            self.draft_thread = None;
            self._draft_editor_observation = None;
        }
        let previous_content = self.active_initial_content(cx);
        let thread = self.create_agent_thread(
            desired_agent,
            None,
            None,
            None,
            previous_content,
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
                |this, _, e: &AcpThreadEvent, cx| {
                    if let AcpThreadEvent::PromptUpdated = e {
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

    pub fn create_thread(
        &mut self,
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ThreadId {
        let agent = self.selected_agent(cx);
        let thread = self.create_agent_thread(agent, None, None, None, None, source, window, cx);
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
        let Some(conversation_view) = self.retained_threads.remove(&id) else {
            return;
        };
        self.set_base_view(
            BaseView::AgentThread { conversation_view },
            focus,
            window,
            cx,
        );
    }

    pub fn remove_thread(&mut self, id: ThreadId, window: &mut Window, cx: &mut Context<Self>) {
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
            self.activate_draft(false, "agent_panel", window, cx);
            self.serialize(cx);
            cx.emit(AgentPanelEvent::ActiveViewChanged);
            cx.notify();
        }
    }

    pub fn active_thread_id(&self, cx: &App) -> Option<ThreadId> {
        match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                Some(conversation_view.read(cx).thread_id)
            }
            _ => None,
        }
    }

    pub fn editor_text(&self, id: ThreadId, cx: &App) -> Option<String> {
        let cv = self
            .retained_threads
            .get(&id)
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
            None
        } else {
            Some(text)
        }
    }

    pub fn clear_editor(&self, id: ThreadId, window: &mut Window, cx: &mut Context<Self>) {
        let cv = self
            .retained_threads
            .get(&id)
            .or_else(|| match &self.base_view {
                BaseView::AgentThread { conversation_view }
                    if conversation_view.read(cx).thread_id == id =>
                {
                    Some(conversation_view)
                }
                _ => None,
            });
        let Some(cv) = cv else { return };
        let Some(tv) = cv.read(cx).root_thread_view() else {
            return;
        };
        let editor = tv.read(cx).message_editor.clone();
        editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
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
                    "agent_panel",
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
        resume_session_id: Option<acp::SessionId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        focus: bool,
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let agent = agent_choice.unwrap_or_else(|| self.selected_agent(cx));
        let thread = self.create_agent_thread(
            agent,
            resume_session_id,
            work_dirs,
            title,
            initial_content,
            source,
            window,
            cx,
        );
        self.set_base_view(thread.into(), focus, window, cx);
    }

    fn deploy_rules_library(
        &mut self,
        action: &OpenRulesLibrary,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        open_rules_library(
            self.language_registry.clone(),
            Box::new(PromptLibraryInlineAssist::new(self.workspace.clone())),
            action
                .prompt_to_select
                .map(|uuid| UserPromptId(uuid).into()),
            cx,
        )
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
            WhichFontSize::None => {}
        }
    }

    pub fn reset_font_size(
        &mut self,
        action: &ResetBufferFontSize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
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

                self.new_thread(&NewThread, window, cx);
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

    /// active thread plus any background threads that are still running or
    /// completed but unseen.
    pub fn parent_threads(&self, cx: &App) -> Vec<Entity<ThreadView>> {
        let mut views = Vec::new();

        if let Some(server_view) = self.active_conversation_view() {
            if let Some(thread_view) = server_view.read(cx).root_thread_view() {
                views.push(thread_view);
            }
        }

        for server_view in self.retained_threads.values() {
            if let Some(thread_view) = server_view.read(cx).root_thread_view() {
                views.push(thread_view);
            }
        }

        views
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
            let thread_agent = conversation_view.read(cx).agent_key().clone();
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
                        cx.emit(AgentPanelEvent::ThreadFocused);
                        cx.notify();
                    }));
                Some(cx.observe_in(
                    conversation_view,
                    window,
                    |this, server_view, window, cx| {
                        this._thread_view_subscription =
                            Self::subscribe_to_active_thread_view(&server_view, window, cx);
                        cx.emit(AgentPanelEvent::ActiveViewChanged);
                        this.serialize(cx);
                        cx.notify();
                    },
                ))
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
                        if this.draft_thread.as_ref().is_some_and(|d| {
                            this.active_conversation_view()
                                .is_some_and(|active| active.entity_id() == d.entity_id())
                        }) {
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

    fn sync_agent_servers_from_extensions(&mut self, cx: &mut Context<Self>) {
        if let Some(extension_store) = ExtensionStore::try_global(cx) {
            let (manifests, extensions_dir) = {
                let store = extension_store.read(cx);
                let installed = store.installed_extensions();
                let manifests: Vec<_> = installed
                    .iter()
                    .map(|(id, entry)| (id.clone(), entry.manifest.clone()))
                    .collect();
                let extensions_dir = paths::extensions_dir().join("installed");
                (manifests, extensions_dir)
            };

            self.project.update(cx, |project, cx| {
                project.agent_server_store().update(cx, |store, cx| {
                    let manifest_refs: Vec<_> = manifests
                        .iter()
                        .map(|(id, manifest)| (id.as_ref(), manifest.as_ref()))
                        .collect();
                    store.sync_extension_agents(manifest_refs, extensions_dir, cx);
                });
            });
        }
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
            "agent_panel",
            window,
            cx,
        );
    }

    pub fn load_agent_thread(
        &mut self,
        agent: Agent,
        session_id: acp::SessionId,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        focus: bool,
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(store) = ThreadMetadataStore::try_global(cx) {
            let thread_id = store
                .read(cx)
                .entry_by_session(&session_id)
                .map(|t| t.thread_id);
            if let Some(thread_id) = thread_id {
                store.update(cx, |store, cx| {
                    store.unarchive(thread_id, cx);
                });
            }
        }

        let has_session = |cv: &Entity<ConversationView>| -> bool {
            cv.read(cx)
                .root_session_id
                .as_ref()
                .is_some_and(|id| id == &session_id)
        };

        // Check if the active view already has this session.
        if let BaseView::AgentThread { conversation_view } = &self.base_view {
            if has_session(conversation_view) {
                self.clear_overlay_state();
                cx.emit(AgentPanelEvent::ActiveViewChanged);
                return;
            }
        }

        // Check if a retained thread has this session — promote it.
        let retained_key = self
            .retained_threads
            .iter()
            .find(|(_, cv)| has_session(cv))
            .map(|(id, _)| *id);
        if let Some(thread_id) = retained_key {
            if let Some(conversation_view) = self.retained_threads.remove(&thread_id) {
                self.set_base_view(
                    BaseView::AgentThread { conversation_view },
                    focus,
                    window,
                    cx,
                );
                return;
            }
        }

        self.external_thread(
            Some(agent),
            Some(session_id),
            work_dirs,
            title,
            None,
            focus,
            source,
            window,
            cx,
        );
    }

    pub(crate) fn create_agent_thread(
        &mut self,
        agent: Agent,
        resume_session_id: Option<acp::SessionId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AgentThread {
        self.create_agent_thread_with_server(
            agent,
            None,
            resume_session_id,
            work_dirs,
            title,
            initial_content,
            source,
            window,
            cx,
        )
    }

    fn create_agent_thread_with_server(
        &mut self,
        agent: Agent,
        server_override: Option<Rc<dyn AgentServer>>,
        resume_session_id: Option<acp::SessionId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        source: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AgentThread {
        let existing_metadata = resume_session_id.as_ref().and_then(|sid| {
            ThreadMetadataStore::try_global(cx)
                .and_then(|store| store.read(cx).entry_by_session(sid).cloned())
        });
        let thread_id = existing_metadata
            .as_ref()
            .map(|m| m.thread_id)
            .unwrap_or_else(ThreadId::new);
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
                self.prompt_store.clone(),
                source,
                window,
                cx,
            )
        });

        cx.observe(&conversation_view, |this, server_view, cx| {
            let is_active = this
                .active_conversation_view()
                .is_some_and(|active| active.entity_id() == server_view.entity_id());
            if is_active {
                cx.emit(AgentPanelEvent::ActiveViewChanged);
                this.serialize(cx);
            } else {
                cx.emit(AgentPanelEvent::RetainedThreadChanged);
            }
            cx.notify();
        })
        .detach();

        AgentThread { conversation_view }
    }

    fn active_thread_has_messages(&self, cx: &App) -> bool {
        self.active_agent_thread(cx)
            .is_some_and(|thread| !thread.read(cx).entries().is_empty())
    }

    pub fn active_thread_is_draft(&self, _cx: &App) -> bool {
        self.draft_thread.as_ref().is_some_and(|draft| {
            self.active_conversation_view()
                .is_some_and(|active| active.entity_id() == draft.entity_id())
        })
    }
}

impl Focusable for AgentPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self.visible_surface() {
            VisibleSurface::Uninitialized => self.focus_handle.clone(),
            VisibleSurface::AgentThread(conversation_view) => conversation_view.focus_handle(cx),
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
    ThreadFocused,
    RetainedThreadChanged,
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
            self.activate_draft(false, "agent_panel", window, cx);
        }
    }

    fn destination_has_meaningful_state(&self, cx: &App) -> bool {
        if self.overlay_view.is_some() || !self.retained_threads.is_empty() {
            return true;
        }

        match &self.base_view {
            BaseView::Uninitialized => false,
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
        self.active_thread_view(cx).and_then(|thread_view| {
            thread_view
                .read(cx)
                .thread
                .read(cx)
                .draft_prompt()
                .map(|draft| AgentInitialContent::ContentBlock {
                    blocks: draft.to_vec(),
                    auto_submit: false,
                })
                .filter(|initial_content| match initial_content {
                    AgentInitialContent::ContentBlock { blocks, .. } => !blocks.is_empty(),
                    _ => true,
                })
                .or_else(|| {
                    let text = thread_view.read(cx).message_editor.read(cx).text(cx);
                    if text.trim().is_empty() {
                        None
                    } else {
                        Some(AgentInitialContent::ContentBlock {
                            blocks: vec![acp::ContentBlock::Text(acp::TextContent::new(text))],
                            auto_submit: false,
                        })
                    }
                })
        })
    }

    fn source_panel_initialization(
        source_workspace: &WeakEntity<Workspace>,
        cx: &App,
    ) -> Option<(Agent, AgentInitialContent)> {
        let source_workspace = source_workspace.upgrade()?;
        let source_panel = source_workspace.read(cx).panel::<AgentPanel>(cx)?;
        let source_panel = source_panel.read(cx);
        let initial_content = source_panel.active_initial_content(cx)?;
        let agent = if source_panel.project.read(cx).is_via_collab() {
            Agent::NativeAgent
        } else {
            source_panel.selected_agent.clone()
        };
        Some((agent, initial_content))
    }

    pub fn initialize_from_source_workspace_if_needed(
        &mut self,
        source_workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.destination_has_meaningful_state(cx) {
            return false;
        }

        let Some((agent, initial_content)) =
            Self::source_panel_initialization(&source_workspace, cx)
        else {
            return false;
        };

        let thread = self.create_agent_thread(
            agent,
            None,
            None,
            None,
            Some(initial_content),
            "agent_panel",
            window,
            cx,
        );
        self.draft_thread = Some(thread.conversation_view.clone());
        self.observe_draft_editor(&thread.conversation_view, cx);
        self.set_base_view(thread.into(), false, window, cx);
        true
    }

    fn render_title_view(&self, _window: &mut Window, cx: &Context<Self>) -> AnyElement {
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
                        Label::new(DEFAULT_THREAD_TITLE)
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
                                .items_center()
                                .child(editable_title)
                                .child(
                                    IconButton::new("retry-thread-title", IconName::XCircle)
                                        .icon_color(Color::Error)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text("Title generation failed. Retry"))
                                        .on_click({
                                            let conversation_view = conversation_view.clone();
                                            move |_event, _window, cx| {
                                                Self::handle_regenerate_thread_title(
                                                    conversation_view.clone(),
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
            VisibleSurface::Configuration(_) => {
                Label::new("Settings").truncate().into_any_element()
            }
            VisibleSurface::Uninitialized => Label::new("Agent").truncate().into_any_element(),
        };

        h_flex()
            .key_context("TitleEditor")
            .id("TitleEditor")
            .flex_grow()
            .w_full()
            .max_w_full()
            .overflow_x_scroll()
            .child(content)
            .into_any()
    }

    fn handle_regenerate_thread_title(conversation_view: Entity<ConversationView>, cx: &mut App) {
        conversation_view.update(cx, |conversation_view, cx| {
            if let Some(thread) = conversation_view.as_native_thread(cx) {
                thread.update(cx, |thread, cx| {
                    if !thread.is_generating_title() {
                        thread.generate_title(cx);
                        cx.notify();
                    }
                });
            }
        });
    }

    fn render_panel_options_menu(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        let conversation_view = match &self.base_view {
            BaseView::AgentThread { conversation_view } => Some(conversation_view.clone()),
            _ => None,
        };

        let can_regenerate_thread_title =
            conversation_view.as_ref().is_some_and(|conversation_view| {
                let conversation_view = conversation_view.read(cx);
                conversation_view.has_user_submitted_prompt(cx)
                    && conversation_view.as_native_thread(cx).is_some()
            });

        let has_auth_methods = match &self.base_view {
            BaseView::AgentThread { conversation_view } => {
                conversation_view.read(cx).has_auth_methods()
            }
            _ => false,
        };

        PopoverMenu::new("agent-options-menu")
            .trigger_with_tooltip(
                IconButton::new("agent-options-menu", IconName::Ellipsis)
                    .icon_size(IconSize::Small),
                {
                    let focus_handle = focus_handle.clone();
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "Toggle Agent Menu",
                            &ToggleOptionsMenu,
                            &focus_handle,
                            cx,
                        )
                    }
                },
            )
            .anchor(Anchor::TopRight)
            .with_handle(self.agent_panel_menu_handle.clone())
            .menu({
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |mut menu, _window, _| {
                        menu = menu.context(focus_handle.clone());

                        if can_regenerate_thread_title {
                            menu = menu.header("Current Thread");

                            if let Some(conversation_view) = conversation_view.as_ref() {
                                menu = menu
                                    .entry("Regenerate Thread Title", None, {
                                        let conversation_view = conversation_view.clone();
                                        move |_, cx| {
                                            Self::handle_regenerate_thread_title(
                                                conversation_view.clone(),
                                                cx,
                                            );
                                        }
                                    })
                                    .separator();
                            }
                        }

                        menu = menu
                            .header("MCP Servers")
                            .action(
                                "View Server Extensions",
                                Box::new(zed_actions::Extensions {
                                    category_filter: Some(
                                        zed_actions::ExtensionCategoryFilter::ContextServers,
                                    ),
                                    id: None,
                                }),
                            )
                            .action("Add Custom Server…", Box::new(AddContextServer))
                            .separator()
                            .action("Rules", Box::new(OpenRulesLibrary::default()))
                            .action("Profiles", Box::new(ManageProfiles::default()))
                            .action("Settings", Box::new(OpenSettings))
                            .separator()
                            .action("Toggle Threads Sidebar", Box::new(ToggleWorkspaceSidebar));

                        if has_auth_methods {
                            menu = menu.action("Reauthenticate", Box::new(ReauthenticateAgent))
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

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_server_store = self.project.read(cx).agent_server_store().clone();

        let focus_handle = self.focus_handle(cx);

        let (selected_agent_custom_icon, selected_agent_label) =
            if let Agent::Custom { id, .. } = &self.selected_agent {
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
            BaseView::Uninitialized => None,
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
                                .when(is_agent_selected(Agent::NativeAgent), |this| {
                                    this.action(Box::new(NewExternalAgentThread { agent: None }))
                                })
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
                                                        panel.new_external_agent_thread(
                                                            &NewExternalAgentThread {
                                                                agent: Some(Agent::NativeAgent),
                                                            },
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
                                        is_agent_selected(Agent::Custom {
                                            id: item.id.clone(),
                                        }),
                                        |this| {
                                            this.action(Box::new(NewExternalAgentThread {
                                                agent: None,
                                            }))
                                        },
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
                                                                    agent: Some(Agent::Custom {
                                                                        id: agent_id.clone(),
                                                                    }),
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
        let selected_agent_custom_icon_for_button = selected_agent_custom_icon.clone();
        let selected_agent_builtin_icon = self.selected_agent.icon();
        let selected_agent_label_for_tooltip = selected_agent_label.clone();

        let selected_agent = div()
            .id("selected_agent_icon")
            .when_some(selected_agent_custom_icon, |this, icon_path| {
                this.px_1().child(
                    Icon::from_external_svg(icon_path)
                        .color(Color::Muted)
                        .size(IconSize::Small),
                )
            })
            .when(!has_custom_icon, |this| {
                this.when_some(selected_agent_builtin_icon, |this, icon| {
                    this.px_1().child(Icon::new(icon).color(Color::Muted))
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

        let is_empty_state = !self.active_thread_has_messages(cx);

        let is_in_history_or_config = self.is_overlay_open();

        let is_full_screen = self.is_zoomed(window, cx);
        let full_screen_button = if is_full_screen {
            IconButton::new("disable-full-screen", IconName::Minimize)
                .icon_size(IconSize::Small)
                .tooltip(move |_, cx| Tooltip::for_action("Disable Full Screen", &ToggleZoom, cx))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.toggle_zoom(&ToggleZoom, window, cx);
                }))
        } else {
            IconButton::new("enable-full-screen", IconName::Maximize)
                .icon_size(IconSize::Small)
                .tooltip(move |_, cx| Tooltip::for_action("Enable Full Screen", &ToggleZoom, cx))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.toggle_zoom(&ToggleZoom, window, cx);
                }))
        };

        let use_v2_empty_toolbar = is_empty_state && !is_in_history_or_config;

        let max_content_width = AgentSettings::get_global(cx).max_content_width;

        let base_container = h_flex()
            .size_full()
            .when(!is_in_history_or_config, |this| {
                this.when_some(max_content_width, |this, max_w| this.max_w(max_w).mx_auto())
            })
            .flex_none()
            .justify_between()
            .gap_2();

        let toolbar_content = if use_v2_empty_toolbar {
            let (chevron_icon, icon_color, label_color) =
                if self.new_thread_menu_handle.is_deployed() {
                    (IconName::ChevronUp, Color::Accent, Color::Accent)
                } else {
                    (IconName::ChevronDown, Color::Muted, Color::Default)
                };

            let agent_icon = if let Some(icon_path) = selected_agent_custom_icon_for_button {
                Icon::from_external_svg(icon_path)
                    .size(IconSize::Small)
                    .color(icon_color)
            } else {
                let icon_name = selected_agent_builtin_icon.unwrap_or(IconName::ZedAgent);
                Icon::new(icon_name).size(IconSize::Small).color(icon_color)
            };

            let agent_selector_button = Button::new("agent-selector-trigger", selected_agent_label)
                .start_icon(agent_icon)
                .color(label_color)
                .end_icon(
                    Icon::new(chevron_icon)
                        .color(icon_color)
                        .size(IconSize::XSmall),
                );

            let agent_selector_menu = PopoverMenu::new("new_thread_menu")
                .trigger_with_tooltip(agent_selector_button, {
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "New Thread…",
                            &ToggleNewThreadMenu,
                            &focus_handle,
                            cx,
                        )
                    }
                })
                .menu({
                    let builder = new_thread_menu_builder.clone();
                    move |window, cx| builder(window, cx)
                })
                .with_handle(self.new_thread_menu_handle.clone())
                .anchor(Anchor::TopLeft)
                .offset(gpui::Point {
                    x: px(1.0),
                    y: px(1.0),
                });

            base_container
                .child(
                    h_flex()
                        .size_full()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .pl(DynamicSpacing::Base04.rems(cx))
                        .child(agent_selector_menu),
                )
                .child(
                    h_flex()
                        .h_full()
                        .flex_none()
                        .gap_1()
                        .pl_1()
                        .pr_1()
                        .child(full_screen_button)
                        .child(self.render_panel_options_menu(window, cx)),
                )
                .into_any_element()
        } else {
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
                        .size_full()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .pl(DynamicSpacing::Base04.rems(cx))
                        .child(if self.is_overlay_open() {
                            self.render_toolbar_back_button(cx).into_any_element()
                        } else {
                            selected_agent.into_any_element()
                        })
                        .child(self.render_title_view(window, cx)),
                )
                .child(
                    h_flex()
                        .h_full()
                        .flex_none()
                        .gap_1()
                        .pl_1()
                        .pr_1()
                        .child(new_thread_menu)
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
            BaseView::Uninitialized => {
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
            BaseView::Uninitialized => false,
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
                let tasks = paths
                    .paths()
                    .iter()
                    .map(|path| {
                        Workspace::project_path_for_path(this.project.clone(), path, false, cx)
                    })
                    .collect::<Vec<_>>();
                cx.spawn_in(window, async move |this, cx| {
                    let mut paths = vec![];
                    let mut added_worktrees = vec![];
                    let opened_paths = futures::future::join_all(tasks).await;
                    for entry in opened_paths {
                        if let Some((worktree, project_path)) = entry.log_err() {
                            added_worktrees.push(worktree);
                            paths.push(project_path);
                        }
                    }
                    this.update_in(cx, |this, window, cx| {
                        this.handle_drop(paths, added_worktrees, window, cx);
                    })
                    .ok();
                })
                .detach();
            }))
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
            BaseView::Uninitialized => {}
        }
    }

    fn render_workspace_trust_message(&self, cx: &Context<Self>) -> Option<impl IntoElement> {
        if !self.show_trust_workspace_message {
            return None;
        }

        let description = "To protect your system, third-party code—like MCP servers—won't run until you mark this workspace as safe.";

        Some(
            Callout::new()
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .border_position(ui::BorderPosition::Bottom)
                .title("You're in Restricted Mode")
                .description(description)
                .actions_slot(
                    Button::new("open-trust-modal", "Configure Project Trust")
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Outlined)
                        .on_click({
                            cx.listener(move |this, _, window, cx| {
                                this.workspace
                                    .update(cx, |workspace, cx| {
                                        workspace
                                            .show_worktree_trust_security_modal(true, window, cx)
                                    })
                                    .log_err();
                            })
                        }),
                ),
        )
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AgentPanel");
        match &self.base_view {
            BaseView::AgentThread { .. } => key_context.add("acp_thread"),
            BaseView::Uninitialized => {}
        }
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
            .relative()
            .size_full()
            .justify_between()
            .key_context(self.key_context())
            .on_action(cx.listener(|this, action: &NewThread, window, cx| {
                this.new_thread(action, window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenSettings, window, cx| {
                this.open_configuration(window, cx);
            }))
            .on_action(cx.listener(Self::open_active_thread_as_markdown))
            .on_action(cx.listener(Self::deploy_rules_library))
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
            .child(self.render_toolbar(window, cx))
            .children(self.render_workspace_trust_message(cx))
            .children(self.render_new_user_onboarding(window, cx))
            .map(|parent| match self.visible_surface() {
                VisibleSurface::Uninitialized => parent,
                VisibleSurface::AgentThread(conversation_view) => parent
                    .child(conversation_view.clone())
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

struct PromptLibraryInlineAssist {
    workspace: WeakEntity<Workspace>,
}

impl PromptLibraryInlineAssist {
    pub fn new(workspace: WeakEntity<Workspace>) -> Self {
        Self { workspace }
    }
}

impl rules_library::InlineAssistDelegate for PromptLibraryInlineAssist {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut Context<RulesLibrary>,
    ) {
        InlineAssistant::update_global(cx, |assistant, cx| {
            let Some(workspace) = self.workspace.upgrade() else {
                return;
            };
            let Some(panel) = workspace.read(cx).panel::<AgentPanel>(cx) else {
                return;
            };
            let project = workspace.read(cx).project().downgrade();
            let panel = panel.read(cx);
            let thread_store = panel.thread_store().clone();
            assistant.assist(
                prompt_editor,
                self.workspace.clone(),
                project,
                thread_store,
                None,
                initial_prompt,
                window,
                cx,
            );
        })
    }

    fn focus_agent_panel(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        workspace.focus_panel::<AgentPanel>(window, cx).is_some()
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
        Self::new(workspace, None, window, cx)
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
            "agent_panel",
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

        let thread = self.create_agent_thread_with_server(
            ext_agent,
            Some(server),
            Some(resume_session_id),
            None,
            None,
            None,
            "agent_panel",
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
            "agent_panel",
            window,
            cx,
        );
        self.draft_thread = Some(thread.conversation_view.clone());
        self.set_base_view(thread.into(), true, window, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NewWorktreeBranchTarget;
    use crate::conversation_view::tests::{StubAgentServer, init_test};
    use crate::test_support::{
        active_session_id, active_thread_id, open_thread_with_connection,
        open_thread_with_custom_connection, send_message,
    };
    use acp_thread::{AgentConnection, StubAgentConnection, ThreadStatus, UserMessageId};
    use action_log::ActionLog;
    use anyhow::{Result, anyhow};
    use feature_flags::FeatureFlagAppExt;
    use fs::FakeFs;
    use gpui::{App, TestAppContext, VisualTestContext};
    use parking_lot::Mutex;
    use project::Project;
    use std::any::Any;

    use serde_json::json;
    use std::path::Path;
    use std::sync::Arc;
    use std::time::Instant;
    use workspace::MultiWorkspace;

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
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
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
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
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
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
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
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
            panel.activate_draft(true, "agent_panel", window, cx);
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

        // No metadata should exist yet for a draft.
        cx.update(|_window, cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            assert!(
                store.entry(thread_id).is_none(),
                "draft thread should not have metadata in the store"
            );
        });

        // Set draft prompt and serialize — the draft should survive a round-trip
        // with its prompt intact but a fresh ACP session.
        let draft_prompt_blocks = vec![acp::ContentBlock::Text(acp::TextContent::new(
            "Hello from draft",
        ))];
        panel.update(cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            thread.update(cx, |thread, cx| {
                thread.set_draft_prompt(Some(draft_prompt_blocks.clone()), cx);
            });
            panel.serialize(cx);
        });
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
                panel.draft_thread.is_some(),
                "reloaded panel should have a draft_thread"
            );
        });

        let reloaded_session_id = active_session_id(&reloaded_panel, cx);
        assert_ne!(
            reloaded_session_id, draft_session_id,
            "reloaded draft should have a fresh ACP session ID"
        );

        let restored_text = reloaded_panel.read_with(cx, |panel, cx| {
            let thread_id = panel.active_thread_id(cx).unwrap();
            panel.editor_text(thread_id, cx)
        });
        assert_eq!(
            restored_text.as_deref(),
            Some("Hello from draft"),
            "draft prompt text should be preserved across serialization"
        );

        // Send a message on the reloaded panel — this promotes the draft to a real thread.
        let panel = reloaded_panel;
        let draft_session_id = reloaded_session_id;
        let thread_id = active_thread_id(&panel, cx);
        send_message(&panel, cx);

        // Verify promotion: draft_thread is cleared, metadata exists.
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
                "same thread ID should remain active after promotion"
            );
        });

        cx.update(|_window, cx| {
            let store = ThreadMetadataStore::global(cx).read(cx);
            let metadata = store
                .entry(thread_id)
                .expect("promoted thread should have metadata");
            assert!(
                metadata.session_id.is_some(),
                "promoted thread metadata should have a real session_id"
            );
            assert_eq!(
                metadata.session_id.as_ref().unwrap(),
                &draft_session_id,
                "metadata session_id should match the thread's ACP session"
            );
        });

        // Serialize the panel, then reload it.
        panel.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_panel = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        cx.run_until_parked();

        // The loaded panel should restore the real thread (not the draft).
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

    async fn setup_panel(cx: &mut TestAppContext) -> (Entity<AgentPanel>, VisualTestContext) {
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
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
        });

        (panel, cx)
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
                session_id_a.clone(),
                None,
                None,
                true,
                "agent_panel",
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
                "agent_panel",
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
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
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
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
        });
        panel_a.update(cx, |panel, _cx| {
            panel.selected_agent = agent_a.clone();
        });

        // Set up workspace B with agent_b
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
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

        let custom_agent = Agent::Custom {
            id: "my-custom-agent".into(),
        };

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Create a draft with the default NativeAgent.
        panel.update_in(cx, |panel, window, cx| {
            panel.activate_draft(true, "agent_panel", window, cx);
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
            panel.activate_draft(true, "agent_panel", window, cx);
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
            panel.activate_draft(true, "agent_panel", window, cx);
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

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        // Create a draft using the Stub agent, which connects synchronously.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Stub;
            panel.activate_draft(true, "agent_panel", window, cx);
        });
        cx.run_until_parked();

        let initial_draft_id = panel.read_with(cx, |panel, _cx| {
            panel.draft_thread.as_ref().unwrap().entity_id()
        });

        // Type some text into the draft editor.
        let thread_view = panel.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
        let message_editor = thread_view.read_with(cx, |view, _cx| view.message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Don't lose me!", window, cx);
        });

        // Press cmd-n (activate_draft again with the same agent).
        cx.dispatch_action(NewExternalAgentThread { agent: None });
        cx.run_until_parked();

        // The draft entity should not have changed.
        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.draft_thread.as_ref().unwrap().entity_id(),
                initial_draft_id,
                "cmd-n should not replace the draft when already on it"
            );
        });

        // The editor content should be preserved.
        let thread_id = panel.read_with(cx, |panel, cx| panel.active_thread_id(cx).unwrap());
        let text = panel.read_with(cx, |panel, cx| panel.editor_text(thread_id, cx));
        assert_eq!(
            text.as_deref(),
            Some("Don't lose me!"),
            "typed content should be preserved when pressing cmd-n on the draft"
        );
    }

    #[gpui::test]
    async fn test_draft_content_carried_over_when_switching_agents(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

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

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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

        // Type text into the first draft's editor.
        let thread_view = panel.read_with(cx, |panel, cx| panel.active_thread_view(cx).unwrap());
        let message_editor = thread_view.read_with(cx, |view, _cx| view.message_editor.clone());
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("carry me over", window, cx);
        });

        // Switch to a different agent. ensure_draft should extract the typed
        // content from the old draft and pre-fill the new one.
        cx.dispatch_action(NewExternalAgentThread {
            agent: Some(Agent::Stub),
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
        });

        // The new draft's editor should contain the text typed in the old draft.
        let thread_id = panel.read_with(cx, |panel, cx| panel.active_thread_id(cx).unwrap());
        let text = panel.read_with(cx, |panel, cx| panel.editor_text(thread_id, cx));
        assert_eq!(
            text.as_deref(),
            Some("carry me over"),
            "content should be carried over to the new agent's draft"
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
        let session_id_a = active_session_id(&panel, &cx);
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
                session_id_a.clone(),
                None,
                None,
                true,
                "agent_panel",
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
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
        let project = Project::test(fs.clone(), [], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(multi_workspace.into(), cx);

        let panel = workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
                "agent_panel",
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
                "agent_panel",
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
        let project_a = Project::test(fs.clone(), [], cx).await;
        let project_b = Project::test(fs.clone(), [], cx).await;

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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
    async fn test_initialize_from_source_does_not_overwrite_existing_content(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project_a = Project::test(fs.clone(), [], cx).await;
        let project_b = Project::test(fs.clone(), [], cx).await;

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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
}
