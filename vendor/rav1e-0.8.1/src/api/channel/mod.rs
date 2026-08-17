// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.
#![allow(missing_docs)]

use crate::api::config::*;
use crate::api::context::RcData;
use crate::api::internal::ContextInner;
use crate::api::util::*;

use crossbeam::channel::*;

use crate::rate::RCState;
use crate::util::Pixel;

use rayon::ThreadPool;
use std::sync::Arc;

mod data;
pub use data::{
  FrameInput, FrameSender, PacketReceiver, PassDataChannel, RcDataReceiver,
  RcDataSender, RecvError, SendError, TryRecvError, TrySendError,
  VideoDataChannel,
};

mod by_gop;

impl Config {
  pub(crate) fn setup<T: Pixel>(
    &self,
  ) -> Result<(ContextInner<T>, Option<Arc<ThreadPool>>), InvalidConfig> {
    self.validate()?;
    let inner = self.new_inner()?;

    let pool = self.new_thread_pool();

    Ok((inner, pool))
  }
}

impl Config {
  /// Create a single pass encoder channel
  ///
  /// Drop the `FrameSender<T>` endpoint to flush the encoder.
  ///
  /// # Errors
  ///
  /// - Returns `InvalidConfig` if the configuration is invalid.
  pub fn new_channel<T: Pixel>(
    &self,
  ) -> Result<VideoDataChannel<T>, InvalidConfig> {
    let rc = &self.rate_control;

    if rc.emit_pass_data || rc.summary.is_some() {
      return Err(InvalidConfig::RateControlConfigurationMismatch);
    }
    let v = if self.slots > 1 {
      self.new_by_gop_channel(self.slots)?
    } else {
      self.new_channel_internal()?.0
    };

    Ok(v)
  }

  /// Create a first pass encoder channel
  ///
  /// The pass data information is emitted through this channel.
  ///
  /// Drop the `FrameSender<T>` endpoint to flush the encoder.
  /// The last buffer in the `PassDataReceiver` is the summary of the whole
  /// encoding process.
  ///
  /// # Errors
  ///
  /// - Returns `InvalidConfig` if the configuration is invalid.
  ///
  /// # Panics
  ///
  /// - If the channel cannot be created. An error should be raised before this,
  ///   so a panic indicates a development error.
  pub fn new_firstpass_channel<T: Pixel>(
    &self,
  ) -> Result<(VideoDataChannel<T>, RcDataReceiver), InvalidConfig> {
    let rc = &self.rate_control;

    if !rc.emit_pass_data {
      return Err(InvalidConfig::RateControlConfigurationMismatch);
    }

    if self.slots > 1 {
      log::warn!(
        "Parallel gop encoding does not support multi pass rate control"
      );
    }

    let (v, (_, r)) = self.new_channel_internal()?;

    Ok((v, r.unwrap()))
  }

  /// Create a second pass encoder channel
  ///
  /// The encoding process require both frames and pass data to progress.
  ///
  /// Drop the `FrameSender<T>` endpoint to flush the encoder.
  ///
  /// # Errors
  ///
  /// - Returns `InvalidConfig` if the configuration is invalid.
  ///
  /// # Panics
  ///
  /// - If the channel cannot be created. An error should be raised before this,
  ///   so a panic indicates a development error.
  pub fn new_secondpass_channel<T: Pixel>(
    &self,
  ) -> Result<(VideoDataChannel<T>, RcDataSender), InvalidConfig> {
    let rc = &self.rate_control;
    if rc.emit_pass_data || rc.summary.is_none() {
      return Err(InvalidConfig::RateControlConfigurationMismatch);
    }

    if self.slots > 1 {
      log::warn!(
        "Parallel gop encoding does not support multi pass rate control"
      );
    }

    let (v, (s, _)) = self.new_channel_internal()?;

    Ok((v, s.unwrap()))
  }

  /// Create a multipass encoder channel
  ///
  /// The `PacketReceiver<T>` may block if not enough pass statistics data
  /// are sent through the `PassDataSender` endpoint
  ///
  /// Drop the `FrameSender<T>` endpoint to flush the encoder.
  /// The last buffer in the `PassDataReceiver` is the summary of the whole
  /// encoding process.
  ///
  /// # Errors
  ///
  /// - Returns `InvalidConfig` if the configuration is invalid.
  ///
  /// # Panics
  ///
  /// - If the channel cannot be created. An error should be raised before this,
  ///   so a panic indicates a development error.
  pub fn new_multipass_channel<T: Pixel>(
    &self,
  ) -> Result<(VideoDataChannel<T>, PassDataChannel), InvalidConfig> {
    let rc = &self.rate_control;
    if rc.summary.is_none() || !rc.emit_pass_data {
      return Err(InvalidConfig::RateControlConfigurationMismatch);
    }

    if self.slots > 1 {
      log::warn!(
        "Parallel gop encoding does not support multi pass rate control"
      );
    }

    let (v, (s, r)) = self.new_channel_internal()?;

    Ok((v, (s.unwrap(), r.unwrap())))
  }
}

trait RcFirstPass {
  fn send_pass_data(&mut self, rc_state: &mut RCState);
  fn send_pass_summary(&mut self, rc_state: &mut RCState);
}

trait RcSecondPass {
  fn feed_pass_data<T: Pixel>(
    &mut self, inner: &mut ContextInner<T>,
  ) -> Result<(), ()>;
}

impl RcFirstPass for Sender<RcData> {
  fn send_pass_data(&mut self, rc_state: &mut RCState) {
    if let Some(data) = rc_state.emit_frame_data() {
      let data = data.to_vec().into_boxed_slice();
      self.send(RcData::Frame(data)).unwrap();
    } else {
      unreachable!(
        "The encoder received more frames than its internal
              limit allows"
      );
    }
  }
  fn send_pass_summary(&mut self, rc_state: &mut RCState) {
    let data = rc_state.emit_summary();
    let data = data.to_vec().into_boxed_slice();
    self.send(RcData::Summary(data)).unwrap();
  }
}

impl RcFirstPass for Option<Sender<RcData>> {
  fn send_pass_data(&mut self, rc_state: &mut RCState) {
    if let Some(s) = self.as_mut() {
      s.send_pass_data(rc_state)
    }
  }
  fn send_pass_summary(&mut self, rc_state: &mut RCState) {
    if let Some(s) = self.as_mut() {
      s.send_pass_summary(rc_state)
    }
  }
}

impl RcSecondPass for Receiver<RcData> {
  fn feed_pass_data<T: Pixel>(
    &mut self, inner: &mut ContextInner<T>,
  ) -> Result<(), ()> {
    while inner.rc_state.twopass_in_frames_needed() > 0
      && !inner.done_processing()
    {
      if let Ok(RcData::Frame(data)) = self.recv() {
        inner
          .rc_state
          .parse_frame_data_packet(data.as_ref())
          .unwrap_or_else(|_| todo!("Error reporting"));
      } else {
        todo!("Error reporting");
      }
    }

    Ok(())
  }
}

impl RcSecondPass for Option<Receiver<RcData>> {
  fn feed_pass_data<T: Pixel>(
    &mut self, inner: &mut ContextInner<T>,
  ) -> Result<(), ()> {
    match self.as_mut() {
      Some(s) => s.feed_pass_data(inner),
      None => Ok(()),
    }
  }
}

impl Config {
  #[allow(clippy::type_complexity)]
  fn new_channel_internal<T: Pixel>(
    &self,
  ) -> Result<
    (VideoDataChannel<T>, (Option<RcDataSender>, Option<RcDataReceiver>)),
    InvalidConfig,
  > {
    // The inner context is already configured to use the summary at this point.
    let (mut inner, pool) = self.setup()?;

    // TODO: make it user-settable
    let input_len = self.enc.speed_settings.rdo_lookahead_frames as usize * 2;

    let (send_frame, receive_frame) = bounded(input_len);
    let (send_packet, receive_packet) = unbounded();

    let rc = &self.rate_control;

    let (mut send_rc_pass1, rc_data_receiver) = if rc.emit_pass_data {
      let (send_rc_pass1, receive_rc_pass1) = unbounded();
      (Some(send_rc_pass1), Some(RcDataReceiver(receive_rc_pass1)))
    } else {
      (None, None)
    };

    let (rc_data_sender, mut receive_rc_pass2, frame_limit) = if rc
      .summary
      .is_some()
    {
      let (frame_limit, pass_limit) =
        rc.summary.as_ref().map(|s| (s.ntus as u64, s.total as u64)).unwrap();

      inner.limit = Some(frame_limit);

      let (send_rc_pass2, receive_rc_pass2) = unbounded();

      (
        Some(RcDataSender::new(pass_limit, send_rc_pass2)),
        Some(receive_rc_pass2),
        frame_limit,
      )
    } else {
      (None, None, i32::MAX as u64)
    };

    let config = Arc::new(self.enc.clone());

    let channel = (
      FrameSender::new(frame_limit, send_frame, config.clone()),
      PacketReceiver { receiver: receive_packet, config },
    );

    let pass_channel = (rc_data_sender, rc_data_receiver);

    let run = move || {
      for f in receive_frame.iter() {
        // info!("frame in {}", inner.frame_count);
        while !inner.needs_more_fi_lookahead() {
          receive_rc_pass2.feed_pass_data(&mut inner).unwrap();
          // needs_more_fi_lookahead() should guard for missing output_frameno
          // already.
          //
          // this call should return either Ok or Err(Encoded)
          let has_pass_data = match inner.receive_packet() {
            Ok(p) => {
              send_packet.send(p).unwrap();
              true
            }
            Err(EncoderStatus::Encoded) => true,
            Err(EncoderStatus::NotReady) => todo!("Error reporting"),
            _ => unreachable!(),
          };
          if has_pass_data {
            send_rc_pass1.send_pass_data(&mut inner.rc_state);
          }
        }

        let (frame, params) = f;
        let _ = inner.send_frame(frame, params); // TODO make sure it cannot fail.
      }

      inner.limit = Some(inner.frame_count);
      let _ = inner.send_frame(None, None);

      loop {
        receive_rc_pass2.feed_pass_data(&mut inner).unwrap();
        let r = inner.receive_packet();
        let has_pass_data = match r {
          Ok(p) => {
            // warn!("Sending out {}", p.input_frameno);
            send_packet.send(p).unwrap();
            true
          }
          Err(EncoderStatus::LimitReached) => break,
          Err(EncoderStatus::Encoded) => true,
          Err(EncoderStatus::NotReady) => todo!("Error reporting"),
          _ => unreachable!(),
        };

        if has_pass_data {
          send_rc_pass1.send_pass_data(&mut inner.rc_state);
        }
      }

      send_rc_pass1.send_pass_summary(&mut inner.rc_state);
    };

    if let Some(pool) = pool {
      pool.spawn(run);
    } else {
      rayon::spawn(run);
    }

    Ok((channel, pass_channel))
  }
}
