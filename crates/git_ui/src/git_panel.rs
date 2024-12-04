use editor::Editor;
use git::repository::GitFileStatus;
use gpui::*;
use settings::Settings;
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;
use theme::ThemeSettings;
use ui::{prelude::*, Checkbox, DividerColor, ElevationIndex, IconButtonShape};
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

const ADDED_COLOR: Hsla = Hsla {
    h: 142. / 360.,
    s: 0.68,
    l: 0.45,
    a: 1.0,
};
const MODIFIED_COLOR: Hsla = Hsla {
    h: 48. / 360.,
    s: 0.76,
    l: 0.47,
    a: 1.0,
};
const REMOVED_COLOR: Hsla = Hsla {
    h: 355. / 360.,
    s: 0.65,
    l: 0.65,
    a: 1.0,
};

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
    commit_message: Option<SharedString>,
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
            commit_message: None,
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
        self.add_files_to_list(&mut items, false);

        // Staged header
        items.push(GitListItem::Header(true));

        // Staged files
        self.add_files_to_list(&mut items, true);

        items
    }

    fn add_files_to_list(&self, items: &mut Vec<GitListItem>, is_staged: bool) {
        let mut files: Vec<_> = self
            .files
            .iter()
            .filter(|(_, file)| file.staged == is_staged)
            .collect();

        files.sort_by(|(_, a), (_, b)| a.file_path.cmp(&b.file_path));

        for (id, file) in files {
            items.push(GitListItem::File {
                id: id.clone(),
                indent_level: 0,
            });
        }
    }

    fn count_files_in_directory(&self, path: &str) -> usize {
        self.files
            .values()
            .filter(|file| file.file_path.starts_with(path))
            .count()
    }

    fn directory_selection(&self, path: &str) -> Selection {
        let mut all_staged = true;
        let mut all_unstaged = true;

        for file in self.files.values() {
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
        if self.show_list {
            self.changed_file_count()
        } else {
            self.changed_file_count() + 2 // +2 for the two headers
        }
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

    fn no_changes(&self) -> bool {
        self.changed_file_count() == 0
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
    model: Model<GitPanelState>,
}

impl ChangedFileItem {
    fn new(
        id: impl Into<ElementId>,
        file: PanelChangedFile,
        is_selected: bool,
        indent_level: usize,
        model: Model<GitPanelState>,
    ) -> Self {
        Self {
            id: id.into(),
            file,
            is_selected,
            indent_level,
            model,
        }
    }
}

impl RenderOnce for ChangedFileItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let file_path = self.file.file_path.clone();

        let (icon_name, color) = match self.file.status {
            GitFileStatus::Added => (IconName::SquarePlus, Color::Custom(ADDED_COLOR)),
            GitFileStatus::Modified => (IconName::SquareDot, Color::Custom(MODIFIED_COLOR)),
            GitFileStatus::Conflict => (IconName::SquareMinus, Color::Custom(REMOVED_COLOR)),
        };

        let is_deleted = self.file.status == GitFileStatus::Conflict;

        let selected_color = cx.theme().status().info.opacity(0.1);

        h_flex()
            .id(self.id.clone())
            .items_center()
            .justify_between()
            .w_full()
            .map(|this| {
                if self.is_selected {
                    this.bg(selected_color)
                } else {
                    this.bg(cx.theme().colors().elevated_surface_background)
                }
            })
            .hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
            .cursor(CursorStyle::PointingHand)
            .group("")
            .rounded_sm()
            .pl(px(12. + (self.indent_level as f32 * 12.)))
            .pr(px(12.))
            .h(px(28.))
            .child(
                h_flex()
                    .gap(px(8.))
                    .child(
                        Checkbox::new(self.id.clone(), self.file.staged.into()).on_click({
                            let model = self.model.clone();
                            let file_path = file_path.clone();

                            move |_, cx| {
                                let file_path = file_path.clone();
                                model.update(cx, |state, cx| {
                                    let file =
                                        state.files.get_mut(&ChangedFileId(file_path.clone()));
                                    if let Some(file) = file {
                                        file.staged = !file.staged;
                                    }
                                    state.update_lines_changed();
                                    state.file_tree = FileTree::new(&state.files);
                                    state.needs_update = true;
                                    cx.notify();
                                });
                            }
                        }),
                    )
                    .child(Icon::new(icon_name).size(IconSize::Small).color(color))
                    .child(
                        Label::new(file_path.clone())
                            .strikethrough(is_deleted)
                            .size(LabelSize::Small)
                            .color(if is_deleted {
                                Color::Placeholder
                            } else {
                                Color::Default
                            }),
                    )
                    .child(render_lines_changed(
                        self.file.lines_added,
                        self.file.lines_removed,
                        cx,
                    )),
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

fn render_lines_changed(added: usize, removed: usize, cx: &WindowContext) -> impl IntoElement {
    h_flex()
        .gap_1()
        .text_size(px(10.))
        .font_buffer(cx)
        .when(added > 0, |this| {
            this.child(div().text_color(ADDED_COLOR).child(format!("+{}", added)))
        })
        .when(removed > 0, |this| {
            this.child(
                div()
                    .text_color(REMOVED_COLOR)
                    .child(format!("-{}", removed)),
            )
        })
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
        let selected_color = cx.theme().status().info.opacity(0.1);

        h_flex()
            .id(self.id.clone())
            .items_center()
            .justify_between()
            .w_full()
            .cursor(CursorStyle::PointingHand)
            .map(|this| {
                if self.is_selected {
                    this.bg(selected_color)
                } else {
                    this.bg(cx.theme().colors().elevated_surface_background)
                }
            })
            .group("")
            .rounded_sm()
            .h(px(28.))
            .child(
                h_flex()
                    .pl(px(12.) + px(self.indent_level as f32 * 12.))
                    .gap(px(8.))
                    .child(Checkbox::new(self.id.clone(), self.selection))
                    // .child(
                    //     Icon::new(if self.is_expanded {
                    //         IconName::ChevronDown
                    //     } else {
                    //         IconName::ChevronRight
                    //     })
                    //     .size(IconSize::Small),
                    // )
                    .child(
                        Icon::new(IconName::Folder)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new(self.path).size(LabelSize::Small)),
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
    model: Model<GitPanelState>,
}

impl ProjectOverviewItem {
    pub fn new(
        id: impl Into<ElementId>,
        changed_file_count: usize,
        lines_changed: GitLines,
        model: Model<GitPanelState>,
    ) -> Self {
        Self {
            id: id.into(),
            changed_file_count,
            lines_changed,
            model,
        }
    }
}

impl ProjectOverviewItem {
    fn render_switch_button(
        &self,
        id: impl Into<ElementId>,
        icon_name: IconName,
        is_list_button: bool,
        cx: &mut WindowContext,
    ) -> IconButton {
        let model = self.model.clone();
        let is_list_view = self.model.read(cx).show_list;

        IconButton::new(id, icon_name)
            .style(if is_list_view == is_list_button {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Transparent
            })
            .size(ButtonSize::Default)
            .icon_size(IconSize::XSmall)
            .icon_color(if is_list_view == is_list_button {
                Color::Default
            } else {
                Color::Muted
            })
            .selected(is_list_view == is_list_button)
            .on_click(move |_, cx| {
                model.update(cx, |state, cx| {
                    state.show_list = is_list_button;
                    state.needs_update = true;
                    cx.notify();
                });
            })
    }
}

impl RenderOnce for ProjectOverviewItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let changed_files: SharedString = format!("{} Changes", self.changed_file_count).into();

        let model = self.model.clone();
        let state = model.read(cx);
        let all_staged = state.all_staged();
        let all_unstaged = state.all_unstaged();

        let checkbox_selection = match (all_staged, all_unstaged) {
            (true, false) => Selection::Selected,
            (false, true) => Selection::Unselected,
            _ => Selection::Indeterminate,
        };

        let fake_segemented_switch = h_flex()
            .overflow_hidden()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_switch_button("list-view", IconName::Quote, true, cx))
            .child(div().h_full().w_px().bg(cx.theme().colors().border))
            .child(self.render_switch_button("split-view", IconName::Split, false, cx));

        h_flex()
            .id(self.id.clone())
            .w_full()
            .bg(cx.theme().colors().elevated_surface_background)
            .px(px(12.))
            .h(px(28.))
            .gap_2()
            .child(
                h_flex()
                    .gap_4()
                    .child(Checkbox::new("all-staged", checkbox_selection).on_click({
                        let model = self.model.clone();
                        move |_, cx| {
                            if model.read(cx).all_staged() {
                                cx.dispatch_action(Box::new(UnstageAll))
                            } else {
                                cx.dispatch_action(Box::new(StageAll))
                            };
                        }
                    }))
                    .child(Label::new(changed_files).size(LabelSize::Small))
                    .child(render_lines_changed(
                        self.lines_changed.added,
                        self.lines_changed.removed,
                        cx,
                    )),
            )
            .child(div().flex_1())
            .child(fake_segemented_switch)
    }
}

#[derive(IntoElement)]
pub struct StagingHeaderItem {
    id: ElementId,
    is_staged: bool,
    count: usize,
    is_selected: bool,
    is_expanded: bool,
    model: Model<GitPanelState>,
}

impl StagingHeaderItem {
    pub fn new(
        id: impl Into<ElementId>,
        is_staged: bool,
        count: usize,
        is_selected: bool,
        is_expanded: bool,
        no_changes: bool,
        model: Model<GitPanelState>,
    ) -> Self {
        Self {
            id: id.into(),
            is_staged,
            count,
            is_selected,
            is_expanded,
            model,
        }
    }
}

impl RenderOnce for StagingHeaderItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let staging_type = if self.is_staged { "Staged" } else { "Unstaged" };
        let label: SharedString = format!("{} Changes", staging_type).into();
        let selected_color = cx.theme().status().info.opacity(0.1);
        let model = self.model.clone();
        let state = model.read(cx);

        h_flex()
            .id(self.id.clone())
            .justify_between()
            .w_full()
            .map(|this| {
                if self.is_selected {
                    this.bg(selected_color)
                } else {
                    this.bg(cx.theme().colors().elevated_surface_background)
                }
            })
            .hover(|this| this.bg(cx.theme().colors().ghost_element_hover))
            .h(px(28.))
            .pl(px(12.))
            .pr_2()
            .child(
                h_flex()
                    .gap_2()
                    .child(Disclosure::new(self.id.clone(), self.is_expanded))
                    .child(Label::new(label).size(LabelSize::Small))
                    .child(
                        Label::new(self.count.to_string())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(h_flex().gap_2().map(|this| {
                if !self.is_staged {
                    this.child(
                        Button::new(
                            ElementId::Name(format!("{}-stage", self.id.clone()).into()),
                            "Stage All",
                        )
                        .style(ButtonStyle::Filled)
                        .size(ButtonSize::Compact)
                        .label_size(LabelSize::Small)
                        .layer(ui::ElevationIndex::ModalSurface)
                        .disabled(state.no_changes() || state.all_staged())
                        .on_click(move |_, cx| cx.dispatch_action(Box::new(StageAll))),
                    )
                } else {
                    this.child(
                        Button::new(
                            ElementId::Name(format!("{}-unstage", self.id.clone()).into()),
                            "Unstage All",
                        )
                        .style(ButtonStyle::Filled)
                        .size(ButtonSize::Compact)
                        .label_size(LabelSize::Small)
                        .layer(ui::ElevationIndex::ModalSurface)
                        .disabled(state.no_changes())
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
    model: Model<GitPanelState>,
) -> AnyElement {
    ChangedFileItem::new(
        ElementId::Name(format!("file-{}", id.0).into()),
        file.clone(),
        is_selected,
        indent_level,
        model,
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
    let state = model.read(cx);
    let items = state.generate_git_list_items();
    let total_items = items.len();

    ListState::new(
        total_items,
        gpui::ListAlignment::Top,
        px(10.),
        move |ix, cx| {
            let state = model.read(cx);
            match &items[ix] {
                GitListItem::Header(is_staged) => {
                    let count = if *is_staged {
                        state.staged_count()
                    } else {
                        state.unstaged_count()
                    };
                    StagingHeaderItem::new(
                        if *is_staged {
                            "staged-header"
                        } else {
                            "unstaged-header"
                        },
                        *is_staged,
                        count,
                        state.selected_index == ix,
                        true, // Always expanded for now
                        count == 0,
                        model.clone(),
                    )
                    .into_any_element()
                }
                GitListItem::Divider => Divider::horizontal().into_any_element(),
                GitListItem::Directory {
                    path,
                    indent_level,
                    is_expanded,
                } => {
                    let file_count = state.count_files_in_directory(path);
                    let selection = state.directory_selection(path);
                    directory_item(
                        path,
                        file_count,
                        state.selected_index == ix,
                        *is_expanded,
                        *indent_level,
                        selection,
                    )
                }
                GitListItem::File { id, indent_level } => {
                    if let Some(file) = state.files.get(id) {
                        changed_file_item(
                            id.clone(),
                            file,
                            state.selected_index == ix,
                            *indent_level,
                            model.clone(),
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
    scroll_handle: UniformListScrollHandle,
    width: Option<Pixels>,
    commit_composer: View<Editor>,
}

impl GitPanel {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        let changed_files = static_changed_files();
        let new_state = GitPanelState::new(changed_files);
        let model = cx.new_model(|_cx| new_state);
        let scroll_handle = UniformListScrollHandle::new();
        let editor = cx.new_view(|cx| {
            let theme = ThemeSettings::get_global(cx);

            let mut text_style = cx.text_style();
            let refinement = TextStyleRefinement {
                font_family: Some(theme.buffer_font.family.clone()),
                font_features: Some(FontFeatures::disable_ligatures()),
                font_size: Some(px(12.).into()),
                color: Some(cx.theme().colors().editor_foreground),
                background_color: Some(gpui::transparent_black()),
                ..Default::default()
            };

            text_style.refine(&refinement);

            let mut editor = Editor::multi_line(cx);
            editor.set_placeholder_text("Add a commit message", cx);
            editor.set_show_gutter(false, cx);
            editor.set_current_line_highlight(None);
            editor.set_text_style_refinement(refinement);
            editor
        });

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            state: model.clone(),
            list_state: update_list(model, cx),
            scroll_handle,
            width: Some(px(400.).into()),
            commit_composer: editor,
        }
    }

    // fn update_composer(&mut self, cx: &mut ViewContext<Self>) -> &mut Self {
    //     let commit_message = self.state.read(cx).commit_message.clone();

    //     self.commit_composer.update(cx, |editor, cx| {
    //         editor.set_text(commit_message.unwrap_or_default(), cx);
    //     });
    //     self
    // }

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

    fn render_unified_items(
        &mut self,
        range: Range<usize>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<AnyElement> {
        let state = self.state.read(cx);
        let model = self.state.clone();

        let mut all_items: Vec<(&ChangedFileId, &PanelChangedFile)> = state.files.iter().collect();
        all_items.sort_by(|a, b| a.1.file_path.cmp(&b.1.file_path));

        range
            .map(|ix| {
                if ix < all_items.len() {
                    let (id, file) = all_items[ix];
                    changed_file_item(
                        id.clone(),
                        file,
                        state.selected_index == ix,
                        0, // No indentation in unified view
                        model.clone(),
                    )
                } else {
                    div().into_any_element() // Placeholder for out-of-range items
                }
            })
            .collect()
    }

    fn render_split_items(
        &mut self,
        range: Range<usize>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<AnyElement> {
        let state = self.state.read(cx);
        let items = state.generate_git_list_items();
        let model = self.state.clone();

        range
            .map(|ix| {
                match &items[ix] {
                    GitListItem::Header(is_staged) => {
                        let count = if *is_staged {
                            state.staged_count()
                        } else {
                            state.unstaged_count()
                        };
                        StagingHeaderItem::new(
                            if *is_staged {
                                "staged-header"
                            } else {
                                "unstaged-header"
                            },
                            *is_staged,
                            count,
                            state.selected_index == ix,
                            true, // Always expanded for now
                            count == 0,
                            model.clone(),
                        )
                        .into_any_element()
                    }
                    GitListItem::Divider => h_flex()
                        .h(px(24.))
                        .items_center()
                        .child(Divider::horizontal().into_any_element())
                        .into_any_element(),
                    GitListItem::Directory {
                        path,
                        indent_level,
                        is_expanded,
                    } => {
                        let file_count = state.count_files_in_directory(path);
                        let selection = state.directory_selection(path);
                        directory_item(
                            path,
                            file_count,
                            state.selected_index == ix,
                            *is_expanded,
                            *indent_level,
                            selection,
                        )
                    }
                    GitListItem::File { id, indent_level } => {
                        if let Some(file) = state.files.get(id) {
                            changed_file_item(
                                id.clone(),
                                file,
                                state.selected_index == ix,
                                *indent_level,
                                model.clone(),
                            )
                        } else {
                            div().into_any_element() // Placeholder for missing files
                        }
                    }
                }
            })
            .collect()
    }

    fn render_commit_composer(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let model = self.state.clone();
        let state = self.state.read(cx);

        let commit_button = Button::new("commit-button", "Commit")
            .style(ButtonStyle::Filled)
            .size(ButtonSize::Compact)
            .on_click(move |_, cx| {
                model.update(cx, |state, cx| {
                    state.commit_message = None;
                    state.files.retain(|_, file| !file.staged);
                    state
                        .file_order
                        .retain(|id| state.files.get(id).map_or(true, |file| !file.staged));
                    state.update_lines_changed();
                    state.file_tree = FileTree::new(&state.files);
                    state.needs_update = true;
                    cx.notify();
                });
            });

        h_flex()
            .relative()
            .p_2()
            .w_full()
            .child(
                v_flex()
                    .relative()
                    .w_full()
                    .h(px(140.))
                    .px_3()
                    .py_2()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.commit_composer.clone())
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        h_flex()
                            .absolute()
                            .bottom_2()
                            .right_2()
                            // spacer
                            .child(div().flex_1().occlude())
                            .child(commit_button),
                    ),
            )
            .into_any_element()
    }
}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.update_list_if_needed(cx);
        let model = self.state.clone();
        let state = self.state.read(cx);
        let items_count = state.total_item_count();
        let view = cx.view().clone();
        let scroll_handle = self.scroll_handle.clone();
        let is_list_view = state.show_list;

        v_flex()
            .font_buffer(cx)
            .py_1()
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
                model.clone(),
            ))
            .child(
                h_flex()
                    .items_center()
                    .h(px(8.))
                    .child(Divider::horizontal_dashed().color(DividerColor::Border)),
            )
            .child(if is_list_view {
                uniform_list(
                    view,
                    "git-panel-list",
                    items_count,
                    Self::render_unified_items,
                )
                .py_1()
                .gap(px(2.))
                .track_scroll(scroll_handle)
                .size_full()
                .into_any_element()
            } else {
                uniform_list(
                    view,
                    "git-panel-list",
                    items_count,
                    Self::render_split_items,
                )
                .py_1()
                .gap(px(2.))
                .track_scroll(scroll_handle)
                .size_full()
                .into_any_element()
            })
            .child(div().flex_1())
            .when(is_list_view, |this| {
                this.child(
                    h_flex()
                        .items_center()
                        .px(px(12.))
                        .child(
                            h_flex()
                                .items_center()
                                .child(
                                    Label::new("Staged & Unstaged")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    Icon::new(IconName::ChevronDown)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                        .child(div().flex_1())
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("discard-all", "Discard All")
                                        .style(ButtonStyle::Filled)
                                        .size(ButtonSize::Compact)
                                        .label_size(LabelSize::Small)
                                        .layer(ui::ElevationIndex::ModalSurface)
                                        .disabled(state.no_changes())
                                        .on_click(move |_, cx| {
                                            cx.dispatch_action(Box::new(DiscardAll))
                                        }),
                                )
                                .child(
                                    Button::new("stage-all", "Stage All")
                                        .style(ButtonStyle::Filled)
                                        .size(ButtonSize::Compact)
                                        .label_size(LabelSize::Small)
                                        .layer(ui::ElevationIndex::ModalSurface)
                                        .disabled(state.no_changes() || state.all_staged())
                                        .on_click(move |_, cx| {
                                            cx.dispatch_action(Box::new(StageAll))
                                        }),
                                ),
                        ),
                )
            })
            .child(
                h_flex()
                    .items_center()
                    .h(px(8.))
                    .child(Divider::horizontal_dashed().color(DividerColor::Border)),
            )
            .child(self.render_commit_composer(cx))
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
