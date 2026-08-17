use std::borrow::BorrowMut;
use std::marker::PhantomData;

use euclid::default::Point2D;
use euclid::{point2, Trig};
use num_traits::{Float, FromPrimitive};

use super::scan_line_hachure::polygon_hachure_lines;
use super::traits::PatternFiller;
use crate::core::{_c, OpSet, Options};
use crate::geometry::Line;
use crate::renderer::_double_line;

pub struct DashedFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for DashedFiller<F>
where
    F: Float + Trig + FromPrimitive,
    P: BorrowMut<Vec<Vec<Point2D<F>>>>,
{
    fn fill_polygons(&self, mut polygon_list: P, o: &mut Options) -> crate::core::OpSet<F> {
        let lines = polygon_hachure_lines(polygon_list.borrow_mut().as_mut_slice(), o);
        let ops = DashedFiller::dashed_line(lines, o);
        OpSet {
            op_set_type: crate::core::OpSetType::FillSketch,
            ops,
            size: None,
            path: None,
        }
    }
}
impl<F: Float + Trig + FromPrimitive> DashedFiller<F> {
    pub fn new() -> Self {
        DashedFiller {
            _phantom: PhantomData,
        }
    }

    fn dashed_line(lines: Vec<Line<F>>, o: &mut Options) -> Vec<crate::core::Op<F>> {
        let dash_offset: F = o.dash_offset.map(_c).unwrap_or_else(|| _c(-1.0));
        let offset = if dash_offset < _c(0.0) {
            let hachure_gap: F = o.hachure_gap.map(_c).unwrap_or_else(|| _c(-1.0));
            if hachure_gap < _c(0.0) {
                o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c(1.0)) * _c(4.0)
            } else {
                hachure_gap
            }
        } else {
            dash_offset
        };
        let dash_gap = o.dash_gap.map(_c).unwrap_or_else(|| _c(-1.0));
        let gap: F = if dash_gap < _c(0.0) {
            let hachure_gap = o.hachure_gap.map(_c).unwrap_or_else(|| _c(-1.0));
            if hachure_gap < _c(0.0) {
                o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c(1.0)) * _c(4.0)
            } else {
                hachure_gap
            }
        } else {
            dash_gap
        };

        let mut ops = vec![];

        for line in lines.iter() {
            let length = line.length();
            let count = (length / (offset + gap)).floor();
            let start_offset = (length + gap - (count * (offset + gap))) / _c(2.0);
            let mut p1 = line.start_point;
            let mut p2 = line.end_point;
            if p1.x > p2.x {
                p1 = line.end_point;
                p2 = line.start_point;
            }
            let alpha = ((p2.y - p1.y) / (p2.x - p1.x)).atan();
            for i in 0..count.to_u32().unwrap() {
                let lstart = F::from(i).unwrap() * (offset + gap);
                let lend = lstart + offset;
                let start: Point2D<F> = point2(
                    p1.x + (lstart * num_traits::Float::cos(alpha))
                        + (start_offset * num_traits::Float::cos(alpha)),
                    p1.y + lstart * num_traits::Float::sin(alpha)
                        + (start_offset * num_traits::Float::sin(alpha)),
                );
                let end: Point2D<F> = point2(
                    p1.x + (lend * num_traits::Float::cos(alpha))
                        + (start_offset * num_traits::Float::cos(alpha)),
                    p1.y + (lend * num_traits::Float::sin(alpha))
                        + (start_offset * num_traits::Float::sin(alpha)),
                );
                let line_ops = _double_line(start.x, start.y, end.x, end.y, o, false);
                ops.extend(line_ops);
            }
        }

        ops
    }
}

impl<F: Float + Trig + FromPrimitive> Default for DashedFiller<F> {
    fn default() -> Self {
        Self::new()
    }
}
