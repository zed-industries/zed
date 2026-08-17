use crate::event_queue::{EventQueue, INVALID_EVENT_ID};
use crate::math::*;
use crate::{
    FillGeometryBuilder, FillOptions, FillVertex, TessellationError, TessellationResult, VertexId,
};

use core::f32::consts::PI;

#[cfg(not(feature = "std"))]
use num_traits::Float;

pub fn fill_rectangle(rect: &Box2D, output: &mut dyn FillGeometryBuilder) -> TessellationResult {
    output.begin_geometry();

    let dummy_queue = EventQueue::new();

    let vertex = &mut |position| {
        output.add_fill_vertex(FillVertex {
            position,
            events: &dummy_queue,
            current_event: INVALID_EVENT_ID,
            attrib_store: None,
            attrib_buffer: &mut [],
        })
    };

    let a = vertex(rect.min)?;
    let b = vertex(bottom_left(rect))?;
    let c = vertex(bottom_right(rect))?;
    let d = vertex(top_right(rect))?;

    output.add_triangle(a, b, c);
    output.add_triangle(a, c, d);

    output.end_geometry();

    Ok(())
}

pub fn fill_circle(
    center: Point,
    radius: f32,
    options: &FillOptions,
    output: &mut dyn FillGeometryBuilder,
) -> TessellationResult {
    let radius = radius.abs();
    if radius == 0.0 {
        return Ok(());
    }

    output.begin_geometry();

    let up = vector(0.0, -1.0);
    let down = vector(0.0, 1.0);
    let left = vector(-1.0, 0.0);
    let right = vector(1.0, 0.0);

    let events = &EventQueue::new();
    let attrib_store = None;
    let current_event = INVALID_EVENT_ID;

    let v = [
        output.add_fill_vertex(FillVertex {
            position: center + (left * radius),
            events,
            current_event,
            attrib_store,
            attrib_buffer: &mut [],
        })?,
        output.add_fill_vertex(FillVertex {
            position: center + (up * radius),
            events,
            current_event,
            attrib_store,
            attrib_buffer: &mut [],
        })?,
        output.add_fill_vertex(FillVertex {
            position: center + (right * radius),
            events,
            current_event,
            attrib_store,
            attrib_buffer: &mut [],
        })?,
        output.add_fill_vertex(FillVertex {
            position: center + (down * radius),
            events,
            current_event,
            attrib_store,
            attrib_buffer: &mut [],
        })?,
    ];

    output.add_triangle(v[0], v[3], v[1]);
    output.add_triangle(v[1], v[3], v[2]);

    let angles = [
        (PI, 1.5 * PI),
        (1.5 * PI, 2.0 * PI),
        (0.0, PI * 0.5),
        (PI * 0.5, PI),
    ];

    let arc_len = 0.5 * PI * radius;
    let step = circle_flattening_step(radius, options.tolerance);
    let num_segments = (arc_len / step).ceil();
    let num_recursions = num_segments.log2() as u32;

    for i in 0..4 {
        fill_border_radius(
            center,
            angles[i],
            radius,
            v[i],
            v[(i + 1) % 4],
            num_recursions,
            events,
            output,
        )?;
    }

    output.end_geometry();

    Ok(())
}

fn bottom_left(rect: &Box2D) -> Point {
    point(rect.min.x, rect.max.y)
}

fn top_right(rect: &Box2D) -> Point {
    point(rect.max.x, rect.min.y)
}

fn bottom_right(rect: &Box2D) -> Point {
    rect.max
}

// Returns the maximum length of individual line segments when approximating a
// circle.
//
// From Pythagorean theorem:
// r² = (d/2)² + (r - t)²
// r² = d²/4 + r² + t² - 2 * e * r
// d² = 4 * (2 * t * r - t²)
// d = 2 * sqrt(2 * t * r - t²)
//
// With:
//  r: the radius
//  t: the tolerance threshold
//  d: the line segment length
pub(crate) fn circle_flattening_step(radius: f32, mut tolerance: f32) -> f32 {
    // Don't allow high tolerance values (compared to the radius) to avoid edge cases.
    tolerance = f32::min(tolerance, radius);
    2.0 * f32::sqrt(2.0 * tolerance * radius - tolerance * tolerance)
}

// recursively tessellate the rounded corners.
fn fill_border_radius(
    center: Point,
    angle: (f32, f32),
    radius: f32,
    va: VertexId,
    vb: VertexId,
    num_recursions: u32,
    dummy_queue: &EventQueue,
    output: &mut dyn FillGeometryBuilder,
) -> Result<(), TessellationError> {
    if num_recursions == 0 {
        return Ok(());
    }

    let mid_angle = (angle.0 + angle.1) * 0.5;

    let normal = vector(mid_angle.cos(), mid_angle.sin());
    let position = center + normal * radius;

    let vertex = output.add_fill_vertex(FillVertex {
        position,
        events: dummy_queue,
        current_event: INVALID_EVENT_ID,
        attrib_store: None,
        attrib_buffer: &mut [],
    })?;

    output.add_triangle(vb, vertex, va);

    fill_border_radius(
        center,
        (angle.0, mid_angle),
        radius,
        va,
        vertex,
        num_recursions - 1,
        dummy_queue,
        output,
    )?;
    fill_border_radius(
        center,
        (mid_angle, angle.1),
        radius,
        vertex,
        vb,
        num_recursions - 1,
        dummy_queue,
        output,
    )
}

#[test]
fn basic_shapes() {
    use crate::GeometryBuilderError;

    let mut tess = crate::FillTessellator::new();

    tess.tessellate_rectangle(
        &Box2D {
            min: point(0.0, 1.0),
            max: point(2.0, 4.0),
        },
        &FillOptions::DEFAULT,
        &mut Builder { next_vertex: 0 },
    )
    .unwrap();

    tess.tessellate_circle(
        point(1.0, 2.0),
        100.0,
        &FillOptions::DEFAULT,
        &mut Builder { next_vertex: 0 },
    )
    .unwrap();

    struct Builder {
        next_vertex: u32,
    }

    impl crate::GeometryBuilder for Builder {
        fn add_triangle(&mut self, _: VertexId, _: VertexId, _: VertexId) {}
    }

    impl crate::FillGeometryBuilder for Builder {
        fn add_fill_vertex(
            &mut self,
            vertex: FillVertex,
        ) -> Result<VertexId, GeometryBuilderError> {
            let _pos = vertex.position();
            assert!(vertex.sources().next().is_none());
            assert!(vertex.as_endpoint_id().is_none());

            let id = self.next_vertex;
            self.next_vertex += 1;

            Ok(VertexId(id))
        }
    }
}
