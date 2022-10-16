use std::{any::Any, rc::Rc};

use collections::HashSet;
use gpui::{
    elements::{MouseEventHandler, Overlay},
    geometry::vector::Vector2F,
    scene::MouseDrag,
    CursorStyle, Element, ElementBox, EventContext, MouseButton, MutableAppContext, RenderContext,
    View, WeakViewHandle,
};

struct State<V: View> {
    window_id: usize,
    position: Vector2F,
    region_offset: Vector2F,
    payload: Rc<dyn Any + 'static>,
    render: Rc<dyn Fn(Rc<dyn Any>, &mut RenderContext<V>) -> ElementBox>,
}

impl<V: View> Clone for State<V> {
    fn clone(&self) -> Self {
        Self {
            window_id: self.window_id.clone(),
            position: self.position.clone(),
            region_offset: self.region_offset.clone(),
            payload: self.payload.clone(),
            render: self.render.clone(),
        }
    }
}

pub struct DragAndDrop<V: View> {
    containers: HashSet<WeakViewHandle<V>>,
    currently_dragged: Option<State<V>>,
}

impl<V: View> Default for DragAndDrop<V> {
    fn default() -> Self {
        Self {
            containers: Default::default(),
            currently_dragged: Default::default(),
        }
    }
}

impl<V: View> DragAndDrop<V> {
    pub fn register_container(&mut self, handle: WeakViewHandle<V>) {
        self.containers.insert(handle);
    }

    pub fn currently_dragged<T: Any>(&self, window_id: usize) -> Option<(Vector2F, Rc<T>)> {
        self.currently_dragged.as_ref().and_then(
            |State {
                 position,
                 payload,
                 window_id: window_dragged_from,
                 ..
             }| {
                if &window_id != window_dragged_from {
                    return None;
                }

                payload
                    .clone()
                    .downcast::<T>()
                    .ok()
                    .map(|payload| (position.clone(), payload))
            },
        )
    }

    pub fn dragging<T: Any>(
        event: MouseDrag,
        payload: Rc<T>,
        cx: &mut EventContext,
        render: Rc<impl 'static + Fn(&T, &mut RenderContext<V>) -> ElementBox>,
    ) {
        let window_id = cx.window_id();
        cx.update_global::<Self, _, _>(|this, cx| {
            let region_offset = if let Some(previous_state) = this.currently_dragged.as_ref() {
                previous_state.region_offset
            } else {
                event.region.origin() - event.prev_mouse_position
            };

            this.currently_dragged = Some(State {
                window_id,
                region_offset,
                position: event.position,
                payload,
                render: Rc::new(move |payload, cx| {
                    render(payload.downcast_ref::<T>().unwrap(), cx)
                }),
            });

            this.notify_containers_for_window(window_id, cx);
        });
    }

    pub fn render(cx: &mut RenderContext<V>) -> Option<ElementBox> {
        let currently_dragged = cx.global::<Self>().currently_dragged.clone();

        currently_dragged.and_then(
            |State {
                 window_id,
                 region_offset,
                 position,
                 payload,
                 render,
             }| {
                if cx.window_id() != window_id {
                    return None;
                }

                let position = position + region_offset;

                enum DraggedElementHandler {}
                Some(
                    Overlay::new(
                        MouseEventHandler::<DraggedElementHandler>::new(0, cx, |_, cx| {
                            render(payload, cx)
                        })
                        .with_cursor_style(CursorStyle::Arrow)
                        .on_up(MouseButton::Left, |_, cx| {
                            cx.defer(|cx| {
                                cx.update_global::<Self, _, _>(|this, cx| this.stop_dragging(cx));
                            });
                            cx.propogate_event();
                        })
                        .on_up_out(MouseButton::Left, |_, cx| {
                            cx.defer(|cx| {
                                cx.update_global::<Self, _, _>(|this, cx| this.stop_dragging(cx));
                            });
                        })
                        // Don't block hover events or invalidations
                        .with_hoverable(false)
                        .boxed(),
                    )
                    .with_anchor_position(position)
                    .boxed(),
                )
            },
        )
    }

    fn stop_dragging(&mut self, cx: &mut MutableAppContext) {
        if let Some(State { window_id, .. }) = self.currently_dragged.take() {
            self.notify_containers_for_window(window_id, cx);
        }
    }

    fn notify_containers_for_window(&mut self, window_id: usize, cx: &mut MutableAppContext) {
        self.containers.retain(|container| {
            if let Some(container) = container.upgrade(cx) {
                if container.window_id() == window_id {
                    container.update(cx, |_, cx| cx.notify());
                }
                true
            } else {
                false
            }
        });
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

impl<Tag> Draggable for MouseEventHandler<Tag> {
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
