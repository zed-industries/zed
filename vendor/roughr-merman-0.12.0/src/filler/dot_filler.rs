use std::borrow::BorrowMut;
use std::marker::PhantomData;

use euclid::default::Point2D;
use euclid::Trig;
use num_traits::{Float, FromPrimitive};

use super::scan_line_hachure::polygon_hachure_lines;
use super::traits::PatternFiller;
use crate::core::{_c, _cc, OpSet, Options};
use crate::geometry::Line;
use crate::renderer::ellipse;

pub struct DotFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for DotFiller<F>
where
    F: Float + Trig + FromPrimitive,
    P: BorrowMut<Vec<Vec<Point2D<F>>>>,
{
    fn fill_polygons(&self, mut polygon_list: P, o: &mut Options) -> crate::core::OpSet<F> {
        o.set_hachure_angle(Some(0.0));
        let lines = polygon_hachure_lines(polygon_list.borrow_mut().as_mut_slice(), o);
        let ops = DotFiller::dots_on_line(lines, o);
        OpSet {
            op_set_type: crate::core::OpSetType::FillSketch,
            ops,
            size: None,
            path: None,
        }
    }
}
impl<F: Float + Trig + FromPrimitive> DotFiller<F> {
    pub fn new() -> Self {
        DotFiller {
            _phantom: PhantomData,
        }
    }

    fn dots_on_line(lines: Vec<Line<F>>, o: &mut Options) -> Vec<crate::core::Op<F>> {
        let mut ops = vec![];
        let mut gap = o.hachure_gap.map(_c::<F>).unwrap_or_else(|| _c::<F>(-1.0));
        if gap < F::zero() {
            gap = o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c::<F>(1.0)) * _c::<F>(4.0);
        }
        gap = gap.max(_c::<F>(0.1));
        let mut fweight = o.fill_weight.map(_c::<F>).unwrap_or_else(|| _c::<F>(-1.0));
        if fweight < F::zero() {
            fweight = o.stroke_width.map(_c::<F>).unwrap_or_else(|| _c::<F>(1.0)) / _c::<F>(2.0);
        }

        let ro = gap / _c::<F>(4.0);
        for line in lines.iter() {
            let length = line.length();
            let dl = length / gap;
            let count = dl.ceil() - F::one();
            if count < F::zero() {
                continue;
            }
            let offset = length - (count * gap);
            let x = ((line.start_point.x + line.end_point.x) / _c::<F>(2.0)) - (gap / _c::<F>(4.0));
            let min_y = F::min(line.start_point.y, line.end_point.y);
            for i in 0..count.to_u64().unwrap() {
                let y = min_y + offset + (F::from(i).unwrap() * gap);
                let cx = (x - ro) + _cc::<F>(o.random()) * _c::<F>(2.0) * ro;
                let cy = (y - ro) + _cc::<F>(o.random()) * _c::<F>(2.0) * ro;
                let ellipse_ops = ellipse(cx, cy, fweight, fweight, o);
                ops.extend(ellipse_ops.ops);
            }
        }

        ops
    }
}

impl<F: Float + Trig + FromPrimitive> Default for DotFiller<F> {
    fn default() -> Self {
        Self::new()
    }
}
