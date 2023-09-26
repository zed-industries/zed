use crate::{
    history::SearchHistory,
    mode::{SearchMode, Side},
    search_bar::{render_nav_button, render_option_button_icon, render_search_mode_button},
    ActivateRegexMode, ActivateSemanticMode, ActivateTextMode, CycleMode, NextHistoryQuery,
    PreviousHistoryQuery, ReplaceAll, ReplaceNext, SearchOptions, SelectNextMatch, SelectPrevMatch,
    ToggleCaseSensitive, ToggleReplace, ToggleWholeWord,
};
use anyhow::{Context, Result};
use collections::HashMap;
use editor::{
    items::active_match_index, scroll::autoscroll::Autoscroll, Anchor, Editor, MultiBuffer,
    SelectAll, MAX_TAB_TITLE_LEN,
};
use futures::StreamExt;
use gpui::{
    actions,
    elements::*,
    platform::{MouseButton, PromptLevel},
    Action, AnyElement, AnyViewHandle, AppContext, Entity, ModelContext, ModelHandle, Subscription,
    Task, View, ViewContext, ViewHandle, WeakModelHandle, WeakViewHandle,
};
use menu::Confirm;
use project::{
    search::{PathMatcher, SearchInputs, SearchQuery},
    Entry, Project,
};
use semantic_index::{SemanticIndex, SemanticIndexStatus};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    collections::HashSet,
    mem,
    ops::{Not, Range},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use util::ResultExt as _;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle},
    searchable::{Direction, SearchableItem, SearchableItemHandle},
    ItemNavHistory, Pane, ToolbarItemLocation, ToolbarItemView, Workspace, WorkspaceId,
};

actions!(
    project_search,
    [SearchInNew, ToggleFocus, NextField, ToggleFilters,]
);

#[derive(Default)]
struct ActiveSearches(HashMap<WeakModelHandle<Project>, WeakViewHandle<ProjectSearchView>>);

#[derive(Default)]
struct ActiveSettings(HashMap<WeakModelHandle<Project>, ProjectSearchSettings>);

pub fn init(cx: &mut AppContext) {
    cx.set_global(ActiveSearches::default());
    cx.set_global(ActiveSettings::default());
    cx.add_action(ProjectSearchView::deploy);
    cx.add_action(ProjectSearchView::move_focus_to_results);
    cx.add_action(ProjectSearchBar::confirm);
    cx.add_action(ProjectSearchBar::search_in_new);
    cx.add_action(ProjectSearchBar::select_next_match);
    cx.add_action(ProjectSearchBar::select_prev_match);
    cx.add_action(ProjectSearchBar::replace_next);
    cx.add_action(ProjectSearchBar::replace_all);
    cx.add_action(ProjectSearchBar::cycle_mode);
    cx.add_action(ProjectSearchBar::next_history_query);
    cx.add_action(ProjectSearchBar::previous_history_query);
    cx.add_action(ProjectSearchBar::activate_regex_mode);
    cx.add_action(ProjectSearchBar::toggle_replace);
    cx.add_action(ProjectSearchBar::toggle_replace_on_a_pane);
    cx.add_action(ProjectSearchBar::activate_text_mode);

    // This action should only be registered if the semantic index is enabled
    // We are registering it all the time, as I dont want to introduce a dependency
    // for Semantic Index Settings globally whenever search is tested.
    cx.add_action(ProjectSearchBar::activate_semantic_mode);

    cx.capture_action(ProjectSearchBar::tab);
    cx.capture_action(ProjectSearchBar::tab_previous);
    cx.capture_action(ProjectSearchView::replace_all);
    cx.capture_action(ProjectSearchView::replace_next);
    add_toggle_option_action::<ToggleCaseSensitive>(SearchOptions::CASE_SENSITIVE, cx);
    add_toggle_option_action::<ToggleWholeWord>(SearchOptions::WHOLE_WORD, cx);
    add_toggle_filters_action::<ToggleFilters>(cx);
}

fn add_toggle_filters_action<A: Action>(cx: &mut AppContext) {
    cx.add_action(move |pane: &mut Pane, _: &A, cx: &mut ViewContext<Pane>| {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<ProjectSearchBar>() {
            if search_bar.update(cx, |search_bar, cx| search_bar.toggle_filters(cx)) {
                return;
            }
        }
        cx.propagate_action();
    });
}

fn add_toggle_option_action<A: Action>(option: SearchOptions, cx: &mut AppContext) {
    cx.add_action(move |pane: &mut Pane, _: &A, cx: &mut ViewContext<Pane>| {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<ProjectSearchBar>() {
            if search_bar.update(cx, |search_bar, cx| {
                search_bar.toggle_search_option(option, cx)
            }) {
                return;
            }
        }
        cx.propagate_action();
    });
}

struct ProjectSearch {
    project: ModelHandle<Project>,
    excerpts: ModelHandle<MultiBuffer>,
    pending_search: Option<Task<Option<()>>>,
    match_ranges: Vec<Range<Anchor>>,
    active_query: Option<SearchQuery>,
    search_id: usize,
    search_history: SearchHistory,
    no_results: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputPanel {
    Query,
    Exclude,
    Include,
}

pub struct ProjectSearchView {
    model: ModelHandle<ProjectSearch>,
    query_editor: ViewHandle<Editor>,
    replacement_editor: ViewHandle<Editor>,
    results_editor: ViewHandle<Editor>,
    semantic_state: Option<SemanticState>,
    semantic_permissioned: Option<bool>,
    search_options: SearchOptions,
    panels_with_errors: HashSet<InputPanel>,
    active_match_index: Option<usize>,
    search_id: usize,
    query_editor_was_focused: bool,
    included_files_editor: ViewHandle<Editor>,
    excluded_files_editor: ViewHandle<Editor>,
    filters_enabled: bool,
    replace_enabled: bool,
    current_mode: SearchMode,
}

struct SemanticState {
    index_status: SemanticIndexStatus,
    maintain_rate_limit: Option<Task<()>>,
    _subscription: Subscription,
}

#[derive(Debug, Clone)]
struct ProjectSearchSettings {
    search_options: SearchOptions,
    filters_enabled: bool,
    current_mode: SearchMode,
}

pub struct ProjectSearchBar {
    active_project_search: Option<ViewHandle<ProjectSearchView>>,
    subscription: Option<Subscription>,
}

impl Entity for ProjectSearch {
    type Event = ();
}

impl ProjectSearch {
    fn new(project: ModelHandle<Project>, cx: &mut ModelContext<Self>) -> Self {
        let replica_id = project.read(cx).replica_id();
        Self {
            project,
            excerpts: cx.add_model(|_| MultiBuffer::new(replica_id)),
            pending_search: Default::default(),
            match_ranges: Default::default(),
            active_query: None,
            search_id: 0,
            search_history: SearchHistory::default(),
            no_results: None,
        }
    }

    fn clone(&self, cx: &mut ModelContext<Self>) -> ModelHandle<Self> {
        cx.add_model(|cx| Self {
            project: self.project.clone(),
            excerpts: self
                .excerpts
                .update(cx, |excerpts, cx| cx.add_model(|cx| excerpts.clone(cx))),
            pending_search: Default::default(),
            match_ranges: self.match_ranges.clone(),
            active_query: self.active_query.clone(),
            search_id: self.search_id,
            search_history: self.search_history.clone(),
            no_results: self.no_results.clone(),
        })
    }

    fn search(&mut self, query: SearchQuery, cx: &mut ModelContext<Self>) {
        let search = self
            .project
            .update(cx, |project, cx| project.search(query.clone(), cx));
        self.search_id += 1;
        self.search_history.add(query.as_str().to_string());
        self.active_query = Some(query);
        self.match_ranges.clear();
        self.pending_search = Some(cx.spawn_weak(|this, mut cx| async move {
            let mut matches = search;
            let this = this.upgrade(&cx)?;
            this.update(&mut cx, |this, cx| {
                this.match_ranges.clear();
                this.excerpts.update(cx, |this, cx| this.clear(cx));
                this.no_results = Some(true);
            });

            while let Some((buffer, anchors)) = matches.next().await {
                let mut ranges = this.update(&mut cx, |this, cx| {
                    this.no_results = Some(false);
                    this.excerpts.update(cx, |excerpts, cx| {
                        excerpts.stream_excerpts_with_context_lines(buffer, anchors, 1, cx)
                    })
                });

                while let Some(range) = ranges.next().await {
                    this.update(&mut cx, |this, _| this.match_ranges.push(range));
                }
                this.update(&mut cx, |_, cx| cx.notify());
            }

            this.update(&mut cx, |this, cx| {
                this.pending_search.take();
                cx.notify();
            });

            None
        }));
        cx.notify();
    }

    fn semantic_search(&mut self, inputs: &SearchInputs, cx: &mut ModelContext<Self>) {
        let search = SemanticIndex::global(cx).map(|index| {
            index.update(cx, |semantic_index, cx| {
                semantic_index.search_project(
                    self.project.clone(),
                    inputs.as_str().to_owned(),
                    10,
                    inputs.files_to_include().to_vec(),
                    inputs.files_to_exclude().to_vec(),
                    cx,
                )
            })
        });
        self.search_id += 1;
        self.match_ranges.clear();
        self.search_history.add(inputs.as_str().to_string());
        self.no_results = None;
        self.pending_search = Some(cx.spawn(|this, mut cx| async move {
            let results = search?.await.log_err()?;
            let matches = results
                .into_iter()
                .map(|result| (result.buffer, vec![result.range.start..result.range.start]));

            this.update(&mut cx, |this, cx| {
                this.no_results = Some(true);
                this.excerpts.update(cx, |excerpts, cx| {
                    excerpts.clear(cx);
                });
            });
            for (buffer, ranges) in matches {
                let mut match_ranges = this.update(&mut cx, |this, cx| {
                    this.no_results = Some(false);
                    this.excerpts.update(cx, |excerpts, cx| {
                        excerpts.stream_excerpts_with_context_lines(buffer, ranges, 3, cx)
                    })
                });
                while let Some(match_range) = match_ranges.next().await {
                    this.update(&mut cx, |this, cx| {
                        this.match_ranges.push(match_range);
                        while let Ok(Some(match_range)) = match_ranges.try_next() {
                            this.match_ranges.push(match_range);
                        }
                        cx.notify();
                    });
                }
            }

            this.update(&mut cx, |this, cx| {
                this.pending_search.take();
                cx.notify();
            });

            None
        }));
        cx.notify();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViewEvent {
    UpdateTab,
    Activate,
    EditorEvent(editor::Event),
    Dismiss,
}

impl Entity for ProjectSearchView {
    type Event = ViewEvent;
}

impl View for ProjectSearchView {
    fn ui_name() -> &'static str {
        "ProjectSearchView"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let model = &self.model.read(cx);
        if model.match_ranges.is_empty() {
            enum Status {}

            let theme = theme::current(cx).clone();

            // If Search is Active -> Major: Searching..., Minor: None
            // If Semantic -> Major: "Search using Natural Language", Minor: {Status}/n{ex...}/n{ex...}
            // If Regex -> Major: "Search using Regex", Minor: {ex...}
            // If Text -> Major: "Text search all files and folders", Minor: {...}

            let current_mode = self.current_mode;
            let mut major_text = if model.pending_search.is_some() {
                Cow::Borrowed("Searching...")
            } else if model.no_results.is_some_and(|v| v) {
                Cow::Borrowed("No Results")
            } else {
                match current_mode {
                    SearchMode::Text => Cow::Borrowed("Text search all files and folders"),
                    SearchMode::Semantic => {
                        Cow::Borrowed("Search all code objects using Natural Language")
                    }
                    SearchMode::Regex => Cow::Borrowed("Regex search all files and folders"),
                }
            };

            let mut show_minor_text = true;
            let semantic_status = self.semantic_state.as_ref().and_then(|semantic| {
                let status = semantic.index_status;
                match status {
                    SemanticIndexStatus::NotAuthenticated => {
                        major_text = Cow::Borrowed("Not Authenticated");
                        show_minor_text = false;
                        Some(
                            "API Key Missing: Please set 'OPENAI_API_KEY' in Environment Variables"
                                .to_string(),
                        )
                    }
                    SemanticIndexStatus::Indexed => Some("Indexing complete".to_string()),
                    SemanticIndexStatus::Indexing {
                        remaining_files,
                        rate_limit_expiry,
                    } => {
                        if remaining_files == 0 {
                            Some(format!("Indexing..."))
                        } else {
                            if let Some(rate_limit_expiry) = rate_limit_expiry {
                                let remaining_seconds =
                                    rate_limit_expiry.duration_since(Instant::now());
                                if remaining_seconds > Duration::from_secs(0) {
                                    Some(format!(
                                        "Remaining files to index (rate limit resets in {}s): {}",
                                        remaining_seconds.as_secs(),
                                        remaining_files
                                    ))
                                } else {
                                    Some(format!("Remaining files to index: {}", remaining_files))
                                }
                            } else {
                                Some(format!("Remaining files to index: {}", remaining_files))
                            }
                        }
                    }
                    SemanticIndexStatus::NotIndexed => None,
                }
            });

            let minor_text = if let Some(no_results) = model.no_results {
                if model.pending_search.is_none() && no_results {
                    vec!["No results found in this project for the provided query".to_owned()]
                } else {
                    vec![]
                }
            } else {
                match current_mode {
                    SearchMode::Semantic => {
                        let mut minor_text = Vec::new();
                        minor_text.push("".into());
                        minor_text.extend(semantic_status);
                        if show_minor_text {
                            minor_text
                                .push("Simply explain the code you are looking to find.".into());
                            minor_text.push(
                                "ex. 'prompt user for permissions to index their project'".into(),
                            );
                        }
                        minor_text
                    }
                    _ => vec![
                        "".to_owned(),
                        "Include/exclude specific paths with the filter option.".to_owned(),
                        "Matching exact word and/or casing is available too.".to_owned(),
                    ],
                }
            };

            let previous_query_keystrokes =
                cx.binding_for_action(&PreviousHistoryQuery {})
                    .map(|binding| {
                        binding
                            .keystrokes()
                            .iter()
                            .map(|k| k.to_string())
                            .collect::<Vec<_>>()
                    });
            let next_query_keystrokes =
                cx.binding_for_action(&NextHistoryQuery {}).map(|binding| {
                    binding
                        .keystrokes()
                        .iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                });
            let new_placeholder_text = match (previous_query_keystrokes, next_query_keystrokes) {
                (Some(previous_query_keystrokes), Some(next_query_keystrokes)) => {
                    format!(
                        "Search ({}/{} for previous/next query)",
                        previous_query_keystrokes.join(" "),
                        next_query_keystrokes.join(" ")
                    )
                }
                (None, Some(next_query_keystrokes)) => {
                    format!(
                        "Search ({} for next query)",
                        next_query_keystrokes.join(" ")
                    )
                }
                (Some(previous_query_keystrokes), None) => {
                    format!(
                        "Search ({} for previous query)",
                        previous_query_keystrokes.join(" ")
                    )
                }
                (None, None) => String::new(),
            };
            self.query_editor.update(cx, |editor, cx| {
                editor.set_placeholder_text(new_placeholder_text, cx);
            });

            MouseEventHandler::new::<Status, _>(0, cx, |_, _| {
                Flex::column()
                    .with_child(Flex::column().contained().flex(1., true))
                    .with_child(
                        Flex::column()
                            .align_children_center()
                            .with_child(Label::new(
                                major_text,
                                theme.search.major_results_status.clone(),
                            ))
                            .with_children(
                                minor_text.into_iter().map(|x| {
                                    Label::new(x, theme.search.minor_results_status.clone())
                                }),
                            )
                            .aligned()
                            .top()
                            .contained()
                            .flex(7., true),
                    )
                    .contained()
                    .with_background_color(theme.editor.background)
            })
            .on_down(MouseButton::Left, |_, _, cx| {
                cx.focus_parent();
            })
            .into_any_named("project search view")
        } else {
            ChildView::new(&self.results_editor, cx)
                .flex(1., true)
                .into_any_named("project search view")
        }
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        let handle = cx.weak_handle();
        cx.update_global(|state: &mut ActiveSearches, cx| {
            state
                .0
                .insert(self.model.read(cx).project.downgrade(), handle)
        });

        cx.update_global(|state: &mut ActiveSettings, cx| {
            state.0.insert(
                self.model.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });

        if cx.is_self_focused() {
            if self.query_editor_was_focused {
                cx.focus(&self.query_editor);
            } else {
                cx.focus(&self.results_editor);
            }
        }
    }
}

impl Item for ProjectSearchView {
    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<Cow<str>> {
        let query_text = self.query_editor.read(cx).text(cx);

        query_text
            .is_empty()
            .not()
            .then(|| query_text.into())
            .or_else(|| Some("Project Search".into()))
    }
    fn should_close_item_on_event(event: &Self::Event) -> bool {
        event == &Self::Event::Dismiss
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a ViewHandle<Self>,
        _: &'a AppContext,
    ) -> Option<&'a AnyViewHandle> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle)
        } else if type_id == TypeId::of::<Editor>() {
            Some(&self.results_editor)
        } else {
            None
        }
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.results_editor
            .update(cx, |editor, cx| editor.deactivated(cx));
    }

    fn tab_content<T: 'static>(
        &self,
        _detail: Option<usize>,
        tab_theme: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<T> {
        Flex::row()
            .with_child(
                Svg::new("icons/magnifying_glass.svg")
                    .with_color(tab_theme.label.text.color)
                    .constrained()
                    .with_width(tab_theme.type_icon_width)
                    .aligned()
                    .contained()
                    .with_margin_right(tab_theme.spacing),
            )
            .with_child({
                let tab_name: Option<Cow<_>> = self
                    .model
                    .read(cx)
                    .search_history
                    .current()
                    .as_ref()
                    .map(|query| {
                        let query_text = util::truncate_and_trailoff(query, MAX_TAB_TITLE_LEN);
                        query_text.into()
                    });
                Label::new(
                    tab_name
                        .filter(|name| !name.is_empty())
                        .unwrap_or("Project search".into()),
                    tab_theme.label.clone(),
                )
                .aligned()
            })
            .into_any()
    }

    fn for_each_project_item(&self, cx: &AppContext, f: &mut dyn FnMut(usize, &dyn project::Item)) {
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
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.save(project, cx))
    }

    fn save_as(
        &mut self,
        _: ModelHandle<Project>,
        _: PathBuf,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called")
    }

    fn reload(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.reload(project, cx))
    }

    fn clone_on_split(&self, _workspace_id: WorkspaceId, cx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        let model = self.model.update(cx, |model, cx| model.clone(cx));
        Some(Self::new(model, cx, None))
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

    fn to_item_events(event: &Self::Event) -> SmallVec<[ItemEvent; 2]> {
        match event {
            ViewEvent::UpdateTab => {
                smallvec::smallvec![ItemEvent::UpdateBreadcrumbs, ItemEvent::UpdateTab]
            }
            ViewEvent::EditorEvent(editor_event) => Editor::to_item_events(editor_event),
            ViewEvent::Dismiss => smallvec::smallvec![ItemEvent::CloseItem],
            _ => SmallVec::new(),
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
        _project: ModelHandle<Project>,
        _workspace: WeakViewHandle<Workspace>,
        _workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        _cx: &mut ViewContext<Pane>,
    ) -> Task<anyhow::Result<ViewHandle<Self>>> {
        unimplemented!()
    }
}

impl ProjectSearchView {
    fn toggle_filters(&mut self, cx: &mut ViewContext<Self>) {
        self.filters_enabled = !self.filters_enabled;
        cx.update_global(|state: &mut ActiveSettings, cx| {
            state.0.insert(
                self.model.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });
    }

    fn current_settings(&self) -> ProjectSearchSettings {
        ProjectSearchSettings {
            search_options: self.search_options,
            filters_enabled: self.filters_enabled,
            current_mode: self.current_mode,
        }
    }
    fn toggle_search_option(&mut self, option: SearchOptions, cx: &mut ViewContext<Self>) {
        self.search_options.toggle(option);
        cx.update_global(|state: &mut ActiveSettings, cx| {
            state.0.insert(
                self.model.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });
    }

    fn index_project(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(semantic_index) = SemanticIndex::global(cx) {
            // Semantic search uses no options
            self.search_options = SearchOptions::none();

            let project = self.model.read(cx).project.clone();

            semantic_index.update(cx, |semantic_index, cx| {
                semantic_index
                    .index_project(project.clone(), cx)
                    .detach_and_log_err(cx);
            });

            self.semantic_state = Some(SemanticState {
                index_status: semantic_index.read(cx).status(&project),
                maintain_rate_limit: None,
                _subscription: cx.observe(&semantic_index, Self::semantic_index_changed),
            });
            self.semantic_index_changed(semantic_index, cx);
        }
    }

    fn semantic_index_changed(
        &mut self,
        semantic_index: ModelHandle<SemanticIndex>,
        cx: &mut ViewContext<Self>,
    ) {
        let project = self.model.read(cx).project.clone();
        if let Some(semantic_state) = self.semantic_state.as_mut() {
            cx.notify();
            semantic_state.index_status = semantic_index.read(cx).status(&project);
            if let SemanticIndexStatus::Indexing {
                rate_limit_expiry: Some(_),
                ..
            } = &semantic_state.index_status
            {
                if semantic_state.maintain_rate_limit.is_none() {
                    semantic_state.maintain_rate_limit =
                        Some(cx.spawn(|this, mut cx| async move {
                            loop {
                                cx.background().timer(Duration::from_secs(1)).await;
                                this.update(&mut cx, |_, cx| cx.notify()).log_err();
                            }
                        }));
                    return;
                }
            } else {
                semantic_state.maintain_rate_limit = None;
            }
        }
    }

    fn clear_search(&mut self, cx: &mut ViewContext<Self>) {
        self.model.update(cx, |model, cx| {
            model.pending_search = None;
            model.no_results = None;
            model.match_ranges.clear();

            model.excerpts.update(cx, |excerpts, cx| {
                excerpts.clear(cx);
            });
        });
    }

    fn activate_search_mode(&mut self, mode: SearchMode, cx: &mut ViewContext<Self>) {
        let previous_mode = self.current_mode;
        if previous_mode == mode {
            return;
        }

        self.clear_search(cx);
        self.current_mode = mode;
        self.active_match_index = None;

        match mode {
            SearchMode::Semantic => {
                let has_permission = self.semantic_permissioned(cx);
                self.active_match_index = None;
                cx.spawn(|this, mut cx| async move {
                    let has_permission = has_permission.await?;

                    if !has_permission {
                        let mut answer = this.update(&mut cx, |this, cx| {
                            let project = this.model.read(cx).project.clone();
                            let project_name = project
                                .read(cx)
                                .worktree_root_names(cx)
                                .collect::<Vec<&str>>()
                                .join("/");
                            let is_plural =
                                project_name.chars().filter(|letter| *letter == '/').count() > 0;
                            let prompt_text = format!("Would you like to index the '{}' project{} for semantic search? This requires sending code to the OpenAI API", project_name,
                                if is_plural {
                                    "s"
                                } else {""});
                            cx.prompt(
                                PromptLevel::Info,
                                prompt_text.as_str(),
                                &["Continue", "Cancel"],
                            )
                        })?;

                        if answer.next().await == Some(0) {
                            this.update(&mut cx, |this, _| {
                                this.semantic_permissioned = Some(true);
                            })?;
                        } else {
                            this.update(&mut cx, |this, cx| {
                                this.semantic_permissioned = Some(false);
                                debug_assert_ne!(previous_mode, SearchMode::Semantic, "Tried to re-enable semantic search mode after user modal was rejected");
                                this.activate_search_mode(previous_mode, cx);
                            })?;
                            return anyhow::Ok(());
                        }
                    }

                    this.update(&mut cx, |this, cx| {
                        this.index_project(cx);
                    })?;

                    anyhow::Ok(())
                }).detach_and_log_err(cx);
            }
            SearchMode::Regex | SearchMode::Text => {
                self.semantic_state = None;
                self.active_match_index = None;
                self.search(cx);
            }
        }

        cx.update_global(|state: &mut ActiveSettings, cx| {
            state.0.insert(
                self.model.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });

        cx.notify();
    }
    fn replace_next(&mut self, _: &ReplaceNext, cx: &mut ViewContext<Self>) {
        let model = self.model.read(cx);
        if let Some(query) = model.active_query.as_ref() {
            if model.match_ranges.is_empty() {
                return;
            }
            if let Some(active_index) = self.active_match_index {
                let query = query.clone().with_replacement(self.replacement(cx));
                self.results_editor.replace(
                    &(Box::new(model.match_ranges[active_index].clone()) as _),
                    &query,
                    cx,
                );
                self.select_match(Direction::Next, cx)
            }
        }
    }
    pub fn replacement(&self, cx: &AppContext) -> String {
        self.replacement_editor.read(cx).text(cx)
    }
    fn replace_all(&mut self, _: &ReplaceAll, cx: &mut ViewContext<Self>) {
        let model = self.model.read(cx);
        if let Some(query) = model.active_query.as_ref() {
            if model.match_ranges.is_empty() {
                return;
            }
            if self.active_match_index.is_some() {
                let query = query.clone().with_replacement(self.replacement(cx));
                let matches = model
                    .match_ranges
                    .iter()
                    .map(|item| Box::new(item.clone()) as _)
                    .collect::<Vec<_>>();
                for item in matches {
                    self.results_editor.replace(&item, &query, cx);
                }
            }
        }
    }

    fn new(
        model: ModelHandle<ProjectSearch>,
        cx: &mut ViewContext<Self>,
        settings: Option<ProjectSearchSettings>,
    ) -> Self {
        let project;
        let excerpts;
        let mut replacement_text = None;
        let mut query_text = String::new();

        // Read in settings if available
        let (mut options, current_mode, filters_enabled) = if let Some(settings) = settings {
            (
                settings.search_options,
                settings.current_mode,
                settings.filters_enabled,
            )
        } else {
            (SearchOptions::NONE, Default::default(), false)
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
        cx.observe(&model, |this, _, cx| this.model_changed(cx))
            .detach();

        let query_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(Arc::new(|theme| theme.search.editor.input.clone())),
                cx,
            );
            editor.set_placeholder_text("Text search all files", cx);
            editor.set_text(query_text, cx);
            editor
        });
        // Subscribe to query_editor in order to reraise editor events for workspace item activation purposes
        cx.subscribe(&query_editor, |_, _, event, cx| {
            cx.emit(ViewEvent::EditorEvent(event.clone()))
        })
        .detach();
        let replacement_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(Arc::new(|theme| theme.search.editor.input.clone())),
                cx,
            );
            editor.set_placeholder_text("Replace in project..", cx);
            if let Some(text) = replacement_text {
                editor.set_text(text, cx);
            }
            editor
        });
        let results_editor = cx.add_view(|cx| {
            let mut editor = Editor::for_multibuffer(excerpts, Some(project.clone()), cx);
            editor.set_searchable(false);
            editor
        });
        cx.observe(&results_editor, |_, _, cx| cx.emit(ViewEvent::UpdateTab))
            .detach();

        cx.subscribe(&results_editor, |this, _, event, cx| {
            if matches!(event, editor::Event::SelectionsChanged { .. }) {
                this.update_match_index(cx);
            }
            // Reraise editor events for workspace item activation purposes
            cx.emit(ViewEvent::EditorEvent(event.clone()));
        })
        .detach();

        let included_files_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(Arc::new(|theme| {
                    theme.search.include_exclude_editor.input.clone()
                })),
                cx,
            );
            editor.set_placeholder_text("Include: crates/**/*.toml", cx);

            editor
        });
        // Subscribe to include_files_editor in order to reraise editor events for workspace item activation purposes
        cx.subscribe(&included_files_editor, |_, _, event, cx| {
            cx.emit(ViewEvent::EditorEvent(event.clone()))
        })
        .detach();

        let excluded_files_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(Arc::new(|theme| {
                    theme.search.include_exclude_editor.input.clone()
                })),
                cx,
            );
            editor.set_placeholder_text("Exclude: vendor/*, *.lock", cx);

            editor
        });
        // Subscribe to excluded_files_editor in order to reraise editor events for workspace item activation purposes
        cx.subscribe(&excluded_files_editor, |_, _, event, cx| {
            cx.emit(ViewEvent::EditorEvent(event.clone()))
        })
        .detach();

        // Check if Worktrees have all been previously indexed
        let mut this = ProjectSearchView {
            replacement_editor,
            search_id: model.read(cx).search_id,
            model,
            query_editor,
            results_editor,
            semantic_state: None,
            semantic_permissioned: None,
            search_options: options,
            panels_with_errors: HashSet::new(),
            active_match_index: None,
            query_editor_was_focused: false,
            included_files_editor,
            excluded_files_editor,
            filters_enabled,
            current_mode,
            replace_enabled: false,
        };
        this.model_changed(cx);
        this
    }

    fn semantic_permissioned(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<bool>> {
        if let Some(value) = self.semantic_permissioned {
            return Task::ready(Ok(value));
        }

        SemanticIndex::global(cx)
            .map(|semantic| {
                let project = self.model.read(cx).project.clone();
                semantic.update(cx, |this, cx| this.project_previously_indexed(&project, cx))
            })
            .unwrap_or(Task::ready(Ok(false)))
    }
    pub fn new_search_in_directory(
        workspace: &mut Workspace,
        dir_entry: &Entry,
        cx: &mut ViewContext<Workspace>,
    ) {
        if !dir_entry.is_dir() {
            return;
        }
        let Some(filter_str) = dir_entry.path.to_str() else {
            return;
        };

        let model = cx.add_model(|cx| ProjectSearch::new(workspace.project().clone(), cx));
        let search = cx.add_view(|cx| ProjectSearchView::new(model, cx, None));
        workspace.add_item(Box::new(search.clone()), cx);
        search.update(cx, |search, cx| {
            search
                .included_files_editor
                .update(cx, |editor, cx| editor.set_text(filter_str, cx));
            search.filters_enabled = true;
            search.focus_query_editor(cx)
        });
    }

    // Re-activate the most recently activated search or the most recent if it has been closed.
    // If no search exists in the workspace, create a new one.
    fn deploy(
        workspace: &mut Workspace,
        _: &workspace::NewSearch,
        cx: &mut ViewContext<Workspace>,
    ) {
        // Clean up entries for dropped projects
        cx.update_global(|state: &mut ActiveSearches, cx| {
            state.0.retain(|project, _| project.is_upgradable(cx))
        });

        let active_search = cx
            .global::<ActiveSearches>()
            .0
            .get(&workspace.project().downgrade());

        let existing = active_search
            .and_then(|active_search| {
                workspace
                    .items_of_type::<ProjectSearchView>(cx)
                    .find(|search| search == active_search)
            })
            .or_else(|| workspace.item_of_type::<ProjectSearchView>(cx));

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

            let model = cx.add_model(|cx| ProjectSearch::new(workspace.project().clone(), cx));
            let view = cx.add_view(|cx| ProjectSearchView::new(model, cx, settings));

            workspace.add_item(Box::new(view.clone()), cx);
            view
        };

        search.update(cx, |search, cx| {
            if let Some(query) = query {
                search.set_query(&query, cx);
            }
            search.focus_query_editor(cx)
        });
    }

    fn search(&mut self, cx: &mut ViewContext<Self>) {
        let mode = self.current_mode;
        match mode {
            SearchMode::Semantic => {
                if self.semantic_state.is_some() {
                    if let Some(query) = self.build_search_query(cx) {
                        self.model
                            .update(cx, |model, cx| model.semantic_search(query.as_inner(), cx));
                    }
                }
            }

            _ => {
                if let Some(query) = self.build_search_query(cx) {
                    self.model.update(cx, |model, cx| model.search(query, cx));
                }
            }
        }
    }

    fn build_search_query(&mut self, cx: &mut ViewContext<Self>) -> Option<SearchQuery> {
        let text = self.query_editor.read(cx).text(cx);
        let included_files =
            match Self::parse_path_matches(&self.included_files_editor.read(cx).text(cx)) {
                Ok(included_files) => {
                    self.panels_with_errors.remove(&InputPanel::Include);
                    included_files
                }
                Err(_e) => {
                    self.panels_with_errors.insert(InputPanel::Include);
                    cx.notify();
                    return None;
                }
            };
        let excluded_files =
            match Self::parse_path_matches(&self.excluded_files_editor.read(cx).text(cx)) {
                Ok(excluded_files) => {
                    self.panels_with_errors.remove(&InputPanel::Exclude);
                    excluded_files
                }
                Err(_e) => {
                    self.panels_with_errors.insert(InputPanel::Exclude);
                    cx.notify();
                    return None;
                }
            };
        let current_mode = self.current_mode;
        match current_mode {
            SearchMode::Regex => {
                match SearchQuery::regex(
                    text,
                    self.search_options.contains(SearchOptions::WHOLE_WORD),
                    self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                    included_files,
                    excluded_files,
                ) {
                    Ok(query) => {
                        self.panels_with_errors.remove(&InputPanel::Query);
                        Some(query)
                    }
                    Err(_e) => {
                        self.panels_with_errors.insert(InputPanel::Query);
                        cx.notify();
                        None
                    }
                }
            }
            _ => match SearchQuery::text(
                text,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                included_files,
                excluded_files,
            ) {
                Ok(query) => {
                    self.panels_with_errors.remove(&InputPanel::Query);
                    Some(query)
                }
                Err(_e) => {
                    self.panels_with_errors.insert(InputPanel::Query);
                    cx.notify();
                    None
                }
            },
        }
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
        cx.focus(&self.query_editor);
    }

    fn set_query(&mut self, query: &str, cx: &mut ViewContext<Self>) {
        self.query_editor
            .update(cx, |query_editor, cx| query_editor.set_text(query, cx));
    }

    fn focus_results_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            let cursor = query_editor.selections.newest_anchor().head();
            query_editor.change_selections(None, cx, |s| s.select_ranges([cursor.clone()..cursor]));
        });
        self.query_editor_was_focused = false;
        cx.focus(&self.results_editor);
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
                        .clone()
                        .map(|range| editor.range_for_match(range));
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select_ranges(range_to_select)
                    });
                }
                editor.highlight_background::<Self>(
                    match_ranges,
                    |theme| theme.search.match_background,
                    cx,
                );
            });
            if is_new_search && self.query_editor.is_focused(cx) {
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

    fn move_focus_to_results(pane: &mut Pane, _: &ToggleFocus, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |search_view, cx| {
                if !search_view.results_editor.is_focused(cx)
                    && !search_view.model.read(cx).match_ranges.is_empty()
                {
                    return search_view.focus_results_editor(cx);
                }
            });
        }

        cx.propagate_action();
    }
}

impl Default for ProjectSearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectSearchBar {
    pub fn new() -> Self {
        Self {
            active_project_search: Default::default(),
            subscription: Default::default(),
        }
    }
    fn cycle_mode(workspace: &mut Workspace, _: &CycleMode, cx: &mut ViewContext<Workspace>) {
        if let Some(search_view) = workspace
            .active_item(cx)
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |this, cx| {
                let new_mode =
                    crate::mode::next_mode(&this.current_mode, SemanticIndex::enabled(cx));
                this.activate_search_mode(new_mode, cx);
                cx.focus(&this.query_editor);
            })
        }
    }
    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        let mut should_propagate = true;
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if !search_view.replacement_editor.is_focused(cx) {
                    should_propagate = false;
                    search_view.search(cx);
                }
            });
        }
        if should_propagate {
            cx.propagate_action();
        }
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
                let model = cx.add_model(|cx| {
                    let mut model = ProjectSearch::new(workspace.project().clone(), cx);
                    model.search(new_query, cx);
                    model
                });
                workspace.add_item(
                    Box::new(cx.add_view(|cx| ProjectSearchView::new(model, cx, None))),
                    cx,
                );
            }
        }
    }

    fn select_next_match(pane: &mut Pane, _: &SelectNextMatch, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |view, cx| view.select_match(Direction::Next, cx));
        } else {
            cx.propagate_action();
        }
    }

    fn replace_next(pane: &mut Pane, _: &ReplaceNext, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |view, cx| view.replace_next(&ReplaceNext, cx));
        } else {
            cx.propagate_action();
        }
    }
    fn replace_all(pane: &mut Pane, _: &ReplaceAll, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |view, cx| view.replace_all(&ReplaceAll, cx));
        } else {
            cx.propagate_action();
        }
    }
    fn select_prev_match(pane: &mut Pane, _: &SelectPrevMatch, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |view, cx| view.select_match(Direction::Prev, cx));
        } else {
            cx.propagate_action();
        }
    }

    fn tab(&mut self, _: &editor::Tab, cx: &mut ViewContext<Self>) {
        self.cycle_field(Direction::Next, cx);
    }

    fn tab_previous(&mut self, _: &editor::TabPrev, cx: &mut ViewContext<Self>) {
        self.cycle_field(Direction::Prev, cx);
    }

    fn cycle_field(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        let active_project_search = match &self.active_project_search {
            Some(active_project_search) => active_project_search,

            None => {
                cx.propagate_action();
                return;
            }
        };

        active_project_search.update(cx, |project_view, cx| {
            let mut views = vec![&project_view.query_editor];
            if project_view.filters_enabled {
                views.extend([
                    &project_view.included_files_editor,
                    &project_view.excluded_files_editor,
                ]);
            }
            if project_view.replace_enabled {
                views.push(&project_view.replacement_editor);
            }
            let current_index = match views
                .iter()
                .enumerate()
                .find(|(_, view)| view.is_focused(cx))
            {
                Some((index, _)) => index,

                None => {
                    cx.propagate_action();
                    return;
                }
            };

            let new_index = match direction {
                Direction::Next => (current_index + 1) % views.len(),
                Direction::Prev if current_index == 0 => views.len() - 1,
                Direction::Prev => (current_index - 1) % views.len(),
            };
            cx.focus(views[new_index]);
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
                if !this.replace_enabled {
                    cx.focus(&this.query_editor);
                }
                cx.notify();
            });
        }
    }
    fn toggle_replace_on_a_pane(pane: &mut Pane, _: &ToggleReplace, cx: &mut ViewContext<Pane>) {
        let mut should_propagate = true;
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |this, cx| {
                should_propagate = false;
                this.replace_enabled = !this.replace_enabled;
                if !this.replace_enabled {
                    cx.focus(&this.query_editor);
                }
                cx.notify();
            });
        }
        if should_propagate {
            cx.propagate_action();
        }
    }
    fn activate_text_mode(pane: &mut Pane, _: &ActivateTextMode, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |view, cx| {
                view.activate_search_mode(SearchMode::Text, cx)
            });
        } else {
            cx.propagate_action();
        }
    }

    fn activate_regex_mode(pane: &mut Pane, _: &ActivateRegexMode, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |view, cx| {
                view.activate_search_mode(SearchMode::Regex, cx)
            });
        } else {
            cx.propagate_action();
        }
    }

    fn activate_semantic_mode(
        pane: &mut Pane,
        _: &ActivateSemanticMode,
        cx: &mut ViewContext<Pane>,
    ) {
        if SemanticIndex::enabled(cx) {
            if let Some(search_view) = pane
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
            {
                search_view.update(cx, |view, cx| {
                    view.activate_search_mode(SearchMode::Semantic, cx)
                });
            } else {
                cx.propagate_action();
            }
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
                cx.refresh_windows();
                cx.notify();
            });
            cx.notify();
            true
        } else {
            false
        }
    }

    fn activate_search_mode(&self, mode: SearchMode, cx: &mut ViewContext<Self>) {
        // Update Current Mode
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.activate_search_mode(mode, cx);
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
                let new_query = search_view.model.update(cx, |model, _| {
                    if let Some(new_query) = model.search_history.next().map(str::to_string) {
                        new_query
                    } else {
                        model.search_history.reset_selection();
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
                        .search_history
                        .current()
                        .map(str::to_string)
                    {
                        search_view.set_query(&new_query, cx);
                        return;
                    }
                }

                if let Some(new_query) = search_view.model.update(cx, |model, _| {
                    model.search_history.previous().map(str::to_string)
                }) {
                    search_view.set_query(&new_query, cx);
                }
            });
        }
    }
}

impl Entity for ProjectSearchBar {
    type Event = ();
}

impl View for ProjectSearchBar {
    fn ui_name() -> &'static str {
        "ProjectSearchBar"
    }

    fn update_keymap_context(
        &self,
        keymap: &mut gpui::keymap_matcher::KeymapContext,
        cx: &AppContext,
    ) {
        Self::reset_to_default_keymap_context(keymap);
        let in_replace = self
            .active_project_search
            .as_ref()
            .map(|search| {
                search
                    .read(cx)
                    .replacement_editor
                    .read_with(cx, |_, cx| cx.is_self_focused())
            })
            .flatten()
            .unwrap_or(false);
        if in_replace {
            keymap.add_identifier("in_replace");
        }
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(_search) = self.active_project_search.as_ref() {
            let search = _search.read(cx);
            let theme = theme::current(cx).clone();
            let query_container_style = if search.panels_with_errors.contains(&InputPanel::Query) {
                theme.search.invalid_editor
            } else {
                theme.search.editor.input.container
            };

            let search = _search.read(cx);
            let filter_button = render_option_button_icon(
                search.filters_enabled,
                "icons/filter.svg",
                0,
                "Toggle filters",
                Box::new(ToggleFilters),
                move |_, this, cx| {
                    this.toggle_filters(cx);
                },
                cx,
            );

            let search = _search.read(cx);
            let is_semantic_available = SemanticIndex::enabled(cx);
            let is_semantic_disabled = search.semantic_state.is_none();
            let icon_style = theme.search.editor_icon.clone();
            let is_active = search.active_match_index.is_some();

            let render_option_button_icon = |path, option, cx: &mut ViewContext<Self>| {
                crate::search_bar::render_option_button_icon(
                    self.is_option_enabled(option, cx),
                    path,
                    option.bits as usize,
                    format!("Toggle {}", option.label()),
                    option.to_toggle_action(),
                    move |_, this, cx| {
                        this.toggle_search_option(option, cx);
                    },
                    cx,
                )
            };
            let case_sensitive = is_semantic_disabled.then(|| {
                render_option_button_icon(
                    "icons/case_insensitive.svg",
                    SearchOptions::CASE_SENSITIVE,
                    cx,
                )
            });

            let whole_word = is_semantic_disabled.then(|| {
                render_option_button_icon("icons/word_search.svg", SearchOptions::WHOLE_WORD, cx)
            });

            let search_button_for_mode = |mode, side, cx: &mut ViewContext<ProjectSearchBar>| {
                let is_active = if let Some(search) = self.active_project_search.as_ref() {
                    let search = search.read(cx);
                    search.current_mode == mode
                } else {
                    false
                };
                render_search_mode_button(
                    mode,
                    side,
                    is_active,
                    move |_, this, cx| {
                        this.activate_search_mode(mode, cx);
                    },
                    cx,
                )
            };

            let search = _search.read(cx);

            let include_container_style =
                if search.panels_with_errors.contains(&InputPanel::Include) {
                    theme.search.invalid_include_exclude_editor
                } else {
                    theme.search.include_exclude_editor.input.container
                };

            let exclude_container_style =
                if search.panels_with_errors.contains(&InputPanel::Exclude) {
                    theme.search.invalid_include_exclude_editor
                } else {
                    theme.search.include_exclude_editor.input.container
                };

            let matches = search.active_match_index.map(|match_ix| {
                Label::new(
                    format!(
                        "{}/{}",
                        match_ix + 1,
                        search.model.read(cx).match_ranges.len()
                    ),
                    theme.search.match_index.text.clone(),
                )
                .contained()
                .with_style(theme.search.match_index.container)
                .aligned()
            });
            let should_show_replace_input = search.replace_enabled;
            let replacement = should_show_replace_input.then(|| {
                Flex::row()
                    .with_child(
                        Svg::for_style(theme.search.replace_icon.clone().icon)
                            .contained()
                            .with_style(theme.search.replace_icon.clone().container),
                    )
                    .with_child(ChildView::new(&search.replacement_editor, cx).flex(1., true))
                    .align_children_center()
                    .flex(1., true)
                    .contained()
                    .with_style(query_container_style)
                    .constrained()
                    .with_min_width(theme.search.editor.min_width)
                    .with_max_width(theme.search.editor.max_width)
                    .with_height(theme.search.search_bar_row_height)
                    .flex(1., false)
            });
            let replace_all = should_show_replace_input.then(|| {
                super::replace_action(
                    ReplaceAll,
                    "Replace all",
                    "icons/replace_all.svg",
                    theme.tooltip.clone(),
                    theme.search.action_button.clone(),
                )
            });
            let replace_next = should_show_replace_input.then(|| {
                super::replace_action(
                    ReplaceNext,
                    "Replace next",
                    "icons/replace_next.svg",
                    theme.tooltip.clone(),
                    theme.search.action_button.clone(),
                )
            });
            let query_column = Flex::column()
                .with_spacing(theme.search.search_row_spacing)
                .with_child(
                    Flex::row()
                        .with_child(
                            Svg::for_style(icon_style.icon)
                                .contained()
                                .with_style(icon_style.container),
                        )
                        .with_child(ChildView::new(&search.query_editor, cx).flex(1., true))
                        .with_child(
                            Flex::row()
                                .with_child(filter_button)
                                .with_children(case_sensitive)
                                .with_children(whole_word)
                                .flex(1., false)
                                .constrained()
                                .contained(),
                        )
                        .align_children_center()
                        .contained()
                        .with_style(query_container_style)
                        .constrained()
                        .with_min_width(theme.search.editor.min_width)
                        .with_max_width(theme.search.editor.max_width)
                        .with_height(theme.search.search_bar_row_height)
                        .flex(1., false),
                )
                .with_children(search.filters_enabled.then(|| {
                    Flex::row()
                        .with_child(
                            ChildView::new(&search.included_files_editor, cx)
                                .contained()
                                .with_style(include_container_style)
                                .constrained()
                                .with_height(theme.search.search_bar_row_height)
                                .flex(1., true),
                        )
                        .with_child(
                            ChildView::new(&search.excluded_files_editor, cx)
                                .contained()
                                .with_style(exclude_container_style)
                                .constrained()
                                .with_height(theme.search.search_bar_row_height)
                                .flex(1., true),
                        )
                        .constrained()
                        .with_min_width(theme.search.editor.min_width)
                        .with_max_width(theme.search.editor.max_width)
                        .flex(1., false)
                }))
                .flex(1., false);
            let switches_column = Flex::row()
                .align_children_center()
                .with_child(super::toggle_replace_button(
                    search.replace_enabled,
                    theme.tooltip.clone(),
                    theme.search.option_button_component.clone(),
                ))
                .constrained()
                .with_height(theme.search.search_bar_row_height)
                .contained()
                .with_style(theme.search.option_button_group);
            let mode_column =
                Flex::row()
                    .with_child(search_button_for_mode(
                        SearchMode::Text,
                        Some(Side::Left),
                        cx,
                    ))
                    .with_child(search_button_for_mode(
                        SearchMode::Regex,
                        if is_semantic_available {
                            None
                        } else {
                            Some(Side::Right)
                        },
                        cx,
                    ))
                    .with_children(is_semantic_available.then(|| {
                        search_button_for_mode(SearchMode::Semantic, Some(Side::Right), cx)
                    }))
                    .contained()
                    .with_style(theme.search.modes_container);

            let nav_button_for_direction = |label, direction, cx: &mut ViewContext<Self>| {
                render_nav_button(
                    label,
                    direction,
                    is_active,
                    move |_, this, cx| {
                        if let Some(search) = this.active_project_search.as_ref() {
                            search.update(cx, |search, cx| search.select_match(direction, cx));
                        }
                    },
                    cx,
                )
            };

            let nav_column = Flex::row()
                .with_children(replace_next)
                .with_children(replace_all)
                .with_child(Flex::row().with_children(matches))
                .with_child(nav_button_for_direction("<", Direction::Prev, cx))
                .with_child(nav_button_for_direction(">", Direction::Next, cx))
                .constrained()
                .with_height(theme.search.search_bar_row_height)
                .flex_float();

            Flex::row()
                .with_child(query_column)
                .with_child(mode_column)
                .with_child(switches_column)
                .with_children(replacement)
                .with_child(nav_column)
                .contained()
                .with_style(theme.search.container)
                .into_any_named("project search")
        } else {
            Empty::new().into_any()
        }
    }
}

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
            search.update(cx, |search, cx| {
                if search.current_mode == SearchMode::Semantic {
                    search.index_project(cx);
                }
            });

            self.subscription = Some(cx.observe(&search, |_, _, cx| cx.notify()));
            self.active_project_search = Some(search);
            ToolbarItemLocation::PrimaryLeft {
                flex: Some((1., true)),
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn row_count(&self, cx: &ViewContext<Self>) -> usize {
        if let Some(search) = self.active_project_search.as_ref() {
            if search.read(cx).filters_enabled {
                return 2;
            }
        }
        1
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use editor::DisplayPoint;
    use gpui::{color::Color, executor::Deterministic, TestAppContext};
    use project::FakeFs;
    use semantic_index::semantic_index_settings::SemanticIndexSettings;
    use serde_json::json;
    use settings::SettingsStore;
    use std::sync::Arc;
    use theme::ThemeSettings;

    #[gpui::test]
    async fn test_project_search(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
        let search = cx.add_model(|cx| ProjectSearch::new(project, cx));
        let search_view = cx
            .add_window(|cx| ProjectSearchView::new(search.clone(), cx, None))
            .root(cx);

        search_view.update(cx, |search_view, cx| {
            search_view
                .query_editor
                .update(cx, |query_editor, cx| query_editor.set_text("TWO", cx));
            search_view.search(cx);
        });
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;"
            );
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.all_text_background_highlights(cx)),
                &[
                    (
                        DisplayPoint::new(2, 32)..DisplayPoint::new(2, 35),
                        Color::red()
                    ),
                    (
                        DisplayPoint::new(2, 37)..DisplayPoint::new(2, 40),
                        Color::red()
                    ),
                    (
                        DisplayPoint::new(5, 6)..DisplayPoint::new(5, 9),
                        Color::red()
                    )
                ]
            );
            assert_eq!(search_view.active_match_index, Some(0));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(2, 32)..DisplayPoint::new(2, 35)]
            );

            search_view.select_match(Direction::Next, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(1));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(2, 37)..DisplayPoint::new(2, 40)]
            );
            search_view.select_match(Direction::Next, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(2));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(5, 6)..DisplayPoint::new(5, 9)]
            );
            search_view.select_match(Direction::Next, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(0));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(2, 32)..DisplayPoint::new(2, 35)]
            );
            search_view.select_match(Direction::Prev, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(2));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(5, 6)..DisplayPoint::new(5, 9)]
            );
            search_view.select_match(Direction::Prev, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(1));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(2, 37)..DisplayPoint::new(2, 40)]
            );
        });
    }

    #[gpui::test]
    async fn test_project_search_focus(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
        let workspace = window.root(cx);

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
            "Expected no search panel to be active, but got: {active_item:?}"
        );

        workspace.update(cx, |workspace, cx| {
            ProjectSearchView::deploy(workspace, &workspace::NewSearch, cx)
        });

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };
        let search_view_id = search_view.id();

        cx.spawn(|mut cx| async move {
            window.dispatch_action(search_view_id, &ToggleFocus, &mut cx);
        })
        .detach();
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert!(
                search_view.query_editor.is_focused(cx),
                "Empty search view should be focused after the toggle focus event: no results panel to focus on",
            );
        });

        search_view.update(cx, |search_view, cx| {
            let query_editor = &search_view.query_editor;
            assert!(
                query_editor.is_focused(cx),
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

        search_view.update(cx, |search_view, cx| {
            search_view.query_editor.update(cx, |query_editor, cx| {
                query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXIST", cx)
            });
            search_view.search(cx);
        });
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            let results_text = search_view
                .results_editor
                .update(cx, |editor, cx| editor.display_text(cx));
            assert!(
                results_text.is_empty(),
                "Search view for mismatching query should have no results but got '{results_text}'"
            );
            assert!(
                search_view.query_editor.is_focused(cx),
                "Search view should be focused after mismatching query had been used in search",
            );
        });
        cx.spawn(
            |mut cx| async move { window.dispatch_action(search_view_id, &ToggleFocus, &mut cx) },
        )
        .detach();
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert!(
                search_view.query_editor.is_focused(cx),
                "Search view with mismatching query should be focused after the toggle focus event: still no results panel to focus on",
            );
        });

        search_view.update(cx, |search_view, cx| {
            search_view
                .query_editor
                .update(cx, |query_editor, cx| query_editor.set_text("TWO", cx));
            search_view.search(cx);
        });
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                "Search view results should match the query"
            );
            assert!(
                search_view.results_editor.is_focused(cx),
                "Search view with mismatching query should be focused after search results are available",
            );
        });
        cx.spawn(|mut cx| async move {
            window.dispatch_action(search_view_id, &ToggleFocus, &mut cx);
        })
        .detach();
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert!(
                search_view.results_editor.is_focused(cx),
                "Search view with matching query should still have its results editor focused after the toggle focus event",
            );
        });

        workspace.update(cx, |workspace, cx| {
            ProjectSearchView::deploy(workspace, &workspace::NewSearch, cx)
        });
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
                search_view.query_editor.is_focused(cx),
                "Focus should be moved into query editor again after search view 2nd open in a row"
            );
        });

        cx.spawn(|mut cx| async move {
            window.dispatch_action(search_view_id, &ToggleFocus, &mut cx);
        })
        .detach();
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert!(
                search_view.results_editor.is_focused(cx),
                "Search view with matching query should switch focus to the results editor after the toggle focus event",
            );
        });
    }

    #[gpui::test]
    async fn test_new_project_search_in_directory(
        deterministic: Arc<Deterministic>,
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace = cx
            .add_window(|cx| Workspace::test_new(project, cx))
            .root(cx);

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
            "Expected no search panel to be active, but got: {active_item:?}"
        );

        let one_file_entry = cx.update(|cx| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .entry_for_path(&(worktree_id, "a/one.rs").into(), cx)
                .expect("no entry for /a/one.rs file")
        });
        assert!(one_file_entry.is_file());
        workspace.update(cx, |workspace, cx| {
            ProjectSearchView::new_search_in_directory(workspace, &one_file_entry, cx)
        });
        let active_search_entry = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        });
        assert!(
            active_search_entry.is_none(),
            "Expected no search panel to be active for file entry"
        );

        let a_dir_entry = cx.update(|cx| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .entry_for_path(&(worktree_id, "a").into(), cx)
                .expect("no entry for /a/ directory")
        });
        assert!(a_dir_entry.is_dir());
        workspace.update(cx, |workspace, cx| {
            ProjectSearchView::new_search_in_directory(workspace, &a_dir_entry, cx)
        });

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
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert!(
                search_view.query_editor.is_focused(cx),
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

        search_view.update(cx, |search_view, cx| {
            search_view
                .query_editor
                .update(cx, |query_editor, cx| query_editor.set_text("const", cx));
            search_view.search(cx);
        });
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\nconst ONE: usize = 1;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                "New search in directory should have a filter that matches a certain directory"
            );
        });
    }

    #[gpui::test]
    async fn test_search_query_history(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background());
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
        let workspace = window.root(cx);
        workspace.update(cx, |workspace, cx| {
            ProjectSearchView::deploy(workspace, &workspace::NewSearch, cx)
        });

        let search_view = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
                .expect("Search view expected to appear after new search event trigger")
        });

        let search_bar = window.add_view(cx, |cx| {
            let mut search_bar = ProjectSearchBar::new();
            search_bar.set_active_pane_item(Some(&search_view), cx);
            // search_bar.show(cx);
            search_bar
        });

        // Add 3 search items into the history + another unsubmitted one.
        search_view.update(cx, |search_view, cx| {
            search_view.search_options = SearchOptions::CASE_SENSITIVE;
            search_view
                .query_editor
                .update(cx, |query_editor, cx| query_editor.set_text("ONE", cx));
            search_view.search(cx);
        });
        cx.foreground().run_until_parked();
        search_view.update(cx, |search_view, cx| {
            search_view
                .query_editor
                .update(cx, |query_editor, cx| query_editor.set_text("TWO", cx));
            search_view.search(cx);
        });
        cx.foreground().run_until_parked();
        search_view.update(cx, |search_view, cx| {
            search_view
                .query_editor
                .update(cx, |query_editor, cx| query_editor.set_text("THREE", cx));
            search_view.search(cx);
        });
        cx.foreground().run_until_parked();
        search_view.update(cx, |search_view, cx| {
            search_view.query_editor.update(cx, |query_editor, cx| {
                query_editor.set_text("JUST_TEXT_INPUT", cx)
            });
        });
        cx.foreground().run_until_parked();

        // Ensure that the latest input with search settings is active.
        search_view.update(cx, |search_view, cx| {
            assert_eq!(
                search_view.query_editor.read(cx).text(cx),
                "JUST_TEXT_INPUT"
            );
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next history query after the latest should set the query to the empty string.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // First previous query for empty current query should set the query to the latest submitted one.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Further previous items should go over the history in reverse order.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Previous items should never go behind the first history item.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next items should go over the history in the original order.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });

        search_view.update(cx, |search_view, cx| {
            search_view
                .query_editor
                .update(cx, |query_editor, cx| query_editor.set_text("TWO_NEW", cx));
            search_view.search(cx);
        });
        cx.foreground().run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // New search input should add another entry to history and move the selection to the end of the history.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.query_editor.read(cx).text(cx), "");
            assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
        });
    }

    pub fn init_test(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        let fonts = cx.font_cache();
        let mut theme = gpui::fonts::with_font_cache(fonts.clone(), theme::Theme::default);
        theme.search.match_background = Color::red();

        cx.update(|cx| {
            cx.set_global(SettingsStore::test(cx));
            cx.set_global(ActiveSearches::default());
            settings::register::<SemanticIndexSettings>(cx);

            theme::init((), cx);
            cx.update_global::<SettingsStore, _, _>(|store, _| {
                let mut settings = store.get::<ThemeSettings>(None).clone();
                settings.theme = Arc::new(theme);
                store.override_global(settings)
            });

            language::init(cx);
            client::init_settings(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            super::init(cx);
        });
    }
}
