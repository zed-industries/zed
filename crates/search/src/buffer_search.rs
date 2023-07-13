use crate::{
    SearchOption, SelectAllMatches, SelectNextMatch, SelectPrevMatch, ToggleCaseSensitive,
    ToggleRegex, ToggleWholeWord,
};
use collections::HashMap;
use editor::Editor;
use gpui::{
    actions,
    elements::*,
    impl_actions,
    platform::{CursorStyle, MouseButton},
    Action, AnyViewHandle, AppContext, Entity, Subscription, Task, View, ViewContext, ViewHandle,
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
    cx.add_action(BufferSearchBar::deploy);
    cx.add_action(BufferSearchBar::dismiss);
    cx.add_action(BufferSearchBar::focus_editor);
    cx.add_action(BufferSearchBar::select_next_match);
    cx.add_action(BufferSearchBar::select_prev_match);
    cx.add_action(BufferSearchBar::select_all_matches);
    cx.add_action(BufferSearchBar::select_next_match_on_pane);
    cx.add_action(BufferSearchBar::select_prev_match_on_pane);
    cx.add_action(BufferSearchBar::select_all_matches_on_pane);
    cx.add_action(BufferSearchBar::handle_editor_cancel);
    add_toggle_option_action::<ToggleCaseSensitive>(SearchOption::CaseSensitive, cx);
    add_toggle_option_action::<ToggleWholeWord>(SearchOption::WholeWord, cx);
    add_toggle_option_action::<ToggleRegex>(SearchOption::Regex, cx);
}

fn add_toggle_option_action<A: Action>(option: SearchOption, cx: &mut AppContext) {
    cx.add_action(move |pane: &mut Pane, _: &A, cx: &mut ViewContext<Pane>| {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            if search_bar.update(cx, |search_bar, cx| search_bar.show(false, false, cx)) {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.toggle_search_option(option, cx);
                });
                return;
            }
        }
        cx.propagate_action();
    });
}

pub struct BufferSearchBar {
    pub query_editor: ViewHandle<Editor>,
    active_searchable_item: Option<Box<dyn SearchableItemHandle>>,
    active_match_index: Option<usize>,
    active_searchable_item_subscription: Option<Subscription>,
    searchable_items_with_matches:
        HashMap<Box<dyn WeakSearchableItemHandle>, Vec<Box<dyn Any + Send>>>,
    pending_search: Option<Task<()>>,
    case_sensitive: bool,
    whole_word: bool,
    regex: bool,
    query_contains_error: bool,
    dismissed: bool,
}

impl Entity for BufferSearchBar {
    type Event = Event;
}

impl View for BufferSearchBar {
    fn ui_name() -> &'static str {
        "BufferSearchBar"
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.query_editor);
        }
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx).clone();
        let editor_container = if self.query_contains_error {
            theme.search.invalid_editor
        } else {
            theme.search.editor.input.container
        };
        let supported_options = self
            .active_searchable_item
            .as_ref()
            .map(|active_searchable_item| active_searchable_item.supported_options())
            .unwrap_or_default();

        Flex::row()
            .with_child(
                Flex::row()
                    .with_child(
                        Flex::row()
                            .with_child(
                                ChildView::new(&self.query_editor, cx)
                                    .aligned()
                                    .left()
                                    .flex(1., true),
                            )
                            .with_children(self.active_searchable_item.as_ref().and_then(
                                |searchable_item| {
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
                                },
                            ))
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
                            .with_children(self.render_search_option(
                                supported_options.case,
                                "Case",
                                SearchOption::CaseSensitive,
                                cx,
                            ))
                            .with_children(self.render_search_option(
                                supported_options.word,
                                "Word",
                                SearchOption::WholeWord,
                                cx,
                            ))
                            .with_children(self.render_search_option(
                                supported_options.regex,
                                "Regex",
                                SearchOption::Regex,
                                cx,
                            ))
                            .contained()
                            .with_style(theme.search.option_button_group)
                            .aligned(),
                    )
                    .flex(1., true),
            )
            .with_child(self.render_close_button(&theme.search, cx))
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
            self.update_matches(false, cx);
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

        Self {
            query_editor,
            active_searchable_item: None,
            active_searchable_item_subscription: None,
            active_match_index: None,
            searchable_items_with_matches: Default::default(),
            case_sensitive: false,
            whole_word: false,
            regex: false,
            pending_search: None,
            query_contains_error: false,
            dismissed: true,
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

    pub fn show(&mut self, focus: bool, suggest_query: bool, cx: &mut ViewContext<Self>) -> bool {
        let searchable_item = if let Some(searchable_item) = &self.active_searchable_item {
            SearchableItemHandle::boxed_clone(searchable_item.as_ref())
        } else {
            return false;
        };

        if suggest_query {
            let text = searchable_item.query_suggestion(cx);
            if !text.is_empty() {
                self.set_query(&text, cx);
            }
        }

        if focus {
            let query_editor = self.query_editor.clone();
            query_editor.update(cx, |query_editor, cx| {
                query_editor.select_all(&editor::SelectAll, cx);
            });
            cx.focus_self();
        }

        self.dismissed = false;
        cx.notify();
        cx.emit(Event::UpdateLocation);
        true
    }

    fn set_query(&mut self, query: &str, cx: &mut ViewContext<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.buffer().update(cx, |query_buffer, cx| {
                let len = query_buffer.len(cx);
                query_buffer.edit([(0..len, query)], None, cx);
            });
        });
    }

    fn render_search_option(
        &self,
        option_supported: bool,
        icon: &'static str,
        option: SearchOption,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement<Self>> {
        if !option_supported {
            return None;
        }

        let tooltip_style = theme::current(cx).tooltip.clone();
        let is_active = self.is_search_option_enabled(option);
        Some(
            MouseEventHandler::<Self, _>::new(option as usize, cx, |state, cx| {
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
                option as usize,
                format!("Toggle {}", option.label()),
                Some(option.to_toggle_action()),
                tooltip_style,
                cx,
            )
            .into_any(),
        )
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
        .on_click(MouseButton::Left, {
            move |_, this, cx| match direction {
                Direction::Prev => this.select_prev_match(&Default::default(), cx),
                Direction::Next => this.select_next_match(&Default::default(), cx),
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

    fn render_close_button(
        &self,
        theme: &theme::Search,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let tooltip = "Dismiss Buffer Search";
        let tooltip_style = theme::current(cx).tooltip.clone();

        enum CloseButton {}
        MouseEventHandler::<CloseButton, _>::new(0, cx, |state, _| {
            let style = theme.dismiss_button.style_for(state);
            Svg::new("icons/x_mark_8.svg")
                .with_color(style.color)
                .constrained()
                .with_width(style.icon_width)
                .aligned()
                .constrained()
                .with_width(style.button_width)
                .contained()
                .with_style(style.container)
        })
        .on_click(MouseButton::Left, move |_, this, cx| {
            this.dismiss(&Default::default(), cx)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .with_tooltip::<CloseButton>(
            0,
            tooltip.to_string(),
            Some(Box::new(Dismiss)),
            tooltip_style,
            cx,
        )
        .into_any()
    }

    fn deploy(pane: &mut Pane, action: &Deploy, cx: &mut ViewContext<Pane>) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            if search_bar.update(cx, |search_bar, cx| search_bar.show(action.focus, true, cx)) {
                return;
            }
        }
        cx.propagate_action();
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

    fn focus_editor(&mut self, _: &FocusEditor, cx: &mut ViewContext<Self>) {
        if let Some(active_editor) = self.active_searchable_item.as_ref() {
            cx.focus(active_editor.as_any());
        }
    }

    fn is_search_option_enabled(&self, search_option: SearchOption) -> bool {
        match search_option {
            SearchOption::WholeWord => self.whole_word,
            SearchOption::CaseSensitive => self.case_sensitive,
            SearchOption::Regex => self.regex,
        }
    }

    fn toggle_search_option(&mut self, search_option: SearchOption, cx: &mut ViewContext<Self>) {
        let value = match search_option {
            SearchOption::WholeWord => &mut self.whole_word,
            SearchOption::CaseSensitive => &mut self.case_sensitive,
            SearchOption::Regex => &mut self.regex,
        };
        *value = !*value;
        self.update_matches(false, cx);
        cx.notify();
    }

    fn select_next_match(&mut self, _: &SelectNextMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Next, cx);
    }

    fn select_prev_match(&mut self, _: &SelectPrevMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Prev, cx);
    }

    fn select_all_matches(&mut self, _: &SelectAllMatches, cx: &mut ViewContext<Self>) {
        if !self.dismissed {
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

    pub fn select_match(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.active_match_index {
            if let Some(searchable_item) = self.active_searchable_item.as_ref() {
                if let Some(matches) = self
                    .searchable_items_with_matches
                    .get(&searchable_item.downgrade())
                {
                    let new_match_index =
                        searchable_item.match_index_for_direction(matches, index, direction, cx);
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
        if let editor::Event::BufferEdited { .. } = event {
            self.query_contains_error = false;
            self.clear_matches(cx);
            self.update_matches(true, cx);
            cx.notify();
        }
    }

    fn on_active_searchable_item_event(&mut self, event: SearchEvent, cx: &mut ViewContext<Self>) {
        match event {
            SearchEvent::MatchesInvalidated => self.update_matches(false, cx),
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

    fn update_matches(&mut self, select_closest_match: bool, cx: &mut ViewContext<Self>) {
        let query = self.query_editor.read(cx).text(cx);
        self.pending_search.take();
        if let Some(active_searchable_item) = self.active_searchable_item.as_ref() {
            if query.is_empty() {
                self.active_match_index.take();
                active_searchable_item.clear_matches(cx);
            } else {
                let query = if self.regex {
                    match SearchQuery::regex(
                        query,
                        self.whole_word,
                        self.case_sensitive,
                        Vec::new(),
                        Vec::new(),
                    ) {
                        Ok(query) => query,
                        Err(_) => {
                            self.query_contains_error = true;
                            cx.notify();
                            return;
                        }
                    }
                } else {
                    SearchQuery::text(
                        query,
                        self.whole_word,
                        self.case_sensitive,
                        Vec::new(),
                        Vec::new(),
                    )
                };

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
                            if !this.dismissed {
                                let matches = this
                                    .searchable_items_with_matches
                                    .get(&active_searchable_item.downgrade())
                                    .unwrap();
                                active_searchable_item.update_matches(matches, cx);
                                if select_closest_match {
                                    if let Some(match_ix) = this.active_match_index {
                                        active_searchable_item
                                            .activate_match(match_ix, matches, cx);
                                    }
                                }
                            }
                            cx.notify();
                        }
                    })
                    .log_err();
                }));
            }
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{DisplayPoint, Editor};
    use gpui::{color::Color, test::EmptyView, TestAppContext};
    use language::Buffer;
    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_search_simple(cx: &mut TestAppContext) {
        crate::project_search::tests::init_test(cx);

        let buffer = cx.add_model(|cx| {
            Buffer::new(
                0,
                r#"
                A regular expression (shortened as regex or regexp;[1] also referred to as
                rational expression[2][3]) is a sequence of characters that specifies a search
                pattern in text. Usually such patterns are used by string-searching algorithms
                for "find" or "find and replace" operations on strings, or for input validation.
                "#
                .unindent(),
                cx,
            )
        });
        let (window_id, _root_view) = cx.add_window(|_| EmptyView);

        let editor = cx.add_view(window_id, |cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = cx.add_view(window_id, |cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(false, true, cx);
            search_bar
        });

        // Search for a string that appears with different casing.
        // By default, search is case-insensitive.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.set_query("us", cx);
        });
        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_background_highlights(cx),
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
            search_bar.toggle_search_option(SearchOption::CaseSensitive, cx);
        });
        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_background_highlights(cx),
                &[(
                    DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),
                    Color::red(),
                )]
            );
        });

        // Search for a string that appears both as a whole word and
        // within other words. By default, all results are found.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.set_query("or", cx);
        });
        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_background_highlights(cx),
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
            search_bar.toggle_search_option(SearchOption::WholeWord, cx);
        });
        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.all_background_highlights(cx),
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
        let buffer = cx.add_model(|cx| Buffer::new(0, buffer_text, cx));
        let (window_id, _root_view) = cx.add_window(|_| EmptyView);

        let editor = cx.add_view(window_id, |cx| Editor::for_buffer(buffer.clone(), None, cx));

        let search_bar = cx.add_view(window_id, |cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(false, true, cx);
            search_bar
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.set_query("a", cx);
        });

        editor.next_notification(cx).await;
        editor.update(cx, |editor, cx| {
            let initial_selections =   editor.selections.display_ranges(cx);
            assert_eq!(
                initial_selections.len(), 1,
                "Expected to have only one selection before adding carets to all matches, but got: {initial_selections:?}",
            )
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_all_matches(&SelectAllMatches, cx);
            let all_selections =
                editor.update(cx, |editor, cx| editor.selections.display_ranges(cx));
            assert_eq!(
                all_selections.len(),
                expected_query_matches_count,
                "Should select all `a` characters in the buffer, but got: {all_selections:?}"
            );
        });
    }
}
