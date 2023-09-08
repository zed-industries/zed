use std::cell::Cell;

use crate::{
    element::{AnyElement, Element, IntoElement, Layout, ParentElement},
    hsla,
    layout_context::LayoutContext,
    paint_context::PaintContext,
    style::{CornerRadii, Style, StyleHelpers, Styleable},
    InteractionHandlers, Interactive,
};
use anyhow::Result;
use gpui::{
    geometry::vector::Vector2F,
    platform::{MouseButton, MouseButtonEvent, MouseMovedEvent},
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
}

pub fn div<V>() -> Div<V> {
    Div {
        styles: Default::default(),
        handlers: Default::default(),
        children: Default::default(),
    }
}

impl<V: 'static> Element<V> for Div<V> {
    type PaintState = ();

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

        Ok((cx.add_layout_node(style, children)?, ()))
    }

    fn paint(
        &mut self,
        view: &mut V,
        parent_origin: Vector2F,
        layout: &Layout,
        _: &mut Self::PaintState,
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
        for child in &mut self.children {
            child.paint(view, bounds.origin(), cx);
        }
        style.paint_foreground(bounds, cx);
        if pop_text_style {
            cx.pop_text_style();
        }

        if cx.is_inspector_enabled() {
            self.paint_inspector(parent_origin, layout, cx);
        }
    }
}

impl<V: 'static> Div<V> {
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
