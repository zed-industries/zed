use crate::{
    active_match_index, match_index_for_direction, query_suggestion_for_editor, Direction,
    SearchOption, SelectNextMatch, SelectPrevMatch,
};
use collections::HashMap;
use editor::{Anchor, Autoscroll, Editor};
use gpui::{
    actions, elements::*, impl_actions, impl_internal_actions, platform::CursorStyle, AppContext,
    Entity, MutableAppContext, RenderContext, Subscription, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use language::OffsetRangeExt;
use project::search::SearchQuery;
use serde::Deserialize;
use settings::Settings;
use std::ops::Range;
use workspace::{ItemHandle, Pane, ToolbarItemLocation, ToolbarItemView};

#[derive(Clone, Deserialize, PartialEq)]
pub struct Deploy {
    pub focus: bool,
}

#[derive(Clone, PartialEq)]
pub struct ToggleSearchOption(pub SearchOption);

actions!(buffer_search, [Dismiss, FocusEditor]);
impl_actions!(buffer_search, [Deploy]);
impl_internal_actions!(buffer_search, [ToggleSearchOption]);

pub enum Event {
    UpdateLocation,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(BufferSearchBar::deploy);
    cx.add_action(BufferSearchBar::dismiss);
    cx.add_action(BufferSearchBar::focus_editor);
    cx.add_action(BufferSearchBar::toggle_search_option);
    cx.add_action(BufferSearchBar::select_next_match);
    cx.add_action(BufferSearchBar::select_prev_match);
    cx.add_action(BufferSearchBar::select_next_match_on_pane);
    cx.add_action(BufferSearchBar::select_prev_match_on_pane);
    cx.add_action(BufferSearchBar::handle_editor_cancel);
}

pub struct BufferSearchBar {
    query_editor: ViewHandle<Editor>,
    active_editor: Option<ViewHandle<Editor>>,
    active_match_index: Option<usize>,
    active_editor_subscription: Option<Subscription>,
    editors_with_matches: HashMap<WeakViewHandle<Editor>, Vec<Range<Anchor>>>,
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

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let editor_container = if self.query_contains_error {
            theme.search.invalid_editor
        } else {
            theme.search.editor.input.container
        };
        Flex::row()
            .with_child(
                Flex::row()
                    .with_child(
                        ChildView::new(&self.query_editor)
                            .aligned()
                            .left()
                            .flex(1., true)
                            .boxed(),
                    )
                    .with_children(self.active_editor.as_ref().and_then(|editor| {
                        let matches = self.editors_with_matches.get(&editor.downgrade())?;
                        let message = if let Some(match_ix) = self.active_match_index {
                            format!("{}/{}", match_ix + 1, matches.len())
                        } else {
                            "No matches".to_string()
                        };

                        Some(
                            Label::new(message, theme.search.match_index.text.clone())
                                .contained()
                                .with_style(theme.search.match_index.container)
                                .aligned()
                                .boxed(),
                        )
                    }))
                    .contained()
                    .with_style(editor_container)
                    .aligned()
                    .constrained()
                    .with_min_width(theme.search.editor.min_width)
                    .with_max_width(theme.search.editor.max_width)
                    .flex(1., false)
                    .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(self.render_nav_button("<", Direction::Prev, cx))
                    .with_child(self.render_nav_button(">", Direction::Next, cx))
                    .aligned()
                    .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(self.render_search_option("Case", SearchOption::CaseSensitive, cx))
                    .with_child(self.render_search_option("Word", SearchOption::WholeWord, cx))
                    .with_child(self.render_search_option("Regex", SearchOption::Regex, cx))
                    .contained()
                    .with_style(theme.search.option_button_group)
                    .aligned()
                    .boxed(),
            )
            .contained()
            .with_style(theme.search.container)
            .named("search bar")
    }
}

impl ToolbarItemView for BufferSearchBar {
    fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_editor_subscription.take();
        self.active_editor.take();
        self.pending_search.take();

        if let Some(editor) = item.and_then(|item| item.act_as::<Editor>(cx)) {
            if editor.read(cx).searchable() {
                self.active_editor_subscription =
                    Some(cx.subscribe(&editor, Self::on_active_editor_event));
                self.active_editor = Some(editor);
                self.update_matches(false, cx);
                if !self.dismissed {
                    return ToolbarItemLocation::Secondary;
                }
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
        if self.active_editor.is_some() && !self.dismissed {
            ToolbarItemLocation::Secondary
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl BufferSearchBar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::auto_height(2, Some(|theme| theme.search.editor.input.clone()), cx)
        });
        cx.subscribe(&query_editor, Self::on_query_editor_event)
            .detach();

        Self {
            query_editor,
            active_editor: None,
            active_editor_subscription: None,
            active_match_index: None,
            editors_with_matches: Default::default(),
            case_sensitive: false,
            whole_word: false,
            regex: false,
            pending_search: None,
            query_contains_error: false,
            dismissed: true,
        }
    }

    fn dismiss(&mut self, _: &Dismiss, cx: &mut ViewContext<Self>) {
        self.dismissed = true;
        for (editor, _) in &self.editors_with_matches {
            if let Some(editor) = editor.upgrade(cx) {
                editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights::<Self>(cx)
                });
            }
        }
        if let Some(active_editor) = self.active_editor.as_ref() {
            cx.focus(active_editor);
        }
        cx.emit(Event::UpdateLocation);
        cx.notify();
    }

    fn show(&mut self, focus: bool, cx: &mut ViewContext<Self>) -> bool {
        let editor = if let Some(editor) = self.active_editor.clone() {
            editor
        } else {
            return false;
        };

        let text = query_suggestion_for_editor(&editor, cx);
        if !text.is_empty() {
            self.set_query(&text, cx);
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
                query_buffer.edit([(0..len, query)], cx);
            });
        });
    }

    fn render_search_option(
        &self,
        icon: &str,
        search_option: SearchOption,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let is_active = self.is_search_option_enabled(search_option);
        MouseEventHandler::new::<Self, _, _>(search_option as usize, cx, |state, cx| {
            let style = &cx
                .global::<Settings>()
                .theme
                .search
                .option_button
                .style_for(state, is_active);
            Label::new(icon.to_string(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(move |_, _, cx| cx.dispatch_action(ToggleSearchOption(search_option)))
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }

    fn render_nav_button(
        &self,
        icon: &str,
        direction: Direction,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        enum NavButton {}
        MouseEventHandler::new::<NavButton, _, _>(direction as usize, cx, |state, cx| {
            let style = &cx
                .global::<Settings>()
                .theme
                .search
                .option_button
                .style_for(state, false);
            Label::new(icon.to_string(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(move |_, _, cx| match direction {
            Direction::Prev => cx.dispatch_action(SelectPrevMatch),
            Direction::Next => cx.dispatch_action(SelectNextMatch),
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }

    fn deploy(pane: &mut Pane, action: &Deploy, cx: &mut ViewContext<Pane>) {
        if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>() {
            if search_bar.update(cx, |search_bar, cx| search_bar.show(action.focus, cx)) {
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
        if let Some(active_editor) = self.active_editor.as_ref() {
            cx.focus(active_editor);
        }
    }

    fn is_search_option_enabled(&self, search_option: SearchOption) -> bool {
        match search_option {
            SearchOption::WholeWord => self.whole_word,
            SearchOption::CaseSensitive => self.case_sensitive,
            SearchOption::Regex => self.regex,
        }
    }

    fn toggle_search_option(
        &mut self,
        ToggleSearchOption(search_option): &ToggleSearchOption,
        cx: &mut ViewContext<Self>,
    ) {
        let value = match search_option {
            SearchOption::WholeWord => &mut self.whole_word,
            SearchOption::CaseSensitive => &mut self.case_sensitive,
            SearchOption::Regex => &mut self.regex,
        };
        *value = !*value;
        self.update_matches(true, cx);
        cx.notify();
    }

    fn select_next_match(&mut self, _: &SelectNextMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Next, cx);
    }

    fn select_prev_match(&mut self, _: &SelectPrevMatch, cx: &mut ViewContext<Self>) {
        self.select_match(Direction::Prev, cx);
    }

    fn select_match(&mut self, direction: Direction, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.active_match_index {
            if let Some(editor) = self.active_editor.as_ref() {
                editor.update(cx, |editor, cx| {
                    if let Some(ranges) = self.editors_with_matches.get(&cx.weak_handle()) {
                        let new_index = match_index_for_direction(
                            ranges,
                            &editor.selections.newest_anchor().head(),
                            index,
                            direction,
                            &editor.buffer().read(cx).snapshot(cx),
                        );
                        let range_to_select = ranges[new_index].clone();
                        editor.unfold_ranges([range_to_select.clone()], false, cx);
                        editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                            s.select_ranges([range_to_select])
                        });
                    }
                });
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

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::BufferEdited { .. } => {
                self.query_contains_error = false;
                self.clear_matches(cx);
                self.update_matches(true, cx);
                cx.notify();
            }
            _ => {}
        }
    }

    fn on_active_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::BufferEdited { .. } => self.update_matches(false, cx),
            editor::Event::SelectionsChanged { .. } => self.update_match_index(cx),
            _ => {}
        }
    }

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        let mut active_editor_matches = None;
        for (editor, ranges) in self.editors_with_matches.drain() {
            if let Some(editor) = editor.upgrade(cx) {
                if Some(&editor) == self.active_editor.as_ref() {
                    active_editor_matches = Some((editor.downgrade(), ranges));
                } else {
                    editor.update(cx, |editor, cx| {
                        editor.clear_background_highlights::<Self>(cx)
                    });
                }
            }
        }
        self.editors_with_matches.extend(active_editor_matches);
    }

    fn update_matches(&mut self, select_closest_match: bool, cx: &mut ViewContext<Self>) {
        let query = self.query_editor.read(cx).text(cx);
        self.pending_search.take();
        if let Some(editor) = self.active_editor.as_ref() {
            if query.is_empty() {
                self.active_match_index.take();
                editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights::<Self>(cx)
                });
            } else {
                let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
                let query = if self.regex {
                    match SearchQuery::regex(query, self.whole_word, self.case_sensitive) {
                        Ok(query) => query,
                        Err(_) => {
                            self.query_contains_error = true;
                            cx.notify();
                            return;
                        }
                    }
                } else {
                    SearchQuery::text(query, self.whole_word, self.case_sensitive)
                };

                let ranges = cx.background().spawn(async move {
                    let mut ranges = Vec::new();
                    if let Some((_, _, excerpt_buffer)) = buffer.as_singleton() {
                        ranges.extend(
                            query
                                .search(excerpt_buffer.as_rope())
                                .await
                                .into_iter()
                                .map(|range| {
                                    buffer.anchor_after(range.start)
                                        ..buffer.anchor_before(range.end)
                                }),
                        );
                    } else {
                        for excerpt in buffer.excerpt_boundaries_in_range(0..buffer.len()) {
                            let excerpt_range = excerpt.range.to_offset(&excerpt.buffer);
                            let rope = excerpt.buffer.as_rope().slice(excerpt_range.clone());
                            ranges.extend(query.search(&rope).await.into_iter().map(|range| {
                                let start = excerpt
                                    .buffer
                                    .anchor_after(excerpt_range.start + range.start);
                                let end = excerpt
                                    .buffer
                                    .anchor_before(excerpt_range.start + range.end);
                                buffer.anchor_in_excerpt(excerpt.id.clone(), start)
                                    ..buffer.anchor_in_excerpt(excerpt.id.clone(), end)
                            }));
                        }
                    }
                    ranges
                });

                let editor = editor.downgrade();
                self.pending_search = Some(cx.spawn_weak(|this, mut cx| async move {
                    let ranges = ranges.await;
                    if let Some((this, editor)) = this.upgrade(&cx).zip(editor.upgrade(&cx)) {
                        this.update(&mut cx, |this, cx| {
                            this.editors_with_matches
                                .insert(editor.downgrade(), ranges.clone());
                            this.update_match_index(cx);
                            if !this.dismissed {
                                editor.update(cx, |editor, cx| {
                                    if select_closest_match {
                                        if let Some(match_ix) = this.active_match_index {
                                            editor.change_selections(
                                                Some(Autoscroll::Fit),
                                                cx,
                                                |s| s.select_ranges([ranges[match_ix].clone()]),
                                            );
                                        }
                                    }

                                    editor.highlight_background::<Self>(
                                        ranges,
                                        |theme| theme.search.match_background,
                                        cx,
                                    );
                                });
                            }
                            cx.notify();
                        });
                    }
                }));
            }
        }
    }

    fn update_match_index(&mut self, cx: &mut ViewContext<Self>) {
        let new_index = self.active_editor.as_ref().and_then(|editor| {
            let ranges = self.editors_with_matches.get(&editor.downgrade())?;
            let editor = editor.read(cx);
            active_match_index(
                &ranges,
                &editor.selections.newest_anchor().head(),
                &editor.buffer().read(cx).snapshot(cx),
            )
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
    use gpui::{color::Color, TestAppContext};
    use language::Buffer;
    use std::sync::Arc;
    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_search_simple(cx: &mut TestAppContext) {
        let fonts = cx.font_cache();
        let mut theme = gpui::fonts::with_font_cache(fonts.clone(), || theme::Theme::default());
        theme.search.match_background = Color::red();
        let settings = Settings::new("Courier", &fonts, Arc::new(theme)).unwrap();
        cx.update(|cx| cx.set_global(settings));

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
        let editor = cx.add_view(Default::default(), |cx| {
            Editor::for_buffer(buffer.clone(), None, cx)
        });

        let search_bar = cx.add_view(Default::default(), |cx| {
            let mut search_bar = BufferSearchBar::new(cx);
            search_bar.set_active_pane_item(Some(&editor), cx);
            search_bar.show(false, cx);
            search_bar
        });

        // Search for a string that appears with different casing.
        // By default, search is case-insensitive.
        search_bar.update(cx, |search_bar, cx| {
            search_bar.set_query("us", cx);
        });
        editor.next_notification(&cx).await;
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
            search_bar.toggle_search_option(&ToggleSearchOption(SearchOption::CaseSensitive), cx);
        });
        editor.next_notification(&cx).await;
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
        editor.next_notification(&cx).await;
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
            search_bar.toggle_search_option(&ToggleSearchOption(SearchOption::WholeWord), cx);
        });
        editor.next_notification(&cx).await;
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
}
