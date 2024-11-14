use editor::Editor;
use git::repository::GitFileStatus;
use gpui::*;
use smallvec::{smallvec, SmallVec};
use ui::{prelude::*, ElevationIndex, IconButtonShape};
use ui::{Disclosure, Divider};
use workspace::item::TabContentParams;
use workspace::{item::ItemHandle, StatusItemView, ToolbarItemEvent, Workspace};

actions!(vcs_status, [Deploy]);

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

pub struct GitLines {
    pub added: usize,
    pub removed: usize,
}

#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub staged: bool,
    pub file_path: SharedString,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub status: GitFileStatus,
}

pub struct GitProjectStatus {
    file_count: usize,
    unstaged_count: usize,
    staged_count: usize,
    lines_changed: GitLines,
    changed_files: Vec<ChangedFile>,
    staged_expanded: bool,
    unstaged_expanded: bool,
    show_list: bool,
    selected_index: usize,
}

impl GitProjectStatus {
    fn new(changed_files: Vec<ChangedFile>) -> Self {
        let file_count = changed_files.len();
        let unstaged_count = changed_files.iter().filter(|f| !f.staged).count();
        let staged_count = file_count - unstaged_count;

        let lines_changed = GitLines {
            added: changed_files.iter().map(|f| f.lines_added).sum(),
            removed: changed_files.iter().map(|f| f.lines_removed).sum(),
        };

        Self {
            file_count,
            unstaged_count,
            staged_count,
            lines_changed,
            changed_files,
            unstaged_expanded: true,
            staged_expanded: false,
            show_list: false,
            selected_index: 0,
        }
    }
}

#[derive(IntoElement)]
pub struct ChangedFileHeader {
    id: ElementId,
    changed_file: ChangedFile,
    is_selected: bool,
}

impl ChangedFileHeader {
    fn new(id: impl Into<ElementId>, changed_file: ChangedFile, is_selected: bool) -> Self {
        Self {
            id: id.into(),
            changed_file,
            is_selected,
        }
    }

    fn icon_for_status(&self) -> impl IntoElement {
        let (icon_name, color) = match self.changed_file.status {
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

        let file_path = self.changed_file.file_path.clone();

        h_flex()
            .id(self.id.clone())
            .justify_between()
            .w_full()
            .hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
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
                            .when(self.changed_file.lines_added > 0, |this| {
                                this.child(
                                    Label::new(format!("+{}", self.changed_file.lines_added))
                                        .color(Color::Created)
                                        .size(LabelSize::Small),
                                )
                            })
                            .when(self.changed_file.lines_removed > 0, |this| {
                                this.child(
                                    Label::new(format!("-{}", self.changed_file.lines_removed))
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
                            .layer(ElevationIndex::Background),
                    )
                    .child(
                        IconButton::new("check-file", IconName::Check)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Accent)
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::Background),
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
        let id = id.into();

        Self { id, project_status }
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

        let id = self.id.clone();
        let changed_files: SharedString = format!("{} Changed files", status.file_count).into();

        let added_label: Option<SharedString> = if status.lines_changed.added > 0 {
            Some(format!("+{}", status.lines_changed.added).into())
        } else {
            None
        };
        let removed_label: Option<SharedString> = if status.lines_changed.removed > 0 {
            Some(format!("-{}", status.lines_changed.removed).into())
        } else {
            None
        };
        let total_label: SharedString = "total lines changed".into();

        h_flex()
            .id(id.clone())
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

// #[derive(IntoElement)]
// pub struct GitStagingControls {
//     id: ElementId,
//     project_status: Model<GitProjectStatus>,
//     is_staged: bool,
//     children: SmallVec<[AnyElement; 2]>,
// }

// impl GitStagingControls {
//     pub fn staged(id: impl Into<ElementId>, project_status: Model<GitProjectStatus>) -> Self {
//         let id = id.into();

//         Self {
//             id,
//             project_status,
//             is_staged: true,
//             children: SmallVec::new(),
//         }
//     }

//     pub fn unstaged(id: impl Into<ElementId>, project_status: Model<GitProjectStatus>) -> Self {
//         let id = id.into();

//         Self {
//             id,
//             project_status,
//             is_staged: false,
//             children: SmallVec::new(),
//         }
//     }
// }

// impl ParentElement for GitStagingControls {
//     fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
//         self.children.extend(elements)
//     }
// }

// impl RenderOnce for GitStagingControls {
//     fn render(self, cx: &mut WindowContext) -> impl IntoElement {
//         let status = self.project_status.read(cx);

//         // TODO
//         let (staging_type, count) = if self.is_staged {
//             ("Staged", status.staged_count)
//         } else {
//             ("Unstaged", status.unstaged_count)
//         };

//         // TODO: Convert staging_type to an enum so we don't have to do this
//         let is_expanded = match staging_type {
//             "Staged" => status.staged_expanded,
//             "Unstaged" => status.unstaged_expanded,
//             _ => false,
//         };

//         let changed_files: Vec<ChangedFile> = status
//             .changed_files
//             .iter()
//             .filter(|file| file.staged == (staging_type == "Staged"))
//             .cloned()
//             .collect();

//         let label: SharedString = format!("{} Changes: {}", staging_type, count).into();

//         let id = self.id.clone();
//         let container_id = ElementId::Name(format!("{}-container", id.clone()).into());
//         let project_status = self.project_status.clone();

//         v_flex()
//             .when(is_expanded, |this| this.flex_1().h_full())
//             .child(
//                 h_flex()
//                     .id(id.clone())
//                     .hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
//                     .on_click(move |_, cx| {
//                         project_status.update(cx, |status, cx| {
//                             if self.is_staged {
//                                 status.staged_expanded = !status.staged_expanded;
//                             } else {
//                                 status.unstaged_expanded = !status.unstaged_expanded;
//                             }
//                             cx.notify();
//                         })
//                     })
//                     .flex_none()
//                     .justify_between()
//                     .w_full()
//                     .bg(cx.theme().colors().elevated_surface_background)
//                     .px_3()
//                     .py_2()
//                     .child(
//                         h_flex()
//                             .gap_2()
//                             .child(Disclosure::new("staging-control", is_expanded).on_click(
//                                 move |_, cx| {
//                                     self.project_status.clone().update(cx, |status, cx| {
//                                         if self.is_staged {
//                                             status.staged_expanded = !status.staged_expanded;
//                                         } else {
//                                             status.unstaged_expanded = !status.unstaged_expanded;
//                                         }
//                                         cx.notify();
//                                     })
//                                 },
//                             ))
//                             .child(Label::new(label).size(LabelSize::Small)),
//                     )
//                     .child(h_flex().gap_2().map(|this| {
//                         if !self.is_staged {
//                             this.child(
//                                 Button::new(
//                                     ElementId::Name(format!("{}-discard", id.clone()).into()),
//                                     "Discard All",
//                                 )
//                                 .style(ButtonStyle::Filled)
//                                 .layer(ui::ElevationIndex::ModalSurface)
//                                 .size(ButtonSize::Compact)
//                                 .label_size(LabelSize::Small)
//                                 .icon(IconName::X)
//                                 .icon_position(IconPosition::Start)
//                                 .icon_color(Color::Muted),
//                             )
//                             .child(
//                                 Button::new(
//                                     ElementId::Name(format!("{}-unstage", id.clone()).into()),
//                                     "Stage All",
//                                 )
//                                 .style(ButtonStyle::Filled)
//                                 .size(ButtonSize::Compact)
//                                 .label_size(LabelSize::Small)
//                                 .layer(ui::ElevationIndex::ModalSurface)
//                                 .icon(IconName::Check)
//                                 .icon_position(IconPosition::Start)
//                                 .icon_color(Color::Muted),
//                             )
//                         } else {
//                             this.child(
//                                 Button::new(
//                                     ElementId::Name(format!("{}-stage", id.clone()).into()),
//                                     "Stage All",
//                                 )
//                                 .layer(ui::ElevationIndex::ModalSurface)
//                                 .icon(IconName::Check)
//                                 .icon_position(IconPosition::Start)
//                                 .icon_color(Color::Muted),
//                             )
//                         }
//                     })),
//             )
//             .when(is_expanded, |this| {
//                 this.child(
//                     v_flex()
//                         .id(container_id)
//                         .flex_1()
//                         .overflow_x_scroll()
//                         .bg(cx.theme().colors().editor_background)
//                         .px_1()
//                         .py_1p5()
//                         .gap_1()
//                         .when(changed_files.is_empty(), |this| {
//                             this.child(
//                                 v_flex()
//                                     .w_full()
//                                     .flex_1()
//                                     .items_center()
//                                     .justify_center()
//                                     .child(
//                                         div()
//                                             .flex_none()
//                                             .mx_auto()
//                                             .text_ui_sm(cx)
//                                             .text_color(Color::Muted.color(cx))
//                                             .child(format!("No {} changes", staging_type)),
//                                     ),
//                             )
//                         })
//                         .when(!changed_files.is_empty(), |this| {
//                             let mut children: SmallVec<[AnyElement; 8]> = smallvec![];
//                             let mut iter = changed_files.iter().peekable();

//                             while let Some(file) = iter.next() {
//                                 let file_id =
//                                     ElementId::Name(format!("{}-file", file.file_path).into());
//                                 children.push(
//                                     ChangedFileHeader::new(file_id, file.clone())
//                                         .into_any_element(),
//                                 );

//                                 if iter.peek().is_some() {
//                                     children.push(Divider::horizontal().into_any_element());
//                                 }
//                             }

//                             this.children(children)
//                         }),
//                 )
//             })
//     }
// }

#[derive(Clone)]
pub struct ProjectStatusTab {
    id: ElementId,
    focus_handle: FocusHandle,
    status: Model<GitProjectStatus>,
    list_state: ListState,
}

impl ProjectStatusTab {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        let changed_files = static_changed_files();
        let status = cx.new_model(|_| GitProjectStatus::new(changed_files.clone()));

        let status_clone = status.clone();
        let list_state = ListState::new(
            changed_files.len(),
            gpui::ListAlignment::Top,
            px(10.),
            move |ix, cx| {
                let is_selected = status_clone.read(cx).selected_index == ix;
                if let Some(changed_file) = changed_files.get(ix) {
                    ChangedFileHeader::new(
                        ElementId::Name(format!("file-{}", ix).into()),
                        changed_file.clone(),
                        is_selected,
                    )
                    .into_any_element()
                } else {
                    div().into_any_element()
                }
            },
        );

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            status,
            list_state,
        }
    }
}

impl ProjectStatusTab {
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        if let Some(existing) = workspace.item_of_type::<ProjectStatusTab>(cx) {
            workspace.activate_item(&existing, true, true, cx);
        } else {
            let status_tab = cx.new_view(|cx| Self::new("project-status-tab", cx));
            workspace.add_item_to_active_pane(Box::new(status_tab), None, true, cx);
        }
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
            status.selected_index = index;
        });

        if jump_to_index {
            self.jump_to_cell(index, cx);
        }
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let count = self.list_state.item_count();
        if count > 0 {
            let index = self.selected_index(cx);
            let ix = if index == count - 1 {
                count - 1
            } else {
                index + 1
            };
            self.set_selected_index(ix, true, cx);
            cx.notify();
        }
    }

    pub fn select_previous(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let count = self.list_state.item_count();
        if count > 0 {
            let index = self.selected_index(cx);
            let ix = if index == 0 { 0 } else { index - 1 };
            self.set_selected_index(ix, true, cx);
            cx.notify();
        }
    }

    pub fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        let count = self.list_state.item_count();
        if count > 0 {
            self.set_selected_index(0, true, cx);
            cx.notify();
        }
    }

    pub fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let count = self.list_state.item_count();
        if count > 0 {
            self.set_selected_index(count - 1, true, cx);
            cx.notify();
        }
    }

    fn jump_to_cell(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        self.list_state.scroll_to_reveal_item(index);
    }
}

impl Render for ProjectStatusTab {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let id = self.id.clone();

        let project_status = self.status.read(cx);

        h_flex()
            .key_context("vcs_status")
            .track_focus(&self.focus_handle)
            .id(id)
            .flex_1()
            .size_full()
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .overflow_hidden()
            .when(project_status.show_list, |this| {
                this.child(
                    v_flex()
                        .bg(ElevationIndex::Surface.bg(cx))
                        .border_r_1()
                        .border_color(cx.theme().colors().border)
                        .w(px(280.))
                        .flex_none()
                        .h_full()
                        .child("sidebar"),
                )
            })
            .child(
                v_flex()
                    .h_full()
                    .flex_1()
                    .overflow_hidden()
                    .bg(ElevationIndex::Surface.bg(cx))
                    .child(GitProjectOverview::new(
                        "project-overview",
                        self.status.clone(),
                    ))
                    .child(Divider::horizontal_dashed())
                    .child(list(self.list_state.clone()).size_full()),
            )
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
            // self._observe_active_editor = Some(cx.observe(&editor, Self::update_status));
            // self.update_status(editor, cx);
        } else {
            self.active_editor = None;
            self.current_status = None;
            self._observe_active_editor = None;
        }
        cx.notify();
    }

    // fn update_status(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
    //     let current_status = GitProjectStatus::default();
    //     if current_status != self.current_status {
    //         self.current_status = Some(current_status);
    //         cx.notify();
    //     }
    // }
}
