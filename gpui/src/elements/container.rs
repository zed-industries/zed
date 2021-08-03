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

#[derive(Clone, Debug, Default)]
pub struct ContainerStyle {
    margin: Margin,
    padding: Padding,
    background_color: Option<ColorU>,
    border: Border,
    corner_radius: f32,
    shadow: Option<Shadow>,
}

pub struct Container {
    child: ElementBox,
    style: ContainerStyle,
}

impl Container {
    pub fn new(child: ElementBox) -> Self {
        Self {
            child,
            style: Default::default(),
        }
    }

    pub fn with_style(mut self, style: &ContainerStyle) -> Self {
        self.style = style.clone();
        self
    }

    pub fn with_margin_top(mut self, margin: f32) -> Self {
        self.style.margin.top = margin;
        self
    }

    pub fn with_margin_left(mut self, margin: f32) -> Self {
        self.style.margin.left = margin;
        self
    }

    pub fn with_horizontal_padding(mut self, padding: f32) -> Self {
        self.style.padding.left = padding;
        self.style.padding.right = padding;
        self
    }

    pub fn with_vertical_padding(mut self, padding: f32) -> Self {
        self.style.padding.top = padding;
        self.style.padding.bottom = padding;
        self
    }

    pub fn with_uniform_padding(mut self, padding: f32) -> Self {
        self.style.padding = Padding {
            top: padding,
            left: padding,
            bottom: padding,
            right: padding,
        };
        self
    }

    pub fn with_padding_right(mut self, padding: f32) -> Self {
        self.style.padding.right = padding;
        self
    }

    pub fn with_padding_bottom(mut self, padding: f32) -> Self {
        self.style.padding.bottom = padding;
        self
    }

    pub fn with_background_color(mut self, color: impl Into<ColorU>) -> Self {
        self.style.background_color = Some(color.into());
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.style.border = border;
        self
    }

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.style.corner_radius = radius;
        self
    }

    pub fn with_shadow(mut self, offset: Vector2F, blur: f32, color: impl Into<ColorU>) -> Self {
        self.style.shadow = Some(Shadow {
            offset,
            blur,
            color: color.into(),
        });
        self
    }

    fn margin_size(&self) -> Vector2F {
        vec2f(
            self.style.margin.left + self.style.margin.right,
            self.style.margin.top + self.style.margin.bottom,
        )
    }

    fn padding_size(&self) -> Vector2F {
        vec2f(
            self.style.padding.left + self.style.padding.right,
            self.style.padding.top + self.style.padding.bottom,
        )
    }

    fn border_size(&self) -> Vector2F {
        let mut x = 0.0;
        if self.style.border.left {
            x += self.style.border.width;
        }
        if self.style.border.right {
            x += self.style.border.width;
        }

        let mut y = 0.0;
        if self.style.border.top {
            y += self.style.border.width;
        }
        if self.style.border.bottom {
            y += self.style.border.width;
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
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size_buffer = self.margin_size() + self.padding_size() + self.border_size();
        let child_constraint = SizeConstraint {
            min: (constraint.min - size_buffer).max(Vector2F::zero()),
            max: (constraint.max - size_buffer).max(Vector2F::zero()),
        };
        let child_size = self.child.layout(child_constraint, cx);
        (child_size + size_buffer, ())
    }

    fn after_layout(
        &mut self,
        _: Vector2F,
        _: &mut Self::LayoutState,
        cx: &mut AfterLayoutContext,
    ) {
        self.child.after_layout(cx);
    }

    fn paint(
        &mut self,
        bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let quad_bounds = RectF::from_points(
            bounds.origin() + vec2f(self.style.margin.left, self.style.margin.top),
            bounds.lower_right() - vec2f(self.style.margin.right, self.style.margin.bottom),
        );

        if let Some(shadow) = self.style.shadow.as_ref() {
            cx.scene.push_shadow(scene::Shadow {
                bounds: quad_bounds + shadow.offset,
                corner_radius: self.style.corner_radius,
                sigma: shadow.blur,
                color: shadow.color,
            });
        }
        cx.scene.push_quad(Quad {
            bounds: quad_bounds,
            background: self.style.background_color,
            border: self.style.border,
            corner_radius: self.style.corner_radius,
        });

        let child_origin = quad_bounds.origin()
            + vec2f(self.style.padding.left, self.style.padding.top)
            + vec2f(
                self.style.border.left_width(),
                self.style.border.top_width(),
            );
        self.child.paint(child_origin, cx);
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
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &crate::DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "Container",
            "bounds": bounds.to_json(),
            "details": self.style.to_json(),
            "child": self.child.debug(cx),
        })
    }
}

impl ToJson for ContainerStyle {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "margin": self.margin.to_json(),
            "padding": self.padding.to_json(),
            "background_color": self.background_color.to_json(),
            "border": self.border.to_json(),
            "corner_radius": self.corner_radius,
            "shadow": self.shadow.to_json(),
        })
    }
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
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
