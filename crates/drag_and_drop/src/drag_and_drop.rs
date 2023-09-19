use std::{any::Any, rc::Rc};

use collections::HashSet;
use gpui::{
    elements::{Empty, MouseEventHandler, Overlay},
    geometry::{rect::RectF, vector::Vector2F},
    platform::{CursorStyle, Modifiers, MouseButton},
    scene::{MouseDown, MouseDrag},
    AnyElement, AnyWindowHandle, Element, View, ViewContext, WeakViewHandle, WindowContext,
};

const DEAD_ZONE: f32 = 4.;

enum State<V> {
    Down {
        region_offset: Vector2F,
        region: RectF,
    },
    DeadZone {
        region_offset: Vector2F,
        region: RectF,
    },
    Dragging {
        modifiers: Modifiers,
        window: AnyWindowHandle,
        position: Vector2F,
        region_offset: Vector2F,
        region: RectF,
        payload: Rc<dyn Any + 'static>,
        render: Rc<dyn Fn(&Modifiers, Rc<dyn Any>, &mut ViewContext<V>) -> AnyElement<V>>,
    },
    Canceled,
}

impl<V> Clone for State<V> {
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
                modifiers,
                window,
                position,
                region_offset,
                region,
                payload,
                render,
            } => Self::Dragging {
                window: window.clone(),
                position: position.clone(),
                region_offset: region_offset.clone(),
                region: region.clone(),
                payload: payload.clone(),
                render: render.clone(),
                modifiers: modifiers.clone(),
            },
            State::Canceled => State::Canceled,
        }
    }
}

pub struct DragAndDrop<V> {
    containers: HashSet<WeakViewHandle<V>>,
    currently_dragged: Option<State<V>>,
}

impl<V> Default for DragAndDrop<V> {
    fn default() -> Self {
        Self {
            containers: Default::default(),
            currently_dragged: Default::default(),
        }
    }
}

impl<V: 'static> DragAndDrop<V> {
    pub fn register_container(&mut self, handle: WeakViewHandle<V>) {
        self.containers.insert(handle);
    }

    pub fn currently_dragged<T: Any>(&self, window: AnyWindowHandle) -> Option<(Vector2F, Rc<T>)> {
        self.currently_dragged.as_ref().and_then(|state| {
            if let State::Dragging {
                position,
                payload,
                window: window_dragged_from,
                ..
            } = state
            {
                if &window != window_dragged_from {
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

    pub fn any_currently_dragged(&self, window: AnyWindowHandle) -> bool {
        self.currently_dragged
            .as_ref()
            .map(|state| {
                if let State::Dragging {
                    window: window_dragged_from,
                    ..
                } = state
                {
                    if &window != window_dragged_from {
                        return false;
                    }

                    true
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }

    pub fn drag_started(event: MouseDown, cx: &mut WindowContext) {
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
        cx: &mut WindowContext,
        render: Rc<impl 'static + Fn(&Modifiers, &T, &mut ViewContext<V>) -> AnyElement<V>>,
    ) {
        let window = cx.window();
        cx.update_global(|this: &mut Self, cx| {
            this.notify_containers_for_window(window, cx);

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
                            modifiers: event.modifiers,
                            window,
                            region_offset,
                            region,
                            position: event.position,
                            payload,
                            render: Rc::new(move |modifiers, payload, cx| {
                                render(modifiers, payload.downcast_ref::<T>().unwrap(), cx)
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
                        modifiers: event.modifiers,
                        window,
                        region_offset,
                        region,
                        position: event.position,
                        payload,
                        render: Rc::new(move |modifiers, payload, cx| {
                            render(modifiers, payload.downcast_ref::<T>().unwrap(), cx)
                        }),
                    });
                }
                _ => {}
            }
        });
    }

    pub fn update_modifiers(new_modifiers: Modifiers, cx: &mut ViewContext<V>) -> bool {
        cx.update_global(|this: &mut Self, _| match &mut this.currently_dragged {
            Some(state) => match state {
                State::Dragging { modifiers, .. } => {
                    *modifiers = new_modifiers;
                    true
                }
                _ => false,
            },
            None => false,
        })
    }

    pub fn render(cx: &mut ViewContext<V>) -> Option<AnyElement<V>> {
        enum DraggedElementHandler {}
        cx.global::<Self>()
            .currently_dragged
            .clone()
            .and_then(|state| {
                match state {
                    State::Down { .. } => None,
                    State::DeadZone { .. } => None,
                    State::Dragging {
                        modifiers,
                        window,
                        region_offset,
                        position,
                        region,
                        payload,
                        render,
                    } => {
                        if cx.window() != window {
                            return None;
                        }

                        let position = (position - region_offset).round();
                        Some(
                            Overlay::new(
                                MouseEventHandler::new::<DraggedElementHandler, _>(
                                    0,
                                    cx,
                                    |_, cx| render(&modifiers, payload, cx),
                                )
                                .with_cursor_style(CursorStyle::Arrow)
                                .on_up(MouseButton::Left, |_, _, cx| {
                                    cx.window_context().defer(|cx| {
                                        cx.update_global::<Self, _, _>(|this, cx| {
                                            this.finish_dragging(cx)
                                        });
                                    });
                                    cx.propagate_event();
                                })
                                .on_up_out(MouseButton::Left, |_, _, cx| {
                                    cx.window_context().defer(|cx| {
                                        cx.update_global::<Self, _, _>(|this, cx| {
                                            this.finish_dragging(cx)
                                        });
                                    });
                                })
                                // Don't block hover events or invalidations
                                .with_hoverable(false)
                                .constrained()
                                .with_width(region.width())
                                .with_height(region.height()),
                            )
                            .with_anchor_position(position)
                            .into_any(),
                        )
                    }

                    State::Canceled => Some(
                        MouseEventHandler::new::<DraggedElementHandler, _>(0, cx, |_, _| {
                            Empty::new().constrained().with_width(0.).with_height(0.)
                        })
                        .on_up(MouseButton::Left, |_, _, cx| {
                            cx.window_context().defer(|cx| {
                                cx.update_global::<Self, _, _>(|this, _| {
                                    this.currently_dragged = None;
                                });
                            });
                        })
                        .on_up_out(MouseButton::Left, |_, _, cx| {
                            cx.window_context().defer(|cx| {
                                cx.update_global::<Self, _, _>(|this, _| {
                                    this.currently_dragged = None;
                                });
                            });
                        })
                        .into_any(),
                    ),
                }
            })
    }

    pub fn cancel_dragging<P: Any>(&mut self, cx: &mut WindowContext) {
        if let Some(State::Dragging {
            payload, window, ..
        }) = &self.currently_dragged
        {
            if payload.is::<P>() {
                let window = *window;
                self.currently_dragged = Some(State::Canceled);
                self.notify_containers_for_window(window, cx);
            }
        }
    }

    fn finish_dragging(&mut self, cx: &mut WindowContext) {
        if let Some(State::Dragging { window, .. }) = self.currently_dragged.take() {
            self.notify_containers_for_window(window, cx);
        }
    }

    fn notify_containers_for_window(&mut self, window: AnyWindowHandle, cx: &mut WindowContext) {
        self.containers.retain(|container| {
            if let Some(container) = container.upgrade(cx) {
                if container.window() == window {
                    container.update(cx, |_, cx| cx.notify());
                }
                true
            } else {
                false
            }
        });
    }
}

pub trait Draggable<V> {
    fn as_draggable<D: View, P: Any>(
        self,
        payload: P,
        render: impl 'static + Fn(&Modifiers, &P, &mut ViewContext<D>) -> AnyElement<D>,
    ) -> Self
    where
        Self: Sized;
}

impl<V: 'static> Draggable<V> for MouseEventHandler<V> {
    fn as_draggable<D: View, P: Any>(
        self,
        payload: P,
        render: impl 'static + Fn(&Modifiers, &P, &mut ViewContext<D>) -> AnyElement<D>,
    ) -> Self
    where
        Self: Sized,
    {
        let payload = Rc::new(payload);
        let render = Rc::new(render);
        self.on_down(MouseButton::Left, move |e, _, cx| {
            cx.propagate_event();
            DragAndDrop::<D>::drag_started(e, cx);
        })
        .on_drag(MouseButton::Left, move |e, _, cx| {
            let payload = payload.clone();
            let render = render.clone();
            DragAndDrop::<D>::dragging(e, payload, cx, render)
        })
    }
}
