//! Path data.

use super::command::{Command, PointsCommands, Verb};
use super::geometry::{Point, Transform};
use super::path_builder::PathBuilder;
use super::segment::segments;
use super::svg_parser::SvgCommands;

#[cfg(feature = "eval")]
use super::stroke::stroke_into;

#[cfg(feature = "eval")]
use super::style::*;

#[cfg(feature = "eval")]
use super::geometry::{Bounds, BoundsBuilder};

#[cfg(feature = "eval")]
use super::path_builder::TransformSink;

use crate::lib::Vec;

/// Trait for types that represent path data.
///
/// A primary design goal for this crate is to be agnostic with regard to
/// storage of path data. This trait provides the abstraction to make that
/// possible.
///
/// All path data is consumed internally as an iterator over path
/// [commands](Command) and as such, this trait is similar to
/// the `IntoIterator` trait, but restricted to iterators of commands and
/// without consuming itself.
///
/// Implementations of this trait are provided for SVG path data (in the form
/// of strings), slices/vectors of commands, and the common point and
/// verb list structure (as the tuple `(&[Point], &[Verb])`).
///
/// As such, these paths are all equivalent:
///
/// ```rust
/// use zeno::{Command, PathData, Point, Verb};
///
/// // SVG path data
/// let path1 = "M1,2 L3,4";
///
/// // Slice of commands
/// let path2 = &[
///     Command::MoveTo(Point::new(1.0, 2.0)),
///     Command::LineTo(Point::new(3.0, 4.0)),
/// ][..];
///
/// // Tuple of slices to points and verbs
/// let path3 = (
///     &[Point::new(1.0, 2.0), Point::new(3.0, 4.0)][..],
///     &[Verb::MoveTo, Verb::LineTo][..],
/// );
///
/// assert!(path1.commands().eq(path2.commands()));
/// assert!(path2.commands().eq(path3.commands()));
/// ```
///
/// Implementing `PathData` is similar to implementing `IntoIterator`:
///
/// ```rust
/// use zeno::{Command, PathData};
///
/// pub struct MyPath {
///     data: Vec<Command>
/// }
///
/// impl<'a> PathData for &'a MyPath {
///     // Copied here because PathData expects Commands by value
///     type Commands = std::iter::Copied<std::slice::Iter<'a, Command>>;
///
///     fn commands(&self) -> Self::Commands {
///         self.data.iter().copied()
///     }
/// }
/// ```
///
/// The provided `copy_into()` method evaluates the command iterator and
/// submits the commands to a sink. You should also implement this if you
/// have a more direct method of dispatching to a sink as rasterizer
/// performance can be sensitive to latencies here.
pub trait PathData {
    /// Command iterator.
    type Commands: Iterator<Item = Command> + Clone;

    /// Returns an iterator over the commands described by the path data.
    fn commands(&self) -> Self::Commands;

    /// Copies the path data into the specified sink.
    fn copy_to(&self, sink: &mut impl PathBuilder) {
        for cmd in self.commands() {
            use Command::*;
            match cmd {
                MoveTo(p) => sink.move_to(p),
                LineTo(p) => sink.line_to(p),
                QuadTo(c, p) => sink.quad_to(c, p),
                CurveTo(c1, c2, p) => sink.curve_to(c1, c2, p),
                Close => sink.close(),
            };
        }
    }
}

/// Computes the total length of the path.
pub fn length(data: impl PathData, transform: Option<Transform>) -> f32 {
    let data = data.commands();
    let mut length = 0.;
    if let Some(transform) = transform {
        for s in segments(data.map(|cmd| cmd.transform(&transform)), false) {
            length += s.length();
        }
    } else {
        for s in segments(data, false) {
            length += s.length();
        }
    }
    length
}

/// Computes the bounding box of the path.
#[cfg(feature = "eval")]
pub fn bounds<'a>(
    data: impl PathData,
    style: impl Into<Style<'a>>,
    transform: Option<Transform>,
) -> Bounds {
    let style = style.into();
    let mut bounds = BoundsBuilder::new();
    apply(data, style, transform, &mut bounds);
    bounds.build()
}

/// Applies the style and transform to the path and emits the result to the
/// specified sink.
#[cfg(feature = "eval")]
pub fn apply<'a>(
    data: impl PathData,
    style: impl Into<Style<'a>>,
    transform: Option<Transform>,
    sink: &mut impl PathBuilder,
) -> Fill {
    let style = style.into();
    match style {
        Style::Fill(fill) => {
            if let Some(transform) = transform {
                let mut transform_sink = TransformSink { sink, transform };
                data.copy_to(&mut transform_sink);
                fill
            } else {
                data.copy_to(sink);
                fill
            }
        }
        Style::Stroke(stroke) => {
            if let Some(transform) = transform {
                if stroke.scale {
                    let mut transform_sink = TransformSink { sink, transform };
                    stroke_into(data.commands(), &stroke, &mut transform_sink);
                } else {
                    stroke_into(
                        data.commands().map(|cmd| cmd.transform(&transform)),
                        &stroke,
                        sink,
                    );
                }
            } else {
                stroke_into(data.commands(), &stroke, sink);
            }
            Fill::NonZero
        }
    }
}

impl<T> PathData for &'_ T
where
    T: PathData,
{
    type Commands = T::Commands;

    fn commands(&self) -> Self::Commands {
        T::commands(*self)
    }

    #[inline(always)]
    fn copy_to(&self, sink: &mut impl PathBuilder) {
        T::copy_to(*self, sink)
    }
}

impl<'a> PathData for &'a str {
    type Commands = SvgCommands<'a>;

    fn commands(&self) -> Self::Commands {
        SvgCommands::new(self)
    }
}

impl<'a> PathData for (&'a [Point], &'a [Verb]) {
    type Commands = PointsCommands<'a>;

    fn commands(&self) -> Self::Commands {
        PointsCommands::new(self.0, self.1)
    }

    #[inline(always)]
    fn copy_to(&self, sink: &mut impl PathBuilder) {
        self.commands().copy_to(sink);
    }
}

impl<'a> PathData for &'a [Command] {
    type Commands = core::iter::Copied<core::slice::Iter<'a, Command>>;

    fn commands(&self) -> Self::Commands {
        self.iter().copied()
    }
}

impl<'a> PathData for &'a Vec<Command> {
    type Commands = core::iter::Copied<core::slice::Iter<'a, Command>>;

    fn commands(&self) -> Self::Commands {
        self.iter().copied()
    }
}
