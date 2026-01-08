use std::rc::Rc;

use crate::util::FluentBuilder;
use crate::{
    AbsoluteLength, AnyElement, App, AppContext, Context, DefiniteLength, DragMoveEvent, ElementId,
    Entity, InteractiveElement, IntoElement, Length, ParentElement, Pixels, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div, px,
};

const RESIZE_HANDLE_WIDTH: f32 = 8.0;

/// Marker type for drag events on resize handles.
#[derive(Debug, Clone)]
struct DraggedSplitHandle(usize);

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
    widths: Vec<DefiniteLength>,
    visible_widths: Vec<DefiniteLength>,
    cached_bounds_width: Pixels,
    initialized: bool,
}

impl ResizableSplitState {
    /// Creates a new state with the given number of panes.
    pub fn new(pane_count: usize, _cx: &mut App) -> Self {
        Self {
            widths: vec![DefiniteLength::default(); pane_count],
            visible_widths: vec![DefiniteLength::default(); pane_count],
            cached_bounds_width: Default::default(),
            initialized: false,
        }
    }

    /// Returns the number of panes.
    pub fn pane_count(&self) -> usize {
        self.widths.len()
    }

    /// Initialize widths from the given initial sizes if not already initialized.
    pub fn initialize_if_needed(&mut self, initial_sizes: &[DefiniteLength]) {
        if !self.initialized {
            self.initialized = true;
            self.widths = initial_sizes.to_vec();
            self.visible_widths = self.widths.clone();
        }
    }

    fn get_fraction(length: &DefiniteLength, bounds_width: Pixels, rem_size: Pixels) -> f32 {
        match length {
            DefiniteLength::Absolute(AbsoluteLength::Pixels(pixels)) => *pixels / bounds_width,
            DefiniteLength::Absolute(AbsoluteLength::Rems(rems_width)) => {
                rems_width.to_pixels(rem_size) / bounds_width
            }
            DefiniteLength::Fraction(fraction) => *fraction,
        }
    }

    fn on_double_click(
        &mut self,
        double_click_position: usize,
        initial_sizes: &[DefiniteLength],
        resize_behavior: &[SplitResizeBehavior],
        window: &mut Window,
    ) {
        let bounds_width = self.cached_bounds_width;
        let rem_size = window.rem_size();
        let initial_sizes: Vec<f32> = initial_sizes
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
            resize_behavior,
        );
        self.widths = widths
            .iter()
            .map(|&f| DefiniteLength::Fraction(f))
            .collect();
        self.visible_widths = self.widths.clone();
    }

    fn reset_to_initial_size(
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

    fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedSplitHandle>,
        resize_behavior: &[SplitResizeBehavior],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;

        let mut pane_position = 0.0;
        let rem_size = window.rem_size();
        let bounds_width = bounds.right() - bounds.left();
        let pane_idx = drag_event.drag(cx).0;

        // Update cached bounds width from the drag event bounds
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
        let pane_count = resize_behavior.len();
        total_length_ratio += (pane_count - 1 - pane_idx) as f32 * handle_width;

        let drag_fraction = (drag_position.x - bounds.left()) / bounds_width;
        let drag_fraction = drag_fraction * total_length_ratio;
        let diff = drag_fraction - pane_position - handle_width / 2.0;

        Self::drag_handle(diff, pane_idx, &mut widths, resize_behavior);

        self.visible_widths = widths
            .iter()
            .map(|&f| DefiniteLength::Fraction(f))
            .collect();
    }

    fn drag_handle(
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

    fn propagate_resize_diff(
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

/// A container element that arranges children horizontally with resizable dividers between them.
#[derive(IntoElement)]
pub struct ResizableSplits {
    id: ElementId,
    children: Vec<AnyElement>,
    initial_sizes: Vec<DefiniteLength>,
    resize_behavior: Vec<SplitResizeBehavior>,
    state: Option<Entity<ResizableSplitState>>,
    handle_color: Option<crate::Hsla>,
    handle_hover_color: Option<crate::Hsla>,
}

impl ResizableSplits {
    /// Creates a new resizable splits container with given initial sizes.
    /// All panes are non-resizable by default until `resizable()` or `with_resize_behavior()` is called.
    pub fn new(id: impl Into<ElementId>, sizes: Vec<impl Into<DefiniteLength>>) -> Self {
        let initial_sizes: Vec<DefiniteLength> = sizes.into_iter().map(Into::into).collect();
        let pane_count = initial_sizes.len();
        Self {
            id: id.into(),
            children: Vec::new(),
            initial_sizes,
            resize_behavior: vec![SplitResizeBehavior::None; pane_count],
            state: None,
            handle_color: None,
            handle_hover_color: None,
        }
    }

    /// Makes all panes resizable with default minimum sizes.
    pub fn resizable(mut self, state: &Entity<ResizableSplitState>, cx: &mut App) -> Self {
        self.resize_behavior = vec![SplitResizeBehavior::Resizable; self.initial_sizes.len()];
        self.state = Some(state.clone());
        let initial_sizes = self.initial_sizes.clone();
        state.update(cx, |state, _| {
            state.initialize_if_needed(&initial_sizes);
        });
        self
    }

    /// Sets custom resize behavior for each pane.
    pub fn with_resize_behavior(
        mut self,
        behavior: Vec<SplitResizeBehavior>,
        state: &Entity<ResizableSplitState>,
        cx: &mut App,
    ) -> Self {
        assert_eq!(
            behavior.len(),
            self.initial_sizes.len(),
            "Resize behavior count must match pane count"
        );
        self.resize_behavior = behavior;
        self.state = Some(state.clone());
        let initial_sizes = self.initial_sizes.clone();
        state.update(cx, |state, _| {
            state.initialize_if_needed(&initial_sizes);
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

    fn lengths(&self, cx: &App) -> Vec<Length> {
        self.state
            .as_ref()
            .map(|entity| {
                entity
                    .read(cx)
                    .visible_widths
                    .iter()
                    .map(|w| Length::Definite(*w))
                    .collect()
            })
            .unwrap_or_else(|| {
                self.initial_sizes
                    .iter()
                    .map(|w| Length::Definite(*w))
                    .collect()
            })
    }
}

fn render_resize_handles(
    pane_widths: &[Length],
    resize_behavior: &Rc<Vec<SplitResizeBehavior>>,
    initial_sizes: &Rc<Vec<DefiniteLength>>,
    state: Option<Entity<ResizableSplitState>>,
    handle_color: Option<crate::Hsla>,
    handle_hover_color: Option<crate::Hsla>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let handle_color = handle_color.unwrap_or(crate::black().opacity(0.2));
    let handle_hover_color = handle_hover_color.unwrap_or(crate::black().opacity(0.4));

    let spacers: Vec<AnyElement> = pane_widths
        .iter()
        .map(|width| div().w(*width).h_full().into_any_element())
        .collect();

    let mut pane_idx = 0;
    let mut resize_behavior_iter = resize_behavior.iter();

    let mut elements: Vec<AnyElement> = Vec::new();

    for (i, spacer) in spacers.into_iter().enumerate() {
        elements.push(spacer);

        if i < pane_widths.len() - 1 {
            let resize_behavior = Rc::clone(resize_behavior);
            let initial_sizes = Rc::clone(initial_sizes);
            let current_idx = pane_idx;

            let handle = window.with_id(current_idx, |window| {
                let mut resize_divider = div()
                    .id(current_idx)
                    .relative()
                    .top_0()
                    .w_px()
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
                        .when_some(state.clone(), |this, state| {
                            let initial_sizes = Rc::clone(&initial_sizes);
                            let resize_behavior = Rc::clone(&resize_behavior);
                            this.on_click(move |event, window, cx| {
                                if event.click_count() >= 2 {
                                    state.update(cx, |state, _| {
                                        state.on_double_click(
                                            current_idx,
                                            &initial_sizes,
                                            &resize_behavior,
                                            window,
                                        );
                                    })
                                }
                                cx.stop_propagation();
                            })
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

impl RenderOnce for ResizableSplits {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pane_widths = self.lengths(cx);
        let resize_behavior = Rc::new(self.resize_behavior.clone());
        let initial_sizes = Rc::new(self.initial_sizes.clone());
        let handle_color = self.handle_color;
        let handle_hover_color = self.handle_hover_color;
        let children = self.children;

        let mut container = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .size_full()
            .relative();

        if let Some(state) = &self.state {
            let resize_behavior_clone = Rc::clone(&resize_behavior);
            let state_clone = state.downgrade();
            let state_for_drop = state.downgrade();

            container = container
                .on_drag_move::<DraggedSplitHandle>({
                    let resize_behavior = Rc::clone(&resize_behavior_clone);
                    move |event, window, cx| {
                        state_clone
                            .update(cx, |state, cx| {
                                state.on_drag_move(event, &resize_behavior, window, cx);
                            })
                            .ok();
                    }
                })
                .on_drop::<DraggedSplitHandle>({
                    move |_, _, cx| {
                        state_for_drop
                            .update(cx, |state, _| {
                                state.widths = state.visible_widths.clone();
                            })
                            .ok();
                    }
                });
        }

        for (i, child) in children.into_iter().enumerate() {
            let width = pane_widths.get(i).copied().unwrap_or(Length::Auto);
            container = container.child(div().h_full().w(width).overflow_hidden().child(child));
        }

        let handles = render_resize_handles(
            &pane_widths,
            &resize_behavior,
            &initial_sizes,
            self.state,
            handle_color,
            handle_hover_color,
            window,
            cx,
        );
        container = container.child(handles);

        container
    }
}

/// Creates a new resizable splits container with given initial sizes.
pub fn resizable_splits(
    id: impl Into<ElementId>,
    sizes: Vec<impl Into<DefiniteLength>>,
) -> ResizableSplits {
    ResizableSplits::new(id, sizes)
}
