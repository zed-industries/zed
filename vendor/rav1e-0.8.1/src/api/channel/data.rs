// Copyright (c) 2018-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::api::color::*;
use crate::api::config::EncoderConfig;
use crate::api::context::RcData;
use crate::api::util::*;
use crate::encoder::*;
use crate::frame::*;
use crate::util::Pixel;

use bitstream_io::{BigEndian, BitWrite, BitWriter};
use crossbeam::channel::{Receiver, Sender};
use thiserror::Error;

use std::io;
use std::sync::Arc;

/// An error returned from the `send` methods.
///
/// The message could not be sent because the channel is disconnected.
///
/// The error contains the message so it can be recovered.
#[derive(PartialEq, Eq, Clone, Copy, Error)]
#[error("sending on a disconnected channel")]
pub struct SendError<T>(pub T);

/// An error returned from the `try_send` methods.
///
/// The error contains the message being sent so it can be recovered.
#[derive(PartialEq, Eq, Clone, Copy, Error)]
pub enum TrySendError<T> {
  /// The message could not be sent because the channel is full.
  #[error("sending on a full channel")]
  Full(T),

  /// The message could not be sent because the channel is disconnected.
  #[error("sending on a disconnected channel")]
  Disconnected(T),
}

/// An error returned from the `recv` methods.
///
/// A message could not be received because the channel is empty and disconnected.
///
#[derive(PartialEq, Eq, Clone, Copy, Debug, Error)]
#[error("receiving on an empty and disconnected channel")]
pub struct RecvError;

/// An error returned from the `try_recv` methods.
///
#[derive(PartialEq, Eq, Clone, Copy, Debug, Error)]
pub enum TryRecvError {
  /// A message could not be received because the channel is empty.
  #[error("receiving on an empty channel")]
  Empty,

  /// The message could not be received because the channel is empty and disconnected.
  #[error("receiving on an empty and disconnected channel")]
  Disconnected,
}

impl<T> SendError<T> {
  fn from(value: crossbeam::channel::SendError<T>) -> Self {
    Self(value.0)
  }
}

impl<T> TrySendError<T> {
  fn from(value: crossbeam::channel::TrySendError<T>) -> Self {
    use crossbeam::channel::TrySendError::*;
    match value {
      Full(v) => TrySendError::Full(v),
      Disconnected(v) => TrySendError::Disconnected(v),
    }
  }
}
impl RecvError {
  fn from(_: crossbeam::channel::RecvError) -> Self {
    RecvError
  }
}

impl TryRecvError {
  fn from(value: crossbeam::channel::TryRecvError) -> Self {
    use crossbeam::channel::TryRecvError::*;
    match value {
      Empty => TryRecvError::Empty,
      Disconnected => TryRecvError::Disconnected,
    }
  }
}

/// Endpoint to send previous-pass statistics data
pub struct RcDataSender {
  pub(crate) sender: Sender<RcData>,
  pub(crate) limit: u64,
  pub(crate) count: u64,
}

impl RcDataSender {
  pub(crate) fn new(limit: u64, sender: Sender<RcData>) -> RcDataSender {
    Self { sender, limit, count: 0 }
  }

  /// # Errors
  ///
  /// - `TrySendError::Full` if the message could not be sent because the channel is full.
  /// - `TrySendError::Disconnected` if the message could not be sent
  ///   because the channel is disconnected.
  pub fn try_send(
    &mut self, data: RcData,
  ) -> Result<(), TrySendError<RcData>> {
    if self.limit <= self.count {
      Err(TrySendError::Full(data))
    } else {
      let r = self.sender.try_send(data).map_err(TrySendError::from);
      if r.is_ok() {
        self.count += 1;
      }
      r
    }
  }

  /// # Errors
  ///
  /// - `SendError` if the message could not be sent because the channel is disconnected.
  pub fn send(&mut self, data: RcData) -> Result<(), SendError<RcData>> {
    if self.limit <= self.count {
      Err(SendError(data))
    } else {
      let r = self.sender.send(data).map_err(SendError::from);
      if r.is_ok() {
        self.count += 1;
      }
      r
    }
  }
  pub fn len(&self) -> usize {
    self.sender.len()
  }
  pub fn is_empty(&self) -> bool {
    self.sender.is_empty()
  }

  // TODO: proxy more methods
}

/// Endpoint to receive current-pass statistics data
pub struct RcDataReceiver(pub(crate) Receiver<RcData>);

impl RcDataReceiver {
  /// Attempts to receive a message from the channel without blocking.
  ///
  /// This method will either receive a message from the channel immediately or return an error
  /// if the channel is empty.
  ///
  /// If called on a zero-capacity channel, this method will receive a message only if there
  /// happens to be a send operation on the other side of the channel at the same time.
  ///
  /// # Errors
  ///
  /// - `TryRecvError::Empty` if the channel is currently empty.
  /// - `TryRecvError::Disconnected` if the channel is empty and has been disconnected.
  pub fn try_recv(&self) -> Result<RcData, TryRecvError> {
    self.0.try_recv().map_err(TryRecvError::from)
  }

  /// Blocks the current thread until a message is received or the channel is empty and
  /// disconnected.
  ///
  /// If the channel is empty and not disconnected, this call will block until the receive
  /// operation can proceed. If the channel is empty and becomes disconnected, this call will
  /// wake up and return an error.
  ///
  /// If called on a zero-capacity channel, this method will wait for a send operation to appear
  /// on the other side of the channel.
  ///
  /// # Errors
  ///
  /// - `RecvError` if the channel is empty and has been disconnected.
  pub fn recv(&self) -> Result<RcData, RecvError> {
    self.0.recv().map_err(RecvError::from)
  }
  pub fn len(&self) -> usize {
    self.0.len()
  }
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
  pub fn iter<'a>(&'a self) -> impl Iterator<Item = RcData> + 'a {
    self.0.iter()
  }

  pub const fn summary_size(&self) -> usize {
    crate::rate::TWOPASS_HEADER_SZ
  }
}

pub type PassDataChannel = (RcDataSender, RcDataReceiver);

pub type FrameInput<T> = (Option<Arc<Frame<T>>>, Option<FrameParameters>);

/// Endpoint to send frames
pub struct FrameSender<T: Pixel> {
  sender: Sender<FrameInput<T>>,
  config: Arc<EncoderConfig>,
  limit: u64,
  count: u64,
}

// Proxy the crossbeam Sender
//
// TODO: enforce the limit
impl<T: Pixel> FrameSender<T> {
  pub(crate) fn new(
    limit: u64, sender: Sender<FrameInput<T>>, config: Arc<EncoderConfig>,
  ) -> FrameSender<T> {
    Self { sender, config, limit, count: 0 }
  }

  /// # Errors
  ///
  /// - `TrySendError::Full` if the message could not be sent because the channel is full.
  /// - `TrySendError::Disconnected` if the message could not be sent
  ///   because the channel is disconnected.
  pub fn try_send<F: IntoFrame<T>>(
    &mut self, frame: F,
  ) -> Result<(), TrySendError<FrameInput<T>>> {
    if self.limit <= self.count {
      Err(TrySendError::Full(frame.into()))
    } else {
      let r = self.sender.try_send(frame.into()).map_err(TrySendError::from);
      if r.is_ok() {
        self.count += 1;
      }
      r
    }
  }

  /// # Errors
  ///
  /// - `SendError` if the message could not be sent because the channel is disconnected.
  pub fn send<F: IntoFrame<T>>(
    &mut self, frame: F,
  ) -> Result<(), SendError<FrameInput<T>>> {
    if self.limit <= self.count {
      Err(SendError(frame.into()))
    } else {
      let r = self.sender.send(frame.into()).map_err(SendError::from);
      if r.is_ok() {
        self.count += 1;
      }
      r
    }
  }
  pub fn len(&self) -> usize {
    self.sender.len()
  }
  pub fn is_empty(&self) -> bool {
    self.sender.is_empty()
  }
  // TODO: proxy more methods
}

// Frame factory
impl<T: Pixel> FrameSender<T> {
  /// Helper to create a new frame with the current encoder configuration
  #[inline]
  pub fn new_frame(&self) -> Frame<T> {
    Frame::new(
      self.config.width,
      self.config.height,
      self.config.chroma_sampling,
    )
  }
}

/// Endpoint to receive packets
pub struct PacketReceiver<T: Pixel> {
  pub(crate) receiver: Receiver<Packet<T>>,
  pub(crate) config: Arc<EncoderConfig>,
}

impl<T: Pixel> PacketReceiver<T> {
  /// Attempts to receive a message from the channel without blocking.
  ///
  /// This method will either receive a message from the channel immediately or return an error
  /// if the channel is empty.
  ///
  /// If called on a zero-capacity channel, this method will receive a message only if there
  /// happens to be a send operation on the other side of the channel at the same time.
  ///
  /// # Errors
  ///
  /// - `TryRecvError::Empty` if the channel is currently empty.
  /// - `TryRecvError::Disconnected` if the channel is empty and has been disconnected.
  pub fn try_recv(&self) -> Result<Packet<T>, TryRecvError> {
    self.receiver.try_recv().map_err(TryRecvError::from)
  }
  /// Blocks the current thread until a message is received or the channel is empty and
  /// disconnected.
  ///
  /// If the channel is empty and not disconnected, this call will block until the receive
  /// operation can proceed. If the channel is empty and becomes disconnected, this call will
  /// wake up and return an error.
  ///
  /// If called on a zero-capacity channel, this method will wait for a send operation to appear
  /// on the other side of the channel.
  ///
  /// # Errors
  ///
  /// - `RecvError` if the channel is empty and has been disconnected.
  pub fn recv(&self) -> Result<Packet<T>, RecvError> {
    self.receiver.recv().map_err(RecvError::from)
  }
  pub fn len(&self) -> usize {
    self.receiver.len()
  }
  pub fn is_empty(&self) -> bool {
    self.receiver.is_empty()
  }
  pub fn iter<'a>(&'a self) -> impl Iterator<Item = Packet<T>> + 'a {
    self.receiver.iter()
  }
}

impl<T: Pixel> PacketReceiver<T> {
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

/// A channel modeling an encoding process
pub type VideoDataChannel<T> = (FrameSender<T>, PacketReceiver<T>);
