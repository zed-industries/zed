use editor::{
    display_map::ToDisplayPoint, Anchor, AnchorRangeExt, Autoscroll, DisplayPoint, Editor,
    EditorSettings, ToPoint,
};
use fuzzy::StringMatch;
use gpui::{
    action,
    elements::*,
    fonts::{self, HighlightStyle},
    geometry::vector::Vector2F,
    keymap::{self, Binding},
    AppContext, Axis, Entity, MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use language::Outline;
use ordered_float::OrderedFloat;
use postage::watch;
use std::{
    cmp::{self, Reverse},
    ops::Range,
    sync::Arc,
};
use workspace::{
    menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrev},
    Settings, Workspace,
};

action!(Toggle);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-shift-O", Toggle, Some("Editor")),
        Binding::new("escape", Toggle, Some("OutlineView")),
    ]);
    cx.add_action(OutlineView::toggle);
    cx.add_action(OutlineView::confirm);
    cx.add_action(OutlineView::select_prev);
    cx.add_action(OutlineView::select_next);
    cx.add_action(OutlineView::select_first);
    cx.add_action(OutlineView::select_last);
}

struct OutlineView {
    handle: WeakViewHandle<Self>,
    active_editor: ViewHandle<Editor>,
    outline: Outline<Anchor>,
    selected_match_index: usize,
    prev_scroll_position: Option<Vector2F>,
    matches: Vec<StringMatch>,
    query_editor: ViewHandle<Editor>,
    list_state: UniformListState,
    settings: watch::Receiver<Settings>,
}

pub enum Event {
    Dismissed,
}

impl Entity for OutlineView {
    type Event = Event;

    fn release(&mut self, cx: &mut MutableAppContext) {
        self.restore_active_editor(cx);
    }
}

impl View for OutlineView {
    fn ui_name() -> &'static str {
        "OutlineView"
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        let settings = self.settings.borrow();

        Flex::new(Axis::Vertical)
            .with_child(
                Container::new(ChildView::new(&self.query_editor).boxed())
                    .with_style(settings.theme.selector.input_editor.container)
                    .boxed(),
            )
            .with_child(Flexible::new(1.0, false, self.render_matches()).boxed())
            .contained()
            .with_style(settings.theme.selector.container)
            .constrained()
            .with_max_width(800.0)
            .with_max_height(1200.0)
            .aligned()
            .top()
            .named("outline view")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }
}

impl OutlineView {
    fn new(
        outline: Outline<Anchor>,
        editor: ViewHandle<Editor>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let query_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            style: settings.theme.selector.input_editor.as_editor(),
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

        let mut this = Self {
            handle: cx.weak_handle(),
            matches: Default::default(),
            selected_match_index: 0,
            prev_scroll_position: Some(editor.update(cx, |editor, cx| editor.scroll_position(cx))),
            active_editor: editor,
            outline,
            query_editor,
            list_state: Default::default(),
            settings,
        };
        this.update_matches(cx);
        this
    }

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        if let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.downcast::<Editor>())
        {
            let settings = workspace.settings();
            let buffer = editor
                .read(cx)
                .buffer()
                .read(cx)
                .read(cx)
                .outline(Some(settings.borrow().theme.editor.syntax.as_ref()));
            if let Some(outline) = buffer {
                workspace.toggle_modal(cx, |cx, _| {
                    let view = cx.add_view(|cx| OutlineView::new(outline, editor, settings, cx));
                    cx.subscribe(&view, Self::on_event).detach();
                    view
                })
            }
        }
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if self.selected_match_index > 0 {
            self.select(self.selected_match_index - 1, true, false, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if self.selected_match_index + 1 < self.matches.len() {
            self.select(self.selected_match_index + 1, true, false, cx);
        }
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        self.select(0, true, false, cx);
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        self.select(self.matches.len().saturating_sub(1), true, false, cx);
    }

    fn select(&mut self, index: usize, navigate: bool, center: bool, cx: &mut ViewContext<Self>) {
        self.selected_match_index = index;
        self.list_state.scroll_to(if center {
            ScrollTarget::Center(index)
        } else {
            ScrollTarget::Show(index)
        });
        if navigate {
            let selected_match = &self.matches[self.selected_match_index];
            let outline_item = &self.outline.items[selected_match.candidate_id];
            self.active_editor.update(cx, |active_editor, cx| {
                let snapshot = active_editor.snapshot(cx).display_snapshot;
                let buffer_snapshot = &snapshot.buffer_snapshot;
                let start = outline_item.range.start.to_point(&buffer_snapshot);
                let end = outline_item.range.end.to_point(&buffer_snapshot);
                let display_rows = start.to_display_point(&snapshot).row()
                    ..end.to_display_point(&snapshot).row() + 1;
                active_editor.set_highlighted_rows(Some(display_rows));
                active_editor.request_autoscroll(Autoscroll::Center, cx);
            });
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        self.prev_scroll_position.take();
        self.active_editor.update(cx, |active_editor, cx| {
            if let Some(rows) = active_editor.highlighted_rows() {
                let snapshot = active_editor.snapshot(cx).display_snapshot;
                let position = DisplayPoint::new(rows.start, 0).to_point(&snapshot);
                active_editor.select_ranges([position..position], Some(Autoscroll::Center), cx);
            }
        });
        cx.emit(Event::Dismissed);
    }

    fn restore_active_editor(&mut self, cx: &mut MutableAppContext) {
        self.active_editor.update(cx, |editor, cx| {
            editor.set_highlighted_rows(None);
            if let Some(scroll_position) = self.prev_scroll_position {
                editor.set_scroll_position(scroll_position, cx);
            }
        })
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<Self>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Dismissed => workspace.dismiss_modal(cx),
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            editor::Event::Edited => self.update_matches(cx),
            _ => {}
        }
    }

    fn update_matches(&mut self, cx: &mut ViewContext<Self>) {
        let selected_index;
        let navigate_to_selected_index;
        let query = self.query_editor.update(cx, |buffer, cx| buffer.text(cx));
        if query.is_empty() {
            self.restore_active_editor(cx);
            self.matches = self
                .outline
                .items
                .iter()
                .enumerate()
                .map(|(index, _)| StringMatch {
                    candidate_id: index,
                    score: Default::default(),
                    positions: Default::default(),
                    string: Default::default(),
                })
                .collect();

            let editor = self.active_editor.read(cx);
            let buffer = editor.buffer().read(cx).read(cx);
            let cursor_offset = editor.newest_selection::<usize>(&buffer).head();
            selected_index = self
                .outline
                .items
                .iter()
                .enumerate()
                .map(|(ix, item)| {
                    let range = item.range.to_offset(&buffer);
                    let distance_to_closest_endpoint = cmp::min(
                        (range.start as isize - cursor_offset as isize).abs() as usize,
                        (range.end as isize - cursor_offset as isize).abs() as usize,
                    );
                    let depth = if range.contains(&cursor_offset) {
                        Some(item.depth)
                    } else {
                        None
                    };
                    (ix, depth, distance_to_closest_endpoint)
                })
                .max_by_key(|(_, depth, distance)| (*depth, Reverse(*distance)))
                .unwrap()
                .0;
            navigate_to_selected_index = false;
        } else {
            self.matches = smol::block_on(self.outline.search(&query, cx.background().clone()));
            selected_index = self
                .matches
                .iter()
                .enumerate()
                .max_by_key(|(_, m)| OrderedFloat(m.score))
                .map(|(ix, _)| ix)
                .unwrap_or(0);
            navigate_to_selected_index = !self.matches.is_empty();
        }
        self.select(selected_index, navigate_to_selected_index, true, cx);
    }

    fn render_matches(&self) -> ElementBox {
        if self.matches.is_empty() {
            let settings = self.settings.borrow();
            return Container::new(
                Label::new(
                    "No matches".into(),
                    settings.theme.selector.empty.label.clone(),
                )
                .boxed(),
            )
            .with_style(settings.theme.selector.empty.container)
            .named("empty matches");
        }

        let handle = self.handle.clone();
        let list = UniformList::new(
            self.list_state.clone(),
            self.matches.len(),
            move |mut range, items, cx| {
                let cx = cx.as_ref();
                let view = handle.upgrade(cx).unwrap();
                let view = view.read(cx);
                let start = range.start;
                range.end = cmp::min(range.end, view.matches.len());
                items.extend(
                    view.matches[range]
                        .iter()
                        .enumerate()
                        .map(move |(ix, m)| view.render_match(m, start + ix)),
                );
            },
        );

        Container::new(list.boxed())
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(&self, string_match: &StringMatch, index: usize) -> ElementBox {
        let settings = self.settings.borrow();
        let style = if index == self.selected_match_index {
            &settings.theme.selector.active_item
        } else {
            &settings.theme.selector.item
        };
        let outline_item = &self.outline.items[string_match.candidate_id];

        Text::new(outline_item.text.clone(), style.label.text.clone())
            .with_soft_wrap(false)
            .with_highlights(combine_syntax_and_fuzzy_match_highlights(
                &outline_item.text,
                style.label.text.clone().into(),
                &outline_item.highlight_ranges,
                &string_match.positions,
            ))
            .contained()
            .with_padding_left(20. * outline_item.depth as f32)
            .contained()
            .with_style(style.container)
            .boxed()
    }
}

fn combine_syntax_and_fuzzy_match_highlights(
    text: &str,
    default_style: HighlightStyle,
    syntax_ranges: &[(Range<usize>, HighlightStyle)],
    match_indices: &[usize],
) -> Vec<(Range<usize>, HighlightStyle)> {
    let mut result = Vec::new();
    let mut match_indices = match_indices.iter().copied().peekable();

    for (range, mut syntax_highlight) in syntax_ranges
        .iter()
        .cloned()
        .chain([(usize::MAX..0, Default::default())])
    {
        syntax_highlight.font_properties.weight(Default::default());

        // Add highlights for any fuzzy match characters before the next
        // syntax highlight range.
        while let Some(&match_index) = match_indices.peek() {
            if match_index >= range.start {
                break;
            }
            match_indices.next();
            let end_index = char_ix_after(match_index, text);
            let mut match_style = default_style;
            match_style.font_properties.weight(fonts::Weight::BOLD);
            result.push((match_index..end_index, match_style));
        }

        if range.start == usize::MAX {
            break;
        }

        // Add highlights for any fuzzy match characters within the
        // syntax highlight range.
        let mut offset = range.start;
        while let Some(&match_index) = match_indices.peek() {
            if match_index >= range.end {
                break;
            }

            match_indices.next();
            if match_index > offset {
                result.push((offset..match_index, syntax_highlight));
            }

            let mut end_index = char_ix_after(match_index, text);
            while let Some(&next_match_index) = match_indices.peek() {
                if next_match_index == end_index && next_match_index < range.end {
                    end_index = char_ix_after(next_match_index, text);
                    match_indices.next();
                } else {
                    break;
                }
            }

            let mut match_style = syntax_highlight;
            match_style.font_properties.weight(fonts::Weight::BOLD);
            result.push((match_index..end_index, match_style));
            offset = end_index;
        }

        if offset < range.end {
            result.push((offset..range.end, syntax_highlight));
        }
    }

    result
}

fn char_ix_after(ix: usize, text: &str) -> usize {
    ix + text[ix..].chars().next().unwrap().len_utf8()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{color::Color, fonts::HighlightStyle};

    #[test]
    fn test_combine_syntax_and_fuzzy_match_highlights() {
        let string = "abcdefghijklmnop";
        let default = HighlightStyle::default();
        let syntax_ranges = [
            (
                0..3,
                HighlightStyle {
                    color: Color::red(),
                    ..default
                },
            ),
            (
                4..8,
                HighlightStyle {
                    color: Color::green(),
                    ..default
                },
            ),
        ];
        let match_indices = [4, 6, 7, 8];
        assert_eq!(
            combine_syntax_and_fuzzy_match_highlights(
                &string,
                default,
                &syntax_ranges,
                &match_indices,
            ),
            &[
                (
                    0..3,
                    HighlightStyle {
                        color: Color::red(),
                        ..default
                    },
                ),
                (
                    4..5,
                    HighlightStyle {
                        color: Color::green(),
                        font_properties: *fonts::Properties::default().weight(fonts::Weight::BOLD),
                        ..default
                    },
                ),
                (
                    5..6,
                    HighlightStyle {
                        color: Color::green(),
                        ..default
                    },
                ),
                (
                    6..8,
                    HighlightStyle {
                        color: Color::green(),
                        font_properties: *fonts::Properties::default().weight(fonts::Weight::BOLD),
                        ..default
                    },
                ),
                (
                    8..9,
                    HighlightStyle {
                        font_properties: *fonts::Properties::default().weight(fonts::Weight::BOLD),
                        ..default
                    },
                ),
            ]
        );
    }
}
