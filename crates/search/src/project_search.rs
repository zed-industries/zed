use crate::{
    FocusSearch, NextHistoryQuery, PreviousHistoryQuery, ReplaceAll, ReplaceNext, SearchOptions,
    SelectNextMatch, SelectPrevMatch, ToggleCaseSensitive, ToggleIncludeIgnored, ToggleRegex,
    ToggleReplace, ToggleWholeWord,
};
use anyhow::Context as _;
use collections::{HashMap, HashSet};
use editor::{
    actions::SelectAll,
    items::active_match_index,
    scroll::{Autoscroll, Axis},
    Anchor, Editor, EditorElement, EditorEvent, EditorStyle, MultiBuffer, MAX_TAB_TITLE_LEN,
};
use gpui::{
    actions, div, Action, AnyElement, AnyView, AppContext, Context as _, Element, EntityId,
    EventEmitter, FocusHandle, FocusableView, FontStyle, Global, Hsla, InteractiveElement,
    IntoElement, Model, ModelContext, ParentElement, Point, Render, SharedString, Styled,
    Subscription, Task, TextStyle, UpdateGlobal, View, ViewContext, VisualContext, WeakModel,
    WeakView, WhiteSpace, WindowContext,
};
use menu::Confirm;
use project::{search::SearchQuery, search_history::SearchHistoryCursor, Project, ProjectPath};
use settings::Settings;
use smol::stream::StreamExt;
use std::{
    any::{Any, TypeId},
    mem,
    ops::{Not, Range},
    path::Path,
};
use theme::ThemeSettings;
use ui::{
    h_flex, prelude::*, v_flex, Icon, IconButton, IconName, Label, LabelCommon, LabelSize,
    Selectable, Tooltip,
};
use util::paths::PathMatcher;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle, TabContentParams},
    searchable::{Direction, SearchableItem, SearchableItemHandle},
    DeploySearch, ItemNavHistory, NewSearch, Pane, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace, WorkspaceId,
};

const MIN_INPUT_WIDTH_REMS: f32 = 15.;
const MAX_INPUT_WIDTH_REMS: f32 = 30.;

actions!(
    project_search,
    [SearchInNew, ToggleFocus, NextField, ToggleFilters]
);

#[derive(Default)]
struct ActiveSettings(HashMap<WeakModel<Project>, ProjectSearchSettings>);

impl Global for ActiveSettings {}

pub fn init(cx: &mut AppContext) {
    cx.set_global(ActiveSettings::default());
    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        register_workspace_action(workspace, move |search_bar, _: &FocusSearch, cx| {
            search_bar.focus_search(cx);
        });
        register_workspace_action(workspace, move |search_bar, _: &ToggleFilters, cx| {
            search_bar.toggle_filters(cx);
        });
        register_workspace_action(workspace, move |search_bar, _: &ToggleCaseSensitive, cx| {
            search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
        });
        register_workspace_action(workspace, move |search_bar, _: &ToggleWholeWord, cx| {
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
        });
        register_workspace_action(workspace, move |search_bar, _: &ToggleRegex, cx| {
            search_bar.toggle_search_option(SearchOptions::REGEX, cx);
        });
        register_workspace_action(workspace, move |search_bar, action: &ToggleReplace, cx| {
            search_bar.toggle_replace(action, cx)
        });
        register_workspace_action(
            workspace,
            move |search_bar, action: &SelectPrevMatch, cx| {
                search_bar.select_prev_match(action, cx)
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, action: &SelectNextMatch, cx| {
                search_bar.select_next_match(action, cx)
            },
        );

        // Only handle search_in_new if there is a search present
        register_workspace_action_for_present_search(workspace, |workspace, action, cx| {
            ProjectSearchView::search_in_new(workspace, action, cx)
        });

        // Both on present and dismissed search, we need to unconditionally handle those actions to focus from the editor.
        workspace.register_action(move |workspace, action: &DeploySearch, cx| {
            if workspace.has_active_modal(cx) {
                cx.propagate();
                return;
            }
            ProjectSearchView::deploy_search(workspace, action, cx);
            cx.notify();
        });
        workspace.register_action(move |workspace, action: &NewSearch, cx| {
            if workspace.has_active_modal(cx) {
                cx.propagate();
                return;
            }
            ProjectSearchView::new_search(workspace, action, cx);
            cx.notify();
        });
    })
    .detach();
}

struct ProjectSearch {
    project: Model<Project>,
    excerpts: Model<MultiBuffer>,
    pending_search: Option<Task<Option<()>>>,
    match_ranges: Vec<Range<Anchor>>,
    active_query: Option<SearchQuery>,
    last_search_query_text: Option<String>,
    search_id: usize,
    no_results: Option<bool>,
    limit_reached: bool,
    search_history_cursor: SearchHistoryCursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputPanel {
    Query,
    Exclude,
    Include,
}

pub struct ProjectSearchView {
    focus_handle: FocusHandle,
    model: Model<ProjectSearch>,
    query_editor: View<Editor>,
    replacement_editor: View<Editor>,
    results_editor: View<Editor>,
    search_options: SearchOptions,
    panels_with_errors: HashSet<InputPanel>,
    active_match_index: Option<usize>,
    search_id: usize,
    query_editor_was_focused: bool,
    included_files_editor: View<Editor>,
    excluded_files_editor: View<Editor>,
    filters_enabled: bool,
    replace_enabled: bool,
    _subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone)]
struct ProjectSearchSettings {
    search_options: SearchOptions,
    filters_enabled: bool,
}

pub struct ProjectSearchBar {
    active_project_search: Option<View<ProjectSearchView>>,
    subscription: Option<Subscription>,
}

impl ProjectSearch {
    fn new(project: Model<Project>, cx: &mut ModelContext<Self>) -> Self {
        let replica_id = project.read(cx).replica_id();
        let capability = project.read(cx).capability();

        Self {
            project,
            excerpts: cx.new_model(|_| MultiBuffer::new(replica_id, capability)),
            pending_search: Default::default(),
            match_ranges: Default::default(),
            active_query: None,
            last_search_query_text: None,
            search_id: 0,
            no_results: None,
            limit_reached: false,
            search_history_cursor: Default::default(),
        }
    }

    fn clone(&self, cx: &mut ModelContext<Self>) -> Model<Self> {
        cx.new_model(|cx| Self {
            project: self.project.clone(),
            excerpts: self
                .excerpts
                .update(cx, |excerpts, cx| cx.new_model(|cx| excerpts.clone(cx))),
            pending_search: Default::default(),
            match_ranges: self.match_ranges.clone(),
            active_query: self.active_query.clone(),
            last_search_query_text: self.last_search_query_text.clone(),
            search_id: self.search_id,
            no_results: self.no_results,
            limit_reached: self.limit_reached,
            search_history_cursor: self.search_history_cursor.clone(),
        })
    }

    fn search(&mut self, query: SearchQuery, cx: &mut ModelContext<Self>) {
        let search = self.project.update(cx, |project, cx| {
            project
                .search_history_mut()
                .add(&mut self.search_history_cursor, query.as_str().to_string());
            project.search(query.clone(), cx)
        });
        self.last_search_query_text = Some(query.as_str().to_string());
        self.search_id += 1;
        self.active_query = Some(query);
        self.match_ranges.clear();
        self.pending_search = Some(cx.spawn(|this, mut cx| async move {
            let mut matches = search;
            let this = this.upgrade()?;
            this.update(&mut cx, |this, cx| {
                this.match_ranges.clear();
                this.excerpts.update(cx, |this, cx| this.clear(cx));
                this.no_results = Some(true);
                this.limit_reached = false;
            })
            .ok()?;

            let mut limit_reached = false;
            while let Some(result) = matches.next().await {
                match result {
                    project::SearchResult::Buffer { buffer, ranges } => {
                        let mut match_ranges = this
                            .update(&mut cx, |this, cx| {
                                this.no_results = Some(false);
                                this.excerpts.update(cx, |excerpts, cx| {
                                    excerpts.stream_excerpts_with_context_lines(
                                        buffer,
                                        ranges,
                                        editor::DEFAULT_MULTIBUFFER_CONTEXT,
                                        cx,
                                    )
                                })
                            })
                            .ok()?;

                        while let Some(range) = match_ranges.next().await {
                            this.update(&mut cx, |this, _| this.match_ranges.push(range))
                                .ok()?;
                        }
                        this.update(&mut cx, |_, cx| cx.notify()).ok()?;
                    }
                    project::SearchResult::LimitReached => {
                        limit_reached = true;
                    }
                }
            }

            this.update(&mut cx, |this, cx| {
                this.limit_reached = limit_reached;
                this.pending_search.take();
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if self.has_matches() {
            div()
                .flex_1()
                .size_full()
                .track_focus(&self.focus_handle)
                .child(self.results_editor.clone())
        } else {
            let model = self.model.read(cx);
            let has_no_results = model.no_results.unwrap_or(false);
            let is_search_underway = model.pending_search.is_some();
            let major_text = if is_search_underway {
                Label::new("Searching...")
            } else if has_no_results {
                Label::new("No results")
            } else {
                Label::new("Search all files")
            };

            let major_text = div().justify_center().max_w_96().child(major_text);

            let minor_text: Option<SharedString> = if let Some(no_results) = model.no_results {
                if model.pending_search.is_none() && no_results {
                    Some("No results found in this project for the provided query".into())
                } else {
                    None
                }
            } else {
                Some(self.landing_text_minor())
            };
            let minor_text = minor_text.map(|text| {
                div()
                    .items_center()
                    .max_w_96()
                    .child(Label::new(text).size(LabelSize::Small))
            });
            v_flex()
                .flex_1()
                .size_full()
                .justify_center()
                .bg(cx.theme().colors().editor_background)
                .track_focus(&self.focus_handle)
                .child(
                    h_flex()
                        .size_full()
                        .justify_center()
                        .child(h_flex().flex_1())
                        .child(v_flex().child(major_text).children(minor_text))
                        .child(h_flex().flex_1()),
                )
        }
    }
}

impl FocusableView for ProjectSearchView {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ProjectSearchView {
    type Event = ViewEvent;
    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
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
        self_handle: &'a View<Self>,
        _: &'a AppContext,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.results_editor.clone().into())
        } else {
            None
        }
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.results_editor
            .update(cx, |editor, cx| editor.deactivated(cx));
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext<'_>) -> AnyElement {
        let last_query: Option<SharedString> = self
            .model
            .read(cx)
            .last_search_query_text
            .as_ref()
            .map(|query| {
                let query = query.replace('\n', "");
                let query_text = util::truncate_and_trailoff(&query, MAX_TAB_TITLE_LEN);
                query_text.into()
            });
        let tab_name = last_query
            .filter(|query| !query.is_empty())
            .unwrap_or_else(|| "Project Search".into());
        h_flex()
            .gap_2()
            .child(
                Icon::new(IconName::MagnifyingGlass).color(if params.selected {
                    Color::Default
                } else {
                    Color::Muted
                }),
            )
            .child(Label::new(tab_name).color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            }))
            .into_any()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("project search")
    }

    fn for_each_project_item(
        &self,
        cx: &AppContext,
        f: &mut dyn FnMut(EntityId, &dyn project::Item),
    ) {
        self.results_editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &AppContext) -> bool {
        false
    }

    fn can_save(&self, _: &AppContext) -> bool {
        true
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.results_editor.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.results_editor.read(cx).has_conflict(cx)
    }

    fn save(
        &mut self,
        format: bool,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.save(format, project, cx))
    }

    fn save_as(
        &mut self,
        _: Model<Project>,
        _: ProjectPath,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called")
    }

    fn reload(
        &mut self,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.reload(project, cx))
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
    where
        Self: Sized,
    {
        let model = self.model.update(cx, |model, cx| model.clone(cx));
        Some(cx.new_view(|cx| Self::new(model, cx, None)))
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        self.results_editor
            .update(cx, |editor, cx| editor.added_to_workspace(workspace, cx));
    }

    fn set_nav_history(&mut self, nav_history: ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.results_editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(&mut self, data: Box<dyn Any>, cx: &mut ViewContext<Self>) -> bool {
        self.results_editor
            .update(cx, |editor, cx| editor.navigate(data, cx))
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

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        if self.has_matches() {
            ToolbarItemLocation::Secondary
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        self.results_editor.breadcrumbs(theme, cx)
    }

    fn serialized_item_kind() -> Option<&'static str> {
        None
    }

    fn deserialize(
        _project: Model<Project>,
        _workspace: WeakView<Workspace>,
        _workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        _cx: &mut ViewContext<Pane>,
    ) -> Task<anyhow::Result<View<Self>>> {
        unimplemented!()
    }
}

impl ProjectSearchView {
    fn toggle_filters(&mut self, cx: &mut ViewContext<Self>) {
        self.filters_enabled = !self.filters_enabled;
        ActiveSettings::update_global(cx, |settings, cx| {
            settings.0.insert(
                self.model.read(cx).project.downgrade(),
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

    fn toggle_search_option(&mut self, option: SearchOptions, cx: &mut ViewContext<Self>) {
        self.search_options.toggle(option);
        ActiveSettings::update_global(cx, |settings, cx| {
            settings.0.insert(
                self.model.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });
    }

    fn replace_next(&mut self, _: &ReplaceNext, cx: &mut ViewContext<Self>) {
        if self.model.read(cx).match_ranges.is_empty() {
            return;
        }
        let Some(active_index) = self.active_match_index else {
            return;
        };

        let query = self.model.read(cx).active_query.clone();
        if let Some(query) = query {
            let query = query.with_replacement(self.replacement(cx));

            // TODO: Do we need the clone here?
            let mat = self.model.read(cx).match_ranges[active_index].clone();
            self.results_editor.update(cx, |editor, cx| {
                editor.replace(&mat, &query, cx);
            });
            self.select_match(Direction::Next, cx)
        }
    }
    pub fn replacement(&self, cx: &AppContext) -> String {
        self.replacement_editor.read(cx).text(cx)
    }
    fn replace_all(&mut self, _: &ReplaceAll, cx: &mut ViewContext<Self>) {
        if self.active_match_index.is_none() {
            return;
        }

        let Some(query) = self.model.read(cx).active_query.as_ref() else {
            return;
        };
        let query = query.clone().with_replacement(self.replacement(cx));

        let match_ranges = self
            .model
            .update(cx, |model, _| mem::take(&mut model.match_ranges));
        if match_ranges.is_empty() {
            return;
        }

        self.results_editor.update(cx, |editor, cx| {
            for item in &match_ranges {
                editor.replace(item, &query, cx);
            }
        });

        self.model.update(cx, |model, _cx| {
            model.match_ranges = match_ranges;
        });
    }

    fn new(
        model: Model<ProjectSearch>,
        cx: &mut ViewContext<Self>,
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
            (SearchOptions::NONE, false)
        };

        {
            let model = model.read(cx);
            project = model.project.clone();
            excerpts = model.excerpts.clone();
            if let Some(active_query) = model.active_query.as_ref() {
                query_text = active_query.as_str().to_string();
                replacement_text = active_query.replacement().map(ToOwned::to_owned);
                options = SearchOptions::from_query(active_query);
            }
        }
        subscriptions.push(cx.observe(&model, |this, _, cx| this.model_changed(cx)));

        let query_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Search all files..", cx);
            editor.set_text(query_text, cx);
            editor
        });
        // Subscribe to query_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(
            cx.subscribe(&query_editor, |_, _, event: &EditorEvent, cx| {
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            }),
        );
        let replacement_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Replace in project..", cx);
            if let Some(text) = replacement_text {
                editor.set_text(text, cx);
            }
            editor
        });
        let results_editor = cx.new_view(|cx| {
            let mut editor = Editor::for_multibuffer(excerpts, Some(project.clone()), true, cx);
            editor.set_searchable(false);
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

        let included_files_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_placeholder_text("Include: crates/**/*.toml", cx);

            editor
        });
        // Subscribe to include_files_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(
            cx.subscribe(&included_files_editor, |_, _, event: &EditorEvent, cx| {
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            }),
        );

        let excluded_files_editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
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
        subscriptions.push(cx.on_focus_in(&focus_handle, |this, cx| {
            if this.focus_handle.is_focused(cx) {
                if this.has_matches() {
                    this.results_editor.focus_handle(cx).focus(cx);
                } else {
                    this.query_editor.focus_handle(cx).focus(cx);
                }
            }
        }));

        // Check if Worktrees have all been previously indexed
        let mut this = ProjectSearchView {
            focus_handle,
            replacement_editor,
            search_id: model.read(cx).search_id,
            model,
            query_editor,
            results_editor,
            search_options: options,
            panels_with_errors: HashSet::default(),
            active_match_index: None,
            query_editor_was_focused: false,
            included_files_editor,
            excluded_files_editor,
            filters_enabled,
            replace_enabled: false,
            _subscriptions: subscriptions,
        };
        this.model_changed(cx);
        this
    }

    pub fn new_search_in_directory(
        workspace: &mut Workspace,
        dir_path: &Path,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(filter_str) = dir_path.to_str() else {
            return;
        };

        let model = cx.new_model(|cx| ProjectSearch::new(workspace.project().clone(), cx));
        let search = cx.new_view(|cx| ProjectSearchView::new(model, cx, None));
        workspace.add_item_to_active_pane(Box::new(search.clone()), None, cx);
        search.update(cx, |search, cx| {
            search
                .included_files_editor
                .update(cx, |editor, cx| editor.set_text(filter_str, cx));
            search.filters_enabled = true;
            search.focus_query_editor(cx)
        });
    }

    // Re-activate the most recently activated search in this pane or the most recent if it has been closed.
    // If no search exists in the workspace, create a new one.
    fn deploy_search(
        workspace: &mut Workspace,
        action: &workspace::DeploySearch,
        cx: &mut ViewContext<Workspace>,
    ) {
        let existing = workspace
            .active_pane()
            .read(cx)
            .items()
            .find_map(|item| item.downcast::<ProjectSearchView>());

        Self::existing_or_new_search(workspace, existing, action, cx);
    }

    fn search_in_new(workspace: &mut Workspace, _: &SearchInNew, cx: &mut ViewContext<Workspace>) {
        if let Some(search_view) = workspace
            .active_item(cx)
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            let new_query = search_view.update(cx, |search_view, cx| {
                let new_query = search_view.build_search_query(cx);
                if new_query.is_some() {
                    if let Some(old_query) = search_view.model.read(cx).active_query.clone() {
                        search_view.query_editor.update(cx, |editor, cx| {
                            editor.set_text(old_query.as_str(), cx);
                        });
                        search_view.search_options = SearchOptions::from_query(&old_query);
                    }
                }
                new_query
            });
            if let Some(new_query) = new_query {
                let model = cx.new_model(|cx| {
                    let mut model = ProjectSearch::new(workspace.project().clone(), cx);
                    model.search(new_query, cx);
                    model
                });
                workspace.add_item_to_active_pane(
                    Box::new(cx.new_view(|cx| ProjectSearchView::new(model, cx, None))),
                    None,
                    cx,
                );
            }
        }
    }

    // Add another search tab to the workspace.
    fn new_search(
        workspace: &mut Workspace,
        _: &workspace::NewSearch,
        cx: &mut ViewContext<Workspace>,
    ) {
        Self::existing_or_new_search(workspace, None, &DeploySearch::find(), cx)
    }

    fn existing_or_new_search(
        workspace: &mut Workspace,
        existing: Option<View<ProjectSearchView>>,
        action: &workspace::DeploySearch,
        cx: &mut ViewContext<Workspace>,
    ) {
        let query = workspace.active_item(cx).and_then(|item| {
            let editor = item.act_as::<Editor>(cx)?;
            let query = editor.query_suggestion(cx);
            if query.is_empty() {
                None
            } else {
                Some(query)
            }
        });

        let search = if let Some(existing) = existing {
            workspace.activate_item(&existing, cx);
            existing
        } else {
            let settings = cx
                .global::<ActiveSettings>()
                .0
                .get(&workspace.project().downgrade());

            let settings = if let Some(settings) = settings {
                Some(settings.clone())
            } else {
                None
            };

            let model = cx.new_model(|cx| ProjectSearch::new(workspace.project().clone(), cx));
            let view = cx.new_view(|cx| ProjectSearchView::new(model, cx, settings));

            workspace.add_item_to_active_pane(Box::new(view.clone()), None, cx);
            view
        };

        search.update(cx, |search, cx| {
            search.replace_enabled = action.replace_enabled;
            if let Some(query) = query {
                search.set_query(&query, cx);
            }
            search.focus_query_editor(cx)
        });
    }

    fn search(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(query) = self.build_search_query(cx) {
            self.model.update(cx, |model, cx| model.search(query, cx));
        }
    }

    fn build_search_query(&mut self, cx: &mut ViewContext<Self>) -> Option<SearchQuery> {
        // Do not bail early in this function, as we want to fill out `self.panels_with_errors`.
        let text = self.query_editor.read(cx).text(cx);
        let included_files =
            match Self::parse_path_matches(&self.included_files_editor.read(cx).text(cx)) {
                Ok(included_files) => {
                    let should_unmark_error = self.panels_with_errors.remove(&InputPanel::Include);
                    if should_unmark_error {
                        cx.notify();
                    }
                    included_files
                }
                Err(_e) => {
                    let should_mark_error = self.panels_with_errors.insert(InputPanel::Include);
                    if should_mark_error {
                        cx.notify();
                    }
                    vec![]
                }
            };
        let excluded_files =
            match Self::parse_path_matches(&self.excluded_files_editor.read(cx).text(cx)) {
                Ok(excluded_files) => {
                    let should_unmark_error = self.panels_with_errors.remove(&InputPanel::Exclude);
                    if should_unmark_error {
                        cx.notify();
                    }

                    excluded_files
                }
                Err(_e) => {
                    let should_mark_error = self.panels_with_errors.insert(InputPanel::Exclude);
                    if should_mark_error {
                        cx.notify();
                    }
                    vec![]
                }
            };

        let query = if self.search_options.contains(SearchOptions::REGEX) {
            match SearchQuery::regex(
                text,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                self.search_options.contains(SearchOptions::INCLUDE_IGNORED),
                included_files,
                excluded_files,
            ) {
                Ok(query) => {
                    let should_unmark_error = self.panels_with_errors.remove(&InputPanel::Query);
                    if should_unmark_error {
                        cx.notify();
                    }

                    Some(query)
                }
                Err(_e) => {
                    let should_mark_error = self.panels_with_errors.insert(InputPanel::Query);
                    if should_mark_error {
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
            ) {
                Ok(query) => {
                    let should_unmark_error = self.panels_with_errors.remove(&InputPanel::Query);
                    if should_unmark_error {
                        cx.notify();
                    }

                    Some(query)
                }
                Err(_e) => {
                    let should_mark_error = self.panels_with_errors.insert(InputPanel::Query);
                    if should_mark_error {
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

    fn parse_path_matches(text: &str) -> anyhow::Result<Vec<PathMatcher>> {
        text.split(',')
            .map(str::trim)
            .filter(|maybe_glob_str| !maybe_glob_str.is_empty())
            .map(|maybe_glob_str| {
                PathMatcher::new(maybe_glob_str)
                    .with_context(|| format!("parsing {maybe_glob_str} as path matcher"))
            })
            .collect()
    }

    fn select_match(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.active_match_index {
            let match_ranges = self.model.read(cx).match_ranges.clone();
            let new_index = self.results_editor.update(cx, |editor, cx| {
                editor.match_index_for_direction(&match_ranges, index, direction, 1, cx)
            });

            let range_to_select = match_ranges[new_index].clone();
            self.results_editor.update(cx, |editor, cx| {
                let range_to_select = editor.range_for_match(&range_to_select);
                editor.unfold_ranges([range_to_select.clone()], false, true, cx);
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_ranges([range_to_select])
                });
            });
        }
    }

    fn focus_query_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.select_all(&SelectAll, cx);
        });
        self.query_editor_was_focused = true;
        let editor_handle = self.query_editor.focus_handle(cx);
        cx.focus(&editor_handle);
    }

    fn set_query(&mut self, query: &str, cx: &mut ViewContext<Self>) {
        self.query_editor
            .update(cx, |query_editor, cx| query_editor.set_text(query, cx));
    }

    fn focus_results_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            let cursor = query_editor.selections.newest_anchor().head();
            query_editor.change_selections(None, cx, |s| s.select_ranges([cursor..cursor]));
        });
        self.query_editor_was_focused = false;
        let results_handle = self.results_editor.focus_handle(cx);
        cx.focus(&results_handle);
    }

    fn model_changed(&mut self, cx: &mut ViewContext<Self>) {
        let match_ranges = self.model.read(cx).match_ranges.clone();
        if match_ranges.is_empty() {
            self.active_match_index = None;
        } else {
            self.active_match_index = Some(0);
            self.update_match_index(cx);
            let prev_search_id = mem::replace(&mut self.search_id, self.model.read(cx).search_id);
            let is_new_search = self.search_id != prev_search_id;
            self.results_editor.update(cx, |editor, cx| {
                if is_new_search {
                    let range_to_select = match_ranges
                        .first()
                        .map(|range| editor.range_for_match(range));
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select_ranges(range_to_select)
                    });
                    editor.scroll(Point::default(), Some(Axis::Vertical), cx);
                }
                editor.highlight_background::<Self>(
                    &match_ranges,
                    |theme| theme.search_match_background,
                    cx,
                );
            });
            if is_new_search && self.query_editor.focus_handle(cx).is_focused(cx) {
                self.focus_results_editor(cx);
            }
        }

        cx.emit(ViewEvent::UpdateTab);
        cx.notify();
    }

    fn update_match_index(&mut self, cx: &mut ViewContext<Self>) {
        let results_editor = self.results_editor.read(cx);
        let new_index = active_match_index(
            &self.model.read(cx).match_ranges,
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

    fn landing_text_minor(&self) -> SharedString {
        "Include/exclude specific paths with the filter option. Matching exact word and/or casing is available too.".into()
    }

    fn border_color_for(&self, panel: InputPanel, cx: &WindowContext) -> Hsla {
        if self.panels_with_errors.contains(&panel) {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        }
    }

    fn move_focus_to_results(&mut self, cx: &mut ViewContext<Self>) {
        if !self.results_editor.focus_handle(cx).is_focused(cx)
            && !self.model.read(cx).match_ranges.is_empty()
        {
            cx.stop_propagation();
            return self.focus_results_editor(cx);
        }
    }
}

impl ProjectSearchBar {
    pub fn new() -> Self {
        Self {
            active_project_search: None,
            subscription: None,
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if !search_view
                    .replacement_editor
                    .focus_handle(cx)
                    .is_focused(cx)
                {
                    cx.stop_propagation();
                    search_view.search(cx);
                }
            });
        }
    }

    fn tab(&mut self, _: &editor::actions::Tab, cx: &mut ViewContext<Self>) {
        self.cycle_field(Direction::Next, cx);
    }

    fn tab_previous(&mut self, _: &editor::actions::TabPrev, cx: &mut ViewContext<Self>) {
        self.cycle_field(Direction::Prev, cx);
    }

    fn focus_search(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.query_editor.focus_handle(cx).focus(cx);
            });
        }
    }

    fn cycle_field(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let active_project_search = match &self.active_project_search {
            Some(active_project_search) => active_project_search,

            None => {
                return;
            }
        };

        active_project_search.update(cx, |project_view, cx| {
            let mut views = vec![&project_view.query_editor];
            if project_view.replace_enabled {
                views.push(&project_view.replacement_editor);
            }
            if project_view.filters_enabled {
                views.extend([
                    &project_view.included_files_editor,
                    &project_view.excluded_files_editor,
                ]);
            }
            let current_index = match views
                .iter()
                .enumerate()
                .find(|(_, view)| view.focus_handle(cx).is_focused(cx))
            {
                Some((index, _)) => index,
                None => return,
            };

            let new_index = match direction {
                Direction::Next => (current_index + 1) % views.len(),
                Direction::Prev if current_index == 0 => views.len() - 1,
                Direction::Prev => (current_index - 1) % views.len(),
            };
            let next_focus_handle = views[new_index].focus_handle(cx);
            cx.focus(&next_focus_handle);
            cx.stop_propagation();
        });
    }

    fn toggle_search_option(&mut self, option: SearchOptions, cx: &mut ViewContext<Self>) -> bool {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.toggle_search_option(option, cx);
                search_view.search(cx);
            });

            cx.notify();
            true
        } else {
            false
        }
    }

    fn toggle_replace(&mut self, _: &ToggleReplace, cx: &mut ViewContext<Self>) {
        if let Some(search) = &self.active_project_search {
            search.update(cx, |this, cx| {
                this.replace_enabled = !this.replace_enabled;
                let editor_to_focus = if this.replace_enabled {
                    this.replacement_editor.focus_handle(cx)
                } else {
                    this.query_editor.focus_handle(cx)
                };
                cx.focus(&editor_to_focus);
                cx.notify();
            });
        }
    }

    fn toggle_filters(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.toggle_filters(cx);
                search_view
                    .included_files_editor
                    .update(cx, |_, cx| cx.notify());
                search_view
                    .excluded_files_editor
                    .update(cx, |_, cx| cx.notify());
                cx.refresh();
                cx.notify();
            });
            cx.notify();
            true
        } else {
            false
        }
    }

    fn move_focus_to_results(&self, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.move_focus_to_results(cx);
            });
            cx.notify();
        }
    }

    fn is_option_enabled(&self, option: SearchOptions, cx: &AppContext) -> bool {
        if let Some(search) = self.active_project_search.as_ref() {
            search.read(cx).search_options.contains(option)
        } else {
            false
        }
    }

    fn next_history_query(&mut self, _: &NextHistoryQuery, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                let new_query = search_view.model.update(cx, |model, cx| {
                    if let Some(new_query) = model.project.update(cx, |project, _| {
                        project
                            .search_history_mut()
                            .next(&mut model.search_history_cursor)
                            .map(str::to_string)
                    }) {
                        new_query
                    } else {
                        model.search_history_cursor.reset();
                        String::new()
                    }
                });
                search_view.set_query(&new_query, cx);
            });
        }
    }

    fn previous_history_query(&mut self, _: &PreviousHistoryQuery, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if search_view.query_editor.read(cx).text(cx).is_empty() {
                    if let Some(new_query) = search_view
                        .model
                        .read(cx)
                        .project
                        .read(cx)
                        .search_history()
                        .current(&search_view.model.read(cx).search_history_cursor)
                        .map(str::to_string)
                    {
                        search_view.set_query(&new_query, cx);
                        return;
                    }
                }

                if let Some(new_query) = search_view.model.update(cx, |model, cx| {
                    model.project.update(cx, |project, _| {
                        project
                            .search_history_mut()
                            .previous(&mut model.search_history_cursor)
                            .map(str::to_string)
                    })
                }) {
                    search_view.set_query(&new_query, cx);
                }
            });
        }
    }

    fn select_next_match(&mut self, _: &SelectNextMatch, cx: &mut ViewContext<Self>) {
        if let Some(search) = self.active_project_search.as_ref() {
            search.update(cx, |this, cx| {
                this.select_match(Direction::Next, cx);
            })
        }
    }

    fn select_prev_match(&mut self, _: &SelectPrevMatch, cx: &mut ViewContext<Self>) {
        if let Some(search) = self.active_project_search.as_ref() {
            search.update(cx, |this, cx| {
                this.select_match(Direction::Prev, cx);
            })
        }
    }

    fn render_text_input(&self, editor: &View<Editor>, cx: &ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.buffer_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };

        EditorElement::new(
            &editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

impl Render for ProjectSearchBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(search) = self.active_project_search.clone() else {
            return div();
        };
        let search = search.read(cx);

        let query_column = h_flex()
            .flex_1()
            .h_8()
            .mr_2()
            .px_2()
            .py_1()
            .border_1()
            .border_color(search.border_color_for(InputPanel::Query, cx))
            .rounded_lg()
            .min_w(rems(MIN_INPUT_WIDTH_REMS))
            .max_w(rems(MAX_INPUT_WIDTH_REMS))
            .on_action(cx.listener(|this, action, cx| this.confirm(action, cx)))
            .on_action(cx.listener(|this, action, cx| this.previous_history_query(action, cx)))
            .on_action(cx.listener(|this, action, cx| this.next_history_query(action, cx)))
            .child(self.render_text_input(&search.query_editor, cx))
            .child(
                h_flex()
                    .child(SearchOptions::CASE_SENSITIVE.as_button(
                        self.is_option_enabled(SearchOptions::CASE_SENSITIVE, cx),
                        cx.listener(|this, _, cx| {
                            this.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
                        }),
                    ))
                    .child(SearchOptions::WHOLE_WORD.as_button(
                        self.is_option_enabled(SearchOptions::WHOLE_WORD, cx),
                        cx.listener(|this, _, cx| {
                            this.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
                        }),
                    ))
                    .child(SearchOptions::REGEX.as_button(
                        self.is_option_enabled(SearchOptions::REGEX, cx),
                        cx.listener(|this, _, cx| {
                            this.toggle_search_option(SearchOptions::REGEX, cx);
                        }),
                    )),
            );

        let mode_column = v_flex().items_start().justify_start().child(
            h_flex()
                .child(
                    IconButton::new("project-search-filter-button", IconName::Filter)
                        .tooltip(|cx| Tooltip::for_action("Toggle filters", &ToggleFilters, cx))
                        .on_click(cx.listener(|this, _, cx| {
                            this.toggle_filters(cx);
                        }))
                        .selected(
                            self.active_project_search
                                .as_ref()
                                .map(|search| search.read(cx).filters_enabled)
                                .unwrap_or_default(),
                        ),
                )
                .child(
                    IconButton::new("project-search-toggle-replace", IconName::Replace)
                        .on_click(cx.listener(|this, _, cx| {
                            this.toggle_replace(&ToggleReplace, cx);
                        }))
                        .selected(
                            self.active_project_search
                                .as_ref()
                                .map(|search| search.read(cx).replace_enabled)
                                .unwrap_or_default(),
                        )
                        .tooltip(|cx| Tooltip::for_action("Toggle replace", &ToggleReplace, cx)),
                ),
        );

        let match_text = search
            .active_match_index
            .and_then(|index| {
                let index = index + 1;
                let match_quantity = search.model.read(cx).match_ranges.len();
                if match_quantity > 0 {
                    debug_assert!(match_quantity >= index);
                    Some(format!("{index}/{match_quantity}").to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "0/0".to_string());

        let limit_reached = search.model.read(cx).limit_reached;

        let matches_column = h_flex()
            .child(
                IconButton::new("project-search-prev-match", IconName::ChevronLeft)
                    .disabled(search.active_match_index.is_none())
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(search) = this.active_project_search.as_ref() {
                            search.update(cx, |this, cx| {
                                this.select_match(Direction::Prev, cx);
                            })
                        }
                    }))
                    .tooltip(|cx| {
                        Tooltip::for_action("Go to previous match", &SelectPrevMatch, cx)
                    }),
            )
            .child(
                IconButton::new("project-search-next-match", IconName::ChevronRight)
                    .disabled(search.active_match_index.is_none())
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(search) = this.active_project_search.as_ref() {
                            search.update(cx, |this, cx| {
                                this.select_match(Direction::Next, cx);
                            })
                        }
                    }))
                    .tooltip(|cx| Tooltip::for_action("Go to next match", &SelectNextMatch, cx)),
            )
            .child(
                h_flex()
                    .min_w(rems_from_px(40.))
                    .child(
                        Label::new(match_text).color(if search.active_match_index.is_some() {
                            Color::Default
                        } else {
                            Color::Disabled
                        }),
                    ),
            )
            .when(limit_reached, |this| {
                this.child(
                    div()
                        .child(Label::new("Search limit reached").color(Color::Warning))
                        .ml_2(),
                )
            });

        let search_line = h_flex()
            .flex_1()
            .child(query_column)
            .child(mode_column)
            .child(matches_column);

        let replace_line = search.replace_enabled.then(|| {
            let replace_column = h_flex()
                .flex_1()
                .min_w(rems(MIN_INPUT_WIDTH_REMS))
                .max_w(rems(MAX_INPUT_WIDTH_REMS))
                .h_8()
                .px_2()
                .py_1()
                .border_1()
                .border_color(cx.theme().colors().border)
                .rounded_lg()
                .child(self.render_text_input(&search.replacement_editor, cx));
            let replace_actions = h_flex().when(search.replace_enabled, |this| {
                this.child(
                    IconButton::new("project-search-replace-next", IconName::ReplaceNext)
                        .on_click(cx.listener(|this, _, cx| {
                            if let Some(search) = this.active_project_search.as_ref() {
                                search.update(cx, |this, cx| {
                                    this.replace_next(&ReplaceNext, cx);
                                })
                            }
                        }))
                        .tooltip(|cx| Tooltip::for_action("Replace next match", &ReplaceNext, cx)),
                )
                .child(
                    IconButton::new("project-search-replace-all", IconName::ReplaceAll)
                        .on_click(cx.listener(|this, _, cx| {
                            if let Some(search) = this.active_project_search.as_ref() {
                                search.update(cx, |this, cx| {
                                    this.replace_all(&ReplaceAll, cx);
                                })
                            }
                        }))
                        .tooltip(|cx| Tooltip::for_action("Replace all matches", &ReplaceAll, cx)),
                )
            });
            h_flex()
                .gap_2()
                .child(replace_column)
                .child(replace_actions)
        });

        let filter_line = search.filters_enabled.then(|| {
            h_flex()
                .w_full()
                .gap_2()
                .child(
                    h_flex()
                        .flex_1()
                        .min_w(rems(MIN_INPUT_WIDTH_REMS))
                        .max_w(rems(MAX_INPUT_WIDTH_REMS))
                        .h_8()
                        .px_2()
                        .py_1()
                        .border_1()
                        .border_color(search.border_color_for(InputPanel::Include, cx))
                        .rounded_lg()
                        .child(self.render_text_input(&search.included_files_editor, cx))
                        .child(
                            SearchOptions::INCLUDE_IGNORED.as_button(
                                search
                                    .search_options
                                    .contains(SearchOptions::INCLUDE_IGNORED),
                                cx.listener(|this, _, cx| {
                                    this.toggle_search_option(SearchOptions::INCLUDE_IGNORED, cx);
                                }),
                            ),
                        ),
                )
                .child(
                    h_flex()
                        .flex_1()
                        .min_w(rems(MIN_INPUT_WIDTH_REMS))
                        .max_w(rems(MAX_INPUT_WIDTH_REMS))
                        .h_8()
                        .px_2()
                        .py_1()
                        .border_1()
                        .border_color(search.border_color_for(InputPanel::Exclude, cx))
                        .rounded_lg()
                        .child(self.render_text_input(&search.excluded_files_editor, cx)),
                )
        });

        v_flex()
            .key_context("ProjectSearchBar")
            .on_action(cx.listener(|this, _: &ToggleFocus, cx| this.move_focus_to_results(cx)))
            .on_action(cx.listener(|this, _: &ToggleFilters, cx| {
                this.toggle_filters(cx);
            }))
            .capture_action(cx.listener(|this, action, cx| {
                this.tab(action, cx);
                cx.stop_propagation();
            }))
            .capture_action(cx.listener(|this, action, cx| {
                this.tab_previous(action, cx);
                cx.stop_propagation();
            }))
            .on_action(cx.listener(|this, action, cx| this.confirm(action, cx)))
            .on_action(cx.listener(|this, action, cx| {
                this.toggle_replace(action, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleWholeWord, cx| {
                this.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleCaseSensitive, cx| {
                this.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
            }))
            .on_action(cx.listener(|this, action, cx| {
                if let Some(search) = this.active_project_search.as_ref() {
                    search.update(cx, |this, cx| {
                        this.replace_next(action, cx);
                    })
                }
            }))
            .on_action(cx.listener(|this, action, cx| {
                if let Some(search) = this.active_project_search.as_ref() {
                    search.update(cx, |this, cx| {
                        this.replace_all(action, cx);
                    })
                }
            }))
            .when(search.filters_enabled, |this| {
                this.on_action(cx.listener(|this, _: &ToggleIncludeIgnored, cx| {
                    this.toggle_search_option(SearchOptions::INCLUDE_IGNORED, cx);
                }))
            })
            .on_action(cx.listener(Self::select_next_match))
            .on_action(cx.listener(Self::select_prev_match))
            .gap_2()
            .w_full()
            .child(search_line)
            .children(replace_line)
            .children(filter_line)
    }
}

impl EventEmitter<ToolbarItemEvent> for ProjectSearchBar {}

impl ToolbarItemView for ProjectSearchBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
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
    callback: fn(&mut ProjectSearchBar, &A, &mut ViewContext<ProjectSearchBar>),
) {
    workspace.register_action(move |workspace, action: &A, cx| {
        if workspace.has_active_modal(cx) {
            cx.propagate();
            return;
        }

        workspace.active_pane().update(cx, |pane, cx| {
            pane.toolbar().update(cx, move |workspace, cx| {
                if let Some(search_bar) = workspace.item_of_type::<ProjectSearchBar>() {
                    search_bar.update(cx, move |search_bar, cx| {
                        if search_bar.active_project_search.is_some() {
                            callback(search_bar, action, cx);
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
    callback: fn(&mut Workspace, &A, &mut ViewContext<Workspace>),
) {
    workspace.register_action(move |workspace, action: &A, cx| {
        if workspace.has_active_modal(cx) {
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
            callback(workspace, action, cx);
            cx.notify();
        } else {
            cx.propagate();
        }
    });
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use editor::{display_map::DisplayRow, DisplayPoint};
    use gpui::{Action, TestAppContext, WindowHandle};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::sync::Arc;
    use workspace::DeploySearch;

    #[gpui::test]
    async fn test_project_search(cx: &mut TestAppContext) {
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
        let search = cx.new_model(|cx| ProjectSearch::new(project, cx));
        let search_view = cx.add_window(|cx| ProjectSearchView::new(search.clone(), cx, None));

        perform_search(search_view, "TWO", cx);
        search_view.update(cx, |search_view, cx| {
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\n\n\nconst TWO: usize = one::ONE + one::ONE;\n"
            );
            let match_background_color = cx.theme().colors().search_match_background;
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.all_text_background_highlights(cx)),
                &[
                    (
                        DisplayPoint::new(DisplayRow(3), 32)..DisplayPoint::new(DisplayRow(3), 35),
                        match_background_color
                    ),
                    (
                        DisplayPoint::new(DisplayRow(3), 37)..DisplayPoint::new(DisplayRow(3), 40),
                        match_background_color
                    ),
                    (
                        DisplayPoint::new(DisplayRow(8), 6)..DisplayPoint::new(DisplayRow(8), 9),
                        match_background_color
                    )
                ]
            );
            assert_eq!(search_view.active_match_index, Some(0));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(DisplayRow(3), 32)..DisplayPoint::new(DisplayRow(3), 35)]
            );

            search_view.select_match(Direction::Next, cx);
        }).unwrap();

        search_view
            .update(cx, |search_view, cx| {
                assert_eq!(search_view.active_match_index, Some(1));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(3), 37)..DisplayPoint::new(DisplayRow(3), 40)]
                );
                search_view.select_match(Direction::Next, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, cx| {
                assert_eq!(search_view.active_match_index, Some(2));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(8), 6)..DisplayPoint::new(DisplayRow(8), 9)]
                );
                search_view.select_match(Direction::Next, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, cx| {
                assert_eq!(search_view.active_match_index, Some(0));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(3), 32)..DisplayPoint::new(DisplayRow(3), 35)]
                );
                search_view.select_match(Direction::Prev, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, cx| {
                assert_eq!(search_view.active_match_index, Some(2));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(8), 6)..DisplayPoint::new(DisplayRow(8), 9)]
                );
                search_view.select_match(Direction::Prev, cx);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, cx| {
                assert_eq!(search_view.active_match_index, Some(1));
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                    [DisplayPoint::new(DisplayRow(3), 37)..DisplayPoint::new(DisplayRow(3), 40)]
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
        let window = cx.add_window(|cx| Workspace::test_new(project, cx));
        let workspace = window;
        let search_bar = window.build_view(cx, |_| ProjectSearchBar::new());

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
            .update(cx, move |workspace, cx| {
                assert_eq!(workspace.panes().len(), 1);
                workspace.panes()[0].update(cx, move |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
                });

                ProjectSearchView::deploy_search(workspace, &workspace::DeploySearch::find(), cx)
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
                .update(&mut cx, |_, cx| {
                    cx.dispatch_action(ToggleFocus.boxed_clone())
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(cx),
                    "Empty search view should be focused after the toggle focus event: no results panel to focus on",
                );
           });
        }).unwrap();

        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    let query_editor = &search_view.query_editor;
                    assert!(
                        query_editor.focus_handle(cx).is_focused(cx),
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
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXIST", cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert!(
                    results_text.is_empty(),
                    "Search view for mismatching query should have no results but got '{results_text}'"
                );
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(cx),
                    "Search view should be focused after mismatching query had been used in search",
                );
            });
        }).unwrap();

        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, cx| {
                cx.dispatch_action(ToggleFocus.boxed_clone())
            })
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(cx),
                    "Search view with mismatching query should be focused after the toggle focus event: still no results panel to focus on",
                );
            });
        }).unwrap();

        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("TWO", cx));
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\n\n\nconst TWO: usize = one::ONE + one::ONE;\n",
                    "Search view results should match the query"
                );
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(cx),
                    "Search view with mismatching query should be focused after search results are available",
                );
            });
        }).unwrap();
        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, cx| {
                    cx.dispatch_action(ToggleFocus.boxed_clone())
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(cx),
                    "Search view with matching query should still have its results editor focused after the toggle focus event",
                );
            });
        }).unwrap();

        workspace
            .update(cx, |workspace, cx| {
                ProjectSearchView::deploy_search(workspace, &workspace::DeploySearch::find(), cx)
            })
            .unwrap();
        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                assert_eq!(search_view.query_editor.read(cx).text(cx), "two", "Query should be updated to first search result after search view 2nd open in a row");
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\n\n\nconst TWO: usize = one::ONE + one::ONE;\n",
                    "Results should be unchanged after search view 2nd open in a row"
                );
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(cx),
                    "Focus should be moved into query editor again after search view 2nd open in a row"
                );
            });
        }).unwrap();

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, cx| {
                    cx.dispatch_action(ToggleFocus.boxed_clone())
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(cx),
                    "Search view with matching query should switch focus to the results editor after the toggle focus event",
                );
            });
        }).unwrap();
    }

    #[gpui::test]
    async fn test_new_project_search_focus(cx: &mut TestAppContext) {
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
        let window = cx.add_window(|cx| Workspace::test_new(project, cx));
        let workspace = window;
        let search_bar = window.build_view(cx, |_| ProjectSearchBar::new());

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
            .update(cx, move |workspace, cx| {
                assert_eq!(workspace.panes().len(), 1);
                workspace.panes()[0].update(cx, move |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
                });

                ProjectSearchView::new_search(workspace, &workspace::NewSearch, cx)
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
                .update(&mut cx, |_, cx| {
                    cx.dispatch_action(ToggleFocus.boxed_clone())
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();

        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(cx),
                        "Empty search view should be focused after the toggle focus event: no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    let query_editor = &search_view.query_editor;
                    assert!(
                        query_editor.focus_handle(cx).is_focused(cx),
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
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXIST", cx)
                    });
                    search_view.search(cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                results_text.is_empty(),
                "Search view for mismatching query should have no results but got '{results_text}'"
            );
                    assert!(
                search_view.query_editor.focus_handle(cx).is_focused(cx),
                "Search view should be focused after mismatching query had been used in search",
            );
                });
            })
            .unwrap();
        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, cx| {
                cx.dispatch_action(ToggleFocus.boxed_clone())
            })
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(cx),
                        "Search view with mismatching query should be focused after the toggle focus event: still no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("TWO", cx));
                    search_view.search(cx);
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx|
        search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\n\n\nconst TWO: usize = one::ONE + one::ONE;\n",
                    "Search view results should match the query"
                );
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(cx),
                    "Search view with mismatching query should be focused after search results are available",
                );
            })).unwrap();
        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, cx| {
                    cx.dispatch_action(ToggleFocus.boxed_clone())
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.results_editor.focus_handle(cx).is_focused(cx),
                        "Search view with matching query should still have its results editor focused after the toggle focus event",
                    );
                });
        }).unwrap();

        workspace
            .update(cx, |workspace, cx| {
                ProjectSearchView::new_search(workspace, &workspace::NewSearch, cx)
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

        window.update(cx, |_, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO", "First search view should not have an updated query");
                    assert_eq!(
                        search_view
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "\n\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\n\n\nconst TWO: usize = one::ONE + one::ONE;\n",
                        "Results of the first search view should not update too"
                    );
                    assert!(
                        !search_view.query_editor.focus_handle(cx).is_focused(cx),
                        "Focus should be moved away from the first search view"
                    );
                });
        }).unwrap();

        window.update(cx, |_, cx| {
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
                        search_view_2.query_editor.focus_handle(cx).is_focused(cx),
                        "Focus should be moved into query editor of the new window"
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, cx| {
                search_view_2.update(cx, |search_view_2, cx| {
                    search_view_2
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("FOUR", cx));
                    search_view_2.search(cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert_eq!(
                        search_view_2
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "\n\n\nconst FOUR: usize = one::ONE + three::THREE;\n",
                        "New search view with the updated query should have new search results"
                    );
                    assert!(
                        search_view_2.results_editor.focus_handle(cx).is_focused(cx),
                        "Search view with mismatching query should be focused after search results are available",
                    );
                });
        }).unwrap();

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, cx| {
                    cx.dispatch_action(ToggleFocus.boxed_clone())
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert!(
                        search_view_2.results_editor.focus_handle(cx).is_focused(cx),
                        "Search view with matching query should switch focus to the results editor after the toggle focus event",
                    );
                });}).unwrap();
    }

    #[gpui::test]
    async fn test_new_project_search_in_directory(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/dir",
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
            project.worktrees().next().unwrap().read(cx).id()
        });
        let window = cx.add_window(|cx| Workspace::test_new(project, cx));
        let workspace = window.root(cx).unwrap();
        let search_bar = window.build_view(cx, |_| ProjectSearchBar::new());

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
            .update(cx, move |workspace, cx| {
                assert_eq!(workspace.panes().len(), 1);
                workspace.panes()[0].update(cx, move |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
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
            .update(cx, |workspace, cx| {
                ProjectSearchView::new_search_in_directory(workspace, &a_dir_entry.path, cx)
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
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(cx),
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
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("const", cx));
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\n\nconst ONE: usize = 1;\n\n\n\n\nconst TWO: usize = one::ONE + one::ONE;\n",
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
        let window = cx.add_window(|cx| Workspace::test_new(project, cx));
        let workspace = window.root(cx).unwrap();
        let search_bar = window.build_view(cx, |_| ProjectSearchBar::new());

        window
            .update(cx, {
                let search_bar = search_bar.clone();
                move |workspace, cx| {
                    assert_eq!(workspace.panes().len(), 1);
                    workspace.panes()[0].update(cx, move |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, cx)
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
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.search_options = SearchOptions::CASE_SENSITIVE;
                    search_view
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("ONE", cx));
                    search_view.search(cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("TWO", cx));
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("THREE", cx));
                    search_view.search(cx);
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("JUST_TEXT_INPUT", cx)
                    });
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();

        // Ensure that the latest input with search settings is active.
        window
            .update(cx, |_, cx| {
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
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.next_history_query(&NextHistoryQuery, cx);
                })
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.next_history_query(&NextHistoryQuery, cx);
                })
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // First previous query for empty current query should set the query to the latest submitted one.
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.previous_history_query(&PreviousHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Further previous items should go over the history in reverse order.
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.previous_history_query(&PreviousHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Previous items should never go behind the first history item.
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.previous_history_query(&PreviousHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.previous_history_query(&PreviousHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Next items should go over the history in the original order.
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.next_history_query(&NextHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .query_editor
                        .update(cx, |query_editor, cx| query_editor.set_text("TWO_NEW", cx));
                    search_view.search(cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // New search input should add another entry to history and move the selection to the end of the history.
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.previous_history_query(&PreviousHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.previous_history_query(&PreviousHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.next_history_query(&NextHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.next_history_query(&NextHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.next_history_query(&NextHistoryQuery, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, cx| {
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
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees().next().unwrap().read(cx).id()
        });

        let window = cx.add_window(|cx| Workspace::test_new(project, cx));
        let workspace = window.root(cx).unwrap();

        let panes: Vec<_> = window
            .update(cx, |this, _| this.panes().to_owned())
            .unwrap();

        let search_bar_1 = window.build_view(cx, |_| ProjectSearchBar::new());
        let search_bar_2 = window.build_view(cx, |_| ProjectSearchBar::new());

        assert_eq!(panes.len(), 1);
        let first_pane = panes.get(0).cloned().unwrap();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 0);
        window
            .update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, "one.rs"),
                    Some(first_pane.downgrade()),
                    true,
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
                let pane = first_pane.clone();
                move |workspace, cx| {
                    pane.update(cx, move |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, cx)
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
            .update(cx, |workspace, cx| {
                workspace.split_and_clone(first_pane.clone(), workspace::SplitDirection::Right, cx)
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
                move |workspace, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    pane.update(cx, move |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, cx)
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
            |search_view: &View<ProjectSearchView>, query: &str, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, cx| {
                        search_view.update(cx, |search_view, cx| {
                            search_view
                                .query_editor
                                .update(cx, |query_editor, cx| query_editor.set_text(query, cx));
                            search_view.search(cx);
                        });
                    })
                    .unwrap();
            };

        let active_query =
            |search_view: &View<ProjectSearchView>, cx: &mut TestAppContext| -> String {
                window
                    .update(cx, |_, cx| {
                        search_view.update(cx, |search_view, cx| {
                            search_view.query_editor.read(cx).text(cx).to_string()
                        })
                    })
                    .unwrap()
            };

        let select_prev_history_item =
            |search_bar: &View<ProjectSearchBar>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, cx| {
                        search_bar.update(cx, |search_bar, cx| {
                            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
                        })
                    })
                    .unwrap();
            };

        let select_next_history_item =
            |search_bar: &View<ProjectSearchBar>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, cx| {
                        search_bar.update(cx, |search_bar, cx| {
                            search_bar.next_history_query(&NextHistoryQuery, cx);
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
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees().next().unwrap().read(cx).id()
        });
        let window = cx.add_window(|cx| Workspace::test_new(project, cx));
        let panes: Vec<_> = window
            .update(cx, |this, _| this.panes().to_owned())
            .unwrap();
        assert_eq!(panes.len(), 1);
        let first_pane = panes.get(0).cloned().unwrap();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 0);
        window
            .update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, "one.rs"),
                    Some(first_pane.downgrade()),
                    true,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 1);
        let second_pane = window
            .update(cx, |workspace, cx| {
                workspace.split_and_clone(first_pane.clone(), workspace::SplitDirection::Right, cx)
            })
            .unwrap()
            .unwrap();
        assert_eq!(cx.update(|cx| second_pane.read(cx).items_len()), 1);
        assert!(window
            .update(cx, |_, cx| second_pane
                .focus_handle(cx)
                .contains_focused(cx))
            .unwrap());
        let search_bar = window.build_view(cx, |_| ProjectSearchBar::new());
        window
            .update(cx, {
                let search_bar = search_bar.clone();
                let pane = first_pane.clone();
                move |workspace, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    pane.update(cx, move |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
                    });
                }
            })
            .unwrap();

        // Add a project search item to the second pane
        window
            .update(cx, {
                let search_bar = search_bar.clone();
                let pane = second_pane.clone();
                move |workspace, cx| {
                    assert_eq!(workspace.panes().len(), 2);
                    pane.update(cx, move |pane, cx| {
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(search_bar, cx))
                    });

                    ProjectSearchView::new_search(workspace, &workspace::NewSearch, cx)
                }
            })
            .unwrap();

        cx.run_until_parked();
        assert_eq!(cx.update(|cx| second_pane.read(cx).items_len()), 2);
        assert_eq!(cx.update(|cx| first_pane.read(cx).items_len()), 1);

        // Focus the first pane
        window
            .update(cx, |workspace, cx| {
                assert_eq!(workspace.active_pane(), &second_pane);
                second_pane.update(cx, |this, cx| {
                    assert_eq!(this.active_item_index(), 1);
                    this.activate_prev_item(false, cx);
                    assert_eq!(this.active_item_index(), 0);
                });
                workspace.activate_pane_in_direction(workspace::SplitDirection::Left, cx);
            })
            .unwrap();
        window
            .update(cx, |workspace, cx| {
                assert_eq!(workspace.active_pane(), &first_pane);
                assert_eq!(first_pane.read(cx).items_len(), 1);
                assert_eq!(second_pane.read(cx).items_len(), 2);
            })
            .unwrap();

        // Deploy a new search
        cx.dispatch_action(window.into(), DeploySearch::find());

        // Both panes should now have a project search in them
        window
            .update(cx, |workspace, cx| {
                assert_eq!(workspace.active_pane(), &first_pane);
                first_pane.update(cx, |this, _| {
                    assert_eq!(this.active_item_index(), 1);
                    assert_eq!(this.items_len(), 2);
                });
                second_pane.update(cx, |this, cx| {
                    assert!(!cx.focus_handle().contains_focused(cx));
                    assert_eq!(this.items_len(), 2);
                });
            })
            .unwrap();

        // Focus the second pane's non-search item
        window
            .update(cx, |_workspace, cx| {
                second_pane.update(cx, |pane, cx| pane.activate_next_item(true, cx));
            })
            .unwrap();

        // Deploy a new search
        cx.dispatch_action(window.into(), DeploySearch::find());

        // The project search view should now be focused in the second pane
        // And the number of items should be unchanged.
        window
            .update(cx, |_workspace, cx| {
                second_pane.update(cx, |pane, _cx| {
                    assert!(pane
                        .active_item()
                        .unwrap()
                        .downcast::<ProjectSearchView>()
                        .is_some());

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
            "/dir",
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
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let search = cx.new_model(|cx| ProjectSearch::new(project, cx));
        let search_view = cx.add_window(|cx| ProjectSearchView::new(search.clone(), cx, None));

        // First search
        perform_search(search_view, "A", cx);
        search_view
            .update(cx, |search_view, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    // Results are correct and scrolled to the top
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(" A ").count(),
                        10
                    );
                    assert_eq!(results_editor.scroll_position(cx), Point::default());

                    // Scroll results all the way down
                    results_editor.scroll(Point::new(0., f32::MAX), Some(Axis::Vertical), cx);
                });
            })
            .expect("unable to update search view");

        // Second search
        perform_search(search_view, "B", cx);
        search_view
            .update(cx, |search_view, cx| {
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
            super::init(cx);
        });
    }

    fn perform_search(
        search_view: WindowHandle<ProjectSearchView>,
        text: impl Into<Arc<str>>,
        cx: &mut TestAppContext,
    ) {
        search_view
            .update(cx, |search_view, cx| {
                search_view
                    .query_editor
                    .update(cx, |query_editor, cx| query_editor.set_text(text, cx));
                search_view.search(cx);
            })
            .unwrap();
        cx.background_executor.run_until_parked();
    }
}
