use lyon_path::{
    geom::{euclid, Angle, Vector},
    traits::PathBuilder,
    ArcFlags, Attributes, Polygon, Winding,
};

pub type Point = euclid::default::Point2D<f32>;

/// Adds a sub-path from a polygon but rounds the corners.
///
/// There must be no sub-path in progress when this method is called.
/// No sub-path is in progress after the method is called.
pub fn add_rounded_polygon<B: PathBuilder>(
    builder: &mut B,
    polygon: Polygon<Point>,
    radius: f32,
    attributes: Attributes,
) {
    if polygon.points.len() < 2 {
        return;
    }

    //p points are original polygon points
    //q points are the actual points we will draw lines and arcs between
    let clamped_radius = clamp_radius(
        radius,
        polygon.points[polygon.points.len() - 1],
        polygon.points[0],
        polygon.points[1],
    );
    let q_first = get_point_between(polygon.points[0], polygon.points[1], clamped_radius);

    //We begin on the line just after the first point
    builder.begin(q_first, attributes);

    for index in 0..polygon.points.len() {
        let p_current = polygon.points[index];
        let p_next = polygon.points[(index + 1) % polygon.points.len()];
        let p_after_next = polygon.points[(index + 2) % polygon.points.len()];

        let clamped_radius = clamp_radius(radius, p_current, p_next, p_after_next);

        //q1 is the second point on the line between p_current and p_next
        let q1 = get_point_between(p_next, p_current, clamped_radius);
        //q2 is the first point on the line between p_next and p_after_next
        let q2 = get_point_between(p_next, p_after_next, clamped_radius);

        builder.line_to(q1, attributes);
        let turn_winding = get_winding(p_current, p_next, p_after_next);

        //Draw the arc near p_next
        arc(
            builder,
            Vector::new(clamped_radius, clamped_radius),
            Angle { radians: 0.0 },
            ArcFlags {
                large_arc: false,
                sweep: turn_winding == Winding::Negative,
            },
            q1,
            q2,
            attributes,
        );
    }

    builder.end(polygon.closed);
}

fn clamp_radius(radius: f32, p_previous: Point, p_current: Point, p_next: Point) -> f32 {
    let shorter_edge = ((p_current - p_next).length()).min((p_previous - p_current).length());

    radius.min(shorter_edge * 0.5)
}

fn get_point_between(p1: Point, p2: Point, radius: f32) -> Point {
    let dist = p1.distance_to(p2);
    let ratio = radius / dist;

    p1.lerp(p2, ratio)
}

fn get_winding(p0: Point, p1: Point, p2: Point) -> Winding {
    let cross = (p2 - p0).cross(p1 - p0);
    if cross.is_sign_positive() {
        Winding::Positive
    } else {
        Winding::Negative
    }
}

fn arc<B: PathBuilder>(
    builder: &mut B,
    radii: Vector<f32>,
    x_rotation: Angle<f32>,
    flags: ArcFlags,
    from: Point,
    to: Point,
    attributes: Attributes,
) {
    let svg_arc = lyon_path::geom::SvgArc {
        from,
        to,
        radii,
        x_rotation,
        flags,
    };

    if svg_arc.is_straight_line() {
        builder.line_to(to, attributes);
    } else {
        let geom_arc = svg_arc.to_arc();
        geom_arc.for_each_quadratic_bezier(&mut |curve| {
            builder.quadratic_bezier_to(curve.ctrl, curve.to, attributes);
        });
    }
}

#[test]
fn rounded_polygon() {
    use crate::geom::point;
    use crate::rounded_polygon::*;
    use alloc::vec::Vec;
    use euclid::approxeq::ApproxEq;

    type Point = euclid::Point2D<f32, euclid::UnknownUnit>;
    type Event = path::Event<Point, Point>;
    let arrow_points = [
        point(-1.0, -0.3),
        point(0.0, -0.3),
        point(0.0, -1.0),
        point(1.5, 0.0),
        point(0.0, 1.0),
        point(0.0, 0.3),
        point(-1.0, 0.3),
    ];

    let arrow_polygon = Polygon {
        points: &arrow_points,
        closed: true,
    };

    let mut builder = lyon_path::Path::builder();
    add_rounded_polygon(&mut builder, arrow_polygon, 0.2, lyon_path::NO_ATTRIBUTES);
    let arrow_path = builder.build();

    //check that we have the right ordering of event types
    let actual_events: alloc::vec::Vec<_> = arrow_path.into_iter().collect();

    let actual_event_types = actual_events
        .iter()
        .map(|x| match x {
            Event::Begin { at: _ } => "b",
            Event::Line { from: _, to: _ } => "l",
            Event::Quadratic {
                from: _,
                ctrl: _,
                to: _,
            } => "q",
            Event::Cubic {
                from: _,
                ctrl1: _,
                ctrl2: _,
                to: _,
            } => "c",
            Event::End {
                last: _,
                first: _,
                close: _,
            } => "e",
        })
        .collect::<alloc::vec::Vec<_>>()
        .concat();

    assert_eq!(actual_event_types, "blqqlqqlqqlqqlqqlqqlqqe");

    let expected_lines = std::vec![
        (point(-0.8, -0.3), point(-0.2, -0.3)),
        (point(0.0, -0.5), point(0.0, -0.8)),
        (point(0.166, -0.889), point(1.333, -0.111)),
        (point(1.334, 0.111), point(0.166, 0.889)),
        (point(0.0, 0.8), point(0.0, 0.5)),
        (point(-0.2, 0.3), point(-0.8, 0.3)),
        (point(-1.0, 0.1), point(-1.0, -0.1))
    ];

    //Check that the lines are approximately correct
    let actual_lines: Vec<_> = arrow_path
        .into_iter()
        .filter_map(|event| match event {
            Event::Line { from, to } => Some((from, to)),
            _ => None,
        })
        .collect();

    for (actual, expected) in actual_lines.into_iter().zip(expected_lines.into_iter()) {
        for (actual_point, expected_point) in [(actual.0, expected.0), (actual.1, expected.1)] {
            assert!(actual_point.approx_eq_eps(&expected_point, &Point::new(0.01, 0.01)))
        }
    }

    //Check that each event goes from the end of the previous event

    let mut previous = actual_events[0].to();

    for e in actual_events {
        e.from().approx_eq(&previous);
        previous = e.to();
    }
}
