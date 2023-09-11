use super::constrain_size_preserving_aspect_ratio;
use crate::json::ToJson;
use crate::PaintContext;
use crate::{
    color::Color,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    scene, Element, SizeConstraint, ViewContext,
};
use schemars::JsonSchema;
use serde_derive::Deserialize;
use serde_json::json;
use std::{borrow::Cow, ops::Range};

pub struct Svg {
    path: Cow<'static, str>,
    color: Color,
}

impl Svg {
    pub fn new(path: impl Into<Cow<'static, str>>) -> Self {
        Self {
            path: path.into(),
            color: Color::black(),
        }
    }

    pub fn for_style<V: 'static>(style: SvgStyle) -> impl Element<V> {
        Self::new(style.asset)
            .with_color(style.color)
            .constrained()
            .with_width(style.dimensions.width)
            .with_height(style.dimensions.height)
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl<V: 'static> Element<V> for Svg {
    type LayoutState = Option<usvg::Tree>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        match cx.asset_cache.svg(&self.path) {
            Ok(tree) => {
                let size = constrain_size_preserving_aspect_ratio(
                    constraint.max,
                    from_usvg_rect(tree.svg_node().view_box.rect).size(),
                );
                (size, Some(tree))
            }
            Err(_error) => {
                #[cfg(not(any(test, feature = "test-support")))]
                log::error!("{}", _error);
                (constraint.min, None)
            }
        }
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _visible_bounds: RectF,
        svg: &mut Self::LayoutState,
        _: &mut V,
        cx: &mut PaintContext<V>,
    ) {
        if let Some(svg) = svg.clone() {
            cx.scene().push_icon(scene::Icon {
                bounds,
                svg,
                path: self.path.clone(),
                color: self.color,
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
        _: &V,
        _: &ViewContext<V>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "type": "Svg",
            "bounds": bounds.to_json(),
            "path": self.path,
            "color": self.color.to_json(),
        })
    }
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct SvgStyle {
    pub color: Color,
    pub asset: String,
    pub dimensions: Dimensions,
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    pub fn to_vec(&self) -> Vector2F {
        vec2f(self.width, self.height)
    }
}

fn from_usvg_rect(rect: usvg::Rect) -> RectF {
    RectF::new(
        vec2f(rect.x() as f32, rect.y() as f32),
        vec2f(rect.width() as f32, rect.height() as f32),
    )
}
