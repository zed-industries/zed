use std::{
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use acp_thread::{AcpThread, MentionUri, ThreadStatus};
use agent::{ContextServerRegistry, SharedThread, ThreadStore};
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use collections::HashSet;
use db::kvp::{Dismissable, KeyValueStore};
use itertools::Itertools;
use project::AgentId;
use serde::{Deserialize, Serialize};
use settings::{LanguageModelProviderSetting, LanguageModelSelection};

use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt as _};
use zed_actions::agent::{
    ConflictContent, OpenClaudeAgentOnboardingModal, ReauthenticateAgent,
    ResolveConflictedFilesWithAgent, ResolveConflictsWithAgent, ReviewBranchDiff,
};

use crate::{
    AddContextServer, AgentDiffPane, ConversationView, CopyThreadToClipboard, CycleStartThreadIn,
    Follow, InlineAssistant, LoadThreadFromClipboard, NewTextThread, NewThread,
    OpenActiveThreadAsMarkdown, OpenAgentDiff, OpenHistory, ResetTrialEndUpsell, ResetTrialUpsell,
    StartThreadIn, ToggleNavigationMenu, ToggleNewThreadMenu, ToggleOptionsMenu,
    agent_configuration::{AgentConfiguration, AssistantConfigurationEvent},
    conversation_view::{AcpThreadViewEvent, ThreadView},
    slash_command::SlashCommandCompletionProvider,
    text_thread_editor::{AgentPanelDelegate, TextThreadEditor, make_lsp_adapter_delegate},
    ui::EndTrialUpsell,
};
use crate::{
    Agent, AgentInitialContent, ExternalSourcePrompt, NewExternalAgentThread,
    NewNativeAgentThreadFromSummary,
};
use crate::{
    DEFAULT_THREAD_TITLE,
    ui::{AcpOnboardingModal, ClaudeCodeOnboardingModal, HoldForDefault},
};
use crate::{
    ExpandMessageEditor, ThreadHistoryView,
    text_thread_history::{TextThreadHistory, TextThreadHistoryEvent},
};
use crate::{ManageProfiles, ThreadHistoryViewEvent};
use crate::{ThreadHistory, agent_connection_store::AgentConnectionStore};
use agent_settings::AgentSettings;
use ai_onboarding::AgentPanelOnboarding;
use anyhow::{Context as _, Result, anyhow};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_text_thread::{TextThread, TextThreadEvent, TextThreadSummary};
use client::UserStore;
use cloud_api_types::Plan;
use collections::HashMap;
use editor::{Anchor, AnchorRangeExt as _, Editor, EditorEvent, MultiBuffer};
use extension::ExtensionEvents;
use extension_host::ExtensionStore;
use fs::Fs;
use gpui::{
    Action, Animation, AnimationExt, AnyElement, App, AsyncWindowContext, ClipboardItem, Corner,
    DismissEvent, Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable, KeyContext, Pixels,
    Subscription, Task, UpdateGlobal, WeakEntity, prelude::*, pulsating_between,
};
use language::LanguageRegistry;
use language_model::{ConfigurationError, LanguageModelRegistry};
use project::project_settings::ProjectSettings;
use project::{Project, ProjectPath, Worktree};
use prompt_store::{PromptBuilder, PromptStore, UserPromptId};
use rules_library::{RulesLibrary, open_rules_library};
use search::{BufferSearchBar, buffer_search};
use settings::{Settings, update_settings_file};
use theme_settings::ThemeSettings;
use ui::{
    Button, Callout, CommonAnimationExt, ContextMenu, ContextMenuEntry, DocumentationSide,
    KeyBinding, PopoverMenu, PopoverMenuHandle, Tab, Tooltip, prelude::*, utils::WithRemSize,
};
use util::{ResultExt as _, debug_panic};
use workspace::{
    CollaboratorId, DraggedSelection, DraggedTab, OpenMode, OpenResult, PathList,
    SerializedPathList, ToggleWorkspaceSidebar, ToggleZoom, ToolbarItemView, Workspace,
    WorkspaceId,
    dock::{DockPosition, Panel, PanelEvent},
};
use zed_actions::{
    DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize,
    agent::{OpenAcpOnboardingModal, OpenSettings, ResetAgentZoom, ResetOnboarding},
    assistant::{OpenRulesLibrary, Toggle, ToggleFocus},
};

const AGENT_PANEL_KEY: &str = "agent_panel";
const RECENTLY_UPDATED_MENU_LIMIT: usize = 6;

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
    selected_agent: Option<AgentType>,
    #[serde(default)]
    last_active_thread: Option<SerializedActiveThread>,
    #[serde(default)]
    start_thread_in: Option<StartThreadIn>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SerializedActiveThread {
    session_id: String,
    agent_type: AgentType,
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
                .register_action(|workspace, _: &OpenHistory, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_history(window, cx));
                    }
                })
                .register_action(|workspace, _: &OpenSettings, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_configuration(window, cx));
                    }
                })
                .register_action(|workspace, _: &NewTextThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.new_text_thread(window, cx);
                        });
                    }
                })
                .register_action(|workspace, action: &NewExternalAgentThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.external_thread(
                                action.agent.clone(),
                                None,
                                None,
                                None,
                                None,
                                true,
                                window,
                                cx,
                            )
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
                                .active_thread()
                                .map(|r| r.read(cx).thread.clone())
                        });

                    if let Some(thread) = thread {
                        AgentDiffPane::deploy_in_workspace(thread, workspace, window, cx);
                    }
                })
                .register_action(|workspace, _: &ToggleNavigationMenu, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            panel.toggle_navigation_menu(&ToggleNavigationMenu, window, cx);
                        });
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
                .register_action(|workspace, _: &OpenAcpOnboardingModal, window, cx| {
                    AcpOnboardingModal::toggle(workspace, window, cx)
                })
                .register_action(
                    |workspace, _: &OpenClaudeAgentOnboardingModal, window, cx| {
                        ClaudeCodeOnboardingModal::toggle(workspace, window, cx)
                    },
                )
                .register_action(|_workspace, _: &ResetOnboarding, window, cx| {
                    window.dispatch_action(workspace::RestoreBanner.boxed_clone(), cx);
                    window.refresh();
                })
                .register_action(|workspace, _: &ResetTrialUpsell, _window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, _| {
                            panel
                                .on_boarding_upsell_dismissed
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
                                window,
                                cx,
                            );
                        });
                    },
                )
                .register_action(|workspace, action: &StartThreadIn, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.set_start_thread_in(action, window, cx);
                        });
                    }
                })
                .register_action(|workspace, _: &CycleStartThreadIn, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.cycle_start_thread_in(window, cx);
                        });
                    }
                });
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum History {
    AgentThreads { view: Entity<ThreadHistoryView> },
    TextThreads,
}

enum ActiveView {
    Uninitialized,
    AgentThread {
        conversation_view: Entity<ConversationView>,
    },
    TextThread {
        text_thread_editor: Entity<TextThreadEditor>,
        title_editor: Entity<Editor>,
        buffer_search_bar: Entity<BufferSearchBar>,
        _subscriptions: Vec<gpui::Subscription>,
    },
    History {
        history: History,
    },
    Configuration,
}

enum WhichFontSize {
    AgentFont,
    BufferFont,
    None,
}

// TODO unify this with ExternalAgent
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentType {
    #[default]
    NativeAgent,
    TextThread,
    Custom {
        #[serde(rename = "name")]
        id: AgentId,
    },
}

impl AgentType {
    pub fn is_native(&self) -> bool {
        matches!(self, Self::NativeAgent)
    }

    fn label(&self) -> SharedString {
        match self {
            Self::NativeAgent | Self::TextThread => "Zed Agent".into(),
            Self::Custom { id, .. } => id.0.clone(),
        }
    }

    fn icon(&self) -> Option<IconName> {
        match self {
            Self::NativeAgent | Self::TextThread => None,
            Self::Custom { .. } => Some(IconName::Sparkle),
        }
    }
}

impl From<Agent> for AgentType {
    fn from(value: Agent) -> Self {
        match value {
            Agent::Custom { id } => Self::Custom { id },
            Agent::NativeAgent => Self::NativeAgent,
        }
    }
}

impl StartThreadIn {
    fn label(&self) -> SharedString {
        match self {
            Self::LocalProject => "Current Worktree".into(),
            Self::NewWorktree => "New Git Worktree".into(),
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum WorktreeCreationStatus {
    Creating,
    Error(SharedString),
}

impl ActiveView {
    pub fn which_font_size_used(&self) -> WhichFontSize {
        match self {
            ActiveView::Uninitialized
            | ActiveView::AgentThread { .. }
            | ActiveView::History { .. } => WhichFontSize::AgentFont,
            ActiveView::TextThread { .. } => WhichFontSize::BufferFont,
            ActiveView::Configuration => WhichFontSize::None,
        }
    }

    pub fn text_thread(
        text_thread_editor: Entity<TextThreadEditor>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let title = text_thread_editor.read(cx).title(cx).to_string();

        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(title, window, cx);
            editor
        });

        // This is a workaround for `editor.set_text` emitting a `BufferEdited` event, which would
        // cause a custom summary to be set. The presence of this custom summary would cause
        // summarization to not happen.
        let mut suppress_first_edit = true;

        let subscriptions = vec![
            window.subscribe(&editor, cx, {
                {
                    let text_thread_editor = text_thread_editor.clone();
                    move |editor, event, window, cx| match event {
                        EditorEvent::BufferEdited => {
                            if suppress_first_edit {
                                suppress_first_edit = false;
                                return;
                            }
                            let new_summary = editor.read(cx).text(cx);

                            text_thread_editor.update(cx, |text_thread_editor, cx| {
                                text_thread_editor
                                    .text_thread()
                                    .update(cx, |text_thread, cx| {
                                        text_thread.set_custom_summary(new_summary, cx);
                                    })
                            })
                        }
                        EditorEvent::Blurred => {
                            if editor.read(cx).text(cx).is_empty() {
                                let summary = text_thread_editor
                                    .read(cx)
                                    .text_thread()
                                    .read(cx)
                                    .summary()
                                    .or_default();

                                editor.update(cx, |editor, cx| {
                                    editor.set_text(summary, window, cx);
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }),
            window.subscribe(&text_thread_editor.read(cx).text_thread().clone(), cx, {
                let editor = editor.clone();
                move |text_thread, event, window, cx| match event {
                    TextThreadEvent::SummaryGenerated => {
                        let summary = text_thread.read(cx).summary().or_default();

                        editor.update(cx, |editor, cx| {
                            editor.set_text(summary, window, cx);
                        })
                    }
                    TextThreadEvent::PathChanged { .. } => {}
                    _ => {}
                }
            }),
        ];

        let buffer_search_bar =
            cx.new(|cx| BufferSearchBar::new(Some(language_registry), window, cx));
        buffer_search_bar.update(cx, |buffer_search_bar, cx| {
            buffer_search_bar.set_active_pane_item(Some(&text_thread_editor), window, cx)
        });

        Self::TextThread {
            text_thread_editor,
            title_editor: editor,
            buffer_search_bar,
            _subscriptions: subscriptions,
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
    text_thread_history: Entity<TextThreadHistory>,
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
    prompt_store: Option<Entity<PromptStore>>,
    connection_store: Entity<AgentConnectionStore>,
    context_server_registry: Entity<ContextServerRegistry>,
    configuration: Option<Entity<AgentConfiguration>>,
    configuration_subscription: Option<Subscription>,
    focus_handle: FocusHandle,
    active_view: ActiveView,
    previous_view: Option<ActiveView>,
    background_threads: HashMap<acp::SessionId, Entity<ConversationView>>,
    new_thread_menu_handle: PopoverMenuHandle<ContextMenu>,
    start_thread_in_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_panel_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_navigation_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_navigation_menu: Option<Entity<ContextMenu>>,
    _extension_subscription: Option<Subscription>,
    _project_subscription: Subscription,
    zoomed: bool,
    pending_serialization: Option<Task<Result<()>>>,
    onboarding: Entity<AgentPanelOnboarding>,
    selected_agent_type: AgentType,
    start_thread_in: StartThreadIn,
    worktree_creation_status: Option<WorktreeCreationStatus>,
    _thread_view_subscription: Option<Subscription>,
    _active_thread_focus_subscription: Option<Subscription>,
    _worktree_creation_task: Option<Task<()>>,
    show_trust_workspace_message: bool,
    last_configuration_error_telemetry: Option<String>,
    on_boarding_upsell_dismissed: AtomicBool,
    _active_view_observation: Option<Subscription>,
}

impl AgentPanel {
    fn serialize(&mut self, cx: &mut App) {
        let Some(workspace_id) = self.workspace_id else {
            return;
        };

        let selected_agent_type = self.selected_agent_type.clone();
        let start_thread_in = Some(self.start_thread_in);

        let last_active_thread = self.active_agent_thread(cx).map(|thread| {
            let thread = thread.read(cx);
            let title = thread.title();
            let work_dirs = thread.work_dirs().cloned();
            SerializedActiveThread {
                session_id: thread.session_id().0.to_string(),
                agent_type: self.selected_agent_type.clone(),
                title: title.map(|t| t.to_string()),
                work_dirs: work_dirs.map(|dirs| dirs.serialize()),
            }
        });

        let kvp = KeyValueStore::global(cx);
        self.pending_serialization = Some(cx.background_spawn(async move {
            save_serialized_panel(
                workspace_id,
                SerializedAgentPanel {
                    selected_agent: Some(selected_agent_type),
                    last_active_thread,
                    start_thread_in,
                },
                kvp,
            )
            .await?;
            anyhow::Ok(())
        }));
    }

    pub fn load(
        workspace: WeakEntity<Workspace>,
        prompt_builder: Arc<PromptBuilder>,
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

            let serialized_panel = cx
                .background_spawn(async move {
                    kvp.and_then(|kvp| {
                        workspace_id
                            .and_then(|id| read_serialized_panel(id, &kvp))
                            .or_else(|| read_legacy_serialized_panel(&kvp))
                    })
                })
                .await;

            let slash_commands = Arc::new(SlashCommandWorkingSet::default());
            let text_thread_store = workspace
                .update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    assistant_text_thread::TextThreadStore::new(
                        project,
                        prompt_builder,
                        slash_commands,
                        cx,
                    )
                })?
                .await?;

            let last_active_thread = if let Some(thread_info) = serialized_panel
                .as_ref()
                .and_then(|p| p.last_active_thread.as_ref())
            {
                if thread_info.agent_type.is_native() {
                    let session_id = acp::SessionId::new(thread_info.session_id.clone());
                    let load_result = cx.update(|_window, cx| {
                        let thread_store = ThreadStore::global(cx);
                        thread_store.update(cx, |store, cx| store.load_thread(session_id, cx))
                    });
                    let thread_exists = if let Ok(task) = load_result {
                        task.await.ok().flatten().is_some()
                    } else {
                        false
                    };
                    if thread_exists {
                        Some(thread_info)
                    } else {
                        log::warn!(
                            "last active thread {} not found in database, skipping restoration",
                            thread_info.session_id
                        );
                        None
                    }
                } else {
                    Some(thread_info)
                }
            } else {
                None
            };

            let panel = workspace.update_in(cx, |workspace, window, cx| {
                let panel =
                    cx.new(|cx| Self::new(workspace, text_thread_store, prompt_store, window, cx));

                if let Some(serialized_panel) = &serialized_panel {
                    panel.update(cx, |panel, cx| {
                        if let Some(selected_agent) = serialized_panel.selected_agent.clone() {
                            panel.selected_agent_type = selected_agent;
                        }
                        if let Some(start_thread_in) = serialized_panel.start_thread_in {
                            let is_worktree_flag_enabled =
                                cx.has_flag::<AgentV2FeatureFlag>();
                            let is_valid = match &start_thread_in {
                                StartThreadIn::LocalProject => true,
                                StartThreadIn::NewWorktree => {
                                    let project = panel.project.read(cx);
                                    is_worktree_flag_enabled && !project.is_via_collab()
                                }
                            };
                            if is_valid {
                                panel.start_thread_in = start_thread_in;
                            } else {
                                log::info!(
                                    "deserialized start_thread_in {:?} is no longer valid, falling back to LocalProject",
                                    start_thread_in,
                                );
                            }
                        }
                        cx.notify();
                    });
                }

                if let Some(thread_info) = last_active_thread {
                    let agent_type = thread_info.agent_type.clone();
                    panel.update(cx, |panel, cx| {
                        panel.selected_agent_type = agent_type;
                        if let Some(agent) = panel.selected_agent() {
                            panel.load_agent_thread(
                                agent,
                                thread_info.session_id.clone().into(),
                                thread_info.work_dirs.as_ref().map(|dirs| PathList::deserialize(dirs)),
                                thread_info.title.as_ref().map(|t| t.clone().into()),
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
        text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
        prompt_store: Option<Entity<PromptStore>>,
        window: &mut Window,
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
        let text_thread_history =
            cx.new(|cx| TextThreadHistory::new(text_thread_store.clone(), window, cx));

        cx.subscribe_in(
            &text_thread_history,
            window,
            |this, _, event, window, cx| match event {
                TextThreadHistoryEvent::Open(thread) => {
                    this.open_saved_text_thread(thread.path.clone(), window, cx)
                        .detach_and_log_err(cx);
                }
            },
        )
        .detach();

        let active_view = ActiveView::Uninitialized;

        let weak_panel = cx.entity().downgrade();

        window.defer(cx, move |window, cx| {
            let panel = weak_panel.clone();
            let agent_navigation_menu =
                ContextMenu::build_persistent(window, cx, move |mut menu, window, cx| {
                    if let Some(panel) = panel.upgrade() {
                        if let Some(history) = panel
                            .update(cx, |panel, cx| panel.history_for_selected_agent(window, cx))
                        {
                            let view_all_label = match history {
                                History::AgentThreads { .. } => "View All",
                                History::TextThreads => "View All Text Threads",
                            };
                            menu = Self::populate_recently_updated_menu_section(
                                menu, panel, history, cx,
                            );
                            menu = menu.action(view_all_label, Box::new(OpenHistory));
                        }
                    }

                    menu = menu
                        .fixed_width(px(320.).into())
                        .keep_open_on_confirm(false)
                        .key_context("NavigationMenu");

                    menu
                });
            weak_panel
                .update(cx, |panel, cx| {
                    cx.subscribe_in(
                        &agent_navigation_menu,
                        window,
                        |_, menu, _: &DismissEvent, window, cx| {
                            menu.update(cx, |menu, _| {
                                menu.clear_selected();
                            });
                            cx.focus_self(window);
                        },
                    )
                    .detach();
                    panel.agent_navigation_menu = Some(agent_navigation_menu);
                })
                .ok();
        });

        let weak_panel = cx.entity().downgrade();
        let onboarding = cx.new(|cx| {
            AgentPanelOnboarding::new(
                user_store.clone(),
                client,
                move |_window, cx| {
                    weak_panel
                        .update(cx, |panel, _| {
                            panel
                                .on_boarding_upsell_dismissed
                                .store(true, Ordering::Release);
                        })
                        .ok();
                    OnboardingUpsell::set_dismissed(true, cx);
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
            active_view,
            workspace,
            user_store,
            project: project.clone(),
            fs: fs.clone(),
            language_registry,
            text_thread_store,
            prompt_store,
            connection_store,
            configuration: None,
            configuration_subscription: None,
            focus_handle: cx.focus_handle(),
            context_server_registry,
            previous_view: None,
            background_threads: HashMap::default(),
            new_thread_menu_handle: PopoverMenuHandle::default(),
            start_thread_in_menu_handle: PopoverMenuHandle::default(),
            agent_panel_menu_handle: PopoverMenuHandle::default(),
            agent_navigation_menu_handle: PopoverMenuHandle::default(),
            agent_navigation_menu: None,
            _extension_subscription: extension_subscription,
            _project_subscription,
            zoomed: false,
            pending_serialization: None,
            onboarding,
            text_thread_history,
            thread_store,
            selected_agent_type: AgentType::default(),
            start_thread_in: StartThreadIn::default(),
            worktree_creation_status: None,
            _thread_view_subscription: None,
            _active_thread_focus_subscription: None,
            _worktree_creation_task: None,
            show_trust_workspace_message: false,
            last_configuration_error_telemetry: None,
            on_boarding_upsell_dismissed: AtomicBool::new(OnboardingUpsell::dismissed(cx)),
            _active_view_observation: None,
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

    pub fn open_thread(
        &mut self,
        session_id: acp::SessionId,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.external_thread(
            Some(crate::Agent::NativeAgent),
            Some(session_id),
            work_dirs,
            title,
            None,
            true,
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

    pub fn new_thread(&mut self, _action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        self.reset_start_thread_in_to_default(cx);
        self.external_thread(None, None, None, None, None, true, window, cx);
    }

    fn new_native_agent_thread_from_summary(
        &mut self,
        action: &NewNativeAgentThreadFromSummary,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let session_id = action.from_session_id.clone();

        let Some(history) = self
            .connection_store
            .read(cx)
            .entry(&Agent::NativeAgent)
            .and_then(|e| e.read(cx).history().cloned())
        else {
            debug_panic!("Native agent is not registered");
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            this.update_in(cx, |this, window, cx| {
                let thread = history
                    .read(cx)
                    .session_for_id(&session_id)
                    .context("Session not found")?;

                this.external_thread(
                    Some(Agent::NativeAgent),
                    None,
                    None,
                    None,
                    Some(AgentInitialContent::ThreadSummary {
                        session_id: thread.session_id,
                        title: thread.title,
                    }),
                    true,
                    window,
                    cx,
                );
                anyhow::Ok(())
            })
        })
        .detach_and_log_err(cx);
    }

    fn new_text_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        telemetry::event!("Agent Thread Started", agent = "zed-text");

        let context = self
            .text_thread_store
            .update(cx, |context_store, cx| context_store.create(cx));
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project, cx)
            .log_err()
            .flatten();

        let text_thread_editor = cx.new(|cx| {
            let mut editor = TextThreadEditor::for_text_thread(
                context,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                window,
                cx,
            );
            editor.insert_default_prompt(window, cx);
            editor
        });

        if self.selected_agent_type != AgentType::TextThread {
            self.selected_agent_type = AgentType::TextThread;
            self.serialize(cx);
        }

        self.set_active_view(
            ActiveView::text_thread(
                text_thread_editor.clone(),
                self.language_registry.clone(),
                window,
                cx,
            ),
            true,
            window,
            cx,
        );
        text_thread_editor.focus_handle(cx).focus(window, cx);
    }

    fn external_thread(
        &mut self,
        agent_choice: Option<crate::Agent>,
        resume_session_id: Option<acp::SessionId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        let project = self.project.clone();
        let fs = self.fs.clone();
        let is_via_collab = self.project.read(cx).is_via_collab();

        const LAST_USED_EXTERNAL_AGENT_KEY: &str = "agent_panel__last_used_external_agent";

        #[derive(Serialize, Deserialize)]
        struct LastUsedExternalAgent {
            agent: crate::Agent,
        }

        let thread_store = self.thread_store.clone();
        let kvp = KeyValueStore::global(cx);

        if let Some(agent) = agent_choice {
            cx.background_spawn({
                let agent = agent.clone();
                let kvp = kvp;
                async move {
                    if let Some(serialized) =
                        serde_json::to_string(&LastUsedExternalAgent { agent }).log_err()
                    {
                        kvp.write_kvp(LAST_USED_EXTERNAL_AGENT_KEY.to_string(), serialized)
                            .await
                            .log_err();
                    }
                }
            })
            .detach();

            let server = agent.server(fs, thread_store);
            self.create_agent_thread(
                server,
                resume_session_id,
                work_dirs,
                title,
                initial_content,
                workspace,
                project,
                agent,
                focus,
                window,
                cx,
            );
        } else {
            cx.spawn_in(window, async move |this, cx| {
                let ext_agent = if is_via_collab {
                    Agent::NativeAgent
                } else {
                    cx.background_spawn(async move { kvp.read_kvp(LAST_USED_EXTERNAL_AGENT_KEY) })
                        .await
                        .log_err()
                        .flatten()
                        .and_then(|value| {
                            serde_json::from_str::<LastUsedExternalAgent>(&value).log_err()
                        })
                        .map(|agent| agent.agent)
                        .unwrap_or(Agent::NativeAgent)
                };

                let server = ext_agent.server(fs, thread_store);
                this.update_in(cx, |agent_panel, window, cx| {
                    agent_panel.create_agent_thread(
                        server,
                        resume_session_id,
                        work_dirs,
                        title,
                        initial_content,
                        workspace,
                        project,
                        ext_agent,
                        focus,
                        window,
                        cx,
                    );
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
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
            Rc::new(|| {
                Rc::new(SlashCommandCompletionProvider::new(
                    Arc::new(SlashCommandWorkingSet::default()),
                    None,
                    None,
                ))
            }),
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

        let Some(active_thread) = conversation_view.read(cx).active_thread().cloned() else {
            return;
        };

        active_thread.update(cx, |active_thread, cx| {
            active_thread.expand_message_editor(&ExpandMessageEditor, window, cx);
            active_thread.focus_handle(cx).focus(window, cx);
        })
    }

    fn has_history_for_selected_agent(&self, cx: &App) -> bool {
        match &self.selected_agent_type {
            AgentType::TextThread | AgentType::NativeAgent => true,
            AgentType::Custom { id } => {
                let agent = Agent::Custom { id: id.clone() };
                self.connection_store
                    .read(cx)
                    .entry(&agent)
                    .map_or(false, |entry| entry.read(cx).history().is_some())
            }
        }
    }

    fn history_for_selected_agent(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<History> {
        match &self.selected_agent_type {
            AgentType::TextThread => Some(History::TextThreads),
            AgentType::NativeAgent => {
                let history = self
                    .connection_store
                    .read(cx)
                    .entry(&Agent::NativeAgent)?
                    .read(cx)
                    .history()?
                    .clone();

                Some(History::AgentThreads {
                    view: self.create_thread_history_view(Agent::NativeAgent, history, window, cx),
                })
            }
            AgentType::Custom { id, .. } => {
                let agent = Agent::Custom { id: id.clone() };
                let history = self
                    .connection_store
                    .read(cx)
                    .entry(&agent)?
                    .read(cx)
                    .history()?
                    .clone();
                Some(History::AgentThreads {
                    view: self.create_thread_history_view(agent, history, window, cx),
                })
            }
        }
    }

    fn create_thread_history_view(
        &self,
        agent: Agent,
        history: Entity<ThreadHistory>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ThreadHistoryView> {
        let view = cx.new(|cx| ThreadHistoryView::new(history.clone(), window, cx));
        cx.subscribe_in(
            &view,
            window,
            move |this, _, event, window, cx| match event {
                ThreadHistoryViewEvent::Open(thread) => {
                    this.load_agent_thread(
                        agent.clone(),
                        thread.session_id.clone(),
                        thread.work_dirs.clone(),
                        thread.title.clone(),
                        true,
                        window,
                        cx,
                    );
                }
            },
        )
        .detach();
        view
    }

    fn open_history(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(history) = self.history_for_selected_agent(window, cx) else {
            return;
        };

        if let ActiveView::History {
            history: active_history,
        } = &self.active_view
        {
            if active_history == &history {
                if let Some(previous_view) = self.previous_view.take() {
                    self.set_active_view(previous_view, true, window, cx);
                }
                return;
            }
        }

        self.set_active_view(ActiveView::History { history }, true, window, cx);
        cx.notify();
    }

    pub(crate) fn open_saved_text_thread(
        &mut self,
        path: Arc<Path>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let text_thread_task = self
            .text_thread_store
            .update(cx, |store, cx| store.open_local(path, cx));
        cx.spawn_in(window, async move |this, cx| {
            let text_thread = text_thread_task.await?;
            this.update_in(cx, |this, window, cx| {
                this.open_text_thread(text_thread, window, cx);
            })
        })
    }

    pub(crate) fn open_text_thread(
        &mut self,
        text_thread: Entity<TextThread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let lsp_adapter_delegate = make_lsp_adapter_delegate(&self.project.clone(), cx)
            .log_err()
            .flatten();
        let editor = cx.new(|cx| {
            TextThreadEditor::for_text_thread(
                text_thread,
                self.fs.clone(),
                self.workspace.clone(),
                self.project.clone(),
                lsp_adapter_delegate,
                window,
                cx,
            )
        });

        if self.selected_agent_type != AgentType::TextThread {
            self.selected_agent_type = AgentType::TextThread;
            self.serialize(cx);
        }

        self.set_active_view(
            ActiveView::text_thread(editor, self.language_registry.clone(), window, cx),
            true,
            window,
            cx,
        );
    }

    pub fn go_back(&mut self, _: &workspace::GoBack, window: &mut Window, cx: &mut Context<Self>) {
        match self.active_view {
            ActiveView::Configuration | ActiveView::History { .. } => {
                if let Some(previous_view) = self.previous_view.take() {
                    self.set_active_view(previous_view, true, window, cx);
                }
                cx.notify();
            }
            _ => {}
        }
    }

    pub fn toggle_navigation_menu(
        &mut self,
        _: &ToggleNavigationMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_history_for_selected_agent(cx) {
            return;
        }
        self.agent_navigation_menu_handle.toggle(window, cx);
    }

    pub fn toggle_options_menu(
        &mut self,
        _: &ToggleOptionsMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
        match self.active_view.which_font_size_used() {
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
            WhichFontSize::BufferFont => {
                // Prompt editor uses the buffer font size, so allow the action to propagate to the
                // default handler that changes that font size.
                cx.propagate();
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
        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let context_server_store = self.project.read(cx).context_server_store();
        let fs = self.fs.clone();

        self.set_active_view(ActiveView::Configuration, true, window, cx);
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

    pub fn background_threads(&self) -> &HashMap<acp::SessionId, Entity<ConversationView>> {
        &self.background_threads
    }

    pub fn active_conversation_view(&self) -> Option<&Entity<ConversationView>> {
        match &self.active_view {
            ActiveView::AgentThread { conversation_view } => Some(conversation_view),
            _ => None,
        }
    }

    pub fn active_thread_view(&self, cx: &App) -> Option<Entity<ThreadView>> {
        let server_view = self.active_conversation_view()?;
        server_view.read(cx).active_thread().cloned()
    }

    pub fn active_agent_thread(&self, cx: &App) -> Option<Entity<AcpThread>> {
        match &self.active_view {
            ActiveView::AgentThread {
                conversation_view, ..
            } => conversation_view
                .read(cx)
                .active_thread()
                .map(|r| r.read(cx).thread.clone()),
            _ => None,
        }
    }

    /// Returns the primary thread views for all retained connections: the
    pub fn is_background_thread(&self, session_id: &acp::SessionId) -> bool {
        self.background_threads.contains_key(session_id)
    }

    pub fn cancel_thread(&self, session_id: &acp::SessionId, cx: &mut Context<Self>) -> bool {
        let conversation_views = self
            .active_conversation_view()
            .into_iter()
            .chain(self.background_threads.values());

        for conversation_view in conversation_views {
            if let Some(thread_view) = conversation_view.read(cx).thread_view(session_id) {
                thread_view.update(cx, |view, cx| view.cancel_generation(cx));
                return true;
            }
        }
        false
    }

    /// active thread plus any background threads that are still running or
    /// completed but unseen.
    pub fn parent_threads(&self, cx: &App) -> Vec<Entity<ThreadView>> {
        let mut views = Vec::new();

        if let Some(server_view) = self.active_conversation_view() {
            if let Some(thread_view) = server_view.read(cx).root_thread(cx) {
                views.push(thread_view);
            }
        }

        for server_view in self.background_threads.values() {
            if let Some(thread_view) = server_view.read(cx).root_thread(cx) {
                views.push(thread_view);
            }
        }

        views
    }

    fn update_thread_work_dirs(&self, cx: &mut Context<Self>) {
        let new_work_dirs = self.project.read(cx).default_path_list(cx);

        // Only update the active thread and still-running background threads.
        // Idle background threads have finished their work against the old
        // worktree set and shouldn't have their metadata rewritten.
        let mut root_threads: Vec<Entity<AcpThread>> = Vec::new();

        if let Some(conversation_view) = self.active_conversation_view() {
            if let Some(connected) = conversation_view.read(cx).as_connected() {
                for thread_view in connected.threads.values() {
                    let thread = &thread_view.read(cx).thread;
                    if thread.read(cx).parent_session_id().is_none() {
                        root_threads.push(thread.clone());
                    }
                }
            }
        }

        for conversation_view in self.background_threads.values() {
            let Some(connected) = conversation_view.read(cx).as_connected() else {
                continue;
            };
            for thread_view in connected.threads.values() {
                let thread = &thread_view.read(cx).thread;
                let thread_ref = thread.read(cx);
                if thread_ref.parent_session_id().is_some() {
                    continue;
                }
                if thread_ref.status() != acp_thread::ThreadStatus::Generating {
                    continue;
                }
                root_threads.push(thread.clone());
            }
        }

        for thread in &root_threads {
            thread.update(cx, |thread, _cx| {
                thread.set_work_dirs(new_work_dirs.clone());
            });
        }

        if let Some(metadata_store) =
            crate::thread_metadata_store::ThreadMetadataStore::try_global(cx)
        {
            metadata_store.update(cx, |store, cx| {
                for thread in &root_threads {
                    let is_archived = store
                        .entry(thread.read(cx).session_id())
                        .map(|t| t.archived)
                        .unwrap_or(false);
                    let metadata = crate::thread_metadata_store::ThreadMetadata::from_thread(
                        is_archived,
                        thread,
                        cx,
                    );
                    store.save(metadata, cx);
                }
            });
        }
    }

    fn retain_running_thread(&mut self, old_view: ActiveView, cx: &mut Context<Self>) {
        let ActiveView::AgentThread { conversation_view } = old_view else {
            return;
        };

        let Some(thread_view) = conversation_view.read(cx).root_thread(cx) else {
            return;
        };

        self.background_threads
            .insert(thread_view.read(cx).id.clone(), conversation_view);
        self.cleanup_background_threads(cx);
    }

    /// We keep threads that are:
    /// - Still running
    /// - Do not support reloading the full session
    /// - Have had the most recent events (up to 5 idle threads)
    fn cleanup_background_threads(&mut self, cx: &App) {
        let mut potential_removals = self
            .background_threads
            .iter()
            .filter(|(_id, view)| {
                let Some(thread_view) = view.read(cx).root_thread(cx) else {
                    return true;
                };
                let thread = thread_view.read(cx).thread.read(cx);
                thread.connection().supports_load_session() && thread.status() == ThreadStatus::Idle
            })
            .collect::<Vec<_>>();

        const MAX_IDLE_BACKGROUND_THREADS: usize = 5;

        potential_removals.sort_unstable_by_key(|(_, view)| view.read(cx).updated_at(cx));
        let n = potential_removals
            .len()
            .saturating_sub(MAX_IDLE_BACKGROUND_THREADS);
        let to_remove = potential_removals
            .into_iter()
            .map(|(id, _)| id.clone())
            .take(n)
            .collect::<Vec<_>>();
        for id in to_remove {
            self.background_threads.remove(&id);
        }
    }

    pub(crate) fn active_native_agent_thread(&self, cx: &App) -> Option<Entity<agent::Thread>> {
        match &self.active_view {
            ActiveView::AgentThread {
                conversation_view, ..
            } => conversation_view.read(cx).as_native_thread(cx),
            _ => None,
        }
    }

    pub(crate) fn active_text_thread_editor(&self) -> Option<Entity<TextThreadEditor>> {
        match &self.active_view {
            ActiveView::TextThread {
                text_thread_editor, ..
            } => Some(text_thread_editor.clone()),
            _ => None,
        }
    }

    fn set_active_view(
        &mut self,
        new_view: ActiveView,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let was_in_agent_history = matches!(
            self.active_view,
            ActiveView::History {
                history: History::AgentThreads { .. }
            }
        );
        let current_is_uninitialized = matches!(self.active_view, ActiveView::Uninitialized);
        let current_is_history = matches!(self.active_view, ActiveView::History { .. });
        let new_is_history = matches!(new_view, ActiveView::History { .. });

        let current_is_config = matches!(self.active_view, ActiveView::Configuration);
        let new_is_config = matches!(new_view, ActiveView::Configuration);

        let current_is_overlay = current_is_history || current_is_config;
        let new_is_overlay = new_is_history || new_is_config;

        if current_is_uninitialized || (current_is_overlay && !new_is_overlay) {
            self.active_view = new_view;
        } else if !current_is_overlay && new_is_overlay {
            self.previous_view = Some(std::mem::replace(&mut self.active_view, new_view));
        } else {
            let old_view = std::mem::replace(&mut self.active_view, new_view);
            if !new_is_overlay {
                if let Some(previous) = self.previous_view.take() {
                    self.retain_running_thread(previous, cx);
                }
            }
            self.retain_running_thread(old_view, cx);
        }

        // Subscribe to the active ThreadView's events (e.g. FirstSendRequested)
        // so the panel can intercept the first send for worktree creation.
        // Re-subscribe whenever the ConnectionView changes, since the inner
        // ThreadView may have been replaced (e.g. navigating between threads).
        self._active_view_observation = match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
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
            _ => {
                self._thread_view_subscription = None;
                self._active_thread_focus_subscription = None;
                None
            }
        };

        if let ActiveView::History { history } = &self.active_view {
            if !was_in_agent_history && let History::AgentThreads { view } = history {
                view.update(cx, |view, cx| {
                    view.history()
                        .update(cx, |history, cx| history.refresh_full_history(cx))
                });
            }
        }

        if focus {
            self.focus_handle(cx).focus(window, cx);
        }
        cx.emit(AgentPanelEvent::ActiveViewChanged);
    }

    fn populate_recently_updated_menu_section(
        mut menu: ContextMenu,
        panel: Entity<Self>,
        history: History,
        cx: &mut Context<ContextMenu>,
    ) -> ContextMenu {
        match history {
            History::AgentThreads { view } => {
                let entries = view
                    .read(cx)
                    .history()
                    .read(cx)
                    .sessions()
                    .iter()
                    .take(RECENTLY_UPDATED_MENU_LIMIT)
                    .cloned()
                    .collect::<Vec<_>>();

                if entries.is_empty() {
                    return menu;
                }

                menu = menu.header("Recently Updated");

                for entry in entries {
                    let title = entry
                        .title
                        .as_ref()
                        .filter(|title| !title.is_empty())
                        .cloned()
                        .unwrap_or_else(|| SharedString::new_static(DEFAULT_THREAD_TITLE));

                    menu = menu.entry(title, None, {
                        let panel = panel.downgrade();
                        let entry = entry.clone();
                        move |window, cx| {
                            let entry = entry.clone();
                            panel
                                .update(cx, move |this, cx| {
                                    if let Some(agent) = this.selected_agent() {
                                        this.load_agent_thread(
                                            agent,
                                            entry.session_id.clone(),
                                            entry.work_dirs.clone(),
                                            entry.title.clone(),
                                            true,
                                            window,
                                            cx,
                                        );
                                    }
                                })
                                .ok();
                        }
                    });
                }
            }
            History::TextThreads => {
                let entries = panel
                    .read(cx)
                    .text_thread_store
                    .read(cx)
                    .ordered_text_threads()
                    .take(RECENTLY_UPDATED_MENU_LIMIT)
                    .cloned()
                    .collect::<Vec<_>>();

                if entries.is_empty() {
                    return menu;
                }

                menu = menu.header("Recent Text Threads");

                for entry in entries {
                    let title = if entry.title.is_empty() {
                        SharedString::new_static(DEFAULT_THREAD_TITLE)
                    } else {
                        entry.title.clone()
                    };

                    menu = menu.entry(title, None, {
                        let panel = panel.downgrade();
                        let entry = entry.clone();
                        move |window, cx| {
                            let path = entry.path.clone();
                            panel
                                .update(cx, move |this, cx| {
                                    this.open_saved_text_thread(path.clone(), window, cx)
                                        .detach_and_log_err(cx);
                                })
                                .ok();
                        }
                    });
                }
            }
        }

        menu.separator()
    }

    fn subscribe_to_active_thread_view(
        server_view: &Entity<ConversationView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Subscription> {
        server_view.read(cx).active_thread().cloned().map(|tv| {
            cx.subscribe_in(
                &tv,
                window,
                |this, view, event: &AcpThreadViewEvent, window, cx| match event {
                    AcpThreadViewEvent::FirstSendRequested { content } => {
                        this.handle_first_send_requested(view.clone(), content.clone(), window, cx);
                    }
                    AcpThreadViewEvent::MessageSentOrQueued => {
                        let session_id = view.read(cx).thread.read(cx).session_id().clone();
                        cx.emit(AgentPanelEvent::MessageSentOrQueued { session_id });
                    }
                },
            )
        })
    }

    pub fn start_thread_in(&self) -> &StartThreadIn {
        &self.start_thread_in
    }

    fn set_start_thread_in(
        &mut self,
        action: &StartThreadIn,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(action, StartThreadIn::NewWorktree) && !cx.has_flag::<AgentV2FeatureFlag>() {
            return;
        }

        let new_target = match *action {
            StartThreadIn::LocalProject => StartThreadIn::LocalProject,
            StartThreadIn::NewWorktree => {
                if !self.project_has_git_repository(cx) {
                    log::error!(
                        "set_start_thread_in: cannot use NewWorktree without a git repository"
                    );
                    return;
                }
                if self.project.read(cx).is_via_collab() {
                    log::error!("set_start_thread_in: cannot use NewWorktree in a collab project");
                    return;
                }
                StartThreadIn::NewWorktree
            }
        };
        self.start_thread_in = new_target;
        if let Some(thread) = self.active_thread_view(cx) {
            thread.update(cx, |thread, cx| thread.focus_handle(cx).focus(window, cx));
        }
        self.serialize(cx);
        cx.notify();
    }

    fn cycle_start_thread_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let next = match self.start_thread_in {
            StartThreadIn::LocalProject => StartThreadIn::NewWorktree,
            StartThreadIn::NewWorktree => StartThreadIn::LocalProject,
        };
        self.set_start_thread_in(&next, window, cx);
    }

    fn reset_start_thread_in_to_default(&mut self, cx: &mut Context<Self>) {
        use settings::{NewThreadLocation, Settings};
        let default = AgentSettings::get_global(cx).new_thread_location;
        let start_thread_in = match default {
            NewThreadLocation::LocalProject => StartThreadIn::LocalProject,
            NewThreadLocation::NewWorktree => {
                if self.project_has_git_repository(cx) {
                    StartThreadIn::NewWorktree
                } else {
                    StartThreadIn::LocalProject
                }
            }
        };
        if self.start_thread_in != start_thread_in {
            self.start_thread_in = start_thread_in;
            self.serialize(cx);
            cx.notify();
        }
    }

    pub(crate) fn selected_agent(&self) -> Option<Agent> {
        match &self.selected_agent_type {
            AgentType::NativeAgent => Some(Agent::NativeAgent),
            AgentType::Custom { id } => Some(Agent::Custom { id: id.clone() }),
            AgentType::TextThread => None,
        }
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
            window,
            cx,
        );
    }

    pub fn new_agent_thread(
        &mut self,
        agent: AgentType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.reset_start_thread_in_to_default(cx);
        self.new_agent_thread_inner(agent, true, window, cx);
    }

    fn new_agent_thread_inner(
        &mut self,
        agent: AgentType,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match agent {
            AgentType::TextThread => {
                window.dispatch_action(NewTextThread.boxed_clone(), cx);
            }
            AgentType::NativeAgent => self.external_thread(
                Some(crate::Agent::NativeAgent),
                None,
                None,
                None,
                None,
                focus,
                window,
                cx,
            ),
            AgentType::Custom { id } => self.external_thread(
                Some(crate::Agent::Custom { id }),
                None,
                None,
                None,
                None,
                focus,
                window,
                cx,
            ),
        }
    }

    pub fn load_agent_thread(
        &mut self,
        agent: Agent,
        session_id: acp::SessionId,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(conversation_view) = self.background_threads.remove(&session_id) {
            self.set_active_view(
                ActiveView::AgentThread { conversation_view },
                focus,
                window,
                cx,
            );
            return;
        }

        if let ActiveView::AgentThread { conversation_view } = &self.active_view {
            if conversation_view
                .read(cx)
                .active_thread()
                .map(|t| t.read(cx).id.clone())
                == Some(session_id.clone())
            {
                cx.emit(AgentPanelEvent::ActiveViewChanged);
                return;
            }
        }

        if let Some(ActiveView::AgentThread { conversation_view }) = &self.previous_view {
            if conversation_view
                .read(cx)
                .active_thread()
                .map(|t| t.read(cx).id.clone())
                == Some(session_id.clone())
            {
                let view = self.previous_view.take().unwrap();
                self.set_active_view(view, focus, window, cx);
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
            window,
            cx,
        );
    }

    pub(crate) fn create_agent_thread(
        &mut self,
        server: Rc<dyn AgentServer>,
        resume_session_id: Option<acp::SessionId>,
        work_dirs: Option<PathList>,
        title: Option<SharedString>,
        initial_content: Option<AgentInitialContent>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        ext_agent: Agent,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let selected_agent = AgentType::from(ext_agent.clone());
        if self.selected_agent_type != selected_agent {
            self.selected_agent_type = selected_agent;
            self.serialize(cx);
        }
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
                ext_agent,
                resume_session_id,
                work_dirs,
                title,
                initial_content,
                workspace.clone(),
                project,
                thread_store,
                self.prompt_store.clone(),
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
                cx.emit(AgentPanelEvent::BackgroundThreadChanged);
            }
            cx.notify();
        })
        .detach();

        self.set_active_view(
            ActiveView::AgentThread { conversation_view },
            focus,
            window,
            cx,
        );
    }

    fn active_thread_has_messages(&self, cx: &App) -> bool {
        self.active_agent_thread(cx)
            .is_some_and(|thread| !thread.read(cx).entries().is_empty())
    }

    pub fn active_thread_is_draft(&self, cx: &App) -> bool {
        self.active_conversation_view().is_some() && !self.active_thread_has_messages(cx)
    }

    fn handle_first_send_requested(
        &mut self,
        thread_view: Entity<ThreadView>,
        content: Vec<acp::ContentBlock>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.start_thread_in == StartThreadIn::NewWorktree {
            self.handle_worktree_creation_requested(content, window, cx);
        } else {
            cx.defer_in(window, move |_this, window, cx| {
                thread_view.update(cx, |thread_view, cx| {
                    let editor = thread_view.message_editor.clone();
                    thread_view.send_impl(editor, window, cx);
                });
            });
        }
    }

    // TODO: The mapping from workspace root paths to git repositories needs a
    // unified approach across the codebase: this method, `sidebar::is_root_repo`,
    // thread persistence (which PathList is saved to the database), and thread
    // querying (which PathList is used to read threads back). All of these need
    // to agree on how repos are resolved for a given workspace, especially in
    // multi-root and nested-repo configurations.
    /// Partitions the project's visible worktrees into git-backed repositories
    /// and plain (non-git) paths. Git repos will have worktrees created for
    /// them; non-git paths are carried over to the new workspace as-is.
    ///
    /// When multiple worktrees map to the same repository, the most specific
    /// match wins (deepest work directory path), with a deterministic
    /// tie-break on entity id. Each repository appears at most once.
    fn classify_worktrees(
        &self,
        cx: &App,
    ) -> (Vec<Entity<project::git_store::Repository>>, Vec<PathBuf>) {
        let project = &self.project;
        let repositories = project.read(cx).repositories(cx).clone();
        let mut git_repos: Vec<Entity<project::git_store::Repository>> = Vec::new();
        let mut non_git_paths: Vec<PathBuf> = Vec::new();
        let mut seen_repo_ids = std::collections::HashSet::new();

        for worktree in project.read(cx).visible_worktrees(cx) {
            let wt_path = worktree.read(cx).abs_path();

            let matching_repo = repositories
                .iter()
                .filter_map(|(id, repo)| {
                    let work_dir = repo.read(cx).work_directory_abs_path.clone();
                    if wt_path.starts_with(work_dir.as_ref())
                        || work_dir.starts_with(wt_path.as_ref())
                    {
                        Some((*id, repo.clone(), work_dir.as_ref().components().count()))
                    } else {
                        None
                    }
                })
                .max_by(
                    |(left_id, _left_repo, left_depth), (right_id, _right_repo, right_depth)| {
                        left_depth
                            .cmp(right_depth)
                            .then_with(|| left_id.cmp(right_id))
                    },
                );

            if let Some((id, repo, _)) = matching_repo {
                if seen_repo_ids.insert(id) {
                    git_repos.push(repo);
                }
            } else {
                non_git_paths.push(wt_path.to_path_buf());
            }
        }

        (git_repos, non_git_paths)
    }

    /// Kicks off an async git-worktree creation for each repository. Returns:
    ///
    /// - `creation_infos`: a vec of `(repo, new_path, receiver)` tuples—the
    ///   receiver resolves once the git worktree command finishes.
    /// - `path_remapping`: `(old_work_dir, new_worktree_path)` pairs used
    ///   later to remap open editor tabs into the new workspace.
    fn start_worktree_creations(
        git_repos: &[Entity<project::git_store::Repository>],
        branch_name: &str,
        worktree_directory_setting: &str,
        cx: &mut Context<Self>,
    ) -> Result<(
        Vec<(
            Entity<project::git_store::Repository>,
            PathBuf,
            futures::channel::oneshot::Receiver<Result<()>>,
        )>,
        Vec<(PathBuf, PathBuf)>,
    )> {
        let mut creation_infos = Vec::new();
        let mut path_remapping = Vec::new();

        for repo in git_repos {
            let (work_dir, new_path, receiver) = repo.update(cx, |repo, _cx| {
                let new_path =
                    repo.path_for_new_linked_worktree(branch_name, worktree_directory_setting)?;
                let receiver =
                    repo.create_worktree(branch_name.to_string(), new_path.clone(), None);
                let work_dir = repo.work_directory_abs_path.clone();
                anyhow::Ok((work_dir, new_path, receiver))
            })?;
            path_remapping.push((work_dir.to_path_buf(), new_path.clone()));
            creation_infos.push((repo.clone(), new_path, receiver));
        }

        Ok((creation_infos, path_remapping))
    }

    /// Waits for every in-flight worktree creation to complete. If any
    /// creation fails, all successfully-created worktrees are rolled back
    /// (removed) so the project isn't left in a half-migrated state.
    async fn await_and_rollback_on_failure(
        creation_infos: Vec<(
            Entity<project::git_store::Repository>,
            PathBuf,
            futures::channel::oneshot::Receiver<Result<()>>,
        )>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Vec<PathBuf>> {
        let mut created_paths: Vec<PathBuf> = Vec::new();
        let mut repos_and_paths: Vec<(Entity<project::git_store::Repository>, PathBuf)> =
            Vec::new();
        let mut first_error: Option<anyhow::Error> = None;

        for (repo, new_path, receiver) in creation_infos {
            match receiver.await {
                Ok(Ok(())) => {
                    created_paths.push(new_path.clone());
                    repos_and_paths.push((repo, new_path));
                }
                Ok(Err(err)) => {
                    if first_error.is_none() {
                        first_error = Some(err);
                    }
                }
                Err(_canceled) => {
                    if first_error.is_none() {
                        first_error = Some(anyhow!("Worktree creation was canceled"));
                    }
                }
            }
        }

        let Some(err) = first_error else {
            return Ok(created_paths);
        };

        // Rollback all successfully created worktrees
        let mut rollback_receivers = Vec::new();
        for (rollback_repo, rollback_path) in &repos_and_paths {
            if let Ok(receiver) = cx.update(|_, cx| {
                rollback_repo.update(cx, |repo, _cx| {
                    repo.remove_worktree(rollback_path.clone(), true)
                })
            }) {
                rollback_receivers.push((rollback_path.clone(), receiver));
            }
        }
        let mut rollback_failures: Vec<String> = Vec::new();
        for (path, receiver) in rollback_receivers {
            match receiver.await {
                Ok(Ok(())) => {}
                Ok(Err(rollback_err)) => {
                    log::error!(
                        "failed to rollback worktree at {}: {rollback_err}",
                        path.display()
                    );
                    rollback_failures.push(format!("{}: {rollback_err}", path.display()));
                }
                Err(rollback_err) => {
                    log::error!(
                        "failed to rollback worktree at {}: {rollback_err}",
                        path.display()
                    );
                    rollback_failures.push(format!("{}: {rollback_err}", path.display()));
                }
            }
        }
        let mut error_message = format!("Failed to create worktree: {err}");
        if !rollback_failures.is_empty() {
            error_message.push_str("\n\nFailed to clean up: ");
            error_message.push_str(&rollback_failures.join(", "));
        }
        Err(anyhow!(error_message))
    }

    fn set_worktree_creation_error(
        &mut self,
        message: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.worktree_creation_status = Some(WorktreeCreationStatus::Error(message));
        if matches!(self.active_view, ActiveView::Uninitialized) {
            let selected_agent_type = self.selected_agent_type.clone();
            self.new_agent_thread(selected_agent_type, window, cx);
        }
        cx.notify();
    }

    fn handle_worktree_creation_requested(
        &mut self,
        content: Vec<acp::ContentBlock>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(
            self.worktree_creation_status,
            Some(WorktreeCreationStatus::Creating)
        ) {
            return;
        }

        self.worktree_creation_status = Some(WorktreeCreationStatus::Creating);
        cx.notify();

        let (git_repos, non_git_paths) = self.classify_worktrees(cx);

        if git_repos.is_empty() {
            self.set_worktree_creation_error(
                "No git repositories found in the project".into(),
                window,
                cx,
            );
            return;
        }

        // Kick off branch listing as early as possible so it can run
        // concurrently with the remaining synchronous setup work.
        let branch_receivers: Vec<_> = git_repos
            .iter()
            .map(|repo| repo.update(cx, |repo, _cx| repo.branches()))
            .collect();

        let worktree_directory_setting = ProjectSettings::get_global(cx)
            .git
            .worktree_directory
            .clone();

        let active_file_path = self.workspace.upgrade().and_then(|workspace| {
            let workspace = workspace.read(cx);
            let active_item = workspace.active_item(cx)?;
            let project_path = active_item.project_path(cx)?;
            workspace
                .project()
                .read(cx)
                .absolute_path(&project_path, cx)
        });

        let workspace = self.workspace.clone();
        let window_handle = window
            .window_handle()
            .downcast::<workspace::MultiWorkspace>();

        let selected_agent = self.selected_agent();

        let task = cx.spawn_in(window, async move |this, cx| {
            // Await the branch listings we kicked off earlier.
            let mut existing_branches = Vec::new();
            for result in futures::future::join_all(branch_receivers).await {
                match result {
                    Ok(Ok(branches)) => {
                        for branch in branches {
                            existing_branches.push(branch.name().to_string());
                        }
                    }
                    Ok(Err(err)) => {
                        Err::<(), _>(err).log_err();
                    }
                    Err(_) => {}
                }
            }

            let existing_branch_refs: Vec<&str> =
                existing_branches.iter().map(|s| s.as_str()).collect();
            let mut rng = rand::rng();
            let branch_name =
                match crate::branch_names::generate_branch_name(&existing_branch_refs, &mut rng) {
                    Some(name) => name,
                    None => {
                        this.update_in(cx, |this, window, cx| {
                            this.set_worktree_creation_error(
                                "Failed to generate a unique branch name".into(),
                                window,
                                cx,
                            );
                        })?;
                        return anyhow::Ok(());
                    }
                };

            let (creation_infos, path_remapping) = match this.update_in(cx, |_this, _window, cx| {
                Self::start_worktree_creations(
                    &git_repos,
                    &branch_name,
                    &worktree_directory_setting,
                    cx,
                )
            }) {
                Ok(Ok(result)) => result,
                Ok(Err(err)) | Err(err) => {
                    this.update_in(cx, |this, window, cx| {
                        this.set_worktree_creation_error(
                            format!("Failed to validate worktree directory: {err}").into(),
                            window,
                            cx,
                        );
                    })
                    .log_err();
                    return anyhow::Ok(());
                }
            };

            let created_paths = match Self::await_and_rollback_on_failure(creation_infos, cx).await
            {
                Ok(paths) => paths,
                Err(err) => {
                    this.update_in(cx, |this, window, cx| {
                        this.set_worktree_creation_error(format!("{err}").into(), window, cx);
                    })?;
                    return anyhow::Ok(());
                }
            };

            let mut all_paths = created_paths;
            let has_non_git = !non_git_paths.is_empty();
            all_paths.extend(non_git_paths.iter().cloned());

            let app_state = match workspace.upgrade() {
                Some(workspace) => cx.update(|_, cx| workspace.read(cx).app_state().clone())?,
                None => {
                    this.update_in(cx, |this, window, cx| {
                        this.set_worktree_creation_error(
                            "Workspace no longer available".into(),
                            window,
                            cx,
                        );
                    })?;
                    return anyhow::Ok(());
                }
            };

            let this_for_error = this.clone();
            if let Err(err) = Self::setup_new_workspace(
                this,
                all_paths,
                app_state,
                window_handle,
                active_file_path,
                path_remapping,
                non_git_paths,
                has_non_git,
                content,
                selected_agent,
                cx,
            )
            .await
            {
                this_for_error
                    .update_in(cx, |this, window, cx| {
                        this.set_worktree_creation_error(
                            format!("Failed to set up workspace: {err}").into(),
                            window,
                            cx,
                        );
                    })
                    .log_err();
            }
            anyhow::Ok(())
        });

        self._worktree_creation_task = Some(cx.foreground_executor().spawn(async move {
            task.await.log_err();
        }));
    }

    async fn setup_new_workspace(
        this: WeakEntity<Self>,
        all_paths: Vec<PathBuf>,
        app_state: Arc<workspace::AppState>,
        window_handle: Option<gpui::WindowHandle<workspace::MultiWorkspace>>,
        active_file_path: Option<PathBuf>,
        path_remapping: Vec<(PathBuf, PathBuf)>,
        non_git_paths: Vec<PathBuf>,
        has_non_git: bool,
        content: Vec<acp::ContentBlock>,
        selected_agent: Option<Agent>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let OpenResult {
            window: new_window_handle,
            workspace: new_workspace,
            ..
        } = cx
            .update(|_window, cx| {
                Workspace::new_local(
                    all_paths,
                    app_state,
                    window_handle,
                    None,
                    None,
                    OpenMode::Add,
                    cx,
                )
            })?
            .await?;

        let panels_task = new_workspace.update(cx, |workspace, _cx| workspace.take_panels_task());

        if let Some(task) = panels_task {
            task.await.log_err();
        }

        new_workspace
            .update(cx, |workspace, cx| {
                workspace.project().read(cx).wait_for_initial_scan(cx)
            })
            .await;

        new_workspace
            .update(cx, |workspace, cx| {
                let repos = workspace
                    .project()
                    .read(cx)
                    .repositories(cx)
                    .values()
                    .cloned()
                    .collect::<Vec<_>>();

                let tasks = repos
                    .into_iter()
                    .map(|repo| repo.update(cx, |repo, _| repo.barrier()));
                futures::future::join_all(tasks)
            })
            .await;

        let initial_content = AgentInitialContent::ContentBlock {
            blocks: content,
            auto_submit: true,
        };

        new_window_handle.update(cx, |_multi_workspace, window, cx| {
            new_workspace.update(cx, |workspace, cx| {
                if has_non_git {
                    let toast_id = workspace::notifications::NotificationId::unique::<AgentPanel>();
                    workspace.show_toast(
                        workspace::Toast::new(
                            toast_id,
                            "Some project folders are not git repositories. \
                             They were included as-is without creating a worktree.",
                        ),
                        cx,
                    );
                }

                // If we had an active buffer, remap its path and reopen it.
                let had_active_file = active_file_path.is_some();
                let remapped_active_path = active_file_path.and_then(|original_path| {
                    let best_match = path_remapping
                        .iter()
                        .filter_map(|(old_root, new_root)| {
                            original_path.strip_prefix(old_root).ok().map(|relative| {
                                (old_root.components().count(), new_root.join(relative))
                            })
                        })
                        .max_by_key(|(depth, _)| *depth);

                    if let Some((_, remapped_path)) = best_match {
                        return Some(remapped_path);
                    }

                    for non_git in &non_git_paths {
                        if original_path.starts_with(non_git) {
                            return Some(original_path);
                        }
                    }
                    None
                });

                if had_active_file && remapped_active_path.is_none() {
                    log::warn!(
                        "Active file could not be remapped to the new worktree; it will not be reopened"
                    );
                }

                if let Some(path) = remapped_active_path {
                    let open_task = workspace.open_paths(
                        vec![path],
                        workspace::OpenOptions::default(),
                        None,
                        window,
                        cx,
                    );
                    cx.spawn(async move |_, _| -> anyhow::Result<()> {
                        for item in open_task.await.into_iter().flatten() {
                            item?;
                        }
                        Ok(())
                    })
                    .detach_and_log_err(cx);
                }

                workspace.focus_panel::<AgentPanel>(window, cx);

                // If no active buffer was open, zoom the agent panel
                // (equivalent to cmd-esc fullscreen behavior).
                // This must happen after focus_panel, which activates
                // and opens the panel in the dock.

                if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel.external_thread(
                            selected_agent,
                            None,
                            None,
                            None,
                            Some(initial_content),
                            true,
                            window,
                            cx,
                        );
                    });
                }
            });
        })?;

        new_window_handle.update(cx, |multi_workspace, window, cx| {
            multi_workspace.activate(new_workspace.clone(), window, cx);
        })?;

        this.update_in(cx, |this, window, cx| {
            this.worktree_creation_status = None;

            if let Some(thread_view) = this.active_thread_view(cx) {
                thread_view.update(cx, |thread_view, cx| {
                    thread_view
                        .message_editor
                        .update(cx, |editor, cx| editor.clear(window, cx));
                });
            }

            cx.notify();
        })?;

        anyhow::Ok(())
    }
}

impl Focusable for AgentPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.active_view {
            ActiveView::Uninitialized => self.focus_handle.clone(),
            ActiveView::AgentThread {
                conversation_view, ..
            } => conversation_view.focus_handle(cx),
            ActiveView::History { history: kind } => match kind {
                History::AgentThreads { view } => view.read(cx).focus_handle(cx),
                History::TextThreads => self.text_thread_history.focus_handle(cx),
            },
            ActiveView::TextThread {
                text_thread_editor, ..
            } => text_thread_editor.focus_handle(cx),
            ActiveView::Configuration => {
                if let Some(configuration) = self.configuration.as_ref() {
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
    BackgroundThreadChanged,
    MessageSentOrQueued { session_id: acp::SessionId },
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
        if active
            && matches!(self.active_view, ActiveView::Uninitialized)
            && !matches!(
                self.worktree_creation_status,
                Some(WorktreeCreationStatus::Creating)
            )
        {
            let selected_agent_type = self.selected_agent_type.clone();
            self.new_agent_thread_inner(selected_agent_type, false, window, cx);
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
    fn render_title_view(&self, _window: &mut Window, cx: &Context<Self>) -> AnyElement {
        const LOADING_SUMMARY_PLACEHOLDER: &str = "Loading Summary…";

        let content = match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
                let server_view_ref = conversation_view.read(cx);
                let is_generating_title = server_view_ref.as_native_thread(cx).is_some()
                    && server_view_ref.root_thread(cx).map_or(false, |tv| {
                        tv.read(cx).thread.read(cx).has_provisional_title()
                    });

                if let Some(title_editor) = server_view_ref
                    .root_thread(cx)
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
                        div()
                            .w_full()
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
                            .child(title_editor)
                            .into_any_element()
                    }
                } else {
                    Label::new(conversation_view.read(cx).title(cx))
                        .color(Color::Muted)
                        .truncate()
                        .into_any_element()
                }
            }
            ActiveView::TextThread {
                title_editor,
                text_thread_editor,
                ..
            } => {
                let summary = text_thread_editor.read(cx).text_thread().read(cx).summary();

                match summary {
                    TextThreadSummary::Pending => Label::new(TextThreadSummary::DEFAULT)
                        .color(Color::Muted)
                        .truncate()
                        .into_any_element(),
                    TextThreadSummary::Content(summary) => {
                        if summary.done {
                            div()
                                .w_full()
                                .child(title_editor.clone())
                                .into_any_element()
                        } else {
                            Label::new(LOADING_SUMMARY_PLACEHOLDER)
                                .truncate()
                                .color(Color::Muted)
                                .with_animation(
                                    "generating_title",
                                    Animation::new(Duration::from_secs(2))
                                        .repeat()
                                        .with_easing(pulsating_between(0.4, 0.8)),
                                    |label, delta| label.alpha(delta),
                                )
                                .into_any_element()
                        }
                    }
                    TextThreadSummary::Error => h_flex()
                        .w_full()
                        .child(title_editor.clone())
                        .child(
                            IconButton::new("retry-summary-generation", IconName::RotateCcw)
                                .icon_size(IconSize::Small)
                                .on_click({
                                    let text_thread_editor = text_thread_editor.clone();
                                    move |_, _window, cx| {
                                        text_thread_editor.update(cx, |text_thread_editor, cx| {
                                            text_thread_editor.regenerate_summary(cx);
                                        });
                                    }
                                })
                                .tooltip(move |_window, cx| {
                                    cx.new(|_| {
                                        Tooltip::new("Failed to generate title")
                                            .meta("Click to try again")
                                    })
                                    .into()
                                }),
                        )
                        .into_any_element(),
                }
            }
            ActiveView::History { history: kind } => {
                let title = match kind {
                    History::AgentThreads { .. } => "History",
                    History::TextThreads => "Text Thread History",
                };
                Label::new(title).truncate().into_any_element()
            }
            ActiveView::Configuration => Label::new("Settings").truncate().into_any_element(),
            ActiveView::Uninitialized => Label::new("Agent").truncate().into_any_element(),
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
                    thread.generate_title(cx);
                });
            }
        });
    }

    fn handle_regenerate_text_thread_title(
        text_thread_editor: Entity<TextThreadEditor>,
        cx: &mut App,
    ) {
        text_thread_editor.update(cx, |text_thread_editor, cx| {
            text_thread_editor.regenerate_summary(cx);
        });
    }

    fn render_panel_options_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        let full_screen_label = if self.is_zoomed(window, cx) {
            "Disable Full Screen"
        } else {
            "Enable Full Screen"
        };

        let text_thread_view = match &self.active_view {
            ActiveView::TextThread {
                text_thread_editor, ..
            } => Some(text_thread_editor.clone()),
            _ => None,
        };
        let text_thread_with_messages = match &self.active_view {
            ActiveView::TextThread {
                text_thread_editor, ..
            } => text_thread_editor
                .read(cx)
                .text_thread()
                .read(cx)
                .messages(cx)
                .any(|message| message.role == language_model::Role::Assistant),
            _ => false,
        };

        let conversation_view = match &self.active_view {
            ActiveView::AgentThread { conversation_view } => Some(conversation_view.clone()),
            _ => None,
        };
        let thread_with_messages = match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
                conversation_view.read(cx).has_user_submitted_prompt(cx)
            }
            _ => false,
        };
        let has_auth_methods = match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
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
            .anchor(Corner::TopRight)
            .with_handle(self.agent_panel_menu_handle.clone())
            .menu({
                move |window, cx| {
                    Some(ContextMenu::build(window, cx, |mut menu, _window, _| {
                        menu = menu.context(focus_handle.clone());

                        if thread_with_messages | text_thread_with_messages {
                            menu = menu.header("Current Thread");

                            if let Some(text_thread_view) = text_thread_view.as_ref() {
                                menu = menu
                                    .entry("Regenerate Thread Title", None, {
                                        let text_thread_view = text_thread_view.clone();
                                        move |_, cx| {
                                            Self::handle_regenerate_text_thread_title(
                                                text_thread_view.clone(),
                                                cx,
                                            );
                                        }
                                    })
                                    .separator();
                            }

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
                            .action("Toggle Threads Sidebar", Box::new(ToggleWorkspaceSidebar))
                            .action(full_screen_label, Box::new(ToggleZoom));

                        if has_auth_methods {
                            menu = menu.action("Reauthenticate", Box::new(ReauthenticateAgent))
                        }

                        menu
                    }))
                }
            })
    }

    fn render_recent_entries_menu(
        &self,
        icon: IconName,
        corner: Corner,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        PopoverMenu::new("agent-nav-menu")
            .trigger_with_tooltip(
                IconButton::new("agent-nav-menu", icon).icon_size(IconSize::Small),
                {
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            "Toggle Recently Updated Threads",
                            &ToggleNavigationMenu,
                            &focus_handle,
                            cx,
                        )
                    }
                },
            )
            .anchor(corner)
            .with_handle(self.agent_navigation_menu_handle.clone())
            .menu({
                let menu = self.agent_navigation_menu.clone();
                move |window, cx| {
                    telemetry::event!("View Thread History Clicked");

                    if let Some(menu) = menu.as_ref() {
                        menu.update(cx, |_, cx| {
                            cx.defer_in(window, |menu, window, cx| {
                                menu.rebuild(window, cx);
                            });
                        })
                    }
                    menu.clone()
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

    fn project_has_git_repository(&self, cx: &App) -> bool {
        !self.project.read(cx).repositories(cx).is_empty()
    }

    fn render_start_thread_in_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        use settings::{NewThreadLocation, Settings};

        let focus_handle = self.focus_handle(cx);
        let has_git_repo = self.project_has_git_repository(cx);
        let is_via_collab = self.project.read(cx).is_via_collab();
        let fs = self.fs.clone();

        let is_creating = matches!(
            self.worktree_creation_status,
            Some(WorktreeCreationStatus::Creating)
        );

        let current_target = self.start_thread_in;
        let trigger_label = self.start_thread_in.label();

        let new_thread_location = AgentSettings::get_global(cx).new_thread_location;
        let is_local_default = new_thread_location == NewThreadLocation::LocalProject;
        let is_new_worktree_default = new_thread_location == NewThreadLocation::NewWorktree;

        let icon = if self.start_thread_in_menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let trigger_button = Button::new("thread-target-trigger", trigger_label)
            .end_icon(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
            .disabled(is_creating);

        let dock_position = AgentSettings::get_global(cx).dock;
        let documentation_side = match dock_position {
            settings::DockPosition::Left => DocumentationSide::Right,
            settings::DockPosition::Bottom | settings::DockPosition::Right => {
                DocumentationSide::Left
            }
        };

        PopoverMenu::new("thread-target-selector")
            .trigger_with_tooltip(trigger_button, {
                move |_window, cx| {
                    Tooltip::for_action_in(
                        "Start Thread In…",
                        &CycleStartThreadIn,
                        &focus_handle,
                        cx,
                    )
                }
            })
            .menu(move |window, cx| {
                let is_local_selected = current_target == StartThreadIn::LocalProject;
                let is_new_worktree_selected = current_target == StartThreadIn::NewWorktree;
                let fs = fs.clone();

                Some(ContextMenu::build(window, cx, move |menu, _window, _cx| {
                    let new_worktree_disabled = !has_git_repo || is_via_collab;

                    menu.header("Start Thread In…")
                        .item(
                            ContextMenuEntry::new("Current Worktree")
                                .toggleable(IconPosition::End, is_local_selected)
                                .documentation_aside(documentation_side, move |_| {
                                    HoldForDefault::new(is_local_default)
                                        .more_content(false)
                                        .into_any_element()
                                })
                                .handler({
                                    let fs = fs.clone();
                                    move |window, cx| {
                                        if window.modifiers().secondary() {
                                            update_settings_file(fs.clone(), cx, |settings, _| {
                                                settings
                                                    .agent
                                                    .get_or_insert_default()
                                                    .set_new_thread_location(
                                                        NewThreadLocation::LocalProject,
                                                    );
                                            });
                                        }
                                        window.dispatch_action(
                                            Box::new(StartThreadIn::LocalProject),
                                            cx,
                                        );
                                    }
                                }),
                        )
                        .item({
                            let entry = ContextMenuEntry::new("New Git Worktree")
                                .toggleable(IconPosition::End, is_new_worktree_selected)
                                .disabled(new_worktree_disabled)
                                .handler({
                                    let fs = fs.clone();
                                    move |window, cx| {
                                        if window.modifiers().secondary() {
                                            update_settings_file(fs.clone(), cx, |settings, _| {
                                                settings
                                                    .agent
                                                    .get_or_insert_default()
                                                    .set_new_thread_location(
                                                        NewThreadLocation::NewWorktree,
                                                    );
                                            });
                                        }
                                        window.dispatch_action(
                                            Box::new(StartThreadIn::NewWorktree),
                                            cx,
                                        );
                                    }
                                });

                            if new_worktree_disabled {
                                entry.documentation_aside(documentation_side, move |_| {
                                    let reason = if !has_git_repo {
                                        "No git repository found in this project."
                                    } else {
                                        "Not available for remote/collab projects yet."
                                    };
                                    Label::new(reason)
                                        .color(Color::Muted)
                                        .size(LabelSize::Small)
                                        .into_any_element()
                                })
                            } else {
                                entry.documentation_aside(documentation_side, move |_| {
                                    HoldForDefault::new(is_new_worktree_default)
                                        .more_content(false)
                                        .into_any_element()
                                })
                            }
                        })
                }))
            })
            .with_handle(self.start_thread_in_menu_handle.clone())
            .anchor(Corner::TopLeft)
            .offset(gpui::Point {
                x: px(1.0),
                y: px(1.0),
            })
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_server_store = self.project.read(cx).agent_server_store().clone();
        let has_visible_worktrees = self.project.read(cx).visible_worktrees(cx).next().is_some();
        let focus_handle = self.focus_handle(cx);

        let (selected_agent_custom_icon, selected_agent_label) =
            if let AgentType::Custom { id, .. } = &self.selected_agent_type {
                let store = agent_server_store.read(cx);
                let icon = store.agent_icon(&id);

                let label = store
                    .agent_display_name(&id)
                    .unwrap_or_else(|| self.selected_agent_type.label());
                (icon, label)
            } else {
                (None, self.selected_agent_type.label())
            };

        let active_thread = match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
                conversation_view.read(cx).as_native_thread(cx)
            }
            ActiveView::Uninitialized
            | ActiveView::TextThread { .. }
            | ActiveView::History { .. }
            | ActiveView::Configuration => None,
        };

        let new_thread_menu_builder: Rc<
            dyn Fn(&mut Window, &mut App) -> Option<Entity<ContextMenu>>,
        > = {
            let selected_agent = self.selected_agent_type.clone();
            let is_agent_selected = move |agent_type: AgentType| selected_agent == agent_type;

            let workspace = self.workspace.clone();
            let is_via_collab = workspace
                .update(cx, |workspace, cx| {
                    workspace.project().read(cx).is_via_collab()
                })
                .unwrap_or_default();

            let focus_handle = focus_handle.clone();
            let agent_server_store = agent_server_store;

            Rc::new(move |window, cx| {
                telemetry::event!("New Thread Clicked");

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
                                    is_agent_selected(AgentType::NativeAgent)
                                        | is_agent_selected(AgentType::TextThread),
                                    |this| {
                                        this.action(Box::new(NewExternalAgentThread {
                                            agent: None,
                                        }))
                                    },
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
                                                        panel.new_agent_thread(
                                                            AgentType::NativeAgent,
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
                        .item(
                            ContextMenuEntry::new("Text Thread")
                                .action(NewTextThread.boxed_clone())
                                .icon(IconName::TextThread)
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
                                                        panel.new_agent_thread(
                                                            AgentType::TextThread,
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
                                        is_agent_selected(AgentType::Custom {
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
                                                            panel.new_agent_thread(
                                                                AgentType::Custom {
                                                                    id: agent_id.clone(),
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
        let selected_agent_builtin_icon = self.selected_agent_type.icon();
        let selected_agent_label_for_tooltip = selected_agent_label.clone();

        let selected_agent = div()
            .id("selected_agent_icon")
            .when_some(selected_agent_custom_icon, |this, icon_path| {
                this.px_1()
                    .child(Icon::from_external_svg(icon_path).color(Color::Muted))
            })
            .when(!has_custom_icon, |this| {
                this.when_some(self.selected_agent_type.icon(), |this, icon| {
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

        let show_history_menu = self.has_history_for_selected_agent(cx);
        let has_v2_flag = cx.has_flag::<AgentV2FeatureFlag>();
        let is_empty_state = !self.active_thread_has_messages(cx);

        let is_in_history_or_config = matches!(
            &self.active_view,
            ActiveView::History { .. } | ActiveView::Configuration
        );

        let is_text_thread = matches!(&self.active_view, ActiveView::TextThread { .. });

        let is_full_screen = self.is_zoomed(window, cx);

        let use_v2_empty_toolbar =
            has_v2_flag && is_empty_state && !is_in_history_or_config && !is_text_thread;

        let base_container = h_flex()
            .id("agent-panel-toolbar")
            .h(Tab::container_height(cx))
            .max_w_full()
            .flex_none()
            .justify_between()
            .gap_2()
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border);

        if use_v2_empty_toolbar {
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
                .anchor(Corner::TopLeft)
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
                        .child(agent_selector_menu)
                        .when(
                            has_visible_worktrees && self.project_has_git_repository(cx),
                            |this| this.child(self.render_start_thread_in_selector(cx)),
                        ),
                )
                .child(
                    h_flex()
                        .h_full()
                        .flex_none()
                        .gap_1()
                        .pl_1()
                        .pr_1()
                        .when(show_history_menu && !has_v2_flag, |this| {
                            this.child(self.render_recent_entries_menu(
                                IconName::MenuAltTemp,
                                Corner::TopRight,
                                cx,
                            ))
                        })
                        .when(is_full_screen, |this| {
                            this.child(
                                IconButton::new("disable-full-screen", IconName::Minimize)
                                    .icon_size(IconSize::Small)
                                    .tooltip(move |_, cx| {
                                        Tooltip::for_action("Disable Full Screen", &ToggleZoom, cx)
                                    })
                                    .on_click({
                                        cx.listener(move |_, _, window, cx| {
                                            window.dispatch_action(ToggleZoom.boxed_clone(), cx);
                                        })
                                    }),
                            )
                        })
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
                .anchor(Corner::TopRight)
                .with_handle(self.new_thread_menu_handle.clone())
                .menu(move |window, cx| new_thread_menu_builder(window, cx));

            base_container
                .child(
                    h_flex()
                        .size_full()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .pl(DynamicSpacing::Base04.rems(cx))
                        .child(match &self.active_view {
                            ActiveView::History { .. } | ActiveView::Configuration => {
                                self.render_toolbar_back_button(cx).into_any_element()
                            }
                            _ => selected_agent.into_any_element(),
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
                        .when(show_history_menu && !has_v2_flag, |this| {
                            this.child(self.render_recent_entries_menu(
                                IconName::MenuAltTemp,
                                Corner::TopRight,
                                cx,
                            ))
                        })
                        .when(is_full_screen, |this| {
                            this.child(
                                IconButton::new("disable-full-screen", IconName::Minimize)
                                    .icon_size(IconSize::Small)
                                    .tooltip(move |_, cx| {
                                        Tooltip::for_action("Disable Full Screen", &ToggleZoom, cx)
                                    })
                                    .on_click({
                                        cx.listener(move |_, _, window, cx| {
                                            window.dispatch_action(ToggleZoom.boxed_clone(), cx);
                                        })
                                    }),
                            )
                        })
                        .child(self.render_panel_options_menu(window, cx)),
                )
                .into_any_element()
        }
    }

    fn render_worktree_creation_status(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let status = self.worktree_creation_status.as_ref()?;
        match status {
            WorktreeCreationStatus::Creating => Some(
                h_flex()
                    .absolute()
                    .bottom_12()
                    .w_full()
                    .p_2()
                    .gap_1()
                    .justify_center()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        Icon::new(IconName::LoadCircle)
                            .size(IconSize::Small)
                            .color(Color::Muted)
                            .with_rotate_animation(3),
                    )
                    .child(
                        Label::new("Creating Worktree…")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .into_any_element(),
            ),
            WorktreeCreationStatus::Error(message) => Some(
                Callout::new()
                    .icon(IconName::Warning)
                    .severity(Severity::Warning)
                    .title(message.clone())
                    .into_any_element(),
            ),
        }
    }

    fn should_render_trial_end_upsell(&self, cx: &mut Context<Self>) -> bool {
        if TrialEndUpsell::dismissed(cx) {
            return false;
        }

        match &self.active_view {
            ActiveView::TextThread { .. } => {
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
            ActiveView::Uninitialized
            | ActiveView::AgentThread { .. }
            | ActiveView::History { .. }
            | ActiveView::Configuration => return false,
        }

        let plan = self.user_store.read(cx).plan();
        let has_previous_trial = self.user_store.read(cx).trial_started_at().is_some();

        plan.is_some_and(|plan| plan == Plan::ZedFree) && has_previous_trial
    }

    fn should_render_onboarding(&self, cx: &mut Context<Self>) -> bool {
        if self.on_boarding_upsell_dismissed.load(Ordering::Acquire) {
            return false;
        }

        let user_store = self.user_store.read(cx);

        if user_store.plan().is_some_and(|plan| plan == Plan::ZedPro)
            && user_store
                .subscription_period()
                .and_then(|period| period.0.checked_add_days(chrono::Days::new(1)))
                .is_some_and(|date| date < chrono::Utc::now())
        {
            OnboardingUpsell::set_dismissed(true, cx);
            self.on_boarding_upsell_dismissed
                .store(true, Ordering::Release);
            return false;
        }

        let has_configured_non_zed_providers = LanguageModelRegistry::read_global(cx)
            .visible_providers()
            .iter()
            .any(|provider| {
                provider.is_authenticated(cx)
                    && provider.id() != language_model::ZED_CLOUD_PROVIDER_ID
            });

        match &self.active_view {
            ActiveView::Uninitialized | ActiveView::History { .. } | ActiveView::Configuration => {
                false
            }
            ActiveView::AgentThread {
                conversation_view, ..
            } if conversation_view.read(cx).as_native_thread(cx).is_none() => false,
            ActiveView::AgentThread { conversation_view } => {
                let history_is_empty = conversation_view
                    .read(cx)
                    .history()
                    .is_none_or(|h| h.read(cx).is_empty());
                history_is_empty || !has_configured_non_zed_providers
            }
            ActiveView::TextThread { .. } => {
                let history_is_empty = self.text_thread_history.read(cx).is_empty();
                history_is_empty || !has_configured_non_zed_providers
            }
        }
    }

    fn render_onboarding(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_onboarding(cx) {
            return None;
        }

        let text_thread_view = matches!(&self.active_view, ActiveView::TextThread { .. });

        Some(
            div()
                .when(text_thread_view, |this| {
                    this.bg(cx.theme().colors().editor_background)
                })
                .child(self.onboarding.clone()),
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

    fn emit_configuration_error_telemetry_if_needed(
        &mut self,
        configuration_error: Option<&ConfigurationError>,
    ) {
        let error_kind = configuration_error.map(|err| match err {
            ConfigurationError::NoProvider => "no_provider",
            ConfigurationError::ModelNotFound => "model_not_found",
            ConfigurationError::ProviderNotAuthenticated(_) => "provider_not_authenticated",
        });

        let error_kind_string = error_kind.map(String::from);

        if self.last_configuration_error_telemetry == error_kind_string {
            return;
        }

        self.last_configuration_error_telemetry = error_kind_string;

        if let Some(kind) = error_kind {
            let message = configuration_error
                .map(|err| err.to_string())
                .unwrap_or_default();

            telemetry::event!("Agent Panel Error Shown", kind = kind, message = message,);
        }
    }

    fn render_configuration_error(
        &self,
        border_bottom: bool,
        configuration_error: &ConfigurationError,
        focus_handle: &FocusHandle,
        cx: &mut App,
    ) -> impl IntoElement {
        let zed_provider_configured = AgentSettings::get_global(cx)
            .default_model
            .as_ref()
            .is_some_and(|selection| selection.provider.0.as_str() == "zed.dev");

        let callout = if zed_provider_configured {
            Callout::new()
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .when(border_bottom, |this| {
                    this.border_position(ui::BorderPosition::Bottom)
                })
                .title("Sign in to continue using Zed as your LLM provider.")
                .actions_slot(
                    Button::new("sign_in", "Sign In")
                        .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                        .label_size(LabelSize::Small)
                        .on_click({
                            let workspace = self.workspace.clone();
                            move |_, _, cx| {
                                let Ok(client) =
                                    workspace.update(cx, |workspace, _| workspace.client().clone())
                                else {
                                    return;
                                };

                                cx.spawn(async move |cx| {
                                    client.sign_in_with_optional_connect(true, cx).await
                                })
                                .detach_and_log_err(cx);
                            }
                        }),
                )
        } else {
            Callout::new()
                .icon(IconName::Warning)
                .severity(Severity::Warning)
                .when(border_bottom, |this| {
                    this.border_position(ui::BorderPosition::Bottom)
                })
                .title(configuration_error.to_string())
                .actions_slot(
                    Button::new("settings", "Configure")
                        .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                        .label_size(LabelSize::Small)
                        .key_binding(
                            KeyBinding::for_action_in(&OpenSettings, focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_event, window, cx| {
                            window.dispatch_action(OpenSettings.boxed_clone(), cx)
                        }),
                )
        };

        match configuration_error {
            ConfigurationError::ModelNotFound
            | ConfigurationError::ProviderNotAuthenticated(_)
            | ConfigurationError::NoProvider => callout.into_any_element(),
        }
    }

    fn render_text_thread(
        &self,
        text_thread_editor: &Entity<TextThreadEditor>,
        buffer_search_bar: &Entity<BufferSearchBar>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let mut registrar = buffer_search::DivRegistrar::new(
            |this, _, _cx| match &this.active_view {
                ActiveView::TextThread {
                    buffer_search_bar, ..
                } => Some(buffer_search_bar.clone()),
                _ => None,
            },
            cx,
        );
        BufferSearchBar::register(&mut registrar);
        registrar
            .into_div()
            .size_full()
            .relative()
            .map(|parent| {
                buffer_search_bar.update(cx, |buffer_search_bar, cx| {
                    if buffer_search_bar.is_dismissed() {
                        return parent;
                    }
                    parent.child(
                        div()
                            .p(DynamicSpacing::Base08.rems(cx))
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .bg(cx.theme().colors().editor_background)
                            .child(buffer_search_bar.render(window, cx)),
                    )
                })
            })
            .child(text_thread_editor.clone())
            .child(self.render_drag_target(cx))
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
        match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
                conversation_view.update(cx, |conversation_view, cx| {
                    conversation_view.insert_dragged_files(paths, added_worktrees, window, cx);
                });
            }
            ActiveView::TextThread {
                text_thread_editor, ..
            } => {
                text_thread_editor.update(cx, |text_thread_editor, cx| {
                    TextThreadEditor::insert_dragged_files(
                        text_thread_editor,
                        paths,
                        added_worktrees,
                        window,
                        cx,
                    );
                });
            }
            ActiveView::Uninitialized | ActiveView::History { .. } | ActiveView::Configuration => {}
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
        match &self.active_view {
            ActiveView::AgentThread { .. } => key_context.add("acp_thread"),
            ActiveView::TextThread { .. } => key_context.add("text_thread"),
            ActiveView::Uninitialized | ActiveView::History { .. } | ActiveView::Configuration => {}
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
            .on_action(cx.listener(|this, _: &OpenHistory, window, cx| {
                this.open_history(window, cx);
            }))
            .on_action(cx.listener(|this, _: &OpenSettings, window, cx| {
                this.open_configuration(window, cx);
            }))
            .on_action(cx.listener(Self::open_active_thread_as_markdown))
            .on_action(cx.listener(Self::deploy_rules_library))
            .on_action(cx.listener(Self::go_back))
            .on_action(cx.listener(Self::toggle_navigation_menu))
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
            .children(self.render_onboarding(window, cx))
            .map(|parent| {
                // Emit configuration error telemetry before entering the match to avoid borrow conflicts
                if matches!(&self.active_view, ActiveView::TextThread { .. }) {
                    let model_registry = LanguageModelRegistry::read_global(cx);
                    let configuration_error =
                        model_registry.configuration_error(model_registry.default_model(), cx);
                    self.emit_configuration_error_telemetry_if_needed(configuration_error.as_ref());
                }

                match &self.active_view {
                    ActiveView::Uninitialized => parent,
                    ActiveView::AgentThread {
                        conversation_view, ..
                    } => parent
                        .child(conversation_view.clone())
                        .child(self.render_drag_target(cx)),
                    ActiveView::History { history: kind } => match kind {
                        History::AgentThreads { view } => parent.child(view.clone()),
                        History::TextThreads => parent.child(self.text_thread_history.clone()),
                    },
                    ActiveView::TextThread {
                        text_thread_editor,
                        buffer_search_bar,
                        ..
                    } => {
                        let model_registry = LanguageModelRegistry::read_global(cx);
                        let configuration_error =
                            model_registry.configuration_error(model_registry.default_model(), cx);

                        parent
                            .map(|this| {
                                if !self.should_render_onboarding(cx)
                                    && let Some(err) = configuration_error.as_ref()
                                {
                                    this.child(self.render_configuration_error(
                                        true,
                                        err,
                                        &self.focus_handle(cx),
                                        cx,
                                    ))
                                } else {
                                    this
                                }
                            })
                            .child(self.render_text_thread(
                                text_thread_editor,
                                buffer_search_bar,
                                window,
                                cx,
                            ))
                    }
                    ActiveView::Configuration => parent.children(self.configuration.clone()),
                }
            })
            .children(self.render_worktree_creation_status(cx))
            .children(self.render_trial_end_upsell(window, cx));

        match self.active_view.which_font_size_used() {
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
            let history = panel
                .read(cx)
                .connection_store()
                .read(cx)
                .entry(&crate::Agent::NativeAgent)
                .and_then(|s| s.read(cx).history())
                .map(|h| h.downgrade());
            let project = workspace.read(cx).project().downgrade();
            let panel = panel.read(cx);
            let thread_store = panel.thread_store().clone();
            assistant.assist(
                prompt_editor,
                self.workspace.clone(),
                project,
                thread_store,
                None,
                history,
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

pub struct ConcreteAssistantPanelDelegate;

impl AgentPanelDelegate for ConcreteAssistantPanelDelegate {
    fn active_text_thread_editor(
        &self,
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<TextThreadEditor>> {
        let panel = workspace.panel::<AgentPanel>(cx)?;
        panel.read(cx).active_text_thread_editor()
    }

    fn open_local_text_thread(
        &self,
        workspace: &mut Workspace,
        path: Arc<Path>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Result<()>> {
        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
            return Task::ready(Err(anyhow!("Agent panel not found")));
        };

        panel.update(cx, |panel, cx| {
            panel.open_saved_text_thread(path, window, cx)
        })
    }

    fn open_remote_text_thread(
        &self,
        _workspace: &mut Workspace,
        _text_thread_id: assistant_text_thread::TextThreadId,
        _window: &mut Window,
        _cx: &mut Context<Workspace>,
    ) -> Task<Result<Entity<TextThreadEditor>>> {
        Task::ready(Err(anyhow!("opening remote context not implemented")))
    }

    fn quote_selection(
        &self,
        workspace: &mut Workspace,
        selection_ranges: Vec<Range<Anchor>>,
        buffer: Entity<MultiBuffer>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
            return;
        };

        if !panel.focus_handle(cx).contains_focused(window, cx) {
            workspace.toggle_panel_focus::<AgentPanel>(window, cx);
        }

        panel.update(cx, |_, cx| {
            // Wait to create a new context until the workspace is no longer
            // being updated.
            cx.defer_in(window, move |panel, window, cx| {
                if let Some(conversation_view) = panel.active_conversation_view() {
                    conversation_view.update(cx, |conversation_view, cx| {
                        conversation_view.insert_selections(window, cx);
                    });
                } else if let Some(text_thread_editor) = panel.active_text_thread_editor() {
                    let snapshot = buffer.read(cx).snapshot(cx);
                    let selection_ranges = selection_ranges
                        .into_iter()
                        .map(|range| range.to_point(&snapshot))
                        .collect::<Vec<_>>();

                    text_thread_editor.update(cx, |text_thread_editor, cx| {
                        text_thread_editor.quote_ranges(selection_ranges, snapshot, window, cx)
                    });
                }
            });
        });
    }

    fn quote_terminal_text(
        &self,
        workspace: &mut Workspace,
        text: String,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
            return;
        };

        if !panel.focus_handle(cx).contains_focused(window, cx) {
            workspace.toggle_panel_focus::<AgentPanel>(window, cx);
        }

        panel.update(cx, |_, cx| {
            // Wait to create a new context until the workspace is no longer
            // being updated.
            cx.defer_in(window, move |panel, window, cx| {
                if let Some(conversation_view) = panel.active_conversation_view() {
                    conversation_view.update(cx, |conversation_view, cx| {
                        conversation_view.insert_terminal_text(text, window, cx);
                    });
                } else if let Some(text_thread_editor) = panel.active_text_thread_editor() {
                    text_thread_editor.update(cx, |text_thread_editor, cx| {
                        text_thread_editor.quote_terminal_text(text, window, cx)
                    });
                }
            });
        });
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
    pub fn test_new(
        workspace: &Workspace,
        text_thread_store: Entity<assistant_text_thread::TextThreadStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(workspace, text_thread_store, None, window, cx)
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
        let workspace = self.workspace.clone();
        let project = self.project.clone();

        let ext_agent = Agent::Custom {
            id: server.agent_id(),
        };

        self.create_agent_thread(
            server, None, None, None, None, workspace, project, ext_agent, true, window, cx,
        );
    }

    /// Returns the currently active thread view, if any.
    ///
    /// This is a test-only accessor that exposes the private `active_thread_view()`
    /// method for test assertions. Not compiled into production builds.
    pub fn active_thread_view_for_tests(&self) -> Option<&Entity<ConversationView>> {
        self.active_conversation_view()
    }

    /// Sets the start_thread_in value directly, bypassing validation.
    ///
    /// This is a test-only helper for visual tests that need to show specific
    /// start_thread_in states without requiring a real git repository.
    pub fn set_start_thread_in_for_tests(&mut self, target: StartThreadIn, cx: &mut Context<Self>) {
        self.start_thread_in = target;
        cx.notify();
    }

    /// Returns the current worktree creation status.
    ///
    /// This is a test-only helper for visual tests.
    pub fn worktree_creation_status_for_tests(&self) -> Option<&WorktreeCreationStatus> {
        self.worktree_creation_status.as_ref()
    }

    /// Sets the worktree creation status directly.
    ///
    /// This is a test-only helper for visual tests that need to show the
    /// "Creating worktree…" spinner or error banners.
    pub fn set_worktree_creation_status_for_tests(
        &mut self,
        status: Option<WorktreeCreationStatus>,
        cx: &mut Context<Self>,
    ) {
        self.worktree_creation_status = status;
        cx.notify();
    }

    /// Opens the history view.
    ///
    /// This is a test-only helper that exposes the private `open_history()`
    /// method for visual tests.
    pub fn open_history_for_tests(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open_history(window, cx);
    }

    /// Opens the start_thread_in selector popover menu.
    ///
    /// This is a test-only helper for visual tests.
    pub fn open_start_thread_in_menu_for_tests(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_thread_in_menu_handle.show(window, cx);
    }

    /// Dismisses the start_thread_in dropdown menu.
    ///
    /// This is a test-only helper for visual tests.
    pub fn close_start_thread_in_menu_for_tests(&mut self, cx: &mut Context<Self>) {
        self.start_thread_in_menu_handle.hide(cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversation_view::tests::{StubAgentServer, init_test};
    use crate::test_support::{
        active_session_id, open_thread_with_connection, open_thread_with_custom_connection,
        send_message,
    };
    use acp_thread::{StubAgentConnection, ThreadStatus};
    use agent_servers::CODEX_ID;
    use assistant_text_thread::TextThreadStore;
    use feature_flags::FeatureFlagAppExt;
    use fs::FakeFs;
    use gpui::{TestAppContext, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use std::time::Instant;
    use workspace::MultiWorkspace;

    #[gpui::test]
    async fn test_active_thread_serialize_and_load_round_trip(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        // --- Create a MultiWorkspace window with two workspaces ---
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

        // --- Set up workspace A: with an active thread ---
        let panel_a = workspace_a.update_in(cx, |workspace, window, cx| {
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project_a.clone(), cx));
            cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx))
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

        let agent_type_a = panel_a.read_with(cx, |panel, _cx| panel.selected_agent_type.clone());

        // --- Set up workspace B: ClaudeCode, no active thread ---
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project_b.clone(), cx));
            cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx))
        });

        panel_b.update(cx, |panel, _cx| {
            panel.selected_agent_type = AgentType::Custom {
                id: "claude-acp".into(),
            };
        });

        // --- Serialize both panels ---
        panel_a.update(cx, |panel, cx| panel.serialize(cx));
        panel_b.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        // --- Load fresh panels for each workspace and verify independent state ---
        let prompt_builder = Arc::new(prompt_store::PromptBuilder::new(None).unwrap());

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_a = AgentPanel::load(workspace_a.downgrade(), prompt_builder.clone(), async_cx)
            .await
            .expect("panel A load should succeed");
        cx.run_until_parked();

        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_b = AgentPanel::load(workspace_b.downgrade(), prompt_builder.clone(), async_cx)
            .await
            .expect("panel B load should succeed");
        cx.run_until_parked();

        // Workspace A should restore its thread and agent type
        loaded_a.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent_type, agent_type_a,
                "workspace A agent type should be restored"
            );
            assert!(
                panel.active_conversation_view().is_some(),
                "workspace A should have its active thread restored"
            );
        });

        // Workspace B should restore its own agent type, with no thread
        loaded_b.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent_type,
                AgentType::Custom {
                    id: "claude-acp".into()
                },
                "workspace B agent type should be restored"
            );
            assert!(
                panel.active_conversation_view().is_none(),
                "workspace B should have no active thread"
            );
        });
    }

    // Simple regression test
    #[gpui::test]
    async fn test_new_text_thread_action_handler(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
            let slash_command_registry =
                assistant_slash_command::SlashCommandRegistry::default_global(cx);
            slash_command_registry
                .register_command(assistant_slash_commands::DefaultSlashCommand, false);
            <dyn fs::Fs>::set_global(fs.clone(), cx);
        });

        let project = Project::test(fs.clone(), [], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let workspace_a = multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                multi_workspace.workspace().clone()
            })
            .unwrap();

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        workspace_a.update_in(cx, |workspace, window, cx| {
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            let panel =
                cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx));
            workspace.add_panel(panel, window, cx);
        });

        cx.run_until_parked();

        workspace_a.update_in(cx, |_, window, cx| {
            window.dispatch_action(NewTextThread.boxed_clone(), cx);
        });

        cx.run_until_parked();
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
    ) -> acp::SessionId {
        open_thread_with_custom_connection(panel, connection.clone(), cx);
        let session_id = active_session_id(panel, cx);
        send_message(panel, cx);
        cx.update(|_, cx| {
            connection.send_update(
                session_id.clone(),
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new("done".into())),
                cx,
            );
        });
        cx.run_until_parked();
        session_id
    }

    fn open_idle_thread_with_non_loadable_connection(
        panel: &Entity<AgentPanel>,
        connection: &StubAgentConnection,
        cx: &mut VisualTestContext,
    ) -> acp::SessionId {
        open_thread_with_custom_connection(panel, connection.clone(), cx);
        let session_id = active_session_id(panel, cx);

        connection.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("done".into()),
        )]);
        send_message(panel, cx);

        session_id
    }

    async fn setup_panel(cx: &mut TestAppContext) -> (Entity<AgentPanel>, VisualTestContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
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
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx))
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
            assert!(panel.background_threads.is_empty());
        });

        // Open a new thread B — thread A should be retained in background.
        let connection_b = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_b, &mut cx);

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.background_threads.len(),
                1,
                "Running thread A should be retained in background_views"
            );
            assert!(
                panel.background_threads.contains_key(&session_id_a),
                "Background view should be keyed by thread A's session ID"
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
        let session_id_a = active_session_id(&panel, &cx);

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
                panel.background_threads.len(),
                1,
                "Idle non-loadable thread A should be retained in background_views"
            );
            assert!(
                panel.background_threads.contains_key(&session_id_a),
                "Background view should be keyed by thread A's session ID"
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

        let session_id_b = active_session_id(&panel, &cx);

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(panel.background_threads.len(), 1);
            assert!(panel.background_threads.contains_key(&session_id_a));
        });

        // Load thread A back via load_agent_thread — should promote from background.
        panel.update_in(&mut cx, |panel, window, cx| {
            panel.load_agent_thread(
                panel.selected_agent().expect("selected agent must be set"),
                session_id_a.clone(),
                None,
                None,
                true,
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
                !panel.background_threads.contains_key(&session_id_a),
                "Promoted thread A should no longer be in background_views"
            );
            assert!(
                panel.background_threads.contains_key(&session_id_b),
                "Thread B (idle, non-loadable) should remain retained in background_views"
            );
        });
    }

    #[gpui::test]
    async fn test_cleanup_background_threads_keeps_five_most_recent_idle_loadable_threads(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;
        let connection = StubAgentConnection::new()
            .with_supports_load_session(true)
            .with_agent_id("loadable-stub".into())
            .with_telemetry_id("loadable-stub".into());
        let mut session_ids = Vec::new();

        for _ in 0..7 {
            session_ids.push(open_generating_thread_with_loadable_connection(
                &panel,
                &connection,
                &mut cx,
            ));
        }

        let base_time = Instant::now();

        for session_id in session_ids.iter().take(6) {
            connection.end_turn(session_id.clone(), acp::StopReason::EndTurn);
        }
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            for (index, session_id) in session_ids.iter().take(6).enumerate() {
                let conversation_view = panel
                    .background_threads
                    .get(session_id)
                    .expect("background thread should exist")
                    .clone();
                conversation_view.update(cx, |view, cx| {
                    view.set_updated_at(base_time + Duration::from_secs(index as u64), cx);
                });
            }
            panel.cleanup_background_threads(cx);
        });

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.background_threads.len(),
                5,
                "cleanup should keep at most five idle loadable background threads"
            );
            assert!(
                !panel.background_threads.contains_key(&session_ids[0]),
                "oldest idle loadable background thread should be removed"
            );
            for session_id in &session_ids[1..6] {
                assert!(
                    panel.background_threads.contains_key(session_id),
                    "more recent idle loadable background threads should be retained"
                );
            }
            assert!(
                !panel.background_threads.contains_key(&session_ids[6]),
                "the active thread should not also be stored as a background thread"
            );
        });
    }

    #[gpui::test]
    async fn test_cleanup_background_threads_preserves_idle_non_loadable_threads(
        cx: &mut TestAppContext,
    ) {
        let (panel, mut cx) = setup_panel(cx).await;

        let non_loadable_connection = StubAgentConnection::new();
        let non_loadable_session_id = open_idle_thread_with_non_loadable_connection(
            &panel,
            &non_loadable_connection,
            &mut cx,
        );

        let loadable_connection = StubAgentConnection::new()
            .with_supports_load_session(true)
            .with_agent_id("loadable-stub".into())
            .with_telemetry_id("loadable-stub".into());
        let mut loadable_session_ids = Vec::new();

        for _ in 0..7 {
            loadable_session_ids.push(open_generating_thread_with_loadable_connection(
                &panel,
                &loadable_connection,
                &mut cx,
            ));
        }

        let base_time = Instant::now();

        for session_id in loadable_session_ids.iter().take(6) {
            loadable_connection.end_turn(session_id.clone(), acp::StopReason::EndTurn);
        }
        cx.run_until_parked();

        panel.update(&mut cx, |panel, cx| {
            for (index, session_id) in loadable_session_ids.iter().take(6).enumerate() {
                let conversation_view = panel
                    .background_threads
                    .get(session_id)
                    .expect("background thread should exist")
                    .clone();
                conversation_view.update(cx, |view, cx| {
                    view.set_updated_at(base_time + Duration::from_secs(index as u64), cx);
                });
            }
            panel.cleanup_background_threads(cx);
        });

        panel.read_with(&cx, |panel, _cx| {
            assert_eq!(
                panel.background_threads.len(),
                6,
                "cleanup should keep the non-loadable idle thread in addition to five loadable ones"
            );
            assert!(
                panel
                    .background_threads
                    .contains_key(&non_loadable_session_id),
                "idle non-loadable background threads should not be cleanup candidates"
            );
            assert!(
                !panel
                    .background_threads
                    .contains_key(&loadable_session_ids[0]),
                "oldest idle loadable background thread should still be removed"
            );
            for session_id in &loadable_session_ids[1..6] {
                assert!(
                    panel.background_threads.contains_key(session_id),
                    "more recent idle loadable background threads should be retained"
                );
            }
            assert!(
                !panel
                    .background_threads
                    .contains_key(&loadable_session_ids[6]),
                "the active loadable thread should not also be stored as a background thread"
            );
        });
    }

    #[gpui::test]
    async fn test_thread_target_local_project(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));

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

        // Wait for the project to discover the git repository.
        cx.run_until_parked();

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            let panel =
                cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        cx.run_until_parked();

        // Default thread target should be LocalProject.
        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                *panel.start_thread_in(),
                StartThreadIn::LocalProject,
                "default thread target should be LocalProject"
            );
        });

        // Start a new thread with the default LocalProject target.
        // Use StubAgentServer so the thread connects immediately in tests.
        panel.update_in(cx, |panel, window, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });

        cx.run_until_parked();

        // MultiWorkspace should still have exactly one workspace (no worktree created).
        multi_workspace
            .read_with(cx, |multi_workspace, _cx| {
                assert_eq!(
                    multi_workspace.workspaces().len(),
                    1,
                    "LocalProject should not create a new workspace"
                );
            })
            .unwrap();

        // The thread should be active in the panel.
        panel.read_with(cx, |panel, cx| {
            assert!(
                panel.active_agent_thread(cx).is_some(),
                "a thread should be running in the current workspace"
            );
        });

        // The thread target should still be LocalProject (unchanged).
        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                *panel.start_thread_in(),
                StartThreadIn::LocalProject,
                "thread target should remain LocalProject"
            );
        });

        // No worktree creation status should be set.
        panel.read_with(cx, |panel, _cx| {
            assert!(
                panel.worktree_creation_status.is_none(),
                "no worktree creation should have occurred"
            );
        });
    }

    #[gpui::test]
    async fn test_thread_target_serialization_round_trip(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));

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

        // Wait for the project to discover the git repository.
        cx.run_until_parked();

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            let panel =
                cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        cx.run_until_parked();

        // Default should be LocalProject.
        panel.read_with(cx, |panel, _cx| {
            assert_eq!(*panel.start_thread_in(), StartThreadIn::LocalProject);
        });

        // Change thread target to NewWorktree.
        panel.update_in(cx, |panel, window, cx| {
            panel.set_start_thread_in(&StartThreadIn::NewWorktree, window, cx);
        });

        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                *panel.start_thread_in(),
                StartThreadIn::NewWorktree,
                "thread target should be NewWorktree after set_thread_target"
            );
        });

        // Let serialization complete.
        cx.run_until_parked();

        // Load a fresh panel from the serialized data.
        let prompt_builder = Arc::new(prompt_store::PromptBuilder::new(None).unwrap());
        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_panel =
            AgentPanel::load(workspace.downgrade(), prompt_builder.clone(), async_cx)
                .await
                .expect("panel load should succeed");
        cx.run_until_parked();

        loaded_panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                *panel.start_thread_in(),
                StartThreadIn::NewWorktree,
                "thread target should survive serialization round-trip"
            );
        });
    }

    #[gpui::test]
    async fn test_set_active_blocked_during_worktree_creation(cx: &mut TestAppContext) {
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
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;

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
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            let panel =
                cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        cx.run_until_parked();

        // Simulate worktree creation in progress and reset to Uninitialized
        panel.update_in(cx, |panel, window, cx| {
            panel.worktree_creation_status = Some(WorktreeCreationStatus::Creating);
            panel.active_view = ActiveView::Uninitialized;
            Panel::set_active(panel, true, window, cx);
            assert!(
                matches!(panel.active_view, ActiveView::Uninitialized),
                "set_active should not create a thread while worktree is being created"
            );
        });

        // Clear the creation status and use open_external_thread_with_server
        // (which bypasses new_agent_thread) to verify the panel can transition
        // out of Uninitialized. We can't call set_active directly because
        // new_agent_thread requires full agent server infrastructure.
        panel.update_in(cx, |panel, window, cx| {
            panel.worktree_creation_status = None;
            panel.active_view = ActiveView::Uninitialized;
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });

        cx.run_until_parked();

        panel.read_with(cx, |panel, _cx| {
            assert!(
                !matches!(panel.active_view, ActiveView::Uninitialized),
                "panel should transition out of Uninitialized once worktree creation is cleared"
            );
        });
    }

    #[test]
    fn test_deserialize_agent_type_variants() {
        assert_eq!(
            serde_json::from_str::<AgentType>(r#""NativeAgent""#).unwrap(),
            AgentType::NativeAgent,
        );
        assert_eq!(
            serde_json::from_str::<AgentType>(r#""TextThread""#).unwrap(),
            AgentType::TextThread,
        );
        assert_eq!(
            serde_json::from_str::<AgentType>(r#"{"Custom":{"name":"my-agent"}}"#).unwrap(),
            AgentType::Custom {
                id: "my-agent".into(),
            },
        );
    }

    #[gpui::test]
    async fn test_worktree_creation_preserves_selected_agent(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
            agent::ThreadStore::init_global(cx);
            language_model::LanguageModelRegistry::test(cx);

            let app_state = workspace::AppState::test(cx);
            workspace::init(app_state.clone(), cx);
            app_state
        });

        let fs = app_state.fs.as_fake();
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": {
                    "main.rs": "fn main() {}"
                }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));

        let project = Project::test(app_state.fs.clone(), [Path::new("/project")], cx).await;

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

        // Register a callback so new workspaces also get an AgentPanel.
        cx.update(|cx| {
            cx.observe_new(
                |workspace: &mut Workspace,
                 window: Option<&mut Window>,
                 cx: &mut Context<Workspace>| {
                    if let Some(window) = window {
                        let project = workspace.project().clone();
                        let text_thread_store =
                            cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
                        let panel = cx.new(|cx| {
                            AgentPanel::new(workspace, text_thread_store, None, window, cx)
                        });
                        workspace.add_panel(panel, window, cx);
                    }
                },
            )
            .detach();
        });

        let cx = &mut VisualTestContext::from_window(multi_workspace.into(), cx);

        // Wait for the project to discover the git repository.
        cx.run_until_parked();

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            let panel =
                cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        cx.run_until_parked();

        // Open a thread (needed so there's an active thread view).
        panel.update_in(cx, |panel, window, cx| {
            panel.open_external_thread_with_server(
                Rc::new(StubAgentServer::default_response()),
                window,
                cx,
            );
        });

        cx.run_until_parked();

        // Set the selected agent to Codex (a custom agent) and start_thread_in
        // to NewWorktree. We do this AFTER opening the thread because
        // open_external_thread_with_server overrides selected_agent_type.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent_type = AgentType::Custom {
                id: CODEX_ID.into(),
            };
            panel.set_start_thread_in(&StartThreadIn::NewWorktree, window, cx);
        });

        // Verify the panel has the Codex agent selected.
        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent_type,
                AgentType::Custom {
                    id: CODEX_ID.into()
                },
            );
        });

        // Directly call handle_worktree_creation_requested, which is what
        // handle_first_send_requested does when start_thread_in == NewWorktree.
        let content = vec![acp::ContentBlock::Text(acp::TextContent::new(
            "Hello from test",
        ))];
        panel.update_in(cx, |panel, window, cx| {
            panel.handle_worktree_creation_requested(content, window, cx);
        });

        // Let the async worktree creation + workspace setup complete.
        cx.run_until_parked();

        // Find the new workspace's AgentPanel and verify it used the Codex agent.
        let found_codex = multi_workspace
            .read_with(cx, |multi_workspace, cx| {
                // There should be more than one workspace now (the original + the new worktree).
                assert!(
                    multi_workspace.workspaces().len() > 1,
                    "expected a new workspace to have been created, found {}",
                    multi_workspace.workspaces().len(),
                );

                // Check the newest workspace's panel for the correct agent.
                let new_workspace = multi_workspace
                    .workspaces()
                    .iter()
                    .find(|ws| ws.entity_id() != workspace.entity_id())
                    .expect("should find the new workspace");
                let new_panel = new_workspace
                    .read(cx)
                    .panel::<AgentPanel>(cx)
                    .expect("new workspace should have an AgentPanel");

                new_panel.read(cx).selected_agent_type.clone()
            })
            .unwrap();

        assert_eq!(
            found_codex,
            AgentType::Custom {
                id: CODEX_ID.into()
            },
            "the new worktree workspace should use the same agent (Codex) that was selected in the original panel",
        );
    }

    #[gpui::test]
    async fn test_work_dirs_update_when_worktrees_change(cx: &mut TestAppContext) {
        use crate::thread_metadata_store::ThreadMetadataStore;

        init_test(cx);
        cx.update(|cx| {
            cx.update_flags(true, vec!["agent-v2".to_string()]);
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
            let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
            cx.new(|cx| AgentPanel::new(workspace, text_thread_store, None, window, cx))
        });

        // Open thread A and send a message. With empty next_prompt_updates it
        // stays generating, so opening B will move A to background_threads.
        let connection_a = StubAgentConnection::new().with_agent_id("agent-a".into());
        open_thread_with_custom_connection(&panel, connection_a.clone(), &mut cx);
        send_message(&panel, &mut cx);
        let session_id_a = active_session_id(&panel, &cx);

        // Open thread C — thread A (generating) moves to background.
        // Thread C completes immediately (idle), then opening B moves C to background too.
        let connection_c = StubAgentConnection::new().with_agent_id("agent-c".into());
        connection_c.set_next_prompt_updates(vec![acp::SessionUpdate::AgentMessageChunk(
            acp::ContentChunk::new("done".into()),
        )]);
        open_thread_with_custom_connection(&panel, connection_c.clone(), &mut cx);
        send_message(&panel, &mut cx);
        let session_id_c = active_session_id(&panel, &cx);

        // Snapshot thread C's initial work_dirs before adding worktrees.
        let initial_c_paths = panel.read_with(&cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            thread.read(cx).work_dirs().cloned().unwrap()
        });

        // Open thread B — thread C (idle, non-loadable) is retained in background.
        let connection_b = StubAgentConnection::new().with_agent_id("agent-b".into());
        open_thread_with_custom_connection(&panel, connection_b.clone(), &mut cx);
        send_message(&panel, &mut cx);
        let session_id_b = active_session_id(&panel, &cx);

        let metadata_store = cx.update(|_, cx| ThreadMetadataStore::global(cx));

        panel.read_with(&cx, |panel, _cx| {
            assert!(
                panel.background_threads.contains_key(&session_id_a),
                "Thread A should be in background_threads"
            );
            assert!(
                panel.background_threads.contains_key(&session_id_c),
                "Thread C should be in background_threads"
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
            let bg_view = panel.background_threads.get(&session_id_a).unwrap();
            let root_thread = bg_view.read(cx).root_thread(cx).unwrap();
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

        // Verify thread C  was NOT updated.
        let unchanged_c_paths = panel.read_with(&cx, |panel, cx| {
            let bg_view = panel.background_threads.get(&session_id_c).unwrap();
            let root_thread = bg_view.read(cx).root_thread(cx).unwrap();
            root_thread
                .read(cx)
                .thread
                .read(cx)
                .work_dirs()
                .cloned()
                .unwrap()
        });
        assert_eq!(
            unchanged_c_paths, initial_c_paths,
            "Thread C (idle background) work_dirs should not change when worktrees change"
        );

        // Verify the metadata store reflects the new paths for running threads only.
        cx.run_until_parked();
        for (label, session_id) in [("thread B", &session_id_b), ("thread A", &session_id_a)] {
            let metadata_paths = metadata_store.read_with(&cx, |store, _cx| {
                let metadata = store
                    .entry(session_id)
                    .unwrap_or_else(|| panic!("{label} thread metadata should exist"));
                metadata.folder_paths.clone()
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
            let bg_view = panel.background_threads.get(&session_id_a).unwrap();
            let root_thread = bg_view.read(cx).root_thread(cx).unwrap();
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
}
