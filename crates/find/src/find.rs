use aho_corasick::AhoCorasickBuilder;
use anyhow::Result;
use collections::HashSet;
use editor::{
    char_kind, display_map::ToDisplayPoint, Anchor, Autoscroll, Bias, Editor, EditorSettings,
    MultiBufferSnapshot,
};
use gpui::{
    action, elements::*, keymap::Binding, platform::CursorStyle, Entity, MutableAppContext,
    RenderContext, Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use postage::watch;
use regex::RegexBuilder;
use smol::future::yield_now;
use std::{
    cmp::{self, Ordering},
    ops::Range,
    sync::Arc,
};
use workspace::{ItemViewHandle, Pane, Settings, Toolbar, Workspace};

action!(Deploy, bool);
action!(Dismiss);
action!(FocusEditor);
action!(ToggleMode, SearchMode);
action!(GoToMatch, Direction);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

#[derive(Clone, Copy)]
pub enum SearchMode {
    WholeWord,
    CaseSensitive,
    Regex,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-f", Deploy(true), Some("Editor && mode == full")),
        Binding::new("cmd-e", Deploy(false), Some("Editor && mode == full")),
        Binding::new("escape", Dismiss, Some("FindBar")),
        Binding::new("cmd-f", FocusEditor, Some("FindBar")),
        Binding::new("enter", GoToMatch(Direction::Next), Some("FindBar")),
        Binding::new("shift-enter", GoToMatch(Direction::Prev), Some("FindBar")),
        Binding::new("cmd-g", GoToMatch(Direction::Next), Some("Pane")),
        Binding::new("cmd-shift-G", GoToMatch(Direction::Prev), Some("Pane")),
    ]);
    cx.add_action(FindBar::deploy);
    cx.add_action(FindBar::dismiss);
    cx.add_action(FindBar::focus_editor);
    cx.add_action(FindBar::toggle_mode);
    cx.add_action(FindBar::go_to_match);
    cx.add_action(FindBar::go_to_match_on_pane);
}

struct FindBar {
    settings: watch::Receiver<Settings>,
    query_editor: ViewHandle<Editor>,
    active_editor: Option<ViewHandle<Editor>>,
    active_match_index: Option<usize>,
    active_editor_subscription: Option<Subscription>,
    highlighted_editors: HashSet<WeakViewHandle<Editor>>,
    pending_search: Option<Task<()>>,
    case_sensitive_mode: bool,
    whole_word_mode: bool,
    regex_mode: bool,
    query_contains_error: bool,
}

impl Entity for FindBar {
    type Event = ();
}

impl View for FindBar {
    fn ui_name() -> &'static str {
        "FindBar"
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme;
        let editor_container = if self.query_contains_error {
            theme.find.invalid_editor
        } else {
            theme.find.editor.input.container
        };
        Flex::row()
            .with_child(
                ChildView::new(&self.query_editor)
                    .contained()
                    .with_style(editor_container)
                    .aligned()
                    .constrained()
                    .with_max_width(theme.find.editor.max_width)
                    .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(self.render_mode_button("Case", SearchMode::CaseSensitive, cx))
                    .with_child(self.render_mode_button("Word", SearchMode::WholeWord, cx))
                    .with_child(self.render_mode_button("Regex", SearchMode::Regex, cx))
                    .contained()
                    .with_style(theme.find.mode_button_group)
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
                let (_, highlighted_ranges) =
                    editor.read(cx).highlighted_ranges_for_type::<Self>()?;
                let message = if let Some(match_ix) = self.active_match_index {
                    format!("{}/{}", match_ix + 1, highlighted_ranges.len())
                } else {
                    "No matches".to_string()
                };

                Some(
                    Label::new(message, theme.find.match_index.text.clone())
                        .contained()
                        .with_style(theme.find.match_index.container)
                        .aligned()
                        .boxed(),
                )
            }))
            .contained()
            .with_style(theme.find.container)
            .constrained()
            .with_height(theme.workspace.toolbar.height)
            .named("find bar")
    }
}

impl Toolbar for FindBar {
    fn active_item_changed(
        &mut self,
        item: Option<Box<dyn ItemViewHandle>>,
        cx: &mut ViewContext<Self>,
    ) -> bool {
        self.active_editor_subscription.take();
        self.active_editor.take();
        self.pending_search.take();

        if let Some(editor) = item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.active_editor_subscription =
                Some(cx.subscribe(&editor, Self::on_active_editor_event));
            self.active_editor = Some(editor);
            self.update_matches(cx);
            true
        } else {
            false
        }
    }

    fn on_dismiss(&mut self, cx: &mut ViewContext<Self>) {
        self.active_editor.take();
        self.active_editor_subscription.take();
        self.active_match_index.take();
        self.pending_search.take();
        self.clear_matches(cx);
    }
}

impl FindBar {
    fn new(settings: watch::Receiver<Settings>, cx: &mut ViewContext<Self>) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::auto_height(
                2,
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            style: settings.theme.find.editor.input.as_editor(),
                            tab_size: settings.tab_size,
                            soft_wrap: editor::SoftWrap::None,
                        }
                    })
                },
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
            highlighted_editors: Default::default(),
            case_sensitive_mode: false,
            whole_word_mode: false,
            regex_mode: false,
            settings,
            pending_search: None,
            query_contains_error: false,
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

    fn render_mode_button(
        &self,
        icon: &str,
        mode: SearchMode,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let theme = &self.settings.borrow().theme.find;
        let is_active = self.is_mode_enabled(mode);
        MouseEventHandler::new::<Self, _, _, _>((cx.view_id(), mode as usize), cx, |state, _| {
            let style = match (is_active, state.hovered) {
                (false, false) => &theme.mode_button,
                (false, true) => &theme.hovered_mode_button,
                (true, false) => &theme.active_mode_button,
                (true, true) => &theme.active_hovered_mode_button,
            };
            Label::new(icon.to_string(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(move |cx| cx.dispatch_action(ToggleMode(mode)))
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }

    fn render_nav_button(
        &self,
        icon: &str,
        direction: Direction,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let theme = &self.settings.borrow().theme.find;
        MouseEventHandler::new::<Self, _, _, _>(
            (cx.view_id(), 10 + direction as usize),
            cx,
            |state, _| {
                let style = if state.hovered {
                    &theme.hovered_mode_button
                } else {
                    &theme.mode_button
                };
                Label::new(icon.to_string(), style.text.clone())
                    .contained()
                    .with_style(style.container)
                    .boxed()
            },
        )
        .on_click(move |cx| cx.dispatch_action(GoToMatch(direction)))
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }

    fn deploy(workspace: &mut Workspace, Deploy(focus): &Deploy, cx: &mut ViewContext<Workspace>) {
        let settings = workspace.settings();
        workspace.active_pane().update(cx, |pane, cx| {
            let findbar_was_visible = pane
                .active_toolbar()
                .map_or(false, |toolbar| toolbar.downcast::<Self>().is_some());

            pane.show_toolbar(cx, |cx| FindBar::new(settings, cx));

            if let Some(find_bar) = pane
                .active_toolbar()
                .and_then(|toolbar| toolbar.downcast::<Self>())
            {
                if !findbar_was_visible {
                    let editor = pane.active_item().unwrap().act_as::<Editor>(cx).unwrap();
                    let display_map = editor
                        .update(cx, |editor, cx| editor.snapshot(cx))
                        .display_snapshot;
                    let selection = editor
                        .read(cx)
                        .newest_selection::<usize>(&display_map.buffer_snapshot);

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
                        find_bar.update(cx, |find_bar, cx| find_bar.set_query(&text, cx));
                    }
                }

                if *focus {
                    if !findbar_was_visible {
                        let query_editor = find_bar.read(cx).query_editor.clone();
                        query_editor.update(cx, |query_editor, cx| {
                            query_editor.select_all(&editor::SelectAll, cx);
                        });
                    }
                    cx.focus(&find_bar);
                }
            }
        });
    }

    fn dismiss(workspace: &mut Workspace, _: &Dismiss, cx: &mut ViewContext<Workspace>) {
        workspace
            .active_pane()
            .update(cx, |pane, cx| pane.dismiss_toolbar(cx));
    }

    fn focus_editor(&mut self, _: &FocusEditor, cx: &mut ViewContext<Self>) {
        if let Some(active_editor) = self.active_editor.as_ref() {
            cx.focus(active_editor);
        }
    }

    fn is_mode_enabled(&self, mode: SearchMode) -> bool {
        match mode {
            SearchMode::WholeWord => self.whole_word_mode,
            SearchMode::CaseSensitive => self.case_sensitive_mode,
            SearchMode::Regex => self.regex_mode,
        }
    }

    fn toggle_mode(&mut self, ToggleMode(mode): &ToggleMode, cx: &mut ViewContext<Self>) {
        let value = match mode {
            SearchMode::WholeWord => &mut self.whole_word_mode,
            SearchMode::CaseSensitive => &mut self.case_sensitive_mode,
            SearchMode::Regex => &mut self.regex_mode,
        };
        *value = !*value;
        self.update_matches(cx);
        cx.notify();
    }

    fn go_to_match(&mut self, GoToMatch(direction): &GoToMatch, cx: &mut ViewContext<Self>) {
        if let Some(mut index) = self.active_match_index {
            if let Some(editor) = self.active_editor.as_ref() {
                editor.update(cx, |editor, cx| {
                    let newest_selection = editor.newest_anchor_selection().cloned();
                    if let Some(((_, ranges), newest_selection)) = editor
                        .highlighted_ranges_for_type::<Self>()
                        .zip(newest_selection)
                    {
                        let position = newest_selection.head();
                        let buffer = editor.buffer().read(cx).read(cx);
                        if ranges[index].start.cmp(&position, &buffer).unwrap().is_gt() {
                            if *direction == Direction::Prev {
                                if index == 0 {
                                    index = ranges.len() - 1;
                                } else {
                                    index -= 1;
                                }
                            }
                        } else if ranges[index].end.cmp(&position, &buffer).unwrap().is_lt() {
                            if *direction == Direction::Next {
                                index = 0;
                            }
                        } else if *direction == Direction::Prev {
                            if index == 0 {
                                index = ranges.len() - 1;
                            } else {
                                index -= 1;
                            }
                        } else if *direction == Direction::Next {
                            if index == ranges.len() - 1 {
                                index = 0
                            } else {
                                index += 1;
                            }
                        }

                        let range_to_select = ranges[index].clone();
                        drop(buffer);
                        editor.select_ranges([range_to_select], Some(Autoscroll::Fit), cx);
                    }
                });
            }
        }
    }

    fn go_to_match_on_pane(pane: &mut Pane, action: &GoToMatch, cx: &mut ViewContext<Pane>) {
        if let Some(find_bar) = pane.toolbar::<FindBar>() {
            find_bar.update(cx, |find_bar, cx| find_bar.go_to_match(action, cx));
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
                self.update_matches(cx);
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
            editor::Event::Edited => self.update_matches(cx),
            editor::Event::SelectionsChanged => self.update_match_index(cx),
            _ => {}
        }
    }

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        for editor in self.highlighted_editors.drain() {
            if let Some(editor) = editor.upgrade(cx) {
                if Some(&editor) != self.active_editor.as_ref() {
                    editor.update(cx, |editor, cx| editor.clear_highlighted_ranges::<Self>(cx));
                }
            }
        }
    }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) {
        let query = self.query_editor.read(cx).text(cx);
        self.pending_search.take();
        if let Some(editor) = self.active_editor.as_ref() {
            if query.is_empty() {
                self.active_match_index.take();
                editor.update(cx, |editor, cx| editor.clear_highlighted_ranges::<Self>(cx));
            } else {
                let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
                let case_sensitive = self.case_sensitive_mode;
                let whole_word = self.whole_word_mode;
                let ranges = if self.regex_mode {
                    cx.background()
                        .spawn(regex_search(buffer, query, case_sensitive, whole_word))
                } else {
                    cx.background().spawn(async move {
                        Ok(search(buffer, query, case_sensitive, whole_word).await)
                    })
                };

                let editor = editor.downgrade();
                self.pending_search = Some(cx.spawn(|this, mut cx| async move {
                    match ranges.await {
                        Ok(ranges) => {
                            if let Some(editor) = cx.read(|cx| editor.upgrade(cx)) {
                                this.update(&mut cx, |this, cx| {
                                    this.highlighted_editors.insert(editor.downgrade());
                                    editor.update(cx, |editor, cx| {
                                        let theme = &this.settings.borrow().theme.find;
                                        editor.highlight_ranges::<Self>(
                                            ranges,
                                            theme.match_background,
                                            cx,
                                        )
                                    });
                                    this.update_match_index(cx);
                                });
                            }
                        }
                        Err(_) => {
                            this.update(&mut cx, |this, cx| {
                                this.query_contains_error = true;
                                cx.notify();
                            });
                        }
                    }
                }));
            }
        }
    }

    fn update_match_index(&mut self, cx: &mut ViewContext<Self>) {
        self.active_match_index = self.active_match_index(cx);
        cx.notify();
    }

    fn active_match_index(&mut self, cx: &mut ViewContext<Self>) -> Option<usize> {
        let editor = self.active_editor.as_ref()?;
        let editor = editor.read(cx);
        let position = editor.newest_anchor_selection()?.head();
        let ranges = editor.highlighted_ranges_for_type::<Self>()?.1;
        if ranges.is_empty() {
            None
        } else {
            let buffer = editor.buffer().read(cx).read(cx);
            match ranges.binary_search_by(|probe| {
                if probe.end.cmp(&position, &*buffer).unwrap().is_lt() {
                    Ordering::Less
                } else if probe.start.cmp(&position, &*buffer).unwrap().is_gt() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }) {
                Ok(i) | Err(i) => Some(cmp::min(i, ranges.len() - 1)),
            }
        }
    }
}

const YIELD_INTERVAL: usize = 20000;

async fn search(
    buffer: MultiBufferSnapshot,
    query: String,
    case_sensitive: bool,
    whole_word: bool,
) -> Vec<Range<Anchor>> {
    let mut ranges = Vec::new();

    let search = AhoCorasickBuilder::new()
        .auto_configure(&[&query])
        .ascii_case_insensitive(!case_sensitive)
        .build(&[&query]);
    for (ix, mat) in search
        .stream_find_iter(buffer.bytes_in_range(0..buffer.len()))
        .enumerate()
    {
        if (ix + 1) % YIELD_INTERVAL == 0 {
            yield_now().await;
        }

        let mat = mat.unwrap();

        if whole_word {
            let prev_kind = buffer.reversed_chars_at(mat.start()).next().map(char_kind);
            let start_kind = char_kind(buffer.chars_at(mat.start()).next().unwrap());
            let end_kind = char_kind(buffer.reversed_chars_at(mat.end()).next().unwrap());
            let next_kind = buffer.chars_at(mat.end()).next().map(char_kind);
            if Some(start_kind) == prev_kind || Some(end_kind) == next_kind {
                continue;
            }
        }

        ranges.push(buffer.anchor_after(mat.start())..buffer.anchor_before(mat.end()));
    }

    ranges
}

async fn regex_search(
    buffer: MultiBufferSnapshot,
    mut query: String,
    case_sensitive: bool,
    whole_word: bool,
) -> Result<Vec<Range<Anchor>>> {
    if whole_word {
        let mut word_query = String::new();
        word_query.push_str("\\b");
        word_query.push_str(&query);
        word_query.push_str("\\b");
        query = word_query;
    }

    let mut ranges = Vec::new();

    if query.contains("\n") || query.contains("\\n") {
        let regex = RegexBuilder::new(&query)
            .case_insensitive(!case_sensitive)
            .multi_line(true)
            .build()?;
        for (ix, mat) in regex.find_iter(&buffer.text()).enumerate() {
            if (ix + 1) % YIELD_INTERVAL == 0 {
                yield_now().await;
            }

            ranges.push(buffer.anchor_after(mat.start())..buffer.anchor_before(mat.end()));
        }
    } else {
        let regex = RegexBuilder::new(&query)
            .case_insensitive(!case_sensitive)
            .build()?;

        let mut line = String::new();
        let mut line_offset = 0;
        for (chunk_ix, chunk) in buffer
            .chunks(0..buffer.len(), None)
            .map(|c| c.text)
            .chain(["\n"])
            .enumerate()
        {
            if (chunk_ix + 1) % YIELD_INTERVAL == 0 {
                yield_now().await;
            }

            for (newline_ix, text) in chunk.split('\n').enumerate() {
                if newline_ix > 0 {
                    for mat in regex.find_iter(&line) {
                        let start = line_offset + mat.start();
                        let end = line_offset + mat.end();
                        ranges.push(buffer.anchor_after(start)..buffer.anchor_before(end));
                    }

                    line_offset += line.len() + 1;
                    line.clear();
                }
                line.push_str(text);
            }
        }
    }

    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{DisplayPoint, Editor, EditorSettings, MultiBuffer};
    use gpui::{color::Color, TestAppContext};
    use std::sync::Arc;
    use unindent::Unindent as _;

    #[gpui::test]
    async fn test_find_simple(mut cx: TestAppContext) {
        let fonts = cx.font_cache();
        let mut theme = gpui::fonts::with_font_cache(fonts.clone(), || theme::Theme::default());
        theme.find.match_background = Color::red();
        let settings = Settings::new("Courier", &fonts, Arc::new(theme)).unwrap();

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
            Editor::new(buffer.clone(), Arc::new(EditorSettings::test), cx)
        });

        let find_bar = cx.add_view(Default::default(), |cx| {
            let mut find_bar = FindBar::new(watch::channel_with(settings).1, cx);
            find_bar.active_item_changed(Some(Box::new(editor.clone())), cx);
            find_bar
        });

        // Search for a string that appears with different casing.
        // By default, search is case-insensitive.
        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.set_query("us", cx);
        });
        editor.next_notification(&cx).await;
        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.all_highlighted_ranges(cx),
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
        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.toggle_mode(&ToggleMode(SearchMode::CaseSensitive), cx);
        });
        editor.next_notification(&cx).await;
        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.all_highlighted_ranges(cx),
                &[(
                    DisplayPoint::new(2, 43)..DisplayPoint::new(2, 45),
                    Color::red(),
                )]
            );
        });

        // Search for a string that appears both as a whole word and
        // within other words. By default, all results are found.
        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.set_query("or", cx);
        });
        editor.next_notification(&cx).await;
        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.all_highlighted_ranges(cx),
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
        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.toggle_mode(&ToggleMode(SearchMode::WholeWord), cx);
        });
        editor.next_notification(&cx).await;
        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.all_highlighted_ranges(cx),
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

        editor.update(&mut cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)], cx);
        });
        find_bar.update(&mut cx, |find_bar, cx| {
            assert_eq!(find_bar.active_match_index, Some(0));
            find_bar.go_to_match(&GoToMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(0));
        });

        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.go_to_match(&GoToMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(1));
        });

        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.go_to_match(&GoToMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(2));
        });

        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.go_to_match(&GoToMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(0));
        });

        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.go_to_match(&GoToMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(2));
        });

        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.go_to_match(&GoToMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(1));
        });

        find_bar.update(&mut cx, |find_bar, cx| {
            find_bar.go_to_match(&GoToMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the previous match selects
        // the closest match to the left.
        editor.update(&mut cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)], cx);
        });
        find_bar.update(&mut cx, |find_bar, cx| {
            assert_eq!(find_bar.active_match_index, Some(1));
            find_bar.go_to_match(&GoToMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(0));
        });

        // Park the cursor in between matches and ensure that going to the next match selects the
        // closest match to the right.
        editor.update(&mut cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)], cx);
        });
        find_bar.update(&mut cx, |find_bar, cx| {
            assert_eq!(find_bar.active_match_index, Some(1));
            find_bar.go_to_match(&GoToMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 11)..DisplayPoint::new(3, 13)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(1));
        });

        // Park the cursor after the last match and ensure that going to the previous match selects
        // the last match.
        editor.update(&mut cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)], cx);
        });
        find_bar.update(&mut cx, |find_bar, cx| {
            assert_eq!(find_bar.active_match_index, Some(2));
            find_bar.go_to_match(&GoToMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(2));
        });

        // Park the cursor after the last match and ensure that going to the next match selects the
        // first match.
        editor.update(&mut cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(3, 60)..DisplayPoint::new(3, 60)], cx);
        });
        find_bar.update(&mut cx, |find_bar, cx| {
            assert_eq!(find_bar.active_match_index, Some(2));
            find_bar.go_to_match(&GoToMatch(Direction::Next), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(0, 41)..DisplayPoint::new(0, 43)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(0));
        });

        // Park the cursor before the first match and ensure that going to the previous match
        // selects the last match.
        editor.update(&mut cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)], cx);
        });
        find_bar.update(&mut cx, |find_bar, cx| {
            assert_eq!(find_bar.active_match_index, Some(0));
            find_bar.go_to_match(&GoToMatch(Direction::Prev), cx);
            assert_eq!(
                editor.update(cx, |editor, cx| editor.selected_display_ranges(cx)),
                [DisplayPoint::new(3, 56)..DisplayPoint::new(3, 58)]
            );
        });
        find_bar.read_with(&cx, |find_bar, _| {
            assert_eq!(find_bar.active_match_index, Some(2));
        });
    }
}
