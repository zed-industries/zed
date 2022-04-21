use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct Overlay {
    child: ElementBox,
}

impl Overlay {
    pub fn new(child: ElementBox) -> Self {
        Self { child }
    }
}

impl Element for Overlay {
    type LayoutState = Vector2F;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size = self.child.layout(constraint, cx);
        (Vector2F::zero(), size)
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: RectF,
        size: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) {
        let bounds = RectF::new(bounds.origin(), *size);
        cx.scene.push_stacking_context(None);
        self.child.paint(bounds.origin(), bounds, cx);
        cx.scene.pop_stacking_context();
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, cx)
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value {
        self.child.debug(cx)
    }
}
