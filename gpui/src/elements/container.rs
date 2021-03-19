use pathfinder_geometry::rect::RectF;

use crate::{
    color::ColorU,
    geometry::vector::{vec2f, Vector2F},
    scene::{Border, Quad},
    AfterLayoutContext, AppContext, Element, Event, EventContext, LayoutContext, MutableAppContext,
    PaintContext, SizeConstraint,
};

pub struct Container {
    margin: Margin,
    padding: Padding,
    overdraw: Overdraw,
    background_color: Option<ColorU>,
    border: Border,
    corner_radius: f32,
    shadow: Option<Shadow>,
    child: Box<dyn Element>,
    size: Option<Vector2F>,
    origin: Option<Vector2F>,
}

impl Container {
    pub fn new(child: Box<dyn Element>) -> Self {
        Self {
            margin: Margin::default(),
            padding: Padding::default(),
            overdraw: Overdraw::default(),
            background_color: None,
            border: Border::default(),
            corner_radius: 0.0,
            shadow: None,
            child,
            size: None,
            origin: None,
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
    fn layout(
        &mut self,
        constraint: SizeConstraint,
        ctx: &mut LayoutContext,
        app: &AppContext,
    ) -> Vector2F {
        let size_buffer = self.margin_size() + self.padding_size() + self.border_size();
        let child_constraint = SizeConstraint {
            min: (constraint.min - size_buffer).max(Vector2F::zero()),
            max: (constraint.max - size_buffer).max(Vector2F::zero()),
        };
        let child_size = self.child.layout(child_constraint, ctx, app);
        let size = child_size + size_buffer;
        self.size = Some(size);
        size
    }

    fn after_layout(&mut self, ctx: &mut AfterLayoutContext, app: &mut MutableAppContext) {
        self.child.after_layout(ctx, app);
    }

    fn paint(&mut self, origin: Vector2F, ctx: &mut PaintContext, app: &AppContext) {
        ctx.scene.push_quad(Quad {
            bounds: RectF::new(origin, self.size.unwrap()),
            background: self.background_color,
            border: self.border,
            corder_radius: self.corner_radius,
        });
        self.child.paint(origin, ctx, app);
    }

    fn dispatch_event(&self, event: &Event, ctx: &mut EventContext, app: &AppContext) -> bool {
        self.child.dispatch_event(event, ctx, app)
    }

    fn size(&self) -> Option<Vector2F> {
        self.size
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
