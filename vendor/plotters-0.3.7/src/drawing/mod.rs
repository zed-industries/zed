/*!
The drawing utils for Plotters. In Plotters, we have two set of drawing APIs: low-level API and
high-level API.

The low-level drawing abstraction, the module defines the `DrawingBackend` trait from the `plotters-backend` create.
It exposes a set of functions which allows basic shape, such as pixels, lines, rectangles, circles, to be drawn on the screen.
The low-level API uses the pixel based coordinate.

The high-level API is built on the top of high-level API. The `DrawingArea` type exposes the high-level drawing API to the remaining part
of Plotters. The basic drawing blocks are composable elements, which can be defined in logic coordinate. To learn more details
about the [coordinate abstraction](../coord/index.html) and [element system](../element/index.html).
*/
mod area;
mod backend_impl;

pub use area::{DrawingArea, DrawingAreaErrorKind, IntoDrawingArea, Rect};

pub use backend_impl::*;
