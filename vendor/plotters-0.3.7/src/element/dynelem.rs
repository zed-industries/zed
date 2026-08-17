use super::{Drawable, PointCollection};
use plotters_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

use std::borrow::Borrow;

trait DynDrawable<DB: DrawingBackend> {
    fn draw_dyn(
        &self,
        points: &mut dyn Iterator<Item = BackendCoord>,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>>;
}

impl<DB: DrawingBackend, T: Drawable<DB>> DynDrawable<DB> for T {
    fn draw_dyn(
        &self,
        points: &mut dyn Iterator<Item = BackendCoord>,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        T::draw(self, points, backend, parent_dim)
    }
}

/// The container for a dynamically dispatched element
pub struct DynElement<'a, DB, Coord>
where
    DB: DrawingBackend,
    Coord: Clone,
{
    points: Vec<Coord>,
    drawable: Box<dyn DynDrawable<DB> + 'a>,
}

impl<'a, 'b: 'a, DB: DrawingBackend, Coord: Clone> PointCollection<'a, Coord>
    for &'a DynElement<'b, DB, Coord>
{
    type Point = &'a Coord;
    type IntoIter = &'a Vec<Coord>;
    fn point_iter(self) -> Self::IntoIter {
        &self.points
    }
}

impl<'a, DB: DrawingBackend, Coord: Clone> Drawable<DB> for DynElement<'a, DB, Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut pos: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        self.drawable.draw_dyn(&mut pos, backend, parent_dim)
    }
}

/// The trait that makes the conversion from the statically dispatched element
/// to the dynamically dispatched element
pub trait IntoDynElement<'a, DB: DrawingBackend, Coord: Clone>
where
    Self: 'a,
{
    /// Make the conversion
    fn into_dyn(self) -> DynElement<'a, DB, Coord>;
}

impl<'b, T, DB, Coord> IntoDynElement<'b, DB, Coord> for T
where
    T: Drawable<DB> + 'b,
    for<'a> &'a T: PointCollection<'a, Coord>,
    Coord: Clone,
    DB: DrawingBackend,
{
    fn into_dyn(self) -> DynElement<'b, DB, Coord> {
        DynElement {
            points: self
                .point_iter()
                .into_iter()
                .map(|x| x.borrow().clone())
                .collect(),
            drawable: Box::new(self),
        }
    }
}
