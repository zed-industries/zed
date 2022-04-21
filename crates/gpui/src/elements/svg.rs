use std::borrow::Cow;

use serde_json::json;

use crate::{
    color::Color,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    scene, DebugContext, Element, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};

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

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Element for Svg {
    type LayoutState = Option<usvg::Tree>;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
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
        cx: &mut PaintContext,
    ) {
        if let Some(svg) = svg.clone() {
            cx.scene.push_icon(scene::Icon {
                bounds,
                svg,
                path: self.path.clone(),
                color: self.color,
            });
        }
    }

    fn dispatch_event(
        &mut self,
        _: &Event,
        _: RectF,
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
            "type": "Svg",
            "bounds": bounds.to_json(),
            "path": self.path,
            "color": self.color.to_json(),
        })
    }
}

use crate::json::ToJson;

use super::constrain_size_preserving_aspect_ratio;

fn from_usvg_rect(rect: usvg::Rect) -> RectF {
    RectF::new(
        vec2f(rect.x() as f32, rect.y() as f32),
        vec2f(rect.width() as f32, rect.height() as f32),
    )
}
