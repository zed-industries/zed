use crate::{
    SearchOption, SelectNextMatch, SelectPrevMatch, ToggleCaseSensitive, ToggleRegex,
    ToggleWholeWord,
};
use collections::HashMap;
use editor::{
    items::active_match_index, scroll::autoscroll::Autoscroll, Anchor, Editor, MultiBuffer,
    SelectAll, MAX_TAB_TITLE_LEN,
};
use futures::StreamExt;
use gpui::{
    actions,
    elements::*,
    platform::{CursorStyle, MouseButton},
    Action, AnyElement, AnyViewHandle, AppContext, Entity, ModelContext, ModelHandle, Subscription,
    Task, View, ViewContext, ViewHandle, WeakModelHandle, WeakViewHandle,
};
use menu::Confirm;
use project::{search::SearchQuery, Project};
use settings::Settings;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    mem,
    ops::Range,
    path::PathBuf,
    sync::Arc,
};
use util::ResultExt as _;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle},
    searchable::{Direction, SearchableItem, SearchableItemHandle},
    ItemNavHistory, Pane, ToolbarItemLocation, ToolbarItemView, Workspace, WorkspaceId,
};

actions!(project_search, [SearchInNew, ToggleFocus]);

#[derive(Default)]
struct ActiveSearches(HashMap<WeakModelHandle<Project>, WeakViewHandle<ProjectSearchView>>);

pub fn init(cx: &mut AppContext) {
    cx.set_global(ActiveSearches::default());
    cx.add_action(ProjectSearchView::deploy);
    cx.add_action(ProjectSearchBar::search);
    cx.add_action(ProjectSearchBar::search_in_new);
    cx.add_action(ProjectSearchBar::select_next_match);
    cx.add_action(ProjectSearchBar::select_prev_match);
    cx.add_action(ProjectSearchBar::toggle_focus);
    cx.capture_action(ProjectSearchBar::tab);
    add_toggle_option_action::<ToggleCaseSensitive>(SearchOption::CaseSensitive, cx);
    add_toggle_option_action::<ToggleWholeWord>(SearchOption::WholeWord, cx);
    add_toggle_option_action::<ToggleRegex>(SearchOption::Regex, cx);
}

fn add_toggle_option_action<A: Action>(option: SearchOption, cx: &mut AppContext) {
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
}

pub struct ProjectSearchView {
    model: ModelHandle<ProjectSearch>,
    query_editor: ViewHandle<Editor>,
    results_editor: ViewHandle<Editor>,
    case_sensitive: bool,
    whole_word: bool,
    regex: bool,
    query_contains_error: bool,
    active_match_index: Option<usize>,
    search_id: usize,
    query_editor_was_focused: bool,
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
        })
    }

    fn search(&mut self, query: SearchQuery, cx: &mut ModelContext<Self>) {
        let search = self
            .project
            .update(cx, |project, cx| project.search(query.clone(), cx));
        self.search_id += 1;
        self.active_query = Some(query);
        self.match_ranges.clear();
        self.pending_search = Some(cx.spawn_weak(|this, mut cx| async move {
            let matches = search.await.log_err()?;
            let this = this.upgrade(&cx)?;
            let mut matches = matches.into_iter().collect::<Vec<_>>();
            let (_task, mut match_ranges) = this.update(&mut cx, |this, cx| {
                this.match_ranges.clear();
                matches.sort_by_key(|(buffer, _)| buffer.read(cx).file().map(|file| file.path()));
                this.excerpts.update(cx, |excerpts, cx| {
                    excerpts.clear(cx);
                    excerpts.stream_excerpts_with_context_lines(matches, 1, cx)
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

            this.update(&mut cx, |this, cx| {
                this.pending_search.take();
                cx.notify();
            });

            None
        }));
        cx.notify();
    }
}

pub enum ViewEvent {
    UpdateTab,
    Activate,
    EditorEvent(editor::Event),
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

            let theme = cx.global::<Settings>().theme.clone();
            let text = if self.query_editor.read(cx).text(cx).is_empty() {
                ""
            } else if model.pending_search.is_some() {
                "Searching..."
            } else {
                "No results"
            };
            MouseEventHandler::<Status, _>::new(0, cx, |_, _| {
                Label::new(text, theme.search.results_status.clone())
                    .aligned()
                    .contained()
                    .with_background_color(theme.editor.background)
                    .flex(1., true)
            })
            .on_down(MouseButton::Left, |_, _, cx| {
                cx.focus_parent_view();
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
        Some(self.query_editor.read(cx).text(cx).into())
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

    fn tab_content<T: View>(
        &self,
        _detail: Option<usize>,
        tab_theme: &theme::Tab,
        cx: &AppContext,
    ) -> AnyElement<T> {
        Flex::row()
            .with_child(
                Svg::new("icons/magnifying_glass_12.svg")
                    .with_color(tab_theme.label.text.color)
                    .constrained()
                    .with_width(tab_theme.type_icon_width)
                    .aligned()
                    .contained()
                    .with_margin_right(tab_theme.spacing),
            )
            .with_children(self.model.read(cx).active_query.as_ref().map(|query| {
                let query_text = util::truncate_and_trailoff(query.as_str(), MAX_TAB_TITLE_LEN);

                Label::new(query_text, tab_theme.label.clone()).aligned()
            }))
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
        Some(Self::new(model, cx))
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

    fn git_diff_recalc(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.git_diff_recalc(project, cx))
    }

    fn to_item_events(event: &Self::Event) -> SmallVec<[ItemEvent; 2]> {
        match event {
            ViewEvent::UpdateTab => {
                smallvec::smallvec![ItemEvent::UpdateBreadcrumbs, ItemEvent::UpdateTab]
            }
            ViewEvent::EditorEvent(editor_event) => Editor::to_item_events(editor_event),
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
    fn new(model: ModelHandle<ProjectSearch>, cx: &mut ViewContext<Self>) -> Self {
        let project;
        let excerpts;
        let mut query_text = String::new();
        let mut regex = false;
        let mut case_sensitive = false;
        let mut whole_word = false;

        {
            let model = model.read(cx);
            project = model.project.clone();
            excerpts = model.excerpts.clone();
            if let Some(active_query) = model.active_query.as_ref() {
                query_text = active_query.as_str().to_string();
                regex = active_query.is_regex();
                case_sensitive = active_query.case_sensitive();
                whole_word = active_query.whole_word();
            }
        }
        cx.observe(&model, |this, _, cx| this.model_changed(cx))
            .detach();

        let query_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(Arc::new(|theme| theme.search.editor.input.clone())),
                cx,
            );
            editor.set_text(query_text, cx);
            editor
        });
        // Subcribe to query_editor in order to reraise editor events for workspace item activation purposes
        cx.subscribe(&query_editor, |_, _, event, cx| {
            cx.emit(ViewEvent::EditorEvent(event.clone()))
        })
        .detach();

        let results_editor = cx.add_view(|cx| {
            let mut editor = Editor::for_multibuffer(excerpts, Some(project), cx);
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

        let mut this = ProjectSearchView {
            search_id: model.read(cx).search_id,
            model,
            query_editor,
            results_editor,
            case_sensitive,
            whole_word,
            regex,
            query_contains_error: false,
            active_match_index: None,
            query_editor_was_focused: false,
        };
        this.model_changed(cx);
        this
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
            let model = cx.add_model(|cx| ProjectSearch::new(workspace.project().clone(), cx));
            let view = cx.add_view(|cx| ProjectSearchView::new(model, cx));
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
        if let Some(query) = self.build_search_query(cx) {
            self.model.update(cx, |model, cx| model.search(query, cx));
        }
    }

    fn build_search_query(&mut self, cx: &mut ViewContext<Self>) -> Option<SearchQuery> {
        let text = self.query_editor.read(cx).text(cx);
        if self.regex {
            match SearchQuery::regex(text, self.whole_word, self.case_sensitive) {
                Ok(query) => Some(query),
                Err(_) => {
                    self.query_contains_error = true;
                    cx.notify();
                    None
                }
            }
        } else {
            Some(SearchQuery::text(
                text,
                self.whole_word,
                self.case_sensitive,
            ))
        }
    }

    fn select_match(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.active_match_index {
            let match_ranges = self.model.read(cx).match_ranges.clone();
            let new_index = self.results_editor.update(cx, |editor, cx| {
                editor.match_index_for_direction(&match_ranges, index, direction, cx)
            });

            let range_to_select = match_ranges[new_index].clone();
            self.results_editor.update(cx, |editor, cx| {
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
            let prev_search_id = mem::replace(&mut self.search_id, self.model.read(cx).search_id);
            let is_new_search = self.search_id != prev_search_id;
            self.results_editor.update(cx, |editor, cx| {
                if is_new_search {
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select_ranges(match_ranges.first().cloned())
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

    fn search(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| search_view.search(cx));
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
                        search_view.regex = old_query.is_regex();
                        search_view.whole_word = old_query.whole_word();
                        search_view.case_sensitive = old_query.case_sensitive();
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
                    Box::new(cx.add_view(|cx| ProjectSearchView::new(model, cx))),
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

    fn toggle_focus(pane: &mut Pane, _: &ToggleFocus, cx: &mut ViewContext<Pane>) {
        if let Some(search_view) = pane
            .active_item()
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            search_view.update(cx, |search_view, cx| {
                if search_view.query_editor.is_focused(cx) {
                    if !search_view.model.read(cx).match_ranges.is_empty() {
                        search_view.focus_results_editor(cx);
                    }
                } else {
                    search_view.focus_query_editor(cx);
                }
            });
        } else {
            cx.propagate_action();
        }
    }

    fn tab(&mut self, _: &editor::Tab, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if search_view.query_editor.is_focused(cx) {
                    if !search_view.model.read(cx).match_ranges.is_empty() {
                        search_view.focus_results_editor(cx);
                    }
                } else {
                    cx.propagate_action();
                }
            });
        } else {
            cx.propagate_action();
        }
    }

    fn toggle_search_option(&mut self, option: SearchOption, cx: &mut ViewContext<Self>) -> bool {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                let value = match option {
                    SearchOption::WholeWord => &mut search_view.whole_word,
                    SearchOption::CaseSensitive => &mut search_view.case_sensitive,
                    SearchOption::Regex => &mut search_view.regex,
                };
                *value = !*value;
                search_view.search(cx);
            });
            cx.notify();
            true
        } else {
            false
        }
    }

    fn render_nav_button(
        &self,
        icon: &'static str,
        direction: Direction,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let action: Box<dyn Action>;
        let tooltip;
        match direction {
            Direction::Prev => {
                action = Box::new(SelectPrevMatch);
                tooltip = "Select Previous Match";
            }
            Direction::Next => {
                action = Box::new(SelectNextMatch);
                tooltip = "Select Next Match";
            }
        };
        let tooltip_style = cx.global::<Settings>().theme.tooltip.clone();

        enum NavButton {}
        MouseEventHandler::<NavButton, _>::new(direction as usize, cx, |state, cx| {
            let style = &cx
                .global::<Settings>()
                .theme
                .search
                .option_button
                .style_for(state, false);
            Label::new(icon, style.text.clone())
                .contained()
                .with_style(style.container)
        })
        .on_click(MouseButton::Left, move |_, this, cx| {
            if let Some(search) = this.active_project_search.as_ref() {
                search.update(cx, |search, cx| search.select_match(direction, cx));
            }
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .with_tooltip::<NavButton>(
            direction as usize,
            tooltip.to_string(),
            Some(action),
            tooltip_style,
            cx,
        )
        .into_any()
    }

    fn render_option_button(
        &self,
        icon: &'static str,
        option: SearchOption,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let tooltip_style = cx.global::<Settings>().theme.tooltip.clone();
        let is_active = self.is_option_enabled(option, cx);
        MouseEventHandler::<Self, _>::new(option as usize, cx, |state, cx| {
            let style = &cx
                .global::<Settings>()
                .theme
                .search
                .option_button
                .style_for(state, is_active);
            Label::new(icon, style.text.clone())
                .contained()
                .with_style(style.container)
        })
        .on_click(MouseButton::Left, move |_, this, cx| {
            this.toggle_search_option(option, cx);
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .with_tooltip::<Self>(
            option as usize,
            format!("Toggle {}", option.label()),
            Some(option.to_toggle_action()),
            tooltip_style,
            cx,
        )
        .into_any()
    }

    fn is_option_enabled(&self, option: SearchOption, cx: &AppContext) -> bool {
        if let Some(search) = self.active_project_search.as_ref() {
            let search = search.read(cx);
            match option {
                SearchOption::WholeWord => search.whole_word,
                SearchOption::CaseSensitive => search.case_sensitive,
                SearchOption::Regex => search.regex,
            }
        } else {
            false
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(search) = self.active_project_search.as_ref() {
            let search = search.read(cx);
            let theme = cx.global::<Settings>().theme.clone();
            let editor_container = if search.query_contains_error {
                theme.search.invalid_editor
            } else {
                theme.search.editor.input.container
            };
            Flex::row()
                .with_child(
                    Flex::row()
                        .with_child(
                            ChildView::new(&search.query_editor, cx)
                                .aligned()
                                .left()
                                .flex(1., true),
                        )
                        .with_children(search.active_match_index.map(|match_ix| {
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
                        }))
                        .contained()
                        .with_style(editor_container)
                        .aligned()
                        .constrained()
                        .with_min_width(theme.search.editor.min_width)
                        .with_max_width(theme.search.editor.max_width)
                        .flex(1., false),
                )
                .with_child(
                    Flex::row()
                        .with_child(self.render_nav_button("<", Direction::Prev, cx))
                        .with_child(self.render_nav_button(">", Direction::Next, cx))
                        .aligned(),
                )
                .with_child(
                    Flex::row()
                        .with_child(self.render_option_button(
                            "Case",
                            SearchOption::CaseSensitive,
                            cx,
                        ))
                        .with_child(self.render_option_button("Word", SearchOption::WholeWord, cx))
                        .with_child(self.render_option_button("Regex", SearchOption::Regex, cx))
                        .contained()
                        .with_style(theme.search.option_button_group)
                        .aligned(),
                )
                .contained()
                .with_style(theme.search.container)
                .aligned()
                .left()
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
            let query_editor = search.read(cx).query_editor.clone();
            cx.reparent(&query_editor);
            self.subscription = Some(cx.observe(&search, |_, _, cx| cx.notify()));
            self.active_project_search = Some(search);
            ToolbarItemLocation::PrimaryLeft {
                flex: Some((1., false)),
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::DisplayPoint;
    use gpui::{color::Color, executor::Deterministic, TestAppContext};
    use project::FakeFs;
    use serde_json::json;
    use std::sync::Arc;

    #[gpui::test]
    async fn test_project_search(deterministic: Arc<Deterministic>, cx: &mut TestAppContext) {
        let fonts = cx.font_cache();
        let mut theme = gpui::fonts::with_font_cache(fonts.clone(), theme::Theme::default);
        theme.search.match_background = Color::red();
        cx.update(|cx| {
            let mut settings = Settings::test(cx);
            settings.theme = Arc::new(theme);
            cx.set_global(settings);
            cx.set_global(ActiveSearches::default());
        });

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
        let (_, search_view) = cx.add_window(|cx| ProjectSearchView::new(search.clone(), cx));

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
                    .update(cx, |editor, cx| editor.all_background_highlights(cx)),
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
}
