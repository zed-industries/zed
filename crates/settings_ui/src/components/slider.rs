use std::rc::Rc;

use gpui::{
    AccessibleAction, Bounds, DragMoveEvent, ElementId, Empty, FocusHandle, MouseButton,
    MouseDownEvent, Pixels, Point, Role, actions, canvas,
};
use ui::prelude::*;

actions!(
    slider,
    [
        /// Decreases the value of the focused slider by one step.
        Decrease,
        /// Increases the value of the focused slider by one step.
        Increase,
        /// Sets the focused slider to its minimum value.
        SetMinimum,
        /// Sets the focused slider to its maximum value.
        SetMaximum,
    ]
);

const KEYBOARD_STEP: f32 = 0.05;
const TRACK_WIDTH: Pixels = px(160.);
const THUMB_SIZE: Pixels = px(12.);

#[derive(Clone)]
struct DraggedSlider(ElementId);

struct SliderState {
    focus_handle: FocusHandle,
    bounds: Bounds<Pixels>,
    dragging: Option<f32>,
    committed: Option<CommittedValue>,
}

struct CommittedValue {
    value: f32,
    replaced: f32,
}

#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    value: f32,
    on_change: Rc<dyn Fn(f32, &mut Window, &mut App)>,
    tab_index: isize,
    aria_label: Option<SharedString>,
}

impl Slider {
    pub fn new(
        id: impl Into<ElementId>,
        value: f32,
        on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            value: value.clamp(0., 1.),
            on_change: Rc::new(on_change),
            tab_index: 0,
            aria_label: None,
        }
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = tab_index;
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let tab_index = self.tab_index;
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| SliderState {
            focus_handle: cx.focus_handle().tab_index(tab_index).tab_stop(true),
            bounds: Bounds::default(),
            dragging: None,
            committed: None,
        });

        let value = self.value;
        let shown = state.update(cx, |state, _| {
            if state
                .committed
                .as_ref()
                .is_some_and(|committed| committed.replaced != value)
            {
                state.committed = None;
            }
            state
                .dragging
                .or(state.committed.as_ref().map(|committed| committed.value))
                .unwrap_or(value)
        });
        let percentage = (shown * 100.).round();
        let focus_handle = state.read(cx).focus_handle.clone();
        let is_focused = focus_handle.is_focused(window);
        let colors = cx.theme().colors();

        let commit: Rc<dyn Fn(f32, &mut Window, &mut App)> = Rc::new({
            let state = state.clone();
            let on_change = self.on_change.clone();
            move |new_value, window, cx| {
                let new_value = new_value.clamp(0., 1.);
                state.update(cx, |state, cx| {
                    state.dragging = None;
                    state.committed = Some(CommittedValue {
                        value: new_value,
                        replaced: value,
                    });
                    cx.notify();
                });
                on_change(new_value, window, cx);
            }
        });
        let release = {
            let state = state.clone();
            let commit = commit.clone();
            move |window: &mut Window, cx: &mut App| {
                if let Some(dragged_value) = state.read(cx).dragging {
                    commit(dragged_value, window, cx);
                }
            }
        };
        let release_out = release.clone();
        let commit_to = |new_value: f32| {
            let commit = commit.clone();
            move |window: &mut Window, cx: &mut App| commit(new_value, window, cx)
        };
        let decrease = commit_to(shown - KEYBOARD_STEP);
        let increase = commit_to(shown + KEYBOARD_STEP);
        let set_minimum = commit_to(0.);
        let set_maximum = commit_to(1.);

        h_flex()
            .gap_2()
            .child(
                div()
                    .id(self.id.clone())
                    .key_context("Slider")
                    .role(Role::Slider)
                    .when_some(self.aria_label, |this, label| this.aria_label(label))
                    .aria_numeric_value(f64::from(percentage))
                    .aria_min_numeric_value(0.)
                    .aria_max_numeric_value(100.)
                    .aria_numeric_value_step(f64::from(KEYBOARD_STEP * 100.))
                    .track_focus(&focus_handle)
                    .relative()
                    .flex()
                    .items_center()
                    .w(TRACK_WIDTH)
                    .h(THUMB_SIZE)
                    .cursor_pointer()
                    .child(
                        canvas(
                            {
                                let state = state.clone();
                                move |bounds, _, cx| {
                                    state.update(cx, |state, _| state.bounds = bounds);
                                }
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full(),
                    )
                    .child(
                        div().w_full().h_1().rounded_full().bg(colors.border).child(
                            div()
                                .h_full()
                                .w(relative(shown))
                                .rounded_full()
                                .bg(colors.text_accent),
                        ),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left(relative(shown))
                            .ml(-THUMB_SIZE / 2.)
                            .size(THUMB_SIZE)
                            .rounded_full()
                            .border_2()
                            .border_color(if is_focused {
                                colors.border_focused
                            } else {
                                colors.text_accent
                            })
                            .bg(colors.editor_background),
                    )
                    .on_mouse_down(MouseButton::Left, {
                        let state = state.clone();
                        move |event: &MouseDownEvent, window, cx| {
                            state.update(cx, |state, cx| {
                                state.dragging = Some(value_at(event.position, state.bounds));
                                cx.notify();
                            });
                            window.focus(&focus_handle, cx);
                        }
                    })
                    .on_drag(DraggedSlider(self.id.clone()), |_, _, _, cx| {
                        cx.new(|_| Empty)
                    })
                    .on_drag_move::<DraggedSlider>({
                        let state = state.clone();
                        let id = self.id.clone();
                        move |event: &DragMoveEvent<DraggedSlider>, _, cx| {
                            if event.drag(cx).0 != id {
                                return;
                            }
                            state.update(cx, |state, cx| {
                                state.dragging = Some(value_at(event.event.position, event.bounds));
                                cx.notify();
                            });
                        }
                    })
                    .on_mouse_up(MouseButton::Left, move |_, window, cx| release(window, cx))
                    .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
                        release_out(window, cx)
                    })
                    .on_action({
                        let decrease = decrease.clone();
                        move |_: &Decrease, window, cx| decrease(window, cx)
                    })
                    .on_action({
                        let increase = increase.clone();
                        move |_: &Increase, window, cx| increase(window, cx)
                    })
                    .on_action(move |_: &SetMinimum, window, cx| set_minimum(window, cx))
                    .on_action(move |_: &SetMaximum, window, cx| set_maximum(window, cx))
                    .on_a11y_action(AccessibleAction::Decrement, move |_, window, cx| {
                        decrease(window, cx)
                    })
                    .on_a11y_action(AccessibleAction::Increment, move |_, window, cx| {
                        increase(window, cx)
                    }),
            )
            .child(
                div().w_8().child(
                    Label::new(format!("{percentage}%"))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
    }
}

fn value_at(position: Point<Pixels>, bounds: Bounds<Pixels>) -> f32 {
    ((position.x - bounds.left()) / bounds.size.width.max(px(1.))).clamp(0., 1.)
}
