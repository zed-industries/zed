/*!
*svgtypes* is a collection of parsers for [SVG](https://www.w3.org/TR/SVG2/) types.

## Supported SVG types

- [`<color>`](https://www.w3.org/TR/css-color-3/)
- [`<number>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumber)
- [`<length>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLength)
- [`<angle>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGAngle)
- [`<viewBox>`](https://www.w3.org/TR/SVG2/coords.html#ViewBoxAttribute)
- [`<path>`](https://www.w3.org/TR/SVG2/paths.html#PathData)
- [`<transform>`](https://www.w3.org/TR/SVG11/types.html#DataTypeTransformList)
- [`<list-of-numbers>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumberList)
- [`<list-of-lengths>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLengthList)
- [`<list-of-points>`](https://www.w3.org/TR/SVG11/shapes.html#PointsBNF)
- [`<filter-value-list>`](https://www.w3.org/TR/filter-effects-1/#typedef-filter-value-list)
- [`<paint>`](https://www.w3.org/TR/SVG2/painting.html#SpecifyingPaint)
- [`<preserveAspectRatio>`](https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute)
- [`<enable-background>`](https://www.w3.org/TR/SVG11/filters.html#EnableBackgroundProperty)
- [`<IRI>`](https://www.w3.org/TR/SVG11/types.html#DataTypeIRI)
- [`<FuncIRI>`](https://www.w3.org/TR/SVG11/types.html#DataTypeFuncIRI)
- [`paint-order`](https://www.w3.org/TR/SVG2/painting.html#PaintOrder)

## Features

- Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
- Implicit path commands will be automatically converted into explicit one.
- Some SVG2 data types support.
- Pretty fast.

## Limitations

- Accepts only [normalized](https://www.w3.org/TR/REC-xml/#AVNormalize) values,
  e.g. an input text should not contain `&#x20;` or `&data;`.
- All keywords must be lowercase.
  Case-insensitive parsing is supported only for colors (requires allocation for named colors).
- The `<color>` followed by the `<icccolor>` is not supported. As the `<icccolor>` itself.
- [System colors](https://www.w3.org/TR/css3-color/#css2-system), like `fill="AppWorkspace"`,
  are not supported. They were deprecated anyway.

## Safety

- The library should not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.

## Alternatives

None.
*/

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

macro_rules! matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => true,
            _ => false
        }
    }
}

mod angle;
mod aspect_ratio;
mod color;
#[rustfmt::skip] mod colors;
mod enable_background;
mod error;
mod filter_functions;
mod funciri;
mod length;
mod number;
mod paint;
mod paint_order;
mod path;
mod points;
mod stream;
mod transform;
mod viewbox;

use crate::stream::{ByteExt, Stream};

pub use crate::angle::*;
pub use crate::aspect_ratio::*;
pub use crate::color::*;
pub use crate::enable_background::*;
pub use crate::error::*;
pub use crate::filter_functions::*;
pub use crate::funciri::*;
pub use crate::length::*;
pub use crate::number::*;
pub use crate::paint::*;
pub use crate::paint_order::*;
pub use crate::path::*;
pub use crate::points::*;
pub use crate::transform::*;
pub use crate::viewbox::*;
