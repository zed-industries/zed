use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::json,
    DebugContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct Hooks {
    child: ElementBox,
    before_layout: Option<Box<dyn FnMut(SizeConstraint, &mut LayoutContext)>>,
}

impl Hooks {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            before_layout: None,
        }
    }

    pub fn on_before_layout(
        mut self,
        f: impl 'static + FnMut(SizeConstraint, &mut LayoutContext),
    ) -> Self {
        self.before_layout = Some(Box::new(f));
        self
    }
}

impl Element for Hooks {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        if let Some(handler) = self.before_layout.as_mut() {
            handler(constraint, cx);
        }
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
        event: &Event,
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
        json!({
            "type": "Hooks",
            "child": self.child.debug(cx),
        })
    }
}
