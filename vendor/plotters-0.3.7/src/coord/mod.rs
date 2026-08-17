/*!

One of the key features of Plotters is flexible coordinate system abstraction and this module
provides all the abstraction used for the coordinate abstraction of Plotters.

Generally speaking, the coordinate system in Plotters is responsible for mapping logic data points into
pixel based backend coordinate. This task is abstracted by a simple trait called
[CoordTranslate](trait.CoordTranslate.html). Please note `CoordTranslate` trait doesn't assume any property
about the coordinate values, thus we are able to extend Plotters's coordinate system to other types of coorindate
easily.

Another important trait is [ReverseCoordTranslate](trait.ReverseCoordTranslate.html). This trait allows some coordinate
retrieve the logic value based on the pixel-based backend coordinate. This is particularly interesting for interactive plots.

Plotters contains a set of pre-defined coordinate specifications that fulfills the most common use. See documentation for
module [types](types/index.html) for details about the basic 1D types.

The coordinate system also can be tweaked by the coordinate combinators, such as logarithmic coordinate, nested coordinate, etc.
See documentation for module [combinators](combinators/index.html)  for details.

Currently we support the following 2D coordinate system:

- 2-dimensional Cartesian Coordinate: This is done by the combinator [Cartesian2d](cartesian/struct.Cartesian2d.html).

*/

use plotters_backend::BackendCoord;

pub mod ranged1d;

///  The coordinate combinators
///
/// Coordinate combinators are very important part of Plotters' coordinate system.
/// The combinator is more about the "combinator pattern", which takes one or more coordinate specification
/// and transform them into a new coordinate specification.
pub mod combinators {
    pub use super::ranged1d::combinators::*;
}

/// The primitive types supported by Plotters coordinate system
pub mod types {
    pub use super::ranged1d::types::*;
}

mod ranged2d;
/// Ranged coordinates in 3d.
pub mod ranged3d;

/// Groups Cartesian ranged coordinates in 2d and 3d.
pub mod cartesian {
    pub use super::ranged2d::cartesian::{Cartesian2d, MeshLine};
    pub use super::ranged3d::Cartesian3d;
}

mod translate;
pub use translate::{CoordTranslate, ReverseCoordTranslate};

/// The coordinate translation that only impose shift
#[derive(Debug, Clone)]
pub struct Shift(pub BackendCoord);

impl CoordTranslate for Shift {
    type From = BackendCoord;
    fn translate(&self, from: &Self::From) -> BackendCoord {
        (from.0 + (self.0).0, from.1 + (self.0).1)
    }
}

impl ReverseCoordTranslate for Shift {
    fn reverse_translate(&self, input: BackendCoord) -> Option<BackendCoord> {
        Some((input.0 - (self.0).0, input.1 - (self.0).1))
    }
}
