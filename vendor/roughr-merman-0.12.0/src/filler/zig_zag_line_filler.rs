use std::borrow::BorrowMut;
use std::marker::PhantomData;

use euclid::default::Point2D;
use euclid::{point2, Trig};
use num_traits::{Float, FloatConst, FromPrimitive};

use super::scan_line_hachure::polygon_hachure_lines;
use super::traits::PatternFiller;
use crate::core::{_c, Op, OpSet, OpSetType, Options};
use crate::geometry::Line;
use crate::renderer::_double_line;

pub struct ZigZagLineFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for ZigZagLineFiller<F>
where
    F: Float + Trig + FromPrimitive,
    P: BorrowMut<Vec<Vec<Point2D<F>>>>,
{
    fn fill_polygons(&self, mut polygon_list: P, o: &mut Options) -> crate::core::OpSet<F> {
        let mut gap = o.hachure_gap.map(_c::<F>).unwrap_or_else(|| _c::<F>(-1.0));
        if gap < F::zero() {
            gap = o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c::<F>(1.0)) * _c::<F>(4.0);
        }
        gap = gap.max(_c::<F>(0.1));

        let mut zig_zag_offset = o
            .zigzag_offset
            .map(_c::<F>)
            .unwrap_or_else(|| _c::<F>(-1.0));
        if zig_zag_offset < F::zero() {
            zig_zag_offset = gap;
        }
        o.set_hachure_gap(Some((gap + zig_zag_offset).to_f32().unwrap()));
        let lines = polygon_hachure_lines(polygon_list.borrow_mut().as_mut_slice(), o);
        OpSet {
            op_set_type: OpSetType::FillSketch,
            ops: ZigZagLineFiller::zig_zag_lines(&lines, zig_zag_offset, o),
            size: None,
            path: None,
        }
    }
}

impl<F: Float + Trig + FromPrimitive> ZigZagLineFiller<F> {
    pub fn new() -> Self {
        ZigZagLineFiller {
            _phantom: PhantomData,
        }
    }

    fn zig_zag_lines(lines: &[Line<F>], zig_zag_offset: F, o: &mut Options) -> Vec<Op<F>> {
        let mut ops = vec![];
        for line in lines.iter() {
            let length = line.length();
            let count = length / (_c::<F>(2.0) * zig_zag_offset);
            let mut p1 = line.start_point;
            let mut p2 = line.end_point;
            if p1.x > p2.x {
                p1 = line.end_point;
                p2 = line.start_point;
            }

            let alpha = ((p2.y - p1.y) / (p2.x - p1.x)).atan();

            for i in 0..(count.to_isize().unwrap()) {
                let lstart = _c::<F>(i as f32) * _c::<F>(2.0) * zig_zag_offset;
                let lend = _c::<F>((i + 1) as f32) * _c::<F>(2.0) * zig_zag_offset;
                let dz = (zig_zag_offset.powi(2) * _c::<F>(2.0)).sqrt();
                let start: Point2D<F> = point2(
                    p1.x + lstart * num_traits::Float::cos(alpha),
                    p1.y + lstart * num_traits::Float::sin(alpha),
                );
                let end: Point2D<F> = point2(
                    p1.x + lend * num_traits::Float::cos(alpha),
                    p1.y + lend * num_traits::Float::sin(alpha),
                );
                let middle: Point2D<F> = point2(
                    start.x + dz * num_traits::Float::cos(alpha + _c::<F>(f32::PI() / 4.0)),
                    start.y + dz * num_traits::Float::sin(alpha + _c::<F>(f32::PI() / 4.0)),
                );
                ops.extend(_double_line(start.x, start.y, middle.x, middle.y, o, false));

                ops.extend(_double_line(middle.x, middle.y, end.x, end.y, o, false));
            }
        }
        ops
    }
}

impl<F: Float + Trig + FromPrimitive> Default for ZigZagLineFiller<F> {
    fn default() -> Self {
        Self::new()
    }
}
