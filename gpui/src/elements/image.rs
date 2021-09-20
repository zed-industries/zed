use super::constrain_size_preserving_aspect_ratio;
use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::{json, ToJson},
    scene, Border, DebugContext, Element, Event, EventContext, ImageData, LayoutContext,
    PaintContext, SizeConstraint,
};
use serde::Deserialize;
use std::sync::Arc;

pub struct Image {
    data: Arc<ImageData>,
    style: ImageStyle,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct ImageStyle {
    #[serde(default)]
    border: Border,
    #[serde(default)]
    corner_radius: f32,
}

impl Image {
    pub fn new(data: Arc<ImageData>) -> Self {
        Self {
            data,
            style: Default::default(),
        }
    }

    pub fn with_style(mut self, style: ImageStyle) -> Self {
        self.style = style;
        self
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
        let size =
            constrain_size_preserving_aspect_ratio(constraint.max, self.data.size().to_f32());
        (size, ())
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
            border: self.style.border,
            corner_radius: self.style.corner_radius,
            data: self.data.clone(),
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
