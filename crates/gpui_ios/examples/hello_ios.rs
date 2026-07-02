//! Touch-input demo for GPUI on iOS: tap counters, move tracking, hover
//! lifecycle, pan-to-scroll with momentum, pinch-to-zoom, hardware keyboard
//! input, and a text field driving the software keyboard and IME, all driven
//! by the touch → pointer compatibility shim and drawn by the Metal renderer.

#[cfg(target_os = "ios")]
mod example {
    use gpui::{
        App, Application, Bounds, ClickEvent, Context, ElementId, ElementInputHandler, Entity,
        EntityInputHandler, FocusHandle, GlobalElementId, InspectorElementId, KeyBinding,
        KeyDownEvent, Keystroke, LayoutId, MouseMoveEvent, PaintQuad, PinchEvent, Pixels, Point,
        Rgba, ShapedLine, SharedString, Style, TextRun, UTF16Selection, UnderlineStyle, Window,
        WindowOptions, actions, div, fill, point, prelude::*, px, relative, rgb, rgba, size,
    };
    use std::{ops::Range, rc::Rc};

    actions!(hello_ios, [Backspace, Left, Right]);

    /// A deliberately minimal single-line text field exercising gpui's
    /// text-input bridge: no selection dragging, no clipboard, no blinking.
    struct TextField {
        focus_handle: FocusHandle,
        content: SharedString,
        /// Selected byte range in `content`; empty means a caret.
        selected_range: Range<usize>,
        /// Byte range of in-progress IME composition in `content`.
        marked_range: Option<Range<usize>>,
        last_layout: Option<ShapedLine>,
        last_bounds: Option<Bounds<Pixels>>,
    }

    impl TextField {
        fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
            if self.selected_range.is_empty() {
                let previous = self.previous_boundary(self.selected_range.start);
                self.selected_range.start = previous;
            }
            self.replace_text_in_range(None, "", window, cx);
        }

        fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
            let offset = if self.selected_range.is_empty() {
                self.previous_boundary(self.selected_range.start)
            } else {
                self.selected_range.start
            };
            self.selected_range = offset..offset;
            cx.notify();
        }

        fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
            let offset = if self.selected_range.is_empty() {
                self.next_boundary(self.selected_range.end)
            } else {
                self.selected_range.end
            };
            self.selected_range = offset..offset;
            cx.notify();
        }

        fn on_tap(&mut self, event: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
            window.focus(&self.focus_handle, cx);
            let offset = self.index_for_position(event.position());
            self.selected_range = offset..offset;
            cx.notify();
        }

        fn index_for_position(&self, position: Point<Pixels>) -> usize {
            let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
            else {
                return self.content.len();
            };
            line.closest_index_for_x(position.x - bounds.left())
        }

        fn previous_boundary(&self, offset: usize) -> usize {
            self.content[..offset]
                .char_indices()
                .next_back()
                .map_or(0, |(index, _)| index)
        }

        fn next_boundary(&self, offset: usize) -> usize {
            self.content[offset..]
                .chars()
                .next()
                .map_or(self.content.len(), |character| {
                    offset + character.len_utf8()
                })
        }

        fn offset_from_utf16(&self, offset_utf16: usize) -> usize {
            let mut utf8_offset = 0;
            let mut utf16_count = 0;
            for character in self.content.chars() {
                if utf16_count >= offset_utf16 {
                    break;
                }
                utf16_count += character.len_utf16();
                utf8_offset += character.len_utf8();
            }
            utf8_offset
        }

        fn offset_to_utf16(&self, offset: usize) -> usize {
            let mut utf16_offset = 0;
            let mut utf8_count = 0;
            for character in self.content.chars() {
                if utf8_count >= offset {
                    break;
                }
                utf8_count += character.len_utf8();
                utf16_offset += character.len_utf16();
            }
            utf16_offset
        }

        fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
            self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
        }

        fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
            self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
        }
    }

    impl EntityInputHandler for TextField {
        fn text_for_range(
            &mut self,
            range_utf16: Range<usize>,
            adjusted_range: &mut Option<Range<usize>>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<String> {
            let range = self.range_from_utf16(&range_utf16);
            adjusted_range.replace(self.range_to_utf16(&range));
            Some(self.content[range].to_string())
        }

        fn selected_text_range(
            &mut self,
            _ignore_disabled_input: bool,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<UTF16Selection> {
            Some(UTF16Selection {
                range: self.range_to_utf16(&self.selected_range),
                reversed: false,
            })
        }

        fn marked_text_range(
            &self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<Range<usize>> {
            self.marked_range
                .as_ref()
                .map(|range| self.range_to_utf16(range))
        }

        fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
            self.marked_range = None;
        }

        fn replace_text_in_range(
            &mut self,
            range_utf16: Option<Range<usize>>,
            new_text: &str,
            _window: &mut Window,
            cx: &mut Context<Self>,
        ) {
            let range = range_utf16
                .as_ref()
                .map(|range_utf16| self.range_from_utf16(range_utf16))
                .or(self.marked_range.clone())
                .unwrap_or(self.selected_range.clone());
            self.content =
                (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                    .into();
            let cursor = range.start + new_text.len();
            self.selected_range = cursor..cursor;
            self.marked_range = None;
            cx.notify();
        }

        fn replace_and_mark_text_in_range(
            &mut self,
            range_utf16: Option<Range<usize>>,
            new_text: &str,
            new_selected_range_utf16: Option<Range<usize>>,
            _window: &mut Window,
            cx: &mut Context<Self>,
        ) {
            let range = range_utf16
                .as_ref()
                .map(|range_utf16| self.range_from_utf16(range_utf16))
                .or(self.marked_range.clone())
                .unwrap_or(self.selected_range.clone());
            self.content =
                (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                    .into();
            self.marked_range = if new_text.is_empty() {
                None
            } else {
                Some(range.start..range.start + new_text.len())
            };
            // The new selection is expressed relative to the marked text.
            self.selected_range = new_selected_range_utf16
                .as_ref()
                .map(|range_utf16| self.range_from_utf16(range_utf16))
                .map(|new_range| range.start + new_range.start..range.start + new_range.end)
                .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());
            cx.notify();
        }

        fn bounds_for_range(
            &mut self,
            range_utf16: Range<usize>,
            element_bounds: Bounds<Pixels>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<Bounds<Pixels>> {
            let layout = self.last_layout.as_ref()?;
            let range = self.range_from_utf16(&range_utf16);
            Some(Bounds::from_corners(
                point(
                    element_bounds.left() + layout.x_for_index(range.start),
                    element_bounds.top(),
                ),
                point(
                    element_bounds.left() + layout.x_for_index(range.end),
                    element_bounds.bottom(),
                ),
            ))
        }

        fn character_index_for_point(
            &mut self,
            position: Point<Pixels>,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> Option<usize> {
            let bounds = self.last_bounds.as_ref()?;
            let layout = self.last_layout.as_ref()?;
            let utf8_index = layout.index_for_x(position.x - bounds.left())?;
            Some(self.offset_to_utf16(utf8_index))
        }
    }

    impl Render for TextField {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("text-field")
                .key_context("TextField")
                .track_focus(&self.focus_handle)
                .on_action(cx.listener(Self::backspace))
                .on_action(cx.listener(Self::left))
                .on_action(cx.listener(Self::right))
                .on_click(cx.listener(Self::on_tap))
                .w(px(220.))
                .h(px(40.))
                .px_2()
                .py_1()
                .bg(gpui::white())
                .rounded_md()
                .border_2()
                .border_color(if self.focus_handle.is_focused(window) {
                    rgb(0x3b82f6)
                } else {
                    rgb(0x6b7280)
                })
                .text_lg()
                .text_color(rgb(0x111827))
                .child(TextFieldElement { field: cx.entity() })
        }
    }

    /// Custom element so painting can register the field as the window's
    /// input handler and draw the caret and marked-text underline.
    struct TextFieldElement {
        field: Entity<TextField>,
    }

    struct TextFieldPrepaintState {
        line: ShapedLine,
        cursor: Option<PaintQuad>,
        selection: Option<PaintQuad>,
    }

    impl IntoElement for TextFieldElement {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }

    impl gpui::Element for TextFieldElement {
        type RequestLayoutState = ();
        type PrepaintState = TextFieldPrepaintState;

        fn id(&self) -> Option<ElementId> {
            None
        }

        fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
            None
        }

        fn request_layout(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            window: &mut Window,
            cx: &mut gpui::App,
        ) -> (LayoutId, Self::RequestLayoutState) {
            let mut style = Style::default();
            style.size.width = relative(1.).into();
            style.size.height = window.line_height().into();
            (window.request_layout(style, [], cx), ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            window: &mut Window,
            cx: &mut gpui::App,
        ) -> Self::PrepaintState {
            let field = self.field.read(cx);
            let content = field.content.clone();
            let selected_range = field.selected_range.clone();
            let marked_range = field.marked_range.clone();
            let style = window.text_style();

            let run = TextRun {
                len: content.len(),
                font: style.font(),
                color: style.color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let runs = if let Some(marked_range) = marked_range.as_ref() {
                vec![
                    TextRun {
                        len: marked_range.start,
                        ..run.clone()
                    },
                    TextRun {
                        len: marked_range.len(),
                        color: gpui::blue(),
                        underline: Some(UnderlineStyle {
                            color: Some(gpui::blue()),
                            thickness: px(2.),
                            wavy: false,
                        }),
                        ..run.clone()
                    },
                    TextRun {
                        len: content.len() - marked_range.end,
                        ..run
                    },
                ]
                .into_iter()
                .filter(|run| run.len > 0)
                .collect()
            } else {
                vec![run]
            };

            let font_size = style.font_size.to_pixels(window.rem_size());
            let line = window
                .text_system()
                .shape_line(content, font_size, &runs, None);

            let (selection, cursor) = if selected_range.is_empty() {
                let cursor_x = line.x_for_index(selected_range.start);
                (
                    None,
                    Some(fill(
                        Bounds::new(
                            point(bounds.left() + cursor_x, bounds.top()),
                            size(px(2.), bounds.size.height),
                        ),
                        gpui::blue(),
                    )),
                )
            } else {
                (
                    Some(fill(
                        Bounds::from_corners(
                            point(
                                bounds.left() + line.x_for_index(selected_range.start),
                                bounds.top(),
                            ),
                            point(
                                bounds.left() + line.x_for_index(selected_range.end),
                                bounds.bottom(),
                            ),
                        ),
                        rgba(0x3b82f640),
                    )),
                    None,
                )
            };
            TextFieldPrepaintState {
                line,
                cursor,
                selection,
            }
        }

        fn paint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&InspectorElementId>,
            bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            prepaint: &mut Self::PrepaintState,
            window: &mut Window,
            cx: &mut gpui::App,
        ) {
            let focus_handle = self.field.read(cx).focus_handle.clone();
            window.handle_input(
                &focus_handle,
                ElementInputHandler::new(bounds, self.field.clone()),
                cx,
            );
            if let Some(selection) = prepaint.selection.take() {
                window.paint_quad(selection);
            }
            let line = std::mem::take(&mut prepaint.line);
            let line_paint = line.paint(
                bounds.origin,
                window.line_height(),
                gpui::TextAlign::Left,
                None,
                window,
                cx,
            );
            if let Err(error) = line_paint {
                eprintln!("failed to paint text field line: {error}");
            }
            if focus_handle.is_focused(window)
                && let Some(cursor) = prepaint.cursor.take()
            {
                window.paint_quad(cursor);
            }
            self.field.update(cx, |field, _cx| {
                field.last_layout = Some(line);
                field.last_bounds = Some(bounds);
            });
        }
    }

    struct HelloIos {
        focus_handle: FocusHandle,
        text_field: Entity<TextField>,
        tap_counts: [usize; 3],
        row_taps: usize,
        pinch_scale: f32,
        last_touch: Option<Point<Pixels>>,
        last_keystroke: Option<Keystroke>,
        key_down_count: usize,
    }

    impl HelloIos {
        fn tap_counter_box(
            &self,
            index: usize,
            background: Rgba,
            hover_background: Rgba,
            cx: &mut Context<Self>,
        ) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .id(index)
                        .size_16()
                        .bg(background)
                        .rounded_md()
                        .border_2()
                        .border_color(rgb(0xffffff))
                        .hover(|style| {
                            style
                                .bg(hover_background)
                                .border_4()
                                .border_color(rgb(0xfbbf24))
                        })
                        .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                            this.tap_counts[index] += 1;
                            cx.notify();
                        })),
                )
                .child(self.tap_counts[index].to_string())
        }

        fn row_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("row-list")
                .h(px(300.))
                .w(px(320.))
                .rounded_lg()
                .border_2()
                .border_color(rgb(0x6b7280))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .text_lg()
                .children((0..40usize).map(|index| {
                    let background = if index % 2 == 0 {
                        rgb(0x334155)
                    } else {
                        rgb(0x475569)
                    };
                    div()
                        .id(("row", index))
                        .flex_none()
                        .h(px(32.))
                        .px_4()
                        .bg(background)
                        .child(format!("Row {index}"))
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                            this.row_taps += 1;
                            cx.notify();
                        }))
                }))
        }
    }

    impl Render for HelloIos {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let last_touch = self.last_touch.map_or_else(
                || "Last touch: none".to_string(),
                |position| {
                    format!(
                        "Last touch: ({:.0}, {:.0})",
                        f32::from(position.x),
                        f32::from(position.y)
                    )
                },
            );

            let last_key = self.last_keystroke.as_ref().map_or_else(
                || "Last key: none".to_string(),
                |keystroke| {
                    // `Keystroke::unparse` emits no platform-modifier prefix
                    // on iOS, so render cmd- ourselves.
                    let command_prefix = if keystroke.modifiers.platform {
                        "cmd-"
                    } else {
                        ""
                    };
                    format!(
                        "Last key: {}{} ({} downs)",
                        command_prefix,
                        keystroke.unparse(),
                        self.key_down_count
                    )
                },
            );

            div()
                .size_full()
                .bg(rgb(0x1f2937))
                .flex()
                .flex_col()
                .gap_4()
                .justify_center()
                .items_center()
                .font_family("Helvetica")
                .text_color(rgb(0xffffff))
                .text_2xl()
                .track_focus(&self.focus_handle)
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                    this.last_keystroke = Some(event.keystroke.clone());
                    this.key_down_count += 1;
                    cx.notify();
                }))
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                    // The platform parks the pointer just off-window after a
                    // touch ends to clear hover; don't display that position.
                    if event.position.x >= px(0.) {
                        this.last_touch = Some(event.position);
                        cx.notify();
                    }
                }))
                .on_pinch(cx.listener(|this, event: &PinchEvent, _, cx| {
                    this.pinch_scale = (this.pinch_scale * (1. + event.delta)).clamp(0.5, 2.5);
                    cx.notify();
                }))
                .child("Hello, iOS!")
                .child(div().text_lg().child(last_touch))
                .child(div().text_lg().child(last_key))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(self.text_field.clone())
                        .child(
                            div()
                                .id("done")
                                .px_3()
                                .py_1()
                                .bg(rgb(0x3b82f6))
                                .rounded_md()
                                .text_lg()
                                .child("Done")
                                .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                                    // Stealing focus back to the root handle
                                    // must dismiss the software keyboard.
                                    window.focus(&this.focus_handle, cx);
                                    cx.notify();
                                })),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .gap_4()
                        .child(self.tap_counter_box(0, rgb(0xef4444), rgb(0xf87171), cx))
                        .child(self.tap_counter_box(1, rgb(0x22c55e), rgb(0x4ade80), cx))
                        .child(self.tap_counter_box(2, rgb(0x3b82f6), rgb(0x60a5fa), cx)),
                )
                .child(format!("Row taps: {}", self.row_taps))
                .child(self.row_list(cx))
                .child(
                    div()
                        .w(px(280.))
                        .p_4()
                        .bg(gpui::white())
                        .rounded_xl()
                        .shadow_lg()
                        .flex()
                        .items_center()
                        .gap_4()
                        .text_lg()
                        .text_color(rgb(0x111827))
                        .child(
                            div()
                                .flex_none()
                                .size(px(40. * self.pinch_scale))
                                .rounded_md()
                                .bg(rgb(0xa855f7)),
                        )
                        .child(format!("Pinch scale: {:.2}", self.pinch_scale)),
                )
        }
    }

    pub fn run() {
        Application::with_platform(Rc::new(gpui_ios::IosPlatform::new())).run(|cx: &mut App| {
            cx.bind_keys([
                KeyBinding::new("backspace", Backspace, Some("TextField")),
                KeyBinding::new("left", Left, Some("TextField")),
                KeyBinding::new("right", Right, Some("TextField")),
            ]);
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| HelloIos {
                    focus_handle: cx.focus_handle(),
                    text_field: cx.new(|cx| TextField {
                        focus_handle: cx.focus_handle(),
                        content: "".into(),
                        selected_range: 0..0,
                        marked_range: None,
                        last_layout: None,
                        last_bounds: None,
                    }),
                    tap_counts: [0; 3],
                    row_taps: 0,
                    pinch_scale: 1.,
                    last_touch: None,
                    last_keystroke: None,
                    key_down_count: 0,
                });
                let focus_handle = view.read(cx).focus_handle.clone();
                window.focus(&focus_handle, cx);
                view
            })
            .unwrap();
        });
    }
}

#[cfg(target_os = "ios")]
fn main() {
    example::run();
}

#[cfg(not(target_os = "ios"))]
fn main() {}
