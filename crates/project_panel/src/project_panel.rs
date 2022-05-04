use editor::{Cancel, Editor};
use gpui::{
    actions,
    anyhow::Result,
    elements::{
        ChildView, ConstrainedBox, Empty, Flex, Label, MouseEventHandler, ParentElement,
        ScrollTarget, Svg, UniformList, UniformListState,
    },
    impl_internal_actions, keymap,
    platform::CursorStyle,
    AppContext, Element, ElementBox, Entity, ModelHandle, MutableAppContext, Task, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use project::{Entry, EntryKind, Project, ProjectEntryId, ProjectPath, Worktree, WorktreeId};
use settings::Settings;
use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap},
    ffi::OsStr,
    ops::Range,
};
use unicase::UniCase;
use workspace::{
    menu::{Confirm, SelectNext, SelectPrev},
    Workspace,
};

const NEW_FILE_ENTRY_ID: ProjectEntryId = ProjectEntryId::MAX;

pub struct ProjectPanel {
    project: ModelHandle<Project>,
    list: UniformListState,
    visible_entries: Vec<(WorktreeId, Vec<Entry>)>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
    selection: Option<Selection>,
    edit_state: Option<EditState>,
    filename_editor: ViewHandle<Editor>,
    handle: WeakViewHandle<Self>,
}

#[derive(Copy, Clone)]
struct Selection {
    worktree_id: WorktreeId,
    entry_id: ProjectEntryId,
}

#[derive(Copy, Clone, Debug)]
struct EditState {
    worktree_id: WorktreeId,
    entry_id: ProjectEntryId,
    new_file: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct EntryDetails {
    filename: String,
    depth: usize,
    kind: EntryKind,
    is_expanded: bool,
    is_selected: bool,
    is_editing: bool,
}

#[derive(Clone)]
pub struct ToggleExpanded(pub ProjectEntryId);

#[derive(Clone)]
pub struct Open(pub ProjectEntryId);

actions!(
    project_panel,
    [ExpandSelectedEntry, CollapseSelectedEntry, AddFile, Rename]
);
impl_internal_actions!(project_panel, [Open, ToggleExpanded]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ProjectPanel::expand_selected_entry);
    cx.add_action(ProjectPanel::collapse_selected_entry);
    cx.add_action(ProjectPanel::toggle_expanded);
    cx.add_action(ProjectPanel::select_prev);
    cx.add_action(ProjectPanel::select_next);
    cx.add_action(ProjectPanel::open_entry);
    cx.add_action(ProjectPanel::add_file);
    cx.add_action(ProjectPanel::rename);
    cx.add_async_action(ProjectPanel::confirm);
    cx.add_action(ProjectPanel::cancel);
}

pub enum Event {
    OpenedEntry(ProjectEntryId),
}

impl ProjectPanel {
    pub fn new(project: ModelHandle<Project>, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
        let project_panel = cx.add_view(|cx: &mut ViewContext<Self>| {
            cx.observe(&project, |this, _, cx| {
                this.update_visible_entries(None, cx);
                cx.notify();
            })
            .detach();
            cx.subscribe(&project, |this, project, event, cx| match event {
                project::Event::ActiveEntryChanged(Some(entry_id)) => {
                    if let Some(worktree_id) = project.read(cx).worktree_id_for_entry(*entry_id, cx)
                    {
                        this.expand_entry(worktree_id, *entry_id, cx);
                        this.update_visible_entries(Some((worktree_id, *entry_id)), cx);
                        this.autoscroll();
                        cx.notify();
                    }
                }
                project::Event::WorktreeRemoved(id) => {
                    this.expanded_dir_ids.remove(id);
                    this.update_visible_entries(None, cx);
                    cx.notify();
                }
                _ => {}
            })
            .detach();

            let filename_editor = cx.add_view(|cx| {
                Editor::single_line(
                    Some(|theme| {
                        let mut style = theme.project_panel.filename_editor.clone();
                        style.container.background_color.take();
                        style
                    }),
                    cx,
                )
            });
            cx.subscribe(&filename_editor, |this, _, event, cx| {
                if let editor::Event::Blurred = event {
                    this.editor_blurred(cx);
                }
            })
            .detach();

            let mut this = Self {
                project: project.clone(),
                list: Default::default(),
                visible_entries: Default::default(),
                expanded_dir_ids: Default::default(),
                selection: None,
                edit_state: None,
                filename_editor,
                handle: cx.weak_handle(),
            };
            this.update_visible_entries(None, cx);
            this
        });
        cx.subscribe(&project_panel, move |workspace, _, event, cx| match event {
            &Event::OpenedEntry(entry_id) => {
                if let Some(worktree) = project.read(cx).worktree_for_entry(entry_id, cx) {
                    if let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                        workspace
                            .open_path(
                                ProjectPath {
                                    worktree_id: worktree.read(cx).id(),
                                    path: entry.path.clone(),
                                },
                                cx,
                            )
                            .detach_and_log_err(cx);
                    }
                }
            }
        })
        .detach();

        project_panel
    }

    fn expand_selected_entry(&mut self, _: &ExpandSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some((worktree, entry)) = self.selected_entry(cx) {
            let expanded_dir_ids =
                if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree.id()) {
                    expanded_dir_ids
                } else {
                    return;
                };

            if entry.is_dir() {
                match expanded_dir_ids.binary_search(&entry.id) {
                    Ok(_) => self.select_next(&SelectNext, cx),
                    Err(ix) => {
                        expanded_dir_ids.insert(ix, entry.id);
                        self.update_visible_entries(None, cx);
                        cx.notify();
                    }
                }
            } else {
                let event = Event::OpenedEntry(entry.id);
                cx.emit(event);
            }
        }
    }

    fn collapse_selected_entry(&mut self, _: &CollapseSelectedEntry, cx: &mut ViewContext<Self>) {
        if let Some((worktree, mut entry)) = self.selected_entry(cx) {
            let expanded_dir_ids =
                if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree.id()) {
                    expanded_dir_ids
                } else {
                    return;
                };

            loop {
                match expanded_dir_ids.binary_search(&entry.id) {
                    Ok(ix) => {
                        expanded_dir_ids.remove(ix);
                        self.update_visible_entries(Some((worktree.id(), entry.id)), cx);
                        cx.notify();
                        break;
                    }
                    Err(_) => {
                        if let Some(parent_entry) =
                            entry.path.parent().and_then(|p| worktree.entry_for_path(p))
                        {
                            entry = parent_entry;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn toggle_expanded(&mut self, action: &ToggleExpanded, cx: &mut ViewContext<Self>) {
        let entry_id = action.0;
        if let Some(worktree_id) = self.project.read(cx).worktree_id_for_entry(entry_id, cx) {
            if let Some(expanded_dir_ids) = self.expanded_dir_ids.get_mut(&worktree_id) {
                match expanded_dir_ids.binary_search(&entry_id) {
                    Ok(ix) => {
                        expanded_dir_ids.remove(ix);
                    }
                    Err(ix) => {
                        expanded_dir_ids.insert(ix, entry_id);
                    }
                }
                self.update_visible_entries(Some((worktree_id, entry_id)), cx);
                cx.focus_self();
            }
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if entry_ix > 0 {
                entry_ix -= 1;
            } else {
                if worktree_ix > 0 {
                    worktree_ix -= 1;
                    entry_ix = self.visible_entries[worktree_ix].1.len() - 1;
                } else {
                    return;
                }
            }

            let (worktree_id, worktree_entries) = &self.visible_entries[worktree_ix];
            self.selection = Some(Selection {
                worktree_id: *worktree_id,
                entry_id: worktree_entries[entry_ix].id,
            });
            self.autoscroll();
            cx.notify();
        } else {
            self.select_first(cx);
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        let edit_state = self.edit_state.take()?;
        cx.focus_self();

        let worktree = self
            .project
            .read(cx)
            .worktree_for_id(edit_state.worktree_id, cx)?;
        let entry = worktree.read(cx).entry_for_id(edit_state.entry_id)?.clone();
        let filename = self.filename_editor.read(cx).text(cx);

        if edit_state.new_file {
            let new_path = entry.path.join(filename);
            let save = self.project.update(cx, |project, cx| {
                project.create_file((edit_state.worktree_id, new_path), cx)
            })?;
            Some(cx.spawn(|this, mut cx| async move {
                let new_entry = save.await?;
                this.update(&mut cx, |this, cx| {
                    this.update_visible_entries(Some((edit_state.worktree_id, new_entry.id)), cx);
                    cx.notify();
                });
                Ok(())
            }))
        } else {
            let old_path = entry.path.clone();
            let new_path = if let Some(parent) = old_path.parent() {
                parent.join(filename)
            } else {
                filename.into()
            };

            let rename = self.project.update(cx, |project, cx| {
                project.rename_entry(entry.id, new_path, cx)
            })?;

            Some(cx.spawn(|this, mut cx| async move {
                let new_entry = rename.await?;
                this.update(&mut cx, |this, cx| {
                    this.update_visible_entries(Some((edit_state.worktree_id, new_entry.id)), cx);
                    cx.notify();
                });
                Ok(())
            }))
        }
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        self.edit_state = None;
        self.update_visible_entries(None, cx);
        cx.focus_self();
        cx.notify();
    }

    fn open_entry(&mut self, action: &Open, cx: &mut ViewContext<Self>) {
        cx.emit(Event::OpenedEntry(action.0));
    }

    fn add_file(&mut self, _: &AddFile, cx: &mut ViewContext<Self>) {
        if let Some(Selection {
            worktree_id,
            entry_id,
        }) = self.selection
        {
            let directory_id;
            if let Some((worktree, expanded_dir_ids)) = self
                .project
                .read(cx)
                .worktree_for_id(worktree_id, cx)
                .zip(self.expanded_dir_ids.get_mut(&worktree_id))
            {
                let worktree = worktree.read(cx);
                if let Some(mut entry) = worktree.entry_for_id(entry_id) {
                    loop {
                        if entry.is_dir() {
                            if let Err(ix) = expanded_dir_ids.binary_search(&entry.id) {
                                expanded_dir_ids.insert(ix, entry.id);
                            }
                            directory_id = entry.id;
                            break;
                        } else {
                            if let Some(parent_path) = entry.path.parent() {
                                if let Some(parent_entry) = worktree.entry_for_path(parent_path) {
                                    entry = parent_entry;
                                    continue;
                                }
                            }
                            return;
                        }
                    }
                } else {
                    return;
                };
            } else {
                return;
            };

            self.edit_state = Some(EditState {
                worktree_id,
                entry_id: directory_id,
                new_file: true,
            });
            self.filename_editor
                .update(cx, |editor, cx| editor.clear(cx));
            cx.focus(&self.filename_editor);
            self.update_visible_entries(None, cx);
            cx.notify();
        }
    }

    fn rename(&mut self, _: &Rename, cx: &mut ViewContext<Self>) {
        if let Some(Selection {
            worktree_id,
            entry_id,
        }) = self.selection
        {
            if let Some(worktree) = self.project.read(cx).worktree_for_id(worktree_id, cx) {
                if let Some(entry) = worktree.read(cx).entry_for_id(entry_id) {
                    self.edit_state = Some(EditState {
                        worktree_id,
                        entry_id,
                        new_file: false,
                    });
                    let filename = entry
                        .path
                        .file_name()
                        .map_or(String::new(), |s| s.to_string_lossy().to_string());
                    self.filename_editor.update(cx, |editor, cx| {
                        editor.set_text(filename, cx);
                        editor.select_all(&Default::default(), cx);
                    });
                    cx.focus(&self.filename_editor);
                    self.update_visible_entries(None, cx);
                    cx.notify();
                }
            }
        }
    }

    fn editor_blurred(&mut self, cx: &mut ViewContext<Self>) {
        self.edit_state = None;
        self.update_visible_entries(None, cx);
        cx.focus_self();
        cx.notify();
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            let (mut worktree_ix, mut entry_ix, _) =
                self.index_for_selection(selection).unwrap_or_default();
            if let Some((_, worktree_entries)) = self.visible_entries.get(worktree_ix) {
                if entry_ix + 1 < worktree_entries.len() {
                    entry_ix += 1;
                } else {
                    worktree_ix += 1;
                    entry_ix = 0;
                }
            }

            if let Some((worktree_id, worktree_entries)) = self.visible_entries.get(worktree_ix) {
                if let Some(entry) = worktree_entries.get(entry_ix) {
                    self.selection = Some(Selection {
                        worktree_id: *worktree_id,
                        entry_id: entry.id,
                    });
                    self.autoscroll();
                    cx.notify();
                }
            }
        } else {
            self.select_first(cx);
        }
    }

    fn select_first(&mut self, cx: &mut ViewContext<Self>) {
        let worktree = self
            .visible_entries
            .first()
            .and_then(|(worktree_id, _)| self.project.read(cx).worktree_for_id(*worktree_id, cx));
        if let Some(worktree) = worktree {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            if let Some(root_entry) = worktree.root_entry() {
                self.selection = Some(Selection {
                    worktree_id,
                    entry_id: root_entry.id,
                });
                self.autoscroll();
                cx.notify();
            }
        }
    }

    fn autoscroll(&mut self) {
        if let Some((_, _, index)) = self.selection.and_then(|s| self.index_for_selection(s)) {
            self.list.scroll_to(ScrollTarget::Show(index));
        }
    }

    fn index_for_selection(&self, selection: Selection) -> Option<(usize, usize, usize)> {
        let mut worktree_index = 0;
        let mut entry_index = 0;
        let mut visible_entries_index = 0;
        for (worktree_id, worktree_entries) in &self.visible_entries {
            if *worktree_id == selection.worktree_id {
                for entry in worktree_entries {
                    if entry.id == selection.entry_id {
                        return Some((worktree_index, entry_index, visible_entries_index));
                    } else {
                        visible_entries_index += 1;
                        entry_index += 1;
                    }
                }
                break;
            } else {
                visible_entries_index += worktree_entries.len();
            }
            worktree_index += 1;
        }
        None
    }

    fn selected_entry<'a>(&self, cx: &'a AppContext) -> Option<(&'a Worktree, &'a project::Entry)> {
        let selection = self.selection?;
        let project = self.project.read(cx);
        let worktree = project.worktree_for_id(selection.worktree_id, cx)?.read(cx);
        Some((worktree, worktree.entry_for_id(selection.entry_id)?))
    }

    fn update_visible_entries(
        &mut self,
        new_selected_entry: Option<(WorktreeId, ProjectEntryId)>,
        cx: &mut ViewContext<Self>,
    ) {
        let worktrees = self
            .project
            .read(cx)
            .worktrees(cx)
            .filter(|worktree| worktree.read(cx).is_visible());
        self.visible_entries.clear();

        for worktree in worktrees {
            let snapshot = worktree.read(cx).snapshot();
            let worktree_id = snapshot.id();

            let expanded_dir_ids = match self.expanded_dir_ids.entry(worktree_id) {
                hash_map::Entry::Occupied(e) => e.into_mut(),
                hash_map::Entry::Vacant(e) => {
                    // The first time a worktree's root entry becomes available,
                    // mark that root entry as expanded.
                    if let Some(entry) = snapshot.root_entry() {
                        e.insert(vec![entry.id]).as_slice()
                    } else {
                        &[]
                    }
                }
            };

            let new_file_parent_id = self.edit_state.and_then(|edit_state| {
                if edit_state.worktree_id == worktree_id && edit_state.new_file {
                    Some(edit_state.entry_id)
                } else {
                    None
                }
            });

            let mut visible_worktree_entries = Vec::new();
            let mut entry_iter = snapshot.entries(false);
            while let Some(entry) = entry_iter.entry() {
                visible_worktree_entries.push(entry.clone());
                if Some(entry.id) == new_file_parent_id {
                    visible_worktree_entries.push(Entry {
                        id: NEW_FILE_ENTRY_ID,
                        kind: project::EntryKind::File(Default::default()),
                        path: entry.path.join("\0").into(),
                        inode: 0,
                        mtime: entry.mtime,
                        is_symlink: false,
                        is_ignored: false,
                    });
                }
                if expanded_dir_ids.binary_search(&entry.id).is_err() {
                    if entry_iter.advance_to_sibling() {
                        continue;
                    }
                }
                entry_iter.advance();
            }
            visible_worktree_entries.sort_by(|entry_a, entry_b| {
                let mut components_a = entry_a.path.components().peekable();
                let mut components_b = entry_b.path.components().peekable();
                loop {
                    match (components_a.next(), components_b.next()) {
                        (Some(component_a), Some(component_b)) => {
                            let a_is_file = components_a.peek().is_none() && entry_a.is_file();
                            let b_is_file = components_b.peek().is_none() && entry_b.is_file();
                            let ordering = a_is_file.cmp(&b_is_file).then_with(|| {
                                let name_a =
                                    UniCase::new(component_a.as_os_str().to_string_lossy());
                                let name_b =
                                    UniCase::new(component_b.as_os_str().to_string_lossy());
                                name_a.cmp(&name_b)
                            });
                            if !ordering.is_eq() {
                                return ordering;
                            }
                        }
                        (Some(_), None) => break Ordering::Greater,
                        (None, Some(_)) => break Ordering::Less,
                        (None, None) => break Ordering::Equal,
                    }
                }
            });
            self.visible_entries
                .push((worktree_id, visible_worktree_entries));
        }

        if let Some((worktree_id, entry_id)) = new_selected_entry {
            self.selection = Some(Selection {
                worktree_id,
                entry_id,
            });
        }
    }

    fn expand_entry(
        &mut self,
        worktree_id: WorktreeId,
        entry_id: ProjectEntryId,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.project.read(cx);
        if let Some((worktree, expanded_dir_ids)) = project
            .worktree_for_id(worktree_id, cx)
            .zip(self.expanded_dir_ids.get_mut(&worktree_id))
        {
            let worktree = worktree.read(cx);

            if let Some(mut entry) = worktree.entry_for_id(entry_id) {
                loop {
                    if let Err(ix) = expanded_dir_ids.binary_search(&entry.id) {
                        expanded_dir_ids.insert(ix, entry.id);
                    }

                    if let Some(parent_entry) =
                        entry.path.parent().and_then(|p| worktree.entry_for_path(p))
                    {
                        entry = parent_entry;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn for_each_visible_entry(
        &self,
        range: Range<usize>,
        cx: &mut ViewContext<ProjectPanel>,
        mut callback: impl FnMut(ProjectEntryId, EntryDetails, &mut ViewContext<ProjectPanel>),
    ) {
        let mut ix = 0;
        for (worktree_id, visible_worktree_entries) in &self.visible_entries {
            if ix >= range.end {
                return;
            }

            if ix + visible_worktree_entries.len() <= range.start {
                ix += visible_worktree_entries.len();
                continue;
            }

            let end_ix = range.end.min(ix + visible_worktree_entries.len());
            if let Some(worktree) = self.project.read(cx).worktree_for_id(*worktree_id, cx) {
                let snapshot = worktree.read(cx).snapshot();
                let expanded_entry_ids = self
                    .expanded_dir_ids
                    .get(&snapshot.id())
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);
                let root_name = OsStr::new(snapshot.root_name());
                for entry in &visible_worktree_entries[range.start.saturating_sub(ix)..end_ix - ix]
                {
                    let mut details = EntryDetails {
                        filename: entry
                            .path
                            .file_name()
                            .unwrap_or(root_name)
                            .to_string_lossy()
                            .to_string(),
                        depth: entry.path.components().count(),
                        kind: entry.kind,
                        is_expanded: expanded_entry_ids.binary_search(&entry.id).is_ok(),
                        is_selected: self.selection.map_or(false, |e| {
                            e.worktree_id == snapshot.id() && e.entry_id == entry.id
                        }),
                        is_editing: false,
                    };
                    if let Some(edit_state) = self.edit_state {
                        if edit_state.new_file {
                            if entry.id == NEW_FILE_ENTRY_ID {
                                details.is_editing = true;
                                details.filename.clear();
                            }
                        } else {
                            if entry.id == edit_state.entry_id {
                                details.is_editing = true;
                            }
                        };
                    }
                    callback(entry.id, details, cx);
                }
            }
            ix = end_ix;
        }
    }

    fn render_entry(
        entry_id: ProjectEntryId,
        details: EntryDetails,
        editor: &ViewHandle<Editor>,
        theme: &theme::ProjectPanel,
        cx: &mut ViewContext<Self>,
    ) -> ElementBox {
        let kind = details.kind;
        MouseEventHandler::new::<Self, _, _>(entry_id.to_usize(), cx, |state, _| {
            let padding = theme.container.padding.left + details.depth as f32 * theme.indent_width;
            let style = theme.entry.style_for(state, details.is_selected);
            let row_container_style = if details.is_editing {
                theme.filename_editor.container
            } else {
                style.container
            };
            Flex::row()
                .with_child(
                    ConstrainedBox::new(if kind == EntryKind::Dir {
                        if details.is_expanded {
                            Svg::new("icons/disclosure-open.svg")
                                .with_color(style.icon_color)
                                .boxed()
                        } else {
                            Svg::new("icons/disclosure-closed.svg")
                                .with_color(style.icon_color)
                                .boxed()
                        }
                    } else {
                        Empty::new().boxed()
                    })
                    .with_max_width(style.icon_size)
                    .with_max_height(style.icon_size)
                    .aligned()
                    .constrained()
                    .with_width(style.icon_size)
                    .boxed(),
                )
                .with_child(if details.is_editing {
                    ChildView::new(editor.clone())
                        .contained()
                        .with_margin_left(theme.entry.default.icon_spacing)
                        .aligned()
                        .left()
                        .flex(1.0, true)
                        .boxed()
                } else {
                    Label::new(details.filename, style.text.clone())
                        .contained()
                        .with_margin_left(style.icon_spacing)
                        .aligned()
                        .left()
                        .boxed()
                })
                .constrained()
                .with_height(theme.entry.default.height)
                .contained()
                .with_style(row_container_style)
                .with_padding_left(padding)
                .boxed()
        })
        .on_click(move |cx| {
            if kind == EntryKind::Dir {
                cx.dispatch_action(ToggleExpanded(entry_id))
            } else {
                cx.dispatch_action(Open(entry_id))
            }
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }
}

impl View for ProjectPanel {
    fn ui_name() -> &'static str {
        "ProjectPanel"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let theme = &cx.global::<Settings>().theme.project_panel;
        let mut container_style = theme.container;
        let padding = std::mem::take(&mut container_style.padding);
        let handle = self.handle.clone();
        UniformList::new(
            self.list.clone(),
            self.visible_entries
                .iter()
                .map(|(_, worktree_entries)| worktree_entries.len())
                .sum(),
            move |range, items, cx| {
                let theme = cx.global::<Settings>().theme.clone();
                let this = handle.upgrade(cx).unwrap();
                this.update(cx.app, |this, cx| {
                    this.for_each_visible_entry(range.clone(), cx, |id, details, cx| {
                        items.push(Self::render_entry(
                            id,
                            details,
                            &this.filename_editor,
                            &theme.project_panel,
                            cx,
                        ));
                    });
                })
            },
        )
        .with_padding_top(padding.top)
        .with_padding_bottom(padding.bottom)
        .contained()
        .with_style(container_style)
        .boxed()
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }
}

impl Entity for ProjectPanel {
    type Event = Event;
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, ViewHandle};
    use project::FakeFs;
    use serde_json::json;
    use std::{collections::HashSet, path::Path};
    use workspace::WorkspaceParams;

    #[gpui::test]
    async fn test_visible_list(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
                "C": {
                    "5": {},
                    "6": { "V": "", "W": "" },
                    "7": { "X": "" },
                    "8": { "Y": {}, "Z": "" }
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "d": {
                    "9": ""
                },
                "e": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1", "/root2"], cx).await;
        let params = cx.update(WorkspaceParams::test);
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        let panel = workspace.update(cx, |_, cx| ProjectPanel::new(project, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        toggle_expand_dir(&panel, "root1/b", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..50, cx),
            &[
                "v root1",
                "    > a",
                "    v b  <== selected",
                "        > 3",
                "        > 4",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        assert_eq!(
            visible_entries_as_strings(&panel, 5..8, cx),
            &[
                //
                "    > C",
                "      .dockerignore",
                "v root2",
            ]
        );
    }

    #[gpui::test(iterations = 30)]
    async fn test_editing_files(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
                "C": {
                    "5": {},
                    "6": { "V": "", "W": "" },
                    "7": { "X": "" },
                    "8": { "Y": {}, "Z": "" }
                }
            }),
        )
        .await;
        fs.insert_tree(
            "/root2",
            json!({
                "d": {
                    "9": ""
                },
                "e": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/root1", "/root2"], cx).await;
        let params = cx.update(WorkspaceParams::test);
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        let panel = workspace.update(cx, |_, cx| ProjectPanel::new(project, cx));

        select_path(&panel, "root1", cx);
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1  <== selected",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        // Add a file with the root folder selected. The filename editor is placed
        // before the first file in the root folder.
        panel.update(cx, |panel, cx| panel.add_file(&AddFile, cx));
        assert!(panel.read_with(cx, |panel, cx| panel.filename_editor.is_focused(cx)));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1  <== selected",
                "    > a",
                "    > b",
                "    > C",
                "      [EDITOR: '']",
                "      .dockerignore",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        panel
            .update(cx, |panel, cx| {
                panel
                    .filename_editor
                    .update(cx, |editor, cx| editor.set_text("the-new-filename", cx));
                panel.confirm(&Confirm, cx).unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..10, cx),
            &[
                "v root1",
                "    > a",
                "    > b",
                "    > C",
                "      .dockerignore",
                "      the-new-filename  <== selected",
                "v root2",
                "    > d",
                "    > e",
            ]
        );

        select_path(&panel, "root1/b", cx);
        panel.update(cx, |panel, cx| panel.add_file(&AddFile, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..9, cx),
            &[
                "v root1",
                "    > a",
                "    v b  <== selected",
                "        > 3",
                "        > 4",
                "          [EDITOR: '']",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        panel
            .update(cx, |panel, cx| {
                panel
                    .filename_editor
                    .update(cx, |editor, cx| editor.set_text("another-filename", cx));
                panel.confirm(&Confirm, cx).unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..9, cx),
            &[
                "v root1",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          another-filename  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        select_path(&panel, "root1/b/another-filename", cx);
        panel.update(cx, |panel, cx| panel.rename(&Rename, cx));
        assert_eq!(
            visible_entries_as_strings(&panel, 0..9, cx),
            &[
                "v root1",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          [EDITOR: 'another-filename']  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );

        panel
            .update(cx, |panel, cx| {
                panel
                    .filename_editor
                    .update(cx, |editor, cx| editor.set_text("a-different-filename", cx));
                panel.confirm(&Confirm, cx).unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            visible_entries_as_strings(&panel, 0..9, cx),
            &[
                "v root1",
                "    > a",
                "    v b",
                "        > 3",
                "        > 4",
                "          a-different-filename  <== selected",
                "    > C",
                "      .dockerignore",
                "      the-new-filename",
            ]
        );
    }

    fn toggle_expand_dir(
        panel: &ViewHandle<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                    panel.toggle_expanded(&ToggleExpanded(entry_id), cx);
                    return;
                }
            }
            panic!("no worktree for path {:?}", path);
        });
    }

    fn select_path(
        panel: &ViewHandle<ProjectPanel>,
        path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) {
        let path = path.as_ref();
        panel.update(cx, |panel, cx| {
            for worktree in panel.project.read(cx).worktrees(cx).collect::<Vec<_>>() {
                let worktree = worktree.read(cx);
                if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                    let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                    panel.selection = Some(Selection {
                        worktree_id: worktree.id(),
                        entry_id,
                    });
                    return;
                }
            }
            panic!("no worktree for path {:?}", path);
        });
    }

    fn visible_entries_as_strings(
        panel: &ViewHandle<ProjectPanel>,
        range: Range<usize>,
        cx: &mut TestAppContext,
    ) -> Vec<String> {
        let mut result = Vec::new();
        let mut project_entries = HashSet::new();
        let mut has_editor = false;
        panel.update(cx, |panel, cx| {
            panel.for_each_visible_entry(range, cx, |project_entry, details, _| {
                if details.is_editing {
                    assert!(!has_editor, "duplicate editor entry");
                    has_editor = true;
                } else {
                    assert!(
                        project_entries.insert(project_entry),
                        "duplicate project entry {:?} {:?}",
                        project_entry,
                        details
                    );
                }

                let indent = "    ".repeat(details.depth);
                let icon = if details.kind == EntryKind::Dir {
                    if details.is_expanded {
                        "v "
                    } else {
                        "> "
                    }
                } else {
                    "  "
                };
                let editor_text = format!("[EDITOR: '{}']", details.filename);
                let name = if details.is_editing {
                    &editor_text
                } else {
                    &details.filename
                };
                let selected = if details.is_selected {
                    "  <== selected"
                } else {
                    ""
                };
                result.push(format!("{indent}{icon}{name}{selected}"));
            });
        });

        result
    }
}
