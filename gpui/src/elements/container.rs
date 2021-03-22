use pathfinder_geometry::rect::RectF;

use crate::{
    color::ColorU,
    geometry::vector::{vec2f, Vector2F},
    scene::{Border, Quad},
    AfterLayoutContext, Element, ElementBox, Event, EventContext, LayoutContext, PaintContext,
    SizeConstraint,
};

pub struct Container {
    margin: Margin,
    padding: Padding,
    overdraw: Overdraw,
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
            overdraw: Overdraw::default(),
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

    pub fn with_background_color(mut self, color: impl Into<ColorU>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn with_overdraw_bottom(mut self, overdraw: f32) -> Self {
        self.overdraw.bottom = overdraw;
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
        ctx.scene.push_quad(Quad {
            bounds,
            background: self.background_color,
            border: self.border,
            corner_radius: self.corner_radius,
        });
        self.child.paint(bounds.origin(), ctx);
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
}

#[derive(Default)]
pub struct Margin {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

#[derive(Default)]
pub struct Padding {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

#[derive(Default)]
pub struct Overdraw {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
}

#[derive(Default)]
pub struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: ColorU,
}
