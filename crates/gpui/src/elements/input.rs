//! A text input element that supports both single-line and multi-line modes.
//!
//! Input-based elements are made up of two parts:
//!
//! - `InputState`: a reusable entity that manages text content, insertion and selection state.
//! - An element: handles layout, painting, interaction and behavior specific to this type of input
//!
//! Use `input()` for single-line text fields and `text_area()` for multi-line text editing.

use crate::{
    Action, App, Bounds, ContentMask, Context, CursorStyle, DispatchPhase, Element, ElementId,
    ElementInputHandler, Entity, FocusHandle, Focusable, GlobalElementId, Hitbox, HitboxBehavior,
    Hsla, InputLineLayout, InputState, InspectorElementId, InteractiveElement, Interactivity,
    IntoElement, LayoutId, Length, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Point, ScrollWheelEvent, SharedString, StyleRefinement, Styled, TextAlign,
    TextDirection, TextRun, TextStyle, Window, WrappedLine, colors, fill, input::INPUT_CONTEXT,
    point, px, relative, size,
};

const CURSOR_WIDTH: f32 = 2.0;
const MARKED_TEXT_UNDERLINE_THICKNESS: f32 = 2.0;

/// Creates a new single-line `Input` element powered by the given `InputState`.
///
/// The `InputState` should be created with `InputState::new_singleline(cx)`.
#[track_caller]
pub fn input(input_state: &Entity<InputState>) -> Input {
    Input::new(input_state, false)
}

/// Creates a new multi-line `Input` element (text area) powered by the given `InputState`.
///
/// The `InputState` should be created with `InputState::new_multiline(cx)`.
#[track_caller]
pub fn text_area(input_state: &Entity<InputState>) -> Input {
    Input::new(input_state, true)
}

/// A text editing element that supports both single-line and multi-line modes.
pub struct Input {
    input: Entity<InputState>,
    interactivity: Interactivity,
    placeholder: Option<SharedString>,
    selection_color: Option<Hsla>,
    cursor_color: Option<Hsla>,
    multiline: bool,
}

impl Input {
    #[track_caller]
    fn new(input_state: &Entity<InputState>, multiline: bool) -> Self {
        let mut input = Input {
            input: input_state.clone(),
            interactivity: Interactivity::new(),
            placeholder: None,
            selection_color: None,
            cursor_color: None,
            multiline,
        };
        input.interactivity.key_context =
            Some(INPUT_CONTEXT.try_into().expect("Invalid INPUT_CONTEXT"));
        input.register_actions();
        input
    }

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
        register_action(&mut self.interactivity, &self.input, InputState::undo);
        register_action(&mut self.interactivity, &self.input, InputState::redo);
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

impl Styled for Input {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Input {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

/// Layout state passed from request_layout to prepaint.
pub struct InputLayoutState {
    text_style: TextStyle,
}

/// Prepaint state passed from prepaint to paint.
pub struct InputPrepaintState {
    hitbox: Option<Hitbox>,
}

impl Element for Input {
    type RequestLayoutState = InputLayoutState;
    type PrepaintState = InputPrepaintState;

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
        let multiline = self.multiline;

        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| {
                window.with_text_style(style.text_style().cloned(), |window| {
                    text_style = Some(window.text_style());

                    let mut style = style.clone();
                    if multiline {
                        if let Length::Auto = style.size.width {
                            style.size.width = relative(1.).into();
                        }
                        if let Length::Auto = style.size.height {
                            style.size.height = relative(1.).into();
                        }
                    }
                    window.request_layout(style, None, cx)
                })
            },
        );

        (
            layout_id,
            InputLayoutState {
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

        let wrap_width = if self.multiline {
            bounds.size.width
        } else {
            px(100000.)
        };

        self.input.update(cx, |input, _cx| {
            input.available_height = bounds.size.height;
            input.available_width = bounds.size.width;
            input.update_line_layouts(wrap_width, line_height, text_color, window);
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

        InputPrepaintState { hitbox }
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
        let multiline = self.multiline;
        let is_focused = focus_handle.is_focused(window);
        let cursor_visible = self
            .input
            .update(cx, |input, cx| input.cursor_visible(is_focused, cx));

        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            prepaint_state.hitbox.as_ref(),
            window,
            cx,
            |_style, window, cx| {
                handle_mouse(&input, bounds, multiline, window, cx);

                let colors = colors.clone();

                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    if multiline {
                        paint_multiline(
                            &input,
                            &focus_handle,
                            bounds,
                            &text_style,
                            placeholder.as_ref(),
                            &colors,
                            cursor_visible,
                            window,
                            cx,
                        );
                    } else {
                        paint_singleline(
                            &input,
                            &focus_handle,
                            bounds,
                            &text_style,
                            placeholder.as_ref(),
                            &colors,
                            cursor_visible,
                            window,
                            cx,
                        );
                    }
                });
            },
        );
    }
}

/// Colors used for painting.
#[derive(Clone)]
struct PaintColors {
    pub selection: Hsla,
    pub cursor: Hsla,
}

/// Registers all mouse event handlers for the input.
fn handle_mouse(
    input: &Entity<InputState>,
    bounds: Bounds<Pixels>,
    multiline: bool,
    window: &mut Window,
    cx: &App,
) {
    mouse_down(input.clone(), bounds, multiline, window);
    mouse_up(input.clone(), window);
    mouse_move(input.clone(), bounds, multiline, window);
    handle_scroll(input.clone(), bounds, multiline, window, cx);
}

fn mouse_down(
    input: Entity<InputState>,
    bounds: Bounds<Pixels>,
    multiline: bool,
    window: &mut Window,
) {
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
                screen_to_text_position(event.position, bounds, input.scroll_offset, multiline);
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

fn mouse_move(
    input: Entity<InputState>,
    bounds: Bounds<Pixels>,
    multiline: bool,
    window: &mut Window,
) {
    window.on_mouse_event(move |event: &MouseMoveEvent, phase, _window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }

        input.update(cx, |input, cx| {
            let text_position =
                screen_to_text_position(event.position, bounds, input.scroll_offset, multiline);
            input.on_mouse_move(text_position, cx);
        });
    });
}

fn handle_scroll(
    input: Entity<InputState>,
    bounds: Bounds<Pixels>,
    multiline: bool,
    window: &mut Window,
    cx: &App,
) {
    let max_scroll = if multiline {
        let total_height = input.read(cx).total_content_height();
        (total_height - bounds.size.height).max(px(0.))
    } else {
        let text_width = input
            .read(cx)
            .line_layouts
            .first()
            .and_then(|l| l.wrapped_line.as_ref())
            .map(|w| w.width())
            .unwrap_or(px(0.));
        (text_width - bounds.size.width).max(px(0.))
    };

    window.on_mouse_event(move |event: &ScrollWheelEvent, phase, _window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }
        if !bounds.contains(&event.position) {
            return;
        }

        let pixel_delta = event.delta.pixel_delta(px(20.));
        input.update(cx, |input, cx| {
            if multiline {
                input.scroll_offset =
                    (input.scroll_offset - pixel_delta.y).clamp(px(0.), max_scroll);
            } else {
                let delta = if pixel_delta.x.abs() > pixel_delta.y.abs() {
                    pixel_delta.x
                } else {
                    pixel_delta.y
                };
                input.scroll_offset = (input.scroll_offset - delta).clamp(px(0.), max_scroll);
            }
            cx.notify();
        });
    });
}

/// Converts a screen position to a position relative to the text area origin,
/// adjusted for scroll offset.
fn screen_to_text_position(
    screen_position: Point<Pixels>,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
    multiline: bool,
) -> Point<Pixels> {
    if multiline {
        point(
            screen_position.x - bounds.origin.x,
            screen_position.y - bounds.origin.y + scroll_offset,
        )
    } else {
        point(
            screen_position.x - bounds.origin.x + scroll_offset,
            screen_position.y - bounds.origin.y,
        )
    }
}

// =============================================================================
// Multi-line painting
// =============================================================================

fn paint_multiline(
    input: &Entity<InputState>,
    focus_handle: &FocusHandle,
    bounds: Bounds<Pixels>,
    text_style: &TextStyle,
    placeholder: Option<&SharedString>,
    colors: &PaintColors,
    cursor_visible: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let input_state = input.read(cx);
    let content = input_state.content().to_string();
    let selected_range = input_state.selected_range().clone();
    let marked_range = input_state.marked_range().cloned();
    let cursor_offset = input_state.cursor_offset();
    let line_layouts = input_state.line_layouts.clone();
    let scroll_offset = input_state.scroll_offset;
    let line_height = input_state.line_height;
    let is_focused = focus_handle.is_focused(window);

    if !selected_range.is_empty() {
        paint_multiline_selection(
            &line_layouts,
            &selected_range,
            bounds,
            scroll_offset,
            line_height,
            colors.selection,
            window,
        );
    }

    if content.is_empty() {
        if let Some(placeholder_str) = placeholder {
            if !placeholder_str.is_empty() {
                paint_multiline_placeholder(placeholder_str, bounds, text_style, window, cx);
            }
        }
    } else {
        paint_multiline_text(
            &line_layouts,
            bounds,
            scroll_offset,
            line_height,
            window,
            cx,
        );
    }

    if let Some(marked_range) = &marked_range {
        if !marked_range.is_empty() {
            paint_multiline_marked_underline(
                &line_layouts,
                marked_range,
                bounds,
                scroll_offset,
                line_height,
                colors.cursor,
                window,
            );
        }
    }

    if is_focused && selected_range.is_empty() && cursor_visible {
        paint_multiline_cursor(
            &line_layouts,
            cursor_offset,
            &content,
            bounds,
            scroll_offset,
            line_height,
            colors.cursor,
            window,
        );
    }
}

fn is_line_visible(
    line_y: Pixels,
    line_height: Pixels,
    visual_line_count: usize,
    visible_height: Pixels,
) -> bool {
    let line_bottom = line_y + line_height * visual_line_count as f32;
    line_bottom >= px(0.) && line_y <= visible_height
}

fn line_intersects_range(
    text_range: &std::ops::Range<usize>,
    selected_range: &std::ops::Range<usize>,
) -> bool {
    if text_range.is_empty() {
        selected_range.start <= text_range.start && selected_range.end > text_range.start
    } else {
        selected_range.end > text_range.start && selected_range.start < text_range.end
    }
}

fn compute_alignment_offset(line: &InputLineLayout, available_width: Pixels) -> Pixels {
    match line.direction {
        TextDirection::Ltr => px(0.),
        TextDirection::Rtl => {
            let line_width = line
                .wrapped_line
                .as_ref()
                .map(|w| w.width())
                .unwrap_or(px(0.));
            available_width - line_width
        }
    }
}

fn compute_visual_line_index(y: Pixels, line_height: Pixels) -> usize {
    (y / line_height).floor() as usize
}

fn paint_multiline_selection(
    line_layouts: &[InputLineLayout],
    selected_range: &std::ops::Range<usize>,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
    line_height: Pixels,
    selection_color: Hsla,
    window: &mut Window,
) {
    for line in line_layouts {
        let line_y = line.y_offset - scroll_offset;

        if !is_line_visible(
            line_y,
            line_height,
            line.visual_line_count,
            bounds.size.height,
        ) {
            continue;
        }

        if !line_intersects_range(&line.text_range, selected_range) {
            continue;
        }

        let alignment_offset = compute_alignment_offset(line, bounds.size.width);

        if line.text_range.is_empty() {
            let empty_line_selection_width = px(6.);
            window.paint_quad(fill(
                Bounds::from_corners(
                    point(bounds.left() + alignment_offset, bounds.top() + line_y),
                    point(
                        bounds.left() + alignment_offset + empty_line_selection_width,
                        bounds.top() + line_y + line_height,
                    ),
                ),
                selection_color,
            ));
        } else if let Some(wrapped) = &line.wrapped_line {
            let line_start = line.text_range.start;
            let line_end = line.text_range.end;

            let sel_start = selected_range.start.max(line_start) - line_start;
            let sel_end = selected_range.end.min(line_end) - line_start;

            let start_pos = wrapped
                .position_for_index(sel_start, line_height)
                .unwrap_or(point(px(0.), px(0.)));
            let end_pos = wrapped
                .position_for_index(sel_end, line_height)
                .unwrap_or_else(|| {
                    let last_line_y = line_height * (line.visual_line_count - 1) as f32;
                    point(wrapped.width(), last_line_y)
                });

            let start_visual_line = compute_visual_line_index(start_pos.y, line_height);
            let end_visual_line = compute_visual_line_index(end_pos.y, line_height);

            if start_visual_line == end_visual_line {
                window.paint_quad(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + alignment_offset + start_pos.x,
                            bounds.top() + line_y + start_pos.y,
                        ),
                        point(
                            bounds.left() + alignment_offset + end_pos.x,
                            bounds.top() + line_y + start_pos.y + line_height,
                        ),
                    ),
                    selection_color,
                ));
            } else {
                let line_width = wrapped.width();

                // First visual line
                window.paint_quad(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + alignment_offset + start_pos.x,
                            bounds.top() + line_y + start_pos.y,
                        ),
                        point(
                            bounds.left() + alignment_offset + line_width,
                            bounds.top() + line_y + start_pos.y + line_height,
                        ),
                    ),
                    selection_color,
                ));

                // Middle visual lines
                for visual_line in (start_visual_line + 1)..end_visual_line {
                    let y = line_height * visual_line as f32;
                    window.paint_quad(fill(
                        Bounds::from_corners(
                            point(bounds.left() + alignment_offset, bounds.top() + line_y + y),
                            point(
                                bounds.left() + alignment_offset + line_width,
                                bounds.top() + line_y + y + line_height,
                            ),
                        ),
                        selection_color,
                    ));
                }

                // Last visual line
                window.paint_quad(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + alignment_offset,
                            bounds.top() + line_y + end_pos.y,
                        ),
                        point(
                            bounds.left() + alignment_offset + end_pos.x,
                            bounds.top() + line_y + end_pos.y + line_height,
                        ),
                    ),
                    selection_color,
                ));
            }
        }
    }
}

fn paint_multiline_placeholder(
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
    let _ = shaped_line.paint(bounds.origin, line_height, window, cx);
}

fn paint_multiline_text(
    line_layouts: &[InputLineLayout],
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
    line_height: Pixels,
    window: &mut Window,
    cx: &mut App,
) {
    for line_layout in line_layouts {
        let line_y = line_layout.y_offset - scroll_offset;

        if !is_line_visible(
            line_y,
            line_height,
            line_layout.visual_line_count,
            bounds.size.height,
        ) {
            continue;
        }

        if let Some(wrapped) = &line_layout.wrapped_line {
            let paint_pos = point(bounds.left(), bounds.top() + line_y);
            let text_align = match line_layout.direction {
                TextDirection::Ltr => TextAlign::Left,
                TextDirection::Rtl => TextAlign::Right,
            };
            let _ = wrapped.paint(paint_pos, line_height, text_align, Some(bounds), window, cx);
        }
    }
}

fn paint_multiline_marked_underline(
    line_layouts: &[InputLineLayout],
    marked_range: &std::ops::Range<usize>,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
    line_height: Pixels,
    underline_color: Hsla,
    window: &mut Window,
) {
    let underline_thickness = px(MARKED_TEXT_UNDERLINE_THICKNESS);
    let underline_offset = line_height - underline_thickness;

    for line in line_layouts {
        let line_y = line.y_offset - scroll_offset;

        if !is_line_visible(
            line_y,
            line_height,
            line.visual_line_count,
            bounds.size.height,
        ) {
            continue;
        }

        if !line_intersects_range(&line.text_range, marked_range) {
            continue;
        }

        if line.text_range.is_empty() {
            continue;
        }

        if let Some(wrapped) = &line.wrapped_line {
            let alignment_offset = compute_alignment_offset(line, bounds.size.width);
            let line_start = line.text_range.start;
            let line_end = line.text_range.end;

            let mark_start = marked_range.start.max(line_start) - line_start;
            let mark_end = marked_range.end.min(line_end) - line_start;

            let start_pos = wrapped
                .position_for_index(mark_start, line_height)
                .unwrap_or(point(px(0.), px(0.)));
            let end_pos = wrapped
                .position_for_index(mark_end, line_height)
                .unwrap_or_else(|| {
                    let last_line_y = line_height * (line.visual_line_count - 1) as f32;
                    point(wrapped.width(), last_line_y)
                });

            let start_visual_line = compute_visual_line_index(start_pos.y, line_height);
            let end_visual_line = compute_visual_line_index(end_pos.y, line_height);

            if start_visual_line == end_visual_line {
                window.paint_quad(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + alignment_offset + start_pos.x,
                            bounds.top() + line_y + start_pos.y + underline_offset,
                        ),
                        point(
                            bounds.left() + alignment_offset + end_pos.x,
                            bounds.top() + line_y + start_pos.y + line_height,
                        ),
                    ),
                    underline_color,
                ));
            } else {
                // First visual line
                window.paint_quad(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + alignment_offset + start_pos.x,
                            bounds.top() + line_y + start_pos.y + underline_offset,
                        ),
                        point(
                            bounds.left() + alignment_offset + wrapped.width(),
                            bounds.top() + line_y + start_pos.y + line_height,
                        ),
                    ),
                    underline_color,
                ));

                // Middle visual lines
                for visual_line in (start_visual_line + 1)..end_visual_line {
                    let y = line_height * visual_line as f32;
                    window.paint_quad(fill(
                        Bounds::from_corners(
                            point(
                                bounds.left() + alignment_offset,
                                bounds.top() + line_y + y + underline_offset,
                            ),
                            point(
                                bounds.left() + alignment_offset + wrapped.width(),
                                bounds.top() + line_y + y + line_height,
                            ),
                        ),
                        underline_color,
                    ));
                }

                // Last visual line
                window.paint_quad(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + alignment_offset,
                            bounds.top() + line_y + end_pos.y + underline_offset,
                        ),
                        point(
                            bounds.left() + alignment_offset + end_pos.x,
                            bounds.top() + line_y + end_pos.y + line_height,
                        ),
                    ),
                    underline_color,
                ));
            }
        }
    }
}

fn paint_multiline_cursor(
    line_layouts: &[InputLineLayout],
    cursor_offset: usize,
    _content: &str,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
    line_height: Pixels,
    cursor_color: Hsla,
    window: &mut Window,
) {
    for line in line_layouts.iter() {
        let line_y = line.y_offset - scroll_offset;

        if !is_line_visible(
            line_y,
            line_height,
            line.visual_line_count,
            bounds.size.height,
        ) {
            continue;
        }

        // Since range is non-inclusive of the end value we need to check for it explicitly
        let is_cursor_in_line = if line.text_range.is_empty() {
            cursor_offset == line.text_range.start
        } else {
            line.text_range.contains(&cursor_offset) || cursor_offset == line.text_range.end
        };

        if !is_cursor_in_line {
            continue;
        }

        let cursor_position = if let Some(wrapped) = &line.wrapped_line {
            let local_offset = cursor_offset.saturating_sub(line.text_range.start);
            wrapped
                .position_for_index(local_offset, line_height)
                .unwrap_or(point(px(0.), px(0.)))
        } else {
            point(px(0.), px(0.))
        };

        let alignment_offset = compute_alignment_offset(line, bounds.size.width);

        window.paint_quad(fill(
            Bounds::new(
                point(
                    bounds.left() + alignment_offset + cursor_position.x,
                    bounds.top() + line_y + cursor_position.y,
                ),
                size(px(CURSOR_WIDTH), line_height),
            ),
            cursor_color,
        ));
        break;
    }
}

// =============================================================================
// Single-line painting
// =============================================================================

/// State for single-line painting that pre-computes character positions.
struct SingleLinePaintState {
    content: String,
    selected_range: std::ops::Range<usize>,
    marked_range: Option<std::ops::Range<usize>>,
    cursor_offset: usize,
    scroll_offset: Pixels,
    line_height: Pixels,
    text_width: Pixels,
    is_focused: bool,
    char_positions: Vec<Pixels>,
    wrapped_line: Option<WrappedLine>,
    direction: TextDirection,
}

impl SingleLinePaintState {
    fn from_input(
        input: &Entity<InputState>,
        focus_handle: &FocusHandle,
        window: &Window,
        cx: &App,
    ) -> Self {
        let input_state = input.read(cx);

        let mut char_positions = Vec::new();
        let mut text_width = px(0.);

        if let Some(line) = input_state.line_layouts.first() {
            if let Some(wrapped) = &line.wrapped_line {
                text_width = wrapped.width();
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
                char_positions.push(text_width);
            }
        }

        let wrapped_line = input_state
            .line_layouts
            .first()
            .and_then(|l| l.wrapped_line.clone());

        let direction = input_state
            .line_layouts
            .first()
            .map(|l| l.direction)
            .unwrap_or_default();

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
            direction,
        }
    }

    fn x_for_index(&self, index: usize) -> Pixels {
        let char_index = self.content[..index.min(self.content.len())]
            .chars()
            .count();
        self.char_positions
            .get(char_index)
            .copied()
            .unwrap_or(self.text_width)
    }

    fn alignment_offset(&self, available_width: Pixels) -> Pixels {
        match self.direction {
            TextDirection::Ltr => px(0.),
            TextDirection::Rtl => available_width - self.text_width,
        }
    }
}

fn paint_singleline(
    input: &Entity<InputState>,
    focus_handle: &FocusHandle,
    bounds: Bounds<Pixels>,
    text_style: &TextStyle,
    placeholder: Option<&SharedString>,
    colors: &PaintColors,
    cursor_visible: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let state = SingleLinePaintState::from_input(input, focus_handle, window, cx);

    if !state.selected_range.is_empty() {
        paint_singleline_selection(&state, bounds, colors.selection, window);
    }

    if state.content.is_empty() {
        if let Some(placeholder_str) = placeholder {
            if !placeholder_str.is_empty() {
                paint_singleline_placeholder(placeholder_str, bounds, text_style, window, cx);
            }
        }
    } else {
        paint_singleline_text(&state, bounds, window, cx);
    }

    if let Some(marked_range) = &state.marked_range {
        if !marked_range.is_empty() {
            paint_singleline_marked_underline(&state, marked_range, bounds, colors.cursor, window);
        }
    }

    if state.is_focused && state.selected_range.is_empty() && cursor_visible {
        paint_singleline_cursor(&state, bounds, colors.cursor, window);
    }
}

fn paint_singleline_selection(
    state: &SingleLinePaintState,
    bounds: Bounds<Pixels>,
    selection_color: Hsla,
    window: &mut Window,
) {
    let alignment_offset = state.alignment_offset(bounds.size.width);
    let start_x =
        state.x_for_index(state.selected_range.start) - state.scroll_offset + alignment_offset;
    let end_x =
        state.x_for_index(state.selected_range.end) - state.scroll_offset + alignment_offset;

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

fn paint_singleline_placeholder(
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

    let y_offset = (bounds.size.height - line_height).max(px(0.)) / 2.0;
    let paint_origin = point(bounds.origin.x, bounds.origin.y + y_offset);

    let _ = shaped_line.paint(paint_origin, line_height, window, cx);
}

fn paint_singleline_text(
    state: &SingleLinePaintState,
    bounds: Bounds<Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(wrapped_line) = &state.wrapped_line else {
        return;
    };

    let y_offset = (bounds.size.height - state.line_height).max(px(0.)) / 2.0;
    let paint_origin = point(
        bounds.origin.x - state.scroll_offset,
        bounds.origin.y + y_offset,
    );

    let text_align = match state.direction {
        TextDirection::Ltr => TextAlign::Left,
        TextDirection::Rtl => TextAlign::Right,
    };
    let _ = wrapped_line.paint(
        paint_origin,
        state.line_height,
        text_align,
        Some(bounds),
        window,
        cx,
    );
}

fn paint_singleline_marked_underline(
    state: &SingleLinePaintState,
    marked_range: &std::ops::Range<usize>,
    bounds: Bounds<Pixels>,
    underline_color: Hsla,
    window: &mut Window,
) {
    let alignment_offset = state.alignment_offset(bounds.size.width);
    let start_x = state.x_for_index(marked_range.start) - state.scroll_offset + alignment_offset;
    let end_x = state.x_for_index(marked_range.end) - state.scroll_offset + alignment_offset;

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

fn paint_singleline_cursor(
    state: &SingleLinePaintState,
    bounds: Bounds<Pixels>,
    cursor_color: Hsla,
    window: &mut Window,
) {
    let alignment_offset = state.alignment_offset(bounds.size.width);
    let cursor_x = state.x_for_index(state.cursor_offset) - state.scroll_offset + alignment_offset;

    let y_offset = (bounds.size.height - state.line_height).max(px(0.)) / 2.0;

    window.paint_quad(fill(
        Bounds::new(
            point(bounds.left() + cursor_x, bounds.top() + y_offset),
            size(px(CURSOR_WIDTH), state.line_height),
        ),
        cursor_color,
    ));
}

impl Focusable for Input {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl IntoElement for Input {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
