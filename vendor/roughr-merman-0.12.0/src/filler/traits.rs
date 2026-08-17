use std::borrow::BorrowMut;

use euclid::default::Point2D;
use euclid::Trig;
use num_traits::{Float, FromPrimitive};

use crate::core::{OpSet, Options};

pub trait PatternFiller<F: Float + Trig + FromPrimitive, P: BorrowMut<Vec<Vec<Point2D<F>>>>> {
    fn fill_polygons(&self, polygon_list: P, o: &mut Options) -> OpSet<F>;
}
