use crate::{
    history::SearchHistory,
    mode::{next_mode, SearchMode, Side},
    search_bar::{render_nav_button, render_search_mode_button},
    CycleMode, NextHistoryQuery, PreviousHistoryQuery, ReplaceAll, ReplaceNext, SearchOptions,
    SelectAllMatches, SelectNextMatch, SelectPrevMatch, ToggleCaseSensitive, ToggleReplace,
    ToggleWholeWord,
};
use collections::HashMap;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{
    actions, elements::*, impl_actions, Action, AnyViewHandle, AppContext, Entity, Subscription,
    Task, View, ViewContext, ViewHandle, WindowContext,
};
use project::search::SearchQuery;
use serde::Deserialize;
use std::{any::Any, sync::Arc};

use util::ResultExt;
use workspace::{
    item::ItemHandle,
    searchable::{Direction, SearchEvent, SearchableItemHandle, WeakSearchableItemHandle},
    Pane, ToolbarItemLocation, ToolbarItemView,
};

#[derive(Clone, Deserialize, PartialEq)]
pub struct Deploy {
    pub focus: bool,
}

actions!(buffer_search, [Dismiss, FocusEditor]);
impl_actions!(buffer_search, [Deploy]);

pub enum Event {
    UpdateLocation,
}

pub fn init(cx: &mut AppContext) {
    cx.add_action(BufferSearchBar::deploy_bar);
    cx.add_action(BufferSearchBar::dismiss);
    cx.add_action(BufferSearchBar::focus_editor);
    cx.add_action(BufferSearchBar::select_next_match);
    cx.add_action(BufferSearchBar::select_prev_match);
    cx.add_action(BufferSearchBar::select_all_matches);
    cx.add_action(BufferSearchBar::select_next_match_on_pane);
    cx.add_action(BufferSearchBar::select_prev_match_on_pane);
    cx.add_action(BufferSearchBar::select_all_matches_on_pane);
    cx.add_action(BufferSearchBar::handle_editor_cancel);
    cx.add_action(BufferSearchBar::next_history_query);
    cx.add_action(BufferSearchBar::previous_history_query);
    cx.add_action(BufferSearchBar::cycle_mode);
    cx.add_action(BufferSearchBar::cycle_mode_on_pane);
    cx.add_action(BufferSearchBar::replace_all);
    cx.add_action(BufferSearchBar::replace_next);
    cx.add_action(BufferSearchBar::replace_all_on_pane);
    cx.add_action(BufferSearchBar::replace_next_on_pane);
    cx.add_action(BufferSearchBar::toggle_replace);
    cx.add_action(BufferSearchBar::toggle_replace_on_a_pane);
    add_toggle_option_action::<ToggleCaseSensitive>(SearchOptions::CASE_SENSITIVE, cx);
    add_toggle_option_action::<ToggleWholeWord>(SearchOptions::WHOLE_WORD, cx);
}

fn add_toggle_option_action<A: Action>(option: SearchOptions, cx: &mut AppContext) {
    cx.add_action(move |pane: &mut Pane, _: &A, cx: &mut ViewContext<Pane>| {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |search_bar, cx| {
                if search_bar.show(cx) {
                    search_bar.toggle_search_option(option, cx);
                }
            });
        }
        cx.propagate_action();
    });
}

pub struct BufferSearchBar {
    query_editor: ViewHandle<Editor>,
    replacement_editor: ViewHandle<Editor>,
    active_searchable_item: Option<Box<dyn SearchableItemHandle>>,
    active_match_index: Option<usize>,
    active_searchable_item_subscription: Option<Subscription>,
    active_search: Option<Arc<SearchQuery>>,
    searchable_items_with_matches:
        HashMap<Box<dyn WeakSearchableItemHandle>, Vec<Box<dyn Any + Send>>>,
    pending_search: Option<Task<()>>,
    search_options: SearchOptions,
    default_options: SearchOptions,
    query_contains_error: bool,
    dismissed: bool,
    search_history: SearchHistory,
    current_mode: SearchMode,
    replace_is_active: bool,
}

impl Entity for BufferSearchBar {
    type Event = Event;
}

impl View for BufferSearchBar {
    fn ui_name() -> &'static str {
        "BufferSearchBar"
    }

    fn update_keymap_context(
        &self,
        keymap: &mut gpui::keymap_matcher::KeymapContext,
        cx: &AppContext,
    ) {
        Self::reset_to_default_keymap_context(keymap);
        let in_replace = self
            .replacement_editor
            .read_with(cx, |_, cx| cx.is_self_focused())
            .unwrap_or(false);
        if in_replace {
            keymap.add_identifier("in_replace");
        }
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.query_editor);
        }
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx).clone();
        let query_container_style = if self.query_contains_error {
            theme.search.invalid_editor
        } else {
            theme.search.editor.input.container
        };
        let supported_options = self
            .active_searchable_item
            .as_ref()
            .map(|active_searchable_item| active_searchable_item.supported_options())
            .unwrap_or_default();

        let previous_query_keystrokes =
            cx.binding_for_action(&PreviousHistoryQuery {})
                .map(|binding| {
                    binding
                        .keystrokes()
                        .iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                });
        let next_query_keystrokes = cx.binding_for_action(&NextHistoryQuery {}).map(|binding| {
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
        self.replacement_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Replace with...", cx);
        });
        let search_button_for_mode = |mode, side, cx: &mut ViewContext<BufferSearchBar>| {
            let is_active = self.current_mode == mode;

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
        let search_option_button = |option| {
            let is_active = self.search_options.contains(option);
            option.as_button(
                is_active,
                theme.tooltip.clone(),
                theme.search.option_button_component.clone(),
            )
        };
        let match_count = self
            .active_searchable_item
            .as_ref()
            .and_then(|searchable_item| {
                if self.query(cx).is_empty() {
                    return None;
                }
                let matches = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())?;
                let message = if let Some(match_ix) = self.active_match_index {
                    format!("{}/{}", match_ix + 1, matches.len())
                } else {
                    "No matches".to_string()
                };

                Some(
                    Label::new(message, theme.search.match_index.text.clone())
                        .contained()
                        .with_style(theme.search.match_index.container)
                        .aligned(),
                )
            });
        let nav_button_for_direction = |label, direction, cx: &mut ViewContext<Self>| {
            render_nav_button(
                label,
                direction,
                self.active_match_index.is_some(),
                move |_, this, cx| match direction {
                    Direction::Prev => this.select_prev_match(&Default::default(), cx),
                    Direction::Next => this.select_next_match(&Default::default(), cx),
                },
                cx,
            )
        };
        let query_column = Flex::row()
            .with_child(
                Svg::for_style(theme.search.editor_icon.clone().icon)
                    .contained()
                    .with_style(theme.search.editor_icon.clone().container),
            )
            .with_child(ChildView::new(&self.query_editor, cx).flex(1., true))
            .with_child(
                Flex::row()
                    .with_children(
                        supported_options
                            .case
                            .then(|| search_option_button(SearchOptions::CASE_SENSITIVE)),
                    )
                    .with_children(
                        supported_options
                            .word
                            .then(|| search_option_button(SearchOptions::WHOLE_WORD)),
                    )
                    .flex_float()
                    .contained(),
            )
            .align_children_center()
            .contained()
            .with_style(query_container_style)
            .constrained()
            .with_min_width(theme.search.editor.min_width)
            .with_max_width(theme.search.editor.max_width)
            .with_height(theme.search.search_bar_row_height)
            .flex(1., false);
        let should_show_replace_input = self.replace_is_active && supported_options.replacement;

        let replacement = should_show_replace_input.then(|| {
            Flex::row()
                .with_child(
                    Svg::for_style(theme.search.replace_icon.clone().icon)
                        .contained()
                        .with_style(theme.search.replace_icon.clone().container),
                )
                .with_child(ChildView::new(&self.replacement_editor, cx).flex(1., true))
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
        let switches_column = supported_options.replacement.then(|| {
            Flex::row()
                .align_children_center()
                .with_child(super::toggle_replace_button(
                    self.replace_is_active,
                    theme.tooltip.clone(),
                    theme.search.option_button_component.clone(),
                ))
                .constrained()
                .with_height(theme.search.search_bar_row_height)
                .contained()
                .with_style(theme.search.option_button_group)
        });
        let mode_column = Flex::row()
            .with_child(search_button_for_mode(
                SearchMode::Text,
                Some(Side::Left),
                cx,
            ))
            .with_child(search_button_for_mode(
                SearchMode::Regex,
                Some(Side::Right),
                cx,
            ))
            .contained()
            .with_style(theme.search.modes_container)
            .constrained()
            .with_height(theme.search.search_bar_row_height);

        let nav_column = Flex::row()
            .align_children_center()
            .with_children(replace_next)
            .with_children(replace_all)
            .with_child(self.render_action_button("icons/select-all.svg", cx))
            .with_child(Flex::row().with_children(match_count))
            .with_child(nav_button_for_direction("<", Direction::Prev, cx))
            .with_child(nav_button_for_direction(">", Direction::Next, cx))
            .constrained()
            .with_height(theme.search.search_bar_row_height)
            .flex_float();

        Flex::row()
            .with_child(query_column)
            .with_children(switches_column)
            .with_children(replacement)
            .with_child(mode_column)
            .with_child(nav_column)
            .contained()
            .with_style(theme.search.container)
            .into_any_named("search bar")
    }
}

impl ToolbarItemView for BufferSearchBar {
    fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_searchable_item_subscription.take();
        self.active_searchable_item.take();
        self.pending_search.take();

        if let Some(searchable_item_handle) =
            item.and_then(|item| item.to_searchable_item_handle(cx))
        {
            let this = cx.weak_handle();
            self.active_searchable_item_subscription =
                Some(searchable_item_handle.subscribe_to_search_events(
                    cx,
                    Box::new(move |search_event, cx| {
                        if let Some(this) = this.upgrade(cx) {
                            this.update(cx, |this, cx| {
                                this.on_active_searchable_item_event(search_event, cx)
                            });
                        }
                    }),
                ));

            self.active_searchable_item = Some(searchable_item_handle);
            let _ = self.update_matches(cx);
            if !self.dismissed {
                return ToolbarItemLocation::Secondary;
            }
        }

        ToolbarItemLocation::Hidden
    }

    fn location_for_event(
        &self,
        _: &Self::Event,
        _: ToolbarItemLocation,
        _: &AppContext,
    ) -> ToolbarItemLocation {
        if self.active_searchable_item.is_some() && !self.dismissed {
            ToolbarItemLocation::Secondary
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn row_count(&self, _: &ViewContext<Self>) -> usize {
        1
    }
}

impl BufferSearchBar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::auto_height(
                2,
                Some(Arc::new(|theme| theme.search.editor.input.clone())),
                cx,
            )
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();
        let replacement_editor = cx.add_view(|cx| {
            Editor::auto_height(
                2,
                Some(Arc::new(|theme| theme.search.editor.input.clone())),
                cx,
            )
        });
        // cx.subscribe(&replacement_editor, Self::on_query_editor_event)
        //     .detach();
        Self {
            query_editor,
            replacement_editor,
            active_searchable_item: None,
            active_searchable_item_subscription: None,
            active_match_index: None,
            searchable_items_with_matches: Default::default(),
            default_options: SearchOptions::NONE,
            search_options: SearchOptions::NONE,
            pending_search: None,
            query_contains_error: false,
            dismissed: true,
            search_history: SearchHistory::default(),
            current_mode: SearchMode::default(),
            active_search: None,
            replace_is_active: false,
        }
    }

    pub fn is_dismissed(&self) -> bool {
        self.dismissed
    }

    pub fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        self.dismissed = true;
        for searchable_item in self.searchable_items_with_matches.keys() {
            if let Some(searchable_item) =
                WeakSearchableItemHandle::upgrade(searchable_item.as_ref(), cx)
            {
                searchable_item.clear_matches(cx);
            }
        }
        if let Some(active_editor) = self.active_searchable_item.as_ref() {
            cx.focus(active_editor.as_any());
        }
        cx.emit(Event::UpdateLocation);
        cx.notify();
    }

    pub fn deploy(&mut self, deploy: &Deploy, cx: &mut ViewContext<Self>) -> bool {
        if self.show(cx) {
            self.search_suggested(cx);
            if deploy.focus {
                self.select_query(cx);
                cx.focus_self();
            }
            return true;
        }

        false
    }

    pub fn show(&mut self, cx: &mut ViewContext<Self>) -> bool {
        if self.active_searchable_item.is_none() {
            return false;
        }
        self.dismissed = false;
        cx.notify();
        cx.emit(Event::UpdateLocation);
        true
    }

    pub fn search_suggested(&mut self, cx: &mut ViewContext<Self>) {
        let search = self
            .query_suggestion(cx)
            .map(|suggestion| self.search(&suggestion, Some(self.default_options), cx));

        if let Some(search) = search {
            cx.spawn(|this, mut cx| async move {
                search.await?;
                this.update(&mut cx, |this, cx| this.activate_current_match(cx))
            })
            .detach_and_log_err(cx);
        }
    }

    pub fn activate_current_match(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(match_ix) = self.active_match_index {
            if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(matches) = self
                    .searchable_items_with_matches
                    .get(&active_searchable_item.downgrade())
                {
                    active_searchable_item.activate_match(match_ix, matches, cx)
                }
            }
        }
    }

    pub fn select_query(&mut self, cx: &mut ViewContext<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.select_all(&Default::default(), cx);
        });
    }

    pub fn query(&self, cx: &WindowContext) -> String {
        self.query_editor.read(cx).text(cx)
    }
    pub fn replacement(&self, cx: &WindowContext) -> String {
        self.replacement_editor.read(cx).text(cx)
    }
    pub fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> Option<String> {
        self.active_searchable_item
            .as_ref()
            .map(|searchable_item| searchable_item.query_suggestion(cx))
    }

    pub fn search(
        &mut self,
        query: &str,
        options: Option<SearchOptions>,
        cx: &mut ViewContext<Self>,
    ) -> oneshot::Receiver<()> {
        let options = options.unwrap_or(self.default_options);
        if query != self.query(cx) || self.search_options != options {
            self.query_editor.update(cx, |query_editor, cx| {
                query_editor.buffer().update(cx, |query_buffer, cx| {
                    let len = query_buffer.len(cx);
                    query_buffer.edit([(0..len, query)], None, cx);
                });
            });
            self.search_options = options;
            self.query_contains_error = false;
            self.clear_matches(cx);
            cx.notify();
        }
        self.update_matches(cx)
    }

    fn render_action_button(
        &self,
        icon: &'static str,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let tooltip = "Select All Matches";
        let tooltip_style = theme::current(cx).tooltip.clone();

        let theme = theme::current(cx);
        let style = theme.search.action_button.clone();

        gpui::elements::Component::element(SafeStylable::with_style(
            theme::components::action_button::Button::action(SelectAllMatches)
                .with_tooltip(tooltip, tooltip_style)
                .with_contents(theme::components::svg::Svg::new(icon)),
            style,
        ))
        .into_any()
    }

    pub fn activate_search_mode(&mut self, mode: SearchMode, cx: &mut ViewContext<Self>) {
        assert_ne!(
            mode,
            SearchMode::Semantic,
            "Semantic search is not supported in buffer search"
        );
        if mode == self.current_mode {
            return;
        }
        self.current_mode = mode;
        let _ = self.update_matches(cx);
        cx.notify();
    }

    fn deploy_bar(pane: &mut Pane, action: &Deploy, cx: &mut ViewContext<Pane>) {
        let mut propagate_action = true;
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |search_bar, cx| {
                if search_bar.deploy(action, cx) {
                    propagate_action = false;
                }
            });
        }
        if propagate_action {
            cx.propagate_action();
        }
    }

    fn handle_editor_cancel(pane: &mut Pane, _: &editor::Cancel, cx: &mut ViewContext<Pane>) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            if !search_bar.read(cx).dismissed {
                search_bar.update(cx, |search_bar, cx| search_bar.dismiss(&Dismiss, cx));
                return;
            }
        }
        cx.propagate_action();
    }

    pub fn focus_editor(&mut self, _: &FocusEditor, cx: &mut ViewContext<Self>) {
        if let Some(active_editor) = self.active_searchable_item.as_ref() {
            cx.focus(active_editor.as_any());
        }
    }

    fn toggle_search_option(&mut self, search_option: SearchOptions, cx: &mut ViewContext<Self>) {
        self.search_options.toggle(search_option);
        self.default_options = self.search_options;
        let _ = self.update_matches(cx);
        cx.notify();
    }

    pub fn set_search_options(
        &mut self,
        search_options: SearchOptions,
        cx: &mut ViewContext<Self>,
    ) {
        self.search_options = search_options;
        cx.notify();
    }

    fn select_next_match(&mut self, _: &SelectNextMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Next, 1, cx);
    }

    fn select_prev_match(&mut self, _: &SelectPrevMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Prev, 1, cx);
    }

    fn select_all_matches(&mut self, _: &SelectAllMatches, cx: &mut ViewContext<Self>) {
        if !self.dismissed && self.active_match_index.is_some() {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(matches) = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())
                {
                    searchable_item.select_matches(matches, cx);
                    self.focus_editor(&FocusEditor, cx);
                }
            }
        }
    }

    pub fn select_match(&mut self, direction: Direction, count: usize, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.active_match_index {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(matches) = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())
                {
                    let new_match_index = searchable_item
                        .match_index_for_direction(matches, index, direction, count, cx);
                    searchable_item.update_matches(matches, cx);
                    searchable_item.activate_match(new_match_index, matches, cx);
                }
            }
        }
    }

    fn select_next_match_on_pane(
        pane: &mut Pane,
        action: &SelectNextMatch,
        cx: &mut ViewContext<Pane>,
    ) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.select_next_match(action, cx));
        }
    }

    fn select_prev_match_on_pane(
        pane: &mut Pane,
        action: &SelectPrevMatch,
        cx: &mut ViewContext<Pane>,
    ) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.select_prev_match(action, cx));
        }
    }

    fn select_all_matches_on_pane(
        pane: &mut Pane,
        action: &SelectAllMatches,
        cx: &mut ViewContext<Pane>,
    ) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.select_all_matches(action, cx));
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        if let editor::Event::Edited { .. } = event {
            self.query_contains_error = false;
            self.clear_matches(cx);
            let search = self.update_matches(cx);
            cx.spawn(|this, mut cx| async move {
                search.await?;
                this.update(&mut cx, |this, cx| this.activate_current_match(cx))
            })
            .detach_and_log_err(cx);
        }
    }

    fn on_active_searchable_item_event(&mut self, event: SearchEvent, cx: &mut ViewContext<Self>) {
        match event {
            SearchEvent::MatchesInvalidated => {
                let _ = self.update_matches(cx);
            }
            SearchEvent::ActiveMatchChanged => self.update_match_index(cx),
        }
    }

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        let mut active_item_matches = None;
        for (searchable_item, matches) in self.searchable_items_with_matches.drain() {
            if let Some(searchable_item) =
                WeakSearchableItemHandle::upgrade(searchable_item.as_ref(), cx)
            {
                if Some(&searchable_item) == self.active_searchable_item.as_ref() {
                    active_item_matches = Some((searchable_item.downgrade(), matches));
                } else {
                    searchable_item.clear_matches(cx);
                }
            }
        }

        self.searchable_items_with_matches
            .extend(active_item_matches);
    }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) -> oneshot::Receiver<()> {
        let (done_tx, done_rx) = oneshot::channel();
        let query = self.query(cx);
        self.pending_search.take();

        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            if query.is_empty() {
                self.active_match_index.take();
                active_searchable_item.clear_matches(cx);
                let _ = done_tx.send(());
                cx.notify();
            } else {
                let query: Arc<_> = if self.current_mode == SearchMode::Regex {
                    match SearchQuery::regex(
                        query,
                        self.search_options.contains(SearchOptions::WHOLE_WORD),
                        self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                        Vec::new(),
                        Vec::new(),
                    ) {
                        Ok(query) => query.with_replacement(self.replacement(cx)),
                        Err(_) => {
                            self.query_contains_error = true;
                            cx.notify();
                            return done_rx;
                        }
                    }
                } else {
                    match SearchQuery::text(
                        query,
                        self.search_options.contains(SearchOptions::WHOLE_WORD),
                        self.search_options.contains(SearchOptions::CASE_SENSITIVE),
                        Vec::new(),
                        Vec::new(),
                    ) {
                        Ok(query) => query.with_replacement(self.replacement(cx)),
                        Err(_) => {
                            self.query_contains_error = true;
                            cx.notify();
                            return done_rx;
                        }
                    }
                }
                .into();
                self.active_search = Some(query.clone());
                let query_text = query.as_str().to_string();
                let matches = active_searchable_item.find_matches(query, cx);

                let active_searchable_item = active_searchable_item.downgrade();
                self.pending_search = Some(cx.spawn(|this, mut cx| async move {
                    let matches = matches.await;
                    this.update(&mut cx, |this, cx| {
                        if let Some(active_searchable_item) =
                            WeakSearchableItemHandle::upgrade(active_searchable_item.as_ref(), cx)
                        {
                            this.searchable_items_with_matches
                                .insert(active_searchable_item.downgrade(), matches);

                            this.update_match_index(cx);
                            this.search_history.add(query_text);
                            if !this.dismissed {
                                let matches = this
                                    .searchable_items_with_matches
                                    .get(&active_searchable_item.downgrade())
                                    .unwrap();
                                active_searchable_item.update_matches(matches, cx);
                                let _ = done_tx.send(());
                            }
                            cx.notify();
                        }
                    })
                    .log_err();
                }));
            }
        }
        done_rx
    }

    fn update_match_index(&mut self, cx: &mut ViewContext<Self>) {
        let new_index = self
            .active_searchable_item
            .as_ref()
            .and_then(|searchable_item| {
                let matches = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())?;
                searchable_item.active_match_index(matches, cx)
            });
        if new_index != self.active_match_index {
            self.active_match_index = new_index;
            cx.notify();
        }
    }

    fn next_history_query(&mut self, _: &NextHistoryQuery, cx: &mut ViewContext<Self>) {
        if let Some(new_query) = self.search_history.next().map(str::to_string) {
            let _ = self.search(&new_query, Some(self.search_options), cx);
        } else {
            self.search_history.reset_selection();
            let _ = self.search("", Some(self.search_options), cx);
        }
    }

    fn previous_history_query(&mut self, _: &PreviousHistoryQuery, cx: &mut ViewContext<Self>) {
        if self.query(cx).is_empty() {
            if let Some(new_query) = self.search_history.current().map(str::to_string) {
                let _ = self.search(&new_query, Some(self.search_options), cx);
                return;
            }
        }

        if let Some(new_query) = self.search_history.previous().map(str::to_string) {
            let _ = self.search(&new_query, Some(self.search_options), cx);
        }
    }
    fn cycle_mode(&mut self, _: &CycleMode, cx: &mut ViewContext<Self>) {
        self.activate_search_mode(next_mode(&self.current_mode, false), cx);
    }
    fn cycle_mode_on_pane(pane: &mut Pane, action: &CycleMode, cx: &mut ViewContext<Pane>) {
        let mut should_propagate = true;
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| {
                if bar.show(cx) {
                    should_propagate = false;
                    bar.cycle_mode(action, cx);
                    false
                } else {
                    true
                }
            });
        }
        if should_propagate {
            cx.propagate_action();
        }
    }
    fn toggle_replace(&mut self, _: &ToggleReplace, cx: &mut ViewContext<Self>) {
        if let Some(_) = &self.active_searchable_item {
            self.replace_is_active = !self.replace_is_active;
            cx.notify();
        }
    }
    fn toggle_replace_on_a_pane(pane: &mut Pane, _: &ToggleReplace, cx: &mut ViewContext<Pane>) {
        let mut should_propagate = true;
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| {
                if let Some(_) = &bar.active_searchable_item {
                    should_propagate = false;
                    bar.replace_is_active = !bar.replace_is_active;
                    if bar.dismissed {
                        bar.show(cx);
                    }
                    cx.notify();
                }
            });
        }
        if should_propagate {
            cx.propagate_action();
        }
    }
    fn replace_next(&mut self, _: &ReplaceNext, cx: &mut ViewContext<Self>) {
        if !self.dismissed && self.active_search.is_some() {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(query) = self.active_search.as_ref() {
                    if let Some(matches) = self
                        .searchable_items_with_matches
                        .get(&searchable_item.downgrade())
                    {
                        if let Some(active_index) = self.active_match_index {
                            let query = query
                                .as_ref()
                                .clone()
                                .with_replacement(self.replacement(cx));
                            searchable_item.replace(&matches[active_index], &query, cx);
                            self.select_next_match(&SelectNextMatch, cx);
                        }
                    }
                }
            }
        }
    }
    fn replace_all(&mut self, _: &ReplaceAll, cx: &mut ViewContext<Self>) {
        if !self.dismissed && self.active_search.is_some() {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(query) = self.active_search.as_ref() {
                    if let Some(matches) = self
                        .searchable_items_with_matches
                        .get(&searchable_item.downgrade())
                    {
                        let query = query
                            .as_ref()
                            .clone()
                            .with_replacement(self.replacement(cx));
                        for m in matches {
                            searchable_item.replace(m, &query, cx);
                        }
                    }
                }
            }
        }
    }
    fn replace_next_on_pane(pane: &mut Pane, action: &ReplaceNext, cx: &mut ViewContext<Pane>) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.replace_next(action, cx));
            return;
        }
        cx.propagate_action();
    }
    fn replace_all_on_pane(pane: &mut Pane, action: &ReplaceAll, cx: &mut ViewContext<Pane>) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.replace_all(action, cx));
            return;
        }
        cx.propagate_action();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{DisplayPoint, Editor};
    use gpui::{color::Color, test::EmptyView, TestAppContext};
    use language::Buffer;
    use unindent::Unindent as _;

    fn init_test(cx: &mut TestAppContext) -> (ViewHandle<Editor>, ViewHandle<BufferSearchBar>) {
        crate::project_search::tests::init_test(cx);

        let buffer = cx.add_model(|cx| {
            Buffer::new(
                0,
                cx.model_id() as u64,
                r#"
                A regular expression (shortened as regex or regexp;[1] also referred to as
                rational expression[2][3]) is a sequence of characters that specifies a search
                pattern in text. Usually such patterns are used by string-searching algorithms
                for "find" or "find and replace" operations on strings, or for input validation.
                "#
                .unindent(),
            )
        });
        let window = cx.add_window(|_| EmptyView);
        let editor = window.add_view(cx, |cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = window.add_view(cx, |cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        (editor, search_bar)
    }

    #[gpui::test]
    async fn test_search_simple(cx: &mut TestAppContext) {
        let (editor, search_bar) = init_test(cx);

        // Search for a string that appears with different casing.
        // By default, search is case-insensitive.
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("us", None, cx))
            .await
            .unwrap();
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_text_background_highlights(cx),
                &[
                    (
                        DisplayPoint::new(2, 17)..DisplayPoint::new(2, 19),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),
                        Color::red(),
                    ),
                ]
            );
        });

        // Switch to a case sensitive search.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
        });
        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_text_background_highlights(cx),
                &[(
                    DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),
                    Color::red(),
                )]
            );
        });

        // Search for a string that appears both as a whole word and
        // within other words. By default, all results are found.
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("or", None, cx))
            .await
            .unwrap();
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_text_background_highlights(cx),
                &[
                    (
                        DisplayPoint::new(0, 24)..DisplayPoint::new(0, 26),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(2, 71)..DisplayPoint::new(2, 73),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(3, 1)..DisplayPoint::new(3, 3),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(3, 60)..DisplayPoint::new(3, 62),
                        Color::red(),
                    ),
                ]
            );
        });

        // Switch to a whole word search.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, cx);
        });
        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_text_background_highlights(cx),
                &[
                    (
                        DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13),
                        Color::red(),
                    ),
                    (
                        DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58),
                        Color::red(),
                    ),
                ]
            );
        });

        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the previous match selects
        // the closest match to the left.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the next match selects the
        // closest match to the right.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        // Park the cursor after the last match and ensure that going to the previous match selects
        // the last match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        // Park the cursor after the last match and ensure that going to the next match selects the
        // first match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_next_match(&SelectNextMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor before the first match and ensure that going to the previous match
        // selects the last match.
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)])
            });
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_prev_match(&SelectPrevMatch, cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });
    }

    #[gpui::test]
    async fn test_search_option_handling(cx: &mut TestAppContext) {
        let (editor, search_bar) = init_test(cx);

        // show with options should make current search case sensitive
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.show(cx);
                search_bar.search("us", Some(SearchOptions::CASE_SENSITIVE), cx)
            })
            .await
            .unwrap();
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_text_background_highlights(cx),
                &[(
                    DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),
                    Color::red(),
                )]
            );
        });

        // search_suggested should restore default options
        search_bar.update(cx, |search_bar, cx| {
            search_bar.search_suggested(cx);
            assert_eq!(search_bar.search_options, SearchOptions::NONE)
        });

        // toggling a search option should update the defaults
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("regex", Some(SearchOptions::CASE_SENSITIVE), cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, cx)
        });
        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_text_background_highlights(cx),
                &[(
                    DisplayPoint::new(0, 35)..DisplayPoint::new(0, 40),
                    Color::red(),
                ),]
            );
        });

        // defaults should still include whole word
        search_bar.update(cx, |search_bar, cx| {
            search_bar.search_suggested(cx);
            assert_eq!(
                search_bar.search_options,
                SearchOptions::CASE_SENSITIVE | SearchOptions::WHOLE_WORD
            )
        });
    }

    #[gpui::test]
    async fn test_search_select_all_matches(cx: &mut TestAppContext) {
        crate::project_search::tests::init_test(cx);

        let buffer_text = r#"
        A regular expression (shortened as regex or regexp;[1] also referred to as
        rational expression[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
        .unindent();
        let expected_query_matches_count = buffer_text
            .chars()
            .filter(|c| c.to_ascii_lowercase() == 'a')
            .count();
        assert!(
            expected_query_matches_count > 1,
            "Should pick a query with multiple results"
        );
        let buffer = cx.add_model(|cx| Buffer::new(0, cx.model_id() as u64, buffer_text));
        let window = cx.add_window(|_| EmptyView);
        let editor = window.add_view(cx, |cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = window.add_view(cx, |cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        search_bar
            .update(cx, |search_bar, cx| search_bar.search("a", None, cx))
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            cx.focus(search_bar.query_editor.as_any());
            search_bar.activate_current_match(cx);
        });

        window.read_with(cx, |cx| {
            assert!(
                !editor.is_focused(cx),
                "Initially, the editor should not be focused"
            );
        });

        let initial_selections = editor.update(cx, |editor, cx| {
            let initial_selections = editor.selections.display_ranges(cx);
            assert_eq!(
                initial_selections.len(), 1,
                "Expected to have only one selection before adding carets to all matches, but got: {initial_selections:?}",
            );
            initial_selections
        });
        search_bar.update(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            cx.focus(search_bar.query_editor.as_any());
            search_bar.select_all_matches(&SelectAllMatches, cx);
        });
        window.read_with(cx, |cx| {
            assert!(
                editor.is_focused(cx),
                "Should focus editor after successful SelectAllMatches"
            );
        });
        search_bar.update(cx, |search_bar, cx| {
            let all_selections =
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
            assert_eq!(
                all_selections.len(),
                expected_query_matches_count,
                "Should select all `a` characters in the buffer, but got: {all_selections:?}"
            );
            assert_eq!(
                search_bar.active_match_index,
                Some(0),
                "Match index should not change after selecting all matches"
            );
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_next_match(&SelectNextMatch, cx);
        });
        window.read_with(cx, |cx| {
            assert!(
                editor.is_focused(cx),
                "Should still have editor focused after SelectNextMatch"
            );
        });
        search_bar.update(cx, |search_bar, cx| {
            let all_selections =
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
            assert_eq!(
                all_selections.len(),
                1,
                "On next match, should deselect items and select the next match"
            );
            assert_ne!(
                all_selections, initial_selections,
                "Next match should be different from the first selection"
            );
            assert_eq!(
                search_bar.active_match_index,
                Some(1),
                "Match index should be updated to the next one"
            );
        });

        search_bar.update(cx, |search_bar, cx| {
            cx.focus(search_bar.query_editor.as_any());
            search_bar.select_all_matches(&SelectAllMatches, cx);
        });
        window.read_with(cx, |cx| {
            assert!(
                editor.is_focused(cx),
                "Should focus editor after successful SelectAllMatches"
            );
        });
        search_bar.update(cx, |search_bar, cx| {
            let all_selections =
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
            assert_eq!(
                all_selections.len(),
                expected_query_matches_count,
                "Should select all `a` characters in the buffer, but got: {all_selections:?}"
            );
            assert_eq!(
                search_bar.active_match_index,
                Some(1),
                "Match index should not change after selecting all matches"
            );
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_prev_match(&SelectPrevMatch, cx);
        });
        window.read_with(cx, |cx| {
            assert!(
                editor.is_focused(cx),
                "Should still have editor focused after SelectPrevMatch"
            );
        });
        let last_match_selections = search_bar.update(cx, |search_bar, cx| {
            let all_selections =
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
            assert_eq!(
                all_selections.len(),
                1,
                "On previous match, should deselect items and select the previous item"
            );
            assert_eq!(
                all_selections, initial_selections,
                "Previous match should be the same as the first selection"
            );
            assert_eq!(
                search_bar.active_match_index,
                Some(0),
                "Match index should be updated to the previous one"
            );
            all_selections
        });

        search_bar
            .update(cx, |search_bar, cx| {
                cx.focus(search_bar.query_editor.as_any());
                search_bar.search("abas_nonexistent_match", None, cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_all_matches(&SelectAllMatches, cx);
        });
        window.read_with(cx, |cx| {
            assert!(
                !editor.is_focused(cx),
                "Should not switch focus to editor if SelectAllMatches does not find any matches"
            );
        });
        search_bar.update(cx, |search_bar, cx| {
            let all_selections =
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
            assert_eq!(
                all_selections, last_match_selections,
                "Should not select anything new if there are no matches"
            );
            assert!(
                search_bar.active_match_index.is_none(),
                "For no matches, there should be no active match index"
            );
        });
    }

    #[gpui::test]
    async fn test_search_query_history(cx: &mut TestAppContext) {
        crate::project_search::tests::init_test(cx);

        let buffer_text = r#"
        A regular expression (shortened as regex or regexp;[1] also referred to as
        rational expression[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
        .unindent();
        let buffer = cx.add_model(|cx| Buffer::new(0, cx.model_id() as u64, buffer_text));
        let window = cx.add_window(|_| EmptyView);

        let editor = window.add_view(cx, |cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = window.add_view(cx, |cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(cx);
            search_bar
        });

        // Add 3 search items into the history.
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("a", None, cx))
            .await
            .unwrap();
        search_bar
            .update(cx, |search_bar, cx| search_bar.search("b", None, cx))
            .await
            .unwrap();
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("c", Some(SearchOptions::CASE_SENSITIVE), cx)
            })
            .await
            .unwrap();
        // Ensure that the latest search is active.
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next history query after the latest should set the query to the empty string.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // First previous query for empty current query should set the query to the latest.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Further previous items should go over the history in reverse order.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Previous items should never go behind the first history item.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "a");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "a");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        // Next items should go over the history in the original order.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::CASE_SENSITIVE);
        });

        search_bar
            .update(cx, |search_bar, cx| search_bar.search("ba", None, cx))
            .await
            .unwrap();
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "ba");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });

        // New search input should add another entry to history and move the selection to the end of the history.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.previous_history_query(&PreviousHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "b");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "c");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "ba");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
        search_bar.update(cx, |search_bar, cx| {
            search_bar.next_history_query(&NextHistoryQuery, cx);
        });
        search_bar.read_with(cx, |search_bar, cx| {
            assert_eq!(search_bar.query(cx), "");
            assert_eq!(search_bar.search_options, SearchOptions::NONE);
        });
    }
    #[gpui::test]
    async fn test_replace_simple(cx: &mut TestAppContext) {
        let (editor, search_bar) = init_test(cx);

        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("expression", None, cx)
            })
            .await
            .unwrap();

        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                // We use $1 here as initially we should be in Text mode, where `$1` should be treated literally.
                editor.set_text("expr$1", cx);
            });
            search_bar.replace_all(&ReplaceAll, cx)
        });
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex or regexp;[1] also referred to as
        rational expr$1[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );

        // Search for word boundaries and replace just a single one.
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.search("or", Some(SearchOptions::WHOLE_WORD), cx)
            })
            .await
            .unwrap();

        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("banana", cx);
            });
            search_bar.replace_next(&ReplaceNext, cx)
        });
        // Notice how the first or in the text (shORtened) is not replaced. Neither are the remaining hits of `or` in the text.
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex banana regexp;[1] also referred to as
        rational expr$1[2][3]) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );
        // Let's turn on regex mode.
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.activate_search_mode(SearchMode::Regex, cx);
                search_bar.search("\\[([^\\]]+)\\]", None, cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("${1}number", cx);
            });
            search_bar.replace_all(&ReplaceAll, cx)
        });
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex banana regexp;1number also referred to as
        rational expr$12number3number) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching algorithms
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );
        // Now with a whole-word twist.
        search_bar
            .update(cx, |search_bar, cx| {
                search_bar.activate_search_mode(SearchMode::Regex, cx);
                search_bar.search("a\\w+s", Some(SearchOptions::WHOLE_WORD), cx)
            })
            .await
            .unwrap();
        search_bar.update(cx, |search_bar, cx| {
            search_bar.replacement_editor.update(cx, |editor, cx| {
                editor.set_text("things", cx);
            });
            search_bar.replace_all(&ReplaceAll, cx)
        });
        // The only word affected by this edit should be `algorithms`, even though there's a bunch
        // of words in this text that would match this regex if not for WHOLE_WORD.
        assert_eq!(
            editor.read_with(cx, |this, cx| { this.text(cx) }),
            r#"
        A regular expr$1 (shortened as regex banana regexp;1number also referred to as
        rational expr$12number3number) is a sequence of characters that specifies a search
        pattern in text. Usually such patterns are used by string-searching things
        for "find" or "find and replace" operations on strings, or for input validation.
        "#
            .unindent()
        );
    }
}
