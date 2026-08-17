use crate::fill::{is_after, Side};
use crate::math::{point, Point};
use crate::{FillGeometryBuilder, VertexId};

use alloc::vec::Vec;

/// Helper class that generates a triangulation from a sequence of vertices describing a monotone
/// polygon (used internally by the `FillTessellator`).
pub(crate) struct BasicMonotoneTessellator {
    stack: Vec<MonotoneVertex>,
    previous: MonotoneVertex,
    triangles: Vec<(VertexId, VertexId, VertexId)>,
}

#[derive(Copy, Clone, Debug)]
struct MonotoneVertex {
    pos: Point,
    id: VertexId,
    side: Side,
}

impl BasicMonotoneTessellator {
    pub fn new() -> Self {
        BasicMonotoneTessellator {
            stack: Vec::new(),
            triangles: Vec::new(),
            // Some placeholder value that will be replaced right away.
            previous: MonotoneVertex {
                pos: Point::new(0.0, 0.0),
                id: VertexId(0),
                side: Side::Left,
            },
        }
    }

    pub fn begin(&mut self, pos: Point, id: VertexId) {
        debug_assert!(id != VertexId::INVALID);
        let first = MonotoneVertex {
            pos,
            id,
            side: Side::Left,
        };
        self.previous = first;

        self.triangles.clear();
        self.triangles.reserve(16);

        self.stack.clear();
        self.stack.reserve(16);
        self.stack.push(first);
    }

    #[inline]
    pub fn vertex(&mut self, pos: Point, id: VertexId, side: Side) {
        self.monotone_vertex(MonotoneVertex { pos, id, side });
    }

    fn monotone_vertex(&mut self, current: MonotoneVertex) {
        debug_assert!(current.id != VertexId::INVALID);
        // cf. test_fixed_to_f32_precision
        debug_assert!(current.pos.y >= self.previous.pos.y);
        debug_assert!(!self.stack.is_empty());

        let changed_side = current.side != self.previous.side;

        if changed_side {
            for i in 0..(self.stack.len() - 1) {
                let mut a = self.stack[i];
                let mut b = self.stack[i + 1];

                let winding = (a.pos - b.pos).cross(current.pos - b.pos) >= 0.0;

                if !winding {
                    core::mem::swap(&mut a, &mut b);
                }

                self.push_triangle(&a, &b, &current);
            }
            self.stack.clear();
            self.stack.push(self.previous);
        } else {
            let mut last_popped = self.stack.pop();
            while !self.stack.is_empty() {
                let mut a = last_popped.unwrap();
                let mut b = *self.stack.last().unwrap();

                if current.side.is_right() {
                    core::mem::swap(&mut a, &mut b);
                }

                let cross = (current.pos - b.pos).cross(a.pos - b.pos);
                if cross >= 0.0 {
                    self.push_triangle(&b, &a, &current);
                    last_popped = self.stack.pop();
                } else {
                    break;
                }
            }
            if let Some(item) = last_popped {
                self.stack.push(item);
            }
        }

        self.stack.push(current);
        self.previous = current;
    }

    pub fn end(&mut self, pos: Point, id: VertexId) {
        let side = self.previous.side.opposite();
        self.vertex(pos, id, side);
        self.stack.clear();
    }

    #[inline]
    fn push_triangle(&mut self, a: &MonotoneVertex, b: &MonotoneVertex, c: &MonotoneVertex) {
        //let threshold = -0.0625; // Floating point errors stroke again :(
        //debug_assert!((a.pos - b.pos).cross(c.pos - b.pos) >= threshold);

        self.push_triangle_ids(a.id, b.id, c.id);
    }

    fn push_triangle_ids(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        debug_assert!(a != b);
        debug_assert!(b != c);
        debug_assert!(a != c);
        debug_assert!(a != VertexId::INVALID);
        debug_assert!(b != VertexId::INVALID);
        debug_assert!(c != VertexId::INVALID);

        self.triangles.push((a, b, c));
    }

    pub fn flush(&mut self, output: &mut dyn FillGeometryBuilder) {
        for &(a, b, c) in &self.triangles {
            output.add_triangle(a, b, c);
        }
        self.triangles.clear();
    }
}

#[test]
fn test_monotone_tess() {
    std::println!(" ------------ ");
    {
        let mut tess = BasicMonotoneTessellator::new();
        tess.begin(point(0.0, 0.0), VertexId(0));
        tess.vertex(point(-1.0, 1.0), VertexId(1), Side::Left);
        tess.end(point(1.0, 2.0), VertexId(2));
        assert_eq!(tess.triangles.len(), 1);
    }
    std::println!(" ------------ ");
    {
        let mut tess = BasicMonotoneTessellator::new();
        tess.begin(point(0.0, 0.0), VertexId(0));
        tess.vertex(point(1.0, 1.0), VertexId(1), Side::Right);
        tess.vertex(point(-1.5, 2.0), VertexId(2), Side::Left);
        tess.vertex(point(-1.0, 3.0), VertexId(3), Side::Left);
        tess.vertex(point(1.0, 4.0), VertexId(4), Side::Right);
        tess.end(point(0.0, 5.0), VertexId(5));
        assert_eq!(tess.triangles.len(), 4);
    }
    std::println!(" ------------ ");
    {
        let mut tess = BasicMonotoneTessellator::new();
        tess.begin(point(0.0, 0.0), VertexId(0));
        tess.vertex(point(1.0, 1.0), VertexId(1), Side::Right);
        tess.vertex(point(3.0, 2.0), VertexId(2), Side::Right);
        tess.vertex(point(1.0, 3.0), VertexId(3), Side::Right);
        tess.vertex(point(1.0, 4.0), VertexId(4), Side::Right);
        tess.vertex(point(4.0, 5.0), VertexId(5), Side::Right);
        tess.end(point(0.0, 6.0), VertexId(6));
        assert_eq!(tess.triangles.len(), 5);
    }
    std::println!(" ------------ ");
    {
        let mut tess = BasicMonotoneTessellator::new();
        tess.begin(point(0.0, 0.0), VertexId(0));
        tess.vertex(point(-1.0, 1.0), VertexId(1), Side::Left);
        tess.vertex(point(-3.0, 2.0), VertexId(2), Side::Left);
        tess.vertex(point(-1.0, 3.0), VertexId(3), Side::Left);
        tess.vertex(point(-1.0, 4.0), VertexId(4), Side::Left);
        tess.vertex(point(-4.0, 5.0), VertexId(5), Side::Left);
        tess.end(point(0.0, 6.0), VertexId(6));
        assert_eq!(tess.triangles.len(), 5);
    }
    std::println!(" ------------ ");
}

struct SideEvents {
    // We decide whether we have to flush a convex vertex chain based on
    // whether the two sides are far apart. reference_point.x contains the
    // center-most x coordinate of the current chain of vertex. It is not
    // enough because a previous chain on one side can still interfere with
    // a chain of vertex on the opposite side that is still current, so we
    // also keep track of a conservative reference x value which is not not
    // relaxed until the opposite side has is flushed. See issue #623.
    reference_point: Point,
    conservative_reference_x: f32,
    // A convex chain of vertex events for this side that can be tessellated
    // without interference from the other side.
    events: Vec<VertexId>,
    prev: Point,
    last: MonotoneVertex,
}

impl SideEvents {
    #[inline]
    fn push(&mut self, vertex: MonotoneVertex) {
        self.events.push(vertex.id);
        self.prev = self.last.pos;
        self.last = vertex;
    }
}

pub(crate) struct AdvancedMonotoneTessellator {
    tess: BasicMonotoneTessellator,
    left: SideEvents,
    right: SideEvents,
    flushing: bool,
}

impl AdvancedMonotoneTessellator {
    pub fn new() -> Self {
        let zero = point(0.0, 0.0);
        let dummy_vtx = MonotoneVertex {
            pos: zero,
            id: VertexId(0),
            side: Side::Left,
        };
        AdvancedMonotoneTessellator {
            left: SideEvents {
                events: Vec::with_capacity(16),
                reference_point: zero,
                conservative_reference_x: 0.0,
                prev: zero,
                last: dummy_vtx,
            },
            right: SideEvents {
                events: Vec::with_capacity(16),
                reference_point: zero,
                conservative_reference_x: 0.0,
                prev: zero,
                last: dummy_vtx,
            },
            tess: BasicMonotoneTessellator::new(),
            flushing: false,
        }
    }

    pub fn begin(&mut self, pos: Point, id: VertexId) {
        self.tess.begin(pos, id);
        self.left.reference_point = pos;
        self.left.conservative_reference_x = pos.x;
        self.right.reference_point = pos;
        self.right.conservative_reference_x = pos.x;
        self.left.prev = pos;
        self.right.prev = pos;
        self.flushing = false;

        self.left.events.clear();
        self.right.events.clear();
        self.left.push(MonotoneVertex {
            pos,
            id,
            side: Side::Left,
        });
        self.right.push(MonotoneVertex {
            pos,
            id,
            side: Side::Right,
        });
    }

    pub fn vertex(&mut self, pos: Point, id: VertexId, side: Side) {
        match side {
            Side::Left => {
                self.left.reference_point.x = self.left.reference_point.x.max(pos.x);
                self.left.conservative_reference_x = self
                    .left
                    .conservative_reference_x
                    .max(self.left.reference_point.x);
            }
            Side::Right => {
                self.right.reference_point.x = self.right.reference_point.x.min(pos.x);
                self.right.conservative_reference_x = self
                    .right
                    .conservative_reference_x
                    .min(self.right.reference_point.x);
            }
        }

        let dx = self.right.conservative_reference_x - self.left.conservative_reference_x;

        let (side_ev, opposite_side_ev) = match side {
            Side::Left => (&mut self.left, &mut self.right),
            Side::Right => (&mut self.right, &mut self.left),
        };

        let dy = pos.y - side_ev.reference_point.y;
        let sides_are_close = dx < dy * 0.1;

        let len = side_ev.events.len();
        let outward_turn = if !sides_are_close && len >= 2 {
            let sign = match side {
                Side::Left => 1.0,
                Side::Right => -1.0,
            };
            let prev = side_ev.prev;
            let last = side_ev.last.pos;
            (prev - last).cross(pos - last) * sign < 0.0
        } else {
            false
        };

        if outward_turn || sides_are_close {
            // To ensure that we don't break the ordering of the vertices forwarded to
            // the inner tessellator we have to flush the opposite side as well
            let must_flush_opposite_side = is_after(side_ev.last.pos, opposite_side_ev.last.pos);

            if must_flush_opposite_side {
                if let Some(v) = flush_side(opposite_side_ev, side.opposite(), &mut self.tess) {
                    self.tess.monotone_vertex(v);
                    side_ev.conservative_reference_x = side_ev.reference_point.x;
                }
            }

            if let Some(v) = flush_side(side_ev, side, &mut self.tess) {
                self.tess.monotone_vertex(v);
                opposite_side_ev.conservative_reference_x = opposite_side_ev.reference_point.x;
            }
        }

        side_ev.push(MonotoneVertex { pos, id, side });
    }

    pub fn end(&mut self, pos: Point, id: VertexId) {
        let a = flush_side(&mut self.left, Side::Left, &mut self.tess);
        let b = flush_side(&mut self.right, Side::Right, &mut self.tess);
        match (a, b) {
            (Some(v), None) | (None, Some(v)) => {
                self.tess.monotone_vertex(v);
            }
            (Some(mut v1), Some(mut v2)) => {
                if is_after(v1.pos, v2.pos) {
                    core::mem::swap(&mut v1, &mut v2);
                }
                self.tess.monotone_vertex(v1);
                self.tess.monotone_vertex(v2);
            }
            (None, None) => {}
        }

        self.tess.end(pos, id);
    }

    pub fn flush(&mut self, output: &mut dyn FillGeometryBuilder) {
        self.tess.flush(output);
    }
}

#[inline(never)]
fn flush_side(
    side: &mut SideEvents,
    s: Side,
    tess: &mut BasicMonotoneTessellator,
) -> Option<MonotoneVertex> {
    let len = side.events.len();
    if len < 2 {
        return None;
    }

    let mut step = 1;
    while step * 2 < len {
        let mut last_index = 0;
        let imax = (len - 1) / (2 * step);
        for i in 0..imax {
            let mut a = i * 2 * step;
            let mut b = a + step;
            last_index = b + step;
            if s == Side::Right {
                core::mem::swap(&mut a, &mut b);
            }
            tess.push_triangle_ids(side.events[a], side.events[b], side.events[last_index]);
        }

        if last_index + step < len {
            let mut b = last_index;
            let mut c = last_index + step;
            if s == Side::Right {
                core::mem::swap(&mut b, &mut c);
            }

            tess.push_triangle_ids(side.events[0], side.events[b], side.events[c]);
        }

        step *= 2;
    }

    side.events.clear();
    side.push(side.last);
    side.reference_point = side.last.pos;

    Some(side.last)
}

pub(crate) type MonotoneTessellator = AdvancedMonotoneTessellator;
