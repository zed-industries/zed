//! Development-time-only helper module for exporting private APIs so that they can be benchmarked.
//! This module is gated behind the "benchmarks" feature.

use crate::common::BytesPerPixel;
use crate::filter::FilterType;
use crate::{BitDepth, ColorType, Info};

/// Re-exporting `unfilter` to make it easier to benchmark, despite some items being only
/// `pub(crate)`: `fn unfilter`, `enum BytesPerPixel`.
pub fn unfilter(filter: FilterType, tbpp: u8, previous: &[u8], current: &mut [u8]) {
    let tbpp = BytesPerPixel::from_usize(tbpp as usize);
    crate::filter::unfilter(filter, tbpp, previous, current)
}

pub use crate::decoder::transform::{create_transform_fn, TransformFn};

pub fn create_info_from_plte_trns_bitdepth<'a>(
    plte: &'a [u8],
    trns: Option<&'a [u8]>,
    bit_depth: u8,
) -> Info<'a> {
    Info {
        color_type: ColorType::Indexed,
        bit_depth: BitDepth::from_u8(bit_depth).unwrap(),
        palette: Some(plte.into()),
        trns: trns.map(Into::into),
        ..Info::default()
    }
}
