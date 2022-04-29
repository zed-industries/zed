use editor::Editor;
use gpui::{
    actions,
    elements::{
        ConstrainedBox, Empty, Flex, Label, MouseEventHandler, ParentElement, ScrollTarget, Svg,
        UniformList, UniformListState,
    },
    impl_internal_actions, keymap,
    platform::CursorStyle,
    AppContext, Element, ElementBox, Entity, ModelHandle, MutableAppContext, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
use project::{Entry, Project, ProjectEntryId, ProjectPath, Worktree, WorktreeId};
use settings::Settings;
use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap},
    ffi::OsStr,
    ops::Range,
};
use unicase::UniCase;
use workspace::{
    menu::{SelectNext, SelectPrev},
    Workspace,
};

pub struct ProjectPanel {
    project: ModelHandle<Project>,
    list: UniformListState,
    visible_entries: Vec<(WorktreeId, Vec<Entry>)>,
    expanded_dir_ids: HashMap<WorktreeId, Vec<ProjectEntryId>>,
    selection: Option<Selection>,
    handle: WeakViewHandle<Self>,
}

#[derive(Copy, Clone)]
struct Selection {
    worktree_id: WorktreeId,
    entry_id: ProjectEntryId,
}

#[derive(Debug, PartialEq, Eq)]
struct EntryDetails {
    filename: String,
    depth: usize,
    is_dir: bool,
    is_expanded: bool,
    is_selected: bool,
}

#[derive(Clone)]
pub struct ToggleExpanded(pub ProjectEntryId);

#[derive(Clone)]
pub struct Open(pub ProjectEntryId);

actions!(project_panel, [ExpandSelectedEntry, CollapseSelectedEntry]);
impl_internal_actions!(project_panel, [Open, ToggleExpanded]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ProjectPanel::expand_selected_entry);
    cx.add_action(ProjectPanel::collapse_selected_entry);
    cx.add_action(ProjectPanel::toggle_expanded);
    cx.add_action(ProjectPanel::select_prev);
    cx.add_action(ProjectPanel::select_next);
    cx.add_action(ProjectPanel::open_entry);
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

            let mut this = Self {
                project: project.clone(),
                list: Default::default(),
                visible_entries: Default::default(),
                expanded_dir_ids: Default::default(),
                selection: None,
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

    fn open_entry(&mut self, action: &Open, cx: &mut ViewContext<Self>) {
        cx.emit(Event::OpenedEntry(action.0));
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

            let mut visible_worktree_entries = Vec::new();
            let mut entry_iter = snapshot.entries(false);
            while let Some(item) = entry_iter.entry() {
                visible_worktree_entries.push(item.clone());
                if expanded_dir_ids.binary_search(&item.id).is_err() {
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
                    let filename = entry.path.file_name().unwrap_or(root_name);
                    let details = EntryDetails {
                        filename: filename.to_string_lossy().to_string(),
                        depth: entry.path.components().count(),
                        is_dir: entry.is_dir(),
                        is_expanded: expanded_entry_ids.binary_search(&entry.id).is_ok(),
                        is_selected: self.selection.map_or(false, |e| {
                            e.worktree_id == snapshot.id() && e.entry_id == entry.id
                        }),
                    };
                    callback(entry.id, details, cx);
                }
            }
            ix = end_ix;
        }
    }

    fn render_entry(
        entry_id: ProjectEntryId,
        details: EntryDetails,
        theme: &theme::ProjectPanel,
        cx: &mut ViewContext<Self>,
    ) -> ElementBox {
        let is_dir = details.is_dir;
        MouseEventHandler::new::<Self, _, _>(entry_id.to_usize(), cx, |state, _| {
            let style = theme.entry.style_for(state, details.is_selected);
            Flex::row()
                .with_child(
                    ConstrainedBox::new(if is_dir {
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
                .with_child(
                    Label::new(details.filename, style.text.clone())
                        .contained()
                        .with_margin_left(style.icon_spacing)
                        .aligned()
                        .left()
                        .boxed(),
                )
                .constrained()
                .with_height(theme.entry.default.height)
                .contained()
                .with_style(style.container)
                .with_padding_left(
                    theme.container.padding.left + details.depth as f32 * theme.indent_width,
                )
                .boxed()
        })
        .on_click(move |cx| {
            if is_dir {
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
                    this.for_each_visible_entry(range.clone(), cx, |entry, details, cx| {
                        items.push(Self::render_entry(entry, details, &theme.project_panel, cx));
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
    use serde_json::json;
    use std::{collections::HashSet, path::Path};
    use workspace::WorkspaceParams;

    #[gpui::test]
    async fn test_visible_list(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let params = cx.update(WorkspaceParams::test);
        let fs = params.fs.as_fake();
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

        let project = cx.update(|cx| {
            Project::local(
                params.client.clone(),
                params.user_store.clone(),
                params.languages.clone(),
                params.fs.clone(),
                cx,
            )
        });
        let (root1, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root1", true, cx)
            })
            .await
            .unwrap();
        root1
            .read_with(cx, |t, _| t.as_local().unwrap().scan_complete())
            .await;
        let (root2, _) = project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root2", true, cx)
            })
            .await
            .unwrap();
        root2
            .read_with(cx, |t, _| t.as_local().unwrap().scan_complete())
            .await;

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        let panel = workspace.update(cx, |_, cx| ProjectPanel::new(project, cx));
        assert_eq!(
            visible_entry_details(&panel, 0..50, cx),
            &[
                EntryDetails {
                    filename: "root1".to_string(),
                    depth: 0,
                    is_dir: true,
                    is_expanded: true,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "a".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "b".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "C".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: ".dockerignore".to_string(),
                    depth: 1,
                    is_dir: false,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "root2".to_string(),
                    depth: 0,
                    is_dir: true,
                    is_expanded: true,
                    is_selected: false
                },
                EntryDetails {
                    filename: "d".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false
                },
                EntryDetails {
                    filename: "e".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false
                }
            ],
        );

        toggle_expand_dir(&panel, "root1/b", cx);
        assert_eq!(
            visible_entry_details(&panel, 0..50, cx),
            &[
                EntryDetails {
                    filename: "root1".to_string(),
                    depth: 0,
                    is_dir: true,
                    is_expanded: true,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "a".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "b".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: true,
                    is_selected: true,
                },
                EntryDetails {
                    filename: "3".to_string(),
                    depth: 2,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "4".to_string(),
                    depth: 2,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "C".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: ".dockerignore".to_string(),
                    depth: 1,
                    is_dir: false,
                    is_expanded: false,
                    is_selected: false,
                },
                EntryDetails {
                    filename: "root2".to_string(),
                    depth: 0,
                    is_dir: true,
                    is_expanded: true,
                    is_selected: false
                },
                EntryDetails {
                    filename: "d".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false
                },
                EntryDetails {
                    filename: "e".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false
                }
            ]
        );

        assert_eq!(
            visible_entry_details(&panel, 5..8, cx),
            [
                EntryDetails {
                    filename: "C".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                    is_selected: false
                },
                EntryDetails {
                    filename: ".dockerignore".to_string(),
                    depth: 1,
                    is_dir: false,
                    is_expanded: false,
                    is_selected: false
                },
                EntryDetails {
                    filename: "root2".to_string(),
                    depth: 0,
                    is_dir: true,
                    is_expanded: true,
                    is_selected: false
                }
            ]
        );

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

        fn visible_entry_details(
            panel: &ViewHandle<ProjectPanel>,
            range: Range<usize>,
            cx: &mut TestAppContext,
        ) -> Vec<EntryDetails> {
            let mut result = Vec::new();
            let mut project_entries = HashSet::new();
            panel.update(cx, |panel, cx| {
                panel.for_each_visible_entry(range, cx, |project_entry, details, _| {
                    assert!(
                        project_entries.insert(project_entry),
                        "duplicate project entry {:?} {:?}",
                        project_entry,
                        details
                    );
                    result.push(details);
                });
            });

            result
        }
    }
}
