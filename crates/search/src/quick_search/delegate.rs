use std::cell::{Cell, RefCell};
use std::ops::Range;
use std::sync::Arc;

use collections::{HashMap, HashSet};
use editor::{
    Editor, EditorSettings, HighlightKey, RowHighlightOptions, SelectionEffects, scroll::Autoscroll,
};
use gpui::{App, AsyncApp, Context, Entity, WeakEntity, Window};
use language::Buffer;
use picker::Picker;
use project::search::{SearchInputKind, SearchQuery};
use project::search_history::SearchHistoryCursor;
use project::{Project, ProjectPath};
use settings::Settings;
use text::{Anchor, Point};
use ui::{ActiveTheme, ContextMenu, PopoverMenuHandle};
use ui_input::ErasedEditor;
use util::paths::PathMatcher;
use workspace::Workspace;

use super::{InputPanel, SearchMatch, SearchMatchLineHighlight};
use crate::SearchOptions;

mod picker_impl;

pub struct QuickSearchDelegate {
    pub(crate) workspace: WeakEntity<Workspace>,
    pub(crate) project: Entity<Project>,
    pub(crate) preview_editor: Entity<Editor>,
    pub(crate) replacement_editor: Arc<dyn ErasedEditor>,
    pub(crate) included_files_editor: Arc<dyn ErasedEditor>,
    pub(crate) excluded_files_editor: Arc<dyn ErasedEditor>,
    pub(crate) replace_enabled: bool,
    pub(crate) filters_enabled: bool,
    pub(crate) included_opened_only: bool,
    pub(crate) matches: Vec<SearchMatch>,
    pub(crate) selected_index: usize,
    pub(crate) cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) last_selection_change_time: Option<std::time::Instant>,
    pub(crate) last_click: Option<(usize, std::time::Instant)>,
    pub(crate) search_options: SearchOptions,
    pub(crate) search_in_progress: bool,
    pub(crate) pending_initial_query: RefCell<Option<String>>,
    pub(crate) editor_configured: Cell<bool>,
    pub(crate) panels_with_errors: HashMap<InputPanel, String>,
    pub(crate) split_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    pub(crate) history_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    pub(crate) search_history_cursor: SearchHistoryCursor,
    pub(crate) file_count: usize,
    pub(crate) unique_files: HashSet<ProjectPath>,
}

impl QuickSearchDelegate {
    pub(crate) fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        preview_editor: Entity<Editor>,
        replacement_editor: Arc<dyn ErasedEditor>,
        included_files_editor: Arc<dyn ErasedEditor>,
        excluded_files_editor: Arc<dyn ErasedEditor>,
        initial_query: Option<String>,
        cx: &App,
    ) -> Self {
        Self {
            workspace,
            project,
            preview_editor,
            replacement_editor,
            included_files_editor,
            excluded_files_editor,
            replace_enabled: false,
            filters_enabled: false,
            included_opened_only: false,
            matches: Vec::new(),
            selected_index: 0,
            cancel_flag: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            last_selection_change_time: None,
            last_click: None,
            search_options: SearchOptions::from_settings(&EditorSettings::get_global(cx).search),
            search_in_progress: false,
            pending_initial_query: RefCell::new(initial_query),
            editor_configured: Cell::new(false),
            panels_with_errors: HashMap::default(),
            split_popover_menu_handle: PopoverMenuHandle::default(),
            history_popover_menu_handle: PopoverMenuHandle::default(),
            search_history_cursor: SearchHistoryCursor::default(),
            file_count: 0,
            unique_files: HashSet::default(),
        }
    }

    pub(crate) fn open_buffers(&self, cx: &App) -> Vec<Entity<Buffer>> {
        let mut buffers = Vec::new();
        if let Some(workspace) = self.workspace.upgrade() {
            let workspace = workspace.read(cx);
            for editor in workspace.items_of_type::<Editor>(cx) {
                if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                    buffers.push(buffer);
                }
            }
        }
        buffers
    }

    pub(crate) fn update_preview(&self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_match) = self.matches.get(self.selected_index) else {
            self.preview_editor.update(cx, |editor, cx| {
                editor.buffer().update(cx, |multi_buffer, cx| {
                    if !multi_buffer.read(cx).is_empty() {
                        multi_buffer.clear(cx);
                    }
                });
            });
            return;
        };

        let buffer = selected_match.buffer.clone();
        let range = selected_match.range.clone();
        let anchor_range = selected_match.anchor_range.clone();

        self.preview_editor.update(cx, |editor, cx| {
            let multi_buffer = editor.buffer().clone();
            let max_point = buffer.read(cx).max_point();

            multi_buffer.update(cx, |multi_buffer, cx| {
                multi_buffer.clear(cx);
                multi_buffer.set_excerpts_for_buffer(
                    buffer.clone(),
                    [Point::new(0, 0)..max_point],
                    0,
                    cx,
                );
            });

            let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
            if let (Some(start_anchor), Some(end_anchor)) = (
                multi_buffer_snapshot.anchor_in_excerpt(anchor_range.start),
                multi_buffer_snapshot.anchor_in_excerpt(anchor_range.end),
            ) {
                editor.highlight_rows::<SearchMatchLineHighlight>(
                    start_anchor..start_anchor,
                    cx.theme().colors().editor_active_line_background,
                    RowHighlightOptions::default(),
                    cx,
                );

                editor.highlight_background(
                    HighlightKey::QuickSearchView,
                    &[start_anchor..end_anchor],
                    |_, theme| theme.colors().search_match_background,
                    cx,
                );
            }

            let start = multi_buffer::MultiBufferOffset(range.start);
            let end = multi_buffer::MultiBufferOffset(range.end);
            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::center()),
                window,
                cx,
                |s| {
                    s.select_ranges([start..end]);
                },
            );
        });
    }

    pub(crate) fn parse_path_matches(&self, text: String, cx: &App) -> anyhow::Result<PathMatcher> {
        let path_style = self.project.read(cx).path_style(cx);
        let queries: Vec<String> = text
            .split(',')
            .map(str::trim)
            .filter(|maybe_glob_str| !maybe_glob_str.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        Ok(PathMatcher::new(&queries, path_style)?)
    }

    pub(crate) fn clear_panel_error(&mut self, panel: InputPanel, cx: &mut Context<Picker<Self>>) {
        if self.panels_with_errors.remove(&panel).is_some() {
            cx.notify();
        }
    }

    pub(crate) fn set_panel_error(
        &mut self,
        panel: InputPanel,
        message: String,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.panels_with_errors.insert(panel, message).is_none() {
            cx.notify();
        }
    }

    pub(crate) fn path_matcher_for_panel(
        &mut self,
        panel: InputPanel,
        editor: &Arc<dyn ErasedEditor>,
        cx: &mut Context<Picker<Self>>,
    ) -> PathMatcher {
        if !self.filters_enabled {
            self.clear_panel_error(panel, cx);
            return PathMatcher::default();
        }

        match self.parse_path_matches(editor.text(cx), cx) {
            Ok(path_matcher) => {
                self.clear_panel_error(panel, cx);
                path_matcher
            }
            Err(error) => {
                self.set_panel_error(panel, error.to_string(), cx);
                PathMatcher::default()
            }
        }
    }

    pub(crate) fn build_search_query(
        &mut self,
        query: &str,
        open_buffers: Option<Vec<Entity<Buffer>>>,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<SearchQuery> {
        if query.is_empty() {
            self.panels_with_errors.remove(&InputPanel::Query);
            return None;
        }

        let included_files_editor = self.included_files_editor.clone();
        let excluded_files_editor = self.excluded_files_editor.clone();
        let files_to_include =
            self.path_matcher_for_panel(InputPanel::Include, &included_files_editor, cx);
        let files_to_exclude =
            self.path_matcher_for_panel(InputPanel::Exclude, &excluded_files_editor, cx);

        // If the project contains multiple visible worktrees, we match the
        // include/exclude patterns against full paths to allow them to be
        // disambiguated. For single worktree projects we use worktree relative
        // paths for convenience.
        let match_full_paths = self.project.read(cx).visible_worktrees(cx).count() > 1;

        let result = if self.search_options.contains(SearchOptions::REGEX) {
            SearchQuery::regex(
                query,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                self.search_options
                    .contains(SearchOptions::ONE_MATCH_PER_LINE),
                files_to_include,
                files_to_exclude,
                match_full_paths,
                open_buffers,
            )
        } else {
            SearchQuery::text(
                query,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                files_to_include,
                files_to_exclude,
                match_full_paths,
                open_buffers,
            )
        };

        match result {
            Ok(search_query) => {
                if self.panels_with_errors.remove(&InputPanel::Query).is_some() {
                    cx.notify();
                }
                Some(search_query)
            }
            Err(e) => {
                if self
                    .panels_with_errors
                    .insert(InputPanel::Query, e.to_string())
                    .is_none()
                {
                    cx.notify();
                }
                None
            }
        }
    }

    pub(crate) fn process_search_result(
        buffer: &Entity<Buffer>,
        ranges: &[Range<Anchor>],
        cx: &AsyncApp,
    ) -> Vec<SearchMatch> {
        if ranges.is_empty() {
            return Vec::new();
        }

        buffer.read_with(cx, |buf, cx| {
            let file = buf.file();
            let path = file.map(|f| ProjectPath {
                worktree_id: f.worktree_id(cx),
                path: f.path().clone(),
            });
            let text = buf.text();

            let mut matches = Vec::new();
            for anchor_range in ranges {
                let start_offset: usize = buf.summary_for_anchor(&anchor_range.start);
                let end_offset: usize = buf.summary_for_anchor(&anchor_range.end);
                let match_row = buf.offset_to_point(start_offset).row;
                let line_number = match_row + 1;
                let line_start = text[..start_offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
                let line_end = text[start_offset..]
                    .find('\n')
                    .map(|i| start_offset + i)
                    .unwrap_or(text.len());
                let line_text = text[line_start..line_end].to_string();

                let relative_start = start_offset - line_start;
                let relative_end = end_offset - line_start;

                if let Some(path) = &path {
                    matches.push(SearchMatch {
                        path: path.clone(),
                        buffer: buffer.clone(),
                        anchor_range: anchor_range.clone(),
                        range: start_offset..end_offset,
                        relative_range: relative_start..relative_end,
                        line_text,
                        line_number,
                    });
                }
            }
            matches
        })
    }
    pub(crate) fn render_history_menu(
        project: &Entity<Project>,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Entity<ContextMenu>> {
        let history_entries: Vec<String> = project
            .read(cx)
            .search_history(SearchInputKind::Query)
            .iter()
            .map(str::to_string)
            .collect();

        let editor = editor.clone();
        Some(ContextMenu::build(
            window,
            cx,
            move |mut menu, _window, _| {
                if history_entries.is_empty() {
                    menu.header("No recent searches")
                } else {
                    for query in history_entries {
                        let editor = editor.clone();
                        let label = query.clone();
                        menu = menu.entry(label, None, move |window, cx| {
                            editor.set_text(&query, window, cx);
                        });
                    }
                    menu
                }
            },
        ))
    }
}
