// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.
#![deny(missing_docs)]

use crate::frame::*;
use crate::serialize::{Deserialize, Serialize};
use crate::stats::EncoderStats;
use crate::util::Pixel;

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use thiserror::*;

/// Opaque type to be passed from Frame to Packet
#[derive(Debug)]
pub struct Opaque(Box<dyn Any + Send + Sync>);

impl Opaque {
  /// Wrap a type in the opaque struct
  pub fn new<T: Any + Send + Sync>(t: T) -> Self {
    Opaque(Box::new(t) as Box<dyn Any + Send + Sync>)
  }

  /// Attempt to downcast the opaque to a concrete type.
  ///
  /// # Errors
  ///
  /// Returns `Err(Self)` if the value could not be downcast to `T`.
  pub fn downcast<T: Any + Send + Sync>(self) -> Result<Box<T>, Opaque> {
    if self.0.is::<T>() {
      // SAFETY: We verified the type of `T` before this cast.
      unsafe {
        let raw: *mut (dyn Any + Send + Sync) = Box::into_raw(self.0);
        Ok(Box::from_raw(raw as *mut T))
      }
    } else {
      Err(self)
    }
  }
}

// TODO: use the num crate?
/// A rational number.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Rational {
  /// Numerator.
  pub num: u64,
  /// Denominator.
  pub den: u64,
}

impl Rational {
  /// Creates a rational number from the given numerator and denominator.
  pub const fn new(num: u64, den: u64) -> Self {
    Rational { num, den }
  }

  /// Returns a rational number that is the reciprocal of the given one.
  pub const fn from_reciprocal(reciprocal: Self) -> Self {
    Rational { num: reciprocal.den, den: reciprocal.num }
  }

  /// Returns the rational number as a floating-point number.
  pub fn as_f64(self) -> f64 {
    self.num as f64 / self.den as f64
  }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for Rational {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    (self.num, self.den).serialize(serializer)
  }
}

#[cfg(feature = "serialize")]
impl<'a> serde::Deserialize<'a> for Rational {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    let (num, den) = serde::Deserialize::deserialize(deserializer)?;

    Ok(Rational::new(num, den))
  }
}

/// Possible types of a frame.
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub enum FrameType {
  /// Key frame.
  KEY,
  /// Inter-frame.
  INTER,
  /// Intra-only frame.
  INTRA_ONLY,
  /// Switching frame.
  SWITCH,
}

impl FrameType {
  /// Returns whether frame can have inter blocks
  #[inline]
  pub fn has_inter(self) -> bool {
    self == FrameType::INTER || self == FrameType::SWITCH
  }
  /// Returns whether frame is only intra blocks
  #[inline]
  pub fn all_intra(self) -> bool {
    self == FrameType::KEY || self == FrameType::INTRA_ONLY
  }
}

impl fmt::Display for FrameType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use self::FrameType::*;
    match self {
      KEY => write!(f, "Key frame"),
      INTER => write!(f, "Inter frame"),
      INTRA_ONLY => write!(f, "Intra only frame"),
      SWITCH => write!(f, "Switching frame"),
    }
  }
}

/// A single T.35 metadata packet.
#[derive(Clone, Debug, Default)]
pub struct T35 {
  /// Country code.
  pub country_code: u8,
  /// Country code extension bytes (if `country_code` == 0xFF)
  pub country_code_extension_byte: u8,
  /// T.35 payload.
  pub data: Box<[u8]>,
}

/// Status that can be returned by [`Context`] functions.
///
/// [`Context`]: struct.Context.html
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum EncoderStatus {
  /// The encoder needs more data to produce an output packet.
  ///
  /// May be emitted by [`Context::receive_packet()`] when frame reordering is
  /// enabled.
  ///
  /// [`Context::receive_packet()`]: struct.Context.html#method.receive_packet
  #[error("need more data")]
  NeedMoreData,
  /// There are enough frames in the queue.
  ///
  /// May be emitted by [`Context::send_frame()`] when trying to send a frame
  /// after the encoder has been flushed.
  ///
  /// [`Context::send_frame()`]: struct.Context.html#method.send_frame
  #[error("enough data")]
  EnoughData,
  /// The encoder has already produced the number of frames requested.
  ///
  /// May be emitted by [`Context::receive_packet()`] after a flush request had
  /// been processed or the frame limit had been reached.
  ///
  /// [`Context::receive_packet()`]: struct.Context.html#method.receive_packet
  #[error("limit reached")]
  LimitReached,
  /// A frame had been encoded but not emitted yet.
  #[error("encoded")]
  Encoded,
  /// Generic fatal error.
  #[error("failure")]
  Failure,
  /// A frame was encoded in the first pass of a 2-pass encode, but its stats
  /// data was not retrieved with [`Context::twopass_out()`], or not enough
  /// stats data was provided in the second pass of a 2-pass encode to encode
  /// the next frame.
  ///
  /// [`Context::twopass_out()`]: struct.Context.html#method.twopass_out
  #[error("not ready")]
  NotReady,
}

/// Represents a packet.
///
/// A packet contains one shown frame together with zero or more additional
/// frames.
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet<T: Pixel> {
  /// The packet data.
  pub data: Vec<u8>,
  /// The reconstruction of the shown frame.
  #[cfg_attr(feature = "serialize", serde(skip))]
  pub rec: Option<Arc<Frame<T>>>,
  /// The Reference Frame
  #[cfg_attr(feature = "serialize", serde(skip))]
  pub source: Option<Arc<Frame<T>>>,
  /// The number of the input frame corresponding to the one shown frame in the
  /// TU stored in this packet. Since AV1 does not explicitly reorder frames,
  /// these will increase sequentially.
  // TODO: When we want to add VFR support, we will need a more explicit time
  // stamp here.
  pub input_frameno: u64,
  /// Type of the shown frame.
  pub frame_type: FrameType,
  /// QP selected for the frame.
  pub qp: u8,
  /// Block-level encoding stats for the frame
  pub enc_stats: EncoderStats,
  /// Optional user-provided opaque data
  #[cfg_attr(feature = "serialize", serde(skip))]
  pub opaque: Option<Opaque>,
}

impl<T: Pixel> PartialEq for Packet<T> {
  fn eq(&self, other: &Self) -> bool {
    self.data == other.data
      && self.input_frameno == other.input_frameno
      && self.frame_type == other.frame_type
      && self.qp == other.qp
  }
}

impl<T: Pixel> fmt::Display for Packet<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Frame {} - {} - {} bytes",
      self.input_frameno,
      self.frame_type,
      self.data.len()
    )
  }
}

/// Types which can be converted into frames.
///
/// This trait is used in [`Context::send_frame`] to allow for passing in
/// frames with optional frame parameters and optionally frames wrapped in
/// `Arc` (to allow for zero-copy, since the encoder uses frames in `Arc`
/// internally).
///
/// [`Context::send_frame`]: struct.Context.html#method.send_frame
pub trait IntoFrame<T: Pixel> {
  /// Converts the type into a tuple of frame and parameters.
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>);
}

impl<T: Pixel> IntoFrame<T> for Option<Arc<Frame<T>>> {
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>) {
    (self, None)
  }
}

impl<T: Pixel> IntoFrame<T> for Arc<Frame<T>> {
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>) {
    (Some(self), None)
  }
}

impl<T: Pixel> IntoFrame<T> for (Arc<Frame<T>>, FrameParameters) {
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>) {
    (Some(self.0), Some(self.1))
  }
}

impl<T: Pixel> IntoFrame<T> for (Arc<Frame<T>>, Option<FrameParameters>) {
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>) {
    (Some(self.0), self.1)
  }
}

impl<T: Pixel> IntoFrame<T> for Frame<T> {
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>) {
    (Some(Arc::new(self)), None)
  }
}

impl<T: Pixel> IntoFrame<T> for (Frame<T>, FrameParameters) {
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>) {
    (Some(Arc::new(self.0)), Some(self.1))
  }
}

impl<T: Pixel> IntoFrame<T> for (Frame<T>, Option<FrameParameters>) {
  fn into(self) -> (Option<Arc<Frame<T>>>, Option<FrameParameters>) {
    (Some(Arc::new(self.0)), self.1)
  }
}
