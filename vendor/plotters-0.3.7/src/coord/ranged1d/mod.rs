/*!
  The one-dimensional coordinate system abstraction.

  Plotters build complex coordinate system with a combinator pattern and all the coordinate system is
  built from the one dimensional coordinate system. This module defines the fundamental types used by
  the one-dimensional coordinate system.

  The key trait for a one dimensional coordinate is [Ranged](trait.Ranged.html). This trait describes a
  set of values which served as the 1D coordinate system in Plotters. In order to extend the coordinate system,
  the new coordinate spec must implement this trait.

  The following example demonstrate how to make a customized coordinate specification
  ```
use plotters::coord::ranged1d::{Ranged, DefaultFormatting, KeyPointHint};
use std::ops::Range;

struct ZeroToOne;

impl Ranged for ZeroToOne {
    type ValueType = f64;
    type FormatOption = DefaultFormatting;

    fn map(&self, &v: &f64, pixel_range: (i32, i32)) -> i32 {
       let size = pixel_range.1 - pixel_range.0;
       let v = v.min(1.0).max(0.0);
       ((size as f64) * v).round() as i32
    }

    fn key_points<Hint:KeyPointHint>(&self, hint: Hint) -> Vec<f64> {
        if hint.max_num_points() < 3 {
            vec![]
        } else {
            vec![0.0, 0.5, 1.0]
        }
    }

    fn range(&self) -> Range<f64> {
        0.0..1.0
    }
}

use plotters::prelude::*;

let mut buffer = vec![0; 1024 * 768 * 3];
let root = BitMapBackend::with_buffer(&mut buffer, (1024, 768)).into_drawing_area();

let chart = ChartBuilder::on(&root)
    .build_cartesian_2d(ZeroToOne, ZeroToOne)
    .unwrap();

  ```
*/
use std::fmt::Debug;
use std::ops::Range;

pub(super) mod combinators;
pub(super) mod types;

mod discrete;
pub use discrete::{DiscreteRanged, IntoSegmentedCoord, SegmentValue, SegmentedCoord};

/// Since stable Rust doesn't have specialization, it's very hard to make our own trait that
/// automatically implemented the value formatter. This trait uses as a marker indicates if we
/// should automatically implement the default value formatter based on it's `Debug` trait
pub trait DefaultValueFormatOption {}

/// This makes the ranged coord uses the default `Debug` based formatting
pub struct DefaultFormatting;
impl DefaultValueFormatOption for DefaultFormatting {}

/// This markers prevent Plotters to implement the default `Debug` based formatting
pub struct NoDefaultFormatting;
impl DefaultValueFormatOption for NoDefaultFormatting {}

/// Determine how we can format a value in a coordinate system by default
pub trait ValueFormatter<V> {
    /// Format the value
    fn format(_value: &V) -> String {
        panic!("Unimplemented formatting method");
    }
    /// Determine how we can format a value in a coordinate system by default
    fn format_ext(&self, value: &V) -> String {
        Self::format(value)
    }
}

// By default the value is formatted by the debug trait
impl<R: Ranged<FormatOption = DefaultFormatting>> ValueFormatter<R::ValueType> for R
where
    R::ValueType: Debug,
{
    fn format(value: &R::ValueType) -> String {
        format!("{:?}", value)
    }
}

/// Specify the weight of key points.
pub enum KeyPointWeight {
    /// Allows only bold key points
    Bold,
    /// Allows any key points
    Any,
}

impl KeyPointWeight {
    /// Check if this key point weight setting allows light point
    pub fn allow_light_points(&self) -> bool {
        match self {
            KeyPointWeight::Bold => false,
            KeyPointWeight::Any => true,
        }
    }
}

/// The trait for a hint provided to the key point algorithm used by the coordinate specs.
/// The most important constraint is the `max_num_points` which means the algorithm could emit no more than specific number of key points
/// `weight` is used to determine if this is used as a bold grid line or light grid line
/// `bold_points` returns the max number of corresponding bold grid lines
pub trait KeyPointHint {
    /// Returns the max number of key points
    fn max_num_points(&self) -> usize;
    /// Returns the weight for this hint
    fn weight(&self) -> KeyPointWeight;
    /// Returns the point number constraint for the bold points
    fn bold_points(&self) -> usize {
        self.max_num_points()
    }
}

impl KeyPointHint for usize {
    fn max_num_points(&self) -> usize {
        *self
    }

    fn weight(&self) -> KeyPointWeight {
        KeyPointWeight::Any
    }
}

///  The key point hint indicates we only need key point for the bold grid lines
pub struct BoldPoints(pub usize);

impl KeyPointHint for BoldPoints {
    fn max_num_points(&self) -> usize {
        self.0
    }

    fn weight(&self) -> KeyPointWeight {
        KeyPointWeight::Bold
    }
}

/// The key point hint indicates that we are using the key points for the light grid lines
pub struct LightPoints {
    bold_points_num: usize,
    light_limit: usize,
}

impl LightPoints {
    /// Create a new light key point hind
    pub fn new(bold_count: usize, requested: usize) -> Self {
        Self {
            bold_points_num: bold_count,
            light_limit: requested,
        }
    }
}

impl KeyPointHint for LightPoints {
    fn max_num_points(&self) -> usize {
        self.light_limit
    }

    fn bold_points(&self) -> usize {
        self.bold_points_num
    }

    fn weight(&self) -> KeyPointWeight {
        KeyPointWeight::Any
    }
}

/// The trait that indicates we have a ordered and ranged value
/// Which is used to describe any 1D axis.
pub trait Ranged {
    /// This marker decides if Plotters default [ValueFormatter](trait.ValueFormatter.html) implementation should be used.
    /// This associated type can be one of the following two types:
    /// - [DefaultFormatting](struct.DefaultFormatting.html) will allow Plotters to automatically impl
    ///   the formatter based on `Debug` trait, if `Debug` trait is not impl for the `Self::Value`,
    ///   [ValueFormatter](trait.ValueFormatter.html) will not impl unless you impl it manually.
    ///
    /// - [NoDefaultFormatting](struct.NoDefaultFormatting.html) Disable the automatic `Debug`
    ///   based value formatting. Thus you have to impl the
    ///   [ValueFormatter](trait.ValueFormatter.html) manually.
    ///
    type FormatOption: DefaultValueFormatOption;

    /// The type of this value in this range specification
    type ValueType;

    /// This function maps the value to i32, which is the drawing coordinate
    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32;

    /// This function gives the key points that we can draw a grid based on this
    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType>;

    /// Get the range of this value
    fn range(&self) -> Range<Self::ValueType>;

    /// This function provides the on-axis part of its range
    #[allow(clippy::range_plus_one)]
    fn axis_pixel_range(&self, limit: (i32, i32)) -> Range<i32> {
        if limit.0 < limit.1 {
            limit.0..limit.1
        } else {
            limit.1..limit.0
        }
    }
}

/// The trait indicates the ranged value can be map reversely, which means
/// an pixel-based coordinate is given, it's possible to figure out the underlying
/// logic value.
pub trait ReversibleRanged: Ranged {
    /// Perform the reverse mapping
    fn unmap(&self, input: i32, limit: (i32, i32)) -> Option<Self::ValueType>;
}

/// The trait for the type that can be converted into a ranged coordinate axis
pub trait AsRangedCoord: Sized {
    /// Type to describe a coordinate system
    type CoordDescType: Ranged<ValueType = Self::Value> + From<Self>;
    /// Type for values in the given coordinate system
    type Value;
}

impl<T> AsRangedCoord for T
where
    T: Ranged,
{
    type CoordDescType = T;
    type Value = T::ValueType;
}
