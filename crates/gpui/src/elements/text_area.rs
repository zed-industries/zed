//! A multi-line text area element.
//!
//! The `TextArea` element renders an [`Input`] entity as a multi-line text editor.
//! It handles layout, painting, and user interaction while the `Input` manages
//! the text content and selection state.
//!
//! # Example
//!
//! ```ignore
//! struct MyView {
//!     input: Entity<Input>,
//! }
//!
//! impl MyView {
//!     fn new(cx: &mut Context<Self>) -> Self {
//!         Self {
//!             input: cx.new(Input::new),
//!         }
//!     }
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         text_area(&self.input)
//!             .bg(cx.theme().colors().editor_background)
//!             .text_color(cx.theme().colors().text)
//!     }
//! }
//! ```

use crate::{
    Action, App, Bounds, ContentMask, Context, CursorStyle, DispatchPhase, Element, ElementId,
    ElementInputHandler, Entity, FocusHandle, Focusable, GlobalElementId, Hitbox, HitboxBehavior,
    Hsla, InspectorElementId, InteractiveElement, Interactivity, IntoElement, LayoutId, Length,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, ScrollWheelEvent,
    SharedString, StyleRefinement, Styled, TextRun, TextStyle, Window, fill, point, px, relative,
    rgba, size,
};

use super::input::{Input, InputLineLayout};

/// Default selection highlight color (blue with transparency).
const DEFAULT_SELECTION_COLOR: u32 = 0x3584e488;

/// Width of the cursor in pixels.
const CURSOR_WIDTH: f32 = 2.0;

/// Thickness of the underline for marked (IME composition) text.
const MARKED_TEXT_UNDERLINE_THICKNESS: f32 = 2.0;

/// Creates a new `TextArea` element for the given `Input` entity.
#[track_caller]
pub fn text_area(input: &Entity<Input>) -> TextArea {
    let mut text_area = TextArea {
        input: input.clone(),
        interactivity: Interactivity::new(),
        placeholder: None,
        selection_color: None,
        cursor_color: None,
    };
    text_area.register_actions();
    text_area
}

/// A multi-line text editing element.
///
/// `TextArea` renders an [`Input`] entity and provides:
/// - Text display with wrapping
/// - Selection highlighting
/// - Cursor rendering
/// - Scrolling for overflow content
/// - Keyboard navigation and editing
/// - Mouse selection
///
/// Use the [`text_area`] function to create a `TextArea`.
pub struct TextArea {
    input: Entity<Input>,
    interactivity: Interactivity,
    placeholder: Option<SharedString>,
    selection_color: Option<Hsla>,
    cursor_color: Option<Hsla>,
}

impl TextArea {
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

    fn register_actions(&mut self) {
        register_action(&mut self.interactivity, &self.input, Input::left);
        register_action(&mut self.interactivity, &self.input, Input::right);
        register_action(&mut self.interactivity, &self.input, Input::up);
        register_action(&mut self.interactivity, &self.input, Input::down);
        register_action(&mut self.interactivity, &self.input, Input::select_left);
        register_action(&mut self.interactivity, &self.input, Input::select_right);
        register_action(&mut self.interactivity, &self.input, Input::select_up);
        register_action(&mut self.interactivity, &self.input, Input::select_down);
        register_action(&mut self.interactivity, &self.input, Input::select_all);
        register_action(&mut self.interactivity, &self.input, Input::home);
        register_action(&mut self.interactivity, &self.input, Input::end);
        register_action(
            &mut self.interactivity,
            &self.input,
            Input::move_to_beginning,
        );
        register_action(&mut self.interactivity, &self.input, Input::move_to_end);
        register_action(
            &mut self.interactivity,
            &self.input,
            Input::select_to_beginning,
        );
        register_action(&mut self.interactivity, &self.input, Input::select_to_end);
        register_action(&mut self.interactivity, &self.input, Input::word_left);
        register_action(&mut self.interactivity, &self.input, Input::word_right);
        register_action(
            &mut self.interactivity,
            &self.input,
            Input::select_word_left,
        );
        register_action(
            &mut self.interactivity,
            &self.input,
            Input::select_word_right,
        );
        register_action(&mut self.interactivity, &self.input, Input::backspace);
        register_action(&mut self.interactivity, &self.input, Input::delete);
        register_action(&mut self.interactivity, &self.input, Input::enter);
        register_action(&mut self.interactivity, &self.input, Input::paste);
        register_action(&mut self.interactivity, &self.input, Input::copy);
        register_action(&mut self.interactivity, &self.input, Input::cut);
    }
}

fn register_action<A: Action>(
    interactivity: &mut Interactivity,
    input: &Entity<Input>,
    listener: fn(&mut Input, &A, &mut Window, &mut Context<Input>),
) {
    let input = input.clone();
    interactivity.on_action::<A>(move |action, window, cx| {
        input.update(cx, |input, cx| {
            listener(input, action, window, cx);
        });
    });
}

impl Styled for TextArea {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for TextArea {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

/// Layout state passed from request_layout to prepaint.
pub struct TextAreaLayoutState {
    text_style: TextStyle,
}

/// Prepaint state passed from prepaint to paint.
pub struct TextAreaPrepaintState {
    hitbox: Option<Hitbox>,
}

impl Element for TextArea {
    type RequestLayoutState = TextAreaLayoutState;
    type PrepaintState = TextAreaPrepaintState;

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

                    let mut style = style.clone();
                    if let Length::Auto = style.size.width {
                        style.size.width = relative(1.).into();
                    }
                    if let Length::Auto = style.size.height {
                        style.size.height = relative(1.).into();
                    }
                    window.request_layout(style, None, cx)
                })
            },
        );

        (
            layout_id,
            TextAreaLayoutState {
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

        self.input.update(cx, |input, _cx| {
            input.available_height = bounds.size.height;
            input.update_line_layouts(bounds.size.width, line_height, text_color, window);
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

        TextAreaPrepaintState { hitbox }
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
        let selection_color = self.selection_color;
        let cursor_color = self.cursor_color;
        let text_style = layout_state.text_style.clone();

        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            prepaint_state.hitbox.as_ref(),
            window,
            cx,
            |_style, window, cx| {
                register_mouse_handlers(&input, bounds, window, cx);

                let paint_state = PaintState::from_input(&input, &focus_handle, bounds, window, cx);
                let colors = PaintColors::new(text_style.color, selection_color, cursor_color);

                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    if !paint_state.selected_range.is_empty() {
                        paint_selection(
                            &paint_state.line_layouts,
                            &paint_state.selected_range,
                            bounds,
                            paint_state.scroll_offset,
                            paint_state.line_height,
                            colors.selection,
                            window,
                        );
                    }

                    if paint_state.content.is_empty() {
                        if let Some(placeholder_str) = placeholder.as_ref() {
                            if !placeholder_str.is_empty() {
                                paint_placeholder(placeholder_str, bounds, &text_style, window, cx);
                            }
                        }
                    } else {
                        paint_text(
                            &paint_state.line_layouts,
                            bounds,
                            paint_state.scroll_offset,
                            paint_state.line_height,
                            window,
                            cx,
                        );
                    }

                    if let Some(marked_range) = &paint_state.marked_range {
                        if !marked_range.is_empty() {
                            paint_marked_text_underline(
                                &paint_state.line_layouts,
                                marked_range,
                                bounds,
                                paint_state.scroll_offset,
                                paint_state.line_height,
                                colors.cursor,
                                window,
                            );
                        }
                    }

                    if paint_state.is_focused && paint_state.selected_range.is_empty() {
                        paint_cursor(
                            &paint_state.line_layouts,
                            paint_state.cursor_offset,
                            &paint_state.content,
                            bounds,
                            paint_state.scroll_offset,
                            paint_state.line_height,
                            colors.cursor,
                            window,
                        );
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
    line_layouts: Vec<InputLineLayout>,
    scroll_offset: Pixels,
    line_height: Pixels,
    is_focused: bool,
}

impl PaintState {
    fn from_input(
        input: &Entity<Input>,
        focus_handle: &FocusHandle,
        _bounds: Bounds<Pixels>,
        window: &Window,
        cx: &App,
    ) -> Self {
        let input_state = input.read(cx);
        Self {
            content: input_state.content().to_string(),
            selected_range: input_state.selected_range().clone(),
            marked_range: input_state.marked_range().cloned(),
            cursor_offset: input_state.cursor_offset(),
            line_layouts: input_state.line_layouts.clone(),
            scroll_offset: input_state.scroll_offset,
            line_height: input_state.line_height,
            is_focused: focus_handle.is_focused(window),
        }
    }
}

/// Colors used for painting.
struct PaintColors {
    selection: Hsla,
    cursor: Hsla,
}

impl PaintColors {
    fn new(text_color: Hsla, selection_color: Option<Hsla>, cursor_color: Option<Hsla>) -> Self {
        Self {
            selection: selection_color.unwrap_or_else(|| rgba(DEFAULT_SELECTION_COLOR).into()),
            cursor: cursor_color.unwrap_or(text_color),
        }
    }
}

/// Registers all mouse event handlers for the text area.
fn register_mouse_handlers(
    input: &Entity<Input>,
    bounds: Bounds<Pixels>,
    window: &mut Window,
    cx: &App,
) {
    register_mouse_down_handler(input.clone(), bounds, window);
    register_mouse_up_handler(input.clone(), window);
    register_mouse_move_handler(input.clone(), bounds, window);
    register_scroll_handler(input.clone(), bounds, window, cx);
}

/// Converts a screen position to a position relative to the text area origin,
/// adjusted for scroll offset.
fn screen_to_text_position(
    screen_position: Point<Pixels>,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
) -> Point<Pixels> {
    point(
        screen_position.x - bounds.origin.x,
        screen_position.y - bounds.origin.y + scroll_offset,
    )
}

fn register_mouse_down_handler(input: Entity<Input>, bounds: Bounds<Pixels>, window: &mut Window) {
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

fn register_mouse_up_handler(input: Entity<Input>, window: &mut Window) {
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

fn register_mouse_move_handler(input: Entity<Input>, bounds: Bounds<Pixels>, window: &mut Window) {
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

fn register_scroll_handler(
    input: Entity<Input>,
    bounds: Bounds<Pixels>,
    window: &mut Window,
    cx: &App,
) {
    let max_scroll = compute_max_scroll(&input, bounds, cx);

    window.on_mouse_event(move |event: &ScrollWheelEvent, phase, _window, cx| {
        if phase != DispatchPhase::Bubble {
            return;
        }
        if !bounds.contains(&event.position) {
            return;
        }

        let pixel_delta = event.delta.pixel_delta(px(20.));
        input.update(cx, |input, cx| {
            input.scroll_offset = (input.scroll_offset - pixel_delta.y).clamp(px(0.), max_scroll);
            cx.notify();
        });
    });
}

/// Computes the maximum scroll offset based on content height and visible area.
fn compute_max_scroll(input: &Entity<Input>, bounds: Bounds<Pixels>, cx: &App) -> Pixels {
    let total_height = input.read(cx).total_content_height();
    (total_height - bounds.size.height).max(px(0.))
}

/// Determines if a line is within the visible area.
fn is_line_visible(
    line_y: Pixels,
    line_height: Pixels,
    visual_line_count: usize,
    visible_height: Pixels,
) -> bool {
    let line_bottom = line_y + line_height * visual_line_count as f32;
    line_bottom >= px(0.) && line_y <= visible_height
}

fn paint_selection(
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

        if line.text_range.is_empty() {
            paint_empty_line_selection(bounds, line_y, line_height, selection_color, window);
        } else if let Some(wrapped) = &line.wrapped_line {
            paint_wrapped_line_selection(
                wrapped,
                line,
                selected_range,
                bounds,
                line_y,
                line_height,
                selection_color,
                window,
            );
        }
    }
}

/// Checks if a line's text range intersects with the selection range.
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

/// Paints selection highlight for an empty line (just a small marker).
fn paint_empty_line_selection(
    bounds: Bounds<Pixels>,
    line_y: Pixels,
    line_height: Pixels,
    selection_color: Hsla,
    window: &mut Window,
) {
    let empty_line_selection_width = px(4.);
    window.paint_quad(fill(
        Bounds::from_corners(
            point(bounds.left(), bounds.top() + line_y),
            point(
                bounds.left() + empty_line_selection_width,
                bounds.top() + line_y + line_height,
            ),
        ),
        selection_color,
    ));
}

/// Paints selection highlight for a wrapped line, handling multi-visual-line selections.
fn paint_wrapped_line_selection(
    wrapped: &crate::WrappedLine,
    line: &InputLineLayout,
    selected_range: &std::ops::Range<usize>,
    bounds: Bounds<Pixels>,
    line_y: Pixels,
    line_height: Pixels,
    selection_color: Hsla,
    window: &mut Window,
) {
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
        paint_single_visual_line_selection(
            bounds,
            line_y,
            start_pos,
            end_pos,
            line_height,
            selection_color,
            window,
        );
    } else {
        paint_multi_visual_line_selection(
            wrapped,
            bounds,
            line_y,
            start_pos,
            end_pos,
            start_visual_line,
            end_visual_line,
            line_height,
            selection_color,
            window,
        );
    }
}

/// Computes which visual line (within a wrapped line) a y-coordinate falls on.
fn compute_visual_line_index(y: Pixels, line_height: Pixels) -> usize {
    (y / line_height).floor() as usize
}

fn paint_single_visual_line_selection(
    bounds: Bounds<Pixels>,
    line_y: Pixels,
    start_pos: Point<Pixels>,
    end_pos: Point<Pixels>,
    line_height: Pixels,
    selection_color: Hsla,
    window: &mut Window,
) {
    window.paint_quad(fill(
        Bounds::from_corners(
            point(
                bounds.left() + start_pos.x,
                bounds.top() + line_y + start_pos.y,
            ),
            point(
                bounds.left() + end_pos.x,
                bounds.top() + line_y + start_pos.y + line_height,
            ),
        ),
        selection_color,
    ));
}

fn paint_multi_visual_line_selection(
    wrapped: &crate::WrappedLine,
    bounds: Bounds<Pixels>,
    line_y: Pixels,
    start_pos: Point<Pixels>,
    end_pos: Point<Pixels>,
    start_visual_line: usize,
    end_visual_line: usize,
    line_height: Pixels,
    selection_color: Hsla,
    window: &mut Window,
) {
    let line_width = wrapped.width();

    // First visual line: from start position to end of line
    window.paint_quad(fill(
        Bounds::from_corners(
            point(
                bounds.left() + start_pos.x,
                bounds.top() + line_y + start_pos.y,
            ),
            point(
                bounds.left() + line_width,
                bounds.top() + line_y + start_pos.y + line_height,
            ),
        ),
        selection_color,
    ));

    // Middle visual lines: full width
    for visual_line in (start_visual_line + 1)..end_visual_line {
        let y = line_height * visual_line as f32;
        window.paint_quad(fill(
            Bounds::from_corners(
                point(bounds.left(), bounds.top() + line_y + y),
                point(
                    bounds.left() + line_width,
                    bounds.top() + line_y + y + line_height,
                ),
            ),
            selection_color,
        ));
    }

    // Last visual line: from start of line to end position
    window.paint_quad(fill(
        Bounds::from_corners(
            point(bounds.left(), bounds.top() + line_y + end_pos.y),
            point(
                bounds.left() + end_pos.x,
                bounds.top() + line_y + end_pos.y + line_height,
            ),
        ),
        selection_color,
    ));
}

/// Paints an underline beneath marked (IME composition) text.
fn paint_marked_text_underline(
    line_layouts: &[InputLineLayout],
    marked_range: &std::ops::Range<usize>,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
    line_height: Pixels,
    underline_color: Hsla,
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

        if !line_intersects_range(&line.text_range, marked_range) {
            continue;
        }

        if line.text_range.is_empty() {
            continue;
        }

        if let Some(wrapped) = &line.wrapped_line {
            paint_wrapped_line_underline(
                wrapped,
                line,
                marked_range,
                bounds,
                line_y,
                line_height,
                underline_color,
                window,
            );
        }
    }
}

/// Paints underline for marked text within a wrapped line.
fn paint_wrapped_line_underline(
    wrapped: &crate::WrappedLine,
    line: &InputLineLayout,
    marked_range: &std::ops::Range<usize>,
    bounds: Bounds<Pixels>,
    line_y: Pixels,
    line_height: Pixels,
    underline_color: Hsla,
    window: &mut Window,
) {
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

    let underline_thickness = px(MARKED_TEXT_UNDERLINE_THICKNESS);
    let underline_offset = line_height - underline_thickness;

    if start_visual_line == end_visual_line {
        window.paint_quad(fill(
            Bounds::from_corners(
                point(
                    bounds.left() + start_pos.x,
                    bounds.top() + line_y + start_pos.y + underline_offset,
                ),
                point(
                    bounds.left() + end_pos.x,
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
                    bounds.left() + start_pos.x,
                    bounds.top() + line_y + start_pos.y + underline_offset,
                ),
                point(
                    bounds.left() + wrapped.width(),
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
                    point(bounds.left(), bounds.top() + line_y + y + underline_offset),
                    point(
                        bounds.left() + wrapped.width(),
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
                    bounds.left(),
                    bounds.top() + line_y + end_pos.y + underline_offset,
                ),
                point(
                    bounds.left() + end_pos.x,
                    bounds.top() + line_y + end_pos.y + line_height,
                ),
            ),
            underline_color,
        ));
    }
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
    let _ = shaped_line.paint(bounds.origin, line_height, window, cx);
}

fn paint_text(
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
            let _ = wrapped.paint(
                paint_pos,
                line_height,
                crate::TextAlign::Left,
                Some(bounds),
                window,
                cx,
            );
        }
    }
}

fn paint_cursor(
    line_layouts: &[InputLineLayout],
    cursor_offset: usize,
    content: &str,
    bounds: Bounds<Pixels>,
    scroll_offset: Pixels,
    line_height: Pixels,
    cursor_color: Hsla,
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

        if !is_cursor_in_line(cursor_offset, &line.text_range, content.len()) {
            continue;
        }

        let cursor_position = compute_cursor_position(line, cursor_offset, line_height);
        paint_cursor_at_position(
            bounds,
            line_y,
            cursor_position,
            line_height,
            cursor_color,
            window,
        );
        break;
    }
}

/// Determines if the cursor is within the given line's text range.
fn is_cursor_in_line(
    cursor_offset: usize,
    text_range: &std::ops::Range<usize>,
    content_len: usize,
) -> bool {
    if text_range.is_empty() {
        cursor_offset == text_range.start
    } else {
        text_range.contains(&cursor_offset)
            || (cursor_offset == text_range.end && cursor_offset == content_len)
    }
}

/// Computes the visual position of the cursor within a line.
fn compute_cursor_position(
    line: &InputLineLayout,
    cursor_offset: usize,
    line_height: Pixels,
) -> Point<Pixels> {
    if let Some(wrapped) = &line.wrapped_line {
        let local_offset = cursor_offset.saturating_sub(line.text_range.start);
        wrapped
            .position_for_index(local_offset, line_height)
            .unwrap_or(point(px(0.), px(0.)))
    } else {
        point(px(0.), px(0.))
    }
}

fn paint_cursor_at_position(
    bounds: Bounds<Pixels>,
    line_y: Pixels,
    cursor_position: Point<Pixels>,
    line_height: Pixels,
    cursor_color: Hsla,
    window: &mut Window,
) {
    window.paint_quad(fill(
        Bounds::new(
            point(
                bounds.left() + cursor_position.x,
                bounds.top() + line_y + cursor_position.y,
            ),
            size(px(CURSOR_WIDTH), line_height),
        ),
        cursor_color,
    ));
}

impl Focusable for TextArea {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl IntoElement for TextArea {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
