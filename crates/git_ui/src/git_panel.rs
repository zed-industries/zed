use git::repository::GitFileStatus;
use gpui::*;
use std::collections::{BTreeMap, HashMap};
use ui::{prelude::*, Checkbox, ElevationIndex, IconButtonShape};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChangedFileId(SharedString);

#[derive(Debug, Clone)]
pub struct PanelChangedFile {
    pub staged: bool,
    pub file_path: SharedString,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub status: GitFileStatus,
}

#[derive(Clone)]
pub struct GitLines {
    pub added: usize,
    pub removed: usize,
}

struct FileTree {
    root: BTreeMap<String, FileTreeNode>,
}

enum FileTreeNode {
    File(ChangedFileId),
    Directory(BTreeMap<String, FileTreeNode>),
}

enum GitListItem {
    Header(bool),
    Divider,
    Directory {
        path: String,
        indent_level: usize,
        is_expanded: bool,
    },
    File {
        id: ChangedFileId,
        indent_level: usize,
    },
}

pub struct GitPanelState {
    files: HashMap<ChangedFileId, PanelChangedFile>,
    file_order: Vec<ChangedFileId>,
    file_tree: FileTree,
    lines_changed: GitLines,
    show_list: bool,
    selected_index: usize,
    needs_update: bool,
}

impl GitPanelState {
    fn new(changed_files: Vec<PanelChangedFile>) -> Self {
        let mut files = HashMap::new();
        let mut file_order = Vec::new();
        let mut lines_changed = GitLines {
            added: 0,
            removed: 0,
        };

        for file in changed_files {
            let id = ChangedFileId(file.file_path.clone());
            file_order.push(id.clone());
            lines_changed.added += file.lines_added;
            lines_changed.removed += file.lines_removed;
            files.insert(id, file);
        }

        let file_tree = FileTree::new(&files);

        Self {
            files,
            file_order,
            file_tree,
            lines_changed,
            show_list: true,
            selected_index: 0,
            needs_update: true,
        }
    }

    fn generate_git_list_items(&self) -> Vec<GitListItem> {
        let mut items = Vec::new();

        // Unstaged header
        items.push(GitListItem::Header(false));

        // Unstaged files
        for (id, file) in &self.files {
            if !file.staged {
                items.push(GitListItem::File {
                    id: id.clone(),
                    indent_level: 0,
                });
            }
        }

        items.push(GitListItem::Divider);

        // Staged header
        items.push(GitListItem::Header(true));

        // Staged files
        for (id, file) in &self.files {
            if file.staged {
                items.push(GitListItem::File {
                    id: id.clone(),
                    indent_level: 0,
                });
            }
        }

        items
    }

    fn count_files_in_directory(&self, path: &str) -> usize {
        self.file_tree
            .root
            .iter()
            .filter(|(dir_path, node)| {
                dir_path.starts_with(path) && matches!(node, FileTreeNode::File(_))
            })
            .count()
    }

    fn directory_selection(&self, path: &str) -> Selection {
        let mut all_staged = true;
        let mut all_unstaged = true;

        for (_, file) in &self.files {
            if file.file_path.starts_with(path) {
                if file.staged {
                    all_unstaged = false;
                } else {
                    all_staged = false;
                }

                if !all_staged && !all_unstaged {
                    break;
                }
            }
        }

        if all_staged {
            Selection::Selected
        } else if all_unstaged {
            Selection::Unselected
        } else {
            Selection::Indeterminate
        }
    }

    fn changed_file_count(&self) -> usize {
        self.files.len()
    }

    fn unstaged_count(&self) -> usize {
        self.files.values().filter(|f| !f.staged).count()
    }

    fn staged_count(&self) -> usize {
        self.files.values().filter(|f| f.staged).count()
    }

    fn total_item_count(&self) -> usize {
        self.changed_file_count() + 2 // +2 for the two headers
    }

    fn no_unstaged(&self) -> bool {
        self.unstaged_count() == 0
    }

    fn all_unstaged(&self) -> bool {
        self.staged_count() == 0
    }

    fn no_staged(&self) -> bool {
        self.staged_count() == 0
    }

    fn all_staged(&self) -> bool {
        self.unstaged_count() == 0
    }

    fn update_lines_changed(&mut self) {
        self.lines_changed = GitLines {
            added: self.files.values().map(|f| f.lines_added).sum(),
            removed: self.files.values().map(|f| f.lines_removed).sum(),
        };
    }

    fn discard_all(&mut self) {
        self.files.clear();
        self.file_order.clear();
        self.update_lines_changed();
        self.file_tree = FileTree::new(&self.files);
    }

    fn stage_all(&mut self) {
        for file in self.files.values_mut() {
            file.staged = true;
        }
        self.file_tree = FileTree::new(&self.files);
    }

    fn unstage_all(&mut self) {
        for file in self.files.values_mut() {
            file.staged = false;
        }
        self.file_tree = FileTree::new(&self.files);
    }

    fn discard_selected(&mut self) {
        if self.selected_index > 0 && self.selected_index <= self.file_order.len() {
            let id = &self.file_order[self.selected_index - 1];
            self.files.remove(id);
            self.file_order.remove(self.selected_index - 1);
            self.update_lines_changed();
            self.file_tree = FileTree::new(&self.files);
        }
    }

    fn stage_selected(&mut self) {
        if self.selected_index > 0 && self.selected_index <= self.file_order.len() {
            let id = &self.file_order[self.selected_index - 1];
            if let Some(file) = self.files.get_mut(id) {
                file.staged = true;
            }
            self.file_tree = FileTree::new(&self.files);
        }
    }

    fn unstage_selected(&mut self) {
        if self.selected_index > 0 && self.selected_index <= self.file_order.len() {
            let id = &self.file_order[self.selected_index - 1];
            if let Some(file) = self.files.get_mut(id) {
                file.staged = false;
            }
            self.file_tree = FileTree::new(&self.files);
        }
    }

    fn generate_changed_file_items(&self) -> Vec<GitListItem> {
        let mut items = Vec::new();
        items.push(GitListItem::Header(false)); // Unstaged header
                                                // Add unstaged files and directories
        items.push(GitListItem::Divider);
        items.push(GitListItem::Header(true)); // Staged header
                                               // Add staged files and directories
                                               // This part needs to be implemented based on the file_tree structure
        items
    }
}

impl FileTree {
    fn new(files: &HashMap<ChangedFileId, PanelChangedFile>) -> Self {
        let mut root = BTreeMap::new();
        for (id, file) in files {
            let path_parts: Vec<&str> = file.file_path.split('/').collect();
            Self::insert_into_tree(&mut root, &path_parts, id);
        }
        Self { root }
    }

    fn insert_into_tree(
        node: &mut BTreeMap<String, FileTreeNode>,
        path_parts: &[&str],
        file_id: &ChangedFileId,
    ) {
        if path_parts.is_empty() {
            return;
        }

        let current = path_parts[0];
        if path_parts.len() == 1 {
            node.insert(current.to_string(), FileTreeNode::File(file_id.clone()));
        } else {
            let entry = node
                .entry(current.to_string())
                .or_insert_with(|| FileTreeNode::Directory(BTreeMap::new()));
            if let FileTreeNode::Directory(ref mut child_map) = entry {
                Self::insert_into_tree(child_map, &path_parts[1..], file_id);
            }
        }
    }
}

#[derive(IntoElement)]
pub struct ChangedFileItem {
    id: ElementId,
    file: PanelChangedFile,
    is_selected: bool,
    indent_level: usize,
}

impl ChangedFileItem {
    fn new(
        id: impl Into<ElementId>,
        file: PanelChangedFile,
        is_selected: bool,
        indent_level: usize,
    ) -> Self {
        Self {
            id: id.into(),
            file,
            is_selected,
            indent_level,
        }
    }
}

impl RenderOnce for ChangedFileItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let file_path = self.file.file_path.clone();

        let (icon_name, color) = match self.file.status {
            GitFileStatus::Added => (
                IconName::SquarePlus,
                Color::Custom(hsla(142. / 360., 0.68, 0.45, 1.0)),
            ),
            GitFileStatus::Modified => (
                IconName::SquareDot,
                Color::Custom(hsla(48. / 360., 0.76, 0.47, 1.0)),
            ),
            GitFileStatus::Conflict => (
                IconName::SquareMinus,
                Color::Custom(hsla(355. / 360., 0.65, 0.65, 1.0)),
            ),
        };

        let is_deleted = self.file.status == GitFileStatus::Conflict;

        h_flex()
            .id(self.id.clone())
            .items_center()
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
            .pl(px(12. + (self.indent_level as f32 * 16.)))
            .h(px(24.))
            .child(
                h_flex()
                    .gap(px(8.))
                    .child(Checkbox::new(self.id.clone(), self.file.staged.into()))
                    .child(Icon::new(icon_name).size(IconSize::Small).color(color))
                    .child(
                        Label::new(file_path)
                            .strikethrough(is_deleted)
                            .size(LabelSize::Small)
                            .color(if is_deleted {
                                Color::Placeholder
                            } else {
                                Color::Default
                            }),
                    )
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
                    .invisible()
                    .group_hover("", |this| this.visible())
                    .child(
                        IconButton::new("more-menu", IconName::EllipsisVertical)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Muted),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct DirectoryItem {
    id: ElementId,
    path: String,
    file_count: usize,
    is_selected: bool,
    is_expanded: bool,
    indent_level: usize,
    selection: Selection,
}

impl DirectoryItem {
    fn new(
        id: impl Into<ElementId>,
        path: String,
        file_count: usize,
        is_selected: bool,
        is_expanded: bool,
        indent_level: usize,
        selection: Selection,
    ) -> Self {
        Self {
            id: id.into(),
            path,
            file_count,
            is_selected,
            is_expanded,
            indent_level,
            selection,
        }
    }
}

impl RenderOnce for DirectoryItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let file_count_label = format!(
            "{} file{}",
            self.file_count,
            if self.file_count == 1 { "" } else { "s" }
        );

        h_flex()
            .id(self.id.clone())
            .items_center()
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
            .h(px(24.))
            .child(
                h_flex()
                    .pl(px(12. + (self.indent_level as f32 * 16.)))
                    .gap(px(8.))
                    .child(Checkbox::new(self.id.clone(), self.selection))
                    .child(
                        Icon::new(if self.is_expanded {
                            IconName::ChevronDown
                        } else {
                            IconName::ChevronRight
                        })
                        .size(IconSize::Small),
                    )
                    .child(
                        Icon::new(IconName::Folder)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new(self.path).size(LabelSize::Small))
                    .child(
                        Label::new(file_count_label)
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .invisible()
                    .group_hover("", |this| this.visible())
                    .child(
                        IconButton::new("more-menu", IconName::EllipsisVertical)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(Color::Muted),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct ProjectOverviewItem {
    id: ElementId,
    changed_file_count: usize,
    lines_changed: GitLines,
}

impl ProjectOverviewItem {
    pub fn new(
        id: impl Into<ElementId>,
        changed_file_count: usize,
        lines_changed: GitLines,
    ) -> Self {
        Self {
            id: id.into(),
            changed_file_count,
            lines_changed,
        }
    }
}

impl RenderOnce for ProjectOverviewItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let changed_files: SharedString =
            format!("{} Changed files", self.changed_file_count).into();

        let added_label: Option<SharedString> =
            (self.lines_changed.added > 0).then(|| format!("+{}", self.lines_changed.added).into());
        let removed_label: Option<SharedString> = (self.lines_changed.removed > 0)
            .then(|| format!("-{}", self.lines_changed.removed).into());
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
pub struct StagingHeaderItem {
    id: ElementId,
    is_staged: bool,
    count: usize,
    is_selected: bool,
    is_expanded: bool,
    no_changes: bool,
}

impl StagingHeaderItem {
    pub fn new(
        id: impl Into<ElementId>,
        is_staged: bool,
        count: usize,
        is_selected: bool,
        is_expanded: bool,
        no_changes: bool,
    ) -> Self {
        Self {
            id: id.into(),
            is_staged,
            count,
            is_selected,
            is_expanded,
            no_changes,
        }
    }
}

impl RenderOnce for StagingHeaderItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let staging_type = if self.is_staged { "Staged" } else { "Unstaged" };
        let label: SharedString = format!("{} Changes: {}", staging_type, self.count).into();

        h_flex()
            .id(self.id.clone())
            .hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
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
                    .child(Disclosure::new(self.id.clone(), self.is_expanded))
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
                        .disabled(self.no_changes)
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
                        .disabled(self.no_changes)
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
                        .disabled(self.no_changes)
                        .on_click(move |_, cx| cx.dispatch_action(Box::new(UnstageAll))),
                    )
                }
            }))
    }
}

fn changed_file_item(
    id: ChangedFileId,
    file: &PanelChangedFile,
    is_selected: bool,
    indent_level: usize,
) -> AnyElement {
    ChangedFileItem::new(
        ElementId::Name(format!("file-{}", id.0).into()),
        file.clone(),
        is_selected,
        indent_level,
    )
    .into_any_element()
}

fn directory_item(
    path: &str,
    file_count: usize,
    is_selected: bool,
    is_expanded: bool,
    indent_level: usize,
    selection: Selection,
) -> AnyElement {
    DirectoryItem::new(
        ElementId::Name(format!("dir-{}", path).into()),
        path.to_string(),
        file_count,
        is_selected,
        is_expanded,
        indent_level,
        selection,
    )
    .into_any_element()
}

fn update_list(model: Model<GitPanelState>, cx: &mut WindowContext) -> ListState {
    let new_items = model.read(cx).generate_git_list_items();
    let total_items = new_items.len();

    ListState::new(
        total_items,
        gpui::ListAlignment::Top,
        px(10.),
        move |ix, cx| {
            let closure_state = model.read(cx);
            match &new_items[ix] {
                GitListItem::Header(is_staged) => {
                    let count = if *is_staged {
                        closure_state.staged_count()
                    } else {
                        closure_state.unstaged_count()
                    };
                    StagingHeaderItem::new(
                        if *is_staged {
                            "staged-header"
                        } else {
                            "unstaged-header"
                        },
                        *is_staged,
                        count,
                        closure_state.selected_index == ix,
                        true, // Always expanded for now
                        count == 0,
                    )
                    .into_any_element()
                }
                GitListItem::Divider => Divider::horizontal().into_any_element(),
                GitListItem::Directory {
                    path,
                    indent_level,
                    is_expanded,
                } => {
                    let file_count = closure_state.count_files_in_directory(path);
                    let selection = closure_state.directory_selection(path);
                    directory_item(
                        path,
                        file_count,
                        closure_state.selected_index == ix,
                        *is_expanded,
                        *indent_level,
                        selection,
                    )
                }
                GitListItem::File { id, indent_level } => {
                    if let Some(file) = closure_state.files.get(id) {
                        changed_file_item(
                            id.clone(),
                            file,
                            closure_state.selected_index == ix,
                            *indent_level,
                        )
                    } else {
                        div().into_any_element() // Placeholder for missing files
                    }
                }
            }
        },
    )
}

#[derive(Clone)]
pub struct GitPanel {
    id: ElementId,
    focus_handle: FocusHandle,
    state: Model<GitPanelState>,
    list_state: ListState,
    width: Option<Pixels>,
}

impl GitPanel {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        let changed_files = static_changed_files();
        let new_state = GitPanelState::new(changed_files);
        let model = cx.new_model(|_cx| new_state);

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            state: model.clone(),
            list_state: update_list(model, cx),
            width: Some(px(400.).into()),
        }
    }

    fn update_list_if_needed(&mut self, cx: &mut ViewContext<Self>) {
        let model = self.state.clone();

        if self.state.read(cx).needs_update {
            update_list(model, cx);
            self.state.update(cx, |state, _| {
                state.needs_update = false;
            });
        }
    }

    fn update_list(&mut self, list_state: ListState) -> &mut Self {
        self.list_state = list_state;
        self
    }

    fn discard_all(&mut self, _: &DiscardAll, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.discard_all();
        });
        self.update_list_if_needed(cx);
        cx.notify();
    }

    fn stage_all(&mut self, _: &StageAll, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.stage_all();
        });
        self.update_list_if_needed(cx);
        cx.notify();
    }

    fn unstage_all(&mut self, _: &UnstageAll, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.unstage_all();
        });
        self.update_list_if_needed(cx);
        cx.notify();
    }

    fn discard_selected(&mut self, _: &DiscardSelected, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.discard_selected();
        });
        self.update_list_if_needed(cx);
        cx.notify();
    }

    fn stage_selected(&mut self, _: &StageSelected, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.stage_selected();
        });
        self.update_list_if_needed(cx);
        cx.notify();
    }

    fn unstage_selected(&mut self, _: &UnstageSelected, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.unstage_selected();
        });
        self.update_list_if_needed(cx);
        cx.notify();
    }

    pub fn set_selected_index(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.selected_index = index.min(state.total_item_count() - 1);
        });
        self.list_state.scroll_to_reveal_item(index);
        cx.notify();
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let current_index = self.state.read(cx).selected_index;
        let total_count = self.state.read(cx).total_item_count();
        let new_index = (current_index + 1).min(total_count - 1);
        self.set_selected_index(new_index, cx);
    }

    pub fn select_previous(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let current_index = self.state.read(cx).selected_index;
        let new_index = current_index.saturating_sub(1);
        self.set_selected_index(new_index, cx);
    }

    pub fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        self.set_selected_index(0, cx);
    }

    pub fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let total_count = self.state.read(cx).total_item_count();
        self.set_selected_index(total_count - 1, cx);
    }
}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.update_list_if_needed(cx);
        let state = self.state.read(cx);

        v_flex()
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
            .size_full()
            .overflow_hidden()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(ProjectOverviewItem::new(
                "project-overview",
                state.changed_file_count(),
                state.lines_changed.clone(),
            ))
            .child(Divider::horizontal_dashed())
            .child(list(self.list_state.clone()).size_full().into_any_element())
    }
}

impl workspace::Item for GitPanel {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(workspace::item::ItemEvent)) {}

    fn tab_content(&self, _params: TabContentParams, _cx: &WindowContext) -> AnyElement {
        Label::new("Git").into_any_element()
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
    fn position(&self, _cx: &gpui::WindowContext) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _position: DockPosition, _cx: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &gpui::WindowContext) -> Pixels {
        self.width.unwrap_or(px(400.))
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        cx.notify();
    }

    fn icon(&self, _cx: &gpui::WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Git")
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
            file_path: "tests/user_auth_test.rs".into(),
            lines_added: 30,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        PanelChangedFile {
            staged: false,
            file_path: "tests/product_api_test.rs".into(),
            lines_added: 45,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        PanelChangedFile {
            staged: true,
            file_path: "tests/order_processing_test.rs".into(),
            lines_added: 55,
            lines_removed: 10,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: false,
            file_path: "tests/database_integration_test.rs".into(),
            lines_added: 40,
            lines_removed: 5,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: true,
            file_path: "tests/performance_test.rs".into(),
            lines_added: 70,
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
            staged: false,
            file_path: "src/models/product.rs".into(),
            lines_added: 18,
            lines_removed: 0,
            status: GitFileStatus::Added,
        },
        PanelChangedFile {
            staged: false,
            file_path: "src/models/order.rs".into(),
            lines_added: 0,
            lines_removed: 22,
            status: GitFileStatus::Conflict,
        },
        PanelChangedFile {
            staged: false,
            file_path: "src/models/customer.rs".into(),
            lines_added: 0,
            lines_removed: 15,
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
            file_path: "src/services/user.rs".into(),
            lines_added: 15,
            lines_removed: 2,
            status: GitFileStatus::Modified,
        },
        PanelChangedFile {
            staged: true,
            file_path: "src/services/database.rs".into(),
            lines_added: 30,
            lines_removed: 0,
            status: GitFileStatus::Added,
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
