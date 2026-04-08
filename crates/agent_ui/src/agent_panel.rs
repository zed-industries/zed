use std::{
    path::PathBuf,
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

use zed_actions::agent::{
    AddSelectionToThread, ConflictContent, ReauthenticateAgent, ResolveConflictedFilesWithAgent,
    ResolveConflictsWithAgent, ReviewBranchDiff,
};

use crate::thread_metadata_store::ThreadMetadataStore;
use crate::{
    AddContextServer, AgentDiffPane, ConversationView, CopyThreadToClipboard, CycleStartThreadIn,
    Follow, InlineAssistant, LoadThreadFromClipboard, NewThread, NewWorktreeBranchTarget,
    OpenActiveThreadAsMarkdown, OpenAgentDiff, OpenHistory, ResetTrialEndUpsell, ResetTrialUpsell,
    StartThreadIn, ToggleNavigationMenu, ToggleNewThreadMenu, ToggleOptionsMenu,
    agent_configuration::{AgentConfiguration, AssistantConfigurationEvent},
    conversation_view::{AcpThreadViewEvent, ThreadView},
    thread_branch_picker::ThreadBranchPicker,
    thread_worktree_picker::ThreadWorktreePicker,
    ui::EndTrialUpsell,
};
use crate::{
    Agent, AgentInitialContent, ExternalSourcePrompt, NewExternalAgentThread,
    NewNativeAgentThreadFromSummary,
};
use crate::{DEFAULT_THREAD_TITLE, ui::AcpOnboardingModal};
use crate::{ExpandMessageEditor, ThreadHistoryView};
use crate::{ManageProfiles, ThreadHistoryViewEvent};
use crate::{ThreadHistory, agent_connection_store::AgentConnectionStore};
use agent_settings::{AgentSettings, WindowLayout};
use ai_onboarding::AgentPanelOnboarding;
use anyhow::{Context as _, Result, anyhow};
use client::UserStore;
use cloud_api_types::Plan;
use collections::HashMap;
use editor::Editor;
use extension::ExtensionEvents;
use extension_host::ExtensionStore;
use fs::Fs;
use gpui::{
    Action, Animation, AnimationExt, AnyElement, App, AsyncWindowContext, ClipboardItem, Corner,
    DismissEvent, Entity, EventEmitter, ExternalPaths, FocusHandle, Focusable, KeyContext, Pixels,
    Subscription, Task, UpdateGlobal, WeakEntity, prelude::*, pulsating_between,
};
use language::LanguageRegistry;
use language_model::LanguageModelRegistry;
use project::git_store::{GitStoreEvent, RepositoryEvent};
use project::project_settings::ProjectSettings;
use project::{Project, ProjectPath, Worktree, linked_worktree_short_name};
use prompt_store::{PromptStore, UserPromptId};
use rules_library::{RulesLibrary, open_rules_library};
use settings::TerminalDockPosition;
use settings::{Settings, update_settings_file};
use terminal::terminal_settings::TerminalSettings;
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use theme_settings::ThemeSettings;
use ui::{
    Button, ButtonLike, Callout, CommonAnimationExt, ContextMenu, ContextMenuEntry, PopoverMenu,
    PopoverMenuHandle, Tab, Tooltip, prelude::*, utils::WithRemSize,
};
use util::{ResultExt as _, debug_panic};
use workspace::{
    CollaboratorId, DraggedSelection, DraggedTab, OpenMode, OpenResult, PathList,
    SerializedPathList, ToggleWorkspaceSidebar, ToggleZoom, Workspace, WorkspaceId,
    dock::{DockPosition, Panel, PanelEvent},
};
use zed_actions::{
    DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize,
    agent::{OpenAcpOnboardingModal, OpenSettings, ResetAgentZoom, ResetOnboarding},
    assistant::{OpenRulesLibrary, Toggle, ToggleFocus},
};

const AGENT_PANEL_KEY: &str = "agent_panel";
const RECENTLY_UPDATED_MENU_LIMIT: usize = 6;
const LAST_USED_AGENT_KEY: &str = "agent_panel__last_used_external_agent";

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
    #[serde(default)]
    start_thread_in: Option<StartThreadIn>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SerializedActiveThread {
    session_id: String,
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
                .register_action(|workspace, action: &NewExternalAgentThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        workspace.focus_panel::<AgentPanel>(window, cx);
                        panel.update(cx, |panel, cx| {
                            let initial_content = panel.take_active_draft_initial_content(cx);
                            panel.external_thread(
                                action.agent.clone(),
                                None,
                                None,
                                None,
                                initial_content,
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
                })
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

enum ActiveView {
    Uninitialized,
    AgentThread {
        conversation_view: Entity<ConversationView>,
    },
    History {
        view: Entity<ThreadHistoryView>,
    },
    Configuration,
}

enum WhichFontSize {
    AgentFont,
    None,
}

struct StartThreadInLabel {
    prefix: Option<SharedString>,
    label: SharedString,
    suffix: Option<SharedString>,
}

impl StartThreadIn {
    fn trigger_label(&self, project: &Project, cx: &App) -> StartThreadInLabel {
        match self {
            Self::LocalProject => {
                let suffix = project.active_repository(cx).and_then(|repo| {
                    let repo = repo.read(cx);
                    let work_dir = &repo.original_repo_abs_path;
                    let visible_paths: Vec<_> = project
                        .visible_worktrees(cx)
                        .map(|wt| wt.read(cx).abs_path().to_path_buf())
                        .collect();

                    for linked in repo.linked_worktrees() {
                        if visible_paths.contains(&linked.path) {
                            return Some(SharedString::from(format!(
                                "({})",
                                linked.display_name()
                            )));
                        }
                    }

                    if let Some(name) = linked_worktree_short_name(
                        repo.original_repo_abs_path.as_ref(),
                        repo.work_directory_abs_path.as_ref(),
                    ) {
                        if visible_paths
                            .iter()
                            .any(|p| p.as_path() == repo.work_directory_abs_path.as_ref())
                        {
                            return Some(SharedString::from(format!("({})", name)));
                        }
                    }

                    if visible_paths
                        .iter()
                        .any(|p| p.as_path() == work_dir.as_ref())
                    {
                        return Some("(main)".into());
                    }

                    None
                });

                StartThreadInLabel {
                    prefix: None,
                    label: "Current Worktree".into(),
                    suffix,
                }
            }
            Self::NewWorktree {
                worktree_name: Some(worktree_name),
                ..
            } => StartThreadInLabel {
                prefix: Some("New:".into()),
                label: worktree_name.clone().into(),
                suffix: None,
            },
            Self::NewWorktree { .. } => StartThreadInLabel {
                prefix: None,
                label: "New Git Worktree".into(),
                suffix: None,
            },
            Self::LinkedWorktree { display_name, .. } => StartThreadInLabel {
                prefix: Some("From:".into()),
                label: display_name.clone().into(),
                suffix: None,
            },
        }
    }

    fn branch_trigger_label(&self, project: &Project, cx: &App) -> Option<StartThreadInLabel> {
        match self {
            Self::NewWorktree { branch_target, .. } => {
                let (branch_name, is_occupied) = match branch_target {
                    NewWorktreeBranchTarget::CurrentBranch => {
                        let name: SharedString = if project.repositories(cx).len() > 1 {
                            "current branches".into()
                        } else {
                            project
                                .active_repository(cx)
                                .and_then(|repo| {
                                    repo.read(cx)
                                        .branch
                                        .as_ref()
                                        .map(|branch| SharedString::from(branch.name().to_string()))
                                })
                                .unwrap_or_else(|| "HEAD".into())
                        };
                        (name, false)
                    }
                    NewWorktreeBranchTarget::ExistingBranch { name } => {
                        let occupied = Self::is_branch_occupied(name, project, cx);
                        (name.clone().into(), occupied)
                    }
                    NewWorktreeBranchTarget::CreateBranch {
                        from_ref: Some(from_ref),
                        ..
                    } => {
                        let occupied = Self::is_branch_occupied(from_ref, project, cx);
                        (from_ref.clone().into(), occupied)
                    }
                    NewWorktreeBranchTarget::CreateBranch { name, .. } => {
                        (name.clone().into(), false)
                    }
                };

                let prefix = if is_occupied {
                    Some("New From:".into())
                } else {
                    None
                };

                Some(StartThreadInLabel {
                    prefix,
                    label: branch_name,
                    suffix: None,
                })
            }
            _ => None,
        }
    }

    fn is_branch_occupied(branch_name: &str, project: &Project, cx: &App) -> bool {
        project.repositories(cx).values().any(|repo| {
            repo.read(cx)
                .linked_worktrees
                .iter()
                .any(|wt| wt.branch_name() == Some(branch_name))
        })
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum WorktreeCreationStatus {
    Creating,
    Error(SharedString),
}

#[derive(Clone, Debug)]
enum WorktreeCreationArgs {
    New {
        worktree_name: Option<String>,
        branch_target: NewWorktreeBranchTarget,
    },
    Linked {
        worktree_path: PathBuf,
    },
}

impl ActiveView {
    pub fn which_font_size_used(&self) -> WhichFontSize {
        match self {
            ActiveView::Uninitialized
            | ActiveView::AgentThread { .. }
            | ActiveView::History { .. } => WhichFontSize::AgentFont,
            ActiveView::Configuration => WhichFontSize::None,
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
    active_view: ActiveView,
    previous_view: Option<ActiveView>,
    background_threads: HashMap<acp::SessionId, Entity<ConversationView>>,
    new_thread_menu_handle: PopoverMenuHandle<ContextMenu>,
    start_thread_in_menu_handle: PopoverMenuHandle<ThreadWorktreePicker>,
    thread_branch_menu_handle: PopoverMenuHandle<ThreadBranchPicker>,
    agent_panel_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_navigation_menu_handle: PopoverMenuHandle<ContextMenu>,
    agent_navigation_menu: Option<Entity<ContextMenu>>,
    _extension_subscription: Option<Subscription>,
    _project_subscription: Subscription,
    _git_store_subscription: Subscription,
    zoomed: bool,
    pending_serialization: Option<Task<Result<()>>>,
    new_user_onboarding: Entity<AgentPanelOnboarding>,
    new_user_onboarding_upsell_dismissed: AtomicBool,
    agent_layout_onboarding: Entity<ai_onboarding::AgentLayoutOnboarding>,
    agent_layout_onboarding_dismissed: AtomicBool,
    selected_agent: Agent,
    start_thread_in: StartThreadIn,
    worktree_creation_status: Option<WorktreeCreationStatus>,
    _thread_view_subscription: Option<Subscription>,
    _active_thread_focus_subscription: Option<Subscription>,
    _worktree_creation_task: Option<Task<()>>,
    show_trust_workspace_message: bool,
    _active_view_observation: Option<Subscription>,
}

impl AgentPanel {
    fn serialize(&mut self, cx: &mut App) {
        let Some(workspace_id) = self.workspace_id else {
            return;
        };

        let selected_agent = self.selected_agent.clone();
        let start_thread_in = Some(self.start_thread_in.clone());

        let last_active_thread = self.active_agent_thread(cx).map(|thread| {
            let thread = thread.read(cx);
            let title = thread.title();
            let work_dirs = thread.work_dirs().cloned();
            SerializedActiveThread {
                session_id: thread.session_id().0.to_string(),
                agent_type: self.selected_agent.clone(),
                title: title.map(|t| t.to_string()),
                work_dirs: work_dirs.map(|dirs| dirs.serialize()),
            }
        });

        let kvp = KeyValueStore::global(cx);
        self.pending_serialization = Some(cx.background_spawn(async move {
            save_serialized_panel(
                workspace_id,
                SerializedAgentPanel {
                    selected_agent: Some(selected_agent),
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

            let last_active_thread = if let Some(thread_info) = serialized_panel
                .as_ref()
                .and_then(|p| p.last_active_thread.as_ref())
            {
                let session_id = acp::SessionId::new(thread_info.session_id.clone());
                let has_metadata = cx
                    .update(|_window, cx| {
                        let store = ThreadMetadataStore::global(cx);
                        store.read(cx).entry(&session_id).is_some()
                    })
                    .unwrap_or(false);
                if has_metadata {
                    Some(thread_info)
                } else {
                    log::warn!(
                        "last active thread {} has no metadata, skipping restoration",
                        thread_info.session_id
                    );
                    None
                }
            } else {
                None
            };

            let panel = workspace.update_in(cx, |workspace, window, cx| {
                let panel =
                    cx.new(|cx| Self::new(workspace, prompt_store, window, cx));

                panel.update(cx, |panel, cx| {
                    let is_via_collab = panel.project.read(cx).is_via_collab();

                    // Only apply a non-native global fallback to local projects.
                    // Collab workspaces only support NativeAgent, so inheriting a
                    // custom agent would cause set_active → new_agent_thread_inner
                    // to bypass the collab guard in external_thread.
                    let global_fallback = global_last_used_agent
                        .filter(|agent| !is_via_collab || agent.is_native());

                    if let Some(serialized_panel) = &serialized_panel {
                        if let Some(selected_agent) = serialized_panel.selected_agent.clone() {
                            panel.selected_agent = selected_agent;
                        } else if let Some(agent) = global_fallback {
                            panel.selected_agent = agent;
                        }
                        if let Some(ref start_thread_in) = serialized_panel.start_thread_in {
                            let is_valid = match &start_thread_in {
                                StartThreadIn::LocalProject => true,
                                StartThreadIn::NewWorktree { .. } => {
                                    let project = panel.project.read(cx);
                                    !project.is_via_collab()
                                }
                                StartThreadIn::LinkedWorktree { path, .. } => {
                                    path.exists()
                                }
                            };
                            if is_valid {
                                panel.start_thread_in = start_thread_in.clone();
                            } else {
                                log::info!(
                                    "deserialized start_thread_in {:?} is no longer valid, falling back to LocalProject",
                                    start_thread_in,
                                );
                            }
                        }
                    } else if let Some(agent) = global_fallback {
                        panel.selected_agent = agent;
                    }
                    cx.notify();
                });

                if let Some(thread_info) = last_active_thread {
                    let agent = thread_info.agent_type.clone();
                    panel.update(cx, |panel, cx| {
                        panel.selected_agent = agent.clone();
                        panel.load_agent_thread(
                            agent,
                            thread_info.session_id.clone().into(),
                            thread_info.work_dirs.as_ref().map(|dirs| PathList::deserialize(dirs)),
                            thread_info.title.as_ref().map(|t| t.clone().into()),
                            false,
                            window,
                            cx,
                        );
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
                            menu = Self::populate_recently_updated_menu_section(
                                menu, panel, history, cx,
                            );
                            menu = menu.action("View All", Box::new(OpenHistory));
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
                        .update(cx, |panel, cx| {
                            panel.dismiss_ai_onboarding(cx);
                        })
                        .ok();
                },
                cx,
            )
        });

        let weak_panel = cx.entity().downgrade();

        let layout = AgentSettings::get_layout(cx);
        let is_agent_layout = matches!(layout, WindowLayout::Agent(_));

        let agent_layout_onboarding = cx.new(|_cx| ai_onboarding::AgentLayoutOnboarding {
            use_agent_layout: Arc::new({
                let fs = fs.clone();
                let weak_panel = weak_panel.clone();
                move |_window, cx| {
                    AgentSettings::set_layout(WindowLayout::Agent(None), fs.clone(), cx);
                    weak_panel
                        .update(cx, |panel, cx| {
                            panel.dismiss_agent_layout_onboarding(cx);
                        })
                        .ok();
                }
            }),
            revert_to_editor_layout: Arc::new({
                let fs = fs.clone();
                let weak_panel = weak_panel.clone();
                move |_window, cx| {
                    AgentSettings::set_layout(WindowLayout::Editor(None), fs.clone(), cx);
                    weak_panel
                        .update(cx, |panel, cx| {
                            panel.dismiss_agent_layout_onboarding(cx);
                        })
                        .ok();
                }
            }),
            dismissed: Arc::new(move |_window, cx| {
                weak_panel
                    .update(cx, |panel, cx| {
                        panel.dismiss_agent_layout_onboarding(cx);
                    })
                    .ok();
            }),
            is_agent_layout,
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
        let git_store = project.read(cx).git_store().clone();
        let _git_store_subscription = cx.subscribe(&git_store, |this, _, event, cx| {
            let should_sync = matches!(
                event,
                GitStoreEvent::ActiveRepositoryChanged(_)
                    | GitStoreEvent::RepositoryAdded
                    | GitStoreEvent::RepositoryRemoved(_)
                    | GitStoreEvent::RepositoryUpdated(
                        _,
                        RepositoryEvent::HeadChanged
                            | RepositoryEvent::BranchListChanged
                            | RepositoryEvent::GitWorktreeListChanged,
                        _,
                    )
            );

            if should_sync {
                this.sync_start_thread_in_with_git_state(cx);
            }
        });

        let mut panel = Self {
            workspace_id,
            active_view,
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
            previous_view: None,
            background_threads: HashMap::default(),
            new_thread_menu_handle: PopoverMenuHandle::default(),
            start_thread_in_menu_handle: PopoverMenuHandle::default(),
            thread_branch_menu_handle: PopoverMenuHandle::default(),
            agent_panel_menu_handle: PopoverMenuHandle::default(),
            agent_navigation_menu_handle: PopoverMenuHandle::default(),
            agent_navigation_menu: None,
            _extension_subscription: extension_subscription,
            _project_subscription,
            _git_store_subscription,
            zoomed: false,
            pending_serialization: None,
            new_user_onboarding: onboarding,
            agent_layout_onboarding,
            thread_store,
            selected_agent: Agent::default(),
            start_thread_in: StartThreadIn::default(),
            worktree_creation_status: None,
            _thread_view_subscription: None,
            _active_thread_focus_subscription: None,
            _worktree_creation_task: None,
            show_trust_workspace_message: false,
            new_user_onboarding_upsell_dismissed: AtomicBool::new(OnboardingUpsell::dismissed(cx)),
            agent_layout_onboarding_dismissed: AtomicBool::new(AgentLayoutOnboarding::dismissed(
                cx,
            )),
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

    /// Reset the panel to the uninitialized state, clearing any active
    /// thread without creating a new draft. Running threads are retained
    /// in the background. The sidebar suppresses the uninitialized state
    /// so no "Draft" entry appears.
    pub fn clear_active_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_active_view(ActiveView::Uninitialized, false, window, cx);
    }

    pub fn new_thread(&mut self, _action: &NewThread, window: &mut Window, cx: &mut Context<Self>) {
        self.reset_start_thread_in_to_default(cx);
        let initial_content = self.take_active_draft_initial_content(cx);
        self.external_thread(None, None, None, None, initial_content, true, window, cx);
    }

    fn take_active_draft_initial_content(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Option<AgentInitialContent> {
        self.active_thread_view(cx).and_then(|thread_view| {
            thread_view.update(cx, |thread_view, cx| {
                let draft_blocks = thread_view
                    .thread
                    .read(cx)
                    .draft_prompt()
                    .map(|draft| draft.to_vec())
                    .filter(|draft| !draft.is_empty());

                let draft_blocks = draft_blocks.or_else(|| {
                    let text = thread_view.message_editor.read(cx).text(cx);
                    if text.trim().is_empty() {
                        None
                    } else {
                        Some(vec![acp::ContentBlock::Text(acp::TextContent::new(text))])
                    }
                });

                draft_blocks.map(|blocks| AgentInitialContent::ContentBlock {
                    blocks,
                    auto_submit: false,
                })
            })
        })
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
        let thread_store = self.thread_store.clone();

        let agent = agent_choice.unwrap_or_else(|| {
            if self.project.read(cx).is_via_collab() {
                Agent::NativeAgent
            } else {
                self.selected_agent.clone()
            }
        });

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

        let Some(active_thread) = conversation_view.read(cx).active_thread().cloned() else {
            return;
        };

        active_thread.update(cx, |active_thread, cx| {
            active_thread.expand_message_editor(&ExpandMessageEditor, window, cx);
            active_thread.focus_handle(cx).focus(window, cx);
        })
    }

    fn has_history_for_selected_agent(&self, cx: &App) -> bool {
        match &self.selected_agent {
            Agent::NativeAgent => true,
            Agent::Custom { .. } => self
                .connection_store
                .read(cx)
                .entry(&self.selected_agent)
                .map_or(false, |entry| entry.read(cx).history().is_some()),
        }
    }

    fn history_for_selected_agent(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<ThreadHistoryView>> {
        let agent = self.selected_agent.clone();
        let history = self
            .connection_store
            .read(cx)
            .entry(&agent)?
            .read(cx)
            .history()?
            .clone();
        Some(self.create_thread_history_view(agent, history, window, cx))
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
        let Some(view) = self.history_for_selected_agent(window, cx) else {
            return;
        };

        if let ActiveView::History { view: active_view } = &self.active_view {
            if active_view == &view {
                if let Some(previous_view) = self.previous_view.take() {
                    self.set_active_view(previous_view, true, window, cx);
                }
                return;
            }
        }

        self.set_active_view(ActiveView::History { view }, true, window, cx);
        cx.notify();
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

    pub fn background_threads(&self) -> &HashMap<acp::SessionId, Entity<ConversationView>> {
        &self.background_threads
    }

    pub fn active_conversation_view(&self) -> Option<&Entity<ConversationView>> {
        match &self.active_view {
            ActiveView::AgentThread { conversation_view } => Some(conversation_view),
            _ => None,
        }
    }

    pub fn conversation_views(&self) -> Vec<Entity<ConversationView>> {
        self.active_conversation_view()
            .into_iter()
            .cloned()
            .chain(self.background_threads.values().cloned())
            .collect()
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

        if let Some(conversation_view) = self.active_conversation_view() {
            conversation_view.update(cx, |conversation_view, cx| {
                conversation_view.set_work_dirs(new_work_dirs.clone(), cx);
            });
        }

        for conversation_view in self.background_threads.values() {
            conversation_view.update(cx, |conversation_view, cx| {
                conversation_view.set_work_dirs(new_work_dirs.clone(), cx);
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

        if thread_view.read(cx).thread.read(cx).entries().is_empty() {
            return;
        }

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

    fn set_active_view(
        &mut self,
        new_view: ActiveView,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let was_in_agent_history = matches!(self.active_view, ActiveView::History { .. });
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

        if let ActiveView::History { view } = &self.active_view {
            if !was_in_agent_history {
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
        view: Entity<ThreadHistoryView>,
        cx: &mut Context<ContextMenu>,
    ) -> ContextMenu {
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
        let new_target = match action {
            StartThreadIn::LocalProject => StartThreadIn::LocalProject,
            StartThreadIn::NewWorktree { .. } => {
                if !self.project_has_git_repository(cx) {
                    log::error!(
                        "set_start_thread_in: cannot use worktree mode without a git repository"
                    );
                    return;
                }
                if self.project.read(cx).is_via_collab() {
                    log::error!(
                        "set_start_thread_in: cannot use worktree mode in a collab project"
                    );
                    return;
                }
                action.clone()
            }
            StartThreadIn::LinkedWorktree { .. } => {
                if !self.project_has_git_repository(cx) {
                    log::error!(
                        "set_start_thread_in: cannot use LinkedWorktree without a git repository"
                    );
                    return;
                }
                if self.project.read(cx).is_via_collab() {
                    log::error!(
                        "set_start_thread_in: cannot use LinkedWorktree in a collab project"
                    );
                    return;
                }
                action.clone()
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
        let next = match &self.start_thread_in {
            StartThreadIn::LocalProject => StartThreadIn::NewWorktree {
                worktree_name: None,
                branch_target: NewWorktreeBranchTarget::default(),
            },
            StartThreadIn::NewWorktree { .. } | StartThreadIn::LinkedWorktree { .. } => {
                StartThreadIn::LocalProject
            }
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
                    StartThreadIn::NewWorktree {
                        worktree_name: None,
                        branch_target: NewWorktreeBranchTarget::default(),
                    }
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

    fn sync_start_thread_in_with_git_state(&mut self, cx: &mut Context<Self>) {
        if matches!(self.start_thread_in, StartThreadIn::LocalProject) {
            return;
        }

        let visible_worktree_paths: Vec<_> = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .collect();
        let repositories = self.project.read(cx).repositories(cx);
        let linked_worktrees = if repositories.len() > 1 {
            Vec::new()
        } else {
            repositories
                .values()
                .flat_map(|repo| repo.read(cx).linked_worktrees().iter().cloned())
                .filter(|worktree| !visible_worktree_paths.contains(&worktree.path))
                .collect::<Vec<_>>()
        };

        let updated_start_thread_in = match &self.start_thread_in {
            StartThreadIn::NewWorktree {
                worktree_name: Some(worktree_name),
                branch_target,
            } => {
                let normalized_worktree_name = worktree_name.replace(' ', "-");
                linked_worktrees
                    .iter()
                    .find(|worktree| {
                        worktree.display_name() == normalized_worktree_name
                            && self.linked_worktree_matches_branch_target(
                                worktree,
                                branch_target,
                                cx,
                            )
                    })
                    .map(|worktree| StartThreadIn::LinkedWorktree {
                        path: worktree.path.clone(),
                        display_name: worktree.display_name().to_string(),
                    })
            }
            StartThreadIn::LinkedWorktree { path, .. } => linked_worktrees
                .iter()
                .find(|worktree| worktree.path == *path)
                .map(|worktree| StartThreadIn::LinkedWorktree {
                    path: worktree.path.clone(),
                    display_name: worktree.display_name().to_string(),
                })
                .or(Some(StartThreadIn::LocalProject)),
            _ => None,
        };

        if let Some(updated_start_thread_in) = updated_start_thread_in {
            if self.start_thread_in != updated_start_thread_in {
                self.start_thread_in = updated_start_thread_in;
                self.serialize(cx);
            }
            cx.notify();
        }
    }

    fn linked_worktree_matches_branch_target(
        &self,
        worktree: &git::repository::Worktree,
        branch_target: &NewWorktreeBranchTarget,
        cx: &App,
    ) -> bool {
        let active_repository = self.project.read(cx).active_repository(cx);
        let current_branch_name = active_repository.as_ref().and_then(|repo| {
            repo.read(cx)
                .branch
                .as_ref()
                .map(|branch| branch.name().to_string())
        });
        let existing_branch_names = active_repository
            .as_ref()
            .map(|repo| {
                repo.read(cx)
                    .branch_list
                    .iter()
                    .map(|branch| branch.name().to_string())
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();

        match branch_target {
            NewWorktreeBranchTarget::CurrentBranch => {
                current_branch_name.as_deref() == worktree.branch_name()
            }
            NewWorktreeBranchTarget::ExistingBranch { name } => {
                existing_branch_names.contains(name)
                    && worktree.branch_name() == Some(name.as_str())
            }
            NewWorktreeBranchTarget::CreateBranch { name, .. } => {
                !existing_branch_names.contains(name)
                    && worktree.branch_name() == Some(name.as_str())
            }
        }
    }

    pub(crate) fn selected_agent(&self) -> Option<Agent> {
        Some(self.selected_agent.clone())
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

    pub fn new_agent_thread(&mut self, agent: Agent, window: &mut Window, cx: &mut Context<Self>) {
        self.reset_start_thread_in_to_default(cx);
        self.new_agent_thread_inner(agent, true, window, cx);
    }

    fn new_agent_thread_inner(
        &mut self,
        agent: Agent,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let initial_content = self.take_active_draft_initial_content(cx);
        self.external_thread(
            Some(agent),
            None,
            None,
            None,
            initial_content,
            focus,
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(store) = ThreadMetadataStore::try_global(cx) {
            store.update(cx, |store, cx| store.unarchive(&session_id, cx));
        }

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
        agent: Agent,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
        match &self.start_thread_in {
            StartThreadIn::NewWorktree {
                worktree_name,
                branch_target,
            } => {
                self.handle_worktree_requested(
                    content,
                    WorktreeCreationArgs::New {
                        worktree_name: worktree_name.clone(),
                        branch_target: branch_target.clone(),
                    },
                    window,
                    cx,
                );
            }
            StartThreadIn::LinkedWorktree { path, .. } => {
                self.handle_worktree_requested(
                    content,
                    WorktreeCreationArgs::Linked {
                        worktree_path: path.clone(),
                    },
                    window,
                    cx,
                );
            }
            StartThreadIn::LocalProject => {
                cx.defer_in(window, move |_this, window, cx| {
                    thread_view.update(cx, |thread_view, cx| {
                        let editor = thread_view.message_editor.clone();
                        thread_view.send_impl(editor, window, cx);
                    });
                });
            }
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

    fn resolve_worktree_branch_target(
        branch_target: &NewWorktreeBranchTarget,
        existing_branches: &HashSet<String>,
        occupied_branches: &HashSet<String>,
    ) -> Result<(String, bool, Option<String>)> {
        let generate_branch_name = || -> Result<String> {
            let refs: Vec<&str> = existing_branches.iter().map(|s| s.as_str()).collect();
            let mut rng = rand::rng();
            crate::branch_names::generate_branch_name(&refs, &mut rng)
                .ok_or_else(|| anyhow!("Failed to generate a unique branch name"))
        };

        match branch_target {
            NewWorktreeBranchTarget::CreateBranch { name, from_ref } => {
                Ok((name.clone(), false, from_ref.clone()))
            }
            NewWorktreeBranchTarget::ExistingBranch { name } => {
                if occupied_branches.contains(name) {
                    Ok((generate_branch_name()?, false, Some(name.clone())))
                } else {
                    Ok((name.clone(), true, None))
                }
            }
            NewWorktreeBranchTarget::CurrentBranch => Ok((generate_branch_name()?, false, None)),
        }
    }

    /// Kicks off an async git-worktree creation for each repository. Returns:
    ///
    /// - `creation_infos`: a vec of `(repo, new_path, receiver)` tuples—the
    ///   receiver resolves once the git worktree command finishes.
    /// - `path_remapping`: `(old_work_dir, new_worktree_path)` pairs used
    ///   later to remap open editor tabs into the new workspace.
    fn start_worktree_creations(
        git_repos: &[Entity<project::git_store::Repository>],
        worktree_name: Option<String>,
        branch_name: &str,
        use_existing_branch: bool,
        start_point: Option<String>,
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

        let worktree_name = worktree_name.unwrap_or_else(|| branch_name.to_string());

        for repo in git_repos {
            let (work_dir, new_path, receiver) = repo.update(cx, |repo, _cx| {
                let new_path =
                    repo.path_for_new_linked_worktree(&worktree_name, worktree_directory_setting)?;
                let target = if use_existing_branch {
                    debug_assert!(
                        git_repos.len() == 1,
                        "use_existing_branch should only be true for a single repo"
                    );
                    git::repository::CreateWorktreeTarget::ExistingBranch {
                        branch_name: branch_name.to_string(),
                    }
                } else {
                    git::repository::CreateWorktreeTarget::NewBranch {
                        branch_name: branch_name.to_string(),
                        base_sha: start_point.clone(),
                    }
                };
                let receiver = repo.create_worktree(target, new_path.clone());
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
            let selected_agent = self.selected_agent.clone();
            self.new_agent_thread(selected_agent, window, cx);
        }
        cx.notify();
    }

    fn handle_worktree_requested(
        &mut self,
        content: Vec<acp::ContentBlock>,
        args: WorktreeCreationArgs,
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

        if matches!(args, WorktreeCreationArgs::New { .. }) && git_repos.is_empty() {
            self.set_worktree_creation_error(
                "No git repositories found in the project".into(),
                window,
                cx,
            );
            return;
        }

        let (branch_receivers, worktree_receivers, worktree_directory_setting) =
            if matches!(args, WorktreeCreationArgs::New { .. }) {
                (
                    Some(
                        git_repos
                            .iter()
                            .map(|repo| repo.update(cx, |repo, _cx| repo.branches()))
                            .collect::<Vec<_>>(),
                    ),
                    Some(
                        git_repos
                            .iter()
                            .map(|repo| repo.update(cx, |repo, _cx| repo.worktrees()))
                            .collect::<Vec<_>>(),
                    ),
                    Some(
                        ProjectSettings::get_global(cx)
                            .git
                            .worktree_directory
                            .clone(),
                    ),
                )
            } else {
                (None, None, None)
            };

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
            let (all_paths, path_remapping, has_non_git) = match args {
                WorktreeCreationArgs::New {
                    worktree_name,
                    branch_target,
                } => {
                    let branch_receivers = branch_receivers
                        .expect("branch receivers must be prepared for new worktree creation");
                    let worktree_receivers = worktree_receivers
                        .expect("worktree receivers must be prepared for new worktree creation");
                    let worktree_directory_setting = worktree_directory_setting
                        .expect("worktree directory must be prepared for new worktree creation");

                    let mut existing_branches = HashSet::default();
                    for result in futures::future::join_all(branch_receivers).await {
                        match result {
                            Ok(Ok(branches)) => {
                                for branch in branches {
                                    existing_branches.insert(branch.name().to_string());
                                }
                            }
                            Ok(Err(err)) => {
                                Err::<(), _>(err).log_err();
                            }
                            Err(_) => {}
                        }
                    }

                    let mut occupied_branches = HashSet::default();
                    for result in futures::future::join_all(worktree_receivers).await {
                        match result {
                            Ok(Ok(worktrees)) => {
                                for worktree in worktrees {
                                    if let Some(branch_name) = worktree.branch_name() {
                                        occupied_branches.insert(branch_name.to_string());
                                    }
                                }
                            }
                            Ok(Err(err)) => {
                                Err::<(), _>(err).log_err();
                            }
                            Err(_) => {}
                        }
                    }

                    let (branch_name, use_existing_branch, start_point) =
                        match Self::resolve_worktree_branch_target(
                            &branch_target,
                            &existing_branches,
                            &occupied_branches,
                        ) {
                            Ok(target) => target,
                            Err(err) => {
                                this.update_in(cx, |this, window, cx| {
                                    this.set_worktree_creation_error(
                                        err.to_string().into(),
                                        window,
                                        cx,
                                    );
                                })?;
                                return anyhow::Ok(());
                            }
                        };

                    let (creation_infos, path_remapping) =
                        match this.update_in(cx, |_this, _window, cx| {
                            Self::start_worktree_creations(
                                &git_repos,
                                worktree_name,
                                &branch_name,
                                use_existing_branch,
                                start_point,
                                &worktree_directory_setting,
                                cx,
                            )
                        }) {
                            Ok(Ok(result)) => result,
                            Ok(Err(err)) | Err(err) => {
                                this.update_in(cx, |this, window, cx| {
                                    this.set_worktree_creation_error(
                                        format!("Failed to validate worktree directory: {err}")
                                            .into(),
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                                return anyhow::Ok(());
                            }
                        };

                    let created_paths =
                        match Self::await_and_rollback_on_failure(creation_infos, cx).await {
                            Ok(paths) => paths,
                            Err(err) => {
                                this.update_in(cx, |this, window, cx| {
                                    this.set_worktree_creation_error(
                                        format!("{err}").into(),
                                        window,
                                        cx,
                                    );
                                })?;
                                return anyhow::Ok(());
                            }
                        };

                    let mut all_paths = created_paths;
                    let has_non_git = !non_git_paths.is_empty();
                    all_paths.extend(non_git_paths.iter().cloned());
                    (all_paths, path_remapping, has_non_git)
                }
                WorktreeCreationArgs::Linked { worktree_path } => {
                    let mut all_paths = vec![worktree_path];
                    let has_non_git = !non_git_paths.is_empty();
                    all_paths.extend(non_git_paths.iter().cloned());
                    (all_paths, Vec::new(), has_non_git)
                }
            };

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
            if let Err(err) = Self::open_worktree_workspace_and_start_thread(
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

        self._worktree_creation_task = Some(cx.background_spawn(async move {
            task.await.log_err();
        }));
    }

    async fn open_worktree_workspace_and_start_thread(
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

            new_workspace.update(cx, |workspace, cx| {
                workspace.run_create_worktree_tasks(window, cx);
            })
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

            this.start_thread_in = StartThreadIn::LocalProject;
            this.serialize(cx);
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
            ActiveView::History { view } => view.read(cx).focus_handle(cx),
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
            let selected_agent = self.selected_agent.clone();
            self.new_agent_thread_inner(selected_agent, false, window, cx);
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
            ActiveView::History { .. } => Label::new("History").truncate().into_any_element(),
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

    fn render_panel_options_menu(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

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

                        if thread_with_messages {
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

    fn project_has_git_repository(&self, cx: &App) -> bool {
        !self.project.read(cx).repositories(cx).is_empty()
    }

    fn render_start_thread_in_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        let is_creating = matches!(
            self.worktree_creation_status,
            Some(WorktreeCreationStatus::Creating)
        );

        let trigger_parts = self
            .start_thread_in
            .trigger_label(self.project.read(cx), cx);

        let icon = if self.start_thread_in_menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let trigger_button = ButtonLike::new("thread-target-trigger")
            .disabled(is_creating)
            .when_some(trigger_parts.prefix, |this, prefix| {
                this.child(Label::new(prefix).color(Color::Muted))
            })
            .child(Label::new(trigger_parts.label))
            .when_some(trigger_parts.suffix, |this, suffix| {
                this.child(Label::new(suffix).color(Color::Muted))
            })
            .child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted));

        let project = self.project.clone();
        let current_target = self.start_thread_in.clone();
        let fs = self.fs.clone();

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
                let fs = fs.clone();
                Some(cx.new(|cx| {
                    ThreadWorktreePicker::new(project.clone(), &current_target, fs, window, cx)
                }))
            })
            .with_handle(self.start_thread_in_menu_handle.clone())
            .anchor(Corner::TopLeft)
            .offset(gpui::Point {
                x: px(1.0),
                y: px(1.0),
            })
    }

    fn render_new_worktree_branch_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_creating = matches!(
            self.worktree_creation_status,
            Some(WorktreeCreationStatus::Creating)
        );

        let project_ref = self.project.read(cx);
        let trigger_parts = self
            .start_thread_in
            .branch_trigger_label(project_ref, cx)
            .unwrap_or_else(|| StartThreadInLabel {
                prefix: Some("From:".into()),
                label: "HEAD".into(),
                suffix: None,
            });

        let icon = if self.thread_branch_menu_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        let trigger_button = ButtonLike::new("thread-branch-trigger")
            .disabled(is_creating)
            .when_some(trigger_parts.prefix, |this, prefix| {
                this.child(Label::new(prefix).color(Color::Muted))
            })
            .child(Label::new(trigger_parts.label))
            .child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted));

        let project = self.project.clone();
        let current_target = self.start_thread_in.clone();

        PopoverMenu::new("thread-branch-selector")
            .trigger_with_tooltip(trigger_button, Tooltip::text("Choose Worktree Branch…"))
            .menu(move |window, cx| {
                Some(cx.new(|cx| {
                    ThreadBranchPicker::new(project.clone(), &current_target, window, cx)
                }))
            })
            .with_handle(self.thread_branch_menu_handle.clone())
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

        let active_thread = match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
                conversation_view.read(cx).as_native_thread(cx)
            }
            ActiveView::Uninitialized | ActiveView::History { .. } | ActiveView::Configuration => {
                None
            }
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
                                                        panel.new_agent_thread(
                                                            Agent::NativeAgent,
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
                                                            panel.new_agent_thread(
                                                                Agent::Custom {
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
        let selected_agent_builtin_icon = self.selected_agent.icon();
        let selected_agent_label_for_tooltip = selected_agent_label.clone();

        let selected_agent = div()
            .id("selected_agent_icon")
            .when_some(selected_agent_custom_icon, |this, icon_path| {
                this.px_1()
                    .child(Icon::from_external_svg(icon_path).color(Color::Muted))
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

        let is_in_history_or_config = matches!(
            &self.active_view,
            ActiveView::History { .. } | ActiveView::Configuration
        );

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
            // TODO: This is only until we remove Agent settings from the panel.
            .when(!is_in_history_or_config, |this| {
                this.max_w(max_content_width).mx_auto()
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
                        )
                        .when(
                            matches!(self.start_thread_in, StartThreadIn::NewWorktree { .. }),
                            |this| this.child(self.render_new_worktree_branch_selector(cx)),
                        ),
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
                    .icon(IconName::XCircleFilled)
                    .severity(Severity::Error)
                    .title("Worktree Creation Error")
                    .description(message.clone())
                    .border_position(ui::BorderPosition::Bottom)
                    .dismiss_action(
                        IconButton::new("dismiss-worktree-error", IconName::Close)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Dismiss"))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.worktree_creation_status = None;
                                cx.notify();
                            })),
                    )
                    .into_any_element(),
            ),
        }
    }

    fn should_render_trial_end_upsell(&self, cx: &mut Context<Self>) -> bool {
        if TrialEndUpsell::dismissed(cx) {
            return false;
        }

        match &self.active_view {
            ActiveView::AgentThread { .. } => {
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
            ActiveView::Uninitialized | ActiveView::History { .. } | ActiveView::Configuration => {
                return false;
            }
        }

        let plan = self.user_store.read(cx).plan();
        let has_previous_trial = self.user_store.read(cx).trial_started_at().is_some();

        plan.is_some_and(|plan| plan == Plan::ZedFree) && has_previous_trial
    }

    fn should_render_agent_layout_onboarding(&self, cx: &mut Context<Self>) -> bool {
        // We only want to show this for existing users: those who
        // have used the agent panel before the sidebar was introduced.
        // We can infer that state by users having seen the onboarding
        // at one point, but not the agent layout onboarding.

        let has_messages = self.active_thread_has_messages(cx);
        let is_dismissed = self
            .agent_layout_onboarding_dismissed
            .load(Ordering::Acquire);

        if is_dismissed || has_messages {
            return false;
        }

        match &self.active_view {
            ActiveView::Uninitialized | ActiveView::History { .. } | ActiveView::Configuration => {
                false
            }
            ActiveView::AgentThread { .. } => {
                let existing_user = self
                    .new_user_onboarding_upsell_dismissed
                    .load(Ordering::Acquire);
                existing_user
            }
        }
    }

    fn render_agent_layout_onboarding(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        if !self.should_render_agent_layout_onboarding(cx) {
            return None;
        }

        Some(div().child(self.agent_layout_onboarding.clone()))
    }

    fn dismiss_agent_layout_onboarding(&mut self, cx: &mut Context<Self>) {
        self.agent_layout_onboarding_dismissed
            .store(true, Ordering::Release);
        AgentLayoutOnboarding::set_dismissed(true, cx);
        cx.notify();
    }

    fn dismiss_ai_onboarding(&mut self, cx: &mut Context<Self>) {
        self.new_user_onboarding_upsell_dismissed
            .store(true, Ordering::Release);
        OnboardingUpsell::set_dismissed(true, cx);
        self.dismiss_agent_layout_onboarding(cx);
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
        match &self.active_view {
            ActiveView::AgentThread { conversation_view } => {
                conversation_view.update(cx, |conversation_view, cx| {
                    conversation_view.insert_dragged_files(paths, added_worktrees, window, cx);
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
            .children(self.render_new_user_onboarding(window, cx))
            .children(self.render_agent_layout_onboarding(window, cx))
            .map(|parent| match &self.active_view {
                ActiveView::Uninitialized => parent,
                ActiveView::AgentThread {
                    conversation_view, ..
                } => parent
                    .child(conversation_view.clone())
                    .child(self.render_drag_target(cx)),
                ActiveView::History { view } => parent.child(view.clone()),
                ActiveView::Configuration => parent.children(self.configuration.clone()),
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

struct OnboardingUpsell;

impl Dismissable for OnboardingUpsell {
    const KEY: &'static str = "dismissed-trial-upsell";
}

struct AgentLayoutOnboarding;

impl Dismissable for AgentLayoutOnboarding {
    const KEY: &'static str = "dismissed-agent-layout-onboarding";
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
    use fs::FakeFs;
    use gpui::{TestAppContext, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use std::path::Path;
    use std::time::Instant;
    use workspace::MultiWorkspace;

    #[gpui::test]
    async fn test_active_thread_serialize_and_load_round_trip(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
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

        // --- Set up workspace B: ClaudeCode, no active thread ---
        let panel_b = workspace_b.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| AgentPanel::new(workspace, None, window, cx))
        });

        panel_b.update(cx, |panel, _cx| {
            panel.selected_agent = Agent::Custom {
                id: "claude-acp".into(),
            };
        });

        // --- Serialize both panels ---
        panel_a.update(cx, |panel, cx| panel.serialize(cx));
        panel_b.update(cx, |panel, cx| panel.serialize(cx));
        cx.run_until_parked();

        // --- Load fresh panels for each workspace and verify independent state ---
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

        // Workspace B should restore its own agent type, with no thread
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
                "workspace B should have no active thread"
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
                "thread without metadata should not be restored"
            );
        });
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
    async fn test_empty_draft_thread_not_retained_when_navigating_away(cx: &mut TestAppContext) {
        let (panel, mut cx) = setup_panel(cx).await;

        let connection_a = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_a, &mut cx);
        let session_id_a = active_session_id(&panel, &cx);

        panel.read_with(&cx, |panel, cx| {
            let thread = panel.active_agent_thread(cx).unwrap();
            assert!(
                thread.read(cx).entries().is_empty(),
                "newly opened draft thread should have no entries"
            );
            assert!(panel.background_threads.is_empty());
        });

        let connection_b = StubAgentConnection::new();
        open_thread_with_connection(&panel, connection_b, &mut cx);

        panel.read_with(&cx, |panel, _cx| {
            assert!(
                panel.background_threads.is_empty(),
                "empty draft thread should not be retained in background_threads"
            );
            assert!(
                !panel.background_threads.contains_key(&session_id_a),
                "empty draft thread should not be keyed in background_threads"
            );
        });
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
        send_message(&panel, &mut cx);

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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
                    multi_workspace.workspaces().count(),
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
    async fn test_thread_target_does_not_sync_to_external_linked_worktree_with_invalid_branch_target(
        cx: &mut TestAppContext,
    ) {
        use git::repository::Worktree as GitWorktree;

        init_test(cx);
        cx.update(|cx| {
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
        fs.insert_branches(Path::new("/project/.git"), &["main", "feature-worktree"]);

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

        cx.run_until_parked();

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        cx.run_until_parked();

        panel.update_in(cx, |panel, window, cx| {
            panel.set_start_thread_in(
                &StartThreadIn::NewWorktree {
                    worktree_name: Some("feature worktree".to_string()),
                    branch_target: NewWorktreeBranchTarget::CurrentBranch,
                },
                window,
                cx,
            );
        });

        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/linked-feature-worktree"),
                ref_name: Some("refs/heads/feature-worktree".into()),
                sha: "abcdef1".into(),
                is_main: false,
            },
        )
        .await;

        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        cx.run_until_parked();

        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                *panel.start_thread_in(),
                StartThreadIn::NewWorktree {
                    worktree_name: Some("feature worktree".to_string()),
                    branch_target: NewWorktreeBranchTarget::CurrentBranch,
                },
                "thread target should remain a named new worktree when the external linked worktree does not match the selected branch target",
            );
        });
    }

    #[gpui::test]
    async fn test_thread_target_serialization_round_trip(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
            panel.set_start_thread_in(
                &StartThreadIn::NewWorktree {
                    worktree_name: None,
                    branch_target: NewWorktreeBranchTarget::default(),
                },
                window,
                cx,
            );
        });

        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                *panel.start_thread_in(),
                StartThreadIn::NewWorktree {
                    worktree_name: None,
                    branch_target: NewWorktreeBranchTarget::default(),
                },
                "thread target should be NewWorktree after set_thread_target"
            );
        });

        // Let serialization complete.
        cx.run_until_parked();

        // Load a fresh panel from the serialized data.
        let async_cx = cx.update(|window, cx| window.to_async(cx));
        let loaded_panel = AgentPanel::load(workspace.downgrade(), async_cx)
            .await
            .expect("panel load should succeed");
        cx.run_until_parked();

        loaded_panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                *panel.start_thread_in(),
                StartThreadIn::NewWorktree {
                    worktree_name: None,
                    branch_target: NewWorktreeBranchTarget::default(),
                },
                "thread target should survive serialization round-trip"
            );
        });
    }

    #[gpui::test]
    async fn test_set_active_blocked_during_worktree_creation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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

    #[test]
    fn test_resolve_worktree_branch_target() {
        let existing_branches = HashSet::from_iter([
            "main".to_string(),
            "feature".to_string(),
            "origin/main".to_string(),
        ]);

        let resolved = AgentPanel::resolve_worktree_branch_target(
            &NewWorktreeBranchTarget::CreateBranch {
                name: "new-branch".to_string(),
                from_ref: Some("main".to_string()),
            },
            &existing_branches,
            &HashSet::from_iter(["main".to_string()]),
        )
        .unwrap();
        assert_eq!(
            resolved,
            ("new-branch".to_string(), false, Some("main".to_string()))
        );

        let resolved = AgentPanel::resolve_worktree_branch_target(
            &NewWorktreeBranchTarget::ExistingBranch {
                name: "feature".to_string(),
            },
            &existing_branches,
            &HashSet::default(),
        )
        .unwrap();
        assert_eq!(resolved, ("feature".to_string(), true, None));

        let resolved = AgentPanel::resolve_worktree_branch_target(
            &NewWorktreeBranchTarget::ExistingBranch {
                name: "main".to_string(),
            },
            &existing_branches,
            &HashSet::from_iter(["main".to_string()]),
        )
        .unwrap();
        assert_eq!(resolved.1, false);
        assert_eq!(resolved.2, Some("main".to_string()));
        assert_ne!(resolved.0, "main");
        assert!(existing_branches.contains("main"));
        assert!(!existing_branches.contains(&resolved.0));
    }

    #[gpui::test]
    async fn test_worktree_creation_preserves_selected_agent(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(|cx| {
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
        multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                multi_workspace.open_sidebar(cx);
            })
            .unwrap();

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
                        let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
            let panel = cx.new(|cx| AgentPanel::new(workspace, None, window, cx));
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
        // open_external_thread_with_server overrides selected_agent.
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_agent = Agent::Custom {
                id: CODEX_ID.into(),
            };
            panel.set_start_thread_in(
                &StartThreadIn::NewWorktree {
                    worktree_name: None,
                    branch_target: NewWorktreeBranchTarget::default(),
                },
                window,
                cx,
            );
        });

        // Verify the panel has the Codex agent selected.
        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_agent,
                Agent::Custom {
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
            panel.handle_worktree_requested(
                content,
                WorktreeCreationArgs::New {
                    worktree_name: None,
                    branch_target: NewWorktreeBranchTarget::default(),
                },
                window,
                cx,
            );
        });

        // Let the async worktree creation + workspace setup complete.
        cx.run_until_parked();

        panel.read_with(cx, |panel, _cx| {
            assert_eq!(
                panel.start_thread_in(),
                &StartThreadIn::LocalProject,
                "the original panel should reset start_thread_in back to the local project after creating a worktree workspace",
            );
        });

        // Find the new workspace's AgentPanel and verify it used the Codex agent.
        let found_codex = multi_workspace
            .read_with(cx, |multi_workspace, cx| {
                // There should be more than one workspace now (the original + the new worktree).
                assert!(
                    multi_workspace.workspaces().count() > 1,
                    "expected a new workspace to have been created, found {}",
                    multi_workspace.workspaces().count(),
                );

                // Check the newest workspace's panel for the correct agent.
                let new_workspace = multi_workspace
                    .workspaces()
                    .find(|ws| ws.entity_id() != workspace.entity_id())
                    .expect("should find the new workspace");
                let new_panel = new_workspace
                    .read(cx)
                    .panel::<AgentPanel>(cx)
                    .expect("new workspace should have an AgentPanel");

                new_panel.read(cx).selected_agent.clone()
            })
            .unwrap();

        assert_eq!(
            found_codex,
            Agent::Custom {
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

        // Verify thread idle C was also updated.
        let updated_c_paths = panel.read_with(&cx, |panel, cx| {
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
}
