use editor::{Bias, Editor, SelectionEffects, scroll::Autoscroll};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, ParentElement, Task, TaskExt, WeakEntity,
    Window,
};
use language::Buffer;
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath, search::SearchResult};
use std::ops::Range;
use text::{Anchor, Point};
use util::{ResultExt as _, paths::PathMatcher};
use workspace::{
    Workspace,
    ui::{
        Button, ButtonSize, ButtonStyle, ContextMenu, ContextMenuEntry, Icon, IconName, IconSize,
        Label, LabelSize, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, prelude::*,
    },
};

use crate::SearchOptions;

const MAX_TODO_MATCHES: usize = 200;
const TODO_MARKER_COLUMN_WIDTH_REMS: f32 = 5.5;

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &zed_actions::todo::Open, window, cx| {
                let project = workspace.project().clone();
                let workspace_handle = cx.entity().downgrade();

                workspace.toggle_modal(window, cx, move |window, cx| {
                    let delegate =
                        TodoPickerDelegate::new(workspace_handle.clone(), project.clone());
                    let preview = picker_preview::editor_preview(project.clone(), window, cx);
                    Picker::uniform_list_with_preview(delegate, preview, window, cx)
                });
            });
        },
    )
    .detach();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TodoMarker {
    Todo,
    Fixme,
    Hack,
    Xxx,
    Note,
    Cleanup,
    Refactor,
    Security,
    Perf,
    Optimize,
    Undone,
}

impl TodoMarker {
    const ALL: [Self; 11] = [
        Self::Todo,
        Self::Fixme,
        Self::Hack,
        Self::Xxx,
        Self::Note,
        Self::Cleanup,
        Self::Refactor,
        Self::Security,
        Self::Perf,
        Self::Optimize,
        Self::Undone,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Todo => "TODO",
            Self::Fixme => "FIXME",
            Self::Hack => "HACK",
            Self::Xxx => "XXX",
            Self::Note => "NOTE",
            Self::Cleanup => "CLEANUP",
            Self::Refactor => "REFACTOR",
            Self::Security => "SECURITY",
            Self::Perf => "PERF",
            Self::Optimize => "OPTIMIZE",
            Self::Undone => "UNDONE",
        }
    }
}

#[derive(Clone, Debug)]
pub struct TodoEntry {
    pub marker: TodoMarker,
    pub project_path: ProjectPath,
    pub buffer: Entity<Buffer>,
    pub anchor_range: Range<Anchor>,
    pub range: Range<usize>,
    pub row: u32,
    pub column: u32,
    pub text: String,
}

impl TodoEntry {
    pub fn new(
        marker: TodoMarker,
        project_path: ProjectPath,
        buffer: Entity<Buffer>,
        anchor_range: Range<Anchor>,
        range: Range<usize>,
        row: u32,
        column: u32,
        text: String,
    ) -> Self {
        Self {
            marker,
            project_path,
            buffer,
            anchor_range,
            range,
            row,
            column,
            text,
        }
    }

    fn candidate_text(&self) -> String {
        format!(
            "{} {}:{} {}",
            self.marker.as_str(),
            self.project_path.path.as_unix_str(),
            self.row + 1,
            self.text
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedTodoComment {
    pub marker: TodoMarker,
    pub column: u32,
    pub text: String,
}

pub fn parse_todo_comment(line: &str) -> Option<ParsedTodoComment> {
    let (marker, marker_start, marker_end) = find_todo_marker(line)?;
    Some(ParsedTodoComment {
        marker,
        column: marker_start as u32,
        text: todo_text_after_marker(line, marker_end),
    })
}

fn find_todo_marker(line: &str) -> Option<(TodoMarker, usize, usize)> {
    let mut best_match = None;

    for marker in TodoMarker::ALL {
        let marker_text = marker.as_str();
        let mut search_start = 0;

        while let Some(relative_start) = line[search_start..].find(marker_text) {
            let marker_start = search_start + relative_start;
            let marker_end = marker_start + marker_text.len();

            if has_word_boundaries(line, marker_start, marker_end)
                && best_match.is_none_or(|(_, best_start, _)| marker_start < best_start)
            {
                best_match = Some((marker, marker_start, marker_end));
            }

            search_start = marker_end;
        }
    }

    best_match
}

fn has_word_boundaries(line: &str, start: usize, end: usize) -> bool {
    let has_start_boundary = line[..start]
        .chars()
        .next_back()
        .map_or(true, |character| !is_word_character(character));
    let has_end_boundary = line[end..]
        .chars()
        .next()
        .map_or(true, |character| !is_word_character(character));

    has_start_boundary && has_end_boundary
}

fn is_word_character(character: char) -> bool {
    character == '_' || character.is_ascii_alphanumeric()
}

fn todo_text_after_marker(line: &str, marker_end: usize) -> String {
    let mut text = line[marker_end..].trim_start();

    if let Some(after_assignee) = text.strip_prefix('(').and_then(|text| {
        text.split_once(')')
            .map(|(_, after_assignee)| after_assignee)
    }) {
        text = after_assignee.trim_start();
    }

    if let Some(stripped) = text.strip_prefix(':').or_else(|| text.strip_prefix('-')) {
        text = stripped.trim_start();
    }

    text.trim().to_string()
}

pub struct TodoPickerDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    selected_match_index: usize,
    entries: Vec<TodoEntry>,
    match_candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    query: String,
    marker_filter: Option<TodoMarker>,
    marker_filter_menu_handle: PopoverMenuHandle<ContextMenu>,
    started_loading: bool,
    loading: bool,
}

impl TodoPickerDelegate {
    pub fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>) -> Self {
        Self {
            workspace,
            project,
            selected_match_index: 0,
            entries: Vec::new(),
            match_candidates: Vec::new(),
            matches: Vec::new(),
            query: String::new(),
            marker_filter: None,
            marker_filter_menu_handle: PopoverMenuHandle::default(),
            started_loading: false,
            loading: false,
        }
    }

    pub fn set_entries(&mut self, entries: Vec<TodoEntry>) {
        self.entries = entries;
        self.match_candidates = self
            .entries
            .iter()
            .enumerate()
            .map(|(id, entry)| StringMatchCandidate::new(id, &entry.candidate_text()))
            .collect();
    }

    fn filter(&mut self, query: &str, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let candidates = self
            .match_candidates
            .iter()
            .filter(|candidate| {
                self.marker_filter
                    .is_none_or(|marker| self.entries[candidate.id].marker == marker)
            })
            .cloned()
            .collect::<Vec<_>>();

        self.matches = cx.foreground_executor().block_on(fuzzy::match_strings(
            &candidates,
            query,
            false,
            true,
            MAX_TODO_MATCHES,
            &Default::default(),
            cx.background_executor().clone(),
        ));
        self.set_selected_index(0, window, cx);
    }

    fn filter_label(&self) -> &'static str {
        self.marker_filter.map(TodoMarker::as_str).unwrap_or("All")
    }

    fn start_loading(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.started_loading = true;
        self.loading = true;

        let project = self.project.clone();
        let match_full_paths = project.read(cx).visible_worktrees(cx).count() > 1;
        let query = SearchOptions::REGEX
            .union(SearchOptions::CASE_SENSITIVE)
            .union(SearchOptions::ONE_MATCH_PER_LINE)
            .build_query(
                todo_search_regex(),
                PathMatcher::default(),
                PathMatcher::default(),
                match_full_paths,
                None,
            );
        let Some(query) = query.log_err() else {
            self.loading = false;
            return;
        };

        let search_results = project.update(cx, |project, cx| project.search(query, cx));
        cx.spawn_in(window, async move |picker, cx| {
            let _search_task = search_results.task_handle;
            let mut entries = Vec::new();

            while let Ok(result) = search_results.rx.recv().await {
                match result {
                    SearchResult::Buffer { buffer, ranges } => {
                        let mut buffer_entries = buffer.read_with(cx, |buffer_snapshot, cx| {
                            todo_entries_for_buffer(&buffer, buffer_snapshot, &ranges, cx)
                        });
                        entries.append(&mut buffer_entries);

                        picker.update_in(cx, |picker, window, cx| {
                            picker.delegate.set_entries(entries.clone());
                            picker.refresh(window, cx);
                        })?;
                    }
                    SearchResult::LimitReached
                    | SearchResult::WaitingForScan
                    | SearchResult::Searching => {}
                }
            }

            picker.update(cx, |picker, cx| {
                picker.delegate.loading = false;
                cx.notify();
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }
}

fn todo_search_regex() -> String {
    format!(
        r"\b({})\b",
        TodoMarker::ALL
            .iter()
            .map(|marker| marker.as_str())
            .collect::<Vec<_>>()
            .join("|")
    )
}

fn todo_entries_for_buffer(
    buffer: &Entity<Buffer>,
    buffer_snapshot: &Buffer,
    ranges: &[Range<Anchor>],
    cx: &App,
) -> Vec<TodoEntry> {
    let Some(file) = buffer_snapshot.file() else {
        return Vec::new();
    };
    let project_path = ProjectPath {
        worktree_id: file.worktree_id(cx),
        path: file.path().clone(),
    };
    let mut entries = Vec::new();

    for range in ranges {
        let start_offset: usize = buffer_snapshot.summary_for_anchor(&range.start);
        let end_offset: usize = buffer_snapshot.summary_for_anchor(&range.end);
        let point = buffer_snapshot.offset_to_point(start_offset);
        let line = buffer_snapshot
            .text_for_range(
                Point::new(point.row, 0)
                    ..Point::new(point.row, buffer_snapshot.line_len(point.row)),
            )
            .collect::<String>();
        let Some(parsed) = parse_todo_comment(&line) else {
            continue;
        };

        entries.push(TodoEntry::new(
            parsed.marker,
            project_path.clone(),
            buffer.clone(),
            range.clone(),
            start_offset..end_offset,
            point.row,
            parsed.column,
            parsed.text,
        ));
    }

    entries
}

impl EventEmitter<DismissEvent> for TodoPickerDelegate {}

impl PickerDelegate for TodoPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "todo picker"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        "Search TODO comments...".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        if self.loading {
            Some("Searching TODO comments...".into())
        } else {
            Some("No TODO comments found".into())
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.query = query.clone();
        if !self.started_loading {
            self.start_loading(window, cx);
        }
        self.filter(&query, window, cx);
        Task::ready(())
    }

    fn searchbar_trailer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let picker = cx.entity();
        let selected_marker = self.marker_filter;

        Some(
            PopoverMenu::new("todo-marker-filter")
                .with_handle(self.marker_filter_menu_handle.clone())
                .trigger(
                    Button::new("todo-marker-filter-trigger", self.filter_label())
                        .style(ButtonStyle::Subtle)
                        .size(ButtonSize::Compact)
                        .end_icon(Icon::new(IconName::ChevronDown).size(IconSize::Small)),
                )
                .menu(move |window, cx| {
                    let picker = picker.clone();

                    Some(ContextMenu::build(
                        window,
                        cx,
                        move |mut menu, _window, _cx| {
                            menu = menu.item(
                                ContextMenuEntry::new("All")
                                    .toggleable(IconPosition::End, selected_marker.is_none())
                                    .handler({
                                        let picker = picker.clone();
                                        move |window, cx| {
                                            picker.update(cx, |picker, cx| {
                                                picker.delegate.marker_filter = None;
                                                picker.refresh(window, cx);
                                            });
                                        }
                                    }),
                            );

                            for marker in TodoMarker::ALL {
                                menu = menu.item(
                                    ContextMenuEntry::new(marker.as_str())
                                        .toggleable(
                                            IconPosition::End,
                                            selected_marker == Some(marker),
                                        )
                                        .handler({
                                            let picker = picker.clone();
                                            move |window, cx| {
                                                picker.update(cx, |picker, cx| {
                                                    picker.delegate.marker_filter = Some(marker);
                                                    picker.refresh(window, cx);
                                                });
                                            }
                                        }),
                                );
                            }

                            menu
                        },
                    ))
                })
                .into_any_element(),
        )
    }

    fn has_another_open_menu(&self, window: &Window, cx: &App) -> bool {
        self.marker_filter_menu_handle.is_deployed()
            || self.marker_filter_menu_handle.is_focused(window, cx)
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self
            .matches
            .get(self.selected_match_index)
            .and_then(|string_match| self.entries.get(string_match.candidate_id))
            .cloned()
        else {
            return;
        };

        let open_buffer = self
            .project
            .update(cx, |project, cx| project.open_path(entry.project_path, cx));
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            let (_, buffer) = open_buffer.await?;
            workspace.update_in(cx, |workspace, window, cx| {
                let position = buffer
                    .read(cx)
                    .snapshot()
                    .clip_point(Point::new(entry.row, entry.column), Bias::Left);
                let pane = if secondary {
                    workspace.adjacent_pane(window, cx)
                } else {
                    workspace.active_pane().clone()
                };
                let editor = workspace
                    .open_project_item::<Editor>(pane, buffer, true, true, true, true, window, cx);

                editor.update(cx, |editor, cx| {
                    let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                    let Some(buffer_snapshot) = multibuffer_snapshot.as_singleton() else {
                        return;
                    };
                    let text_anchor = buffer_snapshot.anchor_before(position);
                    let Some(anchor) = multibuffer_snapshot.anchor_in_buffer(text_anchor) else {
                        return;
                    };
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |selections| selections.select_ranges([anchor..anchor]),
                    );
                });
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn try_get_preview_data_for_match(&self, _cx: &App) -> Option<picker::PreviewUpdate> {
        let entry = self
            .matches
            .get(self.selected_match_index)
            .and_then(|string_match| self.entries.get(string_match.candidate_id))?;

        Some(picker::PreviewUpdate::from_buffer(
            entry.buffer.clone(),
            picker::MatchLocation {
                anchor_range: entry.anchor_range.clone(),
                range: entry.range.clone(),
            },
        ))
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_match_index = ix;
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let string_match = self.matches.get(ix)?;
        let entry = self.entries.get(string_match.candidate_id)?;
        let project = self.project.read(cx);
        let path_style = project.path_style(cx);
        let show_worktree_root_name = project.visible_worktrees(cx).count() > 1;
        let mut path = entry.project_path.path.clone();

        if show_worktree_root_name
            && let Some(worktree) = project.worktree_for_id(entry.project_path.worktree_id, cx)
        {
            path = worktree.read(cx).root_name().join(&path);
        }

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .min_w_0()
                        .gap_2()
                        .child(
                            h_flex()
                                .w(rems(TODO_MARKER_COLUMN_WIDTH_REMS))
                                .flex_none()
                                .justify_end()
                                .child(
                                    Label::new(entry.marker.as_str())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                        .child(
                            v_flex()
                                .flex_1()
                                .min_w_0()
                                .child(Label::new(entry.text.clone()))
                                .child(
                                    h_flex()
                                        .min_w_0()
                                        .child(
                                            Label::new(path.display(path_style).into_owned())
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new(format!(":{}", entry.row + 1))
                                                .size(LabelSize::Small)
                                                .color(Color::Placeholder),
                                        ),
                                ),
                        ),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_todo_comments() {
        for marker in TodoMarker::ALL {
            let line = format!("// {}: handle remote projects", marker.as_str());
            assert_eq!(
                parse_todo_comment(&line),
                Some(ParsedTodoComment {
                    marker,
                    column: 3,
                    text: "handle remote projects".to_string(),
                }),
                "failed to parse {}",
                marker.as_str()
            );
        }
    }

    #[test]
    fn parses_assignees_and_dash_separators() {
        for marker in TodoMarker::ALL {
            assert_eq!(
                parse_todo_comment(&format!("# {}(user): avoid panic here", marker.as_str())),
                Some(ParsedTodoComment {
                    marker,
                    column: 2,
                    text: "avoid panic here".to_string(),
                }),
                "failed to parse assignee for {}",
                marker.as_str()
            );
            assert_eq!(
                parse_todo_comment(&format!("// {} - temporary", marker.as_str())),
                Some(ParsedTodoComment {
                    marker,
                    column: 3,
                    text: "temporary".to_string(),
                }),
                "failed to parse dash separator for {}",
                marker.as_str()
            );
        }
    }

    #[test]
    fn ignores_markers_inside_words() {
        assert_eq!(parse_todo_comment("METHODONOTE"), None);
        assert_eq!(parse_todo_comment("TODO_ITEM"), None);
        assert_eq!(parse_todo_comment("FIXME2"), None);
    }
}
