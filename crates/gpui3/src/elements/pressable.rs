use crate::{
    AnyElement, Bounds, DispatchPhase, Element, Interactive, MouseDownEvent, MouseEventListeners,
    MouseUpEvent, ParentElement, Pixels, StatefulElement, Styled, ViewContext,
};
use refineable::{CascadeSlot, Refineable, RefinementCascade};
use smallvec::SmallVec;
use std::sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Arc,
};

pub struct Pressable<E: Styled> {
    cascade_slot: CascadeSlot,
    pressed_style: <E::Style as Refineable>::Refinement,
    child: E,
}

pub struct PressableState<S> {
    pressed: Arc<AtomicBool>,
    child_state: S,
}

impl<E: Styled> Pressable<E> {
    pub fn new(mut child: E) -> Self {
        Self {
            cascade_slot: child.style_cascade().reserve(),
            pressed_style: Default::default(),
            child,
        }
    }
}

impl<E> Styled for Pressable<E>
where
    E: Styled,
{
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<E::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        &mut self.pressed_style
    }
}

impl<S: 'static + Send + Sync, E: Interactive<S> + Styled> Interactive<S> for Pressable<E> {
    fn listeners(&mut self) -> &mut MouseEventListeners<S> {
        self.child.listeners()
    }
}

impl<E> Element for Pressable<E>
where
    E: Styled + StatefulElement,
    <E as Styled>::Style: 'static + Refineable + Send + Sync + Default,
    <<E as Styled>::Style as Refineable>::Refinement: 'static + Refineable + Send + Sync + Default,
{
    type ViewState = E::ViewState;
    type ElementState = PressableState<E::ElementState>;

    fn layout(
        &mut self,
        state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> (crate::LayoutId, Self::ElementState) {
        if let Some(element_state) = element_state {
            let (id, child_state) = self
                .child
                .layout(state, Some(element_state.child_state), cx);
            let element_state = PressableState {
                pressed: element_state.pressed,
                child_state,
            };
            (id, element_state)
        } else {
            let (id, child_state) = self.child.layout(state, None, cx);
            let element_state = PressableState {
                pressed: Default::default(),
                child_state,
            };
            (id, element_state)
        }
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        let style = element_state
            .pressed
            .load(SeqCst)
            .then_some(self.pressed_style.clone());
        let slot = self.cascade_slot;
        self.style_cascade().set(slot, style);

        let pressed = element_state.pressed.clone();
        cx.on_mouse_event(move |_, event: &MouseDownEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if bounds.contains_point(event.position) {
                    dbg!("pressed");
                    pressed.store(true, SeqCst);
                    cx.notify();
                }
            }
        });
        let pressed = element_state.pressed.clone();
        cx.on_mouse_event(move |_, _: &MouseUpEvent, phase, cx| {
            if phase == DispatchPhase::Capture {
                if pressed.load(SeqCst) {
                    dbg!("released");
                    pressed.store(false, SeqCst);
                    cx.notify();
                }
            }
        });

        self.child
            .paint(bounds, state, &mut element_state.child_state, cx);
    }
}

impl<E: ParentElement + Styled> ParentElement for Pressable<E> {
    type State = E::State;

    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::State>; 2]> {
        self.child.children_mut()
    }
}

// use crate::{
//     element::{AnyElement, Element, IntoElement, Layout, ParentElement},
//     interactive::{InteractionHandlers, Interactive},
//     style::{Style, StyleHelpers, Styleable},
//     ViewContext,
// };
// use anyhow::Result;
// use gpui::{geometry::vector::Vector2F, platform::MouseButtonEvent, LayoutId};
// use refineable::{CascadeSlot, Refineable, RefinementCascade};
// use smallvec::SmallVec;
// use std::{cell::Cell, rc::Rc};

// pub struct Pressable<E: Styleable> {
//     pressed: Rc<Cell<bool>>,
//     pressed_style: <E::Style as Refineable>::Refinement,
//     cascade_slot: CascadeSlot,
//     child: E,
// }

// pub fn pressable<E: Styleable>(mut child: E) -> Pressable<E> {
//     Pressable {
//         pressed: Rc::new(Cell::new(false)),
//         pressed_style: Default::default(),
//         cascade_slot: child.style_cascade().reserve(),
//         child,
//     }
// }

// impl<E: Styleable> Styleable for Pressable<E> {
//     type Style = E::Style;

//     fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
//         &mut self.pressed_style
//     }

//     fn style_cascade(&mut self) -> &mut RefinementCascade<E::Style> {
//         self.child.style_cascade()
//     }
// }

// impl<V: 'static, E: Element<V> + Styleable> Element<V> for Pressable<E> {
//     type PaintState = E::PaintState;

//     fn layout(
//         &mut self,
//         view: &mut V,
//         cx: &mut ViewContext<V>,
//     ) -> Result<(LayoutId, Self::PaintState)>
//     where
//         Self: Sized,
//     {
//         self.child.layout(view, cx)
//     }

//     fn paint(
//         &mut self,
//         view: &mut V,
//         parent_origin: Vector2F,
//         layout: &Layout,
//         paint_state: &mut Self::PaintState,
//         cx: &mut ViewContext<V>,
//     ) where
//         Self: Sized,
//     {
//         let slot = self.cascade_slot;
//         let style = self.pressed.get().then_some(self.pressed_style.clone());
//         self.style_cascade().set(slot, style);

//         let pressed = self.pressed.clone();
//         let bounds = layout.bounds + parent_origin;
//         cx.on_event(layout.order, move |_view, event: &MouseButtonEvent, cx| {
//             if event.is_down {
//                 if bounds.contains_point(event.position) {
//                     pressed.set(true);
//                     cx.repaint();
//                 }
//             } else if pressed.get() {
//                 pressed.set(false);
//                 cx.repaint();
//             }
//         });

//         self.child
//             .paint(view, parent_origin, layout, paint_state, cx);
//     }
// }

// impl<V: 'static, E: Interactive<V> + Styleable> Interactive<V> for Pressable<E> {
//     fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V> {
//         self.child.interaction_handlers()
//     }
// }

// impl<V: 'static, E: ParentElement<V> + Styleable> ParentElement<V> for Pressable<E> {
//     fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
//         self.child.children_mut()
//     }
// }

// impl<V: 'static, E: Element<V> + Styleable> IntoElement<V> for Pressable<E> {
//     type Element = Self;

//     fn into_element(self) -> Self::Element {
//         self
//     }
// }
