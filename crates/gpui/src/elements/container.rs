use pathfinder_geometry::rect::RectF;
use serde::Deserialize;
use serde_json::json;

use crate::{
    color::Color,
    geometry::{
        deserialize_vec2f,
        vector::{vec2f, Vector2F},
    },
    json::ToJson,
    scene::{self, Border, Quad},
    Element, ElementBox, Event, EventContext, LayoutContext, PaintContext, SizeConstraint,
};

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct ContainerStyle {
    #[serde(default)]
    pub margin: Margin,
    #[serde(default)]
    pub padding: Padding,
    #[serde(rename = "background")]
    pub background_color: Option<Color>,
    #[serde(default)]
    pub border: Border,
    #[serde(default)]
    pub corner_radius: f32,
    #[serde(default)]
    pub shadow: Option<Shadow>,
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

    pub fn with_style(mut self, style: ContainerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_margin_top(mut self, margin: f32) -> Self {
        self.style.margin.top = margin;
        self
    }

    pub fn with_margin_bottom(mut self, margin: f32) -> Self {
        self.style.margin.bottom = margin;
        self
    }

    pub fn with_margin_left(mut self, margin: f32) -> Self {
        self.style.margin.left = margin;
        self
    }

    pub fn with_margin_right(mut self, margin: f32) -> Self {
        self.style.margin.right = margin;
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

    pub fn with_padding_left(mut self, padding: f32) -> Self {
        self.style.padding.left = padding;
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

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.style.background_color = Some(color);
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

    pub fn with_shadow(mut self, offset: Vector2F, blur: f32, color: Color) -> Self {
        self.style.shadow = Some(Shadow {
            offset,
            blur,
            color,
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
        let mut size_buffer = self.margin_size() + self.padding_size();
        if !self.style.border.overlay {
            size_buffer += self.border_size();
        }
        let child_constraint = SizeConstraint {
            min: (constraint.min - size_buffer).max(Vector2F::zero()),
            max: (constraint.max - size_buffer).max(Vector2F::zero()),
        };
        let child_size = self.child.layout(child_constraint, cx);
        (child_size + size_buffer, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
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

        let child_origin =
            quad_bounds.origin() + vec2f(self.style.padding.left, self.style.padding.top);

        if self.style.border.overlay {
            cx.scene.push_quad(Quad {
                bounds: quad_bounds,
                background: self.style.background_color,
                border: Default::default(),
                corner_radius: self.style.corner_radius,
            });

            self.child.paint(child_origin, visible_bounds, cx);

            cx.scene.push_layer(None);
            cx.scene.push_quad(Quad {
                bounds: quad_bounds,
                background: Default::default(),
                border: self.style.border,
                corner_radius: self.style.corner_radius,
            });
            cx.scene.pop_layer();
        } else {
            cx.scene.push_quad(Quad {
                bounds: quad_bounds,
                background: self.style.background_color,
                border: self.style.border,
                corner_radius: self.style.corner_radius,
            });

            let child_origin = child_origin
                + vec2f(
                    self.style.border.left_width(),
                    self.style.border.top_width(),
                );
            self.child.paint(child_origin, visible_bounds, cx);
        }
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Margin {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Padding {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}

impl<'de> Deserialize<'de> for Padding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let spacing = Spacing::deserialize(deserializer)?;
        Ok(match spacing {
            Spacing::Uniform(size) => Padding {
                top: size,
                left: size,
                bottom: size,
                right: size,
            },
            Spacing::Specific {
                top,
                left,
                bottom,
                right,
            } => Padding {
                top,
                left,
                bottom,
                right,
            },
        })
    }
}

impl<'de> Deserialize<'de> for Margin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let spacing = Spacing::deserialize(deserializer)?;
        Ok(match spacing {
            Spacing::Uniform(size) => Margin {
                top: size,
                left: size,
                bottom: size,
                right: size,
            },
            Spacing::Specific {
                top,
                left,
                bottom,
                right,
            } => Margin {
                top,
                left,
                bottom,
                right,
            },
        })
    }
}
#[derive(Deserialize)]
#[serde(untagged)]
enum Spacing {
    Uniform(f32),
    Specific {
        #[serde(default)]
        top: f32,
        #[serde(default)]
        left: f32,
        #[serde(default)]
        bottom: f32,
        #[serde(default)]
        right: f32,
    },
}

impl Padding {
    pub fn uniform(padding: f32) -> Self {
        Self {
            top: padding,
            left: padding,
            bottom: padding,
            right: padding,
        }
    }
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

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Shadow {
    #[serde(default, deserialize_with = "deserialize_vec2f")]
    offset: Vector2F,
    #[serde(default)]
    blur: f32,
    #[serde(default)]
    color: Color,
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
