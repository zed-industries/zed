// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use std::cmp;

use crate::api::color::ChromaSampling;
use crate::api::ContextInner;
use crate::encoder::TEMPORAL_DELIMITER;
use crate::quantize::{ac_q, dc_q, select_ac_qi, select_dc_qi};
use crate::util::{
  bexp64, bexp_q24, blog64, clamp, q24_to_q57, q57, q57_to_q24, Pixel,
};

// The number of frame sub-types for which we track distinct parameters.
// This does not include FRAME_SUBTYPE_SEF, because we don't need to do any
//  parameter tracking for Show Existing Frame frames.
pub const FRAME_NSUBTYPES: usize = 4;

pub const FRAME_SUBTYPE_I: usize = 0;
pub const FRAME_SUBTYPE_P: usize = 1;
#[allow(unused)]
pub const FRAME_SUBTYPE_B0: usize = 2;
#[allow(unused)]
pub const FRAME_SUBTYPE_B1: usize = 3;
pub const FRAME_SUBTYPE_SEF: usize = 4;

const PASS_SINGLE: i32 = 0;
const PASS_1: i32 = 1;
const PASS_2: i32 = 2;
const PASS_2_PLUS_1: i32 = 3;

// Magic value at the start of the 2-pass stats file
const TWOPASS_MAGIC: i32 = 0x50324156;
// Version number for the 2-pass stats file
const TWOPASS_VERSION: i32 = 1;
// 4 byte magic + 4 byte version + 4 byte TU count + 4 byte SEF frame count
//  + FRAME_NSUBTYPES*(4 byte frame count + 1 byte exp + 8 byte scale_sum)
pub(crate) const TWOPASS_HEADER_SZ: usize = 16 + FRAME_NSUBTYPES * (4 + 1 + 8);
// 4 byte frame type (show_frame and fti jointly coded) + 4 byte log_scale_q24
const TWOPASS_PACKET_SZ: usize = 8;

const SEF_BITS: i64 = 24;

// The scale of AV1 quantizer tables (relative to the pixel domain), i.e., Q3.
pub(crate) const QSCALE: i32 = 3;

// We clamp the actual I and B frame delays to a minimum of 10 to work
//  within the range of values where later incrementing the delay works as
//  designed.
// 10 is not an exact choice, but rather a good working trade-off.
const INTER_DELAY_TARGET_MIN: i32 = 10;

// The base quantizer for a frame is adjusted based on the frame type using the
//  formula (log_qp*mqp + dqp), where log_qp is the base-2 logarithm of the
//  "linear" quantizer (the actual factor by which coefficients are divided).
// Because log_qp has an implicit offset built in based on the scale of the
//  coefficients (which depends on the pixel bit depth and the transform
//  scale), we normalize the quantizer to the equivalent for 8-bit pixels with
//  orthonormal transforms for the purposes of rate modeling.
const MQP_Q12: &[i32; FRAME_NSUBTYPES] = &[
  // TODO: Use a const function once f64 operations in const functions are
  //  stable.
  (1.0 * (1 << 12) as f64) as i32,
  (1.0 * (1 << 12) as f64) as i32,
  (1.0 * (1 << 12) as f64) as i32,
  (1.0 * (1 << 12) as f64) as i32,
];

// The ratio 33_810_170.0 / 86_043_287.0 was derived by approximating the median
// of a change of 15 quantizer steps in the quantizer tables.
const DQP_Q57: &[i64; FRAME_NSUBTYPES] = &[
  (-(33_810_170.0 / 86_043_287.0) * (1i64 << 57) as f64) as i64,
  (0.0 * (1i64 << 57) as f64) as i64,
  ((33_810_170.0 / 86_043_287.0) * (1i64 << 57) as f64) as i64,
  (2.0 * (33_810_170.0 / 86_043_287.0) * (1i64 << 57) as f64) as i64,
];

// For 8-bit-depth inter frames, log_q_y is derived from log_target_q with a
//  linear model:
//  log_q_y = log_target_q + (log_target_q >> 32) * Q_MODEL_MUL + Q_MODEL_ADD
// Derivation of the linear models:
//  https://github.com/xiph/rav1e/blob/d02bdbd3b0b7b2cb9fc301031cc6a4e67a567a5c/doc/quantizer-weight-analysis.ipynb
#[rustfmt::skip]
const Q_MODEL_ADD: [i64; 4] = [
  // 4:2:0
  -0x24_4FE7_ECB3_DD90,
  // 4:2:2
  -0x37_41DA_38AD_0924,
  // 4:4:4
  -0x70_83BD_A626_311C,
  // 4:0:0
  0,
];
#[rustfmt::skip]
const Q_MODEL_MUL: [i64; 4] = [
  // 4:2:0
  0x8A0_50DD,
  // 4:2:2
  0x887_7666,
  // 4:4:4
  0x8D4_A712,
  // 4:0:0
  0,
];

#[rustfmt::skip]
const ROUGH_TAN_LOOKUP: &[u16; 18] = &[
     0,   358,   722,  1098,  1491,  1910,
  2365,  2868,  3437,  4096,  4881,  5850,
  7094,  8784, 11254, 15286, 23230, 46817
];

// A digital approximation of a 2nd-order low-pass Bessel follower.
// We use this for rate control because it has fast reaction time, but is
//  critically damped.
pub struct IIRBessel2 {
  c: [i32; 2],
  g: i32,
  x: [i32; 2],
  y: [i32; 2],
}

// alpha is Q24 in the range [0,0.5).
// The return value is 5.12.
fn warp_alpha(alpha: i32) -> i32 {
  let i = ((alpha * 36) >> 24).min(16);
  let t0 = ROUGH_TAN_LOOKUP[i as usize];
  let t1 = ROUGH_TAN_LOOKUP[i as usize + 1];
  let d = alpha * 36 - (i << 24);
  ((((t0 as i64) << 32) + (((t1 - t0) << 8) as i64) * (d as i64)) >> 32) as i32
}

// Compute Bessel filter coefficients with the specified delay.
// Return: Filter parameters (c[0], c[1], g).
fn iir_bessel2_get_parameters(delay: i32) -> (i32, i32, i32) {
  // This borrows some code from an unreleased version of Postfish.
  // See the recipe at http://unicorn.us.com/alex/2polefilters.html for details
  //  on deriving the filter coefficients.
  // alpha is Q24
  let alpha = (1 << 24) / delay;
  // warp is 7.12 (5.12? the max value is 70386 in Q12).
  let warp = warp_alpha(alpha).max(1) as i64;
  // k1 is 9.12 (6.12?)
  let k1 = 3 * warp;
  // k2 is 16.24 (11.24?)
  let k2 = k1 * warp;
  // d is 16.15 (10.15?)
  let d = ((((1 << 12) + k1) << 12) + k2 + 256) >> 9;
  // a is 0.32, since d is larger than both 1.0 and k2
  let a = (k2 << 23) / d;
  // ik2 is 25.24
  let ik2 = (1i64 << 48) / k2;
  // b1 is Q56; in practice, the integer ranges between -2 and 2.
  let b1 = 2 * a * (ik2 - (1i64 << 24));
  // b2 is Q56; in practice, the integer ranges between -2 and 2.
  let b2 = (1i64 << 56) - ((4 * a) << 24) - b1;
  // All of the filter parameters are Q24.
  (
    ((b1 + (1i64 << 31)) >> 32) as i32,
    ((b2 + (1i64 << 31)) >> 32) as i32,
    ((a + 128) >> 8) as i32,
  )
}

impl IIRBessel2 {
  pub fn new(delay: i32, value: i32) -> IIRBessel2 {
    let (c0, c1, g) = iir_bessel2_get_parameters(delay);
    IIRBessel2 { c: [c0, c1], g, x: [value, value], y: [value, value] }
  }

  // Re-initialize Bessel filter coefficients with the specified delay.
  // This does not alter the x/y state, but changes the reaction time of the
  //  filter.
  // Altering the time constant of a reactive filter without altering internal
  //  state is something that has to be done carefully, but our design operates
  //  at high enough delays and with small enough time constant changes to make
  //  it safe.
  pub fn reinit(&mut self, delay: i32) {
    let (c0, c1, g) = iir_bessel2_get_parameters(delay);
    self.c[0] = c0;
    self.c[1] = c1;
    self.g = g;
  }

  pub fn update(&mut self, x: i32) -> i32 {
    let c0 = self.c[0] as i64;
    let c1 = self.c[1] as i64;
    let g = self.g as i64;
    let x0 = self.x[0] as i64;
    let x1 = self.x[1] as i64;
    let y0 = self.y[0] as i64;
    let y1 = self.y[1] as i64;
    let ya =
      ((((x as i64) + x0 * 2 + x1) * g + y0 * c0 + y1 * c1 + (1i64 << 23))
        >> 24) as i32;
    self.x[1] = self.x[0];
    self.x[0] = x;
    self.y[1] = self.y[0];
    self.y[0] = ya;
    ya
  }
}

#[derive(Copy, Clone)]
struct RCFrameMetrics {
  // The log base 2 of the scale factor for this frame in Q24 format.
  log_scale_q24: i32,
  // The frame type from pass 1
  fti: usize,
  // Whether or not the frame was hidden in pass 1
  show_frame: bool,
  // TODO: The input frame number corresponding to this frame in the input.
  // input_frameno: u32
  // TODO vfr: PTS
}

impl RCFrameMetrics {
  const fn new() -> RCFrameMetrics {
    RCFrameMetrics { log_scale_q24: 0, fti: 0, show_frame: false }
  }
}

/// Rate control pass summary
///
/// It contains encoding information related to the whole previous
/// encoding pass.
#[derive(Debug, Default, Clone)]
pub struct RCSummary {
  pub(crate) ntus: i32,
  nframes: [i32; FRAME_NSUBTYPES + 1],
  exp: [u8; FRAME_NSUBTYPES],
  scale_sum: [i64; FRAME_NSUBTYPES],
  pub(crate) total: i32,
}

// Backing storage to deserialize Summary and Per-Frame pass data
//
// Can store up to a full header size since it is the largest of the two
// packet kinds.
pub(crate) struct RCDeserialize {
  // The current byte position in the frame metrics buffer.
  pass2_buffer_pos: usize,
  // In pass 2, this represents the number of bytes that are available in the
  //  input buffer.
  pass2_buffer_fill: usize,
  // Buffer for current frame metrics in pass 2.
  pass2_buffer: [u8; TWOPASS_HEADER_SZ],
}

impl Default for RCDeserialize {
  fn default() -> Self {
    RCDeserialize {
      pass2_buffer: [0; TWOPASS_HEADER_SZ],
      pass2_buffer_pos: 0,
      pass2_buffer_fill: 0,
    }
  }
}

impl RCDeserialize {
  // Fill the backing storage by reading enough bytes from the
  // buf slice until goal bytes are available for parsing.
  //
  // goal must be at most TWOPASS_HEADER_SZ.
  pub(crate) fn buffer_fill(
    &mut self, buf: &[u8], consumed: usize, goal: usize,
  ) -> usize {
    let mut consumed = consumed;
    while self.pass2_buffer_fill < goal && consumed < buf.len() {
      self.pass2_buffer[self.pass2_buffer_fill] = buf[consumed];
      self.pass2_buffer_fill += 1;
      consumed += 1;
    }
    consumed
  }

  // Read the next n bytes as i64.
  // n must be within 1 and 8
  fn unbuffer_val(&mut self, n: usize) -> i64 {
    let mut bytes = n;
    let mut ret = 0;
    let mut shift = 0;
    while bytes > 0 {
      bytes -= 1;
      ret |= (self.pass2_buffer[self.pass2_buffer_pos] as i64) << shift;
      self.pass2_buffer_pos += 1;
      shift += 8;
    }
    ret
  }

  // Read metrics for the next frame.
  fn parse_metrics(&mut self) -> Result<RCFrameMetrics, String> {
    debug_assert!(self.pass2_buffer_fill >= TWOPASS_PACKET_SZ);
    let ft_val = self.unbuffer_val(4);
    let show_frame = (ft_val >> 31) != 0;
    let fti = (ft_val & 0x7FFFFFFF) as usize;
    // Make sure the frame type is valid.
    if fti > FRAME_NSUBTYPES {
      return Err("Invalid frame type".to_string());
    }
    let log_scale_q24 = self.unbuffer_val(4) as i32;
    Ok(RCFrameMetrics { log_scale_q24, fti, show_frame })
  }

  // Read the summary header data.
  pub(crate) fn parse_summary(&mut self) -> Result<RCSummary, String> {
    // check the magic value and version number.
    if self.unbuffer_val(4) != TWOPASS_MAGIC as i64 {
      return Err("Magic value mismatch".to_string());
    }
    if self.unbuffer_val(4) != TWOPASS_VERSION as i64 {
      return Err("Version number mismatch".to_string());
    }
    let mut s =
      RCSummary { ntus: self.unbuffer_val(4) as i32, ..Default::default() };

    // Make sure the file claims to have at least one TU.
    // Otherwise we probably got the placeholder data from an aborted
    //  pass 1.
    if s.ntus < 1 {
      return Err("No TUs found in first pass summary".to_string());
    }
    let mut total: i32 = 0;
    for nframes in s.nframes.iter_mut() {
      let n = self.unbuffer_val(4) as i32;
      if n < 0 {
        return Err("Got negative frame count".to_string());
      }
      total = total
        .checked_add(n)
        .ok_or_else(|| "Frame count too large".to_string())?;

      *nframes = n;
    }

    // We can't have more TUs than frames.
    if s.ntus > total {
      return Err("More TUs than frames".to_string());
    }

    s.total = total;

    for exp in s.exp.iter_mut() {
      *exp = self.unbuffer_val(1) as u8;
    }

    for scale_sum in s.scale_sum.iter_mut() {
      *scale_sum = self.unbuffer_val(8);
      if *scale_sum < 0 {
        return Err("Got negative scale sum".to_string());
      }
    }
    Ok(s)
  }
}

pub struct RCState {
  // The target bit-rate in bits per second.
  target_bitrate: i32,
  // The number of TUs over which to distribute the reservoir usage.
  // We use TUs because in our leaky bucket model, we only add bits to the
  //  reservoir on TU boundaries.
  reservoir_frame_delay: i32,
  // Whether or not the reservoir_frame_delay was explicitly specified by the
  //  user, or is the default value.
  reservoir_frame_delay_is_set: bool,
  // The maximum quantizer index to allow (for the luma AC coefficients, other
  //  quantizers will still be adjusted to match).
  maybe_ac_qi_max: Option<u8>,
  // The minimum quantizer index to allow (for the luma AC coefficients).
  ac_qi_min: u8,
  // Will we drop frames to meet bitrate requirements?
  drop_frames: bool,
  // Do we respect the maximum reservoir fullness?
  cap_overflow: bool,
  // Can the reservoir go negative?
  cap_underflow: bool,
  // The log of the first-pass base quantizer.
  pass1_log_base_q: i64,
  // Two-pass mode state.
  // PASS_SINGLE => 1-pass encoding.
  // PASS_1 => 1st pass of 2-pass encoding.
  // PASS_2 => 2nd pass of 2-pass encoding.
  // PASS_2_PLUS_1 => 2nd pass of 2-pass encoding, but also emitting pass 1
  //  data again.
  twopass_state: i32,
  // The log of the number of pixels in a frame in Q57 format.
  log_npixels: i64,
  // The target average bits per Temporal Unit (input frame).
  bits_per_tu: i64,
  // The current bit reservoir fullness (bits available to be used).
  reservoir_fullness: i64,
  // The target buffer fullness.
  // This is where we'd like to be by the last keyframe that appears in the
  //  next reservoir_frame_delay frames.
  reservoir_target: i64,
  // The maximum buffer fullness (total size of the buffer).
  reservoir_max: i64,
  // The log of estimated scale factor for the rate model in Q57 format.
  //
  // TODO: Convert to Q23 or figure out a better way to avoid overflow
  // once 2-pass mode is introduced, if required.
  log_scale: [i64; FRAME_NSUBTYPES],
  // The exponent used in the rate model in Q6 format.
  exp: [u8; FRAME_NSUBTYPES],
  // The log of an estimated scale factor used to obtain the real framerate,
  //  for VFR sources or, e.g., 12 fps content doubled to 24 fps, etc.
  // TODO vfr: log_vfr_scale: i64,
  // Second-order lowpass filters to track scale and VFR.
  scalefilter: [IIRBessel2; FRAME_NSUBTYPES],
  // TODO vfr: vfrfilter: IIRBessel2,
  // The number of frames of each type we have seen, for filter adaptation
  //  purposes.
  // These are only 32 bits to guarantee that we can sum the scales over the
  //  whole file without overflow in a 64-bit int.
  // That limits us to 2.268 years at 60 fps (minus 33% with re-ordering).
  nframes: [i32; FRAME_NSUBTYPES + 1],
  inter_delay: [i32; FRAME_NSUBTYPES - 1],
  inter_delay_target: i32,
  // The total accumulated estimation bias.
  rate_bias: i64,
  // The number of (non-Show Existing Frame) frames that have been encoded.
  nencoded_frames: i64,
  // The number of Show Existing Frames that have been emitted.
  nsef_frames: i64,
  // Buffer for current frame metrics in pass 1.
  pass1_buffer: [u8; TWOPASS_HEADER_SZ],
  // Whether or not the user has retrieved the pass 1 data for the last frame.
  // For PASS_1 or PASS_2_PLUS_1 encoding, this is set to false after each
  //  frame is encoded, and must be set to true by calling twopass_out() before
  //  the next frame can be encoded.
  pub pass1_data_retrieved: bool,
  // Marks whether or not the user has retrieved the summary data at the end of
  //  the encode.
  pass1_summary_retrieved: bool,
  // Whether or not the user has provided enough data to encode in the second
  //  pass.
  // For PASS_2 or PASS_2_PLUS_1 encoding, this is set to false after each
  //  frame, and must be set to true by calling twopass_in() before the next
  //  frame can be encoded.
  pass2_data_ready: bool,
  // TODO: Add a way to force the next frame to be a keyframe in 2-pass mode.
  // Right now we are relying on keyframe detection to detect the same
  //  keyframes.
  // The metrics for the previous frame.
  prev_metrics: RCFrameMetrics,
  // The metrics for the current frame.
  cur_metrics: RCFrameMetrics,
  // The buffered metrics for future frames.
  frame_metrics: Vec<RCFrameMetrics>,
  // The total number of frames still in use in the circular metric buffer.
  nframe_metrics: usize,
  // The index of the current frame in the circular metric buffer.
  frame_metrics_head: usize,
  // Data deserialization
  des: RCDeserialize,
  // The TU count encoded so far.
  ntus: i32,
  // The TU count for the whole file.
  ntus_total: i32,
  // The remaining TU count.
  ntus_left: i32,
  // The frame count of each frame subtype in the whole file.
  nframes_total: [i32; FRAME_NSUBTYPES + 1],
  // The sum of those counts.
  nframes_total_total: i32,
  // The number of frames of each subtype yet to be processed.
  nframes_left: [i32; FRAME_NSUBTYPES + 1],
  // The sum of the scale values for each frame subtype.
  scale_sum: [i64; FRAME_NSUBTYPES],
  // The number of TUs represented by the current scale sums.
  scale_window_ntus: i32,
  // The frame count of each frame subtype in the current scale window.
  scale_window_nframes: [i32; FRAME_NSUBTYPES + 1],
  // The sum of the scale values for each frame subtype in the current window.
  scale_window_sum: [i64; FRAME_NSUBTYPES],
}

// TODO: Separate qi values for each color plane.
pub struct QuantizerParameters {
  // The full-precision, unmodulated log quantizer upon which our modulated
  //  quantizer indices are based.
  // This is only used to limit sudden quality changes from frame to frame, and
  //  as such is not adjusted when we encounter buffer overrun or underrun.
  pub log_base_q: i64,
  // The full-precision log quantizer modulated by the current frame type upon
  //  which our quantizer indices are based (including any adjustments to
  //  prevent buffer overrun or underrun).
  // This is used when estimating the scale parameter once we know the actual
  //  bit usage of a frame.
  pub log_target_q: i64,
  pub dc_qi: [u8; 3],
  pub ac_qi: [u8; 3],
  pub lambda: f64,
  pub dist_scale: [f64; 3],
}

const Q57_SQUARE_EXP_SCALE: f64 =
  (2.0 * ::std::f64::consts::LN_2) / ((1i64 << 57) as f64);

// Daala style log-offset for chroma quantizers
// TODO: Optimal offsets for more configurations than just BT.709
fn chroma_offset(
  log_target_q: i64, chroma_sampling: ChromaSampling,
) -> (i64, i64) {
  let x = log_target_q.max(0);
  // Gradient optimized for CIEDE2000+PSNR on subset3
  let y = match chroma_sampling {
    ChromaSampling::Cs400 => 0,
    ChromaSampling::Cs420 => (x >> 2) + (x >> 6), // 0.266
    ChromaSampling::Cs422 => (x >> 3) + (x >> 4) - (x >> 7), // 0.180
    ChromaSampling::Cs444 => (x >> 4) + (x >> 5) + (x >> 8), // 0.098
  };
  // blog64(7) - blog64(4); blog64(5) - blog64(4)
  (0x19D_5D9F_D501_0B37 - y, 0xA4_D3C2_5E68_DC58 - y)
}

impl QuantizerParameters {
  fn new_from_log_q(
    log_base_q: i64, log_target_q: i64, bit_depth: usize,
    chroma_sampling: ChromaSampling, is_intra: bool,
    log_isqrt_mean_scale: i64,
  ) -> QuantizerParameters {
    let scale = log_isqrt_mean_scale + q57(QSCALE + bit_depth as i32 - 8);

    let mut log_q_y = log_target_q;
    if !is_intra && bit_depth == 8 {
      log_q_y = log_target_q
        + (log_target_q >> 32) * Q_MODEL_MUL[chroma_sampling as usize]
        + Q_MODEL_ADD[chroma_sampling as usize];
    }

    let quantizer = bexp64(log_q_y + scale);
    let (offset_u, offset_v) =
      chroma_offset(log_q_y + log_isqrt_mean_scale, chroma_sampling);
    let mono = chroma_sampling == ChromaSampling::Cs400;
    let log_q_u = log_q_y + offset_u;
    let log_q_v = log_q_y + offset_v;
    let quantizer_u = bexp64(log_q_u + scale);
    let quantizer_v = bexp64(log_q_v + scale);
    let lambda = (::std::f64::consts::LN_2 / 6.0)
      * (((log_target_q + log_isqrt_mean_scale) as f64)
        * Q57_SQUARE_EXP_SCALE)
        .exp();

    let scale = |q| bexp64((log_target_q - q) * 2 + q57(16)) as f64 / 65536.;
    let dist_scale = [scale(log_q_y), scale(log_q_u), scale(log_q_v)];

    let base_q_idx = select_ac_qi(quantizer, bit_depth).max(1);

    // delta_q only gets 6 bits + a sign bit, so it can differ by 63 at most.
    let min_qi = base_q_idx.saturating_sub(63).max(1);
    let max_qi = base_q_idx.saturating_add(63);
    let clamp_qi = |qi: u8| qi.clamp(min_qi, max_qi);

    QuantizerParameters {
      log_base_q,
      log_target_q,
      // TODO: Allow lossless mode; i.e. qi == 0.
      dc_qi: [
        clamp_qi(select_dc_qi(quantizer, bit_depth)),
        if mono { 0 } else { clamp_qi(select_dc_qi(quantizer_u, bit_depth)) },
        if mono { 0 } else { clamp_qi(select_dc_qi(quantizer_v, bit_depth)) },
      ],
      ac_qi: [
        base_q_idx,
        if mono { 0 } else { clamp_qi(select_ac_qi(quantizer_u, bit_depth)) },
        if mono { 0 } else { clamp_qi(select_ac_qi(quantizer_v, bit_depth)) },
      ],
      lambda,
      dist_scale,
    }
  }
}

impl RCState {
  pub fn new(
    frame_width: i32, frame_height: i32, framerate_num: i64,
    framerate_den: i64, target_bitrate: i32, maybe_ac_qi_max: Option<u8>,
    ac_qi_min: u8, max_key_frame_interval: i32,
    maybe_reservoir_frame_delay: Option<i32>,
  ) -> RCState {
    // The default buffer size is set equal to 1.5x the keyframe interval, or 240
    //  frames; whichever is smaller, with a minimum of 12.
    // For user set values, we enforce a minimum of 12.
    // The interval is short enough to allow reaction, but long enough to allow
    //  looking into the next GOP (avoiding the case where the last frames
    //  before an I-frame get starved), in most cases.
    // The 12 frame minimum gives us some chance to distribute bit estimation
    //  errors in the worst case.
    let reservoir_frame_delay = maybe_reservoir_frame_delay
      .unwrap_or_else(|| ((max_key_frame_interval * 3) >> 1).min(240))
      .max(12);
    // TODO: What are the limits on these?
    let npixels = (frame_width as i64) * (frame_height as i64);
    // Insane framerates or frame sizes mean insane bitrates.
    // Let's not get carried away.
    // We also subtract 16 bits from each temporal unit to account for the
    //  temporal delimiter, whose bits are not included in the frame sizes
    //  reported to update_state().
    // TODO: Support constraints imposed by levels.
    let bits_per_tu = clamp(
      (target_bitrate as i64) * framerate_den / framerate_num,
      40,
      0x4000_0000_0000,
    ) - (TEMPORAL_DELIMITER.len() * 8) as i64;
    let reservoir_max = bits_per_tu * (reservoir_frame_delay as i64);
    // Start with a buffer fullness and fullness target of 50%.
    let reservoir_target = (reservoir_max + 1) >> 1;
    // Pick exponents and initial scales for quantizer selection.
    let ibpp = npixels / bits_per_tu;
    // These have been derived by encoding many clips at every quantizer
    // and running a piecewise-linear regression in binary log space.
    let (i_exp, i_log_scale) = if ibpp < 1 {
      (48u8, blog64(36) - q57(QSCALE))
    } else if ibpp < 4 {
      (61u8, blog64(55) - q57(QSCALE))
    } else {
      (77u8, blog64(129) - q57(QSCALE))
    };
    let (p_exp, p_log_scale) = if ibpp < 2 {
      (69u8, blog64(32) - q57(QSCALE))
    } else if ibpp < 139 {
      (104u8, blog64(84) - q57(QSCALE))
    } else {
      (83u8, blog64(19) - q57(QSCALE))
    };
    let (b0_exp, b0_log_scale) = if ibpp < 2 {
      (84u8, blog64(30) - q57(QSCALE))
    } else if ibpp < 92 {
      (120u8, blog64(68) - q57(QSCALE))
    } else {
      (68u8, blog64(4) - q57(QSCALE))
    };
    let (b1_exp, b1_log_scale) = if ibpp < 2 {
      (87u8, blog64(27) - q57(QSCALE))
    } else if ibpp < 126 {
      (139u8, blog64(84) - q57(QSCALE))
    } else {
      (61u8, blog64(1) - q57(QSCALE))
    };

    // TODO: Add support for "golden" P frames.
    RCState {
      target_bitrate,
      reservoir_frame_delay,
      reservoir_frame_delay_is_set: maybe_reservoir_frame_delay.is_some(),
      maybe_ac_qi_max,
      ac_qi_min,
      drop_frames: false,
      cap_overflow: true,
      cap_underflow: false,
      pass1_log_base_q: 0,
      twopass_state: PASS_SINGLE,
      log_npixels: blog64(npixels),
      bits_per_tu,
      reservoir_fullness: reservoir_target,
      reservoir_target,
      reservoir_max,
      log_scale: [i_log_scale, p_log_scale, b0_log_scale, b1_log_scale],
      exp: [i_exp, p_exp, b0_exp, b1_exp],
      scalefilter: [
        IIRBessel2::new(4, q57_to_q24(i_log_scale)),
        IIRBessel2::new(INTER_DELAY_TARGET_MIN, q57_to_q24(p_log_scale)),
        IIRBessel2::new(INTER_DELAY_TARGET_MIN, q57_to_q24(b0_log_scale)),
        IIRBessel2::new(INTER_DELAY_TARGET_MIN, q57_to_q24(b1_log_scale)),
      ],
      // TODO VFR
      nframes: [0; FRAME_NSUBTYPES + 1],
      inter_delay: [INTER_DELAY_TARGET_MIN; FRAME_NSUBTYPES - 1],
      inter_delay_target: reservoir_frame_delay >> 1,
      rate_bias: 0,
      nencoded_frames: 0,
      nsef_frames: 0,
      pass1_buffer: [0; TWOPASS_HEADER_SZ],
      pass1_data_retrieved: true,
      pass1_summary_retrieved: false,
      pass2_data_ready: false,
      prev_metrics: RCFrameMetrics::new(),
      cur_metrics: RCFrameMetrics::new(),
      frame_metrics: Vec::new(),
      nframe_metrics: 0,
      frame_metrics_head: 0,
      ntus: 0,
      ntus_total: 0,
      ntus_left: 0,
      nframes_total: [0; FRAME_NSUBTYPES + 1],
      nframes_total_total: 0,
      nframes_left: [0; FRAME_NSUBTYPES + 1],
      scale_sum: [0; FRAME_NSUBTYPES],
      scale_window_ntus: 0,
      scale_window_nframes: [0; FRAME_NSUBTYPES + 1],
      scale_window_sum: [0; FRAME_NSUBTYPES],
      des: RCDeserialize::default(),
    }
  }

  pub(crate) fn select_first_pass_qi(
    &self, bit_depth: usize, fti: usize, chroma_sampling: ChromaSampling,
  ) -> QuantizerParameters {
    // Adjust the quantizer for the frame type, result is Q57:
    let log_q = ((self.pass1_log_base_q + (1i64 << 11)) >> 12)
      * (MQP_Q12[fti] as i64)
      + DQP_Q57[fti];
    QuantizerParameters::new_from_log_q(
      self.pass1_log_base_q,
      log_q,
      bit_depth,
      chroma_sampling,
      fti == 0,
      0,
    )
  }

  // TODO: Separate quantizers for Cb and Cr.
  #[profiling::function]
  pub(crate) fn select_qi<T: Pixel>(
    &self, ctx: &ContextInner<T>, output_frameno: u64, fti: usize,
    maybe_prev_log_base_q: Option<i64>, log_isqrt_mean_scale: i64,
  ) -> QuantizerParameters {
    // Is rate control active?
    if self.target_bitrate <= 0 {
      // Rate control is not active.
      // Derive quantizer directly from frame type.
      let bit_depth = ctx.config.bit_depth;
      let chroma_sampling = ctx.config.chroma_sampling;
      let (log_base_q, log_q) =
        Self::calc_flat_quantizer(ctx.config.quantizer as u8, bit_depth, fti);
      QuantizerParameters::new_from_log_q(
        log_base_q,
        log_q,
        bit_depth,
        chroma_sampling,
        fti == 0,
        log_isqrt_mean_scale,
      )
    } else {
      let mut nframes: [i32; FRAME_NSUBTYPES + 1] = [0; FRAME_NSUBTYPES + 1];
      let mut log_scale: [i64; FRAME_NSUBTYPES] = self.log_scale;
      let mut reservoir_tus = self.reservoir_frame_delay.min(self.ntus_left);
      let mut reservoir_frames = 0;
      let mut log_cur_scale = (self.scalefilter[fti].y[0] as i64) << 33;
      match self.twopass_state {
        // First pass of 2-pass mode: use a fixed base quantizer.
        PASS_1 => {
          return self.select_first_pass_qi(
            ctx.config.bit_depth,
            fti,
            ctx.config.chroma_sampling,
          );
        }
        // Second pass of 2-pass mode: we know exactly how much of each frame
        //  type there is in the current buffer window, and have estimates for
        //  the scales.
        PASS_2 | PASS_2_PLUS_1 => {
          let mut scale_window_sum: [i64; FRAME_NSUBTYPES] =
            self.scale_window_sum;
          let mut scale_window_nframes: [i32; FRAME_NSUBTYPES + 1] =
            self.scale_window_nframes;
          // Intentionally exclude Show Existing Frame frames from this.
          for ftj in 0..FRAME_NSUBTYPES {
            reservoir_frames += scale_window_nframes[ftj];
          }
          // If we're approaching the end of the file, add some slack to keep
          //  us from slamming into a rail.
          // Our rate accuracy goes down, but it keeps the result sensible.
          // We position the target where the first forced keyframe beyond the
          //  end of the file would be (for consistency with 1-pass mode).
          // TODO: let mut buf_pad = self.reservoir_frame_delay.min(...);
          // if buf_delay < buf_pad {
          //   buf_pad -= buf_delay;
          // }
          // else ...
          // Otherwise, search for the last keyframe in the buffer window and
          //  target that.
          // Currently we only do this when using a finite buffer.
          // We could save the position of the last keyframe in the stream in
          //  the summary data and do it with a whole-file buffer as well, but
          //  it isn't likely to make a difference.
          if !self.frame_metrics.is_empty() {
            let mut fm_tail = self.frame_metrics_head + self.nframe_metrics;
            if fm_tail >= self.frame_metrics.len() {
              fm_tail -= self.frame_metrics.len();
            }
            let mut fmi = fm_tail;
            loop {
              if fmi == 0 {
                fmi += self.frame_metrics.len();
              }
              fmi -= 1;
              // Stop before we remove the first frame.
              if fmi == self.frame_metrics_head {
                break;
              }
              // If we find a keyframe, remove it and everything past it.
              if self.frame_metrics[fmi].fti == FRAME_SUBTYPE_I {
                while fmi != fm_tail {
                  let m = &self.frame_metrics[fmi];
                  let ftj = m.fti;
                  scale_window_nframes[ftj] -= 1;
                  if ftj < FRAME_NSUBTYPES {
                    scale_window_sum[ftj] -= bexp_q24(m.log_scale_q24);
                    reservoir_frames -= 1;
                  }
                  if m.show_frame {
                    reservoir_tus -= 1;
                  }
                  fmi += 1;
                  if fmi >= self.frame_metrics.len() {
                    fmi = 0;
                  }
                }
                // And stop scanning backwards.
                break;
              }
            }
          }
          nframes = scale_window_nframes;
          // If we're not using the same frame type as in pass 1 (because
          //  someone changed some encoding parameters), remove that scale
          //  estimate.
          // We'll add a replacement for the correct frame type below.
          if self.cur_metrics.fti != fti {
            scale_window_nframes[self.cur_metrics.fti] -= 1;
            if self.cur_metrics.fti != FRAME_SUBTYPE_SEF {
              scale_window_sum[self.cur_metrics.fti] -=
                bexp_q24(self.cur_metrics.log_scale_q24);
            }
          } else {
            log_cur_scale = (self.cur_metrics.log_scale_q24 as i64) << 33;
          }
          // If we're approaching the end of the file, add some slack to keep
          //  us from slamming into a rail.
          // Our rate accuracy goes down, but it keeps the result sensible.
          // We position the target where the first forced keyframe beyond the
          //  end of the file would be (for consistency with 1-pass mode).
          if reservoir_tus >= self.ntus_left
            && self.ntus_total as u64
              > ctx.gop_input_frameno_start[&output_frameno]
          {
            let nfinal_gop_tus = self.ntus_total
              - (ctx.gop_input_frameno_start[&output_frameno] as i32);
            if ctx.config.max_key_frame_interval as i32 > nfinal_gop_tus {
              let reservoir_pad = (ctx.config.max_key_frame_interval as i32
                - nfinal_gop_tus)
                .min(self.reservoir_frame_delay - reservoir_tus);
              let (guessed_reservoir_frames, guessed_reservoir_tus) = ctx
                .guess_frame_subtypes(
                  &mut nframes,
                  reservoir_tus + reservoir_pad,
                );
              reservoir_frames = guessed_reservoir_frames;
              reservoir_tus = guessed_reservoir_tus;
            }
          }
          // Blend in the low-pass filtered scale according to how many
          //  frames of each type we need to add compared to the actual sums in
          //  our window.
          for ftj in 0..FRAME_NSUBTYPES {
            let scale = scale_window_sum[ftj]
              + bexp_q24(self.scalefilter[ftj].y[0])
                * (nframes[ftj] - scale_window_nframes[ftj]) as i64;
            log_scale[ftj] = if nframes[ftj] > 0 {
              blog64(scale) - blog64(nframes[ftj] as i64) - q57(24)
            } else {
              -self.log_npixels
            };
          }
        }
        // Single pass.
        _ => {
          // Figure out how to re-distribute bits so that we hit our fullness
          //  target before the last keyframe in our current buffer window
          //  (after the current frame), or the end of the buffer window,
          //  whichever comes first.
          // Count the various types and classes of frames.
          let (guessed_reservoir_frames, guessed_reservoir_tus) =
            ctx.guess_frame_subtypes(&mut nframes, self.reservoir_frame_delay);
          reservoir_frames = guessed_reservoir_frames;
          reservoir_tus = guessed_reservoir_tus;
          // TODO: Scale for VFR.
        }
      }
      // If we've been missing our target, add a penalty term.
      let rate_bias = (self.rate_bias / (self.nencoded_frames + 100))
        * (reservoir_frames as i64);
      // rate_total is the total bits available over the next
      //  reservoir_tus TUs.
      let rate_total = self.reservoir_fullness - self.reservoir_target
        + rate_bias
        + (reservoir_tus as i64) * self.bits_per_tu;
      // Find a target quantizer that meets our rate target for the
      //  specific mix of frame types we'll have over the next
      //  reservoir_frame frames.
      // We model the rate<->quantizer relationship as
      //  rate = scale*(quantizer**-exp)
      // In this case, we have our desired rate, an exponent selected in
      //  setup, and a scale that's been measured over our frame history,
      //  so we're solving for the quantizer.
      // Exponentiation with arbitrary exponents is expensive, so we work
      //  in the binary log domain (binary exp and log aren't too bad):
      //  rate = exp2(log2(scale) - log2(quantizer)*exp)
      // There's no easy closed form solution, so we bisection searh for it.
      let bit_depth = ctx.config.bit_depth;
      let chroma_sampling = ctx.config.chroma_sampling;
      // TODO: Proper handling of lossless.
      let mut log_qlo = blog64(ac_q(self.ac_qi_min, 0, bit_depth).get() as i64)
        - q57(QSCALE + bit_depth as i32 - 8);
      // The AC quantizer tables map to values larger than the DC quantizer
      //  tables, so we use that as the upper bound to make sure we can use
      //  the full table if needed.
      let mut log_qhi = blog64(
        ac_q(self.maybe_ac_qi_max.unwrap_or(255), 0, bit_depth).get() as i64,
      ) - q57(QSCALE + bit_depth as i32 - 8);
      let mut log_base_q = (log_qlo + log_qhi) >> 1;
      while log_qlo < log_qhi {
        // Count bits contributed by each frame type using the model.
        let mut bits = 0i64;
        for ftj in 0..FRAME_NSUBTYPES {
          // Modulate base quantizer by frame type.
          let log_q = ((log_base_q + (1i64 << 11)) >> 12)
            * (MQP_Q12[ftj] as i64)
            + DQP_Q57[ftj];
          // All the fields here are Q57 except for the exponent, which is
          //  Q6.
          bits += (nframes[ftj] as i64)
            * bexp64(
              log_scale[ftj] + self.log_npixels
                - ((log_q + 32) >> 6) * (self.exp[ftj] as i64),
            );
        }
        // The number of bits for Show Existing Frame frames is constant.
        bits += (nframes[FRAME_SUBTYPE_SEF] as i64) * SEF_BITS;
        let diff = bits - rate_total;
        if diff > 0 {
          log_qlo = log_base_q + 1;
        } else if diff < 0 {
          log_qhi = log_base_q - 1;
        } else {
          break;
        }
        log_base_q = (log_qlo + log_qhi) >> 1;
      }
      // If this was not one of the initial frames, limit the change in
      //  base quantizer to within [0.8*Q, 1.2*Q] where Q is the previous
      //  frame's base quantizer.
      if let Some(prev_log_base_q) = maybe_prev_log_base_q {
        log_base_q = clamp(
          log_base_q,
          prev_log_base_q - 0xA4_D3C2_5E68_DC58,
          prev_log_base_q + 0xA4_D3C2_5E68_DC58,
        );
      }
      // Modulate base quantizer by frame type.
      let mut log_q = ((log_base_q + (1i64 << 11)) >> 12)
        * (MQP_Q12[fti] as i64)
        + DQP_Q57[fti];
      // The above allocation looks only at the total rate we'll accumulate
      //  in the next reservoir_frame_delay frames.
      // However, we could overflow the bit reservoir on the very next
      //  frame.
      // Check for that here if we're not using a soft target.
      if self.cap_overflow {
        // Allow 3% of the buffer for prediction error.
        // This should be plenty, and we don't mind if we go a bit over.
        // We only want to keep these bits from being completely wasted.
        let margin = (self.reservoir_max + 31) >> 5;
        // We want to use at least this many bits next frame.
        let soft_limit = self.reservoir_fullness + self.bits_per_tu
          - (self.reservoir_max - margin);
        if soft_limit > 0 {
          let log_soft_limit = blog64(soft_limit);
          // If we're predicting we won't use that many bits...
          // TODO: When using frame re-ordering, we should include the rate
          //  for all of the frames in the current TU.
          // When there is more than one frame, there will be no direct
          //  solution for the required adjustment, however.
          let log_scale_pixels = log_cur_scale + self.log_npixels;
          let exp = self.exp[fti] as i64;
          let mut log_q_exp = ((log_q + 32) >> 6) * exp;
          if log_scale_pixels - log_q_exp < log_soft_limit {
            // Scale the adjustment based on how far into the margin we are.
            log_q_exp += ((log_scale_pixels - log_soft_limit - log_q_exp)
              >> 32)
              * ((margin.min(soft_limit) << 32) / margin);
            log_q = ((log_q_exp + (exp >> 1)) / exp) << 6;
          }
        }
      }
      // We just checked we don't overflow the reservoir next frame, now
      //  check we don't underflow and bust the budget (when not using a
      //  soft target).
      if self.maybe_ac_qi_max.is_none() {
        // Compute the maximum number of bits we can use in the next frame.
        // Allow 50% of the rate for a single frame for prediction error.
        // This may not be enough for keyframes or sudden changes in
        //  complexity.
        let log_hard_limit =
          blog64(self.reservoir_fullness + (self.bits_per_tu >> 1));
        // If we're predicting we'll use more than this...
        // TODO: When using frame re-ordering, we should include the rate
        //  for all of the frames in the current TU.
        // When there is more than one frame, there will be no direct
        //  solution for the required adjustment, however.
        let log_scale_pixels = log_cur_scale + self.log_npixels;
        let exp = self.exp[fti] as i64;
        let mut log_q_exp = ((log_q + 32) >> 6) * exp;
        if log_scale_pixels - log_q_exp > log_hard_limit {
          // Force the target to hit our limit exactly.
          log_q_exp = log_scale_pixels - log_hard_limit;
          log_q = ((log_q_exp + (exp >> 1)) / exp) << 6;
          // If that target is unreasonable, oh well; we'll have to drop.
        }
      }

      if let Some(qi_max) = self.maybe_ac_qi_max {
        let (max_log_base_q, max_log_q) =
          Self::calc_flat_quantizer(qi_max, ctx.config.bit_depth, fti);
        log_base_q = cmp::min(log_base_q, max_log_base_q);
        log_q = cmp::min(log_q, max_log_q);
      }
      if self.ac_qi_min > 0 {
        let (min_log_base_q, min_log_q) =
          Self::calc_flat_quantizer(self.ac_qi_min, ctx.config.bit_depth, fti);
        log_base_q = cmp::max(log_base_q, min_log_base_q);
        log_q = cmp::max(log_q, min_log_q);
      }
      QuantizerParameters::new_from_log_q(
        log_base_q,
        log_q,
        bit_depth,
        chroma_sampling,
        fti == 0,
        log_isqrt_mean_scale,
      )
    }
  }

  // Computes a quantizer directly from the frame type and base quantizer index,
  // without consideration for rate control.
  fn calc_flat_quantizer(
    base_qi: u8, bit_depth: usize, fti: usize,
  ) -> (i64, i64) {
    // TODO: Rename "quantizer" something that indicates it is a quantizer
    //  index, and move it somewhere more sensible (or choose a better way to
    //  parameterize a "quality" configuration parameter).

    // We use the AC quantizer as the source quantizer since its quantizer
    //  tables have unique entries, while the DC tables do not.
    let ac_quantizer = ac_q(base_qi, 0, bit_depth).get() as i64;
    // Pick the nearest DC entry since an exact match may be unavailable.
    let dc_qi = select_dc_qi(ac_quantizer, bit_depth);
    let dc_quantizer = dc_q(dc_qi, 0, bit_depth).get() as i64;
    // Get the log quantizers as Q57.
    let log_ac_q = blog64(ac_quantizer) - q57(QSCALE + bit_depth as i32 - 8);
    let log_dc_q = blog64(dc_quantizer) - q57(QSCALE + bit_depth as i32 - 8);
    // Target the midpoint of the chosen entries.
    let log_base_q = (log_ac_q + log_dc_q + 1) >> 1;
    // Adjust the quantizer for the frame type, result is Q57:
    let log_q = ((log_base_q + (1i64 << 11)) >> 12) * (MQP_Q12[fti] as i64)
      + DQP_Q57[fti];
    (log_base_q, log_q)
  }

  #[profiling::function]
  pub fn update_state(
    &mut self, bits: i64, fti: usize, show_frame: bool, log_target_q: i64,
    trial: bool, droppable: bool,
  ) -> bool {
    if trial {
      assert!(self.needs_trial_encode(fti));
      assert!(bits > 0);
    }
    let mut dropped = false;
    // Update rate control only if rate control is active.
    if self.target_bitrate > 0 {
      let mut estimated_bits = 0;
      let mut bits = bits;
      let mut droppable = droppable;
      let mut log_scale = q57(-64);
      // Drop frames is also disabled for now in the case of infinite-buffer
      //  two-pass mode.
      if !self.drop_frames
        || fti == FRAME_SUBTYPE_SEF
        || (self.twopass_state == PASS_2
          || self.twopass_state == PASS_2_PLUS_1)
          && !self.frame_metrics.is_empty()
      {
        droppable = false;
      }
      if fti == FRAME_SUBTYPE_SEF {
        debug_assert!(bits == SEF_BITS);
        debug_assert!(show_frame);
        // Please don't make trial encodes of a SEF.
        debug_assert!(!trial);
        estimated_bits = SEF_BITS;
        self.nsef_frames += 1;
      } else {
        let log_q_exp = ((log_target_q + 32) >> 6) * (self.exp[fti] as i64);
        let prev_log_scale = self.log_scale[fti];
        if bits <= 0 {
          // We didn't code any blocks in this frame.
          bits = 0;
          dropped = true;
        // TODO: Adjust VFR rate based on drop count.
        } else {
          // Compute the estimated scale factor for this frame type.
          let log_bits = blog64(bits);
          log_scale = (log_bits - self.log_npixels + log_q_exp).min(q57(16));
          estimated_bits =
            bexp64(prev_log_scale + self.log_npixels - log_q_exp);
          if !trial {
            self.nencoded_frames += 1;
          }
        }
      }
      let log_scale_q24 = q57_to_q24(log_scale);
      // Special two-pass processing.
      if self.twopass_state == PASS_2 || self.twopass_state == PASS_2_PLUS_1 {
        // Pass 2 mode:
        if !trial {
          // Move the current metrics back one frame.
          self.prev_metrics = self.cur_metrics;
          // Back out the last frame's statistics from the sliding window.
          let ftj = self.prev_metrics.fti;
          self.nframes_left[ftj] -= 1;
          self.scale_window_nframes[ftj] -= 1;
          if ftj < FRAME_NSUBTYPES {
            self.scale_window_sum[ftj] -=
              bexp_q24(self.prev_metrics.log_scale_q24);
          }
          if self.prev_metrics.show_frame {
            self.ntus_left -= 1;
            self.scale_window_ntus -= 1;
          }
          // Free the corresponding entry in the circular buffer.
          if !self.frame_metrics.is_empty() {
            self.nframe_metrics -= 1;
            self.frame_metrics_head += 1;
            if self.frame_metrics_head >= self.frame_metrics.len() {
              self.frame_metrics_head = 0;
            }
          }
          // Mark us ready for the next 2-pass packet.
          self.pass2_data_ready = false;
          // Update state, so the user doesn't have to keep calling
          //  twopass_in() after they've fed in all the data when we're using
          //  a finite buffer.
          self.twopass_in(None).unwrap_or(0);
        }
      }
      if self.twopass_state == PASS_1 || self.twopass_state == PASS_2_PLUS_1 {
        // Pass 1 mode: save the metrics for this frame.
        self.prev_metrics.log_scale_q24 = log_scale_q24;
        self.prev_metrics.fti = fti;
        self.prev_metrics.show_frame = show_frame;
        self.pass1_data_retrieved = false;
      }
      // Common to all passes:
      if fti != FRAME_SUBTYPE_SEF && bits > 0 {
        // If this is the first example of the given frame type we've seen,
        //  we immediately replace the default scale factor guess with the
        //  estimate we just computed using the first frame.
        if trial || self.nframes[fti] <= 0 {
          let f = &mut self.scalefilter[fti];
          let x = log_scale_q24;
          f.x[0] = x;
          f.x[1] = x;
          f.y[0] = x;
          f.y[1] = x;
          self.log_scale[fti] = log_scale;
        // TODO: Duplicate regular P frame state for first golden P frame.
        } else {
          // Lengthen the time constant for the inter filters as we collect
          //  more frame statistics, until we reach our target.
          if fti > 0
            && self.inter_delay[fti - 1] < self.inter_delay_target
            && self.nframes[fti] >= self.inter_delay[fti - 1]
          {
            self.inter_delay[fti - 1] += 1;
            self.scalefilter[fti].reinit(self.inter_delay[fti - 1]);
          }
          // Update the low-pass scale filter for this frame type regardless
          //  of whether or not we will ultimately drop this frame.
          self.log_scale[fti] =
            q24_to_q57(self.scalefilter[fti].update(log_scale_q24));
        }
        // If this frame busts our budget, it must be dropped.
        if droppable && self.reservoir_fullness + self.bits_per_tu < bits {
          // TODO: Adjust VFR rate based on drop count.
          bits = 0;
          dropped = true;
        } else {
          // TODO: Update a low-pass filter to estimate the "real" frame rate
          //  taking timestamps and drops into account.
          // This is only done if the frame is coded, as it needs the final
          //  count of dropped frames.
        }
      }
      if !trial {
        // Increment the frame count for filter adaptation purposes.
        if !trial && self.nframes[fti] < i32::MAX {
          self.nframes[fti] += 1;
        }
        self.reservoir_fullness -= bits;
        if show_frame {
          self.reservoir_fullness += self.bits_per_tu;
          // TODO: Properly account for temporal delimiter bits.
        }
        // If we're too quick filling the buffer and overflow is capped, that
        //  rate is lost forever.
        if self.cap_overflow {
          self.reservoir_fullness =
            self.reservoir_fullness.min(self.reservoir_max);
        }
        // If we're too quick draining the buffer and underflow is capped,
        //  don't try to make up that rate later.
        if self.cap_underflow {
          self.reservoir_fullness = self.reservoir_fullness.max(0);
        }
        // Adjust the bias for the real bits we've used.
        self.rate_bias += estimated_bits - bits;
      }
    }
    dropped
  }

  pub const fn needs_trial_encode(&self, fti: usize) -> bool {
    self.target_bitrate > 0 && self.nframes[fti] == 0
  }

  pub(crate) const fn ready(&self) -> bool {
    match self.twopass_state {
      PASS_SINGLE => true,
      PASS_1 => self.pass1_data_retrieved,
      PASS_2 => self.pass2_data_ready,
      _ => self.pass1_data_retrieved && self.pass2_data_ready,
    }
  }

  fn buffer_val(&mut self, val: i64, bytes: usize, cur_pos: usize) -> usize {
    let mut val = val;
    let mut bytes = bytes;
    let mut cur_pos = cur_pos;
    while bytes > 0 {
      bytes -= 1;
      self.pass1_buffer[cur_pos] = val as u8;
      cur_pos += 1;
      val >>= 8;
    }
    cur_pos
  }

  pub(crate) fn select_pass1_log_base_q<T: Pixel>(
    &self, ctx: &ContextInner<T>, output_frameno: u64,
  ) -> i64 {
    assert_eq!(self.twopass_state, PASS_SINGLE);
    self.select_qi(ctx, output_frameno, FRAME_SUBTYPE_I, None, 0).log_base_q
  }

  // Initialize the first pass and emit a placeholder summary
  pub(crate) fn init_first_pass(
    &mut self, maybe_pass1_log_base_q: Option<i64>,
  ) {
    if let Some(pass1_log_base_q) = maybe_pass1_log_base_q {
      assert_eq!(self.twopass_state, PASS_SINGLE);
      // Pick first-pass qi for scale calculations.
      self.pass1_log_base_q = pass1_log_base_q;
    } else {
      debug_assert!(self.twopass_state == PASS_2);
    }
    self.twopass_state += PASS_1;
  }

  // Prepare a placeholder summary
  fn emit_placeholder_summary(&mut self) -> &[u8] {
    // Fill in dummy summary values.
    let mut cur_pos = 0;
    cur_pos = self.buffer_val(TWOPASS_MAGIC as i64, 4, cur_pos);
    cur_pos = self.buffer_val(TWOPASS_VERSION as i64, 4, cur_pos);
    cur_pos = self.buffer_val(0, TWOPASS_HEADER_SZ - 8, cur_pos);
    debug_assert!(cur_pos == TWOPASS_HEADER_SZ);
    self.pass1_data_retrieved = true;
    &self.pass1_buffer[..cur_pos]
  }

  // Frame-specific pass data
  pub(crate) fn emit_frame_data(&mut self) -> Option<&[u8]> {
    let mut cur_pos = 0;
    let fti = self.prev_metrics.fti;
    if fti < FRAME_NSUBTYPES {
      self.scale_sum[fti] += bexp_q24(self.prev_metrics.log_scale_q24);
    }
    if self.prev_metrics.show_frame {
      self.ntus += 1;
    }
    // If we have encoded too many frames, prevent us from reaching the
    //  ready state required to encode more.
    if self.nencoded_frames + self.nsef_frames >= i32::MAX as i64 {
      None?
    }
    cur_pos = self.buffer_val(
      ((self.prev_metrics.show_frame as i64) << 31)
        | self.prev_metrics.fti as i64,
      4,
      cur_pos,
    );
    cur_pos =
      self.buffer_val(self.prev_metrics.log_scale_q24 as i64, 4, cur_pos);
    debug_assert!(cur_pos == TWOPASS_PACKET_SZ);
    self.pass1_data_retrieved = true;
    Some(&self.pass1_buffer[..cur_pos])
  }

  // Summary of the whole encoding process.
  pub(crate) fn emit_summary(&mut self) -> &[u8] {
    let mut cur_pos = 0;
    cur_pos = self.buffer_val(TWOPASS_MAGIC as i64, 4, cur_pos);
    cur_pos = self.buffer_val(TWOPASS_VERSION as i64, 4, cur_pos);
    cur_pos = self.buffer_val(self.ntus as i64, 4, cur_pos);
    for fti in 0..=FRAME_NSUBTYPES {
      cur_pos = self.buffer_val(self.nframes[fti] as i64, 4, cur_pos);
    }
    for fti in 0..FRAME_NSUBTYPES {
      cur_pos = self.buffer_val(self.exp[fti] as i64, 1, cur_pos);
    }
    for fti in 0..FRAME_NSUBTYPES {
      cur_pos = self.buffer_val(self.scale_sum[fti], 8, cur_pos);
    }
    debug_assert!(cur_pos == TWOPASS_HEADER_SZ);
    self.pass1_summary_retrieved = true;
    &self.pass1_buffer[..cur_pos]
  }

  // Emit either summary or frame-specific data depending on the previous call
  pub(crate) fn twopass_out(
    &mut self, done_processing: bool,
  ) -> Option<&[u8]> {
    if !self.pass1_data_retrieved {
      if self.twopass_state != PASS_1 && self.twopass_state != PASS_2_PLUS_1 {
        Some(self.emit_placeholder_summary())
      } else {
        self.emit_frame_data()
      }
    } else if done_processing && !self.pass1_summary_retrieved {
      Some(self.emit_summary())
    } else {
      // The data for this frame has already been retrieved.
      None
    }
  }

  // Initialize the rate control for second pass encoding
  pub(crate) fn init_second_pass(&mut self) {
    if self.twopass_state == PASS_SINGLE || self.twopass_state == PASS_1 {
      // Initialize the second pass.
      self.twopass_state += PASS_2;
      // If the user requested a finite buffer, reserve the space required for
      //  it.
      if self.reservoir_frame_delay_is_set {
        debug_assert!(self.reservoir_frame_delay > 0);
        // reservoir_frame_delay counts in TUs, but RCFrameMetrics are stored
        //  per frame (including Show Existing Frame frames).
        // When re-ordering, we will have more frames than TUs.
        // How many more?
        // That depends on the re-ordering scheme used.
        // Doubling the number of TUs and adding a fixed latency equal to the
        //  maximum number of reference frames we can store should be
        //  sufficient for any reasonable scheme, and keeps this code from
        //  depending too closely on the details of the scheme currently used
        //  by rav1e.
        let nmetrics = (self.reservoir_frame_delay as usize) * 2 + 8;
        self.frame_metrics.reserve_exact(nmetrics);
        self.frame_metrics.resize(nmetrics, RCFrameMetrics::new());
      }
    }
  }

  pub(crate) fn setup_second_pass(&mut self, s: &RCSummary) {
    self.ntus_total = s.ntus;
    self.ntus_left = s.ntus;
    self.nframes_total = s.nframes;
    self.nframes_left = s.nframes;
    self.nframes_total_total = s.nframes.iter().sum();
    if self.frame_metrics.is_empty() {
      self.reservoir_frame_delay = s.ntus;
      self.scale_window_nframes = self.nframes_total;
      self.scale_window_sum = s.scale_sum;
      self.reservoir_max =
        self.bits_per_tu * (self.reservoir_frame_delay as i64);
      self.reservoir_target = (self.reservoir_max + 1) >> 1;
      self.reservoir_fullness = self.reservoir_target;
    } else {
      self.reservoir_frame_delay = self.reservoir_frame_delay.min(s.ntus);
    }
    self.exp = s.exp;
  }

  // Parse the rate control summary
  //
  // It returns the amount of data consumed in the process or
  // an empty error on parsing failure.
  fn twopass_parse_summary(&mut self, buf: &[u8]) -> Result<usize, String> {
    let consumed = self.des.buffer_fill(buf, 0, TWOPASS_HEADER_SZ);
    if self.des.pass2_buffer_fill >= TWOPASS_HEADER_SZ {
      self.des.pass2_buffer_pos = 0;

      let s = self.des.parse_summary()?;

      self.setup_second_pass(&s);

      // Got a valid header.
      // Set up pass 2.
      // Clear the header data from the buffer to make room for the
      //  packet data.
      self.des.pass2_buffer_fill = 0;
    }

    Ok(consumed)
  }

  // Return the size of the first buffer twopass_in expects
  //
  // It is the summary size (constant) + the number of frame data packets
  // (variable depending on the configuration) it needs to starts encoding.
  pub(crate) fn twopass_first_packet_size(&self) -> usize {
    let frames_needed = if !self.frame_metrics.is_empty() {
      // If we're not using whole-file buffering, we need at least one
      //  frame per buffer slot.
      self.reservoir_frame_delay as usize
    } else {
      // Otherwise we need just one.
      1
    };

    TWOPASS_HEADER_SZ + frames_needed * TWOPASS_PACKET_SZ
  }

  // Return the number of frame data packets to be parsed before
  // the encoding process can continue.
  pub(crate) fn twopass_in_frames_needed(&self) -> i32 {
    if self.target_bitrate <= 0 {
      return 0;
    }
    if self.frame_metrics.is_empty() {
      return i32::from(!self.pass2_data_ready);
    }
    let mut cur_scale_window_nframes = 0;
    let mut cur_nframes_left = 0;
    for fti in 0..=FRAME_NSUBTYPES {
      cur_scale_window_nframes += self.scale_window_nframes[fti];
      cur_nframes_left += self.nframes_left[fti];
    }

    (self.reservoir_frame_delay - self.scale_window_ntus)
      .clamp(0, cur_nframes_left - cur_scale_window_nframes)
  }

  pub(crate) fn parse_frame_data_packet(
    &mut self, buf: &[u8],
  ) -> Result<(), String> {
    if buf.len() != TWOPASS_PACKET_SZ {
      return Err("Incorrect buffer size".to_string());
    }

    self.des.buffer_fill(buf, 0, TWOPASS_PACKET_SZ);
    self.des.pass2_buffer_pos = 0;
    let m = self.des.parse_metrics()?;
    self.des.pass2_buffer_fill = 0;

    if self.frame_metrics.is_empty() {
      // We're using a whole-file buffer.
      self.cur_metrics = m;
      self.pass2_data_ready = true;
    } else {
      // Safety check
      let frames_needed = self.twopass_in_frames_needed();

      if frames_needed > 0 {
        if self.nframe_metrics >= self.frame_metrics.len() {
          return Err(
            "Read too many frames without finding enough TUs".to_string(),
          );
        }

        let mut fmi = self.frame_metrics_head + self.nframe_metrics;
        if fmi >= self.frame_metrics.len() {
          fmi -= self.frame_metrics.len();
        }
        self.nframe_metrics += 1;
        self.frame_metrics[fmi] = m;
        // And accumulate the statistics over the window.
        self.scale_window_nframes[m.fti] += 1;
        if m.fti < FRAME_NSUBTYPES {
          self.scale_window_sum[m.fti] += bexp_q24(m.log_scale_q24);
        }
        if m.show_frame {
          self.scale_window_ntus += 1;
        }
        if frames_needed == 1 {
          self.pass2_data_ready = true;
          self.cur_metrics = self.frame_metrics[self.frame_metrics_head];
        }
      } else {
        return Err("No frames needed".to_string());
      }
    }

    Ok(())
  }

  // Parse the rate control per-frame data
  //
  // If no buffer is passed return the amount of data it expects
  // to consume next.
  //
  // If a properly sized buffer is passed it returns the amount of data
  // consumed in the process or an empty error on parsing failure.
  fn twopass_parse_frame_data(
    &mut self, maybe_buf: Option<&[u8]>, mut consumed: usize,
  ) -> Result<usize, String> {
    {
      if self.frame_metrics.is_empty() {
        // We're using a whole-file buffer.
        if let Some(buf) = maybe_buf {
          consumed = self.des.buffer_fill(buf, consumed, TWOPASS_PACKET_SZ);
          if self.des.pass2_buffer_fill >= TWOPASS_PACKET_SZ {
            self.des.pass2_buffer_pos = 0;
            // Read metrics for the next frame.
            self.cur_metrics = self.des.parse_metrics()?;
            // Clear the buffer for the next frame.
            self.des.pass2_buffer_fill = 0;
            self.pass2_data_ready = true;
          }
        } else {
          return Ok(TWOPASS_PACKET_SZ - self.des.pass2_buffer_fill);
        }
      } else {
        // We're using a finite buffer.
        let mut cur_scale_window_nframes = 0;
        let mut cur_nframes_left = 0;

        for fti in 0..=FRAME_NSUBTYPES {
          cur_scale_window_nframes += self.scale_window_nframes[fti];
          cur_nframes_left += self.nframes_left[fti];
        }

        let mut frames_needed = self.twopass_in_frames_needed();
        while frames_needed > 0 {
          if let Some(buf) = maybe_buf {
            consumed = self.des.buffer_fill(buf, consumed, TWOPASS_PACKET_SZ);
            if self.des.pass2_buffer_fill >= TWOPASS_PACKET_SZ {
              self.des.pass2_buffer_pos = 0;
              // Read the metrics for the next frame.
              let m = self.des.parse_metrics()?;
              // Add them to the circular buffer.
              if self.nframe_metrics >= self.frame_metrics.len() {
                return Err(
                  "Read too many frames without finding enough TUs"
                    .to_string(),
                );
              }
              let mut fmi = self.frame_metrics_head + self.nframe_metrics;
              if fmi >= self.frame_metrics.len() {
                fmi -= self.frame_metrics.len();
              }
              self.nframe_metrics += 1;
              self.frame_metrics[fmi] = m;
              // And accumulate the statistics over the window.
              self.scale_window_nframes[m.fti] += 1;
              cur_scale_window_nframes += 1;
              if m.fti < FRAME_NSUBTYPES {
                self.scale_window_sum[m.fti] += bexp_q24(m.log_scale_q24);
              }
              if m.show_frame {
                self.scale_window_ntus += 1;
              }
              frames_needed = (self.reservoir_frame_delay
                - self.scale_window_ntus)
                .clamp(0, cur_nframes_left - cur_scale_window_nframes);
              // Clear the buffer for the next frame.
              self.des.pass2_buffer_fill = 0;
            } else {
              // Go back for more data.
              break;
            }
          } else {
            return Ok(
              TWOPASS_PACKET_SZ * (frames_needed as usize)
                - self.des.pass2_buffer_fill,
            );
          }
        }
        // If we've got all the frames we need, fill in the current metrics.
        // We're ready to go.
        if frames_needed <= 0 {
          self.cur_metrics = self.frame_metrics[self.frame_metrics_head];
          // Mark us ready for the next frame.
          self.pass2_data_ready = true;
        }
      }
    }

    Ok(consumed)
  }

  // If called without a buffer it will return the size of the next
  // buffer it expects.
  //
  // If called with a buffer it will consume it fully.
  // It returns Ok(0) if the buffer had been parsed or Err(())
  // if the buffer hadn't been enough or other errors happened.
  pub(crate) fn twopass_in(
    &mut self, maybe_buf: Option<&[u8]>,
  ) -> Result<usize, String> {
    let mut consumed = 0;
    self.init_second_pass();
    // If we haven't got a valid summary header yet, try to parse one.
    if self.nframes_total[FRAME_SUBTYPE_I] == 0 {
      self.pass2_data_ready = false;
      if let Some(buf) = maybe_buf {
        consumed = self.twopass_parse_summary(buf)?
      } else {
        return Ok(self.twopass_first_packet_size());
      }
    }
    if self.nframes_total[FRAME_SUBTYPE_I] > 0 {
      if self.nencoded_frames + self.nsef_frames
        >= self.nframes_total_total as i64
      {
        // We don't want any more data after the last frame, and we don't want
        //  to allow any more frames to be encoded.
        self.pass2_data_ready = false;
      } else if !self.pass2_data_ready {
        return self.twopass_parse_frame_data(maybe_buf, consumed);
      }
    }
    Ok(consumed)
  }
}
