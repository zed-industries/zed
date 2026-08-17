//! Traits for elliptic curve points.

use crate::{Curve, FieldBytes};
use subtle::{Choice, CtOption};

/// Obtain the affine x-coordinate of an elliptic curve point.
pub trait AffineXCoordinate<C: Curve> {
    /// Get the affine x-coordinate as a serialized field element.
    fn x(&self) -> FieldBytes<C>;
}

/// Decompress an elliptic curve point.
///
/// Point decompression recovers an original curve point from its x-coordinate
/// and a boolean flag indicating whether or not the y-coordinate is odd.
pub trait DecompressPoint<C: Curve>: Sized {
    /// Attempt to decompress an elliptic curve point.
    fn decompress(x: &FieldBytes<C>, y_is_odd: Choice) -> CtOption<Self>;
}

/// Decompact an elliptic curve point from an x-coordinate.
///
/// Decompaction relies on properties of specially-generated keys but provides
/// a more compact representation than standard point compression.
pub trait DecompactPoint<C: Curve>: Sized {
    /// Attempt to decompact an elliptic curve point
    fn decompact(x: &FieldBytes<C>) -> CtOption<Self>;
}

/// Point compression settings.
pub trait PointCompression {
    /// Should point compression be applied by default?
    const COMPRESS_POINTS: bool;
}

/// Point compaction settings.
pub trait PointCompaction {
    /// Should point compaction be applied by default?
    const COMPACT_POINTS: bool;
}
