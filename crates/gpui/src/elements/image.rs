use super::constrain_size_preserving_aspect_ratio;
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    presenter::MeasurementContext,
    scene, Border, DebugContext, Element, ImageData, LayoutContext, PaintContext, SizeConstraint,
};
use serde::Deserialize;
use std::{ops::Range, sync::Arc};

enum ImageSource {
    Path(&'static str),
    Data(Arc<ImageData>),
}

pub struct Image {
    source: ImageSource,
    style: ImageStyle,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct ImageStyle {
    #[serde(default)]
    pub border: Border,
    #[serde(default)]
    pub corner_radius: f32,
    #[serde(default)]
    pub height: Option<f32>,
    #[serde(default)]
    pub width: Option<f32>,
    #[serde(default)]
    pub grayscale: bool,
}

impl Image {
    pub fn new(asset_path: &'static str) -> Self {
        Self {
            source: ImageSource::Path(asset_path),
            style: Default::default(),
        }
    }

    pub fn from_data(data: Arc<ImageData>) -> Self {
        Self {
            source: ImageSource::Data(data),
            style: Default::default(),
        }
    }

    pub fn with_style(mut self, style: ImageStyle) -> Self {
        self.style = style;
        self
    }
}

impl Element for Image {
    type LayoutState = Option<Arc<ImageData>>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let data = match &self.source {
            ImageSource::Path(path) => match cx.asset_cache.png(path) {
                Ok(data) => data,
                Err(error) => {
                    log::error!("could not load image: {}", error);
                    return (Vector2F::zero(), None);
                }
            },
            ImageSource::Data(data) => data.clone(),
        };

        let desired_size = vec2f(
            self.style.width.unwrap_or_else(|| constraint.max.x()),
            self.style.height.unwrap_or_else(|| constraint.max.y()),
        );
        let size = constrain_size_preserving_aspect_ratio(
            constraint.constrain(desired_size),
            data.size().to_f32(),
        );

        (size, Some(data))
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        if let Some(data) = layout {
            cx.scene.push_image(scene::Image {
                bounds,
                border: self.style.border,
                corner_radius: self.style.corner_radius,
                grayscale: self.style.grayscale,
                data: data.clone(),
            });
        }
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &MeasurementContext,
    ) -> Option<RectF> {
        None
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
