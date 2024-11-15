use git::repository::GitFileStatus;
use gpui::*;
use ui::{prelude::*, ElevationIndex, IconButtonShape};
use ui::{Disclosure, Divider};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::item::TabContentParams;
use workspace::Workspace;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<GitPanel>(cx);
        });
    })
    .detach();
}

actions!(
    git_panel,
    [
        Deploy,
        DiscardAll,
        StageAll,
        DiscardSelected,
        StageSelected,
        UnstageSelected,
        UnstageAll,
        FilesChanged,
        ToggleFocus
    ]
);

#[derive(Debug, Clone)]
pub struct PanelChangedFile {
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
pub struct PanelChangedFileHeader {
    id: ElementId,
    file: PanelChangedFile,
    is_selected: bool,
}

impl PanelChangedFileHeader {
    fn new(id: impl Into<ElementId>, file: PanelChangedFile, is_selected: bool) -> Self {
        Self {
            id: id.into(),
            file,
            is_selected,
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

impl RenderOnce for PanelChangedFileHeader {
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
                    .when(self.is_selected, |this| {
                        this.child(
                            IconButton::new("more-menu", IconName::EllipsisVertical)
                                .shape(IconButtonShape::Square)
                                .size(ButtonSize::Compact)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Muted),
                        )
                    })
                    .child(
                        IconButton::new("remove-file", IconName::X)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Error)
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::Background)
                            .on_click(move |_, cx| cx.dispatch_action(Box::new(DiscardSelected))),
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
                                if self.file.staged {
                                    cx.dispatch_action(Box::new(UnstageSelected))
                                } else {
                                    cx.dispatch_action(Box::new(StageSelected))
                                }
                            }),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct PanelGitProjectOverview {
    id: ElementId,
    project_status: Model<PanelGitProjectStatus>,
}

impl PanelGitProjectOverview {
    pub fn new(id: impl Into<ElementId>, project_status: Model<PanelGitProjectStatus>) -> Self {
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

impl RenderOnce for PanelGitProjectOverview {
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
pub struct PanelGitStagingControls {
    id: ElementId,
    project_status: Model<PanelGitProjectStatus>,
    is_staged: bool,
    is_selected: bool,
}

impl PanelGitStagingControls {
    pub fn new(
        id: impl Into<ElementId>,
        project_status: Model<PanelGitProjectStatus>,
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

impl RenderOnce for PanelGitStagingControls {
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

pub struct PanelGitProjectStatus {
    unstaged_files: Vec<PanelChangedFile>,
    staged_files: Vec<PanelChangedFile>,
    lines_changed: GitLines,
    staged_expanded: bool,
    unstaged_expanded: bool,
    show_list: bool,
    selected_index: usize,
}

impl PanelGitProjectStatus {
    fn new(changed_files: Vec<PanelChangedFile>) -> Self {
        let (unstaged_files, staged_files): (Vec<_>, Vec<_>) =
            changed_files.into_iter().partition(|f| !f.staged);

        let lines_changed = GitLines {
            added: unstaged_files
                .iter()
                .chain(staged_files.iter())
                .map(|f| f.lines_added)
                .sum(),
            removed: unstaged_files
                .iter()
                .chain(staged_files.iter())
                .map(|f| f.lines_removed)
                .sum(),
        };

        Self {
            unstaged_files,
            staged_files,
            lines_changed,
            staged_expanded: true,
            unstaged_expanded: true,
            show_list: false,
            selected_index: 0,
        }
    }

    fn changed_file_count(&self) -> usize {
        self.unstaged_files.len() + self.staged_files.len()
    }

    fn unstaged_count(&self) -> usize {
        self.unstaged_files.len()
    }

    fn staged_count(&self) -> usize {
        self.staged_files.len()
    }

    fn total_item_count(&self) -> usize {
        self.changed_file_count() + 2 // +2 for the two controls
    }

    fn no_unstaged(&self) -> bool {
        self.unstaged_files.is_empty()
    }

    fn all_unstaged(&self) -> bool {
        self.staged_files.is_empty()
    }

    fn no_staged(&self) -> bool {
        self.staged_files.is_empty()
    }

    fn all_staged(&self) -> bool {
        self.unstaged_files.is_empty()
    }

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

    fn discard_all(&mut self) {
        self.unstaged_files.clear();
        self.staged_files.clear();
        self.update_lines_changed();
    }

    fn stage_all(&mut self) {
        self.staged_files.extend(self.unstaged_files.drain(..));
        self.update_lines_changed();
    }

    fn unstage_all(&mut self) {
        self.unstaged_files.extend(self.staged_files.drain(..));
        self.update_lines_changed();
    }

    fn discard_selected(&mut self) {
        let total_len = self.unstaged_files.len() + self.staged_files.len();
        if self.selected_index > 0 && self.selected_index <= total_len {
            if self.selected_index <= self.unstaged_files.len() {
                self.unstaged_files.remove(self.selected_index - 1);
            } else {
                self.staged_files
                    .remove(self.selected_index - 1 - self.unstaged_files.len());
            }
            self.update_lines_changed();
        }
    }

    fn stage_selected(&mut self) {
        if self.selected_index > 0 && self.selected_index <= self.unstaged_files.len() {
            let file = self.unstaged_files.remove(self.selected_index - 1);
            self.staged_files.push(file);
            self.update_lines_changed();
        }
    }

    fn unstage_selected(&mut self) {
        let unstaged_len = self.unstaged_files.len();
        if self.selected_index > unstaged_len && self.selected_index <= self.total_item_count() - 2
        {
            let file = self
                .staged_files
                .remove(self.selected_index - 1 - unstaged_len);
            self.unstaged_files.push(file);
            self.update_lines_changed();
        }
    }
}

#[derive(Clone)]
pub struct GitPanel {
    id: ElementId,
    focus_handle: FocusHandle,
    status: Model<PanelGitProjectStatus>,
    list_state: ListState,
    width: Option<Pixels>,
}

impl GitPanel {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        let changed_files = static_changed_files();
        let status = cx.new_model(|_| PanelGitProjectStatus::new(changed_files));

        let status_clone = status.clone();
        let list_state = ListState::new(
            status.read(cx).total_item_count(),
            gpui::ListAlignment::Top,
            px(10.),
            move |ix, cx| {
                let status = status_clone.read(cx);
                let is_selected = status.selected_index == ix;
                if ix == 0 {
                    PanelGitStagingControls::new(
                        "unstaged-controls",
                        status_clone.clone(),
                        false,
                        is_selected,
                    )
                    .into_any_element()
                } else if ix == status.total_item_count() - 1 {
                    PanelGitStagingControls::new(
                        "staged-controls",
                        status_clone.clone(),
                        true,
                        is_selected,
                    )
                    .into_any_element()
                } else {
                    let file_ix = ix - 1;
                    let file = if file_ix < status.unstaged_count() {
                        status.unstaged_files.get(file_ix)
                    } else {
                        status.staged_files.get(file_ix - status.unstaged_count())
                    };

                    file.map(|file| {
                        PanelChangedFileHeader::new(
                            ElementId::Name(format!("file-{}", file_ix).into()),
                            file.clone(),
                            is_selected,
                        )
                        .into_any_element()
                    })
                    .unwrap_or_else(|| div().into_any_element())
                }
            },
        );

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            status,
            list_state,
            width: Some(px(400.).into()),
        }
    }

    fn recreate_list_state(&mut self, cx: &mut ViewContext<Self>) {
        let status = self.status.read(cx);
        let status_clone = self.status.clone();

        self.list_state = ListState::new(
            status.total_item_count(),
            gpui::ListAlignment::Top,
            px(10.),
            move |ix, cx| {
                let is_selected = status_clone.read(cx).selected_index == ix;
                if ix == 0 {
                    PanelGitStagingControls::new(
                        "unstaged-controls",
                        status_clone.clone(),
                        false,
                        is_selected,
                    )
                    .into_any_element()
                } else if ix == status_clone.read(cx).total_item_count() - 1 {
                    PanelGitStagingControls::new(
                        "staged-controls",
                        status_clone.clone(),
                        true,
                        is_selected,
                    )
                    .into_any_element()
                } else {
                    let file_ix = ix - 1;
                    let status = status_clone.read(cx);
                    let file = if file_ix < status.unstaged_count() {
                        status.unstaged_files.get(file_ix)
                    } else {
                        status.staged_files.get(file_ix - status.unstaged_count())
                    };

                    file.map(|file| {
                        PanelChangedFileHeader::new(
                            ElementId::Name(format!("file-{}", file_ix).into()),
                            file.clone(),
                            is_selected,
                        )
                        .into_any_element()
                    })
                    .unwrap_or_else(|| div().into_any_element())
                }
            },
        );
    }

    fn discard_all(&mut self, _: &DiscardAll, cx: &mut ViewContext<Self>) {
        self.status.update(cx, |status, _| {
            status.discard_all();
        });
        self.recreate_list_state(cx);
        cx.notify();
    }

    fn stage_all(&mut self, _: &StageAll, cx: &mut ViewContext<Self>) {
        self.status.update(cx, |status, _| {
            status.stage_all();
        });
        self.recreate_list_state(cx);
        cx.notify();
    }

    fn unstage_all(&mut self, _: &UnstageAll, cx: &mut ViewContext<Self>) {
        self.status.update(cx, |status, _| {
            status.unstage_all();
        });
        self.recreate_list_state(cx);
        cx.notify();
    }

    fn discard_selected(&mut self, _: &DiscardSelected, cx: &mut ViewContext<Self>) {
        self.status.update(cx, |status, _| {
            status.discard_selected();
        });
        self.recreate_list_state(cx);
        cx.notify();
    }

    fn stage_selected(&mut self, _: &StageSelected, cx: &mut ViewContext<Self>) {
        self.status.update(cx, |status, _| {
            status.stage_selected();
        });
        self.recreate_list_state(cx);
        cx.notify();
    }

    fn unstage_selected(&mut self, _: &UnstageSelected, cx: &mut ViewContext<Self>) {
        self.status.update(cx, |status, _| {
            status.unstage_selected();
        });
        self.recreate_list_state(cx);
        cx.notify();
    }

    fn selected_index(&self, cx: &WindowContext) -> usize {
        self.status.read(cx).selected_index
    }

    pub fn set_selected_index(
        &mut self,
        index: usize,
        jump_to_index: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.status.update(cx, |status, _| {
            status.selected_index = index.min(status.total_item_count() - 1);
        });

        if jump_to_index {
            self.jump_to_cell(index, cx);
        }
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let current_index = self.status.read(cx).selected_index;
        let total_count = self.status.read(cx).total_item_count();
        let new_index = (current_index + 1).min(total_count - 1);
        self.set_selected_index(new_index, true, cx);
        cx.notify();
    }

    pub fn select_previous(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let current_index = self.status.read(cx).selected_index;
        let new_index = current_index.saturating_sub(1);
        self.set_selected_index(new_index, true, cx);
        cx.notify();
    }

    pub fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        self.set_selected_index(0, true, cx);
        cx.notify();
    }

    pub fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let total_count = self.status.read(cx).total_item_count();
        self.set_selected_index(total_count - 1, true, cx);
        cx.notify();
    }

    fn jump_to_cell(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        self.list_state.scroll_to_reveal_item(index);
    }
}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let project_status = self.status.read(cx);

        h_flex()
            .id(self.id.clone())
            .key_context("vcs_status")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::discard_all))
            .on_action(cx.listener(Self::stage_all))
            .on_action(cx.listener(Self::unstage_all))
            .on_action(cx.listener(Self::discard_selected))
            .on_action(cx.listener(Self::stage_selected))
            .on_action(cx.listener(Self::unstage_selected))
            .on_action(cx.listener(|this, &FilesChanged, cx| this.recreate_list_state(cx)))
            .flex_1()
            .size_full()
            .overflow_hidden()
            .child(
                v_flex()
                    .h_full()
                    .flex_1()
                    .overflow_hidden()
                    .bg(ElevationIndex::Surface.bg(cx))
                    .child(list(self.list_state.clone()).size_full())
                    .child(Divider::horizontal_dashed())
                    .child(PanelGitProjectOverview::new(
                        "project-overview",
                        self.status.clone(),
                    ))
                    .child(div()),
            )
    }
}

impl workspace::Item for GitPanel {
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

impl EventEmitter<()> for GitPanel {}
impl EventEmitter<PanelEvent> for GitPanel {}

impl FocusableView for GitPanel {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for GitPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {}

    fn size(&self, cx: &gpui::WindowContext) -> Pixels {
        self.width.unwrap_or(px(400.))
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        // self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &gpui::WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "GitPanel"
    }
}

fn static_changed_files() -> Vec<PanelChangedFile> {
    vec![
        PanelChangedFile {
            staged: true,
            file_path: "src/main.rs".into(),
            lines_added: 20,
            lines_removed: 6,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: false,
            file_path: "src/lib.rs".into(),
            lines_added: 12,
            lines_removed: 2,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: false,
            file_path: "Cargo.toml".into(),
            lines_added: 1,
            lines_removed: 0,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: true,
            file_path: "README.md".into(),
            lines_added: 5,
            lines_removed: 0,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: false,
            file_path: "src/utils/helpers.rs".into(),
            lines_added: 8,
            lines_removed: 10,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: false,
            file_path: "tests/integration_test.rs".into(),
            lines_added: 25,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        PanelChangedFile {
            staged: false,
            file_path: "src/models/user.rs".into(),
            lines_added: 14,
            lines_removed: 3,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: true,
            file_path: "src/services/auth.rs".into(),
            lines_added: 0,
            lines_removed: 4,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: false,
            file_path: "build.rs".into(),
            lines_added: 7,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
    ]
}
