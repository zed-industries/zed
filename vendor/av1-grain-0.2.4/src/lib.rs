// Copyright (c) 2022-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

// Safety lints
#![deny(bare_trait_objects)]
#![deny(clippy::as_ptr_cast_mut)]
#![deny(clippy::cast_ptr_alignment)]
#![deny(clippy::large_stack_arrays)]
#![deny(clippy::ptr_as_ptr)]
#![deny(clippy::transmute_ptr_to_ptr)]
#![deny(clippy::unwrap_used)]
// Performance lints
#![warn(clippy::cloned_instead_of_copied)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::iter_with_drain)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::linkedlist)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::naive_bytecount)]
#![warn(clippy::needless_bitwise_bool)]
#![warn(clippy::needless_collect)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::no_effect_underscore_binding)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::stable_sort_primitive)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::trivial_regex)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::unnecessary_join)]
#![warn(clippy::unused_async)]
#![warn(clippy::zero_sized_map_values)]
// Correctness lints
#![deny(clippy::case_sensitive_file_extension_comparisons)]
#![deny(clippy::copy_iterator)]
#![deny(clippy::expl_impl_clone_on_copy)]
#![deny(clippy::float_cmp)]
#![warn(clippy::imprecise_flops)]
#![deny(clippy::manual_instant_elapsed)]
#![deny(clippy::match_same_arms)]
#![deny(clippy::mem_forget)]
#![warn(clippy::must_use_candidate)]
#![deny(clippy::path_buf_push_overwrite)]
#![deny(clippy::same_functions_in_if_condition)]
#![warn(clippy::suspicious_operation_groupings)]
#![deny(clippy::unchecked_duration_subtraction)]
#![deny(clippy::unicode_not_nfc)]
// Clarity/formatting lints
#![warn(clippy::borrow_as_ptr)]
#![warn(clippy::checked_conversions)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::derive_partial_eq_without_eq)]
#![warn(clippy::explicit_deref_methods)]
#![warn(clippy::filter_map_next)]
#![warn(clippy::flat_map_option)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::from_iter_instead_of_collect)]
#![warn(clippy::if_not_else)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::iter_not_returning_iterator)]
#![warn(clippy::iter_on_empty_collections)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_clamp)]
#![warn(clippy::manual_let_else)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::manual_string_new)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_bool)]
#![warn(clippy::mut_mut)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_continue)]
#![warn(clippy::option_if_let_else)]
#![warn(clippy::range_minus_one)]
#![warn(clippy::range_plus_one)]
#![warn(clippy::redundant_else)]
#![warn(clippy::ref_binding_to_reference)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unused_peekable)]
#![warn(clippy::unused_rounding)]
#![warn(clippy::unused_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::verbose_bit_mask)]
#![warn(clippy::verbose_file_reads)]
// Documentation lints
#![warn(clippy::doc_link_with_quotes)]
#![warn(clippy::doc_markdown)]

#[cfg(feature = "create")]
mod create;
#[cfg(feature = "diff")]
mod diff;
#[cfg(all(feature = "estimate", feature = "unstable"))]
mod estimate;
#[cfg(feature = "parse")]
mod parse;
mod util;

use arrayvec::ArrayVec;
#[cfg(feature = "create")]
pub use create::*;
#[cfg(feature = "diff")]
pub use diff::*;
#[cfg(all(feature = "estimate", feature = "unstable"))]
pub use estimate::*;
#[cfg(feature = "parse")]
pub use parse::*;
#[cfg(any(feature = "diff", feature = "estimate"))]
pub use v_frame;

/// The max number of luma scaling points for grain synthesis
pub const NUM_Y_POINTS: usize = 14;
/// The max number of scaling points per chroma plane for grain synthesis
pub const NUM_UV_POINTS: usize = 10;
/// The max number of luma coefficients for grain synthesis
pub const NUM_Y_COEFFS: usize = 24;
/// The max number of coefficients per chroma plane for grain synthesis
pub const NUM_UV_COEFFS: usize = 25;

/// A randomly generated u16 to be used as a starting random seed
/// for grain synthesis. The idea behind using a constant random seed
/// is so that encodes are deterministic and reproducible.
pub const DEFAULT_GRAIN_SEED: u16 = 10956;

pub type ScalingPoints = ArrayVec<[u8; 2], NUM_Y_POINTS>;

/// Specifies parameters for enabling decoder-side grain synthesis for
/// a segment of video from `start_time` to `end_time`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize, serde::Serialize))]
pub struct GrainTableSegment {
    /// The beginning timestamp of this segment, in 10,000,000ths of a second.
    pub start_time: u64,
    /// The ending timestamp of this segment, not inclusive, in 10,000,000ths of
    /// a second.
    pub end_time: u64,

    /// Values for the cutoffs and scale factors for luma scaling points
    pub scaling_points_y: ArrayVec<[u8; 2], NUM_Y_POINTS>,
    /// Values for the cutoffs and scale factors for Cb scaling points
    pub scaling_points_cb: ArrayVec<[u8; 2], NUM_UV_POINTS>,
    /// Values for the cutoffs and scale factors for Cr scaling points
    pub scaling_points_cr: ArrayVec<[u8; 2], NUM_UV_POINTS>,

    /// Determines the range and quantization step of the standard deviation
    /// of film grain.
    ///
    /// Accepts values between `8..=11`.
    pub scaling_shift: u8,

    /// A factor specifying how many AR coefficients are provided,
    /// based on the forumla `coeffs_len = (2 * ar_coeff_lag * (ar_coeff_lag +
    /// 1))`.
    ///
    /// Accepts values between `0..=3`.
    pub ar_coeff_lag: u8,
    /// Values for the AR coefficients for luma scaling points
    pub ar_coeffs_y: ArrayVec<i8, NUM_Y_COEFFS>,
    /// Values for the AR coefficients for Cb scaling points
    pub ar_coeffs_cb: ArrayVec<i8, NUM_UV_COEFFS>,
    /// Values for the AR coefficients for Cr scaling points
    pub ar_coeffs_cr: ArrayVec<i8, NUM_UV_COEFFS>,
    /// Shift value: Specifies the range of acceptable AR coefficients
    /// 6: [-2, 2)
    /// 7: [-1, 1)
    /// 8: [-0.5, 0.5)
    /// 9: [-0.25, 0.25)
    pub ar_coeff_shift: u8,
    /// Multiplier to the grain strength of the Cb plane
    pub cb_mult: u8,
    /// Multiplier to the grain strength of the Cb plane inherited from the luma
    /// plane
    pub cb_luma_mult: u8,
    /// A base value for the Cb plane grain
    pub cb_offset: u16,
    /// Multiplier to the grain strength of the Cr plane
    pub cr_mult: u8,
    /// Multiplier to the grain strength of the Cr plane inherited from the luma
    /// plane
    pub cr_luma_mult: u8,
    /// A base value for the Cr plane grain
    pub cr_offset: u16,

    /// Whether film grain blocks should overlap or not
    pub overlap_flag: bool,
    /// Scale chroma grain from luma instead of providing chroma scaling points
    pub chroma_scaling_from_luma: bool,
    /// Specifies how much the Gaussian random numbers should be scaled down
    /// during the grain synthesis process.
    pub grain_scale_shift: u8,
    /// Random seed used for generating grain
    pub random_seed: u16,
}
