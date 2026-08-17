// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.
#![deny(missing_docs)]

use std::fmt;
use std::io;
use std::sync::Arc;

use bitstream_io::{BigEndian, BitWrite, BitWriter};

use crate::api::color::*;
use crate::api::config::*;
use crate::api::internal::*;
use crate::api::util::*;
use crate::encoder::*;
use crate::frame::*;
use crate::util::Pixel;

/// The encoder context.
///
/// Contains the encoding state.
pub struct Context<T: Pixel> {
  pub(crate) inner: ContextInner<T>,
  pub(crate) config: EncoderConfig,
  pub(crate) pool: Option<Arc<rayon::ThreadPool>>,
  pub(crate) is_flushing: bool,
}

impl<T: Pixel> Context<T> {
  /// Allocates and returns a new frame.
  ///
  /// # Examples
  ///
  /// ```
  /// use rav1e::prelude::*;
  ///
  /// # fn main() -> Result<(), InvalidConfig> {
  /// let cfg = Config::default();
  /// let ctx: Context<u8> = cfg.new_context()?;
  /// let frame = ctx.new_frame();
  /// # Ok(())
  /// # }
  /// ```
  #[inline]
  pub fn new_frame(&self) -> Frame<T> {
    Frame::new(
      self.config.width,
      self.config.height,
      self.config.chroma_sampling,
    )
  }

  /// Sends the frame for encoding.
  ///
  /// This method adds the frame into the frame queue and runs the first passes
  /// of the look-ahead computation.
  ///
  /// Passing `None` is equivalent to calling [`flush`].
  ///
  /// The caller is responsible for padding the invisible portion of the frame,
  /// if multiple references to the frame are held.
  /// Calling [`Plane::pad()`] after filling each plane or equivalent is required.
  ///
  /// # Errors
  ///
  /// If this method is called with a frame after the encoder has been flushed
  /// or the encoder internal limit is hit (`std::i32::MAX` frames) the
  /// [`EncoderStatus::EnoughData`] error is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use rav1e::prelude::*;
  ///
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # if false {
  /// let cfg = Config::default();
  /// let mut ctx: Context<u8> = cfg.new_context().unwrap();
  /// let f1 = ctx.new_frame();
  /// let f2 = f1.clone();
  /// let info = FrameParameters {
  ///   frame_type_override: FrameTypeOverride::Key,
  ///   opaque: None,
  ///   ..Default::default()
  /// };
  ///
  /// // Send the plain frame data
  /// ctx.send_frame(f1)?;
  /// // Send the data and the per-frame parameters
  /// // In this case the frame is forced to be a keyframe.
  /// ctx.send_frame((f2, info))?;
  /// // Flush the encoder, it is equivalent to a call to `flush()`
  /// ctx.send_frame(None)?;
  /// # }
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// [`flush`]: #method.flush
  /// [`EncoderStatus::EnoughData`]: enum.EncoderStatus.html#variant.EnoughData
  #[inline]
  pub fn send_frame<F>(&mut self, frame: F) -> Result<(), EncoderStatus>
  where
    F: IntoFrame<T>,
  {
    let (frame, params) = frame.into();

    if frame.is_none() {
      if self.is_flushing {
        return Ok(());
      }
      self.inner.limit = Some(self.inner.frame_count);
      self.is_flushing = true;
    } else if self.is_flushing
      || (self.inner.config.still_picture && self.inner.frame_count > 0)
    {
      return Err(EncoderStatus::EnoughData);
    // The rate control can process at most i32::MAX frames
    } else if self.inner.frame_count == i32::MAX as u64 - 1 {
      self.inner.limit = Some(self.inner.frame_count);
      self.is_flushing = true;
    }

    let inner = &mut self.inner;
    let run = move || inner.send_frame(frame, params);

    match &self.pool {
      Some(pool) => pool.install(run),
      None => run(),
    }
  }

  /// Returns the first-pass data of a two-pass encode for the frame that was
  /// just encoded.
  ///
  /// This should be called BEFORE every call to [`receive_packet`] (including
  /// the very first one), even if no packet was produced by the last call to
  /// [`receive_packet`], if any (i.e., [`EncoderStatus::Encoded`] was
  /// returned).  It needs to be called once more after
  /// [`EncoderStatus::LimitReached`] is returned, to retrieve the header that
  /// should be written to the front of the stats file (overwriting the
  /// placeholder header that was emitted at the start of encoding).
  ///
  /// It is still safe to call this function when [`receive_packet`] returns
  /// any other error. It will return `None` instead of returning a duplicate
  /// copy of the previous frame's data.
  ///
  /// [`receive_packet`]: #method.receive_packet
  /// [`EncoderStatus::Encoded`]: enum.EncoderStatus.html#variant.Encoded
  /// [`EncoderStatus::LimitReached`]:
  /// enum.EncoderStatus.html#variant.LimitReached
  #[inline]
  pub fn twopass_out(&mut self) -> Option<&[u8]> {
    self.inner.rc_state.twopass_out(self.inner.done_processing())
  }

  /// Returns the number of bytes of the stats file needed before the next
  /// frame of the second pass in a two-pass encode can be encoded.
  ///
  /// This is a lower bound (more might be required), but if `0` is returned,
  /// then encoding can proceed. This is just a hint to the application, and
  /// does not need to be called for encoding the second pass to work, so long
  /// as the application continues to provide more data to [`twopass_in`] in a
  /// loop until [`twopass_in`] returns `0`.
  ///
  /// [`twopass_in`]: #method.twopass_in
  #[inline]
  pub fn twopass_bytes_needed(&mut self) -> usize {
    self.inner.rc_state.twopass_in(None).unwrap_or(0)
  }

  /// Provides the stats data produced in the first pass of a two-pass encode
  /// to the second pass.
  ///
  /// On success this returns the number of bytes of the data which were
  /// consumed. When encoding the second pass of a two-pass encode, this should
  /// be called repeatedly in a loop before every call to [`receive_packet`]
  /// (including the very first one) until no bytes are consumed, or until
  /// [`twopass_bytes_needed`] returns `0`.
  ///
  /// [`receive_packet`]: #method.receive_packet
  /// [`twopass_bytes_needed`]: #method.twopass_bytes_needed
  ///
  /// # Errors
  ///
  /// Returns `Err(EncoderStatus::Failure)` if the two-pass data is invalid.
  #[inline]
  pub fn twopass_in(&mut self, buf: &[u8]) -> Result<usize, EncoderStatus> {
    self.inner.rc_state.twopass_in(Some(buf)).or(Err(EncoderStatus::Failure))
  }

  /// Encodes the next frame and returns the encoded data.
  ///
  /// This method is where the main encoding work is done.
  ///
  /// # Errors
  ///
  /// May return `Err(EncoderStatus)`, which should be handled by the caller.
  ///
  /// # Examples
  ///
  /// Encoding a single frame:
  ///
  /// ```
  /// use rav1e::prelude::*;
  ///
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # if false {
  /// let cfg = Config::default();
  /// let mut ctx: Context<u8> = cfg.new_context()?;
  /// let frame = ctx.new_frame();
  ///
  /// ctx.send_frame(frame)?;
  /// ctx.flush();
  ///
  /// loop {
  ///     match ctx.receive_packet() {
  ///         Ok(packet) => { /* Mux the packet. */ },
  ///         Err(EncoderStatus::Encoded) => (),
  ///         Err(EncoderStatus::LimitReached) => break,
  ///         Err(err) => Err(err)?,
  ///     }
  /// }
  /// # }
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// Encoding a sequence of frames:
  ///
  /// ```
  /// use std::sync::Arc;
  /// use rav1e::prelude::*;
  ///
  /// fn encode_frames(
  ///     ctx: &mut Context<u8>,
  ///     mut frames: impl Iterator<Item=Frame<u8>>
  /// ) -> Result<(), EncoderStatus> {
  ///     // This is a slightly contrived example, intended to showcase the
  ///     // various statuses that can be returned from receive_packet().
  ///     // Assume that, for example, there are a lot of frames in the
  ///     // iterator, which are produced lazily, so you don't want to send
  ///     // them all in at once as to not exhaust the memory.
  ///     loop {
  ///         match ctx.receive_packet() {
  ///             Ok(packet) => { /* Mux the packet. */ },
  ///             Err(EncoderStatus::Encoded) => {
  ///                 // A frame was encoded without emitting a packet. This is
  ///                 // normal, just proceed as usual.
  ///             },
  ///             Err(EncoderStatus::LimitReached) => {
  ///                 // All frames have been encoded. Time to break out of the
  ///                 // loop.
  ///                 break;
  ///             },
  ///             Err(EncoderStatus::NeedMoreData) => {
  ///                 // The encoder has requested additional frames. Push the
  ///                 // next frame in, or flush the encoder if there are no
  ///                 // frames left (on None).
  ///                 ctx.send_frame(frames.next().map(Arc::new))?;
  ///             },
  ///             Err(EncoderStatus::EnoughData) => {
  ///                 // Since we aren't trying to push frames after flushing,
  ///                 // this should never happen in this example.
  ///                 unreachable!();
  ///             },
  ///             Err(EncoderStatus::NotReady) => {
  ///                 // We're not doing two-pass encoding, so this can never
  ///                 // occur.
  ///                 unreachable!();
  ///             },
  ///             Err(EncoderStatus::Failure) => {
  ///                 return Err(EncoderStatus::Failure);
  ///             },
  ///         }
  ///     }
  ///
  ///     Ok(())
  /// }
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// #   if false {
  /// #     let mut enc = EncoderConfig::default();
  /// #     // So it runs faster.
  /// #     enc.width = 16;
  /// #     enc.height = 16;
  /// #     let cfg = Config::new().with_encoder_config(enc);
  /// #     let mut ctx: Context<u8> = cfg.new_context()?;
  /// #
  /// #     let frames = vec![ctx.new_frame(); 4].into_iter();
  /// #     encode_frames(&mut ctx, frames);
  /// #   }
  /// #   Ok(())
  /// # }
  /// ```
  #[inline]
  pub fn receive_packet(&mut self) -> Result<Packet<T>, EncoderStatus> {
    let inner = &mut self.inner;
    let mut run = move || inner.receive_packet();

    match &self.pool {
      Some(pool) => pool.install(run),
      None => run(),
    }
  }

  /// Flushes the encoder.
  ///
  /// Flushing signals the end of the video. After the encoder has been
  /// flushed, no additional frames are accepted.
  ///
  /// # Panics
  ///
  /// Panics if `send_frame` returns an `Err`.
  /// This should never happen when calling it with `None`
  /// and indicates a development error.
  #[inline]
  pub fn flush(&mut self) {
    self.send_frame(None).unwrap();
  }

  /// Produces a sequence header matching the current encoding context.
  ///
  /// Its format is compatible with the AV1 Matroska and ISOBMFF specification.
  /// Note that the returned header does not include any config OBUs which are
  /// required for some uses. See [the specification].
  ///
  /// [the specification]:
  /// https://aomediacodec.github.io/av1-isobmff/#av1codecconfigurationbox-section
  ///
  /// # Panics
  ///
  /// Panics if the header cannot be written in memory. This is unrecoverable,
  /// and usually indicates the system is out of memory.
  #[inline]
  pub fn container_sequence_header(&self) -> Vec<u8> {
    fn sequence_header_inner(seq: &Sequence) -> io::Result<Vec<u8>> {
      let mut buf = Vec::new();

      {
        let mut bw = BitWriter::endian(&mut buf, BigEndian);
        bw.write_bit(true)?; // marker
        bw.write::<7, u8>(1)?; // version
        bw.write::<3, u8>(seq.profile)?;
        bw.write::<5, u8>(31)?; // level
        bw.write_bit(false)?; // tier
        bw.write_bit(seq.bit_depth > 8)?; // high_bitdepth
        bw.write_bit(seq.bit_depth == 12)?; // twelve_bit
        bw.write_bit(seq.chroma_sampling == ChromaSampling::Cs400)?; // monochrome
        bw.write_bit(seq.chroma_sampling != ChromaSampling::Cs444)?; // chroma_subsampling_x
        bw.write_bit(seq.chroma_sampling == ChromaSampling::Cs420)?; // chroma_subsampling_y
        bw.write::<2, u8>(0)?; // chroma_sample_position
        bw.write::<3, u8>(0)?; // reserved
        bw.write_bit(false)?; // initial_presentation_delay_present

        bw.write::<4, u8>(0)?; // reserved
      }

      Ok(buf)
    }

    let seq = Sequence::new(&self.config);

    sequence_header_inner(&seq).unwrap()
  }
}

/// Rate Control Data
pub enum RcData {
  /// A Rate Control Summary Packet
  ///
  /// It is emitted once, after the encoder is flushed.
  ///
  /// It contains a summary of the rate control information for the
  /// encoding process that just terminated.
  Summary(Box<[u8]>),
  /// A Rate Control Frame-specific Packet
  ///
  /// It is emitted every time a frame is processed.
  ///
  /// The information contained is required to encode its matching
  /// frame in a second pass encoding.
  Frame(Box<[u8]>),
}

impl<T: Pixel> Context<T> {
  /// Return the Rate Control Summary Packet size
  ///
  /// It is useful mainly to preserve space when saving
  /// both Rate Control Summary and Frame Packets in a single file.
  pub fn rc_summary_size(&self) -> usize {
    crate::rate::TWOPASS_HEADER_SZ
  }

  /// Return the first pass data
  ///
  /// Call it after `receive_packet`, it returns a packet or the encoder
  /// lifecycle statuses [`EncoderStatus::Encoded`] and
  /// [`EncoderStatus::LimitReached`].
  ///
  /// [`EncoderStatus::Encoded`]: enum.EncoderStatus.html#variant.Encoded
  /// [`EncoderStatus::LimitReached`]:
  /// enum.EncoderStatus.html#variant.LimitReached
  ///
  /// It will return a `RcData::Summary` once the encoder is flushed.
  pub fn rc_receive_pass_data(&mut self) -> Option<RcData> {
    if self.inner.done_processing() && self.inner.rc_state.pass1_data_retrieved
    {
      let data = self.inner.rc_state.emit_summary();
      Some(RcData::Summary(data.to_vec().into_boxed_slice()))
    } else if self.inner.rc_state.pass1_data_retrieved {
      None
    } else if let Some(data) = self.inner.rc_state.emit_frame_data() {
      Some(RcData::Frame(data.to_vec().into_boxed_slice()))
    } else {
      unreachable!(
        "The encoder received more frames than its internal limit allows"
      )
    }
  }

  /// Lower bound number of pass data packets required to progress the
  /// encoding process.
  ///
  /// It should be called iteratively until it returns 0.
  pub fn rc_second_pass_data_required(&self) -> usize {
    if self.inner.done_processing() {
      0
    } else {
      self.inner.rc_state.twopass_in_frames_needed() as usize
    }
  }

  /// Feed the first pass Rate Control data to the encoder,
  /// Frame-specific Packets only.
  ///
  /// Call it before `receive_packet()`
  ///
  /// # Errors
  ///
  /// Returns `EncoderStatus::Failure` if the data provided is incorrect
  pub fn rc_send_pass_data(
    &mut self, data: &[u8],
  ) -> Result<(), EncoderStatus> {
    self
      .inner
      .rc_state
      .parse_frame_data_packet(data)
      .map_err(|_| EncoderStatus::Failure)
  }
}

impl<T: Pixel> fmt::Debug for Context<T> {
  fn fmt(
    &self, f: &mut fmt::Formatter<'_>,
  ) -> std::result::Result<(), fmt::Error> {
    write!(
      f,
      "{{ \
        config: {:?}, \
        is_flushing: {}, \
      }}",
      self.config, self.is_flushing,
    )
  }
}
