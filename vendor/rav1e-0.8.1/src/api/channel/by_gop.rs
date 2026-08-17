// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::api::channel::data::*;
use crate::api::config::*;
use crate::api::util::*;
use crate::api::EncoderConfig;
use crate::api::InterConfig;

use crossbeam::channel::*;

use crate::frame::*;
use crate::util::Pixel;
use av_scenechange::SceneChangeDetector;

use std::collections::BTreeMap;
use std::sync::Arc;

struct SubGop<T: Pixel> {
  frames: Vec<Arc<Frame<T>>>,
  end_gop: bool,
}

/*
impl<T: Pixel> SubGop<T> {
  fn build_fi(&self) -> Vec<FrameData<T>> {
    todo!()
  }
}
*/

// TODO: Make the detector logic fitting the model
struct SceneChange<T: Pixel> {
  frames: usize,
  pyramid_size: usize,
  processed: usize,
  last_keyframe: usize,
  detector: SceneChangeDetector<T>,
}

impl<T: Pixel> SceneChange<T> {
  fn new(pyramid_size: usize, enc: &EncoderConfig) -> Self {
    let inter_cfg = InterConfig::new(enc);
    let lookahead_distance = inter_cfg.keyframe_lookahead_distance() as usize;
    let detector = SceneChangeDetector::new(
      (enc.width, enc.height),
      enc.bit_depth,
      av_scenechange::Rational32::new(
        enc.time_base.den as i32,
        enc.time_base.num as i32,
      ),
      enc.chroma_sampling,
      lookahead_distance,
      match enc.speed_settings.scene_detection_mode {
        super::SceneDetectionSpeed::Fast => {
          av_scenechange::SceneDetectionSpeed::Fast
        }
        super::SceneDetectionSpeed::Standard => {
          av_scenechange::SceneDetectionSpeed::Standard
        }
        super::SceneDetectionSpeed::None => {
          av_scenechange::SceneDetectionSpeed::None
        }
      },
      enc.min_key_frame_interval as usize,
      enc.max_key_frame_interval as usize,
      av_scenechange::CpuFeatureLevel::default(),
    );

    Self { frames: 0, pyramid_size, processed: 0, last_keyframe: 0, detector }
  }

  // Tell where to split the lookahead
  //
  fn split(&mut self, lookahead: &[Arc<Frame<T>>]) -> Option<(usize, bool)> {
    self.processed += 1;

    let lookahead_ref: Vec<_> = lookahead[self.frames..].iter().collect();

    let new_gop = self.detector.analyze_next_frame(
      &lookahead_ref,
      self.processed,
      self.last_keyframe,
    );

    if new_gop {
      self.last_keyframe = self.processed;
    }

    if self.frames > self.pyramid_size {
      self.frames -= self.pyramid_size + 1;
      Some((self.pyramid_size + 2, new_gop))
    } else if new_gop {
      let frames = self.frames + 1;
      self.frames = 0;
      Some((frames, true))
    } else {
      self.frames += 1;
      None
    }
  }
}

struct WorkLoad<T: Pixel> {
  s_recv: Receiver<SubGop<T>>,
  send: Sender<Packet<T>>,
}

struct WorkerPoolSend<T: Pixel> {
  recv_workers: Receiver<Sender<Option<WorkLoad<T>>>>,
  send_reassemble: Sender<(usize, Receiver<Packet<T>>)>,
  count: usize,
}

impl<T: Pixel> WorkerPoolSend<T> {
  fn get_worker(&mut self) -> Option<Sender<SubGop<T>>> {
    self.recv_workers.recv().ok().map(|sender| {
      let (s_send, s_recv) = unbounded();
      let (send, recv) = unbounded();

      let _ = self.send_reassemble.send((self.count, recv));

      let wl = WorkLoad { s_recv, send };

      let _ = sender.send(Some(wl));

      self.count += 1;

      s_send
    })
  }
}

struct WorkerPoolRecv<T: Pixel> {
  recv_reassemble: Receiver<(usize, Receiver<Packet<T>>)>,
  recv_workers: Receiver<Sender<Option<WorkLoad<T>>>>,
}

// TODO: make it Drop ?
impl<T: Pixel> WorkerPoolRecv<T> {
  fn close(&self) {
    for worker in self.recv_workers.iter() {
      let _ = worker.send(None);
    }
  }
}

fn workerpool<T: Pixel>(
  s: &rayon::ScopeFifo, workers: usize, mut cfg: Config,
) -> (WorkerPoolSend<T>, WorkerPoolRecv<T>) {
  let (send_workers, recv_workers) = bounded(workers);
  let (send_reassemble, recv_reassemble) = unbounded();

  // TODO: unpack send_frame in process
  cfg.enc.speed_settings.scene_detection_mode = SceneDetectionSpeed::None;

  for _ in 0..workers {
    let (send_workload, recv_workload) = unbounded::<Option<WorkLoad<T>>>();
    let send_workload2 = send_workload.clone();
    let send_back = send_workers.clone();

    let cfg = cfg.clone();
    s.spawn_fifo(move |_| {
      for wl in recv_workload.iter() {
        match wl {
          Some(wl) => {
            let mut inner = cfg.new_inner().unwrap();
            for s in wl.s_recv.iter() {
              for f in s.frames {
                while !inner.needs_more_fi_lookahead() {
                  let r = inner.receive_packet();
                  match r {
                    Ok(p) => {
                      wl.send.send(p).unwrap();
                    }
                    Err(EncoderStatus::Encoded) => {}
                    _ => todo!("Error management {:?}", r),
                  }
                }
                let _ = inner.send_frame(Some(f), None);
              }
            }

            inner.limit = Some(inner.frame_count);
            let _ = inner.send_frame(None, None);

            loop {
              match inner.receive_packet() {
                Ok(p) => wl.send.send(p).unwrap(),
                Err(EncoderStatus::LimitReached) => break,
                Err(EncoderStatus::Encoded) => {}
                _ => todo!("Error management"),
              }
            }

            let _ = send_back.send(send_workload2.clone());
          }
          None => break,
        }
      }
    });
    let _ = send_workers.send(send_workload);
  }

  (
    WorkerPoolSend {
      recv_workers: recv_workers.clone(),
      send_reassemble,
      count: 0,
    },
    WorkerPoolRecv { recv_reassemble, recv_workers },
  )
}

fn reassemble<P: Pixel>(
  pool: WorkerPoolRecv<P>, s: &rayon::ScopeFifo,
  send_packet: Sender<Packet<P>>,
) {
  s.spawn_fifo(move |_| {
    let mut pending = BTreeMap::new();
    let mut last_idx = 0;
    let mut packet_index = 0;
    for (idx, recv) in pool.recv_reassemble.iter() {
      pending.insert(idx, recv);
      while let Some(recv) = pending.remove(&last_idx) {
        for mut p in recv {
          // patch up the packet_index
          p.input_frameno = packet_index;
          let _ = send_packet.send(p);
          packet_index += 1;
        }
        last_idx += 1;
      }
    }

    while !pending.is_empty() {
      if let Some(recv) = pending.remove(&last_idx) {
        for mut p in recv {
          // patch up the packet_index
          p.input_frameno = packet_index;
          let _ = send_packet.send(p);
          packet_index += 1;
        }
      }
      last_idx += 1;
    }

    pool.close();
  });
}

impl Config {
  // Group the incoming frames in Gops, emit a SubGop at time.
  fn scenechange<T: Pixel>(
    &self, s: &rayon::ScopeFifo, r: Receiver<FrameInput<T>>,
  ) -> Receiver<SubGop<T>> {
    let inter_cfg = InterConfig::new(&self.enc);
    let pyramid_size = inter_cfg.keyframe_lookahead_distance() as usize;
    let lookahead_distance = pyramid_size + 1 + 1;
    let (send, recv) = bounded(lookahead_distance * 2);

    let mut sc = SceneChange::new(pyramid_size, &self.enc);

    s.spawn_fifo(move |_| {
      let mut lookahead = Vec::new();
      for f in r.iter() {
        let (frame, _params) = f;

        lookahead.push(frame.unwrap());

        // we need at least lookahead_distance frames to reason
        if lookahead.len() < lookahead_distance {
          continue;
        }

        if let Some((split_pos, end_gop)) = sc.split(&lookahead) {
          let rem = lookahead.split_off(split_pos);
          let _ = send.send(SubGop { frames: lookahead, end_gop });

          lookahead = rem;
        }
      }

      while lookahead.len() > lookahead_distance {
        if let Some((split_pos, end_gop)) = sc.split(&lookahead) {
          let rem = lookahead.split_off(split_pos);
          let _ = send.send(SubGop { frames: lookahead, end_gop });

          lookahead = rem;
        }
      }

      if !lookahead.is_empty() {
        let _ = send.send(SubGop { frames: lookahead, end_gop: true });
      }
    });

    recv
  }

  /// Encode the subgops, dispatch each Gop to an available worker
  fn encode<T: Pixel>(
    &self, s: &rayon::ScopeFifo, workers: usize, r: Receiver<SubGop<T>>,
    send_packet: Sender<Packet<T>>,
  ) {
    let (mut workers, recv) = workerpool(s, workers, self.clone());

    s.spawn_fifo(move |_| {
      let mut sg_send = workers.get_worker().unwrap();
      for sb in r.iter() {
        let end_gop = sb.end_gop;
        let _ = sg_send.send(sb);

        if end_gop {
          sg_send = workers.get_worker().unwrap();
        }
      }
    });

    reassemble(recv, s, send_packet)
  }

  /// Create a single pass by-gop encoder channel
  ///
  /// Drop the `FrameSender<T>` endpoint to flush the encoder.
  ///
  /// # Errors
  ///
  /// - Returns `InvalidConfig` if configuration is invalid.
  pub fn new_by_gop_channel<T: Pixel>(
    &self, slots: usize,
  ) -> Result<VideoDataChannel<T>, InvalidConfig> {
    let rc = &self.rate_control;

    if rc.emit_pass_data || rc.summary.is_some() {
      return Err(InvalidConfig::RateControlConfigurationMismatch);
    }

    self.validate()?;

    // TODO: make it user-settable
    let input_len = self.enc.speed_settings.rdo_lookahead_frames as usize * 4;
    let frame_limit = i32::MAX as u64;

    let (send_frame, receive_frame) = bounded(input_len);
    let (send_packet, receive_packet) = unbounded();

    let cfg = self.clone();

    let pool = self.new_thread_pool();

    // TODO: move the accounting threads outside the threadpool
    let run = move || {
      let _ = rayon::scope_fifo(|s| {
        let sg_recv = cfg.scenechange(s, receive_frame);
        cfg.encode(s, slots, sg_recv, send_packet);
      });
    };

    if let Some(pool) = pool {
      pool.spawn_fifo(run);
    } else {
      rayon::spawn_fifo(run);
    }

    let channel = (
      FrameSender::new(frame_limit, send_frame, Arc::new(self.enc.clone())),
      PacketReceiver {
        receiver: receive_packet,
        config: Arc::new(self.enc.clone()),
      },
    );

    Ok(channel)
  }
}
