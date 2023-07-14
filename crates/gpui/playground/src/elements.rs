use gpui::{
    color::Color,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{json, ToJson},
    scene,
    serde_json::Value,
    AnyElement, Element, LayoutContext, Quad, SceneBuilder, SizeConstraint, View, ViewContext,
};
use std::{any::Any, ops::Range, rc::Rc};

pub struct Atom<V: View> {
    style: Rc<AtomStyle>,
    children: Vec<AnyElement<V>>,
}

impl<V: View> Atom<V> {
    pub fn new(style: impl Into<Rc<AtomStyle>>) -> Self {
        Self {
            style: style.into(),
            children: Vec::new(),
        }
    }

    fn inner_constraint(&self, mut constraint: SizeConstraint) -> SizeConstraint {
        // Constrain width
        constraint
            .max
            .set_x(constraint.max.x().min(self.style.width.max()));

        // Constrain height
        constraint
            .max
            .set_y(constraint.max.y().min(self.style.height.max()));

        // Account for margin, border, and padding
        let inset = self.inset_size();
        SizeConstraint {
            min: (constraint.min - inset).max(Vector2F::zero()),
            max: (constraint.max - inset).max(Vector2F::zero()),
        }
    }

    // fn layout_2d_children(
    //     &mut self,
    //     orientation: Axis2d,
    //     constraint: SizeConstraint,
    //     view: &mut V,
    //     cx: &mut LayoutContext<V>,
    // ) -> Vector2F {
    //     let mut total_flex: Option<f32> = None;
    //     let mut total_size = 0.0;
    //     let mut cross_axis_max: f32 = 0.0;

    //     // First pass: Layout non-flex children only
    //     for child in &mut self.children {
    //         if let Some(child_flex) = child.metadata::<AtomStyle>().and_then(|style| style.flex) {
    //             *total_flex.get_or_insert(0.) += child_flex;
    //         } else {
    //             let child_constraint = match orientation {
    //                 Axis2d::X => SizeConstraint::new(
    //                     vec2f(0.0, constraint.min.y()),
    //                     vec2f(INFINITY, constraint.max.y()),
    //                 ),
    //                 Axis2d::Y => SizeConstraint::new(
    //                     vec2f(constraint.min.x(), 0.0),
    //                     vec2f(constraint.max.x(), INFINITY),
    //                 ),
    //             };
    //             let child_size = child.layout(child_constraint, view, cx);
    //             total_size += match orientation {
    //                 Axis3d::Horizontal => {
    //                     cross_axis_max = cross_axis_max.max(child_size.y());
    //                     child_size.x()
    //                 }
    //                 Axis3d::Vertical => {
    //                     cross_axis_max = cross_axis_max.max(child_size.x());
    //                     child_size.y()
    //                 }
    //             };
    //         }
    //     }

    //     let remaining_space = match orientation {
    //         Axis3d::Vertical => constraint.max.y() - total_size,
    //         Axis3d::Horizontal => constraint.max.x() - total_size,
    //     };

    //     // Second pass: Layout flexible children
    //     if let Some(total_flex) = total_flex {
    //         if total_flex > 0. {
    //             let space_per_flex = remaining_space.max(0.) / total_flex;

    //             for child in &mut self.children {
    //                 if let Some(child_flex) =
    //                     child.metadata::<AtomStyle>().and_then(|style| style.flex)
    //                 {
    //                     let child_max = space_per_flex * child_flex;
    //                     let mut child_constraint = constraint;
    //                     match orientation {
    //                         Axis3d::Vertical => {
    //                             child_constraint.min.set_y(0.0);
    //                             child_constraint.max.set_y(child_max);
    //                         }
    //                         Axis3d::Horizontal => {
    //                             child_constraint.min.set_x(0.0);
    //                             child_constraint.max.set_x(child_max);
    //                         }
    //                     }

    //                     let child_size = child.layout(child_constraint, view, cx);

    //                     cross_axis_max = match orientation {
    //                         Axis3d::Vertical => {
    //                             total_size += child_size.y();
    //                             cross_axis_max.max(child_size.x())
    //                         }
    //                         Axis3d::Horizontal => {
    //                             total_size += child_size.x();
    //                             cross_axis_max.max(child_size.y())
    //                         }
    //                     };
    //                 }
    //             }
    //         }
    //     }

    //     let size = match orientation {
    //         Axis3d::Vertical => vec2f(cross_axis_max, total_size),
    //         Axis3d::Horizontal => vec2f(total_size, cross_axis_max),
    //     };
    //     size
    // }

    // fn layout_stacked_children(
    //     &mut self,
    //     constraint: SizeConstraint,
    //     view: &mut V,
    //     cx: &mut LayoutContext<V>,
    // ) -> Vector2F {
    //     let mut size = Vector2F::zero();

    //     for child in &mut self.children {
    //         let child_size = child.layout(constraint, view, cx);
    //         size.set_x(size.x().max(child_size.x()));
    //         size.set_y(size.y().max(child_size.y()));
    //     }

    //     size
    // }

    fn inset_size(&self) -> Vector2F {
        self.padding_size() + self.border_size() + self.margin_size()
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

impl<V: View> Element<V> for Atom<V> {
    type LayoutState = Vector2F; // Content size
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let inner_constraint = self.inner_constraint(constraint);
        // let size_of_children = match self.style.axis {
        //     // Axis3d::X => self.layout_2d_children(Axis2d::X, constraint, view, cx),
        //     // Axis3d::Y => self.layout_2d_children(Axis2d::Y, constraint, view, cx),
        //     // Axis3d::Z => self.layout_stacked_children(constraint, view, cx),
        // };
        let size_of_children = inner_constraint.max; // TODO!

        // Add back space for padding, border, and margin.
        let mut size = size_of_children + self.inset_size();

        // Impose horizontal constraints
        if constraint.min.x().is_finite() {
            size.set_x(size.x().max(constraint.min.x()));
        }
        if size.x() > constraint.max.x() {
            size.set_x(constraint.max.x());
        }

        // Impose vertical constraints
        if constraint.min.y().is_finite() {
            size.set_y(size.y().max(constraint.min.y()));
        }
        if size.y() > constraint.max.y() {
            size.set_y(constraint.max.y());
        }

        (size, size_of_children)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        size_of_children: &mut Vector2F,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        let margin = &self.style.margin;

        // Account for margins
        let content_bounds = RectF::from_points(
            bounds.origin() + vec2f(margin.left, margin.top),
            bounds.lower_right() - vec2f(margin.right, margin.bottom),
        );

        // Paint drop shadow
        for shadow in &self.style.shadows {
            scene.push_shadow(scene::Shadow {
                bounds: content_bounds + shadow.offset,
                corner_radius: self.style.corner_radius,
                sigma: shadow.blur,
                color: shadow.color,
            });
        }

        // // Paint cursor style
        // if let Some(hit_bounds) = content_bounds.intersection(visible_bounds) {
        //     if let Some(style) = self.style.cursor {
        //         scene.push_cursor_region(CursorRegion {
        //             bounds: hit_bounds,
        //             style,
        //         });
        //     }
        // }

        // Render the background and/or the border (if it not an overlay border).
        match self.style.fill {
            Fill::Color(fill_color) => {}
        }
        if let Fill::Color(fill_color) = self.style.fill {
            let is_fill_visible = !fill_color.is_fully_transparent();
            if is_fill_visible || self.style.border.is_visible() {
                scene.push_quad(Quad {
                    bounds: content_bounds,
                    background: is_fill_visible.then_some(fill_color),
                    border: scene::Border {
                        width: self.style.border.width,
                        color: self.style.border.color,
                        overlay: false,
                        top: self.style.border.top,
                        right: self.style.border.right,
                        bottom: self.style.border.bottom,
                        left: self.style.border.left,
                    },
                    corner_radius: self.style.corner_radius,
                });
            }
        }

        if !self.children.is_empty() {
            // Account for padding first.
            let padding = &self.style.padding;
            let padded_bounds = RectF::from_points(
                content_bounds.origin() + vec2f(padding.left, padding.top),
                content_bounds.lower_right() - vec2f(padding.right, padding.top),
            );
            let parent_size = padded_bounds.size();

            // Now paint the children accourding to the orientation.
            let child_aligment = self.style.align;
            // match self.style.orientation {
            //     Orientation::Axial(axis) => {
            //         let mut child_origin = padded_bounds.origin();
            //         // Align all children together along the primary axis
            //         match axis {
            //             Axis3d::Horizontal => align_child(
            //                 &mut child_origin,
            //                 parent_size,
            //                 *size_of_children,
            //                 child_aligment,
            //                 true,
            //                 false,
            //             ),
            //             Axis3d::Vertical => align_child(
            //                 &mut child_origin,
            //                 parent_size,
            //                 *size_of_children,
            //                 child_aligment,
            //                 false,
            //                 true,
            //             ),
            //         };

            //         for child in &mut self.children {
            //             // Align each child along the cross axis
            //             match axis {
            //                 Axis3d::Horizontal => {
            //                     child_origin.set_y(padded_bounds.origin_y());
            //                     align_child(
            //                         &mut child_origin,
            //                         parent_size,
            //                         child.size(),
            //                         child_aligment,
            //                         false,
            //                         true,
            //                     );
            //                 }
            //                 Axis3d::Vertical => {
            //                     child_origin.set_x(padded_bounds.origin_x());
            //                     align_child(
            //                         &mut child_origin,
            //                         parent_size,
            //                         child.size(),
            //                         child_aligment,
            //                         true,
            //                         false,
            //                     );
            //                 }
            //             }

            //             child.paint(scene, child_origin, visible_bounds, view, cx);

            //             // Advance along the cross axis by the size of this child
            //             match axis {
            //                 Axis3d::Horizontal => {
            //                     child_origin.set_x(child_origin.x() + child.size().x())
            //                 }
            //                 Axis3d::Vertical => {
            //                     child_origin.set_y(child_origin.x() + child.size().y())
            //                 }
            //             }
            //         }
            //     }
            //     Orientation::Stacked => {
            //         for child in &mut self.children {
            //             let mut child_origin = padded_bounds.origin();
            //             align_child(
            //                 &mut child_origin,
            //                 parent_size,
            //                 child.size(),
            //                 child_aligment,
            //                 true,
            //                 true,
            //             );

            //             scene.paint_layer(None, |scene| {
            //                 child.paint(scene, child_origin, visible_bounds, view, cx);
            //             });
            //         }
            //     }
            // }
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.children
            .iter()
            .find_map(|child| child.rect_for_text_range(range_utf16.clone(), view, cx))
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Value {
        json!({
            "type": "Cell",
            "bounds": bounds.to_json(),
            "children": self.children.iter().map(|child| child.debug(view, cx)).collect::<Vec<Value>>()
        })
    }

    fn metadata(&self) -> Option<&dyn Any> {
        Some(&self.style)
    }
}

fn align_child(
    child_origin: &mut Vector2F,
    parent_size: Vector2F,
    child_size: Vector2F,
    alignment: Vector2F,
    horizontal: bool,
    vertical: bool,
) {
    let parent_center = parent_size / 2.;
    let parent_target = parent_center + parent_center * alignment;
    let child_center = child_size / 2.;
    let child_target = child_center + child_center * alignment;

    if horizontal {
        child_origin.set_x(child_origin.x() + parent_target.x() - child_target.x())
    }
    if vertical {
        child_origin.set_y(child_origin.y() + parent_target.y() - child_target.y());
    }
}

struct Interactive<Style> {
    default: Style,
    hovered: Style,
    active: Style,
    disabled: Style,
}

#[derive(Clone, Default)]
pub struct AtomStyle {
    axis: Axis3d,
    wrap: bool,
    align: Vector2F,
    overflow_x: Overflow,
    overflow_y: Overflow,
    gap_x: Gap,
    gap_y: Gap,

    width: Length,
    height: Length,
    margin: Edges<f32>,
    padding: Edges<f32>,

    text_color: Option<Color>,
    font_size: Option<f32>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,

    opacity: f32,
    fill: Fill,
    border: Border,
    corner_radius: f32,
    shadows: Vec<Shadow>,
}

impl AtomStyle {
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn fill(mut self, fill: impl Into<Fill>) -> Self {
        self.fill = fill.into();
        self
    }
}

#[derive(Clone, Default)]
struct Edges<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
}

#[derive(Clone, Default)]
struct CornerRadii {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}

#[derive(Clone)]
enum Fill {
    Color(Color),
}

impl From<Color> for Fill {
    fn from(value: Color) -> Self {
        Fill::Color(value)
    }
}

impl Default for Fill {
    fn default() -> Self {
        Fill::Color(Color::default())
    }
}

#[derive(Clone, Default)]
struct Border {
    color: Color,
    width: f32,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Border {
    fn is_visible(&self) -> bool {
        self.width > 0.
            && !self.color.is_fully_transparent()
            && (self.top || self.bottom || self.left || self.right)
    }
}

#[derive(Clone, Copy)]
enum Length {
    Fixed(f32),
    Auto { flex: f32, min: f32, max: f32 },
}

impl Default for Length {
    fn default() -> Self {
        Length::Auto {
            flex: 1.,
            min: 0.,
            max: f32::INFINITY,
        }
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Length::Fixed(value)
    }
}

impl Length {
    pub fn max(&self) -> f32 {
        match self {
            Length::Fixed(value) => *value,
            Length::Auto { max, .. } => *max,
        }
    }
}

#[derive(Clone, Copy, Default)]
enum Axis3d {
    X,
    #[default]
    Y,
    Z,
}

impl Axis3d {
    fn to_2d(self) -> Option<Axis2d> {
        match self {
            Axis3d::X => Some(Axis2d::X),
            Axis3d::Y => Some(Axis2d::Y),
            Axis3d::Z => None,
        }
    }
}

#[derive(Clone, Copy, Default)]
enum Axis2d {
    X,
    #[default]
    Y,
}

#[derive(Clone, Copy, Default)]
enum Overflow {
    #[default]
    Visible,
    Hidden,
    Auto,
}

#[derive(Clone, Copy)]
enum Gap {
    Fixed(f32),
    Around,
    Between,
    Even,
}

impl Default for Gap {
    fn default() -> Self {
        Gap::Fixed(0.)
    }
}

#[derive(Clone, Copy, Default)]
struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: Color,
}

#[derive(Clone, Copy, Default)]
enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, Default)]
enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    #[default]
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}
