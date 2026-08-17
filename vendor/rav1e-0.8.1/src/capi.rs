// Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved
// Copyright (c) 2017-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

//! # C API for rav1e
//!
//! [rav1e](https://github.com/xiph/rav1e/) is an [AV1](https://aomediacodec.github.io/av1-spec/)
//! encoder written in [Rust](https://rust-lang.org)
//!
//! This is the C-compatible API
#![deny(missing_docs)]
// Basically everything will be unsafe since this is a FFI
#![allow(clippy::undocumented_unsafe_blocks)]
// const extern fns are unstable
#![allow(clippy::missing_const_for_fn)]

use std::slice;
use std::sync::Arc;

use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;

use libc::ptrdiff_t;
use libc::size_t;

use num_derive::*;
use num_traits::cast::FromPrimitive;

use scan_fmt::scan_fmt;

use crate::api::SceneDetectionSpeed;
use crate::prelude as rav1e;

type PixelRange = rav1e::PixelRange;
type ChromaSamplePosition = rav1e::ChromaSamplePosition;
type ChromaSampling = rav1e::ChromaSampling;
type MatrixCoefficients = rav1e::MatrixCoefficients;
type ColorPrimaries = rav1e::ColorPrimaries;
type TransferCharacteristics = rav1e::TransferCharacteristics;
type Rational = rav1e::Rational;
type FrameTypeOverride = rav1e::FrameTypeOverride;
type FrameOpaqueCb = Option<extern fn(*mut c_void)>;
type T35 = rav1e::T35;

#[derive(Clone)]
enum FrameInternal {
  U8(Arc<rav1e::Frame<u8>>),
  U16(Arc<rav1e::Frame<u16>>),
}

impl From<rav1e::Frame<u8>> for FrameInternal {
  fn from(f: rav1e::Frame<u8>) -> FrameInternal {
    FrameInternal::U8(Arc::new(f))
  }
}

impl From<rav1e::Frame<u16>> for FrameInternal {
  fn from(f: rav1e::Frame<u16>) -> FrameInternal {
    FrameInternal::U16(Arc::new(f))
  }
}

impl From<Arc<rav1e::Frame<u8>>> for FrameInternal {
  fn from(f: Arc<rav1e::Frame<u8>>) -> FrameInternal {
    FrameInternal::U8(f)
  }
}

impl From<Arc<rav1e::Frame<u16>>> for FrameInternal {
  fn from(f: Arc<rav1e::Frame<u16>>) -> FrameInternal {
    FrameInternal::U16(f)
  }
}

struct FrameOpaque {
  opaque: *mut c_void,
  cb: FrameOpaqueCb,
}

unsafe impl Send for FrameOpaque {}
unsafe impl Sync for FrameOpaque {}

impl Default for FrameOpaque {
  fn default() -> Self {
    FrameOpaque { opaque: std::ptr::null_mut(), cb: None }
  }
}

impl Drop for FrameOpaque {
  fn drop(&mut self) {
    let FrameOpaque { opaque, cb } = self;
    if let Some(cb) = cb {
      cb(*opaque);
    }
  }
}

/// Raw video Frame
///
/// It can be allocated through `rav1e_frame_new()`,
/// populated using `rav1e_frame_fill_plane()`,
/// and freed using `rav1e_frame_unref()`.
pub struct Frame {
  fi: FrameInternal,
  frame_type: FrameTypeOverride,
  opaque: Option<FrameOpaque>,
  t35_metadata: Vec<T35>,
}

/// Status that can be returned by encoder functions.
#[repr(C)]
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
pub enum EncoderStatus {
  /// Normal operation.
  Success = 0,
  /// The encoder needs more data to produce an output packet.
  ///
  /// May be emitted by `rav1e_receive_packet` when frame reordering is
  /// enabled.
  NeedMoreData,
  /// There are enough frames in the queue.
  ///
  /// May be emitted by `rav1e_send_frame` when trying to send a frame after
  /// the encoder has been flushed or the internal queue is full.
  EnoughData,
  /// The encoder has already produced the number of frames requested.
  ///
  /// May be emitted by `rav1e_receive_packet` after a flush request had been
  /// processed or the frame limit had been reached.
  LimitReached,
  /// A Frame had been encoded but not emitted yet.
  Encoded,
  /// Generic fatal error.
  Failure = -1,
  /// A frame was encoded in the first pass of a 2-pass encode, but its stats
  /// data was not retrieved with `rav1e_twopass_out`, or not enough stats data
  /// was provided in the second pass of a 2-pass encode to encode the next
  /// frame.
  NotReady = -2,
}

impl EncoderStatus {
  const fn to_c(&self) -> *const u8 {
    use self::EncoderStatus::*;
    match self {
      Success => "Normal operation\0".as_ptr(),
      NeedMoreData => "The encoder needs more data to produce an output packet\0".as_ptr(),
      EnoughData => "There are enough frames in the queue\0".as_ptr(),
      LimitReached => "The encoder has already produced the number of frames requested\0".as_ptr(),
      Encoded => "A Frame had been encoded but not emitted yet\0".as_ptr(),
      Failure => "Generic fatal error\0".as_ptr(),
      NotReady => "First-pass stats data not retrieved or not enough second-pass data provided\0".as_ptr(),
    }
  }
}

impl From<Option<rav1e::EncoderStatus>> for EncoderStatus {
  fn from(status: Option<rav1e::EncoderStatus>) -> Self {
    match status {
      None => EncoderStatus::Success,
      Some(s) => match s {
        rav1e::EncoderStatus::NeedMoreData => EncoderStatus::NeedMoreData,
        rav1e::EncoderStatus::EnoughData => EncoderStatus::EnoughData,
        rav1e::EncoderStatus::LimitReached => EncoderStatus::LimitReached,
        rav1e::EncoderStatus::Encoded => EncoderStatus::Encoded,
        rav1e::EncoderStatus::Failure => EncoderStatus::Failure,
        rav1e::EncoderStatus::NotReady => EncoderStatus::NotReady,
      },
    }
  }
}

/// Encoder configuration
///
/// Instantiate it using `rav1e_config_default()` and fine-tune it using
/// `rav1e_config_parse()`.
///
/// Use `rav1e_config_unref()` to free its memory.
pub struct Config {
  cfg: rav1e::Config,
}

enum EncContext {
  U8(rav1e::Context<u8>),
  U16(rav1e::Context<u16>),
}

impl EncContext {
  fn new_frame(&self) -> FrameInternal {
    match self {
      EncContext::U8(ctx) => ctx.new_frame().into(),
      EncContext::U16(ctx) => ctx.new_frame().into(),
    }
  }
  fn send_frame(
    &mut self, frame: Option<FrameInternal>, frame_type: FrameTypeOverride,
    opaque: Option<rav1e::Opaque>, t35_metadata: Box<[T35]>,
  ) -> Result<(), rav1e::EncoderStatus> {
    let info = rav1e::FrameParameters {
      frame_type_override: frame_type,
      opaque,
      t35_metadata,
    };
    if let Some(frame) = frame {
      match (self, frame) {
        (EncContext::U8(ctx), FrameInternal::U8(ref f)) => {
          ctx.send_frame((f.clone(), info))
        }
        (EncContext::U16(ctx), FrameInternal::U16(ref f)) => {
          ctx.send_frame((f.clone(), info))
        }
        _ => Err(rav1e::EncoderStatus::Failure),
      }
    } else {
      match self {
        EncContext::U8(ctx) => ctx.send_frame(None),
        EncContext::U16(ctx) => ctx.send_frame(None),
      }
    }
  }

  fn receive_packet(&mut self) -> Result<Packet, rav1e::EncoderStatus> {
    fn receive_packet<T: rav1e::Pixel>(
      ctx: &mut rav1e::Context<T>,
    ) -> Result<Packet, rav1e::EncoderStatus>
    where
      FrameInternal: From<Arc<v_frame::frame::Frame<T>>>,
    {
      ctx.receive_packet().map(|p| {
        let mut p = std::mem::ManuallyDrop::new(p);
        let opaque = p.opaque.take().map_or_else(std::ptr::null_mut, |o| {
          let mut opaque = o.downcast::<FrameOpaque>().unwrap();
          opaque.cb = None;
          opaque.opaque
        });
        let p = std::mem::ManuallyDrop::into_inner(p);
        let rav1e::Packet {
          data, rec, source, input_frameno, frame_type, ..
        } = p;
        let len = data.len();
        let data = Box::into_raw(data.into_boxed_slice()) as *const u8;
        let rec = if let Some(rec) = rec {
          let rec = FrameInternal::from(rec);
          Box::into_raw(Box::new(Frame {
            fi: rec,
            frame_type: FrameTypeOverride::No,
            opaque: None,
            t35_metadata: Vec::new(),
          }))
        } else {
          std::ptr::null_mut()
        };
        let source = if let Some(source) = source {
          let source = FrameInternal::from(source);
          Box::into_raw(Box::new(Frame {
            fi: source,
            frame_type: FrameTypeOverride::No,
            opaque: None,
            t35_metadata: Vec::new(),
          }))
        } else {
          std::ptr::null_mut()
        };
        Packet { data, rec, source, len, input_frameno, frame_type, opaque }
      })
    }
    match self {
      EncContext::U8(ctx) => receive_packet(ctx),
      EncContext::U16(ctx) => receive_packet(ctx),
    }
  }

  fn container_sequence_header(&self) -> Vec<u8> {
    match self {
      EncContext::U8(ctx) => ctx.container_sequence_header(),
      EncContext::U16(ctx) => ctx.container_sequence_header(),
    }
  }

  fn twopass_bytes_needed(&mut self) -> usize {
    match self {
      EncContext::U8(ctx) => ctx.twopass_bytes_needed(),
      EncContext::U16(ctx) => ctx.twopass_bytes_needed(),
    }
  }

  fn twopass_in(&mut self, buf: &[u8]) -> Result<usize, rav1e::EncoderStatus> {
    match self {
      EncContext::U8(ctx) => ctx.twopass_in(buf),
      EncContext::U16(ctx) => ctx.twopass_in(buf),
    }
  }

  fn twopass_out(&mut self) -> Option<&[u8]> {
    match self {
      EncContext::U8(ctx) => ctx.twopass_out(),
      EncContext::U16(ctx) => ctx.twopass_out(),
    }
  }

  fn rc_summary_size(&self) -> usize {
    match self {
      EncContext::U8(ctx) => ctx.rc_summary_size(),
      EncContext::U16(ctx) => ctx.rc_summary_size(),
    }
  }

  fn rc_receive_pass_data(&mut self) -> Option<rav1e::RcData> {
    match self {
      EncContext::U8(ctx) => ctx.rc_receive_pass_data(),
      EncContext::U16(ctx) => ctx.rc_receive_pass_data(),
    }
  }

  fn rc_second_pass_data_required(&self) -> usize {
    match self {
      EncContext::U8(ctx) => ctx.rc_second_pass_data_required(),
      EncContext::U16(ctx) => ctx.rc_second_pass_data_required(),
    }
  }

  fn rc_send_pass_data(
    &mut self, data: &[u8],
  ) -> Result<(), rav1e::EncoderStatus> {
    match self {
      EncContext::U8(ctx) => ctx.rc_send_pass_data(data),
      EncContext::U16(ctx) => ctx.rc_send_pass_data(data),
    }
  }

  fn config(&self) -> rav1e::EncoderConfig {
    // Ideally this would return a reference instead of cloning,
    // but that would require a breaking change in the CAPI.
    match self {
      EncContext::U8(ctx) => ctx.config.clone(),
      EncContext::U16(ctx) => ctx.config.clone(),
    }
  }
}

/// Encoder context
///
/// Contains the encoding state, it is created by `rav1e_context_new()` using an
/// Encoder configuration.
///
/// Use `rav1e_context_unref()` to free its memory.
pub struct Context {
  ctx: EncContext,
  last_err: Option<rav1e::EncoderStatus>,
}

type FrameType = rav1e::FrameType;

/// Encoded Packet
///
/// The encoded packets are retrieved using `rav1e_receive_packet()`.
///
/// Use `rav1e_packet_unref()` to free its memory.
#[repr(C)]
pub struct Packet {
  /// Encoded data buffer
  pub data: *const u8,
  /// Encoded data buffer size
  pub len: size_t,
  /// Frame sequence number
  pub input_frameno: u64,
  /// Frame type
  pub frame_type: FrameType,
  /// User provided opaque data
  pub opaque: *mut c_void,
  /// The reconstruction of the shown frame.
  /// This is freed automatically by `rav1e_packet_unref()`.
  pub rec: *mut Frame,
  /// The Reference Frame
  /// This is freed automatically by `rav1e_packet_unref()`.
  pub source: *mut Frame,
}

/// Version information as presented in `[package]` `version`.
///
/// e.g. `0.1.0`
///
/// Can be parsed by [semver](https://crates.io/crates/semver).
/// This returns the version of the loaded library, regardless
/// of which version the library user was built against.
#[no_mangle]
pub unsafe extern fn rav1e_version_short() -> *const c_char {
  concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

static FULL_VERSION_C: std::sync::OnceLock<CString> =
  std::sync::OnceLock::new();

/// Version information with the information
/// provided by `git describe --tags`.
///
/// e.g. `0.1.0 (v0.1.0-1-g743d464)`
///
/// This returns the version of the loaded library, regardless
/// of which version the library user was built against.
#[no_mangle]
pub unsafe extern fn rav1e_version_full() -> *const c_char {
  FULL_VERSION_C
    .get_or_init(|| {
      CString::new(crate::version::full()).expect("Bogus version data")
    })
    .as_ptr() as *const c_char
}

/// Simple Data
///
/// Use `rav1e_data_unref()` to free its memory.
#[repr(C)]
pub struct Data {
  /// Pointer to the data buffer
  pub data: *const u8,
  /// Data buffer size
  pub len: size_t,
}

/// Free a `RaData` buffer
#[no_mangle]
pub unsafe extern fn rav1e_data_unref(data: *mut Data) {
  if !data.is_null() {
    let data = Box::from_raw(data);
    let _ = Vec::from_raw_parts(
      data.data as *mut u8,
      data.len as usize,
      data.len as usize,
    );
  }
}

/// Create a `RaConfig` filled with default parameters.
#[no_mangle]
pub unsafe extern fn rav1e_config_default() -> *mut Config {
  let cfg = rav1e::Config::default();

  let c = Box::new(Config { cfg });

  Box::into_raw(c)
}

unsafe fn decode_slice<'a>(
  data: *mut *const u8, len: *mut size_t,
) -> (c_int, Option<&'a [u8]>) {
  if *len < 8 {
    return (8, None);
  }

  let buf = slice::from_raw_parts(*data, *len as usize);
  let (len_bytes, rest) = buf.split_at(std::mem::size_of::<u64>());
  let buf_len = u64::from_be_bytes(len_bytes.try_into().unwrap()) as usize;
  let full_len = buf_len + 8;
  if buf_len > rest.len() {
    return (full_len as c_int, None);
  }

  *len -= full_len;
  *data = (*data).offset(full_len.try_into().unwrap());

  (0, Some(&rest[..buf_len]))
}

/// Setup a second pass rate control using the provided summary
///
/// Passing `NULL` data resets the rate control settings.
///
/// If additional data is required, pointer and len stay unchanged, otherwise
/// they are updated.
///
/// Return:
/// `0` on success
/// `> 0` if the buffer has to be larger
/// `< 0` on failure
#[no_mangle]
// Panic can never occur here
#[allow(clippy::missing_panics_doc)]
pub unsafe extern fn rav1e_config_set_rc_summary(
  cfg: *mut Config, data: *mut *const u8, len: *mut size_t,
) -> c_int {
  if data.is_null() {
    (*cfg).cfg.rate_control.summary = None;

    return 0;
  }

  let (needed, maybe_buf) = decode_slice(data, len);

  if maybe_buf.is_none() {
    return needed;
  }

  let summary = rav1e::RateControlSummary::from_slice(maybe_buf.unwrap()).ok();
  if summary.is_none() {
    -1
  } else {
    (*cfg).cfg.rate_control.summary = summary;

    0
  }
}

/// Request to emit pass data
///
/// Set emit to 0 to not emit pass data, non-zero to emit pass data.
///
#[no_mangle]
pub unsafe extern fn rav1e_config_set_emit_data(
  cfg: *mut Config, emit: c_int,
) {
  (*cfg).cfg.rate_control.emit_pass_data = emit != 0;
}

/// Set the display aspect ratio of the stream
///
/// Needed for anamorphic video.
#[no_mangle]
pub unsafe extern fn rav1e_config_set_sample_aspect_ratio(
  cfg: *mut Config, sample_aspect_ratio: Rational,
) {
  (*cfg).cfg.enc.sample_aspect_ratio = sample_aspect_ratio
}

/// Set the time base of the stream
///
/// Needed for rate control.
#[no_mangle]
pub unsafe extern fn rav1e_config_set_time_base(
  cfg: *mut Config, time_base: Rational,
) {
  (*cfg).cfg.enc.time_base = time_base
}

/// Set pixel format of the stream.
///
/// Supported values for `subsampling` and `chroma_pos` are defined by the
/// enum types `RaChromaSampling` and `RaChromaSamplePosition` respectively.
/// Valid values for `pixel_range` are 0 and 1.
///
/// Returns a negative value on error or 0.
#[no_mangle]
pub unsafe extern fn rav1e_config_set_pixel_format(
  cfg: *mut Config, bit_depth: u8, subsampling: ChromaSampling,
  chroma_pos: ChromaSamplePosition, pixel_range: PixelRange,
) -> c_int {
  if bit_depth != 8 && bit_depth != 10 && bit_depth != 12 {
    return -1;
  }
  (*cfg).cfg.enc.bit_depth = bit_depth as usize;

  let subsampling_val =
    std::mem::transmute::<ChromaSampling, i32>(subsampling);
  if ChromaSampling::from_i32(subsampling_val).is_none() {
    return -1;
  }
  (*cfg).cfg.enc.chroma_sampling = subsampling;

  let chroma_pos_val =
    std::mem::transmute::<ChromaSamplePosition, i32>(chroma_pos);
  if ChromaSamplePosition::from_i32(chroma_pos_val).is_none() {
    return -1;
  }
  (*cfg).cfg.enc.chroma_sample_position = chroma_pos;

  let pixel_range_val = std::mem::transmute::<PixelRange, i32>(pixel_range);
  if PixelRange::from_i32(pixel_range_val).is_none() {
    return -1;
  }
  (*cfg).cfg.enc.pixel_range = pixel_range;

  0
}

/// Set color properties of the stream.
///
/// Supported values are defined by the enum types
/// `RaMatrixCoefficients`, `RaColorPrimaries`, and `RaTransferCharacteristics`
/// respectively.
///
/// Return a negative value on error or 0.
#[no_mangle]
pub unsafe extern fn rav1e_config_set_color_description(
  cfg: *mut Config, matrix: MatrixCoefficients, primaries: ColorPrimaries,
  transfer: TransferCharacteristics,
) -> c_int {
  (*cfg).cfg.enc.color_description = Some(rav1e::ColorDescription {
    matrix_coefficients: matrix,
    color_primaries: primaries,
    transfer_characteristics: transfer,
  });

  if (*cfg).cfg.enc.color_description.is_some() {
    0
  } else {
    -1
  }
}

/// Set the content light level information for HDR10 streams.
///
/// Return a negative value on error or 0.
#[no_mangle]
pub unsafe extern fn rav1e_config_set_content_light(
  cfg: *mut Config, max_content_light_level: u16,
  max_frame_average_light_level: u16,
) -> c_int {
  (*cfg).cfg.enc.content_light = Some(rav1e::ContentLight {
    max_content_light_level,
    max_frame_average_light_level,
  });

  if (*cfg).cfg.enc.content_light.is_some() {
    0
  } else {
    -1
  }
}

/// Set the mastering display information for HDR10 streams.
///
/// `primaries` and `white_point` arguments are `RaChromaticityPoint`,
/// containing 0.16 fixed point values.
/// `max_luminance` is a 24.8 fixed point value.
/// `min_luminance` is a 18.14 fixed point value.
///
/// Returns a negative value on error or 0.
/// cbindgen:ptrs-as-arrays=[[primaries;3]]
#[no_mangle]
pub unsafe extern fn rav1e_config_set_mastering_display(
  cfg: *mut Config, primaries: *const rav1e::ChromaticityPoint,
  white_point: rav1e::ChromaticityPoint, max_luminance: u32,
  min_luminance: u32,
) -> c_int {
  let primaries = *(primaries as *const [rav1e::ChromaticityPoint; 3]);

  (*cfg).cfg.enc.mastering_display = Some(rav1e::MasteringDisplay {
    primaries,
    white_point,
    max_luminance,
    min_luminance,
  });

  if (*cfg).cfg.enc.mastering_display.is_some() {
    0
  } else {
    -1
  }
}

/// Free the `RaConfig`.
#[no_mangle]
pub unsafe extern fn rav1e_config_unref(cfg: *mut Config) {
  if !cfg.is_null() {
    let _ = Box::from_raw(cfg);
  }
}

unsafe fn option_match(
  cfg: *mut Config, key: *const c_char, value: *const c_char,
) -> Result<(), ()> {
  let key = CStr::from_ptr(key).to_str().map_err(|_| ())?;
  let value = CStr::from_ptr(value).to_str().map_err(|_| ())?;
  let enc = &mut (*cfg).cfg.enc;

  match key {
    "width" => enc.width = value.parse().map_err(|_| ())?,
    "height" => enc.height = value.parse().map_err(|_| ())?,
    "speed" => {
      enc.speed_settings =
        rav1e::SpeedSettings::from_preset(value.parse().map_err(|_| ())?)
    }

    "threads" => (*cfg).cfg.threads = value.parse().map_err(|_| ())?,

    "tiles" => enc.tiles = value.parse().map_err(|_| ())?,
    "tile_rows" => enc.tile_rows = value.parse().map_err(|_| ())?,
    "tile_cols" => enc.tile_cols = value.parse().map_err(|_| ())?,

    "tune" => enc.tune = value.parse().map_err(|_| ())?,
    "quantizer" => enc.quantizer = value.parse().map_err(|_| ())?,
    "min_quantizer" => enc.min_quantizer = value.parse().map_err(|_| ())?,
    "bitrate" => enc.bitrate = value.parse().map_err(|_| ())?,

    "key_frame_interval" => {
      enc.set_key_frame_interval(
        enc.min_key_frame_interval,
        value.parse().map_err(|_| ())?,
      );
    }
    "min_key_frame_interval" => {
      enc.set_key_frame_interval(
        value.parse().map_err(|_| ())?,
        enc.max_key_frame_interval,
      );
    }
    "switch_frame_interval" => {
      enc.switch_frame_interval = value.parse().map_err(|_| ())?
    }
    "reservoir_frame_delay" => {
      enc.reservoir_frame_delay = Some(value.parse().map_err(|_| ())?)
    }
    "rdo_lookahead_frames" => {
      enc.speed_settings.rdo_lookahead_frames =
        value.parse().map_err(|_| ())?
    }
    "low_latency" => enc.low_latency = value.parse().map_err(|_| ())?,
    "enable_timing_info" => {
      enc.enable_timing_info = value.parse().map_err(|_| ())?
    }
    "still_picture" => enc.still_picture = value.parse().map_err(|_| ())?,
    "no_scene_detection" => {
      if value.parse::<bool>().map_err(|_| ())? {
        enc.speed_settings.scene_detection_mode = SceneDetectionSpeed::None;
      }
    }

    "level" => {
      enc.level_idx = match value {
        "auto" => None,
        "unconstrained" => Some(31),
        _ => {
          let (major, minor) =
            scan_fmt!(value, "{}.{}", u8, u8).map_err(|_| ())?;
          if major > 7 || minor > 3 {
            return Err(());
          }
          Some(((major - 2) << 2) + minor)
        }
      };
    }

    _ => return Err(()),
  }

  Ok(())
}

/// Set a configuration parameter using its key and value as string.
///
/// Available keys and values
/// - `"width"`: width of the frame, default `640`
/// - `"height"`: height of the frame, default `480`
/// - `"speed"`: 0-10, default `6`
/// - `"threads"`: maximum number of threads to be used, default auto
/// - `"tune"`: `"psnr"` or `"psychovisual"`, default `"psychovisual"`
/// - `"quantizer"`: 0-255, default `100`
/// - `"tiles"`: total number of tiles desired (0 denotes auto), default `0`
/// - `"tile_rows"`: number of tiles horizontally (must be a power of two, overridden by tiles if present), default `0`
/// - `"tile_cols"`: number of tiles vertically (must be a power of two, overridden by tiles if present), default `0`
/// - `"min_quantizer"`: minimum allowed base quantizer to use in bitrate mode, default `0`
/// - `"bitrate"`: target bitrate for the bitrate mode (required for two pass mode), default `0`
/// - `"key_frame_interval"`: maximum interval between two keyframes, default `240`
/// - `"min_key_frame_interval"`: minimum interval between two keyframes, default `12`
/// - `"switch_frame_interval"`: interval between switch frames, default `0`
/// - `"reservoir_frame_delay"`: number of temporal units over which to distribute the reservoir usage, default `None`
/// - `"rdo_lookahead_frames"`: number of frames to read ahead for the RDO lookahead computation, default `40`
/// - `"low_latency"`: flag to enable low latency mode, default `false`
/// - `"enable_timing_info"`: flag to enable signaling timing info in the bitstream, default `false`
/// - `"still_picture"`: flag for still picture mode, default `false`
/// - `"no_scene_detection"`: flag to disable scene detection, default `false`
///
/// Return a negative value on error or 0.
#[no_mangle]
pub unsafe extern fn rav1e_config_parse(
  cfg: *mut Config, key: *const c_char, value: *const c_char,
) -> c_int {
  if option_match(cfg, key, value) == Ok(()) {
    0
  } else {
    -1
  }
}

/// Set a configuration parameter using its key and value as integer.
///
/// Available keys and values are the same as `rav1e_config_parse()`
///
/// Return a negative value on error or 0.
#[no_mangle]
// Panic can never occur here
#[allow(clippy::missing_panics_doc)]
pub unsafe extern fn rav1e_config_parse_int(
  cfg: *mut Config, key: *const c_char, value: c_int,
) -> c_int {
  let val = CString::new(value.to_string()).unwrap();
  if option_match(cfg, key, val.as_ptr()) == Ok(()) {
    0
  } else {
    config_parse_bool(cfg, key, value)
  }
}

unsafe fn config_parse_bool(
  cfg: *mut Config, key: *const c_char, value: c_int,
) -> c_int {
  let val = CString::new(if value != 0 { "true" } else { "false" }).unwrap();
  if option_match(cfg, key, val.as_ptr()) == Ok(()) {
    0
  } else {
    -1
  }
}

/// Generate a new encoding context from a populated encoder configuration
///
/// Multiple contexts can be generated through it.
/// Returns `Null` if context creation failed, e.g. by passing
/// an invalid `Config`.
#[no_mangle]
pub unsafe extern fn rav1e_context_new(cfg: *const Config) -> *mut Context {
  let cfg = &(*cfg).cfg;
  let enc = &cfg.enc;

  let ctx = match enc.bit_depth {
    8 => cfg.new_context().map(EncContext::U8),
    _ => cfg.new_context().map(EncContext::U16),
  };

  if let Ok(ctx) = ctx {
    Box::into_raw(Box::new(Context { ctx, last_err: None }))
  } else {
    std::ptr::null_mut()
  }
}

/// Free the `RaContext`.
#[no_mangle]
pub unsafe extern fn rav1e_context_unref(ctx: *mut Context) {
  if !ctx.is_null() {
    let _ = Box::from_raw(ctx);
  }
}

/// Produce a new frame from the encoding context
///
/// It must be populated using `rav1e_frame_fill_plane()`.
///
/// The frame is reference counted and must be released passing it to `rav1e_frame_unref()`,
/// see `rav1e_send_frame()`.
#[no_mangle]
pub unsafe extern fn rav1e_frame_new(ctx: *const Context) -> *mut Frame {
  let fi = (*ctx).ctx.new_frame();
  let frame_type = rav1e::FrameTypeOverride::No;
  let f = Frame { fi, frame_type, opaque: None, t35_metadata: Vec::new() };
  let frame = Box::new(f);

  Box::into_raw(frame)
}

/// Free the `RaFrame`.
#[no_mangle]
pub unsafe extern fn rav1e_frame_unref(frame: *mut Frame) {
  if !frame.is_null() {
    let _ = Box::from_raw(frame);
  }
}

/// Overrides the encoders frame type decision for a frame
///
/// Must be called before `rav1e_send_frame()` if used.
#[no_mangle]
pub unsafe extern fn rav1e_frame_set_type(
  frame: *mut Frame, frame_type: FrameTypeOverride,
) -> c_int {
  let frame_type_val =
    std::mem::transmute::<FrameTypeOverride, i32>(frame_type);
  if FrameTypeOverride::from_i32(frame_type_val).is_none() {
    return -1;
  }
  (*frame).frame_type = frame_type;

  0
}

/// Register an opaque data and a destructor to the frame
///
/// It takes the ownership of its memory:
/// - it will relinquish the ownership to the context if
///   `rav1e_send_frame` is called.
/// - it will call the destructor if `rav1e_frame_unref` is called
///   otherwise.
#[no_mangle]
pub unsafe extern fn rav1e_frame_set_opaque(
  frame: *mut Frame, opaque: *mut c_void, cb: FrameOpaqueCb,
) {
  if opaque.is_null() {
    (*frame).opaque = None;
  } else {
    (*frame).opaque = Some(FrameOpaque { opaque, cb });
  }
}

/// Add generic T35 metadata to a frame
///
/// The buffer will be copied into the frame and can be freed
/// immediately after this call.
///
/// Can be called multiple times to add multiple T35 metadata
/// blocks.
#[no_mangle]
pub unsafe extern fn rav1e_frame_add_t35_metadata(
  frame: *mut Frame, country_code: u8, country_code_extension_byte: u8,
  data: *const u8, data_len: size_t,
) {
  (*frame).t35_metadata.push(T35 {
    country_code,
    country_code_extension_byte,
    data: slice::from_raw_parts(data, data_len).into(),
  });
}

/// Retrieve the first-pass data of a two-pass encode for the frame that was
/// just encoded. This should be called BEFORE every call to `rav1e_receive_packet()`
/// (including the very first one), even if no packet was produced by the
/// last call to `rav1e_receive_packet`, if any (i.e., `RA_ENCODER_STATUS_ENCODED`
/// was returned). It needs to be called once more after
/// `RA_ENCODER_STATUS_LIMIT_REACHED` is returned, to retrieve the header that
/// should be written to the front of the stats file (overwriting the
/// placeholder header that was emitted at the start of encoding).
///
/// It is still safe to call this function when `rav1e_receive_packet()` returns any
/// other error. It will return `NULL` instead of returning a duplicate copy
/// of the previous frame's data.
///
/// Must be freed with `rav1e_data_unref()`.
#[no_mangle]
// Panic can never occur here
#[allow(clippy::missing_panics_doc)]
pub unsafe extern fn rav1e_twopass_out(ctx: *mut Context) -> *mut Data {
  let buf = (*ctx).ctx.twopass_out();

  if buf.is_none() {
    return std::ptr::null_mut();
  }

  let v = buf.unwrap().to_vec();
  Box::into_raw(Box::new(Data {
    len: v.len(),
    data: Box::into_raw(v.into_boxed_slice()) as *mut u8,
  }))
}

/// Rate Control Data
#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum RcDataKind {
  /// A Rate Control Summary Packet
  ///
  /// It is emitted once, after the encoder is flushed.
  ///
  /// It contains a summary of the rate control information for the
  /// encoding process that just terminated.
  Summary,
  /// A Rate Control Frame-specific Packet
  ///
  /// It is emitted every time a frame is processed.
  ///
  /// The information contained is required to encode its matching
  /// frame in a second pass encoding.
  Frame,
  /// There is no pass data available for now
  ///
  /// This is emitted if `rav1e_rc_receive_pass_data` is called more
  /// often than it should.
  Empty,
}

/// Return the Rate Control Summary Packet size
///
/// It is useful mainly to preserve space when saving
/// both Rate Control Summary and Frame Packets in a single file
#[no_mangle]
pub unsafe extern fn rav1e_rc_summary_size(ctx: *const Context) -> size_t {
  (*ctx).ctx.rc_summary_size() as size_t + 8
}

/// Return the first pass data
///
/// Call it after `rav1e_receive_packet()` returns a normal condition status:
/// - `EncoderStatus::Encoded`,
/// - `EncoderStatus::Success`,
/// - `EncoderStatus::LimitReached`.
///
/// use `rav1e_data_unref()` to free the data.
///
/// It will return a `RcDataKind::Summary` once the encoder is flushed.
#[no_mangle]
pub unsafe extern fn rav1e_rc_receive_pass_data(
  ctx: *mut Context, data: *mut *mut Data,
) -> RcDataKind {
  use crate::api::RcData::*;
  let (buf, kind) = match (*ctx).ctx.rc_receive_pass_data() {
    Some(Summary(data)) => (data, RcDataKind::Summary),
    Some(Frame(data)) => (data, RcDataKind::Frame),
    None => return RcDataKind::Empty,
  };

  let mut full_buf = Vec::with_capacity(buf.len() + 8);

  full_buf.extend_from_slice(&(buf.len() as u64).to_be_bytes());
  full_buf.extend_from_slice(&buf);

  let full_buf = full_buf.into_boxed_slice();

  *data = Box::into_raw(Box::new(Data {
    len: full_buf.len(),
    data: Box::into_raw(full_buf) as *mut u8,
  }));

  kind
}

/// Number of pass data packets required to progress the encoding process.
///
/// At least that number of packets must be passed before the encoder can
/// progress.
///
/// Stop feeding-in pass data packets once the function returns 0.
///
/// ``` c
/// while (rav1e_rc_second_pass_data_required(ctx) > 0) {
///   int more = rav1e_rc_send_pass_data(ctx, &data, &len);
///   if (more > 0) {
///      refill(&data, &len);
///   } else if (more < 0) {
///     goto fail;
///   }
/// }
/// ```
///
#[no_mangle]
pub unsafe extern fn rav1e_rc_second_pass_data_required(
  ctx: *const Context,
) -> i32 {
  (*ctx).ctx.rc_second_pass_data_required() as i32
}

/// Feed the first pass Rate Control data to the encoder,
/// Frame-specific Packets only.
///
/// Call it before `receive_packet()`
///
/// If additional data is required, pointer and len stay unchanged, otherwise
/// they are updated.
///
/// Returns:
/// - `0` on success,
/// - `> 0` the amount of bytes needed
/// - `< 0` on unrecoverable failure
#[no_mangle]
// Panic can never occur here
#[allow(clippy::missing_panics_doc)]
pub unsafe extern fn rav1e_rc_send_pass_data(
  ctx: *mut Context, data: *mut *const u8, len: *mut size_t,
) -> c_int {
  let (need, maybe_buf) = decode_slice(data, len);

  if maybe_buf.is_none() {
    return need;
  }

  let ret = (*ctx)
    .ctx
    .rc_send_pass_data(maybe_buf.unwrap())
    .map(|_v| None)
    .unwrap_or_else(Some);

  (*ctx).last_err = ret;

  if ret.is_some() {
    -1
  } else {
    0
  }
}

/// Ask how many bytes of the stats file are needed before the next frame
/// of the second pass in a two-pass encode can be encoded. This is a lower
/// bound (more might be required), but if 0 is returned, then encoding can
/// proceed. This is just a hint to the application, and does not need to
/// be called for encoding the second pass to work, so long as the
/// application continues to provide more data to `rav1e_twopass_in()` in a loop
/// until `rav1e_twopass_in()` returns 0.
#[no_mangle]
pub unsafe extern fn rav1e_twopass_bytes_needed(ctx: *mut Context) -> size_t {
  (*ctx).ctx.twopass_bytes_needed() as size_t
}

/// Provide stats data produced in the first pass of a two-pass encode to the
/// second pass. On success this returns the number of bytes of that data
/// which were consumed. When encoding the second pass of a two-pass encode,
/// this should be called repeatedly in a loop before every call to
/// `rav1e_receive_packet()` (including the very first one) until no bytes are
/// consumed, or until `twopass_bytes_needed()` returns 0. Returns -1 on failure.
#[no_mangle]
pub unsafe extern fn rav1e_twopass_in(
  ctx: *mut Context, buf: *mut u8, buf_size: size_t,
) -> c_int {
  let buf_slice = slice::from_raw_parts(buf, buf_size as usize);
  let r = (*ctx).ctx.twopass_in(buf_slice);
  match r {
    Ok(v) => v as c_int,
    Err(v) => {
      (*ctx).last_err = Some(v);
      -1
    }
  }
}

/// Send the frame for encoding
///
/// The function increases the frame internal reference count and it can be passed multiple
/// times to different `rav1e_send_frame()` with a caveat:
///
/// The opaque data, if present, will be moved from the `Frame` to the `Context`
/// and returned by `rav1e_receive_packet` in the `Packet` `opaque` field or
/// the destructor will be called on `rav1e_context_unref` if the frame is
/// still pending in the encoder.
///
/// Returns:
/// - `0` on success,
/// - `> 0` if the input queue is full
/// - `< 0` on unrecoverable failure
#[no_mangle]
pub unsafe extern fn rav1e_send_frame(
  ctx: *mut Context, frame: *mut Frame,
) -> EncoderStatus {
  if !frame.is_null() {
    let rav1e::EncoderConfig { width, height, chroma_sampling, .. } =
      (*ctx).ctx.config();
    let planes = if chroma_sampling == ChromaSampling::Cs400 { 1 } else { 3 };
    match (*frame).fi {
      FrameInternal::U8(ref mut f) => {
        rav1e_frame_pad_internal(f, planes, width, height)
      }
      FrameInternal::U16(ref mut f) => {
        rav1e_frame_pad_internal(f, planes, width, height)
      }
    }
  }

  let frame_internal =
    if frame.is_null() { None } else { Some((*frame).fi.clone()) };
  let frame_type = if frame.is_null() {
    rav1e::FrameTypeOverride::No
  } else {
    (*frame).frame_type
  };

  let maybe_opaque = if frame.is_null() {
    None
  } else {
    (*frame).opaque.take().map(rav1e::Opaque::new)
  };

  let t35_metadata = if frame.is_null() {
    Box::new([])
  } else {
    mem::take(&mut (*frame).t35_metadata).into_boxed_slice()
  };

  let ret = (*ctx)
    .ctx
    .send_frame(frame_internal, frame_type, maybe_opaque, t35_metadata)
    .map(|_v| None)
    .unwrap_or_else(Some);

  (*ctx).last_err = ret;

  ret.into()
}

/// Return the last encoder status
#[no_mangle]
pub unsafe extern fn rav1e_last_status(ctx: *const Context) -> EncoderStatus {
  (*ctx).last_err.into()
}

/// Return a static string matching the `EncoderStatus` variant.
///
#[no_mangle]
pub unsafe extern fn rav1e_status_to_str(
  status: EncoderStatus,
) -> *const c_char {
  if EncoderStatus::from_i32(std::mem::transmute(status)).is_none() {
    return std::ptr::null();
  }

  status.to_c() as *const c_char
}

/// Receive encoded data
///
/// Returns:
/// - `0` on success
/// - `> 0` if additional frame data is required
/// - `< 0` on unrecoverable failure
#[no_mangle]
pub unsafe extern fn rav1e_receive_packet(
  ctx: *mut Context, pkt: *mut *mut Packet,
) -> EncoderStatus {
  let ret = (*ctx)
    .ctx
    .receive_packet()
    .map(|packet| {
      *pkt = Box::into_raw(Box::new(packet));
      None
    })
    .unwrap_or_else(Some);

  (*ctx).last_err = ret;

  ret.into()
}

/// Free the `RaPacket`.
#[no_mangle]
pub unsafe extern fn rav1e_packet_unref(pkt: *mut Packet) {
  if !pkt.is_null() {
    let pkt = Box::from_raw(pkt);
    let _ = Vec::from_raw_parts(
      pkt.data as *mut u8,
      pkt.len as usize,
      pkt.len as usize,
    );
    rav1e_frame_unref(pkt.rec);
    rav1e_frame_unref(pkt.source);
  }
}

/// Produce a sequence header matching the current encoding context
///
/// Its format is compatible with the AV1 Matroska and ISOBMFF specification.
///
/// Use `rav1e_data_unref()` to free it.
#[no_mangle]
pub unsafe extern fn rav1e_container_sequence_header(
  ctx: *const Context,
) -> *mut Data {
  let buf = (*ctx).ctx.container_sequence_header();

  Box::into_raw(Box::new(Data {
    len: buf.len(),
    data: Box::into_raw(buf.into_boxed_slice()) as *mut u8,
  }))
}

fn rav1e_frame_fill_plane_internal<T: rav1e::Pixel>(
  f: &mut Arc<rav1e::Frame<T>>, plane: c_int, data_slice: &[u8],
  stride: ptrdiff_t, bytewidth: c_int,
) {
  let input = Arc::get_mut(f).unwrap();
  input.planes[plane as usize].copy_from_raw_u8(
    data_slice,
    stride as usize,
    bytewidth as usize,
  );
}

fn rav1e_frame_pad_internal<T: rav1e::Pixel>(
  f: &mut Arc<rav1e::Frame<T>>, planes: usize, width: usize, height: usize,
) {
  if let Some(ref mut input) = Arc::get_mut(f) {
    for plane in input.planes[..planes].iter_mut() {
      plane.pad(width, height);
    }
  }
}

fn rav1e_frame_extract_plane_internal<T: rav1e::Pixel>(
  f: &Arc<rav1e::Frame<T>>, plane: c_int, data_slice: &mut [u8],
  stride: ptrdiff_t, bytewidth: c_int,
) {
  f.planes[plane as usize].copy_to_raw_u8(
    data_slice,
    stride as usize,
    bytewidth as usize,
  );
}

/// Fill a frame plane
///
/// Currently the frame contains 3 planes, the first is luminance followed by
/// chrominance.
///
/// The data is copied and this function has to be called for each plane.
///
/// `frame`: A frame provided by `rav1e_frame_new()`
/// `plane`: The index of the plane starting from 0
/// `data`: The data to be copied
/// `data_len`: Length of the buffer
/// `stride`: Plane line in bytes, including padding
/// `bytewidth`: Number of bytes per component, either 1 or 2
#[no_mangle]
pub unsafe extern fn rav1e_frame_fill_plane(
  frame: *mut Frame, plane: c_int, data: *const u8, data_len: size_t,
  stride: ptrdiff_t, bytewidth: c_int,
) {
  let data_slice = slice::from_raw_parts(data, data_len as usize);

  match (*frame).fi {
    FrameInternal::U8(ref mut f) => {
      rav1e_frame_fill_plane_internal(f, plane, data_slice, stride, bytewidth)
    }
    FrameInternal::U16(ref mut f) => {
      rav1e_frame_fill_plane_internal(f, plane, data_slice, stride, bytewidth)
    }
  }
}

/// Extract a frame plane
///
/// This is the reverse of `rav1e_frame_fill_plane()`, primarily used for
/// extracting the source and reconstruction data from a `RaPacket`.
///
/// Currently the frame contains 3 planes, the first is luminance followed by
/// chrominance.
///
/// The data is copied out of the frame for a single plane.
///
/// `frame`: A frame provided inside a packet returned by `rav1e_receive_packet()`
/// `plane`: The index of the plane starting from 0
/// `data`: The destination for the data
/// `data_len`: Length of the buffer
/// `stride`: Plane line in bytes, including padding
/// `bytewidth`: Number of bytes per component, either 1 or 2
#[no_mangle]
pub unsafe extern fn rav1e_frame_extract_plane(
  frame: *const Frame, plane: c_int, data: *mut u8, data_len: size_t,
  stride: ptrdiff_t, bytewidth: c_int,
) {
  let data_slice = slice::from_raw_parts_mut(data, data_len as usize);

  match (*frame).fi {
    FrameInternal::U8(ref f) => rav1e_frame_extract_plane_internal(
      f, plane, data_slice, stride, bytewidth,
    ),
    FrameInternal::U16(ref f) => rav1e_frame_extract_plane_internal(
      f, plane, data_slice, stride, bytewidth,
    ),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  use std::ffi::CString;

  #[test]
  fn forward_opaque() {
    unsafe {
      let rac = rav1e_config_default();
      let w = CString::new("width").unwrap();
      rav1e_config_parse_int(rac, w.as_ptr(), 64);
      let h = CString::new("height").unwrap();
      rav1e_config_parse_int(rac, h.as_ptr(), 64);
      let s = CString::new("speed").unwrap();
      rav1e_config_parse_int(rac, s.as_ptr(), 10);
      let l = CString::new("level").unwrap();
      let lo = CString::new("6.2").unwrap();
      rav1e_config_parse(rac, l.as_ptr(), lo.as_ptr());

      let rax = rav1e_context_new(rac);

      let f = rav1e_frame_new(rax);

      let pixels = [42; 64 * 64];
      rav1e_frame_fill_plane(f, 0, pixels.as_ptr(), pixels.len(), 64, 1);

      for i in 0..30 {
        let v = Box::new(i as u8);
        extern fn cb(o: *mut c_void) {
          let v = unsafe { Box::from_raw(o as *mut u8) };
          eprintln!("Would free {}", v);
        }
        rav1e_frame_set_opaque(f, Box::into_raw(v) as *mut c_void, Some(cb));
        rav1e_send_frame(rax, f);
      }

      rav1e_send_frame(rax, std::ptr::null_mut());

      for _ in 0..15 {
        let mut p: *mut Packet = std::ptr::null_mut();
        let ret = rav1e_receive_packet(rax, &mut p);

        if ret == EncoderStatus::Success {
          let mut source = vec![1; 64 * 64];
          rav1e_frame_extract_plane(
            (*p).source,
            0,
            source.as_mut_ptr(),
            64 * 64,
            64,
            1,
          );
          assert_eq!(source, vec![42; 64 * 64]);
          let v = Box::from_raw((*p).opaque as *mut u8);
          eprintln!("Opaque {}", v);
        }

        if ret == EncoderStatus::LimitReached {
          break;
        }
      }

      let v = Box::new(42u64);
      extern fn cb(o: *mut c_void) {
        let v = unsafe { Box::from_raw(o as *mut u64) };
        eprintln!("Would free {}", v);
      }
      rav1e_frame_set_opaque(f, Box::into_raw(v) as *mut c_void, Some(cb));

      // 42 would be freed after this
      rav1e_frame_unref(f);
      // 15 - reorder delay .. 29 would be freed after this
      rav1e_context_unref(rax);
      rav1e_config_unref(rac);
    }
  }

  #[test]
  fn two_pass_encoding() {
    unsafe {
      let rac = rav1e_config_default();
      let w = CString::new("width").unwrap();
      rav1e_config_parse_int(rac, w.as_ptr(), 64);
      let h = CString::new("height").unwrap();
      rav1e_config_parse_int(rac, h.as_ptr(), 64);
      let s = CString::new("speed").unwrap();
      rav1e_config_parse_int(rac, s.as_ptr(), 10);
      let s = CString::new("bitrate").unwrap();
      rav1e_config_parse_int(rac, s.as_ptr(), 1000);
      rav1e_config_set_emit_data(rac, 1);

      let rax = rav1e_context_new(rac);
      let f = rav1e_frame_new(rax);

      let pixels = [42; 64 * 64];
      rav1e_frame_fill_plane(f, 0, pixels.as_ptr(), pixels.len(), 64, 1);

      for _ in 0..10 {
        rav1e_send_frame(rax, f);
      }

      rav1e_send_frame(rax, std::ptr::null_mut());

      let mut frame_data = std::collections::VecDeque::new();
      let mut summary: *mut Data = std::ptr::null_mut();

      loop {
        let mut p: *mut Packet = std::ptr::null_mut();
        let ret = rav1e_receive_packet(rax, &mut p);
        rav1e_packet_unref(p);
        if ret == EncoderStatus::LimitReached {
          let kind = rav1e_rc_receive_pass_data(rax, &mut summary);
          assert_eq!(kind, RcDataKind::Summary);
          eprintln!("Got rc summary {} bytes", (*summary).len);
          break;
        } else if ret == EncoderStatus::Encoded
          || ret == EncoderStatus::Success
        {
          let mut p: *mut Data = std::ptr::null_mut();
          let kind = rav1e_rc_receive_pass_data(rax, &mut p);
          assert_eq!(kind, RcDataKind::Frame);
          eprintln!("Got rc frame data {} bytes", (*p).len);
          frame_data.push_back(p);
        }
      }

      rav1e_config_set_emit_data(rac, 0);
      let mut data = (*summary).data;
      let mut len = (*summary).len;
      let ret = rav1e_config_set_rc_summary(rac, &mut data, &mut len);
      assert_eq!(ret, 0);

      rav1e_data_unref(summary);

      for _ in 0..10 {
        rav1e_send_frame(rax, f);
      }

      rav1e_send_frame(rax, std::ptr::null_mut());

      loop {
        let mut p: *mut Packet = std::ptr::null_mut();
        while rav1e_rc_second_pass_data_required(rax) > 0 {
          let d = frame_data.pop_front().unwrap();
          let mut data = (*d).data;
          let mut len = (*d).len;
          rav1e_rc_send_pass_data(rax, &mut data, &mut len);
          rav1e_data_unref(d);
        }

        let ret = rav1e_receive_packet(rax, &mut p);
        rav1e_packet_unref(p);
        if ret == EncoderStatus::LimitReached {
          break;
        }
      }

      rav1e_frame_unref(f);
      rav1e_context_unref(rax);
      rav1e_config_unref(rac);
    }
  }

  #[test]
  fn invalid_level() {
    unsafe {
      let rac = rav1e_config_default();
      let l = CString::new("level").unwrap();
      let lo = CString::new("8.3").unwrap();
      assert_eq!(rav1e_config_parse(rac, l.as_ptr(), lo.as_ptr()), -1);
      rav1e_config_unref(rac);
    }
  }
}
