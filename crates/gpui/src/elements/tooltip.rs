use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

use super::{Element, ElementBox, MouseEventHandler};
use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::json,
    ElementStateHandle, LayoutContext, PaintContext, RenderContext, SizeConstraint, Task, View,
};

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(500);

pub struct Tooltip {
    child: ElementBox,
    tooltip: Option<ElementBox>,
    state: ElementStateHandle<Rc<TooltipState>>,
}

#[derive(Default)]
struct TooltipState {
    visible: Cell<bool>,
    position: Cell<Vector2F>,
    debounce: RefCell<Option<Task<()>>>,
}

impl Tooltip {
    pub fn new<T: View>(
        id: usize,
        child: ElementBox,
        tooltip: ElementBox,
        cx: &mut RenderContext<T>,
    ) -> Self {
        let state_handle = cx.element_state::<TooltipState, Rc<TooltipState>>(id);
        let state = state_handle.read(cx).clone();
        let tooltip = if state.visible.get() {
            Some(tooltip)
        } else {
            None
        };
        let child = MouseEventHandler::new::<Self, _, _>(id, cx, |_, _| child)
            .on_hover(move |position, hover, cx| {
                let window_id = cx.window_id();
                if let Some(view_id) = cx.view_id() {
                    if hover {
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
                    }
                }
            })
            .boxed();
        Self {
            child,
            tooltip,
            state: state_handle,
        }
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
            let origin = self.state.read(cx).position.get();
            let mut bounds = RectF::new(origin, tooltip.size());
            if bounds.lower_right().x() > cx.window_size.x() {
                bounds.set_origin_x(bounds.origin_x() - bounds.width());
            }
            if bounds.lower_right().y() > cx.window_size.y() {
                bounds.set_origin_y(bounds.origin_y() - bounds.height());
            }

            cx.scene.push_stacking_context(None);
            tooltip.paint(bounds.origin(), bounds, cx);
            cx.scene.pop_stacking_context();
        }
    }

    fn dispatch_event(
        &mut self,
        event: &crate::Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut crate::EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
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
