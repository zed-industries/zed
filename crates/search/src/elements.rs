use std::ops::Range;

use gpui::color::Color;
use gpui::geometry::rect::RectF;
use gpui::geometry::vector::IntoVector2F;
use gpui::json::{self, ToJson};
use gpui::{scene::Path, LayoutContext};
use gpui::{Element, SceneBuilder, View, ViewContext};

type CreatePath = fn(RectF, Color) -> Path;
type AdjustBorder = fn(RectF, f32) -> RectF;
type BorderThickness = f32;

pub(crate) struct ButtonSide {
    color: Color,
    factory: CreatePath,
    /// After the outline is drawn with border color,
    /// the drawing bounds have to be adjusted by different factors in different dimensions.
    border_adjustment: AdjustBorder,
    border: Option<(BorderThickness, Color)>,
}

impl ButtonSide {
    fn new(color: Color, factory: CreatePath, border_adjustment: AdjustBorder) -> Self {
        Self {
            color,
            factory,
            border_adjustment,
            border: None,
        }
    }
    pub fn with_border(mut self, width: f32, color: Color) -> Self {
        self.border = Some((width, color));
        self
    }
    pub fn left(color: Color) -> Self {
        Self::new(color, left_button_side, left_button_border_adjust)
    }
    pub fn right(color: Color) -> Self {
        Self::new(color, right_button_side, right_button_border_adjust)
    }
}

fn left_button_border_adjust(bounds: RectF, width: f32) -> RectF {
    let width = width.into_vector_2f();
    let mut lower_right = bounds.clone().lower_right();
    lower_right.set_x(lower_right.x() + width.x());
    RectF::from_points(bounds.origin() + width, lower_right)
}
fn right_button_border_adjust(bounds: RectF, width: f32) -> RectF {
    let width = width.into_vector_2f();
    let mut origin = bounds.clone().origin();
    origin.set_x(origin.x() - width.x());
    RectF::from_points(origin, bounds.lower_right() - width)
}
fn left_button_side(bounds: RectF, color: Color) -> Path {
    use gpui::geometry::PathBuilder;
    let mut path = PathBuilder::new();
    path.reset(bounds.lower_right());
    path.line_to(bounds.upper_right());
    let mut middle_point = bounds.origin();
    let distance_to_line = (middle_point.y() - bounds.lower_left().y()) / 4.;
    middle_point.set_y(middle_point.y() - distance_to_line);
    path.curve_to(middle_point, bounds.origin());
    let mut target = bounds.lower_left();
    target.set_y(target.y() + distance_to_line);
    path.line_to(target);
    path.curve_to(bounds.lower_right(), bounds.lower_left());
    path.build(color, None)
}

fn right_button_side(bounds: RectF, color: Color) -> Path {
    use gpui::geometry::PathBuilder;
    let mut path = PathBuilder::new();
    path.reset(bounds.lower_left());
    path.line_to(bounds.origin());
    let mut middle_point = bounds.upper_right();
    let distance_to_line = (middle_point.y() - bounds.lower_right().y()) / 4.;
    middle_point.set_y(middle_point.y() - distance_to_line);
    path.curve_to(middle_point, bounds.upper_right());
    let mut target = bounds.lower_right();
    target.set_y(target.y() + distance_to_line);
    path.line_to(target);
    path.curve_to(bounds.lower_left(), bounds.lower_right());
    path.build(color, None)
}

impl<V: View> Element<V> for ButtonSide {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut V,
        _: &mut LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut V,
        _: &mut ViewContext<V>,
    ) -> Self::PaintState {
        let mut bounds = bounds;
        if let Some((border_width, border_color)) = self.border.as_ref() {
            scene.push_path((self.factory)(bounds, border_color.clone()));
            bounds = (self.border_adjustment)(bounds, *border_width);
        };
        scene.push_path((self.factory)(bounds, self.color));
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
    ) -> gpui::json::Value {
        json::json!({
            "type": "ButtonSide",
            "bounds": bounds.to_json(),
            "color": self.color.to_json(),
        })
    }
}
