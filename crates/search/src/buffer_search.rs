use crate::{active_match_index, match_index_for_direction, Direction, SearchOption, SelectMatch};
use collections::HashMap;
use editor::{display_map::ToDisplayPoint, Anchor, Autoscroll, Bias, Editor};
use gpui::{
    action, elements::*, keymap::Binding, platform::CursorStyle, Entity, MutableAppContext,
    RenderContext, Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::OffsetRangeExt;
use postage::watch;
use project::search::SearchQuery;
use std::ops::Range;
use workspace::{ItemViewHandle, Pane, Settings, Toolbar, Workspace};

action!(Deploy, bool);
action!(Dismiss);
action!(FocusEditor);
action!(ToggleSearchOption, SearchOption);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-f", Deploy(true), Some("Editor && mode == full")),
        Binding::new("cmd-e", Deploy(false), Some("Editor && mode == full")),
        Binding::new("escape", Dismiss, Some("SearchBar")),
        Binding::new("cmd-f", FocusEditor, Some("SearchBar")),
        Binding::new("enter", SelectMatch(Direction::Next), Some("SearchBar")),
        Binding::new(
            "shift-enter",
            SelectMatch(Direction::Prev),
            Some("SearchBar"),
        ),
        Binding::new("cmd-g", SelectMatch(Direction::Next), Some("Pane")),
        Binding::new("cmd-shift-G", SelectMatch(Direction::Prev), Some("Pane")),
    ]);
    cx.add_action(SearchBar::deploy);
    cx.add_action(SearchBar::dismiss);
    cx.add_action(SearchBar::focus_editor);
    cx.add_action(SearchBar::toggle_search_option);
    cx.add_action(SearchBar::select_match);
    cx.add_action(SearchBar::select_match_on_pane);
}

struct SearchBar {
    settings: watch::Receiver<Settings>,
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

impl Entity for SearchBar {
    type Event = ();
}

impl View for SearchBar {
    fn ui_name() -> &'static str {
        "SearchBar"
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme;
        let editor_container = if self.query_contains_error {
            theme.search.invalid_editor
        } else {
            theme.search.editor.input.container
        };
        Flex::row()
            .with_child(
                ChildView::new(&self.query_editor)
                    .contained()
                    .with_style(editor_container)
                    .aligned()
                    .constrained()
                    .with_max_width(theme.search.editor.max_width)
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
            .with_child(
                Flex::row()
                    .with_child(self.render_nav_button("<", Direction::Prev, cx))
                    .with_child(self.render_nav_button(">", Direction::Next, cx))
                    .aligned()
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
            .with_style(theme.search.container)
            .constrained()
            .with_height(theme.workspace.toolbar.height)
            .named("search bar")
    }
}

impl Toolbar for SearchBar {
    fn active_item_changed(
        &mut self,
        item: Option<Box<dyn ItemViewHandle>>,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        self.active_editor_subscription.take();
        self.active_editor.take();
        self.pending_search.take();

        if let Some(editor) = item.and_then(|item| item.act_as::<Editor>(cx)) {
            if editor.read(cx).searchable() {
                self.active_editor_subscription =
                    Some(cx.subscribe(&editor, Self::on_active_editor_event));
                self.active_editor = Some(editor);
                self.update_matches(false, cx);
                return true;
            }
        }
        false
    }

    fn on_dismiss(&mut self, cx: &mut ViewContext<Self>) {
        self.dismissed = true;
        for (editor, _) in &self.editors_with_matches {
            if let Some(editor) = editor.upgrade(cx) {
                editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights::<Self>(cx)
                });
            }
        }
    }
}

impl SearchBar {
    fn new(settings: watch::Receiver<Settings>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::auto_height(
                2,
                settings.clone(),
                Some(|theme| theme.search.editor.input.clone()),
                cx,
            )
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
            settings,
            pending_search: None,
            query_contains_error: false,
            dismissed: false,
        }
    }

    fn set_query(&mut self, query: &str, cx: &mut ViewContext<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.buffer().update(cx, |query_buffer, cx| {
                let len = query_buffer.read(cx).len();
                query_buffer.edit([0..len], query, cx);
            });
        });
    }

    fn render_search_option(
        &self,
        icon: &str,
        search_option: SearchOption,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let theme = &self.settings.borrow().theme.search;
        let is_active = self.is_search_option_enabled(search_option);
        MouseEventHandler::new::<Self, _, _>(search_option as usize, cx, |state, _| {
            let style = match (is_active, state.hovered) {
                (false, false) => &theme.option_button,
                (false, true) => &theme.hovered_option_button,
                (true, false) => &theme.active_option_button,
                (true, true) => &theme.active_hovered_option_button,
            };
            Label::new(icon.to_string(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(move |cx| cx.dispatch_action(ToggleSearchOption(search_option)))
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }

    fn render_nav_button(
        &self,
        icon: &str,
        direction: Direction,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let theme = &self.settings.borrow().theme.search;
        enum NavButton {}
        MouseEventHandler::new::<NavButton, _, _>(direction as usize, cx, |state, _| {
            let style = if state.hovered {
                &theme.hovered_option_button
            } else {
                &theme.option_button
            };
            Label::new(icon.to_string(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(move |cx| cx.dispatch_action(SelectMatch(direction)))
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }

    fn deploy(workspace: &mut Workspace, Deploy(focus): &Deploy, cx: &mut ViewContext<Workspace>) {
        let settings = workspace.settings();
        workspace.active_pane().update(cx, |pane, cx| {
            pane.show_toolbar(cx, |cx| SearchBar::new(settings, cx));

            if let Some(search_bar) = pane
                .active_toolbar()
                .and_then(|toolbar| toolbar.downcast::<Self>())
            {
                search_bar.update(cx, |search_bar, _| search_bar.dismissed = false);
                let editor = pane.active_item().unwrap().act_as::<Editor>(cx).unwrap();
                let display_map = editor
                    .update(cx, |editor, cx| editor.snapshot(cx))
                    .display_snapshot;
                let selection = editor
                    .read(cx)
                    .newest_selection_with_snapshot::<usize>(&display_map.buffer_snapshot);

                let mut text: String;
                if selection.start == selection.end {
                    let point = selection.start.to_display_point(&display_map);
                    let range = editor::movement::surrounding_word(&display_map, point);
                    let range = range.start.to_offset(&display_map, Bias::Left)
                        ..range.end.to_offset(&display_map, Bias::Right);
                    text = display_map.buffer_snapshot.text_for_range(range).collect();
                    if text.trim().is_empty() {
                        text = String::new();
                    }
                } else {
                    text = display_map
                        .buffer_snapshot
                        .text_for_range(selection.start..selection.end)
                        .collect();
                }

                if !text.is_empty() {
                    search_bar.update(cx, |search_bar, cx| search_bar.set_query(&text, cx));
                }

                if *focus {
                    let query_editor = search_bar.read(cx).query_editor.clone();
                    query_editor.update(cx, |query_editor, cx| {
                        query_editor.select_all(&editor::SelectAll, cx);
                    });
                    cx.focus(&search_bar);
                }
            } else {
                cx.propagate_action();
            }
        });
    }

    fn dismiss(pane: &mut Pane, _: &Dismiss, cx: &mut ViewContext<Pane>) {
        if pane.toolbar::<SearchBar>().is_some() {
            pane.dismiss_toolbar(cx);
        }
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

    fn select_match(&mut self, &SelectMatch(direction): &SelectMatch, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.active_match_index {
            if let Some(editor) = self.active_editor.as_ref() {
                editor.update(cx, |editor, cx| {
                    if let Some(ranges) = self.editors_with_matches.get(&cx.weak_handle()) {
                        let new_index = match_index_for_direction(
                            ranges,
                            &editor.newest_anchor_selection().head(),
                            index,
                            direction,
                            &editor.buffer().read(cx).read(cx),
                        );
                        editor.select_ranges(
                            [ranges[new_index].clone()],
                            Some(Autoscroll::Fit),
                            cx,
                        );
                    }
                });
            }
        }
    }

    fn select_match_on_pane(pane: &mut Pane, action: &SelectMatch, cx: &mut ViewContext<Pane>) {
        if let Some(search_bar) = pane.toolbar::<SearchBar>() {
            search_bar.update(cx, |search_bar, cx| search_bar.select_match(action, cx));
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Edited => {
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
            editor::Event::Edited => self.update_matches(false, cx),
            editor::Event::SelectionsChanged => self.update_match_index(cx),
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
                                    let theme = &this.settings.borrow().theme.search;

                                    if select_closest_match {
                                        if let Some(match_ix) = this.active_match_index {
                                            editor.select_ranges(
                                                [ranges[match_ix].clone()],
                                                Some(Autoscroll::Fit),
                                                cx,
                                            );
                                        }
                                    }

                                    editor.highlight_background::<Self>(
                                        ranges,
                                        theme.match_background,
                                        cx,
                                    );
                                });
                            }
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
                &editor.newest_anchor_selection().head(),
                &editor.buffer().read(cx).read(cx),
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
    use editor::{DisplayPoint, Editor, MultiBuffer};
    use gpui::{color::Color, TestAppContext};
    use std::sync::Arc;
    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_search_simple(cx: &mut TestAppContext) {
        let fonts = cx.font_cache();
        let mut theme = gpui::fonts::with_font_cache(fonts.clone(), || theme::Theme::default());
        theme.search.match_background = Color::red();
        let settings = Settings::new("Courier", &fonts, Arc::new(theme)).unwrap();
        let settings = watch::channel_with(settings).1;

        let buffer = cx.update(|cx| {
            MultiBuffer::build_simple(
                &r#"
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
            Editor::for_buffer(buffer.clone(), None, settings.clone(), cx)
        });

        let search_bar = cx.add_view(Default::default(), |cx| {
            let mut search_bar = SearchBar::new(settings, cx);
            search_bar.active_item_changed(Some(Box::new(editor.clone())), cx);
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
            editor.select_display_ranges(&[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)], cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_match(&SelectMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_match(&SelectMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_match(&SelectMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_match(&SelectMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_match(&SelectMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_match(&SelectMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        search_bar.update(cx, |search_bar, cx| {
            search_bar.select_match(&SelectMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the previous match selects
        // the closest match to the left.
        editor.update(cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)], cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_match(&SelectMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the next match selects the
        // closest match to the right.
        editor.update(cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)], cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(1));
            search_bar.select_match(&SelectMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(1));
        });

        // Park the cursor after the last match and ensure that going to the previous match selects
        // the last match.
        editor.update(cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)], cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_match(&SelectMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });

        // Park the cursor after the last match and ensure that going to the next match selects the
        // first match.
        editor.update(cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)], cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(2));
            search_bar.select_match(&SelectMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(0));
        });

        // Park the cursor before the first match and ensure that going to the previous match
        // selects the last match.
        editor.update(cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)], cx);
        });
        search_bar.update(cx, |search_bar, cx| {
            assert_eq!(search_bar.active_match_index, Some(0));
            search_bar.select_match(&SelectMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        search_bar.read_with(cx, |search_bar, _| {
            assert_eq!(search_bar.active_match_index, Some(2));
        });
    }
}
