use crate::{
    SearchOptions, SelectNextMatch, SelectPrevMatch, ToggleCaseSensitive, ToggleRegex,
    ToggleWholeWord,
};
use anyhow::Result;
use collections::HashMap;
use editor::{
    items::active_match_index, scroll::autoscroll::Autoscroll, Anchor, Editor, MultiBuffer,
    SelectAll, MAX_TAB_TITLE_LEN,
};
use futures::{future::Shared, FutureExt, StreamExt};
use globset::{Glob, GlobMatcher};
use gpui::{
    actions,
    elements::*,
    platform::{CursorStyle, MouseButton},
    Action, AnyElement, AnyViewHandle, AppContext, Element, Entity, ModelContext, ModelHandle,
    Subscription, Task, View, ViewContext, ViewHandle, WeakModelHandle, WeakViewHandle,
};
use menu::Confirm;
use project::{search::SearchQuery, Project};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    collections::HashSet,
    mem,
    ops::{Not, Range},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};
use util::ResultExt as _;
use workspace::{
    item::{BreadcrumbText, Item, ItemEvent, ItemHandle},
    searchable::{Direction, SearchableItem, SearchableItemHandle},
    ItemNavHistory, Pane, ToolbarItemLocation, ToolbarItemView, Workspace, WorkspaceId,
};

actions!(
    project_search,
    [
        SearchInNew,
        ToggleFocus,
        NextField,
        Replace,
        ReplaceAll,
        ToggleReplace,
        ToggleFilter,
        Undo
    ]
);

#[derive(Default)]
struct ActiveSearches(HashMap<WeakModelHandle<Project>, WeakViewHandle<ProjectSearchView>>);

pub fn init(cx: &mut AppContext) {
    cx.set_global(ActiveSearches::default());
    cx.add_action(ProjectSearchView::deploy);
    cx.add_action(ProjectSearchView::toggle_focus);
    cx.add_action(ProjectSearchView::toggle_filter);
    cx.add_action(ProjectSearchView::toggle_replace);
    cx.add_action(ProjectSearchView::replace_all_action);
    cx.add_action(ProjectSearchView::replace_action);
    cx.add_action(ProjectSearchView::undo_action);
    cx.add_action(ProjectSearchBar::toggle_focus);
    cx.add_action(ProjectSearchBar::search);
    cx.add_action(ProjectSearchBar::replace_all);
    cx.add_action(ProjectSearchBar::replace);
    cx.add_action(ProjectSearchBar::undo);
    cx.add_action(ProjectSearchBar::toggle_filter);
    cx.add_action(ProjectSearchBar::toggle_replace);
    cx.add_action(ProjectSearchBar::search_in_new);
    cx.add_action(ProjectSearchBar::select_next_match);
    cx.add_action(ProjectSearchBar::select_prev_match);
    cx.capture_action(ProjectSearchBar::tab);
    cx.capture_action(ProjectSearchBar::tab_previous);
    add_toggle_option_action::<ToggleCaseSensitive>(SearchOptions::CASE_SENSITIVE, cx);
    add_toggle_option_action::<ToggleWholeWord>(SearchOptions::WHOLE_WORD, cx);
    add_toggle_option_action::<ToggleRegex>(SearchOptions::REGEX, cx);
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
    pending_search: Option<Shared<Task<Option<()>>>>,
    match_ranges: Vec<Range<Anchor>>,
    out_of_date_ranges: Vec<Range<Anchor>>,
    active_query: Option<SearchQuery>,
    search_id: usize,
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
    results_editor: ViewHandle<Editor>,
    search_options: SearchOptions,
    show_filter: bool,
    show_replace: bool,
    panels_with_errors: HashSet<InputPanel>,
    // None -> No results
    // Some(None) Results, but no match index (e.g. after a replace all)
    // Some(Some(_)) Results, with an index
    active_match_index: Option<Option<usize>>,
    search_id: usize,
    query_editor_was_focused: bool,
    included_files_editor: ViewHandle<Editor>,
    excluded_files_editor: ViewHandle<Editor>,
    replace_editor: ViewHandle<Editor>,
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
            out_of_date_ranges: Default::default(),
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
            out_of_date_ranges: Default::default(),
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
        self.out_of_date_ranges.clear();
        self.pending_search = Some(
            cx.spawn_weak(|this, mut cx| async move {
                let matches = search.await.log_err()?;
                let this = this.upgrade(&cx)?;
                let mut matches = matches.into_iter().collect::<Vec<_>>();
                let (_task, mut match_ranges) = this.update(&mut cx, |this, cx| {
                    this.match_ranges.clear();
                    this.out_of_date_ranges.clear();

                    matches
                        .sort_by_key(|(buffer, _)| buffer.read(cx).file().map(|file| file.path()));
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
            })
            .shared(),
        );
        cx.notify();
    }

    fn replace_all(&mut self, replacement_text: Arc<str>, cx: &mut ModelContext<Self>) {
        self.excerpts.update(cx, |multibuffer, cx| {
            multibuffer.edit(
                self.match_ranges
                    .iter()
                    .map(|range| (range.clone(), replacement_text.to_string())),
                None,
                cx,
            )
        });

        self.out_of_date_ranges.extend(self.match_ranges.drain(..));
    }

    fn replace(
        &mut self,
        index: usize,
        replacement_text: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) -> Option<usize> {
        if self.match_ranges.len() == 0 {
            return None;
        }

        debug_assert!(index < self.match_ranges.len());

        self.excerpts.update(cx, |multibuffer, cx| {
            multibuffer.edit(
                [(
                    self.match_ranges[index].clone(),
                    replacement_text.to_string(),
                )],
                None,
                cx,
            );
        });

        self.out_of_date_ranges
            .push(self.match_ranges.remove(index));
        cx.notify();
        Some(self.match_ranges.len())
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
        if model.match_ranges.is_empty() && self.active_match_index.is_none() {
            enum Status {}

            let theme = theme::current(cx).clone();
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
        let mut options = SearchOptions::NONE;

        {
            let model = model.read(cx);
            project = model.project.clone();
            excerpts = model.excerpts.clone();
            if let Some(active_query) = model.active_query.as_ref() {
                query_text = active_query.as_str().to_string();
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
            editor.set_text(query_text, cx);
            editor
        });
        // Subscribe to query_editor in order to reraise editor events for workspace item activation purposes
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

        let replace_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(Arc::new(|theme| {
                    theme.search.include_exclude_editor.input.clone()
                })),
                cx,
            );
            editor.set_placeholder_text("Replace", cx);

            editor
        });
        // Subscribe to include_files_editor in order to reraise editor events for workspace item activation purposes
        cx.subscribe(&replace_editor, |_, _, event, cx| {
            cx.emit(ViewEvent::EditorEvent(event.clone()))
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

        let mut this = ProjectSearchView {
            search_id: model.read(cx).search_id,
            model,
            query_editor,
            results_editor,
            search_options: options,
            show_filter: false,
            show_replace: false,
            panels_with_errors: HashSet::new(),
            active_match_index: None,
            query_editor_was_focused: false,
            included_files_editor,
            excluded_files_editor,
            replace_editor,
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

    fn replace_action(&mut self, _: &Replace, cx: &mut ViewContext<Self>) {
        self.replace(cx);
    }

    fn replace(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(query) = self.build_search_query(cx) {
            if let Some(replace_text) = query.replace_text() {
                if let Some(Some(idx)) = self.active_match_index {
                    let len = self
                        .model
                        .update(cx, |model, cx| model.replace(idx, replace_text, cx));

                    if let Some(len) = len {
                        if idx >= len {
                            self.active_match_index = Some(Some(0));
                            self.select_index(0, cx)
                        } else {
                            self.select_index(idx, cx)
                        }
                    } else {
                        self.active_match_index = Some(None);
                    }
                }
            }
        }
    }

    fn replace_all_action(&mut self, _: &ReplaceAll, cx: &mut ViewContext<Self>) {
        self.replace_all(cx);
    }

    fn replace_all(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(query) = self.build_search_query(cx) {
            if let Some(replace_text) = query.replace_text() {
                self.model
                    .update(cx, |model, cx| model.replace_all(replace_text, cx));
                self.active_match_index = Some(None);
            }
        }
    }

    fn undo_action(&mut self, _: &Undo, cx: &mut ViewContext<Self>) {
        self.undo(cx);
    }

    fn undo(&mut self, cx: &mut ViewContext<Self>) {
        // self.replace_all(cx);
    }

    fn build_search_query(&mut self, cx: &mut ViewContext<Self>) -> Option<SearchQuery> {
        let text = self.query_editor.read(cx).text(cx);
        let replace = self
            .show_replace
            .then(|| self.replace_editor.read(cx).text(cx));
        let included_files =
            match Self::load_glob_set(&self.included_files_editor.read(cx).text(cx)) {
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
            match Self::load_glob_set(&self.excluded_files_editor.read(cx).text(cx)) {
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
        if self.search_options.contains(SearchOptions::REGEX) {
            match SearchQuery::regex(
                text,
                replace,
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
        } else {
            Some(SearchQuery::text(
                text,
                replace,
                self.search_options.contains(SearchOptions::WHOLE_WORD),
                self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                included_files,
                excluded_files,
            ))
        }
    }

    fn load_glob_set(text: &str) -> Result<Vec<GlobMatcher>> {
        text.split(',')
            .map(str::trim)
            .filter(|glob_str| !glob_str.is_empty())
            .map(|glob_str| anyhow::Ok(Glob::new(glob_str)?.compile_matcher()))
            .collect()
    }

    fn select_match(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        if let Some(Some(index)) = self.active_match_index {
            let match_ranges = self.model.read(cx).match_ranges.clone();
            let new_index = self.results_editor.update(cx, |editor, cx| {
                editor.match_index_for_direction(&match_ranges, index, direction, 1, cx)
            });

            self.select_index(new_index, cx)
        }
    }

    fn select_index(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        let Some(range_to_select) = self.model.read(cx).match_ranges.get(index).map(|range| range.clone()) else {
            return
        };
        self.results_editor.update(cx, |editor, cx| {
            editor.unfold_ranges([range_to_select.clone()], false, true, cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_ranges([range_to_select])
            });
        });
    }

    fn focus_editor(&mut self, handle: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        handle.update(cx, |editor, cx| {
            editor.select_all(&SelectAll, cx);
        });
        self.query_editor_was_focused = false;
        cx.focus(&handle);
    }

    fn focus_query_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.focus_editor(self.query_editor.clone(), cx)
    }

    fn focus_replace_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.show_replace = true;
        self.focus_editor(self.replace_editor.clone(), cx);
        cx.notify();
    }

    fn focus_filter_editor(&mut self, cx: &mut ViewContext<Self>) {
        self.show_filter = true;
        self.focus_editor(self.included_files_editor.clone(), cx);
        cx.notify();
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
        self.update_match_index(cx);
        if match_ranges.is_empty() {
            self.active_match_index = None;
        } else {
            self.active_match_index = Some(None);
            let prev_search_id = mem::replace(&mut self.search_id, self.model.read(cx).search_id);
            let is_new_search = self.search_id != prev_search_id;
            self.results_editor.update(cx, |editor, cx| {
                if is_new_search {
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                        s.select_ranges(match_ranges.first().cloned())
                    });
                }
            });
            if is_new_search && self.query_editor.is_focused(cx) {
                self.focus_results_editor(cx);
            }
        }
        self.results_editor.update(cx, |editor, cx| {
            editor.highlight_background::<ProjectSearch>(
                self.model.read(cx).out_of_date_ranges.clone(),
                |theme| theme.search.out_of_date_match_background,
                cx,
            );
            editor.highlight_background::<Self>(
                self.model.read(cx).match_ranges.clone(),
                |theme| theme.search.match_background,
                cx,
            );
        });

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
        if self.active_match_index != Some(new_index) {
            self.active_match_index = Some(new_index);
            cx.notify();
        }
    }

    pub fn has_matches(&self) -> bool {
        self.active_match_index.is_some()
    }

    fn toggle_focus(
        search_view: &mut ProjectSearchView,
        _: &ToggleFocus,
        cx: &mut ViewContext<ProjectSearchView>,
    ) {
        search_view.focus_query_editor(cx);
    }

    fn toggle_replace(
        search_view: &mut ProjectSearchView,
        _: &ToggleReplace,
        cx: &mut ViewContext<ProjectSearchView>,
    ) {
        search_view.focus_replace_editor(cx);
    }

    fn toggle_filter(
        search_view: &mut ProjectSearchView,
        _: &ToggleFilter,
        cx: &mut ViewContext<ProjectSearchView>,
    ) {
        return search_view.focus_filter_editor(cx);
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

    fn toggle_focus(
        search_bar: &mut ProjectSearchBar,
        _: &ToggleFocus,
        cx: &mut ViewContext<ProjectSearchBar>,
    ) {
        if let Some(view) = &search_bar.active_project_search {
            view.update(cx, |view, cx| view.focus_results_editor(cx));
        }
    }

    fn search(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if search_view.replace_editor.is_focused(cx) {
                    search_view.replace(cx)
                } else {
                    search_view.search(cx);
                }
            });
        }
    }

    fn replace(&mut self, _: &Replace, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| search_view.replace(cx));
        }
    }

    fn undo(&mut self, _: &Undo, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| search_view.undo(cx));
        }
    }

    fn replace_all(&mut self, _: &ReplaceAll, cx: &mut ViewContext<Self>) {
        // TODO
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| search_view.replace_all(cx));
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
            let views = &[
                &project_view.query_editor,
                &project_view.included_files_editor,
                &project_view.excluded_files_editor,
            ];

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
                search_view.search_options.toggle(option);
                search_view.search(cx);
            });
            cx.notify();
            true
        } else {
            false
        }
    }

    fn toggle_filter(&mut self, _: &ToggleFilter, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if search_view.show_filter && search_view.included_files_editor.is_focused(cx)
                    || search_view.excluded_files_editor.is_focused(cx)
                {
                    search_view.focus_query_editor(cx);
                }

                search_view.show_filter = !search_view.show_filter;

                if search_view.show_filter {
                    search_view.focus_filter_editor(cx);
                }
                cx.notify();
            });
            cx.notify();
        }
    }

    fn toggle_replace(&mut self, _: &ToggleReplace, cx: &mut ViewContext<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if search_view.show_replace && search_view.replace_editor.is_focused(cx) {
                    search_view.focus_query_editor(cx);
                }

                search_view.show_replace = !search_view.show_replace;

                if search_view.show_replace {
                    search_view.focus_replace_editor(cx);
                }
                cx.notify();
            });
            cx.notify();
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
        let tooltip_style = theme::current(cx).tooltip.clone();

        enum NavButton {}
        MouseEventHandler::<NavButton, _>::new(direction as usize, cx, |state, cx| {
            let theme = theme::current(cx);
            let style = theme.search.option_button.inactive_state().style_for(state);
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
        option: SearchOptions,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let tooltip_style = theme::current(cx).tooltip.clone();
        let is_active = self.is_option_enabled(option, cx);
        MouseEventHandler::<Self, _>::new(option.bits as usize, cx, |state, cx| {
            let theme = theme::current(cx);
            let style = theme
                .search
                .option_button
                .in_state(is_active)
                .style_for(state);
            Label::new(icon, style.text.clone())
                .contained()
                .with_style(style.container)
        })
        .on_click(MouseButton::Left, move |_, this, cx| {
            this.toggle_search_option(option, cx);
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .with_tooltip::<Self>(
            option.bits as usize,
            format!("Toggle {}", option.label()),
            Some(option.to_toggle_action()),
            tooltip_style,
            cx,
        )
        .into_any()
    }

    fn is_option_enabled(&self, option: SearchOptions, cx: &AppContext) -> bool {
        if let Some(search) = self.active_project_search.as_ref() {
            search.read(cx).search_options.contains(option)
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
            let theme = theme::current(cx).clone();
            let query_container_style = if search.panels_with_errors.contains(&InputPanel::Query) {
                theme.search.invalid_editor
            } else {
                theme.search.editor.input.container
            };
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

            let included_files_view = ChildView::new(&search.included_files_editor, cx)
                .aligned()
                .left()
                .flex(1.0, true);
            let excluded_files_view = ChildView::new(&search.excluded_files_editor, cx)
                .aligned()
                .right()
                .flex(1.0, true);
            let replace_view = ChildView::new(&search.replace_editor, cx)
                .aligned()
                .left()
                .flex(1.0, true);

            let row_spacing = theme.workspace.toolbar.container.padding.bottom;

            let (show_replace, show_filter) = self
                .active_project_search
                .as_ref()
                .map(|search_view| {
                    let search_view = search_view.read(cx);
                    (search_view.show_replace, search_view.show_filter)
                })
                .unwrap_or((false, false));

            Flex::column()
                .with_child(
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
                                            match_ix.map(|ix| ix + 1).unwrap_or(0),
                                            search.model.read(cx).match_ranges.len()
                                        ),
                                        theme.search.match_index.text.clone(),
                                    )
                                    .contained()
                                    .with_style(theme.search.match_index.container)
                                    .aligned()
                                }))
                                .contained()
                                .with_style(query_container_style)
                                .aligned()
                                .constrained()
                                .with_min_width(theme.search.editor.min_width)
                                .with_max_width(theme.search.editor.max_width)
                                .flex(1., false),
                        )
                        .with_child(
                            Flex::row()
                                .with_child(
                                    ToggleIconButton::new(0, show_replace, "Ⓡ")
                                        .with_action(|_| Box::new(ToggleReplace), "Toggle replace"),
                                )
                                .with_child(
                                    ToggleIconButton::new(1, show_filter, "Υ")
                                        .with_action(|_| Box::new(ToggleFilter), "Toggle filter"),
                                )
                                .contained()
                                .with_style(theme.search.option_button_group)
                                .aligned(),
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
                                    SearchOptions::CASE_SENSITIVE,
                                    cx,
                                ))
                                .with_child(self.render_option_button(
                                    "Word",
                                    SearchOptions::WHOLE_WORD,
                                    cx,
                                ))
                                .with_child(self.render_option_button(
                                    "Regex",
                                    SearchOptions::REGEX,
                                    cx,
                                ))
                                .contained()
                                .with_style(theme.search.option_button_group)
                                .aligned(),
                        )
                        .contained()
                        .with_margin_bottom(row_spacing),
                )
                .with_children(show_replace.then(|| {
                    Flex::row()
                        .with_child(
                            Flex::row()
                                .with_child(replace_view)
                                .contained()
                                .with_style(include_container_style)
                                .aligned()
                                .constrained()
                                .with_min_width(theme.search.editor.min_width)
                                .with_max_width(theme.search.editor.max_width)
                                .flex(1., false),
                        )
                        .with_child(
                            Flex::row()
                                .with_child(
                                    IconButton::new(0, "Ⓡ+")
                                        .with_action(Box::new(Replace), "Replace"),
                                )
                                .with_child(
                                    IconButton::new(1, "Ⓡ*")
                                        .with_action(Box::new(ReplaceAll), "Replace All"),
                                )
                                .contained()
                                .with_style(theme.search.option_button_group)
                                .aligned(),
                        )
                        .contained()
                        .with_margin_bottom(row_spacing)
                }))
                .with_children(show_filter.then(|| {
                    Flex::row()
                        .with_child(
                            Flex::row()
                                .with_child(included_files_view)
                                .contained()
                                .with_style(include_container_style)
                                .aligned()
                                .constrained()
                                .with_min_width(theme.search.include_exclude_editor.min_width)
                                .with_max_width(theme.search.include_exclude_editor.max_width)
                                .flex(1., false),
                        )
                        .with_child(
                            Flex::row()
                                .with_child(excluded_files_view)
                                .contained()
                                .with_style(exclude_container_style)
                                .aligned()
                                .constrained()
                                .with_min_width(theme.search.include_exclude_editor.min_width)
                                .with_max_width(theme.search.include_exclude_editor.max_width)
                                .flex(1., false),
                        )
                }))
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
            self.subscription = Some(cx.observe(&search, |_, _, cx| cx.notify()));
            self.active_project_search = Some(search);
            ToolbarItemLocation::PrimaryLeft {
                flex: Some((1., false)),
            }
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn row_count(&self, cx: &AppContext) -> usize {
        1 + self
            .active_project_search
            .as_ref()
            .map(|active_project_search| {
                active_project_search.read(cx).show_filter as usize
                    + active_project_search.read(cx).show_replace as usize
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use editor::DisplayPoint;
    use gpui::{color::Color, executor::Deterministic, TestAppContext};
    use project::FakeFs;
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
            assert_eq!(search_view.active_match_index, Some(Some(0)));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(2, 32)..DisplayPoint::new(2, 35)]
            );

            search_view.select_match(Direction::Next, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(Some(1)));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(2, 37)..DisplayPoint::new(2, 40)]
            );
            search_view.select_match(Direction::Next, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(Some(2)));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(5, 6)..DisplayPoint::new(5, 9)]
            );
            search_view.select_match(Direction::Next, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(Some(0)));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(2, 32)..DisplayPoint::new(2, 35)]
            );
            search_view.select_match(Direction::Prev, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(Some(2)));
            assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(5, 6)..DisplayPoint::new(5, 9)]
            );
            search_view.select_match(Direction::Prev, cx);
        });

        search_view.update(cx, |search_view, cx| {
            assert_eq!(search_view.active_match_index, Some(Some(1)));
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

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

        cx.spawn(
            |mut cx| async move { cx.dispatch_action(window_id, search_view_id, &ToggleFocus) },
        )
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
            |mut cx| async move { cx.dispatch_action(window_id, search_view_id, &ToggleFocus) },
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
        cx.spawn(
            |mut cx| async move { cx.dispatch_action(window_id, search_view_id, &ToggleFocus) },
        )
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

        cx.spawn(
            |mut cx| async move { cx.dispatch_action(window_id, search_view_id, &ToggleFocus) },
        )
        .detach();
        deterministic.run_until_parked();
        search_view.update(cx, |search_view, cx| {
            assert!(
                search_view.results_editor.is_focused(cx),
                "Search view with matching query should switch focus to the results editor after the toggle focus event",
            );
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

#[derive(Element)]
struct ToggleIconButton {
    id: usize,
    state: bool,
    icon: String,
    tooltip: Option<String>,
    action: Option<Rc<dyn Fn(bool) -> Box<dyn Action>>>,
}

impl ToggleIconButton {
    fn new(id: usize, state: bool, icon: impl AsRef<str>) -> Self {
        let icon = icon.as_ref().to_owned();
        Self {
            id,
            state,
            icon,
            tooltip: None,
            action: None,
        }
    }

    fn with_action(
        mut self,
        f: impl Fn(bool) -> Box<dyn Action> + 'static,
        tooltip: impl AsRef<str>,
    ) -> Self {
        let tooltip = tooltip.as_ref().to_owned();
        self.action = Some(Rc::new(f));
        self.tooltip = Some(tooltip);
        self
    }
}

impl ToggleIconButton {
    fn render<V: View>(&mut self, _view: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V> {
        enum ViewButton {}

        let button_state = self.state;
        let icon = self.icon.clone();
        let icon_button = MouseEventHandler::<ViewButton, _>::new(self.id, cx, |state, cx| {
            let theme = theme::current(cx);

            let style = theme
                .search
                .option_button
                .in_state(button_state)
                .style_for(state);

            Label::new(icon, style.text.clone())
                .contained()
                .with_style(style.container)
        });

        if let Some((tooltip, action)) = self.tooltip.as_ref().zip(self.action.as_ref()) {
            let tooltip_style = theme::current(cx).tooltip.clone();
            let tooltip = tooltip.clone();

            icon_button
                .on_click(MouseButton::Left, {
                    let action = action.clone();
                    move |_, _, cx| {
                        let action = (action)(!button_state);
                        let window_id = cx.window_id();
                        let view_id = cx.view_id();
                        cx.spawn(|_, mut cx| async move {
                            cx.dispatch_action(window_id, view_id, action.as_ref()).ok();
                        })
                        .detach()
                    }
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .with_tooltip::<ViewButton>(
                    self.id,
                    tooltip,
                    Some((action)(!self.state)),
                    tooltip_style,
                    cx,
                )
                .into_any()
        } else {
            icon_button.into_any()
        }
    }
}

#[derive(Element)]
struct IconButton {
    id: usize,
    icon: String,
    tooltip: Option<String>,
    action: Option<Box<dyn Action>>,
}

impl IconButton {
    fn new(id: usize, icon: impl AsRef<str>) -> Self {
        let icon = icon.as_ref().to_owned();
        Self {
            id,
            icon,
            tooltip: None,
            action: None,
        }
    }

    fn with_action(mut self, action: Box<dyn Action>, tooltip: impl AsRef<str>) -> Self {
        let tooltip = tooltip.as_ref().to_owned();
        self.action = Some(action);
        self.tooltip = Some(tooltip);
        self
    }
}

impl IconButton {
    fn render<V: View>(&mut self, _view: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V> {
        enum ViewButton {}

        let icon = self.icon.clone();
        let icon_button = MouseEventHandler::<ViewButton, _>::new(self.id, cx, |state, cx| {
            let theme = theme::current(cx);

            let style = theme.search.option_button.inactive_state().style_for(state);

            Label::new(icon, style.text.clone())
                .contained()
                .with_style(style.container)
        });

        if let Some((tooltip, action)) = self.tooltip.as_ref().zip(self.action.as_ref()) {
            let tooltip_style = theme::current(cx).tooltip.clone();
            let tooltip = tooltip.clone();

            icon_button
                .on_click(MouseButton::Left, {
                    let action = action.boxed_clone();
                    move |_, _, cx| {
                        let window_id = cx.window_id();
                        let view_id = cx.view_id();
                        let action = action.boxed_clone();
                        cx.spawn(|_, mut cx| async move {
                            cx.dispatch_action(window_id, view_id, action.as_ref()).ok();
                        })
                        .detach()
                    }
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .with_tooltip::<ViewButton>(
                    self.id,
                    tooltip,
                    Some(action.boxed_clone()),
                    tooltip_style,
                    cx,
                )
                .into_any()
        } else {
            icon_button.into_any()
        }
    }
}
