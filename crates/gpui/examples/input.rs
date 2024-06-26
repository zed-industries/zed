use std::ops::Range;

use gpui::*;
use unicode_segmentation::*;

actions!(
    text_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        Home,
        End,
        ShowCharacterPalette
    ]
);

struct TextInput {
    focus_handle: FocusHandle,
    content: SharedString,
    selected_range: Range<usize>,
    marked_range: Option<Range<usize>>, // New field for marked text range
    last_layout: Option<ShapedLine>,
}

impl TextInput {
    fn left(&mut self, _: &Left, cx: &mut ViewContext<Self>) {
        let new_pos = self.previous_grapheme_boundary(self.selected_range.end);
        self.selected_range = new_pos..new_pos;
        cx.notify()
    }

    fn right(&mut self, _: &Right, cx: &mut ViewContext<Self>) {
        let new_pos = self.next_grapheme_boundary(self.selected_range.end);
        self.selected_range = new_pos..new_pos;
        cx.notify()
    }

    fn home(&mut self, _: &Home, cx: &mut ViewContext<Self>) {
        self.selected_range = 0..0;
        cx.notify()
    }

    fn end(&mut self, _: &End, cx: &mut ViewContext<Self>) {
        self.selected_range = self.content.len()..self.content.len();
        cx.notify()
    }

    fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        if self.selected_range.start == self.selected_range.end {
            self.selected_range.start = self.previous_grapheme_boundary(self.selected_range.start);
        }
        self.replace_text_in_range(None, "", cx)
    }

    fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        if self.selected_range.start == self.selected_range.end {
            self.selected_range.end = self.next_grapheme_boundary(self.selected_range.end);
        }
        self.replace_text_in_range(None, "", cx)
    }

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, cx: &mut ViewContext<Self>) {
        cx.show_character_palette();
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_grapheme_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .skip_while(|(idx, _)| idx >= &offset)
            .next()
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    fn next_grapheme_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .skip_while(|(idx, _)| idx <= &offset)
            .next()
            .map(|(idx, _)| idx)
            .unwrap_or(self.content.len())
    }
}

impl ViewInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _cx: &mut ViewContext<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        if range.start <= range.end && range.end <= self.content.len() {
            Some(self.content[range].to_string())
        } else {
            None
        }
    }

    fn selected_text_range(&mut self, _cx: &mut ViewContext<Self>) -> Option<Range<usize>> {
        Some(self.range_to_utf16(&self.selected_range))
    }

    fn marked_text_range(&self, _cx: &mut ViewContext<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _cx: &mut ViewContext<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        cx: &mut ViewContext<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        cx: &mut ViewContext<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.marked_range = Some(range.start..range.start + new_text.len());
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _cx: &mut ViewContext<Self>,
    ) -> Option<Bounds<Pixels>> {
        let Some(last_layout) = self.last_layout.as_ref() else {
            return None;
        };
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }
}

struct TextElement {
    input: View<TextInput>,
}

struct PrepaintState {
    line: ShapedLine,
    focus_handle: FocusHandle,
    cursor: PaintQuad,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();

    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = cx.line_height().into();
        (cx.request_layout(style, []), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let content = input.content.clone();
        let focus_handle = input.focus_handle.clone();
        let selection = input.selected_range.clone();
        let style = cx.text_style();
        let run = TextRun {
            len: input.content.len(),
            font: style.font(),
            color: style.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if let Some(marked_range) = input.marked_range.as_ref() {
            vec![
                TextRun {
                    len: marked_range.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: input.content.len() - marked_range.end,
                    ..run.clone()
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(cx.rem_size());
        let line = cx
            .text_system()
            .shape_line(content, font_size, &runs)
            .unwrap();

        let cursor_pos = line.x_for_index(selection.end);
        let cursor = fill(
            Bounds::new(
                point(bounds.left() + cursor_pos, bounds.top()),
                size(px(2.), bounds.bottom() - bounds.top()),
            ),
            gpui::blue(),
        );
        PrepaintState {
            line,
            focus_handle,
            cursor,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        cx.handle_input(
            &prepaint.focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
        );
        prepaint
            .line
            .paint(bounds.origin, cx.line_height(), cx)
            .unwrap();
        cx.paint_quad(prepaint.cursor.clone());
        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(prepaint.line.clone());
        });
    }
}

impl Render for TextInput {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .key_context("TextInput")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::show_character_palette))
            .bg(rgb(0xeeeeee))
            .size_full()
            .line_height(px(30.))
            .text_size(px(24.))
            .child(
                div()
                    .h(px(30. + 4. * 2.))
                    .w_full()
                    .p(px(4.))
                    .bg(white())
                    .child(TextElement {
                        input: cx.view().clone(),
                    }),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        ]);
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |cx| {
                    cx.new_view(|cx| TextInput {
                        focus_handle: cx.focus_handle(),
                        content: "".into(),
                        selected_range: 0..0,
                        marked_range: None,
                        last_layout: None,
                    })
                },
            )
            .unwrap();
        window
            .update(cx, |view, cx| {
                view.focus_handle.focus(cx);
                cx.activate(true)
            })
            .unwrap();
    });
}
