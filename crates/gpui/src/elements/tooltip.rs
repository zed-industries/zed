use super::{
    AnyElement, ContainerStyle, Element, Flex, KeystrokeLabel, MouseEventHandler, Overlay,
    OverlayFitMode, ParentElement, Text,
};
use crate::{
    fonts::TextStyle,
    geometry::{rect::RectF, vector::Vector2F},
    json::json,
    Action, Axis, ElementStateHandle, LayoutContext, PaintContext, SceneBuilder, SizeConstraint,
    Task, View, ViewContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    ops::Range,
    rc::Rc,
    time::Duration,
};
use util::ResultExt;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(500);

pub struct Tooltip<V: View> {
    child: AnyElement<V>,
    tooltip: Option<AnyElement<V>>,
    _state: ElementStateHandle<Rc<TooltipState>>,
}

#[derive(Default)]
struct TooltipState {
    visible: Cell<bool>,
    position: Cell<Vector2F>,
    debounce: RefCell<Option<Task<()>>>,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct TooltipStyle {
    #[serde(flatten)]
    pub container: ContainerStyle,
    pub text: TextStyle,
    keystroke: KeystrokeStyle,
    pub max_text_width: Option<f32>,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct KeystrokeStyle {
    #[serde(flatten)]
    container: ContainerStyle,
    #[serde(flatten)]
    text: TextStyle,
}

impl<V: View> Tooltip<V> {
    pub fn new<Tag: 'static>(
        id: usize,
        text: impl Into<Cow<'static, str>>,
        action: Option<Box<dyn Action>>,
        style: TooltipStyle,
        child: AnyElement<V>,
        cx: &mut ViewContext<V>,
    ) -> Self {
        struct ElementState<Tag>(Tag);
        struct MouseEventHandlerState<Tag>(Tag);
        let focused_view_id = cx.focused_view_id();

        let state_handle = cx.default_element_state::<ElementState<Tag>, Rc<TooltipState>>(id);
        let state = state_handle.read(cx).clone();
        let text = text.into();

        let tooltip = if state.visible.get() {
            let mut collapsed_tooltip = Self::render_tooltip(
                focused_view_id,
                text.clone(),
                style.clone(),
                action.as_ref().map(|a| a.boxed_clone()),
                true,
            );
            Some(
                Overlay::new(
                    Self::render_tooltip(focused_view_id, text, style, action, false)
                        .constrained()
                        .dynamically(move |constraint, view, cx| {
                            SizeConstraint::strict_along(
                                Axis::Vertical,
                                collapsed_tooltip.layout(constraint, view, cx).0.y(),
                            )
                        }),
                )
                .with_fit_mode(OverlayFitMode::SwitchAnchor)
                .with_anchor_position(state.position.get())
                .into_any(),
            )
        } else {
            None
        };
        let child = MouseEventHandler::<MouseEventHandlerState<Tag>, _>::new(id, cx, |_, _| child)
            .on_hover(move |e, _, cx| {
                let position = e.position;
                if e.started {
                    if !state.visible.get() {
                        state.position.set(position);

                        let mut debounce = state.debounce.borrow_mut();
                        if debounce.is_none() {
                            *debounce = Some(cx.spawn({
                                let state = state.clone();
                                |view, mut cx| async move {
                                    cx.background().timer(DEBOUNCE_TIMEOUT).await;
                                    state.visible.set(true);
                                    view.update(&mut cx, |_, cx| cx.notify()).log_err();
                                }
                            }));
                        }
                    }
                } else {
                    state.visible.set(false);
                    state.debounce.take();
                    cx.notify();
                }
            })
            .into_any();
        Self {
            child,
            tooltip,
            _state: state_handle,
        }
    }

    pub fn render_tooltip(
        focused_view_id: Option<usize>,
        text: impl Into<Cow<'static, str>>,
        style: TooltipStyle,
        action: Option<Box<dyn Action>>,
        measure: bool,
    ) -> impl Element<V> {
        Flex::row()
            .with_child({
                let text = if let Some(max_text_width) = style.max_text_width {
                    Text::new(text, style.text)
                        .constrained()
                        .with_max_width(max_text_width)
                } else {
                    Text::new(text, style.text).constrained()
                };

                if measure {
                    text.flex(1., false).into_any()
                } else {
                    text.flex(1., false).aligned().into_any()
                }
            })
            .with_children(action.and_then(|action| {
                let keystroke_label = KeystrokeLabel::new(
                    focused_view_id?,
                    action,
                    style.keystroke.container,
                    style.keystroke.text,
                );
                if measure {
                    Some(keystroke_label.into_any())
                } else {
                    Some(keystroke_label.aligned().into_any())
                }
            }))
            .contained()
            .with_style(style.container)
    }
}

impl<V: View> Element<V> for Tooltip<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, view, cx);
        if let Some(tooltip) = self.tooltip.as_mut() {
            tooltip.layout(
                SizeConstraint::new(Vector2F::zero(), cx.window_size()),
                view,
                cx,
            );
        }
        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) {
        self.child
            .paint(scene, bounds.origin(), visible_bounds, view, cx);
        if let Some(tooltip) = self.tooltip.as_mut() {
            tooltip.paint(scene, bounds.origin(), visible_bounds, view, cx);
        }
    }

    fn rect_for_text_range(
        &self,
        range: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range, view, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "child": self.child.debug(view, cx),
            "tooltip": self.tooltip.as_ref().map(|t| t.debug(view, cx)),
        })
    }
}
