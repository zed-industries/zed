use std::rc::Rc;

use crate::util::FluentBuilder;
use crate::{
    AbsoluteLength, AnyElement, App, AppContext, Context, DefiniteLength, DragMoveEvent, ElementId,
    Entity, InteractiveElement, IntoElement, Length, ParentElement, Pixels, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div, px,
};

/// The width of resize handles in pixels.
pub const RESIZE_HANDLE_WIDTH: f32 = 8.0;

/// Marker type for drag events on resize handles.
#[derive(Debug, Clone)]
pub struct DraggedSplitHandle(pub usize);

/// Defines how a pane can be resized.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SplitResizeBehavior {
    /// The pane cannot be resized.
    None,
    /// The pane can be resized with a default minimum size (5% of container).
    Resizable,
    /// The pane can be resized with a custom minimum size (as a fraction of the container).
    MinSize(f32),
}

impl SplitResizeBehavior {
    /// Returns true if this pane can be resized.
    pub fn is_resizable(&self) -> bool {
        *self != SplitResizeBehavior::None
    }

    /// Returns the minimum size as a fraction of the container, if resizable.
    pub fn min_size(&self) -> Option<f32> {
        match self {
            SplitResizeBehavior::None => None,
            SplitResizeBehavior::Resizable => Some(0.05),
            SplitResizeBehavior::MinSize(min_size) => Some(*min_size),
        }
    }
}

/// State entity that tracks the widths of resizable splits.
pub struct ResizableSplitState {
    initial_sizes: Vec<DefiniteLength>,
    resize_behavior: Vec<SplitResizeBehavior>,
    widths: Vec<DefiniteLength>,
    visible_widths: Vec<DefiniteLength>,
    cached_bounds_width: Pixels,
}

impl ResizableSplitState {
    /// Creates a new state with the given initial sizes.
    /// All panes are non-resizable by default until `set_resize_behavior` is called.
    pub fn new(initial_sizes: Vec<impl Into<DefiniteLength>>, _cx: &mut App) -> Self {
        let initial_sizes: Vec<DefiniteLength> =
            initial_sizes.into_iter().map(Into::into).collect();
        let pane_count = initial_sizes.len();
        Self {
            widths: initial_sizes.clone(),
            visible_widths: initial_sizes.clone(),
            resize_behavior: vec![SplitResizeBehavior::None; pane_count],
            initial_sizes,
            cached_bounds_width: Default::default(),
        }
    }

    /// Returns the number of panes.
    pub fn pane_count(&self) -> usize {
        self.initial_sizes.len()
    }

    /// Returns the initial sizes.
    pub fn initial_sizes(&self) -> &[DefiniteLength] {
        &self.initial_sizes
    }

    /// Returns the resize behavior for each pane.
    pub fn resize_behavior(&self) -> &[SplitResizeBehavior] {
        &self.resize_behavior
    }

    /// Sets the resize behavior for each pane.
    pub fn set_resize_behavior(&mut self, behavior: Vec<SplitResizeBehavior>) {
        assert_eq!(
            behavior.len(),
            self.initial_sizes.len(),
            "Resize behavior count must match pane count"
        );
        self.resize_behavior = behavior;
    }

    /// Returns the current visible widths.
    pub fn visible_widths(&self) -> &[DefiniteLength] {
        &self.visible_widths
    }

    /// Returns the current visible widths as Length values.
    pub fn lengths(&self) -> Vec<Length> {
        self.visible_widths
            .iter()
            .map(|w| Length::Definite(*w))
            .collect()
    }

    /// Gets the cached bounds width.
    pub fn cached_bounds_width(&self) -> Pixels {
        self.cached_bounds_width
    }

    /// Sets the cached bounds width.
    pub fn set_cached_bounds_width(&mut self, width: Pixels) {
        self.cached_bounds_width = width;
    }

    /// Commits visible widths to the stored widths (called when drag ends).
    pub fn commit_widths(&mut self) {
        self.widths = self.visible_widths.clone();
    }

    /// Converts a DefiniteLength to a fraction of the container width.
    fn get_fraction(length: &DefiniteLength, bounds_width: Pixels, rem_size: Pixels) -> f32 {
        match length {
            DefiniteLength::Absolute(AbsoluteLength::Pixels(pixels)) => *pixels / bounds_width,
            DefiniteLength::Absolute(AbsoluteLength::Rems(rems_width)) => {
                rems_width.to_pixels(rem_size) / bounds_width
            }
            DefiniteLength::Fraction(fraction) => *fraction,
        }
    }

    /// Handles double-click to reset a pane to its initial size.
    pub fn on_double_click(&mut self, double_click_position: usize, window: &mut Window) {
        let bounds_width = self.cached_bounds_width;
        let rem_size = window.rem_size();
        let initial_sizes: Vec<f32> = self
            .initial_sizes
            .iter()
            .map(|length| Self::get_fraction(length, bounds_width, rem_size))
            .collect();
        let mut widths: Vec<f32> = self
            .widths
            .iter()
            .map(|length| Self::get_fraction(length, bounds_width, rem_size))
            .collect();

        Self::reset_to_initial_size(
            double_click_position,
            &mut widths,
            &initial_sizes,
            &self.resize_behavior,
        );
        self.widths = widths
            .iter()
            .map(|&f| DefiniteLength::Fraction(f))
            .collect();
        self.visible_widths = self.widths.clone();
    }

    /// Resets a specific pane to its initial size, adjusting neighbors as needed.
    pub fn reset_to_initial_size(
        pane_idx: usize,
        widths: &mut [f32],
        initial_sizes: &[f32],
        resize_behavior: &[SplitResizeBehavior],
    ) {
        let diff = initial_sizes[pane_idx] - widths[pane_idx];

        let left_diff: f32 =
            initial_sizes[..pane_idx].iter().sum::<f32>() - widths[..pane_idx].iter().sum::<f32>();
        let right_diff: f32 = initial_sizes[pane_idx + 1..].iter().sum::<f32>()
            - widths[pane_idx + 1..].iter().sum::<f32>();

        let go_left_first = if diff < 0.0 {
            left_diff > right_diff
        } else {
            left_diff < right_diff
        };

        if !go_left_first {
            let diff_remaining =
                Self::propagate_resize_diff(diff, pane_idx, widths, resize_behavior, 1);

            if diff_remaining != 0.0 && pane_idx > 0 {
                Self::propagate_resize_diff(diff_remaining, pane_idx, widths, resize_behavior, -1);
            }
        } else {
            let diff_remaining =
                Self::propagate_resize_diff(diff, pane_idx, widths, resize_behavior, -1);

            if diff_remaining != 0.0 {
                Self::propagate_resize_diff(diff_remaining, pane_idx, widths, resize_behavior, 1);
            }
        }
    }

    /// Handles drag move events to resize panes.
    pub fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedSplitHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;

        let mut pane_position = 0.0;
        let rem_size = window.rem_size();
        let bounds_width = bounds.right() - bounds.left();
        let pane_idx = drag_event.drag(cx).0;

        self.cached_bounds_width = bounds_width;

        let handle_width = Self::get_fraction(
            &DefiniteLength::Absolute(AbsoluteLength::Pixels(px(RESIZE_HANDLE_WIDTH))),
            bounds_width,
            rem_size,
        );

        let mut widths: Vec<f32> = self
            .widths
            .iter()
            .map(|length| Self::get_fraction(length, bounds_width, rem_size))
            .collect();

        for length in widths[0..=pane_idx].iter() {
            pane_position += length + handle_width;
        }

        let mut total_length_ratio = pane_position;
        for length in widths[pane_idx + 1..].iter() {
            total_length_ratio += length;
        }
        let pane_count = self.resize_behavior.len();
        total_length_ratio += (pane_count - 1 - pane_idx) as f32 * handle_width;

        let drag_fraction = (drag_position.x - bounds.left()) / bounds_width;
        let drag_fraction = drag_fraction * total_length_ratio;
        let diff = drag_fraction - pane_position - handle_width / 2.0;

        Self::drag_handle(diff, pane_idx, &mut widths, &self.resize_behavior);

        self.visible_widths = widths
            .iter()
            .map(|&f| DefiniteLength::Fraction(f))
            .collect();
    }

    /// Handles the resize from dragging a handle.
    pub fn drag_handle(
        diff: f32,
        pane_idx: usize,
        widths: &mut [f32],
        resize_behavior: &[SplitResizeBehavior],
    ) {
        if diff > 0.0 {
            Self::propagate_resize_diff(diff, pane_idx, widths, resize_behavior, 1);
        } else {
            Self::propagate_resize_diff(-diff, pane_idx + 1, widths, resize_behavior, -1);
        }
    }

    /// Propagates a resize diff across panes, respecting minimum sizes.
    pub fn propagate_resize_diff(
        diff: f32,
        pane_idx: usize,
        widths: &mut [f32],
        resize_behavior: &[SplitResizeBehavior],
        direction: i8,
    ) -> f32 {
        let mut diff_remaining = diff;
        if resize_behavior[pane_idx].min_size().is_none() {
            return diff;
        }

        let step_right;
        let step_left;
        if direction < 0 {
            step_right = 0;
            step_left = 1;
        } else {
            step_right = 1;
            step_left = 0;
        }
        if pane_idx == 0 && direction < 0 {
            return diff;
        }
        let mut curr_pane = pane_idx + step_right - step_left;

        while diff_remaining != 0.0 && curr_pane < widths.len() {
            let Some(min_size) = resize_behavior[curr_pane].min_size() else {
                if curr_pane == 0 {
                    break;
                }
                curr_pane -= step_left;
                curr_pane += step_right;
                continue;
            };

            let curr_width = widths[curr_pane] - diff_remaining;
            widths[curr_pane] = curr_width;

            if min_size > curr_width {
                diff_remaining = min_size - curr_width;
                widths[curr_pane] = min_size;
            } else {
                diff_remaining = 0.0;
                break;
            }
            if curr_pane == 0 {
                break;
            }
            curr_pane -= step_left;
            curr_pane += step_right;
        }
        widths[pane_idx] = widths[pane_idx] + (diff - diff_remaining);

        diff_remaining
    }
}

/// Renders resize handles as an overlay.
///
/// This renders invisible resize handles that can be placed as an absolute overlay
/// on top of content. It creates spacers matching each pane width with resize handles between them.
pub fn render_resize_handles(
    state: &Entity<ResizableSplitState>,
    handle_color: crate::Hsla,
    handle_hover_color: crate::Hsla,
    render_spacer: impl Fn(Length) -> AnyElement,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let state_read = state.read(cx);
    let pane_widths = state_read.lengths();
    let resize_behavior = Rc::new(state_read.resize_behavior.clone());

    let spacers: Vec<AnyElement> = pane_widths
        .iter()
        .map(|&width| render_spacer(width))
        .collect();

    let mut pane_idx = 0;
    let mut resize_behavior_iter = resize_behavior.iter();

    let mut elements: Vec<AnyElement> = Vec::new();

    for (i, spacer) in spacers.into_iter().enumerate() {
        elements.push(spacer);

        if i < pane_widths.len() - 1 {
            let current_idx = pane_idx;
            let state = state.clone();

            let handle = window.with_id(current_idx, |window| {
                let mut resize_divider = div()
                    .id(current_idx)
                    .relative()
                    .top_0()
                    .w(px(1.1))
                    .h_full()
                    .bg(handle_color);

                let mut resize_handle = div()
                    .id("split-resize-handle")
                    .absolute()
                    .left_neg_0p5()
                    .w(px(RESIZE_HANDLE_WIDTH))
                    .h_full();

                if resize_behavior_iter
                    .next()
                    .is_some_and(SplitResizeBehavior::is_resizable)
                {
                    let hovered = window.use_state(cx, |_window, _cx| false);
                    let is_hovered = *hovered.read(cx);

                    resize_divider = resize_divider.when(is_hovered, |d| d.bg(handle_hover_color));

                    resize_handle = resize_handle
                        .on_hover(move |&was_hovered, _, cx| hovered.write(cx, was_hovered))
                        .cursor_col_resize()
                        .on_click({
                            let state = state.clone();
                            move |event, window, cx| {
                                if event.click_count() >= 2 {
                                    state.update(cx, |state, _| {
                                        state.on_double_click(current_idx, window);
                                    })
                                }
                                cx.stop_propagation();
                            }
                        })
                        .on_drag(
                            DraggedSplitHandle(current_idx),
                            |_, _offset, _window, cx| cx.new(|_cx| crate::Empty),
                        );
                }

                resize_divider.child(resize_handle).into_any_element()
            });

            elements.push(handle);
            pane_idx += 1;
        }
    }

    div()
        .id("resize-handles")
        .absolute()
        .inset_0()
        .w_full()
        .flex()
        .flex_row()
        .children(elements)
        .into_any_element()
}

/// Adds `on_children_prepainted` handler to track container bounds for resize operations.
///
/// This must be called on a `Div` before `.id()` is called, because `on_children_prepainted`
/// is only available on `Div`, not on `Stateful<Div>`.
pub fn with_bounds_tracking(
    container: crate::Div,
    state: &Entity<ResizableSplitState>,
) -> crate::Div {
    let state_for_bounds = state.downgrade();

    container.on_children_prepainted({
        move |bounds, _, cx| {
            state_for_bounds
                .update(cx, |state, _| {
                    if !bounds.is_empty() {
                        state.set_cached_bounds_width(bounds[0].right() - bounds[0].left());
                    }
                })
                .ok();
        }
    })
}

/// Adds drag/drop handlers for resize operations to a stateful div.
///
/// This should be called after `.id()` has been called on the div.
pub fn with_drag_resize_handlers(
    container: crate::Stateful<crate::Div>,
    state: &Entity<ResizableSplitState>,
) -> crate::Stateful<crate::Div> {
    let state_for_drag = state.downgrade();
    let state_for_drop = state.downgrade();

    container
        .on_drag_move::<DraggedSplitHandle>({
            move |event, window, cx| {
                state_for_drag
                    .update(cx, |state, cx| {
                        state.on_drag_move(event, window, cx);
                    })
                    .ok();
            }
        })
        .on_drop::<DraggedSplitHandle>(move |_, _, cx| {
            state_for_drop
                .update(cx, |state, _| {
                    state.commit_widths();
                })
                .ok();
        })
}

/// A container element that arranges children horizontally with resizable dividers between them.
#[derive(IntoElement)]
pub struct ResizableSplits {
    id: ElementId,
    children: Vec<AnyElement>,
    state: Entity<ResizableSplitState>,
    handle_color: Option<crate::Hsla>,
    handle_hover_color: Option<crate::Hsla>,
}

impl ResizableSplits {
    /// Creates a new resizable splits container with the given state.
    pub fn new(id: impl Into<ElementId>, state: Entity<ResizableSplitState>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            state,
            handle_color: None,
            handle_hover_color: None,
        }
    }

    /// Makes all panes resizable with default minimum sizes.
    pub fn resizable(self, cx: &mut App) -> Self {
        let pane_count = self.state.read(cx).pane_count();
        self.state.update(cx, |state, _| {
            state.set_resize_behavior(vec![SplitResizeBehavior::Resizable; pane_count]);
        });
        self
    }

    /// Sets custom resize behavior for each pane.
    pub fn with_resize_behavior(self, behavior: Vec<SplitResizeBehavior>, cx: &mut App) -> Self {
        self.state.update(cx, |state, _| {
            state.set_resize_behavior(behavior);
        });
        self
    }

    /// Adds a child pane to the container.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Sets the color of the resize handles.
    pub fn handle_color(mut self, color: crate::Hsla) -> Self {
        self.handle_color = Some(color);
        self
    }

    /// Sets the color of the resize handles when hovered.
    pub fn handle_hover_color(mut self, color: crate::Hsla) -> Self {
        self.handle_hover_color = Some(color);
        self
    }
}

impl RenderOnce for ResizableSplits {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let handle_color = self.handle_color.unwrap_or(crate::black().opacity(0.2));
        let handle_hover_color = self
            .handle_hover_color
            .unwrap_or(crate::black().opacity(0.4));
        let children = self.children;

        let pane_widths = self.state.read(cx).lengths();
        let handles = render_resize_handles(
            &self.state,
            handle_color,
            handle_hover_color,
            |width| div().w(width).h_full().into_any_element(),
            window,
            cx,
        );

        let mut container = div().flex().flex_row().size_full().relative();
        container = with_bounds_tracking(container, &self.state);

        let mut container = container.id(self.id.clone());
        container = with_drag_resize_handlers(container, &self.state);

        for (i, child) in children.into_iter().enumerate() {
            let width = pane_widths.get(i).copied().unwrap_or(Length::Auto);
            container = container.child(div().h_full().w(width).overflow_hidden().child(child));
        }

        container = container.child(handles);

        container
    }
}

/// Creates a new resizable splits container with the given state.
pub fn resizable_splits(
    id: impl Into<ElementId>,
    state: Entity<ResizableSplitState>,
) -> ResizableSplits {
    ResizableSplits::new(id, state)
}
