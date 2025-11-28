//! A single-line text input element.
//!
//! Input-based elements are made up of two parts:
//!
//! - `InputState`: a reusable entity that manages text content, insertion and selection state.
//! - An element: handles layout, painting, interaction and behavior specific to this type of input

use crate::{
    Action, App, Bounds, ContentMask, Context, CursorStyle, DispatchPhase, Element, ElementId,
    ElementInputHandler, Entity, FocusHandle, Focusable, GlobalElementId, Hitbox, HitboxBehavior,
    Hsla, InputState, InspectorElementId, InteractiveElement, Interactivity, IntoElement, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollWheelEvent,
    SharedString, StyleRefinement, Styled, TextRun, TextStyle, Window, WrappedLine, colors, fill,
    point, px, size,
};

const CURSOR_WIDTH: f32 = 2.0;
const MARKED_TEXT_UNDERLINE_THICKNESS: f32 = 2.0;

/// Creates a new `TextInput` element powered by the given `InputState`.
///
/// The `InputState` should be created with `InputState::new_multiline(false, cx)`.
#[track_caller]
pub fn text_input(input: &Entity<InputState>) -> TextInput {
    let mut text_input = TextInput {
        input: input.clone(),
        interactivity: Interactivity::new(),
        placeholder: None,
        selection_color: None,
        cursor_color: None,
    };
    text_input.register_actions();
    text_input
}

/// A single-line text editing element.
pub struct TextInput {
    input: Entity<InputState>,
    interactivity: Interactivity,
    placeholder: Option<SharedString>,
    selection_color: Option<Hsla>,
    cursor_color: Option<Hsla>,
}

impl TextInput {
    /// Sets the placeholder text shown when the input is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the color used for text selection highlighting.
    pub fn selection_color(mut self, color: impl Into<Hsla>) -> Self {
        self.selection_color = Some(color.into());
        self
    }

    /// Sets the color of the text cursor.
    pub fn cursor_color(mut self, color: impl Into<Hsla>) -> Self {
        self.cursor_color = Some(color.into());
        self
    }

    fn color(&self, window: &Window) -> PaintColors {
        let default_colors = colors::Colors::for_appearance(window);

        PaintColors {
            selection: self
                .selection_color
                .unwrap_or(default_colors.selected.into()),
            cursor: self.cursor_color.unwrap_or(default_colors.cursor.into()),
        }
    }

    fn register_actions(&mut self) {
        register_action(&mut self.interactivity, &self.input, InputState::left);
        register_action(&mut self.interactivity, &self.input, InputState::right);
        register_action(&mut self.interactivity, &self.input, InputState::up);
        register_action(&mut self.interactivity, &self.input, InputState::down);
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::select_left,
        );
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::select_right,
        );
        register_action(&mut self.interactivity, &self.input, InputState::select_up);
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::select_down,
        );
        register_action(&mut self.interactivity, &self.input, InputState::select_all);
        register_action(&mut self.interactivity, &self.input, InputState::home);
        register_action(&mut self.interactivity, &self.input, InputState::end);
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::move_to_beginning,
        );
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::move_to_end,
        );
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::select_to_beginning,
        );
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::select_to_end,
        );
        register_action(&mut self.interactivity, &self.input, InputState::word_left);
        register_action(&mut self.interactivity, &self.input, InputState::word_right);
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::select_word_left,
        );
        register_action(
            &mut self.interactivity,
            &self.input,
            InputState::select_word_right,
        );
        register_action(&mut self.interactivity, &self.input, InputState::backspace);
        register_action(&mut self.interactivity, &self.input, InputState::delete);
        register_action(&mut self.interactivity, &self.input, InputState::enter);
        register_action(&mut self.interactivity, &self.input, InputState::tab);
        register_action(&mut self.interactivity, &self.input, InputState::paste);
        register_action(&mut self.interactivity, &self.input, InputState::copy);
        register_action(&mut self.interactivity, &self.input, InputState::cut);
    }
}

fn register_action<A: Action>(
    interactivity: &mut Interactivity,
    input: &Entity<InputState>,
    listener: fn(&mut InputState, &A, &mut Window, &mut Context<InputState>),
) {
    let input = input.clone();
    interactivity.on_action::<A>(move |action, window, cx| {
        input.update(cx, |input, cx| {
            listener(input, action, window, cx);
        });
    });
}

impl Styled for TextInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for TextInput {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

/// Layout state passed from request_layout to prepaint.
pub struct TextInputLayoutState {
    text_style: TextStyle,
}

/// Prepaint state passed from prepaint to paint.
pub struct TextInputPrepaintState {
    hitbox: Option<Hitbox>,
}

impl Element for TextInput {
    type RequestLayoutState = TextInputLayoutState;
    type PrepaintState = TextInputPrepaintState;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.interactivity.source_location()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let focus_handle = self.input.focus_handle(cx);
        self.interactivity.tracked_focus_handle = Some(focus_handle);

        let mut text_style = None;

        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| {
                window.with_text_style(style.text_style().cloned(), |window| {
                    text_style = Some(window.text_style());

                    let style = style.clone();
                    window.request_layout(style, None, cx)
                })
            },
        );

        (
            layout_id,
            TextInputLayoutState {
                text_style: text_style.unwrap_or_else(|| window.text_style()),
            },
        )
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout_state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let text_color = layout_state.text_style.color;
        let line_height = layout_state
            .text_style
            .line_height_in_pixels(window.rem_size());

        // For single-line input, we use a very large wrap width to prevent wrapping
        let no_wrap_width = px(100000.);

        self.input.update(cx, |input, _cx| {
            input.available_height = bounds.size.height;
            input.available_width = bounds.size.width;
            input.update_line_layouts(no_wrap_width, line_height, text_color, window);
        });

        let hitbox = self.interactivity.prepaint(
            global_id,
            inspector_id,
            bounds,
            bounds.size,
            window,
            cx,
            |_style, _point, hitbox, window, _cx| {
                hitbox.or_else(|| Some(window.insert_hitbox(bounds, HitboxBehavior::Normal)))
            },
        );

        TextInputPrepaintState { hitbox }
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout_state: &mut Self::RequestLayoutState,
        prepaint_state: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.focus_handle(cx);
        let colors = self.color(window);

        if let Some(hitbox) = &prepaint_state.hitbox {
            window.set_cursor_style(CursorStyle::IBeam, hitbox);
        }

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );

        let input = self.input.clone();
        let placeholder = self.placeholder.clone();
        let text_style = layout_state.text_style.clone();

        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            prepaint_state.hitbox.as_ref(),
            window,
            cx,
            |_style, window, cx| {
                handle_mouse(&input, bounds, window, cx);

                let state = PaintState::from_input(&input, &focus_handle, window, cx);
                let colors = colors.clone();

                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    if !state.selected_range.is_empty() {
                        paint_selection(&state, bounds, colors.selection, window);
                    }

                    if state.content.is_empty() {
                        if let Some(placeholder_str) = placeholder.as_ref() {
                            if !placeholder_str.is_empty() {
                                paint_placeholder(placeholder_str, bounds, &text_style, window, cx);
                            }
                        }
                    } else {
                        paint_text(&state, bounds, window, cx);
                    }

                    if let Some(marked_range) = &state.marked_range {
                        if !marked_range.is_empty() {
                            paint_marked_text_underline(
                                &state,
                                marked_range,
                                bounds,
                                colors.cursor,
                                window,
                            );
                        }
                    }

                    if state.is_focused && state.selected_range.is_empty() {
                        paint_cursor(&state, bounds, colors.cursor, window);
                    }
                });
            },
        );
    }
}

/// Collected state needed for painting.
struct PaintState {
    content: String,
    selected_range: std::ops::Range<usize>,
    marked_range: Option<std::ops::Range<usize>>,
    cursor_offset: usize,
    scroll_offset: Pixels,
    line_height: Pixels,
    text_width: Pixels,
    is_focused: bool,
    /// Position for each character index (x offset from start of text)
    char_positions: Vec<Pixels>,
    /// The wrapped line layout for painting text
    wrapped_line: Option<WrappedLine>,
}

impl PaintState {
    fn from_input(
        input: &Entity<InputState>,
        focus_handle: &FocusHandle,
        window: &Window,
        cx: &App,
    ) -> Self {
        let input_state = input.read(cx);

        // Get character positions from the line layout
        let mut char_positions = Vec::new();
        let mut text_width = px(0.);

        if let Some(line) = input_state.line_layouts.first() {
            if let Some(wrapped) = &line.wrapped_line {
                text_width = wrapped.width();
                // Build position map for each character
                let content = input_state.content();
                let mut idx = 0;
                for ch in content.chars() {
                    if let Some(pos) = wrapped.position_for_index(idx, input_state.line_height) {
                        char_positions.push(pos.x);
                    } else {
                        char_positions.push(text_width);
                    }
                    idx += ch.len_utf8();
                }
                // Add end position
                char_positions.push(text_width);
            }
        }

        let wrapped_line = input_state
            .line_layouts
            .first()
            .and_then(|l| l.wrapped_line.clone());

        Self {
            content: input_state.content().to_string(),
            selected_range: input_state.selected_range().clone(),
            marked_range: input_state.marked_range().cloned(),
            cursor_offset: input_state.cursor_offset(),
            scroll_offset: input_state.scroll_offset,
            line_height: input_state.line_height,
            text_width,
            is_focused: focus_handle.is_focused(window),
            char_positions,
            wrapped_line,
        }
    }

    /// Get x position for a character index
    fn x_for_index(&self, index: usize) -> Pixels {
        // Convert byte index to char index
        let char_index = self.content[..index.min(self.content.len())]
            .chars()
            .count();
        self.char_positions
            .get(char_index)
            .copied()
            .unwrap_or(self.text_width)
    }
}

/// Colors used for painting.
#[derive(Clone)]
struct PaintColors {
    pub selection: Hsla,
    pub cursor: Hsla,
}

/// Registers all mouse event handlers for the text input.
fn handle_mouse(input: &Entity<InputState>, bounds: Bounds<Pixels>, window: &mut Window, cx: &App) {
    mouse_down(input.clone(), bounds, window);
    mouse_up(input.clone(), window);
    mouse_move(input.clone(), bounds, window);
    handle_scroll(input.clone(), bounds, window, cx);
}

/// Converts a screen position to a position relative to the text area origin,
/// adjusted for horizontal scroll offset.
fn screen_to_text_position(
    screen_position: Point<Pixels>,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
) -> Point<Pixels> {
    point(
        screen_position.x - bounds.origin.x + scroll_offset,
        screen_position.y - bounds.origin.y,
    )
}

fn mouse_down(input: Entity<InputState>, bounds: Bounds<Pixels>, window: &mut Window) {
    window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }
        if !bounds.contains(&event.position) {
            return;
        }
        if event.button != MouseButton::Left {
            return;
        }

        input.update(cx, |input, cx| {
            let text_position =
                screen_to_text_position(event.position, bounds, input.scroll_offset);
            input.on_mouse_down(
                text_position,
                event.click_count,
                event.modifiers.shift,
                window,
                cx,
            );
        });
    });
}

fn mouse_up(input: Entity<InputState>, window: &mut Window) {
    window.on_mouse_event(move |event: &MouseUpEvent, phase, _window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }
        if event.button != MouseButton::Left {
            return;
        }

        input.update(cx, |input, cx| {
            input.on_mouse_up(cx);
        });
    });
}

fn mouse_move(input: Entity<InputState>, bounds: Bounds<Pixels>, window: &mut Window) {
    window.on_mouse_event(move |event: &MouseMoveEvent, phase, _window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }

        input.update(cx, |input, cx| {
            let text_position =
                screen_to_text_position(event.position, bounds, input.scroll_offset);
            input.on_mouse_move(text_position, cx);
        });
    });
}

fn handle_scroll(input: Entity<InputState>, bounds: Bounds<Pixels>, window: &mut Window, cx: &App) {
    let text_width = input
        .read(cx)
        .line_layouts
        .first()
        .and_then(|l| l.wrapped_line.as_ref())
        .map(|w| w.width())
        .unwrap_or(px(0.));
    let max_scroll = (text_width - bounds.size.width).max(px(0.));

    window.on_mouse_event(move |event: &ScrollWheelEvent, phase, _window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }
        if !bounds.contains(&event.position) {
            return;
        }

        let pixel_delta = event.delta.pixel_delta(px(20.));
        input.update(cx, |input, cx| {
            // For single-line, horizontal scroll uses the x delta (or y if shift is held)
            let delta = if pixel_delta.x.abs() > pixel_delta.y.abs() {
                pixel_delta.x
            } else {
                pixel_delta.y
            };
            input.scroll_offset = (input.scroll_offset - delta).clamp(px(0.), max_scroll);
            cx.notify();
        });
    });
}

fn paint_selection(
    state: &PaintState,
    bounds: Bounds<Pixels>,
    selection_color: Hsla,
    window: &mut Window,
) {
    let start_x = state.x_for_index(state.selected_range.start) - state.scroll_offset;
    let end_x = state.x_for_index(state.selected_range.end) - state.scroll_offset;

    // Vertically center the selection within the bounds
    let y_offset = (bounds.size.height - state.line_height).max(px(0.)) / 2.0;

    window.paint_quad(fill(
        Bounds::from_corners(
            point(bounds.left() + start_x, bounds.top() + y_offset),
            point(
                bounds.left() + end_x,
                bounds.top() + y_offset + state.line_height,
            ),
        ),
        selection_color,
    ));
}

fn paint_placeholder(
    placeholder: &SharedString,
    bounds: Bounds<Pixels>,
    text_style: &TextStyle,
    window: &mut Window,
    cx: &mut App,
) {
    let placeholder_color = text_style.color.opacity(0.5);
    let run = TextRun {
        len: placeholder.len(),
        font: text_style.font(),
        color: placeholder_color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };

    let font_size = text_style.font_size.to_pixels(window.rem_size());
    let shaped_line = window
        .text_system()
        .shape_line(placeholder.clone(), font_size, &[run], None);
    let line_height = text_style.line_height_in_pixels(window.rem_size());

    // Vertically center the placeholder
    let y_offset = (bounds.size.height - line_height).max(px(0.)) / 2.0;
    let paint_origin = point(bounds.origin.x, bounds.origin.y + y_offset);

    let _ = shaped_line.paint(paint_origin, line_height, window, cx);
}

fn paint_text(state: &PaintState, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
    let Some(wrapped_line) = &state.wrapped_line else {
        return;
    };

    // Vertically center the text
    let y_offset = (bounds.size.height - state.line_height).max(px(0.)) / 2.0;
    let paint_origin = point(
        bounds.origin.x - state.scroll_offset,
        bounds.origin.y + y_offset,
    );

    let _ = wrapped_line.paint(
        paint_origin,
        state.line_height,
        crate::TextAlign::Left,
        Some(bounds),
        window,
        cx,
    );
}

fn paint_marked_text_underline(
    state: &PaintState,
    marked_range: &std::ops::Range<usize>,
    bounds: Bounds<Pixels>,
    underline_color: Hsla,
    window: &mut Window,
) {
    let start_x = state.x_for_index(marked_range.start) - state.scroll_offset;
    let end_x = state.x_for_index(marked_range.end) - state.scroll_offset;

    let underline_thickness = px(MARKED_TEXT_UNDERLINE_THICKNESS);
    let y_offset = (bounds.size.height - state.line_height).max(px(0.)) / 2.0;
    let underline_y = bounds.top() + y_offset + state.line_height - underline_thickness;

    window.paint_quad(fill(
        Bounds::from_corners(
            point(bounds.left() + start_x, underline_y),
            point(bounds.left() + end_x, underline_y + underline_thickness),
        ),
        underline_color,
    ));
}

fn paint_cursor(
    state: &PaintState,
    bounds: Bounds<Pixels>,
    cursor_color: Hsla,
    window: &mut Window,
) {
    let cursor_x = state.x_for_index(state.cursor_offset) - state.scroll_offset;

    // Vertically center the cursor
    let y_offset = (bounds.size.height - state.line_height).max(px(0.)) / 2.0;

    window.paint_quad(fill(
        Bounds::new(
            point(bounds.left() + cursor_x, bounds.top() + y_offset),
            size(px(CURSOR_WIDTH), state.line_height),
        ),
        cursor_color,
    ));
}

impl Focusable for TextInput {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl IntoElement for TextInput {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
