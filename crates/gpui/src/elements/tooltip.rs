use super::{
    ContainerStyle, Element, ElementBox, Flex, KeystrokeLabel, MouseEventHandler, Overlay,
    OverlayFitMode, ParentElement, Text,
};
use crate::{
    fonts::TextStyle,
    geometry::{rect::RectF, vector::Vector2F},
    json::json,
    presenter::MeasurementContext,
    Action, Axis, ElementStateHandle, LayoutContext, PaintContext, RenderContext, SizeConstraint,
    Task, View,
};
use serde::Deserialize;
use std::{
    cell::{Cell, RefCell},
    ops::Range,
    rc::Rc,
    time::Duration,
};

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(500);

pub struct Tooltip {
    child: ElementBox,
    tooltip: Option<ElementBox>,
    _state: ElementStateHandle<Rc<TooltipState>>,
}

#[derive(Default)]
struct TooltipState {
    visible: Cell<bool>,
    position: Cell<Vector2F>,
    debounce: RefCell<Option<Task<()>>>,
}

#[derive(Clone, Deserialize, Default)]
pub struct TooltipStyle {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    keystroke: KeystrokeStyle,
    pub max_text_width: f32,
}

#[derive(Clone, Deserialize, Default)]
pub struct KeystrokeStyle {
    #[serde(flatten)]
    container: ContainerStyle,
    #[serde(flatten)]
    text: TextStyle,
}

impl Tooltip {
    pub fn new<Tag: 'static, T: View>(
        id: usize,
        text: String,
        action: Option<Box<dyn Action>>,
        style: TooltipStyle,
        child: ElementBox,
        cx: &mut RenderContext<T>,
    ) -> Self {
        struct ElementState<Tag>(Tag);
        struct MouseEventHandlerState<Tag>(Tag);
        let focused_view_id = cx.focused_view_id(cx.window_id).unwrap();

        let state_handle = cx.default_element_state::<ElementState<Tag>, Rc<TooltipState>>(id);
        let state = state_handle.read(cx).clone();
        let tooltip = if state.visible.get() {
            let mut collapsed_tooltip = Self::render_tooltip(
                cx.window_id,
                focused_view_id,
                text.clone(),
                style.clone(),
                action.as_ref().map(|a| a.boxed_clone()),
                true,
            )
            .boxed();
            Some(
                Overlay::new(
                    Self::render_tooltip(cx.window_id, focused_view_id, text, style, action, false)
                        .constrained()
                        .dynamically(move |constraint, cx| {
                            SizeConstraint::strict_along(
                                Axis::Vertical,
                                collapsed_tooltip.layout(constraint, cx).y(),
                            )
                        })
                        .boxed(),
                )
                .with_fit_mode(OverlayFitMode::SwitchAnchor)
                .with_anchor_position(state.position.get())
                .boxed(),
            )
        } else {
            None
        };
        let child = MouseEventHandler::<MouseEventHandlerState<Tag>>::new(id, cx, |_, _| child)
            .on_hover(move |e, cx| {
                let position = e.position;
                let window_id = cx.window_id();
                if let Some(view_id) = cx.view_id() {
                    if e.started {
                        if !state.visible.get() {
                            state.position.set(position);

                            let mut debounce = state.debounce.borrow_mut();
                            if debounce.is_none() {
                                *debounce = Some(cx.spawn({
                                    let state = state.clone();
                                    |mut cx| async move {
                                        cx.background().timer(DEBOUNCE_TIMEOUT).await;
                                        state.visible.set(true);
                                        cx.update(|cx| cx.notify_view(window_id, view_id));
                                    }
                                }));
                            }
                        }
                    } else {
                        state.visible.set(false);
                        state.debounce.take();
                        cx.notify();
                    }
                }
            })
            .boxed();
        Self {
            child,
            tooltip,
            _state: state_handle,
        }
    }

    pub fn render_tooltip(
        window_id: usize,
        focused_view_id: usize,
        text: String,
        style: TooltipStyle,
        action: Option<Box<dyn Action>>,
        measure: bool,
    ) -> impl Element {
        Flex::row()
            .with_child({
                let text = Text::new(text, style.text)
                    .constrained()
                    .with_max_width(style.max_text_width);
                if measure {
                    text.flex(1., false).boxed()
                } else {
                    text.flex(1., false).aligned().boxed()
                }
            })
            .with_children(action.map(|action| {
                let keystroke_label = KeystrokeLabel::new(
                    window_id,
                    focused_view_id,
                    action,
                    style.keystroke.container,
                    style.keystroke.text,
                );
                if measure {
                    keystroke_label.boxed()
                } else {
                    keystroke_label.aligned().boxed()
                }
            }))
            .contained()
            .with_style(style.container)
    }
}

impl Element for Tooltip {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, cx);
        if let Some(tooltip) = self.tooltip.as_mut() {
            tooltip.layout(SizeConstraint::new(Vector2F::zero(), cx.window_size), cx);
        }
        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) {
        self.child.paint(bounds.origin(), visible_bounds, cx);
        if let Some(tooltip) = self.tooltip.as_mut() {
            tooltip.paint(bounds.origin(), visible_bounds, cx);
        }
    }

    fn rect_for_text_range(
        &self,
        range: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &crate::DebugContext,
    ) -> serde_json::Value {
        json!({
            "child": self.child.debug(cx),
            "tooltip": self.tooltip.as_ref().map(|t| t.debug(cx)),
        })
    }
}
