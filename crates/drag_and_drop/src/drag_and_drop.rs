use std::{any::Any, rc::Rc};

use gpui::{
    elements::{Container, MouseEventHandler},
    geometry::vector::Vector2F,
    scene::DragRegionEvent,
    CursorStyle, Element, ElementBox, EventContext, MouseButton, RenderContext, View, ViewContext,
    WeakViewHandle,
};

struct State<V: View> {
    position: Vector2F,
    region_offset: Vector2F,
    payload: Rc<dyn Any + 'static>,
    render: Rc<dyn Fn(Rc<dyn Any>, &mut RenderContext<V>) -> ElementBox>,
}

impl<V: View> Clone for State<V> {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            region_offset: self.region_offset.clone(),
            payload: self.payload.clone(),
            render: self.render.clone(),
        }
    }
}

pub struct DragAndDrop<V: View> {
    parent: WeakViewHandle<V>,
    currently_dragged: Option<State<V>>,
}

impl<V: View> DragAndDrop<V> {
    pub fn new(parent: WeakViewHandle<V>, cx: &mut ViewContext<V>) -> Self {
        cx.observe_global::<Self, _>(|cx| {
            if let Some(parent) = cx.global::<Self>().parent.upgrade(cx) {
                parent.update(cx, |_, cx| cx.notify())
            }
        })
        .detach();

        Self {
            parent,
            currently_dragged: None,
        }
    }

    pub fn currently_dragged<T: Any>(&self) -> Option<(Vector2F, Rc<T>)> {
        self.currently_dragged.as_ref().and_then(
            |State {
                 position, payload, ..
             }| {
                payload
                    .clone()
                    .downcast::<T>()
                    .ok()
                    .map(|payload| (position.clone(), payload))
            },
        )
    }

    pub fn dragging<T: Any>(
        event: DragRegionEvent,
        payload: Rc<T>,
        cx: &mut EventContext,
        render: Rc<impl 'static + Fn(&T, &mut RenderContext<V>) -> ElementBox>,
    ) {
        cx.update_global::<Self, _, _>(|this, cx| {
            let region_offset = if let Some(previous_state) = this.currently_dragged.as_ref() {
                previous_state.region_offset
            } else {
                event.region.origin() - event.prev_mouse_position
            };

            this.currently_dragged = Some(State {
                region_offset,
                position: event.position,
                payload,
                render: Rc::new(move |payload, cx| {
                    render(payload.downcast_ref::<T>().unwrap(), cx)
                }),
            });

            if let Some(parent) = this.parent.upgrade(cx) {
                parent.update(cx, |_, cx| cx.notify())
            }
        });
    }

    pub fn render(cx: &mut RenderContext<V>) -> Option<ElementBox> {
        let currently_dragged = cx.global::<Self>().currently_dragged.clone();

        currently_dragged.map(
            |State {
                 region_offset,
                 position,
                 payload,
                 render,
             }| {
                let position = position + region_offset;

                MouseEventHandler::new::<Self, _, _>(0, cx, |_, cx| {
                    Container::new(render(payload, cx))
                        .with_margin_left(position.x())
                        .with_margin_top(position.y())
                        .aligned()
                        .top()
                        .left()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::Arrow)
                .on_up(MouseButton::Left, |_, cx| {
                    cx.defer(|cx| {
                        cx.update_global::<Self, _, _>(|this, _| this.currently_dragged.take());
                    });
                    cx.propogate_event();
                })
                // Don't block hover events or invalidations
                .with_hoverable(false)
                .boxed()
            },
        )
    }
}

pub trait Draggable {
    fn as_draggable<V: View, P: Any>(
        self,
        payload: P,
        render: impl 'static + Fn(&P, &mut RenderContext<V>) -> ElementBox,
    ) -> Self
    where
        Self: Sized;
}

impl Draggable for MouseEventHandler {
    fn as_draggable<V: View, P: Any>(
        self,
        payload: P,
        render: impl 'static + Fn(&P, &mut RenderContext<V>) -> ElementBox,
    ) -> Self
    where
        Self: Sized,
    {
        let payload = Rc::new(payload);
        let render = Rc::new(render);
        self.on_drag(MouseButton::Left, move |e, cx| {
            let payload = payload.clone();
            let render = render.clone();
            DragAndDrop::<V>::dragging(e, payload, cx, render)
        })
    }
}
