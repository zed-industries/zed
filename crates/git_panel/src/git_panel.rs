mod git_panel_settings;

use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use file_icons::FileIcons;
use git2;
use gpui::{
    actions, impl_actions, Action, AppContext, AssetSource, AsyncWindowContext, EventEmitter,
    FocusHandle, FocusableView, InteractiveElement, IntoElement, KeyContext, Model, ParentElement,
    Pixels, Render, Styled, Subscription, Task, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};

use git_panel_settings::{GitPanelDockPosition, GitPanelSettings};
use project::{Fs, Project};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use theme::ThemeSettings;
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    ui::{h_flex, v_flex, IconName},
    Workspace,
};

#[derive(Clone, Default, Deserialize, PartialEq)]
pub struct Open {
    change_selection: bool,
}

impl_actions!(outline_panel, [Open]);

actions!(
    outline_panel,
    [RevealInFileManager, SelectParent, ToggleFocus,]
);

const OUTLINE_PANEL_KEY: &str = "GitPanel";

#[derive(Debug, Clone)]
struct GitStatus {
    branch: BranchInfo,
    files: Vec<FileStatus>,
}

#[derive(Debug, Clone)]
struct BranchInfo {
    current_branch: String,
}

#[derive(Debug, Clone)]
struct FileStatus {
    path: String,
    status: GitFileStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum GitFileStatus {
    Modified,
    Added,
    Deleted,
    Renamed(String), // Contains the old path
    Untracked,
}

pub struct GitPanel {
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    project: Model<Project>,
    active: bool,
    pending_serialization: Task<Option<()>>,
    _subscriptions: Vec<Subscription>,
    filter_editor: View<Editor>,
    git_status: Option<GitStatus>,
    refresh_task: Task<()>,
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

#[derive(Serialize, Deserialize)]
struct SerializedOutlinePanel {
    width: Option<Pixels>,
    active: Option<bool>,
}

pub fn init_settings(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
}

pub fn init(assets: impl AssetSource, cx: &mut AppContext) {
    init_settings(cx);
    file_icons::init(assets, cx);

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<GitPanel>(cx);
        });
    })
    .detach();
}

impl GitPanel {
    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(OUTLINE_PANEL_KEY) })
            .await
            .context("loading git panel")
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedOutlinePanel>(&panel))
            .transpose()
            .log_err()
            .flatten();

        workspace.update(&mut cx, |workspace, cx| {
            let panel = Self::new(workspace, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width.map(|px| px.round());
                    panel.active = serialized_panel.active.unwrap_or(false);
                    cx.notify();
                });
            }
            panel
        })
    }

    fn get_workspace_root_path(&self, cx: &AppContext) -> Option<PathBuf> {
        let project = self.project.read(cx);
        project
            .worktrees(cx)
            .next() // Get first worktree
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
    }

    fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let git_panel = cx.new_view(|cx| {
            let filter_editor = cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter...", cx);
                editor
            });

            let icons_subscription = cx.observe_global::<FileIcons>(|_, cx| {
                cx.notify();
            });

            let mut git_panel_settings = *GitPanelSettings::get_global(cx);
            let mut current_theme = ThemeSettings::get_global(cx).clone();
            let settings_subscription = cx.observe_global::<SettingsStore>(move |_, cx| {
                let new_settings = GitPanelSettings::get_global(cx);
                let new_theme = ThemeSettings::get_global(cx);
                if &current_theme != new_theme {
                    git_panel_settings = *new_settings;
                    current_theme = new_theme.clone();
                } else if &git_panel_settings != new_settings {
                    git_panel_settings = *new_settings;
                    cx.notify();
                }
            });

            let mut git_panel = Self {
                active: false,
                fs: workspace.app_state().fs.clone(),
                project: workspace.project().clone(),
                filter_editor,
                width: None,
                pending_serialization: Task::ready(None),
                _subscriptions: vec![settings_subscription, icons_subscription],
                git_status: None,
                refresh_task: Task::ready(()),
            };

            if git_panel.active {
                git_panel.refresh_git_status(cx);
            }

            git_panel
        });

        git_panel
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        let active = Some(self.active);
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        OUTLINE_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedOutlinePanel { width, active })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn dispatch_context(&self, cx: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");
        dispatch_context.add("menu");
        let identifier = if self.filter_editor.focus_handle(cx).is_focused(cx) {
            "editing"
        } else {
            "not_editing"
        };
        dispatch_context.add(identifier);
        dispatch_context
    }

    fn refresh_git_status(&mut self, cx: &mut ViewContext<Self>) {
        let workspace_path = self.get_workspace_root_path(cx).unwrap();

        self.refresh_task = cx.spawn(
            |panel: WeakView<GitPanel>, mut cx: AsyncWindowContext| async move {
                // Create a new repository instance
                if let Ok(repo) = git2::Repository::open(&workspace_path) {
                    // Get branch information
                    if let Ok(head) = repo.head() {
                        let branch_name = head.shorthand().unwrap_or("HEAD detached").to_string();

                        // Get status of files
                        let mut files = Vec::new();
                        if let Ok(statuses) = repo.statuses(None) {
                            for entry in statuses.iter() {
                                let status = entry.status();
                                let path = entry.path().unwrap_or("").to_string();

                                let file_status = if status.is_wt_modified() {
                                    GitFileStatus::Modified
                                } else if status.is_wt_new() {
                                    GitFileStatus::Added
                                } else if status.is_wt_deleted() {
                                    GitFileStatus::Deleted
                                } else if status.is_wt_renamed() {
                                    GitFileStatus::Renamed(
                                        entry
                                            .head_to_index()
                                            .unwrap()
                                            .old_file()
                                            .path()
                                            .unwrap()
                                            .to_string_lossy()
                                            .into(),
                                    )
                                } else if status.is_ignored() {
                                    continue;
                                } else {
                                    GitFileStatus::Untracked
                                };

                                files.push(FileStatus {
                                    path,
                                    status: file_status,
                                });
                            }
                        }

                        let git_status = GitStatus {
                            branch: BranchInfo {
                                current_branch: branch_name,
                            },
                            files,
                        };

                        panel
                            .update(&mut cx, |panel, cx| {
                                panel.git_status = Some(git_status);
                                cx.notify();
                            })
                            .ok();
                    }
                }
            },
        );
    }

    fn clear_git_status(&mut self, cx: &mut ViewContext<Self>) {
        self.git_status = None;
        cx.notify();
    }

    fn force_refresh_git_status(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_git_status(cx);
        self.refresh_git_status(cx);
    }
}

impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "Outline Panel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match GitPanelSettings::get_global(cx).dock {
            GitPanelDockPosition::Left => DockPosition::Left,
            GitPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<GitPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => GitPanelDockPosition::Left,
                    DockPosition::Right => GitPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| GitPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        GitPanelSettings::get_global(cx)
            .button
            .then_some(IconName::Git)
    }

    fn icon_tooltip(&self, _: &WindowContext) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &WindowContext) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        cx.spawn(|git_panel, mut cx| async move {
            git_panel
                .update(&mut cx, |git_panel, cx| {
                    git_panel.active = active;

                    if active {
                        // Force immediate refresh when panel becomes active
                        git_panel.force_refresh_git_status(cx);
                    } else {
                        git_panel.clear_git_status(cx);
                    }

                    git_panel.serialize(cx);
                })
                .ok();
        })
        .detach()
    }
}

impl FocusableView for GitPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.filter_editor.focus_handle(cx).clone()
    }
}

impl EventEmitter<Event> for GitPanel {}

impl EventEmitter<PanelEvent> for GitPanel {}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let outline_panel = v_flex()
            .id("git-panel")
            .size_full()
            .relative()
            .key_context(self.dispatch_context(cx));

        if let Some(git_status) = &self.git_status {
            outline_panel.child(
                v_flex()
                    .gap_2()
                    .p_1()
                    .child(
                        // Branch information
                        v_flex()
                            .gap_1()
                            .child(format!("Branch: {}", git_status.branch.current_branch)),
                    )
                    .child(
                        // File changes
                        v_flex()
                            .gap_0()
                            .children(git_status.files.iter().map(|file| {
                                let status_icon = match file.status {
                                    GitFileStatus::Modified => "M",
                                    GitFileStatus::Added => "A",
                                    GitFileStatus::Deleted => "D",
                                    GitFileStatus::Renamed(_) => "R",
                                    GitFileStatus::Untracked => "?",
                                };
                                h_flex()
                                    .gap_1()
                                    .p_4()
                                    .child(status_icon)
                                    .child(format!("{}", &file.path))
                            })),
                    ),
            )
        } else {
            outline_panel.child("No git repository found")
        }
    }
}
