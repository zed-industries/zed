use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::{json, ToJson},
    scene, DebugContext, Element, Event, EventContext, ImageData, LayoutContext, PaintContext,
    SizeConstraint,
};
use std::sync::Arc;

pub struct Image(Arc<ImageData>);

impl Image {
    pub fn new(data: Arc<ImageData>) -> Self {
        Self(data)
    }
}

impl Element for Image {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        cx.scene.push_image(scene::Image {
            bounds,
            data: self.0.clone(),
        });
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut EventContext,
    ) -> bool {
        false
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "Image",
            "bounds": bounds.to_json(),
        })
    }
}
