use crate::{
    BufferSearchBar, FocusSearch, NextHistoryQuery, PreviousHistoryQuery, ReplaceAll, ReplaceNext,
    SearchOption, SearchOptions, SearchSource, SelectNextMatch, SelectPreviousMatch,
    ToggleCaseSensitive, ToggleIncludeIgnored, ToggleRegex, ToggleReplace, ToggleWholeWord,
    buffer_search::Deploy,
    search_bar::{ActionButtonState, input_base_styles, render_action_button, render_text_input},
};
use anyhow::Context as _;
use collections::HashMap;
use editor::{
    Anchor, Editor, EditorEvent, EditorSettings, MAX_TAB_TITLE_LEN, MultiBuffer, SelectionEffects,
    actions::{Backtab, SelectAll, Tab},
    items::active_match_index,
};
use futures::{StreamExt, stream::FuturesOrdered};
use gpui::{
    Action, AnyElement, AnyView, App, Axis, Context, Entity, EntityId, EventEmitter, FocusHandle,
    Focusable, Global, Hsla, InteractiveElement, IntoElement, KeyContext, ParentElement, Point,
    Render, SharedString, Styled, Subscription, Task, UpdateGlobal, WeakEntity, Window, actions,
    div,
};
use language::{Buffer, Language};
use menu::Confirm;
use project::{
    Project, ProjectPath,
    search::{SearchInputKind, SearchQuery},
    search_history::SearchHistoryCursor,
};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    mem,
    ops::{Not, Range},
    path::Path,
    pin::pin,
    sync::Arc,
};
use ui::{
    Icon, IconButton, IconButtonShape, IconName, KeyBinding, Label, LabelCommon, LabelSize,
    Toggleable, Tooltip, h_flex, prelude::*, utils::SearchInputWidth, v_flex,
};
use util::{ResultExt as _, paths::PathMatcher};
use workspace::{
    DeploySearch, ItemNavHistory, NewSearch, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace, WorkspaceId,
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle, SaveOptions},
    searchable::{Direction, SearchableItem, SearchableItemHandle},
};

actions!(
    project_search,
    [
        /// Searches in a new project search tab.
        SearchInNew,
        /// Toggles focus between the search bar and the search results.
        ToggleFocus,
        /// Moves to the next input field.
        NextField,
        /// Toggles the search filters panel.
        ToggleFilters
    ]
);

#[derive(Default)]
struct ActiveSettings(HashMap<WeakEntity<Project>, ProjectSearchSettings>);

impl Global for ActiveSettings {}

pub fn init(cx: &mut App) {
    cx.set_global(ActiveSettings::default());
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        register_workspace_action(workspace, move |search_bar, _: &Deploy, window, cx| {
            search_bar.focus_search(window, cx);
        });
        register_workspace_action(workspace, move |search_bar, _: &FocusSearch, window, cx| {
            search_bar.focus_search(window, cx);
        });
        register_workspace_action(
            workspace,
            move |search_bar, _: &ToggleFilters, window, cx| {
                search_bar.toggle_filters(window, cx);
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, _: &ToggleCaseSensitive, window, cx| {
                search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, _: &ToggleWholeWord, window, cx| {
                search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
            },
        );
        register_workspace_action(workspace, move |search_bar, _: &ToggleRegex, window, cx| {
            search_bar.toggle_search_option(SearchOptions::REGEX, window, cx);
        });
        register_workspace_action(
            workspace,
            move |search_bar, action: &ToggleReplace, window, cx| {
                search_bar.toggle_replace(action, window, cx)
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, action: &SelectPreviousMatch, window, cx| {
                search_bar.select_prev_match(action, window, cx)
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, action: &SelectNextMatch, window, cx| {
                search_bar.select_next_match(action, window, cx)
            },
        );

        // Only handle search_in_new if there is a search present
        register_workspace_action_for_present_search(workspace, |workspace, action, window, cx| {
            ProjectSearchView::search_in_new(workspace, action, window, cx)
        });

        register_workspace_action_for_present_search(
            workspace,
            |workspace, _: &menu::Cancel, window, cx| {
                if let Some(project_search_bar) = workspace
                    .active_pane()
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<ProjectSearchBar>()
                {
                    project_search_bar.update(cx, |project_search_bar, cx| {
                        let search_is_focused = project_search_bar
                            .active_project_search
                            .as_ref()
                            .is_some_and(|search_view| {
                                search_view
                                    .read(cx)
                                    .query_editor
                                    .read(cx)
                                    .focus_handle(cx)
                                    .is_focused(window)
                            });
                        if search_is_focused {
                            project_search_bar.move_focus_to_results(window, cx);
                        } else {
                            project_search_bar.focus_search(window, cx)
                        }
                    });
                } else {
                    cx.propagate();
                }
            },
        );

        // Both on present and dismissed search, we need to unconditionally handle those actions to focus from the editor.
        workspace.register_action(move |workspace, action: &DeploySearch, window, cx| {
            if workspace.has_active_modal(window, cx) {
                cx.propagate();
                return;
            }
            ProjectSearchView::deploy_search(workspace, action, window, cx);
            cx.notify();
        });
        workspace.register_action(move |workspace, action: &NewSearch, window, cx| {
            if workspace.has_active_modal(window, cx) {
                cx.propagate();
                return;
            }
            ProjectSearchView::new_search(workspace, action, window, cx);
            cx.notify();
        });
    })
    .detach();
}

fn contains_uppercase(str: &str) -> bool {
    str.chars().any(|c| c.is_uppercase())
}

pub struct ProjectSearch {
    project: Entity<Project>,
    excerpts: Entity<MultiBuffer>,
    pending_search: Option<Task<Option<()>>>,
    match_ranges: Vec<Range<Anchor>>,
    active_query: Option<SearchQuery>,
    last_search_query_text: Option<String>,
    search_id: usize,
    no_results: Option<bool>,
    limit_reached: bool,
    search_history_cursor: SearchHistoryCursor,
    search_included_history_cursor: SearchHistoryCursor,
    search_excluded_history_cursor: SearchHistoryCursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputPanel {
    Query,
    Replacement,
    Exclude,
    Include,
}

pub struct ProjectSearchView {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    entity: Entity<ProjectSearch>,
    query_editor: Entity<Editor>,
    replacement_editor: Entity<Editor>,
    results_editor: Entity<Editor>,
    search_options: SearchOptions,
    panels_with_errors: HashMap<InputPanel, String>,
    active_match_index: Option<usize>,
    search_id: usize,
    included_files_editor: Entity<Editor>,
    excluded_files_editor: Entity<Editor>,
    filters_enabled: bool,
    replace_enabled: bool,
    included_opened_only: bool,
    regex_language: Option<Arc<Language>>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone)]
pub struct ProjectSearchSettings {
    search_options: SearchOptions,
    filters_enabled: bool,
}

pub struct ProjectSearchBar {
    active_project_search: Option<Entity<ProjectSearchView>>,
    subscription: Option<Subscription>,
}

impl ProjectSearch {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let capability = project.read(cx).capability();

        Self {
            project,
            excerpts: cx.new(|_| MultiBuffer::new(capability)),
            pending_search: Default::default(),
            match_ranges: Default::default(),
            active_query: None,
            last_search_query_text: None,
            search_id: 0,
            no_results: None,
            limit_reached: false,
            search_history_cursor: Default::default(),
            search_included_history_cursor: Default::default(),
            search_excluded_history_cursor: Default::default(),
        }
    }

    fn clone(&self, cx: &mut Context<Self>) -> Entity<Self> {
        cx.new(|cx| Self {
            project: self.project.clone(),
            excerpts: self
                .excerpts
                .update(cx, |excerpts, cx| cx.new(|cx| excerpts.clone(cx))),
            pending_search: Default::default(),
            match_ranges: self.match_ranges.clone(),
            active_query: self.active_query.clone(),
            last_search_query_text: self.last_search_query_text.clone(),
            search_id: self.search_id,
            no_results: self.no_results,
            limit_reached: self.limit_reached,
            search_history_cursor: self.search_history_cursor.clone(),
            search_included_history_cursor: self.search_included_history_cursor.clone(),
            search_excluded_history_cursor: self.search_excluded_history_cursor.clone(),
        })
    }
    fn cursor(&self, kind: SearchInputKind) -> &SearchHistoryCursor {
        match kind {
            SearchInputKind::Query => &self.search_history_cursor,
            SearchInputKind::Include => &self.search_included_history_cursor,
            SearchInputKind::Exclude => &self.search_excluded_history_cursor,
        }
    }
    fn cursor_mut(&mut self, kind: SearchInputKind) -> &mut SearchHistoryCursor {
        match kind {
            SearchInputKind::Query => &mut self.search_history_cursor,
            SearchInputKind::Include => &mut self.search_included_history_cursor,
            SearchInputKind::Exclude => &mut self.search_excluded_history_cursor,
        }
    }

    fn search(&mut self, query: SearchQuery, cx: &mut Context<Self>) {
        let search = self.project.update(cx, |project, cx| {
            project
                .search_history_mut(SearchInputKind::Query)
                .add(&mut self.search_history_cursor, query.as_str().to_string());
            let included = query.as_inner().files_to_include().sources().join(",");
            if !included.is_empty() {
                project
                    .search_history_mut(SearchInputKind::Include)
                    .add(&mut self.search_included_history_cursor, included);
            }
            let excluded = query.as_inner().files_to_exclude().sources().join(",");
            if !excluded.is_empty() {
                project
                    .search_history_mut(SearchInputKind::Exclude)
                    .add(&mut self.search_excluded_history_cursor, excluded);
            }
            project.search(query.clone(), cx)
        });
        self.last_search_query_text = Some(query.as_str().to_string());
        self.search_id += 1;
        self.active_query = Some(query);
        self.match_ranges.clear();
        self.pending_search = Some(cx.spawn(async move |project_search, cx| {
            let mut matches = pin!(search.ready_chunks(1024));
            project_search
                .update(cx, |project_search, cx| {
                    project_search.match_ranges.clear();
                    project_search
                        .excerpts
                        .update(cx, |excerpts, cx| excerpts.clear(cx));
                    project_search.no_results = Some(true);
                    project_search.limit_reached = false;
                })
                .ok()?;

            let mut limit_reached = false;
            while let Some(results) = matches.next().await {
                let mut buffers_with_ranges = Vec::with_capacity(results.len());
                for result in results {
                    match result {
                        project::search::SearchResult::Buffer { buffer, ranges } => {
                            buffers_with_ranges.push((buffer, ranges));
                        }
                        project::search::SearchResult::LimitReached => {
                            limit_reached = true;
                        }
                    }
                }

                let mut new_ranges = project_search
                    .update(cx, |project_search, cx| {
                        project_search.excerpts.update(cx, |excerpts, cx| {
                            buffers_with_ranges
                                .into_iter()
                                .map(|(buffer, ranges)| {
                                    excerpts.set_anchored_excerpts_for_path(
                                        buffer,
                                        ranges,
                                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
                                        cx,
                                    )
                                })
                                .collect::<FuturesOrdered<_>>()
                        })
                    })
                    .ok()?;

                while let Some(new_ranges) = new_ranges.next().await {
                    project_search
                        .update(cx, |project_search, cx| {
                            project_search.match_ranges.extend(new_ranges);
                            cx.notify();
                        })
                        .ok()?;
                }
            }

            project_search
                .update(cx, |project_search, cx| {
                    if !project_search.match_ranges.is_empty() {
                        project_search.no_results = Some(false);
                    }
                    project_search.limit_reached = limit_reached;
                    project_search.pending_search.take();
                    cx.notify();
                })
                .ok()?;

            None
        }));
        cx.notify();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViewEvent {
    UpdateTab,
    Activate,
    EditorEvent(editor::EditorEvent),
    Dismiss,
}

impl EventEmitter<ViewEvent> for ProjectSearchView {}

impl Render for ProjectSearchView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.has_matches() {
            div()
                .flex_1()
                .size_full()
                .track_focus(&self.focus_handle(cx))
                .child(self.results_editor.clone())
        } else {
            let model = self.entity.read(cx);
            let has_no_results = model.no_results.unwrap_or(false);
            let is_search_underway = model.pending_search.is_some();

            let heading_text = if is_search_underway {
                "Searching…"
            } else if has_no_results {
                "No Results"
            } else {
                "Search All Files"
            };

            let heading_text = div()
                .justify_center()
                .child(Label::new(heading_text).size(LabelSize::Large));

            let page_content: Option<AnyElement> = if let Some(no_results) = model.no_results {
                if model.pending_search.is_none() && no_results {
                    Some(
                        Label::new("No results found in this project for the provided query")
                            .size(LabelSize::Small)
                            .into_any_element(),
                    )
                } else {
                    None
                }
            } else {
                Some(self.landing_text_minor(window, cx).into_any_element())
            };

            let page_content = page_content.map(|text| div().child(text));

            h_flex()
                .size_full()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .bg(cx.theme().colors().editor_background)
                .track_focus(&self.focus_handle(cx))
                .child(
                    v_flex()
                        .id("project-search-landing-page")
                        .overflow_y_scroll()
                        .gap_1()
                        .child(heading_text)
                        .children(page_content),
                )
        }
    }
}

impl Focusable for ProjectSearchView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ProjectSearchView {
    type Event = ViewEvent;
    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let query_text = self.query_editor.read(cx).text(cx);

        query_text
            .is_empty()
            .not()
            .then(|| query_text.into())
            .or_else(|| Some("Project Search".into()))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.results_editor.clone().into())
        } else {
            None
        }
    }
    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.results_editor.clone()))
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.results_editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::MagnifyingGlass))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let last_query: Option<SharedString> = self
            .entity
            .read(cx)
            .last_search_query_text
            .as_ref()
            .map(|query| {
                let query = query.replace('\n', "");
                let query_text = util::truncate_and_trailoff(&query, MAX_TAB_TITLE_LEN);
                query_text.into()
            });

        last_query
            .filter(|query| !query.is_empty())
            .unwrap_or_else(|| "Project Search".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Project Search Opened")
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    ) {
        self.results_editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &App) -> bool {
        false
    }

    fn can_save(&self, _: &App) -> bool {
        true
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.results_editor.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.results_editor.read(cx).has_conflict(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.save(options, project, window, cx))
    }

    fn save_as(
        &mut self,
        _: Entity<Project>,
        _: ProjectPath,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called")
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.reload(project, window, cx))
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        let model = self.entity.update(cx, |model, cx| model.clone(cx));
        Some(cx.new(|cx| Self::new(self.workspace.clone(), model, window, cx, None)))
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.results_editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.results_editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(
        &mut self,
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.results_editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        match event {
            ViewEvent::UpdateTab => {
                f(ItemEvent::UpdateBreadcrumbs);
                f(ItemEvent::UpdateTab);
            }
            ViewEvent::EditorEvent(editor_event) => {
                Editor::to_item_events(editor_event, f);
            }
            ViewEvent::Dismiss => f(ItemEvent::CloseItem),
            _ => {}
        }
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        if self.has_matches() {
            ToolbarItemLocation::Secondary
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.results_editor.breadcrumbs(theme, cx)
    }
}

impl ProjectSearchView {
    pub fn get_matches(&self, cx: &App) -> Vec<Range<Anchor>> {
        self.entity.read(cx).match_ranges.clone()
    }

    fn toggle_filters(&mut self, cx: &mut Context<Self>) {
        self.filters_enabled = !self.filters_enabled;
        ActiveSettings::update_global(cx, |settings, cx| {
            settings.0.insert(
                self.entity.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });
    }

    fn current_settings(&self) -> ProjectSearchSettings {
        ProjectSearchSettings {
            search_options: self.search_options,
            filters_enabled: self.filters_enabled,
        }
    }

    fn toggle_search_option(&mut self, option: SearchOptions, cx: &mut Context<Self>) {
        self.search_options.toggle(option);
        ActiveSettings::update_global(cx, |settings, cx| {
            settings.0.insert(
                self.entity.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });
        self.adjust_query_regex_language(cx);
    }

    fn toggle_opened_only(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.included_opened_only = !self.included_opened_only;
    }

    fn replace_next(&mut self, _: &ReplaceNext, window: &mut Window, cx: &mut Context<Self>) {
        if self.entity.read(cx).match_ranges.is_empty() {
            return;
        }
        let Some(active_index) = self.active_match_index else {
            return;
        };

        let query = self.entity.read(cx).active_query.clone();
        if let Some(query) = query {
            let query = query.with_replacement(self.replacement(cx));

            // TODO: Do we need the clone here?
            let mat = self.entity.read(cx).match_ranges[active_index].clone();
            self.results_editor.update(cx, |editor, cx| {
                editor.replace(&mat, &query, window, cx);
            });
            self.select_match(Direction::Next, window, cx)
        }
    }
    pub fn replacement(&self, cx: &App) -> String {
        self.replacement_editor.read(cx).text(cx)
    }
    fn replace_all(&mut self, _: &ReplaceAll, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_match_index.is_none() {
            return;
        }

        let Some(query) = self.entity.read(cx).active_query.as_ref() else {
            return;
        };
        let query = query.clone().with_replacement(self.replacement(cx));

        let match_ranges = self
            .entity
            .update(cx, |model, _| mem::take(&mut model.match_ranges));
        if match_ranges.is_empty() {
            return;
        }

        self.results_editor.update(cx, |editor, cx| {
            editor.replace_all(&mut match_ranges.iter(), &query, window, cx);
        });

        self.entity.update(cx, |model, _cx| {
            model.match_ranges = match_ranges;
        });
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        entity: Entity<ProjectSearch>,
        window: &mut Window,
        cx: &mut Context<Self>,
        settings: Option<ProjectSearchSettings>,
    ) -> Self {
        let project;
        let excerpts;
        let mut replacement_text = None;
        let mut query_text = String::new();
        let mut subscriptions = Vec::new();

        // Read in settings if available
        let (mut options, filters_enabled) = if let Some(settings) = settings {
            (settings.search_options, settings.filters_enabled)
        } else {
            let search_options =
                SearchOptions::from_settings(&EditorSettings::get_global(cx).search);
            (search_options, false)
        };

        {
            let entity = entity.read(cx);
            project = entity.project.clone();
            excerpts = entity.excerpts.clone();
            if let Some(active_query) = entity.active_query.as_ref() {
                query_text = active_query.as_str().to_string();
                replacement_text = active_query.replacement().map(ToOwned::to_owned);
                options = SearchOptions::from_query(active_query);
            }
        }
        subscriptions.push(cx.observe_in(&entity, window, |this, _, window, cx| {
            this.entity_changed(window, cx)
        }));

        let query_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search all files…", cx);
            editor.set_text(query_text, window, cx);
            editor
        });
        // Subscribe to query_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(
            cx.subscribe(&query_editor, |this, _, event: &EditorEvent, cx| {
                if let EditorEvent::Edited { .. } = event
                    && EditorSettings::get_global(cx).use_smartcase_search
                {
                    let query = this.search_query_text(cx);
                    if !query.is_empty()
                        && this.search_options.contains(SearchOptions::CASE_SENSITIVE)
                            != contains_uppercase(&query)
                    {
                        this.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
                    }
                }
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            }),
        );
        let replacement_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Replace in project…", cx);
            if let Some(text) = replacement_text {
                editor.set_text(text, window, cx);
            }
            editor
        });
        let results_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(excerpts, Some(project.clone()), window, cx);
            editor.set_searchable(false);
            editor.set_in_project_search(true);
            editor
        });
        subscriptions.push(cx.observe(&results_editor, |_, _, cx| cx.emit(ViewEvent::UpdateTab)));

        subscriptions.push(
            cx.subscribe(&results_editor, |this, _, event: &EditorEvent, cx| {
                if matches!(event, editor::EditorEvent::SelectionsChanged { .. }) {
                    this.update_match_index(cx);
                }
                // Reraise editor events for workspace item activation purposes
                cx.emit(ViewEvent::EditorEvent(event.clone()));
            }),
        );

        let included_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Include: crates/**/*.toml", cx);

            editor
        });
        // Subscribe to include_files_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(
            cx.subscribe(&included_files_editor, |_, _, event: &EditorEvent, cx| {
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            }),
        );

        let excluded_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Exclude: vendor/*, *.lock", cx);

            editor
        });
        // Subscribe to excluded_files_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(
            cx.subscribe(&excluded_files_editor, |_, _, event: &EditorEvent, cx| {
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            }),
        );

        let focus_handle = cx.focus_handle();
        subscriptions.push(cx.on_focus(&focus_handle, window, |_, window, cx| {
            cx.on_next_frame(window, |this, window, cx| {
                if this.focus_handle.is_focused(window) {
                    if this.has_matches() {
                        this.results_editor.focus_handle(cx).focus(window);
                    } else {
                        this.query_editor.focus_handle(cx).focus(window);
                    }
                }
            });
        }));

        let languages = project.read(cx).languages().clone();
        cx.spawn(async move |project_search_view, cx| {
            let regex_language = languages
                .language_for_name("regex")
                .await
                .context("loading regex language")?;
            project_search_view
                .update(cx, |project_search_view, cx| {
                    project_search_view.regex_language = Some(regex_language);
                    project_search_view.adjust_query_regex_language(cx);
                })
                .ok();
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        // Check if Worktrees have all been previously indexed
        let mut this = ProjectSearchView {
            workspace,
            focus_handle,
            replacement_editor,
            search_id: entity.read(cx).search_id,
            entity,
            query_editor,
            results_editor,
            search_options: options,
            panels_with_errors: HashMap::default(),
            active_match_index: None,
            included_files_editor,
            excluded_files_editor,
            filters_enabled,
            replace_enabled: false,
            included_opened_only: false,
            regex_language: None,
            _subscriptions: subscriptions,
        };
        this.entity_changed(window, cx);
        this
    }

    pub fn new_search_in_directory(
        workspace: &mut Workspace,
        dir_path: &Path,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(filter_str) = dir_path.to_str() else {
            return;
        };

        let weak_workspace = cx.entity().downgrade();

        let entity = cx.new(|cx| ProjectSearch::new(workspace.project().clone(), cx));
        let search = cx.new(|cx| ProjectSearchView::new(weak_workspace, entity, window, cx, None));
        workspace.add_item_to_active_pane(Box::new(search.clone()), None, true, window, cx);
        search.update(cx, |search, cx| {
            search
                .included_files_editor
                .update(cx, |editor, cx| editor.set_text(filter_str, window, cx));
            search.filters_enabled = true;
            search.focus_query_editor(window, cx)
        });
    }

    /// Re-activate the most recently activated search in this pane or the most recent if it has been closed.
    /// If no search exists in the workspace, create a new one.
    pub fn deploy_search(
        workspace: &mut Workspace,
        action: &workspace::DeploySearch,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let existing = workspace
            .active_pane()
            .read(cx)
            .items()
            .find_map(|item| item.downcast::<ProjectSearchView>());

        Self::existing_or_new_search(workspace, existing, action, window, cx);
    }

    fn search_in_new(
        workspace: &mut Workspace,
        _: &SearchInNew,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(search_view) = workspace
            .active_item(cx)
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            let new_query = search_view.update(cx, |search_view, cx| {
                let new_query = search_view.build_search_query(cx);
                if new_query.is_some()
                    && let Some(old_query) = search_view.entity.read(cx).active_query.clone()
                {
                    search_view.query_editor.update(cx, |editor, cx| {
                        editor.set_text(old_query.as_str(), window, cx);
                    });
                    search_view.search_options = SearchOptions::from_query(&old_query);
                    search_view.adjust_query_regex_language(cx);
                }
                new_query
            });
            if let Some(new_query) = new_query {
                let entity = cx.new(|cx| {
                    let mut entity = ProjectSearch::new(workspace.project().clone(), cx);
                    entity.search(new_query, cx);
                    entity
                });
                let weak_workspace = cx.entity().downgrade();
                workspace.add_item_to_active_pane(
                    Box::new(cx.new(|cx| {
                        ProjectSearchView::new(weak_workspace, entity, window, cx, None)
                    })),
                    None,
                    true,
                    window,
                    cx,
                );
            }
        }
    }

    // Add another search tab to the workspace.
    fn new_search(
        workspace: &mut Workspace,
        _: &workspace::NewSearch,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::existing_or_new_search(workspace, None, &DeploySearch::find(), window, cx)
    }

    fn existing_or_new_search(
        workspace: &mut Workspace,
        existing: Option<Entity<ProjectSearchView>>,
        action: &workspace::DeploySearch,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let query = workspace.active_item(cx).and_then(|item| {
            if let Some(buffer_search_query) = buffer_search_query(workspace, item.as_ref(), cx) {
                return Some(buffer_search_query);
            }

            let editor = item.act_as::<Editor>(cx)?;
            let query = editor.query_suggestion(window, cx);
            if query.is_empty() { None } else { Some(query) }
        });

        let search = if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let settings = cx
                .global::<ActiveSettings>()
                .0
                .get(&workspace.project().downgrade());

            let settings = settings.cloned();

            let weak_workspace = cx.entity().downgrade();

            let project_search = cx.new(|cx| ProjectSearch::new(workspace.project().clone(), cx));
            let project_search_view = cx.new(|cx| {
                ProjectSearchView::new(weak_workspace, project_search, window, cx, settings)
            });

            workspace.add_item_to_active_pane(
                Box::new(project_search_view.clone()),
                None,
                true,
                window,
                cx,
            );
            project_search_view
        };

        search.update(cx, |search, cx| {
            search.replace_enabled = action.replace_enabled;
            if let Some(query) = query {
                search.set_query(&query, window, cx);
            }
            if let Some(included_files) = action.included_files.as_deref() {
                search
                    .included_files_editor
                    .update(cx, |editor, cx| editor.set_text(included_files, window, cx));
                search.filters_enabled = true;
            }
            if let Some(excluded_files) = action.excluded_files.as_deref() {
                search
                    .excluded_files_editor
                    .update(cx, |editor, cx| editor.set_text(excluded_files, window, cx));
                search.filters_enabled = true;
            }
            search.focus_query_editor(window, cx)
        });
    }

    fn prompt_to_save_if_dirty_then_search(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        use workspace::AutosaveSetting;

        let project = self.entity.read(cx).project.clone();

        let can_autosave = self.results_editor.can_autosave(cx);
        let autosave_setting = self.results_editor.workspace_settings(cx).autosave;

        let will_autosave = can_autosave
            && matches!(
                autosave_setting,
                AutosaveSetting::OnFocusChange | AutosaveSetting::OnWindowChange
            );

        let is_dirty = self.is_dirty(cx);

        cx.spawn_in(window, async move |this, cx| {
            let skip_save_on_close = this
                .read_with(cx, |this, cx| {
                    this.workspace.read_with(cx, |workspace, cx| {
                        workspace::Pane::skip_save_on_close(&this.results_editor, workspace, cx)
                    })
                })?
                .unwrap_or(false);

            let should_prompt_to_save = !skip_save_on_close && !will_autosave && is_dirty;

            let should_search = if should_prompt_to_save {
                let options = &["Save", "Don't Save", "Cancel"];
                let result_channel = this.update_in(cx, |_, window, cx| {
                    window.prompt(
                        gpui::PromptLevel::Warning,
                        "Project search buffer contains unsaved edits. Do you want to save it?",
                        None,
                        options,
                        cx,
                    )
                })?;
                let result = result_channel.await?;
                let should_save = result == 0;
                if should_save {
                    this.update_in(cx, |this, window, cx| {
                        this.save(
                            SaveOptions {
                                format: true,
                                autosave: false,
                            },
                            project,
                            window,
                            cx,
                        )
                    })?
                    .await
                    .log_err();
                }

                result != 2
            } else {
                true
            };
            if should_search {
                this.update(cx, |this, cx| {
                    this.search(cx);
                })?;
            }
            anyhow::Ok(())
        })
    }

    fn search(&mut self, cx: &mut Context<Self>) {
        if let Some(query) = self.build_search_query(cx) {
            self.entity.update(cx, |model, cx| model.search(query, cx));
        }
    }

    pub fn search_query_text(&self, cx: &App) -> String {
        self.query_editor.read(cx).text(cx)
    }

    fn build_search_query(&mut self, cx: &mut Context<Self>) -> Option<SearchQuery> {
        // Do not bail early in this function, as we want to fill out `self.panels_with_errors`.
        let text = self.query_editor.read(cx).text(cx);
        let open_buffers = if self.included_opened_only {
            Some(self.open_buffers(cx))
        } else {
            None
        };
        let included_files = self
            .filters_enabled
            .then(|| {
                match Self::parse_path_matches(&self.included_files_editor.read(cx).text(cx)) {
                    Ok(included_files) => {
                        let should_unmark_error =
                            self.panels_with_errors.remove(&InputPanel::Include);
                        if should_unmark_error.is_some() {
                            cx.notify();
                        }
                        included_files
                    }
                    Err(e) => {
                        let should_mark_error = self
                            .panels_with_errors
                            .insert(InputPanel::Include, e.to_string());
                        if should_mark_error.is_none() {
                            cx.notify();
                        }
                        PathMatcher::default()
                    }
                }
            })
            .unwrap_or_default();
        let excluded_files = self
            .filters_enabled
            .then(|| {
                match Self::parse_path_matches(&self.excluded_files_editor.read(cx).text(cx)) {
                    Ok(excluded_files) => {
                        let should_unmark_error =
                            self.panels_with_errors.remove(&InputPanel::Exclude);
                        if should_unmark_error.is_some() {
                            cx.notify();
                        }

                        excluded_files
                    }
                    Err(e) => {
                        let should_mark_error = self
                            .panels_with_errors
                            .insert(InputPanel::Exclude, e.to_string());
                        if should_mark_error.is_none() {
                            cx.notify();
                        }
                        PathMatcher::default()
                    }
                }
            })
            .unwrap_or_default();

        // If the project contains multiple visible worktrees, we match the
        // include/exclude patterns against full paths to allow them to be
        // disambiguated. For single worktree projects we use worktree relative
        // paths for convenience.
        let match_full_paths = self
            .entity
            .read(cx)
            .project
            .read(cx)
            .visible_worktrees(cx)
            .count()
            > 1;

        let query = if self.search_options.contains(SearchOptions::REGEX) {
            match SearchQuery::regex(
                text,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                self.search_options
                    .contains(SearchOptions::ONE_MATCH_PER_LINE),
                included_files,
                excluded_files,
                match_full_paths,
                open_buffers,
            ) {
                Ok(query) => {
                    let should_unmark_error = self.panels_with_errors.remove(&InputPanel::Query);
                    if should_unmark_error.is_some() {
                        cx.notify();
                    }

                    Some(query)
                }
                Err(e) => {
                    let should_mark_error = self
                        .panels_with_errors
                        .insert(InputPanel::Query, e.to_string());
                    if should_mark_error.is_none() {
                        cx.notify();
                    }

                    None
                }
            }
        } else {
            match SearchQuery::text(
                text,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                included_files,
                excluded_files,
                match_full_paths,
                open_buffers,
            ) {
                Ok(query) => {
                    let should_unmark_error = self.panels_with_errors.remove(&InputPanel::Query);
                    if should_unmark_error.is_some() {
                        cx.notify();
                    }

                    Some(query)
                }
                Err(e) => {
                    let should_mark_error = self
                        .panels_with_errors
                        .insert(InputPanel::Query, e.to_string());
                    if should_mark_error.is_none() {
                        cx.notify();
                    }

                    None
                }
            }
        };
        if !self.panels_with_errors.is_empty() {
            return None;
        }
        if query.as_ref().is_some_and(|query| query.is_empty()) {
            return None;
        }
        query
    }

    fn open_buffers(&self, cx: &mut Context<Self>) -> Vec<Entity<Buffer>> {
        let mut buffers = Vec::new();
        self.workspace
            .update(cx, |workspace, cx| {
                for editor in workspace.items_of_type::<Editor>(cx) {
                    if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                        buffers.push(buffer);
                    }
                }
            })
            .ok();
        buffers
    }

    fn parse_path_matches(text: &str) -> anyhow::Result<PathMatcher> {
        let queries = text
            .split(',')
            .map(str::trim)
            .filter(|maybe_glob_str| !maybe_glob_str.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        Ok(PathMatcher::new(&queries)?)
    }

    fn select_match(&mut self, direction: Direction, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.active_match_index {
            let match_ranges = self.entity.read(cx).match_ranges.clone();

            if !EditorSettings::get_global(cx).search_wrap
                && ((direction == Direction::Next && index + 1 >= match_ranges.len())
                    || (direction == Direction::Prev && index == 0))
            {
                crate::show_no_more_matches(window, cx);
                return;
            }

            let new_index = self.results_editor.update(cx, |editor, cx| {
                editor.match_index_for_direction(&match_ranges, index, direction, 1, window, cx)
            });

            let range_to_select = match_ranges[new_index].clone();
            self.results_editor.update(cx, |editor, cx| {
                let range_to_select = editor.range_for_match(&range_to_select);
                editor.unfold_ranges(std::slice::from_ref(&range_to_select), false, true, cx);
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges([range_to_select])
                });
            });
        }
    }

    fn focus_query_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.select_all(&SelectAll, window, cx);
        });
        let editor_handle = self.query_editor.focus_handle(cx);
        window.focus(&editor_handle);
    }

    fn set_query(&mut self, query: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.set_search_editor(SearchInputKind::Query, query, window, cx);
        if EditorSettings::get_global(cx).use_smartcase_search
            && !query.is_empty()
            && self.search_options.contains(SearchOptions::CASE_SENSITIVE)
                != contains_uppercase(query)
        {
            self.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx)
        }
    }

    fn set_search_editor(
        &mut self,
        kind: SearchInputKind,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = match kind {
            SearchInputKind::Query => &self.query_editor,
            SearchInputKind::Include => &self.included_files_editor,

            SearchInputKind::Exclude => &self.excluded_files_editor,
        };
        editor.update(cx, |included_editor, cx| {
            included_editor.set_text(text, window, cx)
        });
    }

    fn focus_results_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            let cursor = query_editor.selections.newest_anchor().head();
            query_editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([cursor..cursor])
            });
        });
        let results_handle = self.results_editor.focus_handle(cx);
        window.focus(&results_handle);
    }

    fn entity_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let match_ranges = self.entity.read(cx).match_ranges.clone();
        if match_ranges.is_empty() {
            self.active_match_index = None;
        } else {
            self.active_match_index = Some(0);
            self.update_match_index(cx);
            let prev_search_id = mem::replace(&mut self.search_id, self.entity.read(cx).search_id);
            let is_new_search = self.search_id != prev_search_id;
            self.results_editor.update(cx, |editor, cx| {
                if is_new_search {
                    let range_to_select = match_ranges
                        .first()
                        .map(|range| editor.range_for_match(range));
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_ranges(range_to_select)
                    });
                    editor.scroll(Point::default(), Some(Axis::Vertical), window, cx);
                }
                editor.highlight_background::<Self>(
                    &match_ranges,
                    |theme| theme.colors().search_match_background,
                    cx,
                );
            });
            if is_new_search && self.query_editor.focus_handle(cx).is_focused(window) {
                self.focus_results_editor(window, cx);
            }
        }

        cx.emit(ViewEvent::UpdateTab);
        cx.notify();
    }

    fn update_match_index(&mut self, cx: &mut Context<Self>) {
        let results_editor = self.results_editor.read(cx);
        let new_index = active_match_index(
            Direction::Next,
            &self.entity.read(cx).match_ranges,
            &results_editor.selections.newest_anchor().head(),
            &results_editor.buffer().read(cx).snapshot(cx),
        );
        if self.active_match_index != new_index {
            self.active_match_index = new_index;
            cx.notify();
        }
    }

    pub fn has_matches(&self) -> bool {
        self.active_match_index.is_some()
    }

    fn landing_text_minor(&self, window: &mut Window, cx: &App) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        v_flex()
            .gap_1()
            .child(
                Label::new("Hit enter to search. For more options:")
                    .color(Color::Muted)
                    .mb_2(),
            )
            .child(
                Button::new("filter-paths", "Include/exclude specific paths")
                    .icon(IconName::Filter)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleFilters,
                        &focus_handle,
                        window,
                        cx,
                    ))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleFilters.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("find-replace", "Find and replace")
                    .icon(IconName::Replace)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleReplace,
                        &focus_handle,
                        window,
                        cx,
                    ))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleReplace.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("regex", "Match with regex")
                    .icon(IconName::Regex)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleRegex,
                        &focus_handle,
                        window,
                        cx,
                    ))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleRegex.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("match-case", "Match case")
                    .icon(IconName::CaseSensitive)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleCaseSensitive,
                        &focus_handle,
                        window,
                        cx,
                    ))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleCaseSensitive.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("match-whole-words", "Match whole words")
                    .icon(IconName::WholeWord)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleWholeWord,
                        &focus_handle,
                        window,
                        cx,
                    ))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleWholeWord.boxed_clone(), cx)
                    }),
            )
    }

    fn border_color_for(&self, panel: InputPanel, cx: &App) -> Hsla {
        if self.panels_with_errors.contains_key(&panel) {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        }
    }

    fn move_focus_to_results(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.results_editor.focus_handle(cx).is_focused(window)
            && !self.entity.read(cx).match_ranges.is_empty()
        {
            cx.stop_propagation();
            self.focus_results_editor(window, cx)
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn results_editor(&self) -> &Entity<Editor> {
        &self.results_editor
    }

    fn adjust_query_regex_language(&self, cx: &mut App) {
        let enable = self.search_options.contains(SearchOptions::REGEX);
        let query_buffer = self
            .query_editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("query editor should be backed by a singleton buffer");
        if enable {
            if let Some(regex_language) = self.regex_language.clone() {
                query_buffer.update(cx, |query_buffer, cx| {
                    query_buffer.set_language(Some(regex_language), cx);
                })
            }
        } else {
            query_buffer.update(cx, |query_buffer, cx| {
                query_buffer.set_language(None, cx);
            })
        }
    }
}

fn buffer_search_query(
    workspace: &mut Workspace,
    item: &dyn ItemHandle,
    cx: &mut Context<Workspace>,
) -> Option<String> {
    let buffer_search_bar = workspace
        .pane_for(item)
        .and_then(|pane| {
            pane.read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<BufferSearchBar>()
        })?
        .read(cx);
    if buffer_search_bar.query_editor_focused() {
        let buffer_search_query = buffer_search_bar.query(cx);
        if !buffer_search_query.is_empty() {
            return Some(buffer_search_query);
        }
    }
    None
}

impl Default for ProjectSearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectSearchBar {
    pub fn new() -> Self {
        Self {
            active_project_search: None,
            subscription: None,
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if !search_view
                    .replacement_editor
                    .focus_handle(cx)
                    .is_focused(window)
                {
                    cx.stop_propagation();
                    search_view
                        .prompt_to_save_if_dirty_then_search(window, cx)
                        .detach_and_log_err(cx);
                }
            });
        }
    }

    fn tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_field(Direction::Next, window, cx);
    }

    fn backtab(&mut self, _: &Backtab, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_field(Direction::Prev, window, cx);
    }

    fn focus_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.query_editor.focus_handle(cx).focus(window);
            });
        }
    }

    fn cycle_field(&mut self, direction: Direction, window: &mut Window, cx: &mut Context<Self>) {
        let active_project_search = match &self.active_project_search {
            Some(active_project_search) => active_project_search,
            None => return,
        };

        active_project_search.update(cx, |project_view, cx| {
            let mut views = vec![project_view.query_editor.focus_handle(cx)];
            if project_view.replace_enabled {
                views.push(project_view.replacement_editor.focus_handle(cx));
            }
            if project_view.filters_enabled {
                views.extend([
                    project_view.included_files_editor.focus_handle(cx),
                    project_view.excluded_files_editor.focus_handle(cx),
                ]);
            }
            let current_index = match views.iter().position(|focus| focus.is_focused(window)) {
                Some(index) => index,
                None => return,
            };

            let new_index = match direction {
                Direction::Next => (current_index + 1) % views.len(),
                Direction::Prev if current_index == 0 => views.len() - 1,
                Direction::Prev => (current_index - 1) % views.len(),
            };
            let next_focus_handle = &views[new_index];
            window.focus(next_focus_handle);
            cx.stop_propagation();
        });
    }

    pub(crate) fn toggle_search_option(
        &mut self,
        option: SearchOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.active_project_search.is_none() {
            return false;
        }

        cx.spawn_in(window, async move |this, cx| {
            let task = this.update_in(cx, |this, window, cx| {
                let search_view = this.active_project_search.as_ref()?;
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_search_option(option, cx);
                    search_view
                        .entity
                        .read(cx)
                        .active_query
                        .is_some()
                        .then(|| search_view.prompt_to_save_if_dirty_then_search(window, cx))
                })
            })?;
            if let Some(task) = task {
                task.await?;
            }
            this.update(cx, |_, cx| {
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach();
        true
    }

    fn toggle_replace(&mut self, _: &ToggleReplace, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search) = &self.active_project_search {
            search.update(cx, |this, cx| {
                this.replace_enabled = !this.replace_enabled;
                let editor_to_focus = if this.replace_enabled {
                    this.replacement_editor.focus_handle(cx)
                } else {
                    this.query_editor.focus_handle(cx)
                };
                window.focus(&editor_to_focus);
                cx.notify();
            });
        }
    }

    fn toggle_filters(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.toggle_filters(cx);
                search_view
                    .included_files_editor
                    .update(cx, |_, cx| cx.notify());
                search_view
                    .excluded_files_editor
                    .update(cx, |_, cx| cx.notify());
                window.refresh();
                cx.notify();
            });
            cx.notify();
            true
        } else {
            false
        }
    }

    fn toggle_opened_only(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.active_project_search.is_none() {
            return false;
        }

        cx.spawn_in(window, async move |this, cx| {
            let task = this.update_in(cx, |this, window, cx| {
                let search_view = this.active_project_search.as_ref()?;
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_opened_only(window, cx);
                    search_view
                        .entity
                        .read(cx)
                        .active_query
                        .is_some()
                        .then(|| search_view.prompt_to_save_if_dirty_then_search(window, cx))
                })
            })?;
            if let Some(task) = task {
                task.await?;
            }
            this.update(cx, |_, cx| {
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach();
        true
    }

    fn is_opened_only_enabled(&self, cx: &App) -> bool {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.read(cx).included_opened_only
        } else {
            false
        }
    }

    fn move_focus_to_results(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.move_focus_to_results(window, cx);
            });
            cx.notify();
        }
    }

    fn next_history_query(
        &mut self,
        _: &NextHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                for (editor, kind) in [
                    (search_view.query_editor.clone(), SearchInputKind::Query),
                    (
                        search_view.included_files_editor.clone(),
                        SearchInputKind::Include,
                    ),
                    (
                        search_view.excluded_files_editor.clone(),
                        SearchInputKind::Exclude,
                    ),
                ] {
                    if editor.focus_handle(cx).is_focused(window) {
                        let new_query = search_view.entity.update(cx, |model, cx| {
                            let project = model.project.clone();

                            if let Some(new_query) = project.update(cx, |project, _| {
                                project
                                    .search_history_mut(kind)
                                    .next(model.cursor_mut(kind))
                                    .map(str::to_string)
                            }) {
                                new_query
                            } else {
                                model.cursor_mut(kind).reset();
                                String::new()
                            }
                        });
                        search_view.set_search_editor(kind, &new_query, window, cx);
                    }
                }
            });
        }
    }

    fn previous_history_query(
        &mut self,
        _: &PreviousHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                for (editor, kind) in [
                    (search_view.query_editor.clone(), SearchInputKind::Query),
                    (
                        search_view.included_files_editor.clone(),
                        SearchInputKind::Include,
                    ),
                    (
                        search_view.excluded_files_editor.clone(),
                        SearchInputKind::Exclude,
                    ),
                ] {
                    if editor.focus_handle(cx).is_focused(window) {
                        if editor.read(cx).text(cx).is_empty()
                            && let Some(new_query) = search_view
                                .entity
                                .read(cx)
                                .project
                                .read(cx)
                                .search_history(kind)
                                .current(search_view.entity.read(cx).cursor(kind))
                                .map(str::to_string)
                        {
                            search_view.set_search_editor(kind, &new_query, window, cx);
                            return;
                        }

                        if let Some(new_query) = search_view.entity.update(cx, |model, cx| {
                            let project = model.project.clone();
                            project.update(cx, |project, _| {
                                project
                                    .search_history_mut(kind)
                                    .previous(model.cursor_mut(kind))
                                    .map(str::to_string)
                            })
                        }) {
                            search_view.set_search_editor(kind, &new_query, window, cx);
                        }
                    }
                }
            });
        }
    }

    fn select_next_match(
        &mut self,
        _: &SelectNextMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search) = self.active_project_search.as_ref() {
            search.update(cx, |this, cx| {
                this.select_match(Direction::Next, window, cx);
            })
        }
    }

    fn select_prev_match(
        &mut self,
        _: &SelectPreviousMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search) = self.active_project_search.as_ref() {
            search.update(cx, |this, cx| {
                this.select_match(Direction::Prev, window, cx);
            })
        }
    }
}

impl Render for ProjectSearchBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(search) = self.active_project_search.clone() else {
            return div();
        };
        let search = search.read(cx);
        let focus_handle = search.focus_handle(cx);

        let container_width = window.viewport_size().width;
        let input_width = SearchInputWidth::calc_width(container_width);

        let input_base_styles = |panel: InputPanel| {
            input_base_styles(search.border_color_for(panel, cx), |div| match panel {
                InputPanel::Query | InputPanel::Replacement => div.w(input_width),
                InputPanel::Include | InputPanel::Exclude => div.flex_grow(),
            })
        };
        let theme_colors = cx.theme().colors();
        let project_search = search.entity.read(cx);
        let limit_reached = project_search.limit_reached;

        let color_override = match (
            &project_search.pending_search,
            project_search.no_results,
            &project_search.active_query,
            &project_search.last_search_query_text,
        ) {
            (None, Some(true), Some(q), Some(p)) if q.as_str() == p => Some(Color::Error),
            _ => None,
        };

        let match_text = search
            .active_match_index
            .and_then(|index| {
                let index = index + 1;
                let match_quantity = project_search.match_ranges.len();
                if match_quantity > 0 {
                    debug_assert!(match_quantity >= index);
                    if limit_reached {
                        Some(format!("{index}/{match_quantity}+"))
                    } else {
                        Some(format!("{index}/{match_quantity}"))
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "0/0".to_string());

        let query_column = input_base_styles(InputPanel::Query)
            .on_action(cx.listener(|this, action, window, cx| this.confirm(action, window, cx)))
            .on_action(cx.listener(|this, action, window, cx| {
                this.previous_history_query(action, window, cx)
            }))
            .on_action(
                cx.listener(|this, action, window, cx| this.next_history_query(action, window, cx)),
            )
            .child(render_text_input(&search.query_editor, color_override, cx))
            .child(
                h_flex()
                    .gap_1()
                    .child(SearchOption::CaseSensitive.as_button(
                        search.search_options,
                        SearchSource::Project(cx),
                        focus_handle.clone(),
                    ))
                    .child(SearchOption::WholeWord.as_button(
                        search.search_options,
                        SearchSource::Project(cx),
                        focus_handle.clone(),
                    ))
                    .child(SearchOption::Regex.as_button(
                        search.search_options,
                        SearchSource::Project(cx),
                        focus_handle.clone(),
                    )),
            );

        let query_focus = search.query_editor.focus_handle(cx);

        let matches_column = h_flex()
            .pl_2()
            .ml_2()
            .border_l_1()
            .border_color(theme_colors.border_variant)
            .child(render_action_button(
                "project-search-nav-button",
                IconName::ChevronLeft,
                search
                    .active_match_index
                    .is_none()
                    .then_some(ActionButtonState::Disabled),
                "Select Previous Match",
                &SelectPreviousMatch,
                query_focus.clone(),
            ))
            .child(render_action_button(
                "project-search-nav-button",
                IconName::ChevronRight,
                search
                    .active_match_index
                    .is_none()
                    .then_some(ActionButtonState::Disabled),
                "Select Next Match",
                &SelectNextMatch,
                query_focus,
            ))
            .child(
                div()
                    .id("matches")
                    .ml_2()
                    .min_w(rems_from_px(40.))
                    .child(Label::new(match_text).size(LabelSize::Small).color(
                        if search.active_match_index.is_some() {
                            Color::Default
                        } else {
                            Color::Disabled
                        },
                    ))
                    .when(limit_reached, |el| {
                        el.tooltip(Tooltip::text(
                            "Search limits reached.\nTry narrowing your search.",
                        ))
                    }),
            );

        let mode_column = h_flex()
            .gap_1()
            .min_w_64()
            .child(
                IconButton::new("project-search-filter-button", IconName::Filter)
                    .shape(IconButtonShape::Square)
                    .tooltip(|window, cx| {
                        Tooltip::for_action("Toggle Filters", &ToggleFilters, window, cx)
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_filters(window, cx);
                    }))
                    .toggle_state(
                        self.active_project_search
                            .as_ref()
                            .map(|search| search.read(cx).filters_enabled)
                            .unwrap_or_default(),
                    )
                    .tooltip({
                        let focus_handle = focus_handle.clone();
                        move |window, cx| {
                            Tooltip::for_action_in(
                                "Toggle Filters",
                                &ToggleFilters,
                                &focus_handle,
                                window,
                                cx,
                            )
                        }
                    }),
            )
            .child(render_action_button(
                "project-search",
                IconName::Replace,
                self.active_project_search
                    .as_ref()
                    .map(|search| search.read(cx).replace_enabled)
                    .and_then(|enabled| enabled.then_some(ActionButtonState::Toggled)),
                "Toggle Replace",
                &ToggleReplace,
                focus_handle.clone(),
            ))
            .child(matches_column);

        let search_line = h_flex()
            .w_full()
            .gap_2()
            .child(query_column)
            .child(mode_column);

        let replace_line = search.replace_enabled.then(|| {
            let replace_column = input_base_styles(InputPanel::Replacement)
                .child(render_text_input(&search.replacement_editor, None, cx));

            let focus_handle = search.replacement_editor.read(cx).focus_handle(cx);

            let replace_actions = h_flex()
                .min_w_64()
                .gap_1()
                .child(render_action_button(
                    "project-search-replace-button",
                    IconName::ReplaceNext,
                    Default::default(),
                    "Replace Next Match",
                    &ReplaceNext,
                    focus_handle.clone(),
                ))
                .child(render_action_button(
                    "project-search-replace-button",
                    IconName::ReplaceAll,
                    Default::default(),
                    "Replace All Matches",
                    &ReplaceAll,
                    focus_handle,
                ));

            h_flex()
                .w_full()
                .gap_2()
                .child(replace_column)
                .child(replace_actions)
        });

        let filter_line = search.filters_enabled.then(|| {
            let include = input_base_styles(InputPanel::Include)
                .on_action(cx.listener(|this, action, window, cx| {
                    this.previous_history_query(action, window, cx)
                }))
                .on_action(cx.listener(|this, action, window, cx| {
                    this.next_history_query(action, window, cx)
                }))
                .child(render_text_input(&search.included_files_editor, None, cx));
            let exclude = input_base_styles(InputPanel::Exclude)
                .on_action(cx.listener(|this, action, window, cx| {
                    this.previous_history_query(action, window, cx)
                }))
                .on_action(cx.listener(|this, action, window, cx| {
                    this.next_history_query(action, window, cx)
                }))
                .child(render_text_input(&search.excluded_files_editor, None, cx));
            let mode_column = h_flex()
                .gap_1()
                .min_w_64()
                .child(
                    IconButton::new("project-search-opened-only", IconName::FolderSearch)
                        .shape(IconButtonShape::Square)
                        .toggle_state(self.is_opened_only_enabled(cx))
                        .tooltip(Tooltip::text("Only Search Open Files"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_opened_only(window, cx);
                        })),
                )
                .child(SearchOption::IncludeIgnored.as_button(
                    search.search_options,
                    SearchSource::Project(cx),
                    focus_handle.clone(),
                ));
            h_flex()
                .w_full()
                .gap_2()
                .child(
                    h_flex()
                        .gap_2()
                        .w(input_width)
                        .child(include)
                        .child(exclude),
                )
                .child(mode_column)
        });

        let mut key_context = KeyContext::default();
        key_context.add("ProjectSearchBar");
        if search
            .replacement_editor
            .focus_handle(cx)
            .is_focused(window)
        {
            key_context.add("in_replace");
        }

        let query_error_line = search
            .panels_with_errors
            .get(&InputPanel::Query)
            .map(|error| {
                Label::new(error)
                    .size(LabelSize::Small)
                    .color(Color::Error)
                    .mt_neg_1()
                    .ml_2()
            });

        let filter_error_line = search
            .panels_with_errors
            .get(&InputPanel::Include)
            .or_else(|| search.panels_with_errors.get(&InputPanel::Exclude))
            .map(|error| {
                Label::new(error)
                    .size(LabelSize::Small)
                    .color(Color::Error)
                    .mt_neg_1()
                    .ml_2()
            });

        v_flex()
            .gap_2()
            .py(px(1.0))
            .w_full()
            .key_context(key_context)
            .on_action(cx.listener(|this, _: &ToggleFocus, window, cx| {
                this.move_focus_to_results(window, cx)
            }))
            .on_action(cx.listener(|this, _: &ToggleFilters, window, cx| {
                this.toggle_filters(window, cx);
            }))
            .capture_action(cx.listener(Self::tab))
            .capture_action(cx.listener(Self::backtab))
            .on_action(cx.listener(|this, action, window, cx| this.confirm(action, window, cx)))
            .on_action(cx.listener(|this, action, window, cx| {
                this.toggle_replace(action, window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleWholeWord, window, cx| {
                this.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleCaseSensitive, window, cx| {
                this.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
            }))
            .on_action(cx.listener(|this, action, window, cx| {
                if let Some(search) = this.active_project_search.as_ref() {
                    search.update(cx, |this, cx| {
                        this.replace_next(action, window, cx);
                    })
                }
            }))
            .on_action(cx.listener(|this, action, window, cx| {
                if let Some(search) = this.active_project_search.as_ref() {
                    search.update(cx, |this, cx| {
                        this.replace_all(action, window, cx);
                    })
                }
            }))
            .when(search.filters_enabled, |this| {
                this.on_action(cx.listener(|this, _: &ToggleIncludeIgnored, window, cx| {
                    this.toggle_search_option(SearchOptions::INCLUDE_IGNORED, window, cx);
                }))
            })
            .on_action(cx.listener(Self::select_next_match))
            .on_action(cx.listener(Self::select_prev_match))
            .child(search_line)
            .children(query_error_line)
            .children(replace_line)
            .children(filter_line)
            .children(filter_error_line)
    }
}

impl EventEmitter<ToolbarItemEvent> for ProjectSearchBar {}

impl ToolbarItemView for ProjectSearchBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.subscription = None;
        self.active_project_search = None;
        if let Some(search) = active_pane_item.and_then(|i| i.downcast::<ProjectSearchView>()) {
            self.subscription = Some(cx.observe(&search, |_, _, cx| cx.notify()));
            self.active_project_search = Some(search);
            ToolbarItemLocation::PrimaryLeft {}
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

fn register_workspace_action<A: Action>(
    workspace: &mut Workspace,
    callback: fn(&mut ProjectSearchBar, &A, &mut Window, &mut Context<ProjectSearchBar>),
) {
    workspace.register_action(move |workspace, action: &A, window, cx| {
        if workspace.has_active_modal(window, cx) {
            cx.propagate();
            return;
        }

        workspace.active_pane().update(cx, |pane, cx| {
            pane.toolbar().update(cx, move |workspace, cx| {
                if let Some(search_bar) = workspace.item_of_type::<ProjectSearchBar>() {
                    search_bar.update(cx, move |search_bar, cx| {
                        if search_bar.active_project_search.is_some() {
                            callback(search_bar, action, window, cx);
                            cx.notify();
                        } else {
                            cx.propagate();
                        }
                    });
                }
            });
        })
    });
}

fn register_workspace_action_for_present_search<A: Action>(
    workspace: &mut Workspace,
    callback: fn(&mut Workspace, &A, &mut Window, &mut Context<Workspace>),
) {
    workspace.register_action(move |workspace, action: &A, window, cx| {
        if workspace.has_active_modal(window, cx) {
            cx.propagate();
            return;
        }

        let should_notify = workspace
            .active_pane()
            .read(cx)
            .toolbar()
            .read(cx)
            .item_of_type::<ProjectSearchBar>()
            .map(|search_bar| search_bar.read(cx).active_project_search.is_some())
            .unwrap_or(false);
        if should_notify {
            callback(workspace, action, window, cx);
            cx.notify();
        } else {
            cx.propagate();
        }
    });
}

#[cfg(any(test, feature = "test-support"))]
pub fn perform_project_search(
    search_view: &Entity<ProjectSearchView>,
    text: impl Into<std::sync::Arc<str>>,
    cx: &mut gpui::VisualTestContext,
) {
    cx.run_until_parked();
    search_view.update_in(cx, |search_view, window, cx| {
        search_view.query_editor.update(cx, |query_editor, cx| {
            query_editor.set_text(text, window, cx)
        });
        search_view.search(cx);
    });
    cx.run_until_parked();
}

#[cfg(test)]
pub mod tests {
    use std::{ops::Deref as _, sync::Arc};

    use super::*;
    use editor::{DisplayPoint, display_map::DisplayRow};
    use gpui::{Action, TestAppContext, VisualTestContext, WindowHandle};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::DeploySearch;

    #[gpui::test]
    async fn test_project_search(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "TWO", cx);
        search_view.update(cx, |search_view, window, cx| {
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;"
            );
            let match_background_color = cx.theme().colors().search_match_background;
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.all_text_background_highlights(window, cx)),
                &[
                    (
                        DisplayPoint::new(DisplayRow(2), 32)..DisplayPoint::new(DisplayRow(2), 35),
                        match_background_color
                    ),
                    (
                        DisplayPoint::new(DisplayRow(2), 37)..DisplayPoint::new(DisplayRow(2), 40),
                        match_background_color
                    ),
                    (
                        DisplayPoint::new(DisplayRow(5), 6)..DisplayPoint::new(DisplayRow(5), 9),
                        match_background_color
                    )
                ]
            );
            assert_eq!(search_view.active_match_index, Some(0));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(2), 32)..DisplayPoint::new(DisplayRow(2), 35)]
            );

            search_view.select_match(Direction::Next, window, cx);
        }).unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                assert_eq!(search_view.active_match_index, Some(1));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(2), 37)..DisplayPoint::new(DisplayRow(2), 40)]
                );
                search_view.select_match(Direction::Next, window, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                assert_eq!(search_view.active_match_index, Some(2));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(5), 6)..DisplayPoint::new(DisplayRow(5), 9)]
                );
                search_view.select_match(Direction::Next, window, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                assert_eq!(search_view.active_match_index, Some(0));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(2), 32)..DisplayPoint::new(DisplayRow(2), 35)]
                );
                search_view.select_match(Direction::Prev, window, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                assert_eq!(search_view.active_match_index, Some(2));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(5), 6)..DisplayPoint::new(DisplayRow(5), 9)]
                );
                search_view.select_match(Direction::Prev, window, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, _, cx| {
                assert_eq!(search_view.active_match_index, Some(1));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(2), 37)..DisplayPoint::new(DisplayRow(2), 40)]
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_deploy_project_search_focus(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window;
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        let active_item = cx.read(|cx| {
            workspace
                .read(cx)
                .unwrap()
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        });
        assert!(
            active_item.is_none(),
            "Expected no search panel to be active"
        );

        window
            .update(cx, move |workspace, window, cx| {
                assert_eq!(workspace.panes().len(), 1);
                workspace.panes()[0].update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });

                ProjectSearchView::deploy_search(
                    workspace,
                    &workspace::DeploySearch::find(),
                    window,
                    cx,
                )
            })
            .unwrap();

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .unwrap()
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Empty search view should be focused after the toggle focus event: no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let query_editor = &search_view.query_editor;
                    assert!(
                        query_editor.focus_handle(cx).is_focused(window),
                        "Search view should be focused after the new search view is activated",
                    );
                    let query_text = query_editor.read(cx).text(cx);
                    assert!(
                        query_text.is_empty(),
                        "New search query should be empty but got '{query_text}'",
                    );
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Empty search view should have no results but got '{results_text}'"
                    );
                });
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXIST", window, cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Search view for mismatching query should have no results but got '{results_text}'"
                    );
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Search view should be focused after mismatching query had been used in search",
                    );
                });
            }).unwrap();

        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, window, cx| {
                window.dispatch_action(ToggleFocus.boxed_clone(), cx)
            })
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(window),
                    "Search view with mismatching query should be focused after the toggle focus event: still no results panel to focus on",
                );
            });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                    "Search view results should match the query"
                );
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with mismatching query should be focused after search results are available",
                );
            });
        }).unwrap();
        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with matching query should still have its results editor focused after the toggle focus event",
                );
            });
        }).unwrap();

        workspace
            .update(cx, |workspace, window, cx| {
                ProjectSearchView::deploy_search(
                    workspace,
                    &workspace::DeploySearch::find(),
                    window,
                    cx,
                )
            })
            .unwrap();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert_eq!(search_view.query_editor.read(cx).text(cx), "two", "Query should be updated to first search result after search view 2nd open in a row");
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                    "Results should be unchanged after search view 2nd open in a row"
                );
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(window),
                    "Focus should be moved into query editor again after search view 2nd open in a row"
                );
            });
        }).unwrap();

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with matching query should switch focus to the results editor after the toggle focus event",
                );
            });
        }).unwrap();
    }

    #[gpui::test]
    async fn test_filters_consider_toggle_state(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window;
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        window
            .update(cx, move |workspace, window, cx| {
                workspace.panes()[0].update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });

                ProjectSearchView::deploy_search(
                    workspace,
                    &workspace::DeploySearch::find(),
                    window,
                    cx,
                )
            })
            .unwrap();

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .unwrap()
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("const FOUR", window, cx)
                    });
                    search_view.toggle_filters(cx);
                    search_view
                        .excluded_files_editor
                        .update(cx, |exclude_editor, cx| {
                            exclude_editor.set_text("four.rs", window, cx)
                        });
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Search view for query with the only match in an excluded file should have no results but got '{results_text}'"
                    );
                });
            }).unwrap();

        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, window, cx| {
                window.dispatch_action(ToggleFocus.boxed_clone(), cx)
            })
        })
        .detach();
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_filters(cx);
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst FOUR: usize = one::ONE + three::THREE;",
                    "Search view results should contain the queried result in the previously excluded file with filters toggled off"
                );
            });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_new_project_search_focus(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window;
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        let active_item = cx.read(|cx| {
            workspace
                .read(cx)
                .unwrap()
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        });
        assert!(
            active_item.is_none(),
            "Expected no search panel to be active"
        );

        window
            .update(cx, move |workspace, window, cx| {
                assert_eq!(workspace.panes().len(), 1);
                workspace.panes()[0].update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });

                ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
            })
            .unwrap();

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .unwrap()
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();

        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Empty search view should be focused after the toggle focus event: no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let query_editor = &search_view.query_editor;
                    assert!(
                        query_editor.focus_handle(cx).is_focused(window),
                        "Search view should be focused after the new search view is activated",
                    );
                    let query_text = query_editor.read(cx).text(cx);
                    assert!(
                        query_text.is_empty(),
                        "New search query should be empty but got '{query_text}'",
                    );
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Empty search view should have no results but got '{results_text}'"
                    );
                });
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXIST", window, cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                results_text.is_empty(),
                "Search view for mismatching query should have no results but got '{results_text}'"
            );
                    assert!(
                search_view.query_editor.focus_handle(cx).is_focused(window),
                "Search view should be focused after mismatching query had been used in search",
            );
                });
            })
            .unwrap();
        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, window, cx| {
                window.dispatch_action(ToggleFocus.boxed_clone(), cx)
            })
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Search view with mismatching query should be focused after the toggle focus event: still no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                    search_view.search(cx);
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx|
        search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                    "Search view results should match the query"
                );
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with mismatching query should be focused after search results are available",
                );
            })).unwrap();
        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.results_editor.focus_handle(cx).is_focused(window),
                        "Search view with matching query should still have its results editor focused after the toggle focus event",
                    );
                });
        }).unwrap();

        workspace
            .update(cx, |workspace, window, cx| {
                ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        let Some(search_view_2) = cx.read(|cx| {
            workspace
                .read(cx)
                .unwrap()
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };
        assert!(
            search_view_2 != search_view,
            "New search view should be open after `workspace::NewSearch` event"
        );

        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO", "First search view should not have an updated query");
                    assert_eq!(
                        search_view
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                        "Results of the first search view should not update too"
                    );
                    assert!(
                        !search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Focus should be moved away from the first search view"
                    );
                });
        }).unwrap();

        window.update(cx, |_, window, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert_eq!(
                        search_view_2.query_editor.read(cx).text(cx),
                        "two",
                        "New search view should get the query from the text cursor was at during the event spawn (first search view's first result)"
                    );
                    assert_eq!(
                        search_view_2
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "",
                        "No search results should be in the 2nd view yet, as we did not spawn a search for it"
                    );
                    assert!(
                        search_view_2.query_editor.focus_handle(cx).is_focused(window),
                        "Focus should be moved into query editor of the new window"
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view_2.update(cx, |search_view_2, cx| {
                    search_view_2.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("FOUR", window, cx)
                    });
                    search_view_2.search(cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert_eq!(
                        search_view_2
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "\n\nconst FOUR: usize = one::ONE + three::THREE;",
                        "New search view with the updated query should have new search results"
                    );
                    assert!(
                        search_view_2.results_editor.focus_handle(cx).is_focused(window),
                        "Search view with mismatching query should be focused after search results are available",
                    );
                });
        }).unwrap();

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert!(
                        search_view_2.results_editor.focus_handle(cx).is_focused(window),
                        "Search view with matching query should switch focus to the results editor after the toggle focus event",
                    );
                });}).unwrap();
    }

    #[gpui::test]
    async fn test_new_project_search_in_directory(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a": {
                    "one.rs": "const ONE: usize = 1;",
                    "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                },
                "b": {
                    "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                    "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
                },
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).unwrap();
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        let active_item = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        });
        assert!(
            active_item.is_none(),
            "Expected no search panel to be active"
        );

        window
            .update(cx, move |workspace, window, cx| {
                assert_eq!(workspace.panes().len(), 1);
                workspace.panes()[0].update(cx, move |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });
            })
            .unwrap();

        let a_dir_entry = cx.update(|cx| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .entry_for_path(&(worktree_id, "a").into(), cx)
                .expect("no entry for /a/ directory")
        });
        assert!(a_dir_entry.is_dir());
        window
            .update(cx, |workspace, window, cx| {
                ProjectSearchView::new_search_in_directory(workspace, &a_dir_entry.path, window, cx)
            })
            .unwrap();

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search in directory event trigger")
        };
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "On new search in directory, focus should be moved into query editor"
                    );
                    search_view.excluded_files_editor.update(cx, |editor, cx| {
                        assert!(
                            editor.display_text(cx).is_empty(),
                            "New search in directory should not have any excluded files"
                        );
                    });
                    search_view.included_files_editor.update(cx, |editor, cx| {
                        assert_eq!(
                            editor.display_text(cx),
                            a_dir_entry.path.to_str().unwrap(),
                            "New search in directory should have included dir entry path"
                        );
                    });
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("const", window, cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\nconst ONE: usize = 1;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                "New search in directory should have a filter that matches a certain directory"
            );
                })
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_search_query_history(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).unwrap();
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        window
            .update(cx, {
                let search_bar = search_bar.clone();
                |workspace, window, cx| {
                    assert_eq!(workspace.panes().len(), 1);
                    workspace.panes()[0].update(cx, |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
                }
            })
            .unwrap();

        let search_view = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
                .expect("Search view expected to appear after new search event trigger")
        });

        // Add 3 search items into the history + another unsubmitted one.
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.search_options = SearchOptions::CASE_SENSITIVE;
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("THREE", window, cx)
                    });
                    search_view.search(cx);
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("JUST_TEXT_INPUT", window, cx)
                    });
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();

        // Ensure that the latest input with search settings is active.
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(
                        search_view.query_editor.read(cx).text(cx),
                        "JUST_TEXT_INPUT"
                    );
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Next history query after the latest should set the query to the empty string.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                })
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                })
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // First previous query for empty current query should set the query to the latest submitted one.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Further previous items should go over the history in reverse order.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Previous items should never go behind the first history item.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Next items should go over the history in the original order.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO_NEW", window, cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // New search input should add another entry to history and move the selection to the end of the history.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_search_query_history_with_multiple_views(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees(cx).next().unwrap().read(cx).id()
        });

        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let workspace = window.root(cx).unwrap();

        let panes: Vec<_> = window
            .update(cx, |this, _, _| this.panes().to_owned())
            .unwrap();

        let search_bar_1 = window.build_entity(cx, |_, _| ProjectSearchBar::new());
        let search_bar_2 = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        assert_eq!(panes.len(), 1);
        let first_pane = panes.first().cloned().unwrap();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 0);
        window
            .update(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, "one.rs"),
                    Some(first_pane.downgrade()),
                    true,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 1);

        // Add a project search item to the first pane
        window
            .update(cx, {
                let search_bar = search_bar_1.clone();
                |workspace, window, cx| {
                    first_pane.update(cx, |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
                }
            })
            .unwrap();
        let search_view_1 = cx.read(|cx| {
            workspace
                .read(cx)
                .active_item(cx)
                .and_then(|item| item.downcast::<ProjectSearchView>())
                .expect("Search view expected to appear after new search event trigger")
        });

        let second_pane = window
            .update(cx, |workspace, window, cx| {
                workspace.split_and_clone(
                    first_pane.clone(),
                    workspace::SplitDirection::Right,
                    window,
                    cx,
                )
            })
            .unwrap()
            .unwrap();
        assert_eq!(cx.update(|cx| second_pane.read(cx).items_len()), 1);

        assert_eq!(cx.update(|cx| second_pane.read(cx).items_len()), 1);
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 2);

        // Add a project search item to the second pane
        window
            .update(cx, {
                let search_bar = search_bar_2.clone();
                let pane = second_pane.clone();
                move |workspace, window, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    pane.update(cx, |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
                }
            })
            .unwrap();

        let search_view_2 = cx.read(|cx| {
            workspace
                .read(cx)
                .active_item(cx)
                .and_then(|item| item.downcast::<ProjectSearchView>())
                .expect("Search view expected to appear after new search event trigger")
        });

        cx.run_until_parked();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 2);
        assert_eq!(cx.update(|cx| second_pane.read(cx).items_len()), 2);

        let update_search_view =
            |search_view: &Entity<ProjectSearchView>, query: &str, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, cx| {
                        search_view.update(cx, |search_view, cx| {
                            search_view.query_editor.update(cx, |query_editor, cx| {
                                query_editor.set_text(query, window, cx)
                            });
                            search_view.search(cx);
                        });
                    })
                    .unwrap();
            };

        let active_query =
            |search_view: &Entity<ProjectSearchView>, cx: &mut TestAppContext| -> String {
                window
                    .update(cx, |_, _, cx| {
                        search_view.update(cx, |search_view, cx| {
                            search_view.query_editor.read(cx).text(cx)
                        })
                    })
                    .unwrap()
            };

        let select_prev_history_item =
            |search_bar: &Entity<ProjectSearchBar>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, cx| {
                        search_bar.update(cx, |search_bar, cx| {
                            search_bar.focus_search(window, cx);
                            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                        })
                    })
                    .unwrap();
            };

        let select_next_history_item =
            |search_bar: &Entity<ProjectSearchBar>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, cx| {
                        search_bar.update(cx, |search_bar, cx| {
                            search_bar.focus_search(window, cx);
                            search_bar.next_history_query(&NextHistoryQuery, window, cx);
                        })
                    })
                    .unwrap();
            };

        update_search_view(&search_view_1, "ONE", cx);
        cx.background_executor.run_until_parked();

        update_search_view(&search_view_2, "TWO", cx);
        cx.background_executor.run_until_parked();

        assert_eq!(active_query(&search_view_1, cx), "ONE");
        assert_eq!(active_query(&search_view_2, cx), "TWO");

        // Selecting previous history item should select the query from search view 1.
        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        // Selecting the previous history item should not change the query as it is already the first item.
        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        // Changing the query in search view 2 should not affect the history of search view 1.
        assert_eq!(active_query(&search_view_1, cx), "ONE");

        // Deploying a new search in search view 2
        update_search_view(&search_view_2, "THREE", cx);
        cx.background_executor.run_until_parked();

        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "THREE");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "TWO");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        // Search view 1 should now see the query from search view 2.
        assert_eq!(active_query(&search_view_1, cx), "ONE");

        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "TWO");

        // Here is the new query from search view 2
        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "THREE");

        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "");

        select_next_history_item(&search_bar_1, cx);
        assert_eq!(active_query(&search_view_1, cx), "TWO");

        select_next_history_item(&search_bar_1, cx);
        assert_eq!(active_query(&search_view_1, cx), "THREE");

        select_next_history_item(&search_bar_1, cx);
        assert_eq!(active_query(&search_view_1, cx), "");
    }

    #[gpui::test]
    async fn test_deploy_search_with_multiple_panes(cx: &mut TestAppContext) {
        init_test(cx);

        // Setup 2 panes, both with a file open and one with a project search.
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees(cx).next().unwrap().read(cx).id()
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
        let panes: Vec<_> = window
            .update(cx, |this, _, _| this.panes().to_owned())
            .unwrap();
        assert_eq!(panes.len(), 1);
        let first_pane = panes.first().cloned().unwrap();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 0);
        window
            .update(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, "one.rs"),
                    Some(first_pane.downgrade()),
                    true,
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 1);
        let second_pane = window
            .update(cx, |workspace, window, cx| {
                workspace.split_and_clone(
                    first_pane.clone(),
                    workspace::SplitDirection::Right,
                    window,
                    cx,
                )
            })
            .unwrap()
            .unwrap();
        assert_eq!(cx.update(|cx| second_pane.read(cx).items_len()), 1);
        assert!(
            window
                .update(cx, |_, window, cx| second_pane
                    .focus_handle(cx)
                    .contains_focused(window, cx))
                .unwrap()
        );
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());
        window
            .update(cx, {
                let search_bar = search_bar.clone();
                let pane = first_pane.clone();
                move |workspace, window, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    pane.update(cx, move |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                    });
                }
            })
            .unwrap();

        // Add a project search item to the second pane
        window
            .update(cx, {
                |workspace, window, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    second_pane.update(cx, |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
                }
            })
            .unwrap();

        cx.run_until_parked();
        assert_eq!(cx.update(|cx| second_pane.read(cx).items_len()), 2);
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 1);

        // Focus the first pane
        window
            .update(cx, |workspace, window, cx| {
                assert_eq!(workspace.active_pane(), &second_pane);
                second_pane.update(cx, |this, cx| {
                    assert_eq!(this.active_item_index(), 1);
                    this.activate_prev_item(false, window, cx);
                    assert_eq!(this.active_item_index(), 0);
                });
                workspace.activate_pane_in_direction(workspace::SplitDirection::Left, window, cx);
            })
            .unwrap();
        window
            .update(cx, |workspace, _, cx| {
                assert_eq!(workspace.active_pane(), &first_pane);
                assert_eq!(first_pane.read(cx).items_len(), 1);
                assert_eq!(second_pane.read(cx).items_len(), 2);
            })
            .unwrap();

        // Deploy a new search
        cx.dispatch_action(window.into(), DeploySearch::find());

        // Both panes should now have a project search in them
        window
            .update(cx, |workspace, window, cx| {
                assert_eq!(workspace.active_pane(), &first_pane);
                first_pane.read_with(cx, |this, _| {
                    assert_eq!(this.active_item_index(), 1);
                    assert_eq!(this.items_len(), 2);
                });
                second_pane.update(cx, |this, cx| {
                    assert!(!cx.focus_handle().contains_focused(window, cx));
                    assert_eq!(this.items_len(), 2);
                });
            })
            .unwrap();

        // Focus the second pane's non-search item
        window
            .update(cx, |_workspace, window, cx| {
                second_pane.update(cx, |pane, cx| pane.activate_next_item(true, window, cx));
            })
            .unwrap();

        // Deploy a new search
        cx.dispatch_action(window.into(), DeploySearch::find());

        // The project search view should now be focused in the second pane
        // And the number of items should be unchanged.
        window
            .update(cx, |_workspace, _, cx| {
                second_pane.update(cx, |pane, _cx| {
                    assert!(
                        pane.active_item()
                            .unwrap()
                            .downcast::<ProjectSearchView>()
                            .is_some()
                    );

                    assert_eq!(pane.items_len(), 2);
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_scroll_search_results_to_top(cx: &mut TestAppContext) {
        init_test(cx);

        // We need many lines in the search results to be able to scroll the window
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "1.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "2.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "3.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "4.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "5.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "6.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "7.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "8.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "9.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "a.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "b.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "c.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "d.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "e.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "f.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "g.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "h.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "i.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "j.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "k.rs": "\n\n\n\n\n B \n\n\n\n\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        // First search
        perform_search(search_view, "A", cx);
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    // Results are correct and scrolled to the top
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(" A ").count(),
                        10
                    );
                    assert_eq!(results_editor.scroll_position(cx), Point::default());

                    // Scroll results all the way down
                    results_editor.scroll(
                        Point::new(0., f32::MAX),
                        Some(Axis::Vertical),
                        window,
                        cx,
                    );
                });
            })
            .expect("unable to update search view");

        // Second search
        perform_search(search_view, "B", cx);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    // Results are correct...
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(" B ").count(),
                        10
                    );
                    // ...and scrolled back to the top
                    assert_eq!(results_editor.scroll_position(cx), Point::default());
                });
            })
            .expect("unable to update search view");
    }

    #[gpui::test]
    async fn test_buffer_search_query_reused(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees(cx).next().unwrap().read(cx).id()
        });
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();
        let mut cx = VisualTestContext::from_window(*window.deref(), cx);

        let editor = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, "one.rs"), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        // Wait for the unstaged changes to be loaded
        cx.run_until_parked();

        let buffer_search_bar = cx.new_window_entity(|window, cx| {
            let mut search_bar =
                BufferSearchBar::new(Some(project.read(cx).languages().clone()), window, cx);
            search_bar.set_active_pane_item(Some(&editor), window, cx);
            search_bar.show(window, cx);
            search_bar
        });

        let panes: Vec<_> = window
            .update(&mut cx, |this, _, _| this.panes().to_owned())
            .unwrap();
        assert_eq!(panes.len(), 1);
        let pane = panes.first().cloned().unwrap();
        pane.update_in(&mut cx, |pane, window, cx| {
            pane.toolbar().update(cx, |toolbar, cx| {
                toolbar.add_item(buffer_search_bar.clone(), window, cx);
            })
        });

        let buffer_search_query = "search bar query";
        buffer_search_bar
            .update_in(&mut cx, |buffer_search_bar, window, cx| {
                buffer_search_bar.focus_handle(cx).focus(window);
                buffer_search_bar.search(buffer_search_query, None, window, cx)
            })
            .await
            .unwrap();

        workspace.update_in(&mut cx, |workspace, window, cx| {
            ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
        });
        cx.run_until_parked();
        let project_search_view = pane
            .read_with(&cx, |pane, _| {
                pane.active_item()
                    .and_then(|item| item.downcast::<ProjectSearchView>())
            })
            .expect("should open a project search view after spawning a new search");
        project_search_view.update(&mut cx, |search_view, cx| {
            assert_eq!(
                search_view.search_query_text(cx),
                buffer_search_query,
                "Project search should take the query from the buffer search bar since it got focused and had a query inside"
            );
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme::init(theme::LoadThemes::JustBase, cx);

            language::init(cx);
            client::init_settings(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            crate::init(cx);
        });
    }

    fn perform_search(
        search_view: WindowHandle<ProjectSearchView>,
        text: impl Into<Arc<str>>,
        cx: &mut TestAppContext,
    ) {
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text(text, window, cx)
                });
                search_view.search(cx);
            })
            .unwrap();
        cx.background_executor.run_until_parked();
    }
}
