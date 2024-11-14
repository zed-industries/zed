use editor::Editor;
use git::repository::GitFileStatus;
use gpui::*;
use ui::{prelude::*, ElevationIndex, IconButtonShape};
use ui::{Disclosure, Divider};
use workspace::item::TabContentParams;
use workspace::{item::ItemHandle, StatusItemView, ToolbarItemEvent, Workspace};

actions!(
    vcs_status,
    [
        Deploy,
        DiscardAll,
        StageAll,
        DiscardSelected,
        StageSelected,
        UnstageSelected,
        UnstageAll,
        Reset
    ]
);

#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub staged: bool,
    pub file_path: SharedString,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub status: GitFileStatus,
}

pub struct GitLines {
    pub added: usize,
    pub removed: usize,
}

#[derive(IntoElement)]
pub struct ChangedFileHeader {
    id: ElementId,
    file: ChangedFile,
    is_selected: bool,
    index: usize,
    project_status: Model<GitProjectStatus>,
}

impl ChangedFileHeader {
    fn new(
        id: impl Into<ElementId>,
        file: ChangedFile,
        is_selected: bool,
        index: usize,
        project_status: Model<GitProjectStatus>,
    ) -> Self {
        Self {
            id: id.into(),
            file,
            is_selected,
            index,
            project_status,
        }
    }

    fn icon_for_status(&self) -> impl IntoElement {
        let (icon_name, color) = match self.file.status {
            GitFileStatus::Added => (IconName::SquarePlus, Color::Created),
            GitFileStatus::Modified => (IconName::SquareDot, Color::Modified),
            GitFileStatus::Conflict => (IconName::SquareMinus, Color::Conflict),
        };

        Icon::new(icon_name).size(IconSize::Small).color(color)
    }
}

impl RenderOnce for ChangedFileHeader {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let disclosure_id = ElementId::Name(format!("{}-file-disclosure", self.id.clone()).into());
        let file_path = self.file.file_path.clone();

        h_flex()
            .id(self.id.clone())
            .justify_between()
            .w_full()
            .when(!self.is_selected, |this| {
                this.hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
            })
            .cursor(CursorStyle::PointingHand)
            .when(self.is_selected, |this| {
                this.bg(cx.theme().colors().ghost_element_active)
            })
            .group("")
            .rounded_sm()
            .px_2()
            .py_1p5()
            .child(
                h_flex()
                    .gap_2()
                    .child(Disclosure::new(disclosure_id, false))
                    .child(self.icon_for_status())
                    .child(Label::new(file_path).size(LabelSize::Small))
                    .child(
                        h_flex()
                            .gap_1()
                            .when(self.file.lines_added > 0, |this| {
                                this.child(
                                    Label::new(format!("+{}", self.file.lines_added))
                                        .color(Color::Created)
                                        .size(LabelSize::Small),
                                )
                            })
                            .when(self.file.lines_removed > 0, |this| {
                                this.child(
                                    Label::new(format!("-{}", self.file.lines_removed))
                                        .color(Color::Deleted)
                                        .size(LabelSize::Small),
                                )
                            }),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        IconButton::new("more-menu", IconName::EllipsisVertical)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Muted),
                    )
                    .child(
                        IconButton::new("remove-file", IconName::X)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Error)
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::Background)
                            .on_click(move |_, cx| {
                                self.project_status.update(cx, |status, _| {
                                    status.discard_file(self.index);
                                });
                            }),
                    )
                    .child(
                        IconButton::new("check-file", IconName::Check)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Accent)
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::Background)
                            .on_click(move |_, cx| {
                                self.project_status.update(cx, |status, _| {
                                    if self.file.staged {
                                        status.unstage_file(self.index);
                                    } else {
                                        status.stage_file(self.index);
                                    }
                                });
                            }),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct GitProjectOverview {
    id: ElementId,
    project_status: Model<GitProjectStatus>,
}

impl GitProjectOverview {
    pub fn new(id: impl Into<ElementId>, project_status: Model<GitProjectStatus>) -> Self {
        Self {
            id: id.into(),
            project_status,
        }
    }

    pub fn toggle_file_list(&self, cx: &mut WindowContext) {
        self.project_status.update(cx, |status, cx| {
            status.show_list = !status.show_list;
            cx.notify();
        });
    }
}

impl RenderOnce for GitProjectOverview {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let status = self.project_status.read(cx);

        let changed_files: SharedString =
            format!("{} Changed files", status.changed_file_count()).into();

        let added_label: Option<SharedString> = (status.lines_changed.added > 0)
            .then(|| format!("+{}", status.lines_changed.added).into());
        let removed_label: Option<SharedString> = (status.lines_changed.removed > 0)
            .then(|| format!("-{}", status.lines_changed.removed).into());
        let total_label: SharedString = "total lines changed".into();

        h_flex()
            .id(self.id.clone())
            .w_full()
            .bg(cx.theme().colors().elevated_surface_background)
            .px_2()
            .py_2p5()
            .gap_2()
            .child(
                IconButton::new("open-sidebar", IconName::PanelLeft)
                    .selected(self.project_status.read(cx).show_list)
                    .icon_color(Color::Muted)
                    .on_click(move |_, cx| self.toggle_file_list(cx)),
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(Label::new(changed_files).size(LabelSize::Small))
                    .child(
                        h_flex()
                            .gap_1()
                            .when(added_label.is_some(), |this| {
                                this.child(
                                    Label::new(added_label.unwrap())
                                        .color(Color::Created)
                                        .size(LabelSize::Small),
                                )
                            })
                            .when(removed_label.is_some(), |this| {
                                this.child(
                                    Label::new(removed_label.unwrap())
                                        .color(Color::Deleted)
                                        .size(LabelSize::Small),
                                )
                            })
                            .child(
                                Label::new(total_label)
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct GitStagingControls {
    id: ElementId,
    project_status: Model<GitProjectStatus>,
    is_staged: bool,
    is_selected: bool,
}

impl GitStagingControls {
    pub fn new(
        id: impl Into<ElementId>,
        project_status: Model<GitProjectStatus>,
        is_staged: bool,
        is_selected: bool,
    ) -> Self {
        Self {
            id: id.into(),
            project_status,
            is_staged,
            is_selected,
        }
    }
}

impl RenderOnce for GitStagingControls {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let status = self.project_status.read(cx);

        let (staging_type, count) = if self.is_staged {
            ("Staged", status.staged_count())
        } else {
            ("Unstaged", status.unstaged_count())
        };

        let is_expanded = if self.is_staged {
            status.staged_expanded
        } else {
            status.unstaged_expanded
        };

        let label: SharedString = format!("{} Changes: {}", staging_type, count).into();

        h_flex()
            .id(self.id.clone())
            .hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
            .on_click(move |_, cx| {
                self.project_status.update(cx, |status, cx| {
                    if self.is_staged {
                        status.staged_expanded = !status.staged_expanded;
                    } else {
                        status.unstaged_expanded = !status.unstaged_expanded;
                    }
                    cx.notify();
                })
            })
            .justify_between()
            .w_full()
            .map(|this| {
                if self.is_selected {
                    this.bg(cx.theme().colors().ghost_element_active)
                } else {
                    this.bg(cx.theme().colors().elevated_surface_background)
                }
            })
            .px_3()
            .py_2()
            .child(
                h_flex()
                    .gap_2()
                    .child(Disclosure::new(self.id.clone(), is_expanded))
                    .child(Label::new(label).size(LabelSize::Small)),
            )
            .child(h_flex().gap_2().map(|this| {
                if !self.is_staged {
                    this.child(
                        Button::new(
                            ElementId::Name(format!("{}-discard", self.id.clone()).into()),
                            "Discard All",
                        )
                        .style(ButtonStyle::Filled)
                        .layer(ui::ElevationIndex::ModalSurface)
                        .size(ButtonSize::Compact)
                        .label_size(LabelSize::Small)
                        .icon(IconName::X)
                        .icon_position(IconPosition::Start)
                        .icon_color(Color::Muted)
                        .disabled(status.changed_file_count() == 0)
                        .on_click(move |_, cx| cx.dispatch_action(Box::new(DiscardAll))),
                    )
                    .child(
                        Button::new(
                            ElementId::Name(format!("{}-stage", self.id.clone()).into()),
                            "Stage All",
                        )
                        .style(ButtonStyle::Filled)
                        .size(ButtonSize::Compact)
                        .label_size(LabelSize::Small)
                        .layer(ui::ElevationIndex::ModalSurface)
                        .icon(IconName::Check)
                        .icon_position(IconPosition::Start)
                        .icon_color(Color::Muted)
                        .disabled(status.no_unstaged())
                        .on_click(move |_, cx| cx.dispatch_action(Box::new(StageAll))),
                    )
                } else {
                    this.child(
                        Button::new(
                            ElementId::Name(format!("{}-unstage", self.id.clone()).into()),
                            "Unstage All",
                        )
                        .layer(ui::ElevationIndex::ModalSurface)
                        .icon(IconName::Check)
                        .icon_position(IconPosition::Start)
                        .icon_color(Color::Muted)
                        .disabled(status.no_staged())
                        .on_click(move |_, cx| cx.dispatch_action(Box::new(UnstageAll))),
                    )
                }
            }))
    }
}
{{REWRITTEN_CODE}}

#[derive(Debug, Clone)]
pub struct GitProjectStatusInner {
    pub unstaged_files: Vec<ChangedFile>,
    pub staged_files: Vec<ChangedFile>,
    pub lines_changed: GitLines,
    pub staged_expanded: bool,
    pub unstaged_expanded: bool,
    pub show_list: bool,
    pub selected_index: usize,
}

pub struct GitProjectStatus {
    inner: Model<GitProjectStatusInner>,
    list_state: ListState,
}

impl GitProjectStatus {
    fn new(changed_files: Vec<ChangedFile>, cx: &mut WindowContext) -> Self {
        let (unstaged_files, staged_files): (Vec<_>, Vec<_>) =
            changed_files.into_iter().partition(|f| !f.staged);

        let lines_changed = GitLines {
            added: unstaged_files.iter().chain(staged_files.iter()).map(|f| f.lines_added).sum(),
            removed: unstaged_files.iter().chain(staged_files.iter()).map(|f| f.lines_removed).sum(),
        };

        let inner = GitProjectStatusInner {
            unstaged_files,
            staged_files,
            lines_changed,
            staged_expanded: true,
            unstaged_expanded: true,
            show_list: false,
            selected_index: 0,
        };

        let model = cx.new_model(|_| inner);

        let mut this = Self {
            inner: model.clone(),
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.), |_, _| {
                div().into_any()
            }),
        };

        this.update_list_state(cx);
        this
    }

    fn update_list_state(&mut self, cx: &WindowContext) {
        let inner_clone = self.inner.clone();
        self.list_state = ListState::new(
            inner_clone.read(cx).total_item_count(),
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, cx| {
                let inner_ref = inner_clone.read(cx);

                if ix == 0 {
                    GitStagingControls::new(
                        "unstaged-controls",
                        inner_clone.clone(),
                        false,
                        ix == inner_ref.selected_index,
                    )
                    .into_any_element()
                } else if ix == inner_ref.total_item_count() - 1 {
                    GitStagingControls::new(
                        "staged-controls",
                        inner_clone.clone(),
                        true,
                        ix == inner_ref.selected_index,
                    )
                    .into_any_element()
                } else {
                    let file_ix = ix - 1;
                    let (file, is_staged) = if file_ix < inner_ref.unstaged_count() {
                        (&inner_ref.unstaged_files[file_ix], false)
                    } else {
                        (&inner_ref.staged_files[file_ix - inner_ref.unstaged_count()], true)
                    };

                    ChangedFileHeader::new(
                        ElementId::Name(format!("file-{}", file_ix).into()),
                        file.clone(),
                        ix == inner_ref.selected_index,
                        ix,
                        inner_clone.clone(),
                    )
                    .into_any_element()
                }
            },
        );
    }

    fn stage_file(&mut self, index: usize, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            if index < inner.unstaged_files.len() {
                let file = inner.unstaged_files.remove(index);
                inner.staged_files.push(file);
                inner.update_lines_changed();
            }
        });
        self.update_list_state(cx);
    }

    fn changed_file_count(&self, cx: &WindowContext) -> usize {
        let inner = self.inner.read(cx);
        inner.unstaged_files.len() + inner.staged_files.len()
    }

    fn unstaged_count(&self, cx: &WindowContext) -> usize {
        let inner = self.inner.read(cx);
        inner.unstaged_files.len()
    }

    fn staged_count(&self, cx: &WindowContext) -> usize {
        let inner = self.inner.read(cx);
        inner.staged_files.len()
    }

    fn total_item_count(&self, cx: &WindowContext) -> usize {
        self.changed_file_count(cx) + 2 // +2 for the two controls
    }

    fn no_unstaged(&self, cx: &WindowContext) -> bool {
        self.unstaged_count(cx) == 0
    }

    fn all_unstaged(&self, cx: &WindowContext) -> bool {
        self.staged_count(cx) == 0
    }

    fn no_staged(&self, cx: &WindowContext) -> bool {
        self.staged_count(cx) == 0
    }

    fn all_staged(&self, cx: &WindowContext) -> bool {
        self.unstaged_count(cx) == 0
    }

    fn discard_all(&mut self, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            inner.unstaged_files.clear();
            inner.staged_files.clear();
            inner.update_lines_changed();
        });
        self.update_list_state(cx);
    }

    fn stage_all(&mut self, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            inner.staged_files.extend(inner.unstaged_files.drain(..));
            inner.update_lines_changed();
        });
        self.update_list_state(cx);
    }

    fn unstage_all(&mut self, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            inner.unstaged_files.extend(inner.staged_files.drain(..));
            inner.update_lines_changed();
        });
        self.update_list_state(cx);
    }

    fn discard_selected(&mut self, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            let total_len = inner.unstaged_files.len() + inner.staged_files.len();
            if inner.selected_index > 0 && inner.selected_index <= total_len {
                if inner.selected_index <= inner.unstaged_files.len() {
                    inner.unstaged_files.remove(inner.selected_index - 1);
                } else {
                    inner.staged_files
                        .remove(inner.selected_index - 1 - inner.unstaged_files.len());
                }
                inner.update_lines_changed();
            }
        });
        self.update_list_state(cx);
    }

    fn stage_selected(&mut self, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            if inner.selected_index > 0 && inner.selected_index <= inner.unstaged_files.len() {
                let file = inner.unstaged_files.remove(inner.selected_index - 1);
                inner.staged_files.push(file);
                inner.update_lines_changed();
            }
        });
        self.update_list_state(cx);
    }

    fn unstage_selected(&mut self, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            let unstaged_len = inner.unstaged_files.len();
            if inner.selected_index > unstaged_len && inner.selected_index <= inner.total_item_count() - 2
            {
                let file = inner
                    .staged_files
                    .remove(inner.selected_index - 1 - unstaged_len);
                inner.unstaged_files.push(file);
                inner.update_lines_changed();
            }
        });
        self.update_list_state(cx);
    }

    fn unstage_file(&mut self, index: usize, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            if index < inner.staged_files.len() {
                let file = inner.staged_files.remove(index);
                inner.unstaged_files.push(file);
                inner.update_lines_changed();
            }
        });
        self.update_list_state(cx);
    }

    fn discard_file(&mut self, index: usize, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            if index < inner.unstaged_files.len() {
                inner.unstaged_files.remove(index);
                inner.update_lines_changed();
            } else {
                let staged_index = index - inner.unstaged_files.len();
                if staged_index < inner.staged_files.len() {
                    inner.staged_files.remove(staged_index);
                    inner.update_lines_changed();
                }
            }
        });
        self.update_list_state(cx);
    }

    fn set_selected_index(&mut self, index: usize, cx: &mut WindowContext) {
        self.inner.update(cx, |inner| {
            inner.selected_index = index;
        });
        self.update_list_state(cx);
    }
}

impl GitProjectStatusInner {
    fn update_lines_changed(&mut self) {
        self.lines_changed = GitLines {
            added: self
                .unstaged_files
                .iter()
                .chain(self.staged_files.iter())
                .map(|f| f.lines_added)
                .sum(),
            removed: self
                .unstaged_files
                .iter()
                .chain(self.staged_files.iter())
                .map(|f| f.lines_removed)
                .sum(),
        };
    }
}

impl EventEmitter<()> for ProjectStatusTab {}

impl FocusableView for ProjectStatusTab {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl workspace::Item for ProjectStatusTab {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(workspace::item::ItemEvent)) {}

    fn tab_content(&self, _params: TabContentParams, _cx: &WindowContext) -> AnyElement {
        Label::new("Project Status").into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn is_singleton(&self, _cx: &AppContext) -> bool {
        true
    }
}

pub struct GitStatusIndicator {
    active_editor: Option<WeakView<Editor>>,
    workspace: WeakView<Workspace>,
    current_status: Option<GitProjectStatus>,
    _observe_active_editor: Option<Subscription>,
}

impl Render for GitStatusIndicator {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex().h(rems(1.375)).gap_2().child(
            IconButton::new("git-status-indicator", IconName::GitBranch).on_click(cx.listener(
                |this, _, cx| {
                    if let Some(workspace) = this.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            ProjectStatusTab::deploy(workspace, &Default::default(), cx)
                        })
                    }
                },
            )),
        )
    }
}

impl GitStatusIndicator {
    pub fn new(workspace: &Workspace, _: &mut ViewContext<Self>) -> Self {
        Self {
            active_editor: None,
            workspace: workspace.weak_handle(),
            current_status: None,
            _observe_active_editor: None,
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for GitStatusIndicator {}

impl StatusItemView for GitStatusIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self.active_editor = Some(editor.downgrade());
        } else {
            self.active_editor = None;
            self.current_status = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}

fn static_changed_files() -> Vec<ChangedFile> {
    vec![
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file1".into(),
            lines_added: 10,
            lines_removed: 5,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file2".into(),
            lines_added: 8,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file3".into(),
            lines_added: 15,
            lines_removed: 20,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file4".into(),
            lines_added: 5,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file5".into(),
            lines_added: 12,
            lines_removed: 7,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file6".into(),
            lines_added: 0,
            lines_removed: 12,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file7".into(),
            lines_added: 7,
            lines_removed: 3,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file8".into(),
            lines_added: 2,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file9".into(),
            lines_added: 18,
            lines_removed: 15,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file10".into(),
            lines_added: 22,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file11".into(),
            lines_added: 5,
            lines_removed: 5,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file12".into(),
            lines_added: 7,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file13".into(),
            lines_added: 3,
            lines_removed: 11,
            status: GitFileStatus::Modified,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file14".into(),
            lines_added: 30,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        ChangedFile {
            staged: false,
            file_path: "path/to/changed_file15".into(),
            lines_added: 12,
            lines_removed: 22,
            status: GitFileStatus::Modified,
        },
    ]
}
