use crate::{
    color::ColorU,
    geometry::vector::{vec2f, Vector2F},
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
        // self.origin = Some(origin);

        // let canvas = &mut ctx.canvas;
        // let size = self.size.unwrap() - self.margin_size()
        //     + vec2f(self.overdraw.right, self.overdraw.bottom);
        // let origin = origin + vec2f(self.margin.left, self.margin.top)
        //     - vec2f(self.overdraw.left, self.overdraw.top);
        // let rect = RectF::new(origin, size);

        // let mut path = Path2D::new();
        // if self.corner_radius > 0.0 {
        //     path.move_to(rect.upper_right() - vec2f(self.corner_radius, 0.0));
        //     path.arc_to(
        //         rect.upper_right(),
        //         rect.upper_right() + vec2f(0.0, self.corner_radius),
        //         self.corner_radius,
        //     );
        //     path.line_to(rect.lower_right() - vec2f(0.0, self.corner_radius));
        //     path.arc_to(
        //         rect.lower_right(),
        //         rect.lower_right() - vec2f(self.corner_radius, 0.0),
        //         self.corner_radius,
        //     );
        //     path.line_to(rect.lower_left() + vec2f(self.corner_radius, 0.0));
        //     path.arc_to(
        //         rect.lower_left(),
        //         rect.lower_left() - vec2f(0.0, self.corner_radius),
        //         self.corner_radius,
        //     );
        //     path.line_to(origin + vec2f(0.0, self.corner_radius));
        //     path.arc_to(
        //         origin,
        //         origin + vec2f(self.corner_radius, 0.0),
        //         self.corner_radius,
        //     );
        //     path.close_path();
        // } else {
        //     path.rect(rect);
        // }

        // canvas.save();
        // if let Some(shadow) = self.shadow.as_ref() {
        //     canvas.set_shadow_offset(shadow.offset);
        //     canvas.set_shadow_blur(shadow.blur);
        //     canvas.set_shadow_color(shadow.color);
        // }

        // if let Some(background_color) = self.background_color {
        //     canvas.set_fill_style(FillStyle::Color(background_color));
        //     canvas.fill_path(path.clone(), FillRule::Winding);
        // }

        // canvas.set_line_width(self.border.width);
        // canvas.set_stroke_style(FillStyle::Color(self.border.color));

        // let border_rect = rect.contract(self.border.width / 2.0);

        // // For now, we ignore the corner radius unless we draw a border on all sides.
        // // This could be improved.
        // if self.border.all_sides() {
        //     let mut path = Path2D::new();
        //     path.rect(border_rect);
        //     canvas.stroke_path(path);
        // } else {
        //     canvas.set_line_cap(LineCap::Square);

        //     if self.border.top {
        //         let mut path = Path2D::new();
        //         path.move_to(border_rect.origin());
        //         path.line_to(border_rect.upper_right());
        //         canvas.stroke_path(path);
        //     }

        //     if self.border.left {
        //         let mut path = Path2D::new();
        //         path.move_to(border_rect.origin());
        //         path.line_to(border_rect.lower_left());
        //         canvas.stroke_path(path);
        //     }

        //     if self.border.bottom {
        //         let mut path = Path2D::new();
        //         path.move_to(border_rect.lower_left());
        //         path.line_to(border_rect.lower_right());
        //         canvas.stroke_path(path);
        //     }

        //     if self.border.right {
        //         let mut path = Path2D::new();
        //         path.move_to(border_rect.upper_right());
        //         path.line_to(border_rect.lower_right());
        //         canvas.stroke_path(path);
        //     }
        // }
        // canvas.restore();

        // let mut child_origin = origin + vec2f(self.padding.left, self.padding.top);
        // if self.border.left {
        //     child_origin.set_x(child_origin.x() + self.border.width);
        // }
        // if self.border.top {
        //     child_origin.set_y(child_origin.y() + self.border.width);
        // }
        // self.child.paint(child_origin, ctx, app);
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
pub struct Border {
    width: f32,
    color: ColorU,
    pub top: bool,
    pub left: bool,
    pub bottom: bool,
    pub right: bool,
}

impl Border {
    pub fn new(width: f32, color: impl Into<ColorU>) -> Self {
        Self {
            width,
            color: color.into(),
            top: false,
            left: false,
            bottom: false,
            right: false,
        }
    }

    pub fn all(width: f32, color: impl Into<ColorU>) -> Self {
        Self {
            width,
            color: color.into(),
            top: true,
            left: true,
            bottom: true,
            right: true,
        }
    }

    pub fn top(width: f32, color: impl Into<ColorU>) -> Self {
        let mut border = Self::new(width, color);
        border.top = true;
        border
    }

    pub fn left(width: f32, color: impl Into<ColorU>) -> Self {
        let mut border = Self::new(width, color);
        border.left = true;
        border
    }

    pub fn bottom(width: f32, color: impl Into<ColorU>) -> Self {
        let mut border = Self::new(width, color);
        border.bottom = true;
        border
    }

    pub fn right(width: f32, color: impl Into<ColorU>) -> Self {
        let mut border = Self::new(width, color);
        border.right = true;
        border
    }

    pub fn with_sides(mut self, top: bool, left: bool, bottom: bool, right: bool) -> Self {
        self.top = top;
        self.left = left;
        self.bottom = bottom;
        self.right = right;
        self
    }

    fn all_sides(&self) -> bool {
        self.top && self.left && self.bottom && self.right
    }
}

#[derive(Default)]
pub struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: ColorU,
}
