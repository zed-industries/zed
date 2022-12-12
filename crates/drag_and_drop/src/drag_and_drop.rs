use std::{any::Any, rc::Rc};

use collections::HashSet;
use gpui::{
    elements::{Empty, MouseEventHandler, Overlay},
    geometry::{rect::RectF, vector::Vector2F},
    scene::{MouseDown, MouseDrag},
    CursorStyle, Element, ElementBox, EventContext, MouseButton, MutableAppContext, RenderContext,
    View, WeakViewHandle,
};

const DEAD_ZONE: f32 = 4.;

enum State<V: View> {
    Down {
        region_offset: Vector2F,
        region: RectF,
    },
    DeadZone {
        region_offset: Vector2F,
        region: RectF,
    },
    Dragging {
        window_id: usize,
        position: Vector2F,
        region_offset: Vector2F,
        region: RectF,
        payload: Rc<dyn Any + 'static>,
        render: Rc<dyn Fn(Rc<dyn Any>, &mut RenderContext<V>) -> ElementBox>,
    },
    Canceled,
}

impl<V: View> Clone for State<V> {
    fn clone(&self) -> Self {
        match self {
            &State::Down {
                region_offset,
                region,
            } => State::Down {
                region_offset,
                region,
            },
            &State::DeadZone {
                region_offset,
                region,
            } => State::DeadZone {
                region_offset,
                region,
            },
            State::Dragging {
                window_id,
                position,
                region_offset,
                region,
                payload,
                render,
            } => Self::Dragging {
                window_id: window_id.clone(),
                position: position.clone(),
                region_offset: region_offset.clone(),
                region: region.clone(),
                payload: payload.clone(),
                render: render.clone(),
            },
            State::Canceled => State::Canceled,
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
        self.currently_dragged.as_ref().and_then(|state| {
            if let State::Dragging {
                position,
                payload,
                window_id: window_dragged_from,
                ..
            } = state
            {
                if &window_id != window_dragged_from {
                    return None;
                }

                payload
                    .is::<T>()
                    .then(|| payload.clone().downcast::<T>().ok())
                    .flatten()
                    .map(|payload| (position.clone(), payload))
            } else {
                None
            }
        })
    }

    pub fn drag_started(event: MouseDown, cx: &mut EventContext) {
        cx.update_global(|this: &mut Self, _| {
            this.currently_dragged = Some(State::Down {
                region_offset: event.position - event.region.origin(),
                region: event.region,
            });
        })
    }

    pub fn dragging<T: Any>(
        event: MouseDrag,
        payload: Rc<T>,
        cx: &mut EventContext,
        render: Rc<impl 'static + Fn(&T, &mut RenderContext<V>) -> ElementBox>,
    ) {
        let window_id = cx.window_id();
        cx.update_global(|this: &mut Self, cx| {
            this.notify_containers_for_window(window_id, cx);

            match this.currently_dragged.as_ref() {
                Some(&State::Down {
                    region_offset,
                    region,
                })
                | Some(&State::DeadZone {
                    region_offset,
                    region,
                }) => {
                    if (event.position - (region.origin() + region_offset)).length() > DEAD_ZONE {
                        this.currently_dragged = Some(State::Dragging {
                            window_id,
                            region_offset,
                            region,
                            position: event.position,
                            payload,
                            render: Rc::new(move |payload, cx| {
                                render(payload.downcast_ref::<T>().unwrap(), cx)
                            }),
                        });
                    } else {
                        this.currently_dragged = Some(State::DeadZone {
                            region_offset,
                            region,
                        })
                    }
                }
                Some(&State::Dragging {
                    region_offset,
                    region,
                    ..
                }) => {
                    this.currently_dragged = Some(State::Dragging {
                        window_id,
                        region_offset,
                        region,
                        position: event.position,
                        payload,
                        render: Rc::new(move |payload, cx| {
                            render(payload.downcast_ref::<T>().unwrap(), cx)
                        }),
                    });
                }
                _ => {}
            }
        });
    }

    pub fn render(cx: &mut RenderContext<V>) -> Option<ElementBox> {
        enum DraggedElementHandler {}
        cx.global::<Self>()
            .currently_dragged
            .clone()
            .and_then(|state| {
                match state {
                    State::Down { .. } => None,
                    State::DeadZone { .. } => None,
                    State::Dragging {
                        window_id,
                        region_offset,
                        position,
                        region,
                        payload,
                        render,
                    } => {
                        if cx.window_id() != window_id {
                            return None;
                        }

                        let position = position - region_offset;
                        Some(
                            Overlay::new(
                                MouseEventHandler::<DraggedElementHandler>::new(0, cx, |_, cx| {
                                    render(payload, cx)
                                })
                                .with_cursor_style(CursorStyle::Arrow)
                                .on_up(MouseButton::Left, |_, cx| {
                                    cx.defer(|cx| {
                                        cx.update_global::<Self, _, _>(|this, cx| {
                                            this.finish_dragging(cx)
                                        });
                                    });
                                    cx.propagate_event();
                                })
                                .on_up_out(MouseButton::Left, |_, cx| {
                                    cx.defer(|cx| {
                                        cx.update_global::<Self, _, _>(|this, cx| {
                                            this.finish_dragging(cx)
                                        });
                                    });
                                })
                                // Don't block hover events or invalidations
                                .with_hoverable(false)
                                .constrained()
                                .with_width(region.width())
                                .with_height(region.height())
                                .boxed(),
                            )
                            .with_anchor_position(position)
                            .boxed(),
                        )
                    }

                    State::Canceled => Some(
                        MouseEventHandler::<DraggedElementHandler>::new(0, cx, |_, _| {
                            Empty::new()
                                .constrained()
                                .with_width(0.)
                                .with_height(0.)
                                .boxed()
                        })
                        .on_up(MouseButton::Left, |_, cx| {
                            cx.defer(|cx| {
                                cx.update_global::<Self, _, _>(|this, _| {
                                    this.currently_dragged = None;
                                });
                            });
                        })
                        .on_up_out(MouseButton::Left, |_, cx| {
                            cx.defer(|cx| {
                                cx.update_global::<Self, _, _>(|this, _| {
                                    this.currently_dragged = None;
                                });
                            });
                        })
                        .boxed(),
                    ),
                }
            })
    }

    pub fn cancel_dragging<P: Any>(&mut self, cx: &mut MutableAppContext) {
        if let Some(State::Dragging {
            payload, window_id, ..
        }) = &self.currently_dragged
        {
            if payload.is::<P>() {
                let window_id = *window_id;
                self.currently_dragged = Some(State::Canceled);
                self.notify_containers_for_window(window_id, cx);
            }
        }
    }

    fn finish_dragging(&mut self, cx: &mut MutableAppContext) {
        if let Some(State::Dragging { window_id, .. }) = self.currently_dragged.take() {
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
        self.on_down(MouseButton::Left, move |e, cx| {
            cx.propagate_event();
            DragAndDrop::<V>::drag_started(e, cx);
        })
        .on_drag(MouseButton::Left, move |e, cx| {
            let payload = payload.clone();
            let render = render.clone();
            DragAndDrop::<V>::dragging(e, payload, cx, render)
        })
    }
}
