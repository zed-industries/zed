use std::{cell::Cell, rc::Rc};

use crate::{
    element::{AnyElement, Element, IntoElement, Layout, ParentElement},
    hsla,
    layout_context::LayoutContext,
    paint_context::PaintContext,
    style::{CornerRadii, Overflow, Style, StyleHelpers, Styleable},
    InteractionHandlers, Interactive,
};
use anyhow::Result;
use gpui::{
    geometry::{rect::RectF, vector::Vector2F, Point},
    platform::{MouseButton, MouseButtonEvent, MouseMovedEvent, ScrollWheelEvent},
    scene::{self},
    LayoutId,
};
use refineable::{Refineable, RefinementCascade};
use smallvec::SmallVec;
use util::ResultExt;

pub struct Div<V: 'static> {
    styles: RefinementCascade<Style>,
    handlers: InteractionHandlers<V>,
    children: SmallVec<[AnyElement<V>; 2]>,
    scroll_state: Option<ScrollState>,
}

pub fn div<V>() -> Div<V> {
    Div {
        styles: Default::default(),
        handlers: Default::default(),
        children: Default::default(),
        scroll_state: None,
    }
}

impl<V: 'static> Element<V> for Div<V> {
    type PaintState = Vec<LayoutId>;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Result<(LayoutId, Self::PaintState)>
    where
        Self: Sized,
    {
        let style = self.computed_style();
        let pop_text_style = style.text_style(cx).map_or(false, |style| {
            cx.push_text_style(&style).log_err().is_some()
        });

        let children = self
            .children
            .iter_mut()
            .map(|child| child.layout(view, cx))
            .collect::<Result<Vec<LayoutId>>>()?;

        if pop_text_style {
            cx.pop_text_style();
        }

        Ok((cx.add_layout_node(style, children.clone())?, children))
    }

    fn paint(
        &mut self,
        view: &mut V,
        parent_origin: Vector2F,
        layout: &Layout,
        child_layouts: &mut Vec<LayoutId>,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized,
    {
        let order = layout.order;
        let bounds = layout.bounds + parent_origin;

        let style = self.computed_style();
        let pop_text_style = style.text_style(cx).map_or(false, |style| {
            cx.push_text_style(&style).log_err().is_some()
        });
        style.paint_background(bounds, cx);
        self.interaction_handlers().paint(order, bounds, cx);

        let scrolled_origin = bounds.origin() - self.scroll_offset(&style.overflow);

        // TODO: Support only one dimension being hidden
        let mut pop_layer = false;
        if style.overflow.y != Overflow::Visible || style.overflow.x != Overflow::Visible {
            cx.scene.push_layer(Some(bounds));
            pop_layer = true;
        }

        for child in &mut self.children {
            child.paint(view, scrolled_origin, cx);
        }

        if pop_layer {
            cx.scene.pop_layer();
        }

        style.paint_foreground(bounds, cx);
        if pop_text_style {
            cx.pop_text_style();
        }

        self.handle_scroll(order, bounds, style.overflow.clone(), child_layouts, cx);

        if cx.is_inspector_enabled() {
            self.paint_inspector(parent_origin, layout, cx);
        }
    }
}

impl<V: 'static> Div<V> {
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

    fn scroll_offset(&self, overflow: &Point<Overflow>) -> Vector2F {
        let mut offset = Vector2F::zero();
        if overflow.y == Overflow::Scroll {
            offset.set_y(self.scroll_state.as_ref().unwrap().y());
        }
        if overflow.x == Overflow::Scroll {
            offset.set_x(self.scroll_state.as_ref().unwrap().x());
        }

        offset
    }

    fn handle_scroll(
        &mut self,
        order: u32,
        bounds: RectF,
        overflow: Point<Overflow>,
        child_layout_ids: &[LayoutId],
        cx: &mut PaintContext<V>,
    ) {
        if overflow.y == Overflow::Scroll || overflow.x == Overflow::Scroll {
            let mut scroll_max = Vector2F::zero();
            for child_layout_id in child_layout_ids {
                if let Some(child_layout) = cx
                    .layout_engine()
                    .unwrap()
                    .computed_layout(*child_layout_id)
                    .log_err()
                {
                    scroll_max = scroll_max.max(child_layout.bounds.lower_right());
                }
            }
            scroll_max -= bounds.size();

            let scroll_state = self.scroll_state.as_ref().unwrap().clone();
            cx.on_event(order, move |_, event: &ScrollWheelEvent, cx| {
                if bounds.contains_point(event.position) {
                    let scroll_delta = match event.delta {
                        gpui::platform::ScrollDelta::Pixels(delta) => delta,
                        gpui::platform::ScrollDelta::Lines(delta) => {
                            delta * cx.text_style().font_size
                        }
                    };
                    if overflow.x == Overflow::Scroll {
                        scroll_state.set_x(
                            (scroll_state.x() - scroll_delta.x())
                                .max(0.)
                                .min(scroll_max.x()),
                        );
                    }
                    if overflow.y == Overflow::Scroll {
                        scroll_state.set_y(
                            (scroll_state.y() - scroll_delta.y())
                                .max(0.)
                                .min(scroll_max.y()),
                        );
                    }
                    cx.repaint();
                } else {
                    cx.bubble_event();
                }
            })
        }
    }

    fn paint_inspector(&self, parent_origin: Vector2F, layout: &Layout, cx: &mut PaintContext<V>) {
        let style = self.styles.merged();
        let bounds = layout.bounds + parent_origin;

        let hovered = bounds.contains_point(cx.mouse_position());
        if hovered {
            let rem_size = cx.rem_size();
            cx.scene.push_quad(scene::Quad {
                bounds,
                background: Some(hsla(0., 0., 1., 0.05).into()),
                border: gpui::Border {
                    color: hsla(0., 0., 1., 0.2).into(),
                    top: 1.,
                    right: 1.,
                    bottom: 1.,
                    left: 1.,
                },
                corner_radii: CornerRadii::default()
                    .refined(&style.corner_radii)
                    .to_gpui(bounds.size(), rem_size),
            })
        }

        let pressed = Cell::new(hovered && cx.is_mouse_down(MouseButton::Left));
        cx.on_event(layout.order, move |_, event: &MouseButtonEvent, _| {
            if bounds.contains_point(event.position) {
                if event.is_down {
                    pressed.set(true);
                } else if pressed.get() {
                    pressed.set(false);
                    eprintln!("clicked div {:?} {:#?}", bounds, style);
                }
            }
        });

        let hovered = Cell::new(hovered);
        cx.on_event(layout.order, move |_, event: &MouseMovedEvent, cx| {
            cx.bubble_event();
            let hovered_now = bounds.contains_point(event.position);
            if hovered.get() != hovered_now {
                hovered.set(hovered_now);
                cx.repaint();
            }
        });
    }
}

impl<V> Styleable for Div<V> {
    type Style = Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style> {
        &mut self.styles
    }

    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
        self.styles.base()
    }
}

impl<V> StyleHelpers for Div<V> {}

impl<V> Interactive<V> for Div<V> {
    fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V> {
        &mut self.handlers
    }
}

impl<V: 'static> ParentElement<V> for Div<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

impl<V: 'static> IntoElement<V> for Div<V> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Default, Clone)]
pub struct ScrollState(Rc<Cell<Vector2F>>);

impl ScrollState {
    pub fn x(&self) -> f32 {
        self.0.get().x()
    }

    pub fn set_x(&self, value: f32) {
        let mut current_value = self.0.get();
        current_value.set_x(value);
        self.0.set(current_value);
    }

    pub fn y(&self) -> f32 {
        self.0.get().y()
    }

    pub fn set_y(&self, value: f32) {
        let mut current_value = self.0.get();
        current_value.set_y(value);
        self.0.set(current_value);
    }
}
