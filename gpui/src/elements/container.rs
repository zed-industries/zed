use pathfinder_geometry::rect::RectF;
use serde_json::json;

use crate::{
    color::ColorU,
    geometry::vector::{vec2f, Vector2F},
    json::ToJson,
    scene::{self, Border, Quad},
    AfterLayoutContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct Container {
    margin: Margin,
    padding: Padding,
    background_color: Option<ColorU>,
    border: Border,
    corner_radius: f32,
    shadow: Option<Shadow>,
    child: ElementBox,
}

impl Container {
    pub fn new(child: ElementBox) -> Self {
        Self {
            margin: Margin::default(),
            padding: Padding::default(),
            background_color: None,
            border: Border::default(),
            corner_radius: 0.0,
            shadow: None,
            child,
        }
    }

    pub fn with_margin_top(mut self, margin: f32) -> Self {
        self.margin.top = margin;
        self
    }

    pub fn with_margin_left(mut self, margin: f32) -> Self {
        self.margin.left = margin;
        self
    }

    pub fn with_horizontal_padding(mut self, padding: f32) -> Self {
        self.padding.left = padding;
        self.padding.right = padding;
        self
    }

    pub fn with_vertical_padding(mut self, padding: f32) -> Self {
        self.padding.top = padding;
        self.padding.bottom = padding;
        self
    }

    pub fn with_uniform_padding(mut self, padding: f32) -> Self {
        self.padding = Padding {
            top: padding,
            left: padding,
            bottom: padding,
            right: padding,
        };
        self
    }

    pub fn with_padding_right(mut self, padding: f32) -> Self {
        self.padding.right = padding;
        self
    }

    pub fn with_padding_bottom(mut self, padding: f32) -> Self {
        self.padding.bottom = padding;
        self
    }

    pub fn with_background_color(mut self, color: impl Into<ColorU>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn with_shadow(mut self, offset: Vector2F, blur: f32, color: impl Into<ColorU>) -> Self {
        self.shadow = Some(Shadow {
            offset,
            blur,
            color: color.into(),
        });
        self
    }

    fn margin_size(&self) -> Vector2F {
        vec2f(
            self.margin.left + self.margin.right,
            self.margin.top + self.margin.bottom,
        )
    }

    fn padding_size(&self) -> Vector2F {
        vec2f(
            self.padding.left + self.padding.right,
            self.padding.top + self.padding.bottom,
        )
    }

    fn border_size(&self) -> Vector2F {
        let mut x = 0.0;
        if self.border.left {
            x += self.border.width;
        }
        if self.border.right {
            x += self.border.width;
        }

        let mut y = 0.0;
        if self.border.top {
            y += self.border.width;
        }
        if self.border.bottom {
            y += self.border.width;
        }

        vec2f(x, y)
    }
}

impl Element for Container {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size_buffer = self.margin_size() + self.padding_size() + self.border_size();
        let child_constraint = SizeConstraint {
            min: (constraint.min - size_buffer).max(Vector2F::zero()),
            max: (constraint.max - size_buffer).max(Vector2F::zero()),
        };
        let child_size = self.child.layout(child_constraint, ctx);
        (child_size + size_buffer, ())
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        ctx: &mut AfterLayoutContext,
    ) {
        self.child.after_layout(ctx);
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        ctx: &mut PaintContext,
    ) -> Self::PaintState {
        let quad_bounds = RectF::from_points(
            bounds.origin() + vec2f(self.margin.left, self.margin.top),
            bounds.lower_right() - vec2f(self.margin.right, self.margin.bottom),
        );

        if let Some(shadow) = self.shadow.as_ref() {
            ctx.scene.push_shadow(scene::Shadow {
                bounds: quad_bounds + shadow.offset,
                corner_radius: self.corner_radius,
                sigma: shadow.blur,
                color: shadow.color,
            });
        }
        ctx.scene.push_quad(Quad {
            bounds: quad_bounds,
            background: self.background_color,
            border: self.border,
            corner_radius: self.corner_radius,
        });

        let child_origin = quad_bounds.origin()
            + vec2f(self.padding.left, self.padding.top)
            + vec2f(self.border.left_width(), self.border.top_width());
        self.child.paint(child_origin, ctx);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        ctx: &mut EventContext,
    ) -> bool {
        self.child.dispatch_event(event, ctx)
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        ctx: &crate::DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "Container",
            "bounds": bounds.to_json(),
            "details": {
                "margin": self.margin.to_json(),
                "padding": self.padding.to_json(),
                "background_color": self.background_color.to_json(),
                "border": self.border.to_json(),
                "corner_radius": self.corner_radius,
                "shadow": self.shadow.to_json(),
            },
            "child": self.child.debug(ctx),
        })
    }
}

#[derive(Default)]
pub struct Margin {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

impl ToJson for Margin {
    fn to_json(&self) -> serde_json::Value {
        let mut value = json!({});
        if self.top > 0. {
            value["top"] = json!(self.top);
        }
        if self.right > 0. {
            value["right"] = json!(self.right);
        }
        if self.bottom > 0. {
            value["bottom"] = json!(self.bottom);
        }
        if self.left > 0. {
            value["left"] = json!(self.left);
        }
        value
    }
}

#[derive(Default)]
pub struct Padding {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

impl ToJson for Padding {
    fn to_json(&self) -> serde_json::Value {
        let mut value = json!({});
        if self.top > 0. {
            value["top"] = json!(self.top);
        }
        if self.right > 0. {
            value["right"] = json!(self.right);
        }
        if self.bottom > 0. {
            value["bottom"] = json!(self.bottom);
        }
        if self.left > 0. {
            value["left"] = json!(self.left);
        }
        value
    }
}

#[derive(Default)]
pub struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: ColorU,
}

impl ToJson for Shadow {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "offset": self.offset.to_json(),
            "blur": self.blur,
            "color": self.color.to_json()
        })
    }
}
