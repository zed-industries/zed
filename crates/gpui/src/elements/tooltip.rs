use super::{ContainerStyle, Element, ElementBox};
use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::{json, ToJson},
    ElementStateHandle, LayoutContext, PaintContext, RenderContext, SizeConstraint, View,
};

pub struct Tooltip {
    state: ElementStateHandle<TooltipState>,
    child: ElementBox,
    style: ContainerStyle,
    text: String,
}

#[derive(Default)]
struct TooltipState {}

impl Tooltip {
    pub fn new<T: View>(
        id: usize,
        child: ElementBox,
        text: String,
        cx: &mut RenderContext<T>,
    ) -> Self {
        Self {
            state: cx.element_state::<Self, _>(id),
            child,
            text,
            style: Default::default(),
        }
    }

    pub fn with_style(mut self, style: ContainerStyle) -> Self {
        self.style = style;
        self
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
            "style": self.style.to_json(),
            "text": &self.text,
        })
    }
}
