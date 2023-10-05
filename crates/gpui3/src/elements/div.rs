use crate::{
    AnyElement, Bounds, Element, Layout, LayoutId, Overflow, ParentElement, Pixels, Point,
    Refineable, RefinementCascade, Result, Style, StyleHelpers, Styled, ViewContext,
};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::sync::Arc;
use util::ResultExt;

pub struct Div<S: 'static> {
    styles: RefinementCascade<Style>,
    // handlers: InteractionHandlers<V>,
    children: SmallVec<[AnyElement<S>; 2]>,
    scroll_state: Option<ScrollState>,
}

pub fn div<S>() -> Div<S> {
    Div {
        styles: Default::default(),
        // handlers: Default::default(),
        children: Default::default(),
        scroll_state: None,
    }
}

impl<S: 'static + Send + Sync> Element for Div<S> {
    type State = S;
    type FrameState = Vec<LayoutId>;

    fn layout(
        &mut self,
        view: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        let style = self.computed_style();
        let child_layout_ids = style.apply_text_style(cx, |cx| self.layout_children(view, cx))?;
        let layout_id = cx.request_layout(style.into(), child_layout_ids.clone())?;
        Ok((layout_id, child_layout_ids))
    }

    fn paint(
        &mut self,
        layout: Layout,
        state: &mut S,
        child_layouts: &mut Self::FrameState,
        cx: &mut ViewContext<S>,
    ) -> Result<()> {
        let Layout { order, bounds } = layout;

        let style = self.computed_style();
        cx.stack(0, |cx| style.paint(order, bounds, cx));

        let overflow = &style.overflow;
        style.apply_text_style(cx, |cx| {
            cx.stack(1, |cx| {
                style.apply_overflow(layout.bounds, cx, |cx| {
                    self.paint_children(overflow, state, cx)
                })
            })
        })?;
        self.handle_scroll(order, bounds, style.overflow.clone(), child_layouts, cx);

        // todo!("enable inspector")
        // if cx.is_inspector_enabled() {
        //     self.paint_inspector(parent_origin, layout, cx);
        // }
        //
        Ok(())
    }
}

impl<S: 'static> Div<S> {
    pub fn overflow_hidden(mut self) -> Self {
        self.declared_style().overflow.x = Some(Overflow::Hidden);
        self.declared_style().overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_x(mut self) -> Self {
        self.declared_style().overflow.x = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_hidden_y(mut self) -> Self {
        self.declared_style().overflow.y = Some(Overflow::Hidden);
        self
    }

    pub fn overflow_scroll(mut self, scroll_state: ScrollState) -> Self {
        self.scroll_state = Some(scroll_state);
        self.declared_style().overflow.x = Some(Overflow::Scroll);
        self.declared_style().overflow.y = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_x_scroll(mut self, scroll_state: ScrollState) -> Self {
        self.scroll_state = Some(scroll_state);
        self.declared_style().overflow.x = Some(Overflow::Scroll);
        self
    }

    pub fn overflow_y_scroll(mut self, scroll_state: ScrollState) -> Self {
        self.scroll_state = Some(scroll_state);
        self.declared_style().overflow.y = Some(Overflow::Scroll);
        self
    }

    fn scroll_offset(&self, overflow: &Point<Overflow>) -> Point<Pixels> {
        let mut offset = Point::default();
        if overflow.y == Overflow::Scroll {
            offset.y = self.scroll_state.as_ref().unwrap().y();
        }
        if overflow.x == Overflow::Scroll {
            offset.x = self.scroll_state.as_ref().unwrap().x();
        }

        offset
    }

    fn layout_children(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> Result<Vec<LayoutId>> {
        self.children
            .iter_mut()
            .map(|child| child.layout(view, cx))
            .collect::<Result<Vec<LayoutId>>>()
    }

    fn paint_children(
        &mut self,
        overflow: &Point<Overflow>,
        state: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<()> {
        let scroll_offset = self.scroll_offset(overflow);
        for child in &mut self.children {
            child.paint(state, Some(scroll_offset), cx)?;
        }
        Ok(())
    }

    fn handle_scroll(
        &mut self,
        _order: u32,
        bounds: Bounds<Pixels>,
        overflow: Point<Overflow>,
        child_layout_ids: &[LayoutId],
        cx: &mut ViewContext<S>,
    ) {
        if overflow.y == Overflow::Scroll || overflow.x == Overflow::Scroll {
            let mut scroll_max = Point::default();
            for child_layout_id in child_layout_ids {
                if let Some(child_layout) = cx.layout(*child_layout_id).log_err() {
                    scroll_max = scroll_max.max(&child_layout.bounds.lower_right());
                }
            }
            scroll_max -= bounds.size;

            // todo!("handle scroll")
            // let scroll_state = self.scroll_state.as_ref().unwrap().clone();
            // cx.on_event(order, move |_, event: &ScrollWheelEvent, cx| {
            //     if bounds.contains_point(event.position) {
            //         let scroll_delta = match event.delta {
            //             ScrollDelta::Pixels(delta) => delta,
            //             ScrollDelta::Lines(delta) => cx.text_style().font_size * delta,
            //         };
            //         if overflow.x == Overflow::Scroll {
            //             scroll_state.set_x(
            //                 (scroll_state.x() - scroll_delta.x())
            //                     .max(px(0.))
            //                     .min(scroll_max.x),
            //             );
            //         }
            //         if overflow.y == Overflow::Scroll {
            //             scroll_state.set_y(
            //                 (scroll_state.y() - scroll_delta.y())
            //                     .max(px(0.))
            //                     .min(scroll_max.y),
            //             );
            //         }
            //         cx.repaint();
            //     } else {
            //         cx.bubble_event();
            //     }
            // })
        }
    }

    // fn paint_inspector(
    //     &self,
    //     parent_origin: Point<Pixels>,
    //     layout: &Layout,
    //     cx: &mut ViewContext<V>,
    // ) {
    //     let style = self.styles.merged();
    //     let bounds = layout.bounds;

    //     let hovered = bounds.contains_point(cx.mouse_position());
    //     if hovered {
    //         let rem_size = cx.rem_size();
    //         // cx.scene().push_quad(scene::Quad {
    //         //     bounds,
    //         //     background: Some(hsla(0., 0., 1., 0.05).into()),
    //         //     border: gpui::Border {
    //         //         color: hsla(0., 0., 1., 0.2).into(),
    //         //         top: 1.,
    //         //         right: 1.,
    //         //         bottom: 1.,
    //         //         left: 1.,
    //         //     },
    //         //     corner_radii: CornerRadii::default()
    //         //         .refined(&style.corner_radii)
    //         //         .to_gpui(bounds.size(), rem_size),
    //         // })
    //     }

    //     // let pressed = Cell::new(hovered && cx.is_mouse_down(MouseButton::Left));
    //     // cx.on_event(layout.order, move |_, event: &MouseButtonEvent, _| {
    //     //     if bounds.contains_point(event.position) {
    //     //         if event.is_down {
    //     //             pressed.set(true);
    //     //         } else if pressed.get() {
    //     //             pressed.set(false);
    //     //             eprintln!("clicked div {:?} {:#?}", bounds, style);
    //     //         }
    //     //     }
    //     // });

    //     // let hovered = Cell::new(hovered);
    //     // cx.on_event(layout.order, move |_, event: &MouseMovedEvent, cx| {
    //     //     cx.bubble_event();
    //     //     let hovered_now = bounds.contains_point(event.position);
    //     //     if hovered.get() != hovered_now {
    //     //         hovered.set(hovered_now);
    //     //         cx.repaint();
    //     //     }
    //     // });
    // }
    //
}

impl<V> Styled for Div<V> {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style> {
        &mut self.styles
    }

    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
        self.styles.base()
    }
}

impl<V> StyleHelpers for Div<V> {}

// impl<V> Interactive<V> for Div<V> {
//     fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V> {
//         &mut self.handlers
//     }
// }

impl<V: 'static> ParentElement<V> for Div<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

#[derive(Default, Clone)]
pub struct ScrollState(Arc<Mutex<Point<Pixels>>>);

impl ScrollState {
    pub fn x(&self) -> Pixels {
        self.0.lock().x
    }

    pub fn set_x(&self, value: Pixels) {
        self.0.lock().x = value;
    }

    pub fn y(&self) -> Pixels {
        self.0.lock().y
    }

    pub fn set_y(&self, value: Pixels) {
        self.0.lock().y = value;
    }
}
