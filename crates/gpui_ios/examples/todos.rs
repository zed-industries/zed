//! A touch-friendly todo list built from zed's `ui` component library on the
//! zed theme, demonstrating GPUI's UIKit platform: tap targets, flick
//! scrolling with momentum, and text entry through the software keyboard and
//! IME.

#[cfg(target_os = "ios")]
mod example {
    use gpui::{
        App, Application, Bounds, ClickEvent, ElementInputHandler, Entity, EntityInputHandler,
        EventEmitter, FocusHandle, Font, FontWeight, GlobalElementId, InspectorElementId,
        KeyBinding, LayoutId, MouseButton, PaintQuad, Point, ShapedLine, Style, Subscription,
        TextRun, UTF16Selection, UnderlineStyle, WindowOptions, actions, fill, font, point,
        relative, size,
    };
    use std::{ops::Range, rc::Rc};
    use theme::{LoadThemes, ThemeSettingsProvider, UiDensity};
    use ui::prelude::*;

    actions!(todos, [Backspace, Left, Right, Submit]);

    const UI_FONT_FAMILY: &str = ".SystemUIFont";

    struct TodoThemeSettings {
        ui_font: Font,
        buffer_font: Font,
    }

    impl Default for TodoThemeSettings {
        fn default() -> Self {
            Self {
                ui_font: font(UI_FONT_FAMILY),
                buffer_font: font("Lilex"),
            }
        }
    }

    impl ThemeSettingsProvider for TodoThemeSettings {
        fn ui_font<'a>(&'a self, _cx: &'a App) -> &'a Font {
            &self.ui_font
        }

        fn buffer_font<'a>(&'a self, _cx: &'a App) -> &'a Font {
            &self.buffer_font
        }

        fn ui_font_size(&self, _cx: &App) -> Pixels {
            px(16.)
        }

        fn buffer_font_size(&self, _cx: &App) -> Pixels {
            px(15.)
        }

        fn ui_density(&self, _cx: &App) -> UiDensity {
            UiDensity::Comfortable
        }
    }

    /// Emitted by [`TextField`] when the user presses the return key.
    struct InputSubmitted;

    /// A deliberately minimal single-line text field exercising gpui's
    /// text-input bridge: no selection dragging, no clipboard, no blinking.
    struct TextField {
        focus_handle: FocusHandle,
        content: SharedString,
        placeholder: SharedString,
        /// Selected byte range in `content`; empty means a caret.
        selected_range: Range<usize>,
        /// Byte range of in-progress IME composition in `content`.
        marked_range: Option<Range<usize>>,
        last_layout: Option<ShapedLine>,
        last_bounds: Option<Bounds<Pixels>>,
    }

    impl EventEmitter<InputSubmitted> for TextField {}

    impl TextField {
        fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
            Self {
                focus_handle: cx.focus_handle(),
                content: "".into(),
                placeholder: placeholder.into(),
                selected_range: 0..0,
                marked_range: None,
                last_layout: None,
                last_bounds: None,
            }
        }

        fn text(&self) -> &str {
            &self.content
        }

        fn clear(&mut self, cx: &mut Context<Self>) {
            self.content = "".into();
            self.selected_range = 0..0;
            self.marked_range = None;
            cx.notify();
        }

        fn submit(&mut self, _: &Submit, window: &mut Window, cx: &mut Context<Self>) {
            if self.content.trim().is_empty() {
                // Return on an empty field dismisses the keyboard (blurring
                // resigns the platform's text-input responder).
                window.blur();
            } else {
                cx.emit(InputSubmitted);
            }
        }

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
            // The last layout may be of the placeholder text, which is longer
            // than the (empty) content, so clamp.
            line.closest_index_for_x(position.x - bounds.left())
                .min(self.content.len())
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
            let utf8_index = layout
                .index_for_x(position.x - bounds.left())?
                .min(self.content.len());
            Some(self.offset_to_utf16(utf8_index))
        }
    }

    impl Render for TextField {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let theme = cx.theme().clone();
            let border_color = if self.focus_handle.is_focused(window) {
                theme.colors().border_focused
            } else {
                theme.colors().border_variant
            };
            h_flex()
                .id("todo-input")
                .key_context("TodoInput")
                .track_focus(&self.focus_handle)
                .on_action(cx.listener(Self::submit))
                .on_action(cx.listener(Self::backspace))
                .on_action(cx.listener(Self::left))
                .on_action(cx.listener(Self::right))
                .on_click(cx.listener(Self::on_tap))
                .w_full()
                .h(px(48.))
                .px_4()
                .bg(theme.colors().editor_background)
                .rounded_full()
                .border_1()
                .border_color(border_color)
                .text_size(px(16.))
                .text_color(theme.colors().text)
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

    impl Element for TextFieldElement {
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
            cx: &mut App,
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
            cx: &mut App,
        ) -> Self::PrepaintState {
            let theme = cx.theme().clone();
            let field = self.field.read(cx);
            let content = field.content.clone();
            let placeholder = field.placeholder.clone();
            let selected_range = field.selected_range.clone();
            let marked_range = field.marked_range.clone();
            let style = window.text_style();

            let (display_text, text_color) = if content.is_empty() {
                (placeholder, theme.colors().text_placeholder)
            } else {
                (content, style.color)
            };

            let run = TextRun {
                len: display_text.len(),
                font: style.font(),
                color: text_color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let runs = if let Some(marked_range) = marked_range.as_ref() {
                let accent = theme.colors().text_accent;
                vec![
                    TextRun {
                        len: marked_range.start,
                        ..run.clone()
                    },
                    TextRun {
                        len: marked_range.len(),
                        color: accent,
                        underline: Some(UnderlineStyle {
                            color: Some(accent),
                            thickness: px(2.),
                            wavy: false,
                        }),
                        ..run.clone()
                    },
                    TextRun {
                        len: display_text.len() - marked_range.end,
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
                .shape_line(display_text, font_size, &runs, None);

            let cursor_color = theme.players().local().cursor;
            let (selection, cursor) = if selected_range.is_empty() {
                let cursor_x = line.x_for_index(selected_range.start);
                (
                    None,
                    Some(fill(
                        Bounds::new(
                            point(bounds.left() + cursor_x, bounds.top()),
                            size(px(2.), bounds.size.height),
                        ),
                        cursor_color,
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
                        theme.players().local().selection,
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
            cx: &mut App,
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

    #[derive(Clone)]
    struct Todo {
        id: usize,
        title: SharedString,
        done: bool,
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Filter {
        All,
        Active,
        Done,
    }

    impl Filter {
        fn accepts(self, todo: &Todo) -> bool {
            match self {
                Filter::All => true,
                Filter::Active => !todo.done,
                Filter::Done => todo.done,
            }
        }
    }

    struct TodosApp {
        input: Entity<TextField>,
        todos: Vec<Todo>,
        next_todo_id: usize,
        filter: Filter,
        _input_subscription: Subscription,
    }

    impl TodosApp {
        fn new(cx: &mut Context<Self>) -> Self {
            let input = cx.new(|cx| TextField::new("What needs doing?", cx));
            let input_subscription =
                cx.subscribe(&input, |this, _input, _event: &InputSubmitted, cx| {
                    this.add_todo(cx);
                });
            let seed_titles: [(&str, bool); 6] = [
                ("Water the plants", true),
                ("Reply to Ana's email", false),
                ("Book dentist appointment", false),
                ("Fix the balcony light", false),
                ("Read the GPUI docs", true),
                ("Plan weekend hike", false),
            ];
            let todos = seed_titles
                .into_iter()
                .enumerate()
                .map(|(id, (title, done))| Todo {
                    id,
                    title: title.into(),
                    done,
                })
                .collect::<Vec<_>>();
            Self {
                input,
                next_todo_id: todos.len(),
                todos,
                filter: Filter::All,
                _input_subscription: input_subscription,
            }
        }

        fn add_todo(&mut self, cx: &mut Context<Self>) {
            let title = self.input.read(cx).text().trim().to_string();
            if title.is_empty() {
                return;
            }
            self.todos.push(Todo {
                id: self.next_todo_id,
                title: title.into(),
                done: false,
            });
            self.next_todo_id += 1;
            self.input.update(cx, |input, cx| input.clear(cx));
            cx.notify();
        }

        fn toggle_todo(&mut self, todo_id: usize, cx: &mut Context<Self>) {
            if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == todo_id) {
                todo.done = !todo.done;
                cx.notify();
            }
        }

        fn delete_todo(&mut self, todo_id: usize, cx: &mut Context<Self>) {
            self.todos.retain(|todo| todo.id != todo_id);
            cx.notify();
        }

        fn set_filter(&mut self, filter: Filter, cx: &mut Context<Self>) {
            self.filter = filter;
            cx.notify();
        }

        fn clear_completed(&mut self, cx: &mut Context<Self>) {
            self.todos.retain(|todo| !todo.done);
            cx.notify();
        }

        fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
            let theme = cx.theme().clone();
            let total = self.todos.len();
            let done = self.todos.iter().filter(|todo| todo.done).count();
            let progress = if total == 0 {
                0.
            } else {
                done as f32 / total as f32
            };
            let summary = match (total, total - done) {
                (0, _) => "Add your first todo".to_string(),
                (_, 0) => "All done \u{2014} nice work".to_string(),
                (_, 1) => "1 task left".to_string(),
                (_, remaining) => format!("{remaining} tasks left"),
            };
            v_flex()
                .flex_none()
                .gap_3()
                .child(
                    h_flex()
                        .justify_between()
                        .items_end()
                        .child(
                            div()
                                .text_size(px(34.))
                                .line_height(px(38.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.colors().text)
                                .child("Todos"),
                        )
                        .child(
                            div()
                                .text_size(px(15.))
                                .pb(px(5.))
                                .text_color(theme.colors().text_muted)
                                .child(summary),
                        ),
                )
                .child(
                    div()
                        .h(px(5.))
                        .rounded_full()
                        .bg(theme.colors().element_background)
                        .child(
                            div()
                                .h_full()
                                .rounded_full()
                                .w(relative(progress))
                                .bg(theme.colors().text_accent),
                        ),
                )
        }

        fn render_add_row(&self, cx: &mut Context<Self>) -> impl IntoElement {
            let accent = cx.theme().colors().text_accent;
            h_flex()
                .flex_none()
                // Keep taps on the field and add button from reaching the
                // root's blur handler, so the keyboard stays up for rapid entry.
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .gap_3()
                .child(div().flex_1().child(self.input.clone()))
                .child(
                    div()
                        .id("add-todo")
                        .size(px(48.))
                        .flex_none()
                        .rounded_full()
                        .bg(accent)
                        .active(|style| style.bg(accent.alpha(0.7)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            Icon::new(IconName::Plus)
                                .size(IconSize::Medium)
                                .color(Color::Custom(gpui::white())),
                        )
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| this.add_todo(cx))),
                )
        }

        fn render_todo_row(&self, todo: &Todo, cx: &mut Context<Self>) -> impl IntoElement {
            let theme = cx.theme().clone();
            let todo_id = todo.id;
            let accent = theme.colors().text_accent;
            let checkbox = div()
                .size(px(26.))
                .flex_none()
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .map(|circle| {
                    if todo.done {
                        circle.bg(accent).child(
                            Icon::new(IconName::Check)
                                .size(IconSize::Small)
                                .color(Color::Custom(gpui::white())),
                        )
                    } else {
                        circle.border_2().border_color(theme.colors().border)
                    }
                });
            let title = div()
                .flex_1()
                .text_size(px(17.))
                .map(|title| {
                    if todo.done {
                        title.text_color(theme.colors().text_muted).line_through()
                    } else {
                        title.text_color(theme.colors().text)
                    }
                })
                .child(todo.title.clone());
            h_flex()
                .id(("todo", todo_id))
                .flex_none()
                .min_h(px(56.))
                .pl_4()
                .pr_2()
                .gap_3()
                .active(|style| style.bg(theme.colors().element_active))
                .on_click(
                    cx.listener(move |this, _: &ClickEvent, _, cx| this.toggle_todo(todo_id, cx)),
                )
                // The row handles taps (including on the checkbox); giving the
                // checkbox its own handler would double-toggle since gpui click
                // handlers don't stop propagation.
                .child(checkbox)
                .child(title)
                .child(
                    IconButton::new(("delete", todo_id), IconName::Trash)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::Large)
                        .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                            this.delete_todo(todo_id, cx)
                        })),
                )
        }

        fn render_todo_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
            let theme = cx.theme().clone();
            let visible_todos = self
                .todos
                .iter()
                .filter(|todo| self.filter.accepts(todo))
                .cloned()
                .collect::<Vec<_>>();

            let mut rows = Vec::<AnyElement>::new();
            for (index, todo) in visible_todos.iter().enumerate() {
                if index > 0 {
                    // Inset separator aligned with the title text, iOS-style.
                    rows.push(
                        div()
                            .flex_none()
                            .h(px(1.))
                            .ml(px(54.))
                            .bg(theme.colors().border_variant)
                            .into_any_element(),
                    );
                }
                rows.push(self.render_todo_row(todo, cx).into_any_element());
            }

            v_flex()
                .id("todo-list")
                // Grow with content but shrink (and scroll) when space runs out,
                // so the card hugs its rows instead of filling the screen.
                .flex_initial()
                .min_h_0()
                .overflow_y_scroll()
                .rounded_xl()
                .border_1()
                .border_color(theme.colors().border_variant)
                .bg(theme.colors().elevated_surface_background)
                .shadow_md()
                .when(visible_todos.is_empty(), |this| {
                    this.child(
                        v_flex()
                            .items_center()
                            .gap_2()
                            .py_12()
                            .child(
                                Icon::new(IconName::Check)
                                    .size(IconSize::XLarge)
                                    .color(Color::Muted),
                            )
                            .child(
                                Label::new(match self.filter {
                                    Filter::All => "Nothing to do yet",
                                    Filter::Active => "No active todos",
                                    Filter::Done => "Nothing done yet",
                                })
                                .size(LabelSize::Large)
                                .color(Color::Muted),
                            ),
                    )
                })
                .children(rows)
        }

        fn render_segment(
            &self,
            index: usize,
            label: &'static str,
            filter: Filter,
            cx: &mut Context<Self>,
        ) -> impl IntoElement {
            let theme = cx.theme().clone();
            let selected = self.filter == filter;
            div()
                .id(("filter", index))
                .flex_1()
                .h_full()
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(15.))
                .map(|segment| {
                    if selected {
                        segment
                            .bg(theme.colors().element_selected)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors().text)
                            .shadow_sm()
                    } else {
                        segment.text_color(theme.colors().text_muted)
                    }
                })
                .child(label)
                .on_click(
                    cx.listener(move |this, _: &ClickEvent, _, cx| this.set_filter(filter, cx)),
                )
        }

        fn render_footer(&self, cx: &mut Context<Self>) -> impl IntoElement {
            let theme = cx.theme().clone();
            let has_completed = self.todos.iter().any(|todo| todo.done);
            v_flex()
                .flex_none()
                .gap_1()
                .child(
                    h_flex()
                        .h(px(40.))
                        .p(px(3.))
                        .gap_1()
                        .rounded_full()
                        .bg(theme.colors().element_background)
                        .child(self.render_segment(0, "All", Filter::All, cx))
                        .child(self.render_segment(1, "Active", Filter::Active, cx))
                        .child(self.render_segment(2, "Done", Filter::Done, cx)),
                )
                .child(
                    Button::new("clear-completed", "Clear completed")
                        .style(ButtonStyle::Transparent)
                        .size(ButtonSize::Large)
                        .label_size(LabelSize::Default)
                        .color(Color::Muted)
                        .disabled(!has_completed)
                        .full_width()
                        .on_click(
                            cx.listener(|this, _: &ClickEvent, _, cx| this.clear_completed(cx)),
                        ),
                )
        }
    }

    impl Render for TodosApp {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let theme = cx.theme().clone();
            v_flex()
                .size_full()
                // Tapping anywhere outside the add row unfocuses the input,
                // which dismisses the software keyboard.
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|_, _, window, _| window.blur()),
                )
                .bg(theme.colors().background)
                .text_color(theme.colors().text)
                .font_family(UI_FONT_FAMILY)
                // No safe-area API yet: pad past the status bar / dynamic
                // island at the top and the home indicator at the bottom.
                .pt(px(64.))
                .pb(px(40.))
                .px(px(20.))
                .gap_4()
                .child(self.render_header(cx))
                .child(self.render_add_row(cx))
                .child(self.render_todo_list(cx))
                .child(div().flex_1())
                .child(self.render_footer(cx))
        }
    }

    pub fn run() {
        Application::with_platform(Rc::new(gpui_ios::IosPlatform::new()))
            .with_assets(assets::Assets)
            .run(|cx: &mut App| {
                assets::Assets
                    .load_fonts(cx)
                    .expect("failed to load embedded fonts");
                theme::init(LoadThemes::JustBase, cx);
                theme::set_theme_settings_provider(Box::new(TodoThemeSettings::default()), cx);
                cx.bind_keys([
                    KeyBinding::new("enter", Submit, Some("TodoInput")),
                    KeyBinding::new("backspace", Backspace, Some("TodoInput")),
                    KeyBinding::new("left", Left, Some("TodoInput")),
                    KeyBinding::new("right", Right, Some("TodoInput")),
                ]);
                cx.open_window(WindowOptions::default(), |_window, cx| {
                    cx.new(TodosApp::new)
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
