// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.
#![deny(missing_docs)]

use crate::activity::ActivityMask;
use crate::api::lookahead::*;
use crate::api::{
  EncoderConfig, EncoderStatus, FrameType, Opaque, Packet, T35,
};
use crate::color::ChromaSampling::Cs400;
use crate::dist::get_satd;
use crate::encoder::*;
use crate::frame::*;
use crate::partition::*;
use crate::rate::{
  RCState, FRAME_NSUBTYPES, FRAME_SUBTYPE_I, FRAME_SUBTYPE_P,
  FRAME_SUBTYPE_SEF,
};
use crate::stats::EncoderStats;
use crate::tiling::Area;
use crate::util::Pixel;
use arrayvec::ArrayVec;
use av_scenechange::SceneChangeDetector;
use std::cmp;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

/// The set of options that controls frame re-ordering and reference picture
///  selection.
/// The options stored here are invariant over the whole encode.
#[derive(Debug, Clone, Copy)]
pub struct InterConfig {
  /// Whether frame re-ordering is enabled.
  reorder: bool,
  /// Whether P-frames can use multiple references.
  pub(crate) multiref: bool,
  /// The depth of the re-ordering pyramid.
  /// The current code cannot support values larger than 2.
  pub(crate) pyramid_depth: u64,
  /// Number of input frames in group.
  pub(crate) group_input_len: u64,
  /// Number of output frames in group.
  /// This includes both hidden frames and "show existing frame" frames.
  group_output_len: u64,
  /// Interval between consecutive S-frames.
  /// Keyframes reset this interval.
  /// This MUST be a multiple of `group_input_len`.
  pub(crate) switch_frame_interval: u64,
}

impl InterConfig {
  pub(crate) fn new(enc_config: &EncoderConfig) -> InterConfig {
    let reorder = !enc_config.low_latency;
    // A group always starts with (group_output_len - group_input_len) hidden
    //  frames, followed by group_input_len shown frames.
    // The shown frames iterate over the input frames in order, with frames
    //  already encoded as hidden frames now displayed with Show Existing
    //  Frame.
    // For example, for a pyramid depth of 2, the group is as follows:
    //                      |TU         |TU |TU |TU
    // idx_in_group_output:   0   1   2   3   4   5
    // input_frameno:         4   2   1  SEF  3  SEF
    // output_frameno:        1   2   3   4   5   6
    // level:                 0   1   2   1   2   0
    //                        ^^^^^   ^^^^^^^^^^^^^
    //                        hidden      shown
    // TODO: This only works for pyramid_depth <= 2 --- after that we need
    //  more hidden frames in the middle of the group.
    let pyramid_depth = if reorder { 2 } else { 0 };
    let group_input_len = 1 << pyramid_depth;
    let group_output_len = group_input_len + pyramid_depth;
    let switch_frame_interval = enc_config.switch_frame_interval;
    assert!(switch_frame_interval % group_input_len == 0);
    InterConfig {
      reorder,
      multiref: reorder || enc_config.speed_settings.multiref,
      pyramid_depth,
      group_input_len,
      group_output_len,
      switch_frame_interval,
    }
  }

  /// Get the index of an output frame in its re-ordering group given the output
  ///  frame number of the frame in the current keyframe gop.
  /// When re-ordering is disabled, this always returns 0.
  pub(crate) fn get_idx_in_group_output(
    &self, output_frameno_in_gop: u64,
  ) -> u64 {
    // The first frame in the GOP should be a keyframe and is not re-ordered,
    //  so we should not be calling this function on it.
    debug_assert!(output_frameno_in_gop > 0);
    (output_frameno_in_gop - 1) % self.group_output_len
  }

  /// Get the order-hint of an output frame given the output frame number of the
  ///  frame in the current keyframe gop and the index of that output frame
  ///  in its re-ordering gorup.
  pub(crate) fn get_order_hint(
    &self, output_frameno_in_gop: u64, idx_in_group_output: u64,
  ) -> u32 {
    // The first frame in the GOP should be a keyframe, but currently this
    //  function only handles inter frames.
    // We could return 0 for keyframes if keyframe support is needed.
    debug_assert!(output_frameno_in_gop > 0);
    // Which P-frame group in the current gop is this output frame in?
    // Subtract 1 because the first frame in the gop is always a keyframe.
    let group_idx = (output_frameno_in_gop - 1) / self.group_output_len;
    // Get the offset to the corresponding input frame.
    // TODO: This only works with pyramid_depth <= 2.
    let offset = if idx_in_group_output < self.pyramid_depth {
      self.group_input_len >> idx_in_group_output
    } else {
      idx_in_group_output - self.pyramid_depth + 1
    };
    // Construct the final order hint relative to the start of the group.
    (self.group_input_len * group_idx + offset) as u32
  }

  /// Get the level of the current frame in the pyramid.
  pub(crate) const fn get_level(&self, idx_in_group_output: u64) -> u64 {
    if !self.reorder {
      0
    } else if idx_in_group_output < self.pyramid_depth {
      // Hidden frames are output first (to be shown in the future).
      idx_in_group_output
    } else {
      // Shown frames
      // TODO: This only works with pyramid_depth <= 2.
      pos_to_lvl(
        idx_in_group_output - self.pyramid_depth + 1,
        self.pyramid_depth,
      )
    }
  }

  pub(crate) const fn get_slot_idx(&self, level: u64, order_hint: u32) -> u32 {
    // Frames with level == 0 are stored in slots 0..4, and frames with higher
    //  values of level in slots 4..8
    if level == 0 {
      (order_hint >> self.pyramid_depth) & 3
    } else {
      // This only works with pyramid_depth <= 4.
      3 + level as u32
    }
  }

  pub(crate) const fn get_show_frame(&self, idx_in_group_output: u64) -> bool {
    idx_in_group_output >= self.pyramid_depth
  }

  pub(crate) const fn get_show_existing_frame(
    &self, idx_in_group_output: u64,
  ) -> bool {
    // The self.reorder test here is redundant, but short-circuits the rest,
    //  avoiding a bunch of work when it's false.
    self.reorder
      && self.get_show_frame(idx_in_group_output)
      && (idx_in_group_output - self.pyramid_depth + 1).count_ones() == 1
      && idx_in_group_output != self.pyramid_depth
  }

  pub(crate) fn get_input_frameno(
    &self, output_frameno_in_gop: u64, gop_input_frameno_start: u64,
  ) -> u64 {
    if output_frameno_in_gop == 0 {
      gop_input_frameno_start
    } else {
      let idx_in_group_output =
        self.get_idx_in_group_output(output_frameno_in_gop);
      let order_hint =
        self.get_order_hint(output_frameno_in_gop, idx_in_group_output);
      gop_input_frameno_start + order_hint as u64
    }
  }

  const fn max_reordering_latency(&self) -> u64 {
    self.group_input_len
  }

  pub(crate) const fn keyframe_lookahead_distance(&self) -> u64 {
    self.max_reordering_latency() + 1
  }

  pub(crate) const fn allowed_ref_frames(&self) -> &[RefType] {
    use crate::partition::RefType::*;
    if self.reorder {
      &ALL_INTER_REFS
    } else if self.multiref {
      &[LAST_FRAME, LAST2_FRAME, LAST3_FRAME, GOLDEN_FRAME]
    } else {
      &[LAST_FRAME]
    }
  }
}

// Thin wrapper for frame-related data
// that gets cached and reused throughout the life of a frame.
#[derive(Clone)]
pub(crate) struct FrameData<T: Pixel> {
  pub(crate) fi: FrameInvariants<T>,
  pub(crate) fs: FrameState<T>,
}

impl<T: Pixel> FrameData<T> {
  pub(crate) fn new(fi: FrameInvariants<T>, frame: Arc<Frame<T>>) -> Self {
    let fs = FrameState::new_with_frame(&fi, frame);
    FrameData { fi, fs }
  }
}

type FrameQueue<T> = BTreeMap<u64, Option<Arc<Frame<T>>>>;
type FrameDataQueue<T> = BTreeMap<u64, Option<FrameData<T>>>;

// the fields pub(super) are accessed only by the tests
pub(crate) struct ContextInner<T: Pixel> {
  pub(crate) frame_count: u64,
  pub(crate) limit: Option<u64>,
  pub(crate) output_frameno: u64,
  pub(super) inter_cfg: InterConfig,
  pub(super) frames_processed: u64,
  /// Maps *`input_frameno`* to frames
  pub(super) frame_q: FrameQueue<T>,
  /// Maps *`output_frameno`* to frame data
  pub(super) frame_data: FrameDataQueue<T>,
  /// A list of the `input_frameno` for keyframes in this encode.
  /// Needed so that we don't need to keep all of the `frame_invariants` in
  ///  memory for the whole life of the encode.
  // TODO: Is this needed at all?
  keyframes: BTreeSet<u64>,
  // TODO: Is this needed at all?
  keyframes_forced: BTreeSet<u64>,
  /// A storage space for reordered frames.
  packet_data: Vec<u8>,
  /// Maps `output_frameno` to `gop_output_frameno_start`.
  gop_output_frameno_start: BTreeMap<u64, u64>,
  /// Maps `output_frameno` to `gop_input_frameno_start`.
  pub(crate) gop_input_frameno_start: BTreeMap<u64, u64>,
  keyframe_detector: SceneChangeDetector<T>,
  pub(crate) config: Arc<EncoderConfig>,
  seq: Arc<Sequence>,
  pub(crate) rc_state: RCState,
  maybe_prev_log_base_q: Option<i64>,
  /// The next `input_frameno` to be processed by lookahead.
  next_lookahead_frame: u64,
  /// The next `output_frameno` to be computed by lookahead.
  next_lookahead_output_frameno: u64,
  /// Optional opaque to be sent back to the user
  opaque_q: BTreeMap<u64, Opaque>,
  /// Optional T35 metadata per frame
  t35_q: BTreeMap<u64, Box<[T35]>>,
}

impl<T: Pixel> ContextInner<T> {
  pub fn new(enc: &EncoderConfig) -> Self {
    // initialize with temporal delimiter
    let packet_data = TEMPORAL_DELIMITER.to_vec();
    let mut keyframes = BTreeSet::new();
    keyframes.insert(0);

    let maybe_ac_qi_max =
      if enc.quantizer < 255 { Some(enc.quantizer as u8) } else { None };

    let seq = Arc::new(Sequence::new(enc));
    let inter_cfg = InterConfig::new(enc);
    let lookahead_distance = inter_cfg.keyframe_lookahead_distance() as usize;
    let mut keyframe_detector = SceneChangeDetector::new(
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
    keyframe_detector.enable_cache();

    ContextInner {
      frame_count: 0,
      limit: None,
      inter_cfg,
      output_frameno: 0,
      frames_processed: 0,
      frame_q: BTreeMap::new(),
      frame_data: BTreeMap::new(),
      keyframes,
      keyframes_forced: BTreeSet::new(),
      packet_data,
      gop_output_frameno_start: BTreeMap::new(),
      gop_input_frameno_start: BTreeMap::new(),
      keyframe_detector,
      config: Arc::new(enc.clone()),
      seq,
      rc_state: RCState::new(
        enc.width as i32,
        enc.height as i32,
        enc.time_base.den as i64,
        enc.time_base.num as i64,
        enc.bitrate,
        maybe_ac_qi_max,
        enc.min_quantizer,
        enc.max_key_frame_interval as i32,
        enc.reservoir_frame_delay,
      ),
      maybe_prev_log_base_q: None,
      next_lookahead_frame: 1,
      next_lookahead_output_frameno: 0,
      opaque_q: BTreeMap::new(),
      t35_q: BTreeMap::new(),
    }
  }

  #[profiling::function]
  pub fn send_frame(
    &mut self, mut frame: Option<Arc<Frame<T>>>,
    params: Option<FrameParameters>,
  ) -> Result<(), EncoderStatus> {
    if let Some(ref mut frame) = frame {
      use crate::api::color::ChromaSampling;
      let EncoderConfig { width, height, chroma_sampling, .. } = *self.config;
      let planes =
        if chroma_sampling == ChromaSampling::Cs400 { 1 } else { 3 };
      // Try to add padding
      if let Some(ref mut frame) = Arc::get_mut(frame) {
        for plane in frame.planes[..planes].iter_mut() {
          plane.pad(width, height);
        }
      }
      // Enforce that padding is added
      for (p, plane) in frame.planes[..planes].iter().enumerate() {
        assert!(
          plane.probe_padding(width, height),
          "Plane {p} was not padded before passing Frame to send_frame()."
        );
      }
    }

    let input_frameno = self.frame_count;
    let is_flushing = frame.is_none();
    if !is_flushing {
      self.frame_count += 1;
    }
    self.frame_q.insert(input_frameno, frame);

    if let Some(params) = params {
      if params.frame_type_override == FrameTypeOverride::Key {
        self.keyframes_forced.insert(input_frameno);
      }
      if let Some(op) = params.opaque {
        self.opaque_q.insert(input_frameno, op);
      }
      self.t35_q.insert(input_frameno, params.t35_metadata);
    }

    if !self.needs_more_frame_q_lookahead(self.next_lookahead_frame) {
      let lookahead_frames = self
        .frame_q
        .range(self.next_lookahead_frame - 1..)
        .filter_map(|(&_input_frameno, frame)| frame.as_ref())
        .collect::<Vec<&Arc<Frame<T>>>>();

      if is_flushing {
        // This is the last time send_frame is called, process all the
        // remaining frames.
        for cur_lookahead_frames in
          std::iter::successors(Some(&lookahead_frames[..]), |s| s.get(1..))
        {
          if cur_lookahead_frames.len() < 2 {
            // All frames have been processed
            break;
          }

          Self::compute_keyframe_placement(
            cur_lookahead_frames,
            &self.keyframes_forced,
            &mut self.keyframe_detector,
            &mut self.next_lookahead_frame,
            &mut self.keyframes,
          );
        }
      } else {
        Self::compute_keyframe_placement(
          &lookahead_frames,
          &self.keyframes_forced,
          &mut self.keyframe_detector,
          &mut self.next_lookahead_frame,
          &mut self.keyframes,
        );
      }
    }

    self.compute_frame_invariants();

    Ok(())
  }

  /// Indicates whether more frames need to be read into the frame queue
  /// in order for frame queue lookahead to be full.
  fn needs_more_frame_q_lookahead(&self, input_frameno: u64) -> bool {
    let lookahead_end = self.frame_q.keys().last().cloned().unwrap_or(0);
    let frames_needed =
      input_frameno + self.inter_cfg.keyframe_lookahead_distance() + 1;
    lookahead_end < frames_needed && self.needs_more_frames(lookahead_end)
  }

  /// Indicates whether more frames need to be processed into `FrameInvariants`
  /// in order for FI lookahead to be full.
  pub fn needs_more_fi_lookahead(&self) -> bool {
    let ready_frames = self.get_rdo_lookahead_frames().count();
    ready_frames < self.config.speed_settings.rdo_lookahead_frames + 1
      && self.needs_more_frames(self.next_lookahead_frame)
  }

  pub fn needs_more_frames(&self, frame_count: u64) -> bool {
    self.limit.map(|limit| frame_count < limit).unwrap_or(true)
  }

  fn get_rdo_lookahead_frames(
    &self,
  ) -> impl Iterator<Item = (&u64, &FrameData<T>)> {
    self
      .frame_data
      .iter()
      .skip_while(move |(&output_frameno, _)| {
        output_frameno < self.output_frameno
      })
      .filter_map(|(fno, data)| data.as_ref().map(|data| (fno, data)))
      .filter(|(_, data)| !data.fi.is_show_existing_frame())
      .take(self.config.speed_settings.rdo_lookahead_frames + 1)
  }

  fn next_keyframe_input_frameno(
    &self, gop_input_frameno_start: u64, ignore_limit: bool,
  ) -> u64 {
    let next_detected = self
      .keyframes
      .iter()
      .find(|&&input_frameno| input_frameno > gop_input_frameno_start)
      .cloned();
    let mut next_limit =
      gop_input_frameno_start + self.config.max_key_frame_interval;
    if !ignore_limit && self.limit.is_some() {
      next_limit = next_limit.min(self.limit.unwrap());
    }
    if next_detected.is_none() {
      return next_limit;
    }
    cmp::min(next_detected.unwrap(), next_limit)
  }

  fn set_frame_properties(
    &mut self, output_frameno: u64,
  ) -> Result<(), EncoderStatus> {
    let fi = self.build_frame_properties(output_frameno)?;

    self.frame_data.insert(
      output_frameno,
      fi.map(|fi| {
        let frame = self
          .frame_q
          .get(&fi.input_frameno)
          .as_ref()
          .unwrap()
          .as_ref()
          .unwrap();
        FrameData::new(fi, frame.clone())
      }),
    );

    Ok(())
  }

  #[allow(unused)]
  pub fn build_dump_properties() -> PathBuf {
    let mut data_location = PathBuf::new();
    if env::var_os("RAV1E_DATA_PATH").is_some() {
      data_location.push(env::var_os("RAV1E_DATA_PATH").unwrap());
    } else {
      data_location.push(env::current_dir().unwrap());
      data_location.push(".lookahead_data");
    }
    fs::create_dir_all(&data_location).unwrap();
    data_location
  }

  fn build_frame_properties(
    &mut self, output_frameno: u64,
  ) -> Result<Option<FrameInvariants<T>>, EncoderStatus> {
    let (prev_gop_output_frameno_start, prev_gop_input_frameno_start) =
      if output_frameno == 0 {
        (0, 0)
      } else {
        (
          self.gop_output_frameno_start[&(output_frameno - 1)],
          self.gop_input_frameno_start[&(output_frameno - 1)],
        )
      };

    self
      .gop_output_frameno_start
      .insert(output_frameno, prev_gop_output_frameno_start);
    self
      .gop_input_frameno_start
      .insert(output_frameno, prev_gop_input_frameno_start);

    let output_frameno_in_gop =
      output_frameno - self.gop_output_frameno_start[&output_frameno];
    let mut input_frameno = self.inter_cfg.get_input_frameno(
      output_frameno_in_gop,
      self.gop_input_frameno_start[&output_frameno],
    );

    if self.needs_more_frame_q_lookahead(input_frameno) {
      return Err(EncoderStatus::NeedMoreData);
    }

    let t35_metadata = if let Some(t35) = self.t35_q.remove(&input_frameno) {
      t35
    } else {
      Box::new([])
    };

    if output_frameno_in_gop > 0 {
      let next_keyframe_input_frameno = self.next_keyframe_input_frameno(
        self.gop_input_frameno_start[&output_frameno],
        false,
      );
      let prev_input_frameno =
        self.get_previous_fi(output_frameno).input_frameno;
      if input_frameno >= next_keyframe_input_frameno {
        if !self.inter_cfg.reorder
          || ((output_frameno_in_gop - 1) % self.inter_cfg.group_output_len
            == 0
            && prev_input_frameno == (next_keyframe_input_frameno - 1))
        {
          input_frameno = next_keyframe_input_frameno;

          // If we'll return early, do it before modifying the state.
          match self.frame_q.get(&input_frameno) {
            Some(Some(_)) => {}
            _ => {
              return Err(EncoderStatus::NeedMoreData);
            }
          }

          *self.gop_output_frameno_start.get_mut(&output_frameno).unwrap() =
            output_frameno;
          *self.gop_input_frameno_start.get_mut(&output_frameno).unwrap() =
            next_keyframe_input_frameno;
        } else {
          let fi = FrameInvariants::new_inter_frame(
            self.get_previous_coded_fi(output_frameno),
            &self.inter_cfg,
            self.gop_input_frameno_start[&output_frameno],
            output_frameno_in_gop,
            next_keyframe_input_frameno,
            self.config.error_resilient,
            t35_metadata,
          );
          assert!(fi.is_none());
          return Ok(fi);
        }
      }
    }

    match self.frame_q.get(&input_frameno) {
      Some(Some(_)) => {}
      _ => {
        return Err(EncoderStatus::NeedMoreData);
      }
    }

    // Now that we know the input_frameno, look up the correct frame type
    let frame_type = if self.keyframes.contains(&input_frameno) {
      FrameType::KEY
    } else {
      FrameType::INTER
    };
    if frame_type == FrameType::KEY {
      *self.gop_output_frameno_start.get_mut(&output_frameno).unwrap() =
        output_frameno;
      *self.gop_input_frameno_start.get_mut(&output_frameno).unwrap() =
        input_frameno;
    }

    let output_frameno_in_gop =
      output_frameno - self.gop_output_frameno_start[&output_frameno];
    if output_frameno_in_gop == 0 {
      let fi = FrameInvariants::new_key_frame(
        self.config.clone(),
        self.seq.clone(),
        self.gop_input_frameno_start[&output_frameno],
        t35_metadata,
      );
      Ok(Some(fi))
    } else {
      let next_keyframe_input_frameno = self.next_keyframe_input_frameno(
        self.gop_input_frameno_start[&output_frameno],
        false,
      );
      let fi = FrameInvariants::new_inter_frame(
        self.get_previous_coded_fi(output_frameno),
        &self.inter_cfg,
        self.gop_input_frameno_start[&output_frameno],
        output_frameno_in_gop,
        next_keyframe_input_frameno,
        self.config.error_resilient,
        t35_metadata,
      );
      assert!(fi.is_some());
      Ok(fi)
    }
  }

  fn get_previous_fi(&self, output_frameno: u64) -> &FrameInvariants<T> {
    let res = self
      .frame_data
      .iter()
      .filter(|(fno, _)| **fno < output_frameno)
      .rfind(|(_, fd)| fd.is_some())
      .unwrap();
    &res.1.as_ref().unwrap().fi
  }

  fn get_previous_coded_fi(&self, output_frameno: u64) -> &FrameInvariants<T> {
    let res = self
      .frame_data
      .iter()
      .filter(|(fno, _)| **fno < output_frameno)
      .rfind(|(_, fd)| {
        fd.as_ref().map(|fd| !fd.fi.is_show_existing_frame()).unwrap_or(false)
      })
      .unwrap();
    &res.1.as_ref().unwrap().fi
  }

  pub(crate) fn done_processing(&self) -> bool {
    self.limit.map(|limit| self.frames_processed == limit).unwrap_or(false)
  }

  /// Computes lookahead motion vectors and fills in `lookahead_mvs`,
  /// `rec_buffer` and `lookahead_rec_buffer` on the `FrameInvariants`. This
  /// function must be called after every new `FrameInvariants` is initially
  /// computed.
  #[profiling::function]
  fn compute_lookahead_motion_vectors(&mut self, output_frameno: u64) {
    let frame_data = self.frame_data.get(&output_frameno).unwrap();

    // We're only interested in valid frames which are not show-existing-frame.
    // Those two don't modify the rec_buffer so there's no need to do anything
    // special about it either, it'll propagate on its own.
    if frame_data
      .as_ref()
      .map(|fd| fd.fi.is_show_existing_frame())
      .unwrap_or(true)
    {
      return;
    }

    let qps = {
      let fti = frame_data.as_ref().unwrap().fi.get_frame_subtype();
      self.rc_state.select_qi(
        self,
        output_frameno,
        fti,
        self.maybe_prev_log_base_q,
        0,
      )
    };

    let frame_data =
      self.frame_data.get_mut(&output_frameno).unwrap().as_mut().unwrap();
    let fs = &mut frame_data.fs;
    let fi = &mut frame_data.fi;
    let coded_data = fi.coded_frame_data.as_mut().unwrap();

    #[cfg(feature = "dump_lookahead_data")]
    {
      let data_location = Self::build_dump_properties();
      let plane = &fs.input_qres;
      let mut file_name = format!("{:010}-qres", fi.input_frameno);
      let buf: Vec<_> = plane.iter().map(|p| p.as_()).collect();
      image::GrayImage::from_vec(
        plane.cfg.width as u32,
        plane.cfg.height as u32,
        buf,
      )
      .unwrap()
      .save(data_location.join(file_name).with_extension("png"))
      .unwrap();
      let plane = &fs.input_hres;
      file_name = format!("{:010}-hres", fi.input_frameno);
      let buf: Vec<_> = plane.iter().map(|p| p.as_()).collect();
      image::GrayImage::from_vec(
        plane.cfg.width as u32,
        plane.cfg.height as u32,
        buf,
      )
      .unwrap()
      .save(data_location.join(file_name).with_extension("png"))
      .unwrap();
    }

    // Do not modify the next output frame's FrameInvariants.
    if self.output_frameno == output_frameno {
      // We do want to propagate the lookahead_rec_buffer though.
      let rfs = Arc::new(ReferenceFrame {
        order_hint: fi.order_hint,
        width: fi.width as u32,
        height: fi.height as u32,
        render_width: fi.render_width,
        render_height: fi.render_height,
        // Use the original frame contents.
        frame: fs.input.clone(),
        input_hres: fs.input_hres.clone(),
        input_qres: fs.input_qres.clone(),
        cdfs: fs.cdfs,
        frame_me_stats: fs.frame_me_stats.clone(),
        output_frameno,
        segmentation: fs.segmentation,
      });
      for i in 0..REF_FRAMES {
        if (fi.refresh_frame_flags & (1 << i)) != 0 {
          coded_data.lookahead_rec_buffer.frames[i] = Some(Arc::clone(&rfs));
          coded_data.lookahead_rec_buffer.deblock[i] = fs.deblock;
        }
      }

      return;
    }

    // Our lookahead_rec_buffer should be filled with correct original frame
    // data from the previous frames. Copy it into rec_buffer because that's
    // what the MV search uses. During the actual encoding rec_buffer is
    // overwritten with its correct values anyway.
    fi.rec_buffer = coded_data.lookahead_rec_buffer.clone();

    // Estimate lambda with rate-control dry-run
    fi.set_quantizers(&qps);

    // TODO: as in the encoding code, key frames will have no references.
    // However, for block importance purposes we want key frames to act as
    // P-frames in this instance.
    //
    // Compute the motion vectors.
    compute_motion_vectors(fi, fs, &self.inter_cfg);

    let coded_data = fi.coded_frame_data.as_mut().unwrap();

    #[cfg(feature = "dump_lookahead_data")]
    {
      use crate::partition::RefType::*;
      let data_location = Self::build_dump_properties();
      let file_name = format!("{:010}-mvs", fi.input_frameno);
      let second_ref_frame = if !self.inter_cfg.multiref {
        LAST_FRAME // make second_ref_frame match first
      } else if fi.idx_in_group_output == 0 {
        LAST2_FRAME
      } else {
        ALTREF_FRAME
      };

      // Use the default index, it corresponds to the last P-frame or to the
      // backwards lower reference (so the closest previous frame).
      let index = if second_ref_frame.to_index() != 0 { 0 } else { 1 };

      let me_stats = &fs.frame_me_stats.read().expect("poisoned lock")[index];
      use byteorder::{NativeEndian, WriteBytesExt};
      // dynamic allocation: debugging only
      let mut buf = vec![];
      buf.write_u64::<NativeEndian>(me_stats.rows as u64).unwrap();
      buf.write_u64::<NativeEndian>(me_stats.cols as u64).unwrap();
      for y in 0..me_stats.rows {
        for x in 0..me_stats.cols {
          let mv = me_stats[y][x].mv;
          buf.write_i16::<NativeEndian>(mv.row).unwrap();
          buf.write_i16::<NativeEndian>(mv.col).unwrap();
        }
      }
      ::std::fs::write(
        data_location.join(file_name).with_extension("bin"),
        buf,
      )
      .unwrap();
    }

    // Set lookahead_rec_buffer on this FrameInvariants for future
    // FrameInvariants to pick it up.
    let rfs = Arc::new(ReferenceFrame {
      order_hint: fi.order_hint,
      width: fi.width as u32,
      height: fi.height as u32,
      render_width: fi.render_width,
      render_height: fi.render_height,
      // Use the original frame contents.
      frame: fs.input.clone(),
      input_hres: fs.input_hres.clone(),
      input_qres: fs.input_qres.clone(),
      cdfs: fs.cdfs,
      frame_me_stats: fs.frame_me_stats.clone(),
      output_frameno,
      segmentation: fs.segmentation,
    });
    for i in 0..REF_FRAMES {
      if (fi.refresh_frame_flags & (1 << i)) != 0 {
        coded_data.lookahead_rec_buffer.frames[i] = Some(Arc::clone(&rfs));
        coded_data.lookahead_rec_buffer.deblock[i] = fs.deblock;
      }
    }
  }

  /// Computes lookahead intra cost approximations and fills in
  /// `lookahead_intra_costs` on the `FrameInvariants`.
  fn compute_lookahead_intra_costs(&mut self, output_frameno: u64) {
    let frame_data = self.frame_data.get(&output_frameno).unwrap();
    let fd = &frame_data.as_ref();

    // We're only interested in valid frames which are not show-existing-frame.
    if fd.map(|fd| fd.fi.is_show_existing_frame()).unwrap_or(true) {
      return;
    }

    let fi = &fd.unwrap().fi;

    self
      .frame_data
      .get_mut(&output_frameno)
      .unwrap()
      .as_mut()
      .unwrap()
      .fi
      .coded_frame_data
      .as_mut()
      .unwrap()
      .lookahead_intra_costs = self
      .keyframe_detector
      .intra_costs
      .as_mut()
      .and_then(|intra_costs| intra_costs.remove(&(fi.input_frameno as usize)))
      .unwrap_or_else(|| {
        // We use the cached values from scenechange above if available,
        // otherwise we need to calculate them here.
        let frame = self.frame_q[&fi.input_frameno].as_ref().unwrap();
        let mut temp_plane = frame.planes[0].clone();

        estimate_intra_costs(
          &mut temp_plane,
          &**frame,
          fi.sequence.bit_depth,
          fi.cpu_feature_level,
        )
      });
  }

  #[profiling::function]
  pub fn compute_keyframe_placement(
    lookahead_frames: &[&Arc<Frame<T>>], keyframes_forced: &BTreeSet<u64>,
    keyframe_detector: &mut SceneChangeDetector<T>,
    next_lookahead_frame: &mut u64, keyframes: &mut BTreeSet<u64>,
  ) {
    if keyframes_forced.contains(next_lookahead_frame)
      || keyframe_detector.analyze_next_frame(
        lookahead_frames,
        *next_lookahead_frame as usize,
        *keyframes.iter().last().unwrap() as usize,
      )
    {
      keyframes.insert(*next_lookahead_frame);
    }

    *next_lookahead_frame += 1;
  }

  #[profiling::function]
  pub fn compute_frame_invariants(&mut self) {
    while self.set_frame_properties(self.next_lookahead_output_frameno).is_ok()
    {
      self
        .compute_lookahead_motion_vectors(self.next_lookahead_output_frameno);
      if self.config.temporal_rdo() {
        self.compute_lookahead_intra_costs(self.next_lookahead_output_frameno);
      }
      self.next_lookahead_output_frameno += 1;
    }
  }

  #[profiling::function]
  fn update_block_importances(
    fi: &FrameInvariants<T>, me_stats: &crate::me::FrameMEStats,
    frame: &Frame<T>, reference_frame: &Frame<T>, bit_depth: usize,
    bsize: BlockSize, len: usize,
    reference_frame_block_importances: &mut [f32],
  ) {
    let coded_data = fi.coded_frame_data.as_ref().unwrap();
    let plane_org = &frame.planes[0];
    let plane_ref = &reference_frame.planes[0];
    let lookahead_intra_costs_lines =
      coded_data.lookahead_intra_costs.chunks_exact(coded_data.w_in_imp_b);
    let block_importances_lines =
      coded_data.block_importances.chunks_exact(coded_data.w_in_imp_b);

    lookahead_intra_costs_lines
      .zip(block_importances_lines)
      .zip(me_stats.rows_iter().step_by(2))
      .enumerate()
      .flat_map(
        |(y, ((lookahead_intra_costs, block_importances), me_stats_line))| {
          lookahead_intra_costs
            .iter()
            .zip(block_importances.iter())
            .zip(me_stats_line.iter().step_by(2))
            .enumerate()
            .map(move |(x, ((&intra_cost, &future_importance), &me_stat))| {
              let mv = me_stat.mv;

              // Coordinates of the top-left corner of the reference block, in MV
              // units.
              let reference_x =
                x as i64 * IMP_BLOCK_SIZE_IN_MV_UNITS + mv.col as i64;
              let reference_y =
                y as i64 * IMP_BLOCK_SIZE_IN_MV_UNITS + mv.row as i64;

              let region_org = plane_org.region(Area::Rect {
                x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
                y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
                width: IMPORTANCE_BLOCK_SIZE,
                height: IMPORTANCE_BLOCK_SIZE,
              });

              let region_ref = plane_ref.region(Area::Rect {
                x: reference_x as isize
                  / IMP_BLOCK_MV_UNITS_PER_PIXEL as isize,
                y: reference_y as isize
                  / IMP_BLOCK_MV_UNITS_PER_PIXEL as isize,
                width: IMPORTANCE_BLOCK_SIZE,
                height: IMPORTANCE_BLOCK_SIZE,
              });

              let inter_cost = get_satd(
                &region_org,
                &region_ref,
                bsize.width(),
                bsize.height(),
                bit_depth,
                fi.cpu_feature_level,
              ) as f32;

              let intra_cost = intra_cost as f32;
              //          let intra_cost = lookahead_intra_costs[x] as f32;
              //          let future_importance = block_importances[x];

              let propagate_fraction = if intra_cost <= inter_cost {
                0.
              } else {
                1. - inter_cost / intra_cost
              };

              let propagate_amount = (intra_cost + future_importance)
                * propagate_fraction
                / len as f32;
              (propagate_amount, reference_x, reference_y)
            })
        },
      )
      .for_each(|(propagate_amount, reference_x, reference_y)| {
        let mut propagate =
          |block_x_in_mv_units, block_y_in_mv_units, fraction| {
            let x = block_x_in_mv_units / IMP_BLOCK_SIZE_IN_MV_UNITS;
            let y = block_y_in_mv_units / IMP_BLOCK_SIZE_IN_MV_UNITS;

            // TODO: propagate partially if the block is partially off-frame
            // (possible on right and bottom edges)?
            if x >= 0
              && y >= 0
              && (x as usize) < coded_data.w_in_imp_b
              && (y as usize) < coded_data.h_in_imp_b
            {
              reference_frame_block_importances
                [y as usize * coded_data.w_in_imp_b + x as usize] +=
                propagate_amount * fraction;
            }
          };

        // Coordinates of the top-left corner of the block intersecting the
        // reference block from the top-left.
        let top_left_block_x = (reference_x
          - if reference_x < 0 { IMP_BLOCK_SIZE_IN_MV_UNITS - 1 } else { 0 })
          / IMP_BLOCK_SIZE_IN_MV_UNITS
          * IMP_BLOCK_SIZE_IN_MV_UNITS;
        let top_left_block_y = (reference_y
          - if reference_y < 0 { IMP_BLOCK_SIZE_IN_MV_UNITS - 1 } else { 0 })
          / IMP_BLOCK_SIZE_IN_MV_UNITS
          * IMP_BLOCK_SIZE_IN_MV_UNITS;

        debug_assert!(reference_x >= top_left_block_x);
        debug_assert!(reference_y >= top_left_block_y);

        let top_right_block_x = top_left_block_x + IMP_BLOCK_SIZE_IN_MV_UNITS;
        let top_right_block_y = top_left_block_y;
        let bottom_left_block_x = top_left_block_x;
        let bottom_left_block_y =
          top_left_block_y + IMP_BLOCK_SIZE_IN_MV_UNITS;
        let bottom_right_block_x = top_right_block_x;
        let bottom_right_block_y = bottom_left_block_y;

        let top_left_block_fraction = ((top_right_block_x - reference_x)
          * (bottom_left_block_y - reference_y))
          as f32
          / IMP_BLOCK_AREA_IN_MV_UNITS as f32;

        propagate(top_left_block_x, top_left_block_y, top_left_block_fraction);

        let top_right_block_fraction =
          ((reference_x + IMP_BLOCK_SIZE_IN_MV_UNITS - top_right_block_x)
            * (bottom_left_block_y - reference_y)) as f32
            / IMP_BLOCK_AREA_IN_MV_UNITS as f32;

        propagate(
          top_right_block_x,
          top_right_block_y,
          top_right_block_fraction,
        );

        let bottom_left_block_fraction = ((top_right_block_x - reference_x)
          * (reference_y + IMP_BLOCK_SIZE_IN_MV_UNITS - bottom_left_block_y))
          as f32
          / IMP_BLOCK_AREA_IN_MV_UNITS as f32;

        propagate(
          bottom_left_block_x,
          bottom_left_block_y,
          bottom_left_block_fraction,
        );

        let bottom_right_block_fraction =
          ((reference_x + IMP_BLOCK_SIZE_IN_MV_UNITS - top_right_block_x)
            * (reference_y + IMP_BLOCK_SIZE_IN_MV_UNITS - bottom_left_block_y))
            as f32
            / IMP_BLOCK_AREA_IN_MV_UNITS as f32;

        propagate(
          bottom_right_block_x,
          bottom_right_block_y,
          bottom_right_block_fraction,
        );
      });
  }

  /// Computes the block importances for the current output frame.
  #[profiling::function]
  fn compute_block_importances(&mut self) {
    // SEF don't need block importances.
    if self.frame_data[&self.output_frameno]
      .as_ref()
      .unwrap()
      .fi
      .is_show_existing_frame()
    {
      return;
    }

    // Get a list of output_framenos that we want to propagate through.
    let output_framenos = self
      .get_rdo_lookahead_frames()
      .map(|(&output_frameno, _)| output_frameno)
      .collect::<Vec<_>>();

    // The first one should be the current output frame.
    assert_eq!(output_framenos[0], self.output_frameno);

    // First, initialize them all with zeros.
    for output_frameno in output_framenos.iter() {
      let fi = &mut self
        .frame_data
        .get_mut(output_frameno)
        .unwrap()
        .as_mut()
        .unwrap()
        .fi;
      for x in
        fi.coded_frame_data.as_mut().unwrap().block_importances.iter_mut()
      {
        *x = 0.;
      }
    }

    // Now compute and propagate the block importances from the end. The
    // current output frame will get its block importances from the future
    // frames.
    let bsize = BlockSize::from_width_and_height(
      IMPORTANCE_BLOCK_SIZE,
      IMPORTANCE_BLOCK_SIZE,
    );

    for &output_frameno in output_framenos.iter().skip(1).rev() {
      // TODO: see comment above about key frames not having references.
      if self
        .frame_data
        .get(&output_frameno)
        .unwrap()
        .as_ref()
        .unwrap()
        .fi
        .frame_type
        == FrameType::KEY
      {
        continue;
      }

      // Remove fi from the map temporarily and put it back in in the end of
      // the iteration. This is required because we need to mutably borrow
      // referenced fis from the map, and that wouldn't be possible if this was
      // an active borrow.
      //
      // Performance note: Contrary to intuition,
      // removing the data and re-inserting it at the end
      // is more performant because it avoids a very expensive clone.
      let output_frame_data =
        self.frame_data.remove(&output_frameno).unwrap().unwrap();
      {
        let fi = &output_frame_data.fi;
        let fs = &output_frame_data.fs;

        let frame = self.frame_q[&fi.input_frameno].as_ref().unwrap();

        // There can be at most 3 of these.
        let mut unique_indices = ArrayVec::<_, 3>::new();

        for (mv_index, &rec_index) in fi.ref_frames.iter().enumerate() {
          if !unique_indices.iter().any(|&(_, r)| r == rec_index) {
            unique_indices.push((mv_index, rec_index));
          }
        }

        let bit_depth = self.config.bit_depth;
        let frame_data = &mut self.frame_data;
        let len = unique_indices.len();

        let lookahead_me_stats =
          fs.frame_me_stats.read().expect("poisoned lock");

        // Compute and propagate the importance, split evenly between the
        // referenced frames.
        unique_indices.iter().for_each(|&(mv_index, rec_index)| {
          // Use rec_buffer here rather than lookahead_rec_buffer because
          // rec_buffer still contains the reference frames for the current frame
          // (it's only overwritten when the frame is encoded), while
          // lookahead_rec_buffer already contains reference frames for the next
          // frame (for the reference propagation to work correctly).
          let reference =
            fi.rec_buffer.frames[rec_index as usize].as_ref().unwrap();
          let reference_frame = &reference.frame;
          let reference_output_frameno = reference.output_frameno;
          let me_stats = &lookahead_me_stats[mv_index];

          // We should never use frame as its own reference.
          assert_ne!(reference_output_frameno, output_frameno);

          if let Some(reference_frame_block_importances) =
            frame_data.get_mut(&reference_output_frameno).map(|data| {
              &mut data
                .as_mut()
                .unwrap()
                .fi
                .coded_frame_data
                .as_mut()
                .unwrap()
                .block_importances
            })
          {
            Self::update_block_importances(
              fi,
              me_stats,
              frame,
              reference_frame,
              bit_depth,
              bsize,
              len,
              reference_frame_block_importances,
            );
          }
        });
      }
      self.frame_data.insert(output_frameno, Some(output_frame_data));
    }

    if !output_framenos.is_empty() {
      let fi = &mut self
        .frame_data
        .get_mut(&output_framenos[0])
        .unwrap()
        .as_mut()
        .unwrap()
        .fi;
      let coded_data = fi.coded_frame_data.as_mut().unwrap();
      let block_importances = coded_data.block_importances.iter();
      let lookahead_intra_costs = coded_data.lookahead_intra_costs.iter();
      let distortion_scales = coded_data.distortion_scales.iter_mut();
      for ((&propagate_cost, &intra_cost), distortion_scale) in
        block_importances.zip(lookahead_intra_costs).zip(distortion_scales)
      {
        *distortion_scale = crate::rdo::distortion_scale_for(
          propagate_cost as f64,
          intra_cost as f64,
        );
      }
      #[cfg(feature = "dump_lookahead_data")]
      {
        use byteorder::{NativeEndian, WriteBytesExt};

        let coded_data = fi.coded_frame_data.as_ref().unwrap();

        let mut buf = vec![];
        let data_location = Self::build_dump_properties();
        let file_name = format!("{:010}-imps", fi.input_frameno);
        buf.write_u64::<NativeEndian>(coded_data.h_in_imp_b as u64).unwrap();
        buf.write_u64::<NativeEndian>(coded_data.w_in_imp_b as u64).unwrap();
        buf.write_u64::<NativeEndian>(fi.get_frame_subtype() as u64).unwrap();
        for y in 0..coded_data.h_in_imp_b {
          for x in 0..coded_data.w_in_imp_b {
            buf
              .write_f32::<NativeEndian>(f64::from(
                coded_data.distortion_scales[y * coded_data.w_in_imp_b + x],
              ) as f32)
              .unwrap();
          }
        }
        ::std::fs::write(
          data_location.join(file_name).with_extension("bin"),
          buf,
        )
        .unwrap();
      }
    }
  }

  pub(crate) fn encode_packet(
    &mut self, cur_output_frameno: u64,
  ) -> Result<Packet<T>, EncoderStatus> {
    if self
      .frame_data
      .get(&cur_output_frameno)
      .unwrap()
      .as_ref()
      .unwrap()
      .fi
      .is_show_existing_frame()
    {
      if !self.rc_state.ready() {
        return Err(EncoderStatus::NotReady);
      }

      self.encode_show_existing_packet(cur_output_frameno)
    } else if let Some(Some(_)) = self.frame_q.get(
      &self
        .frame_data
        .get(&cur_output_frameno)
        .unwrap()
        .as_ref()
        .unwrap()
        .fi
        .input_frameno,
    ) {
      if !self.rc_state.ready() {
        return Err(EncoderStatus::NotReady);
      }

      self.encode_normal_packet(cur_output_frameno)
    } else {
      Err(EncoderStatus::NeedMoreData)
    }
  }

  #[profiling::function]
  pub fn encode_show_existing_packet(
    &mut self, cur_output_frameno: u64,
  ) -> Result<Packet<T>, EncoderStatus> {
    let frame_data =
      self.frame_data.get_mut(&cur_output_frameno).unwrap().as_mut().unwrap();
    let sef_data = encode_show_existing_frame(
      &frame_data.fi,
      &mut frame_data.fs,
      &self.inter_cfg,
    );
    let bits = (sef_data.len() * 8) as i64;
    self.packet_data.extend(sef_data);
    self.rc_state.update_state(
      bits,
      FRAME_SUBTYPE_SEF,
      frame_data.fi.show_frame,
      0,
      false,
      false,
    );
    let (rec, source) = if frame_data.fi.show_frame {
      (Some(frame_data.fs.rec.clone()), Some(frame_data.fs.input.clone()))
    } else {
      (None, None)
    };

    self.output_frameno += 1;

    let input_frameno = frame_data.fi.input_frameno;
    let frame_type = frame_data.fi.frame_type;
    let qp = frame_data.fi.base_q_idx;
    let enc_stats = frame_data.fs.enc_stats.clone();
    self.finalize_packet(rec, source, input_frameno, frame_type, qp, enc_stats)
  }

  #[profiling::function]
  pub fn encode_normal_packet(
    &mut self, cur_output_frameno: u64,
  ) -> Result<Packet<T>, EncoderStatus> {
    let mut frame_data =
      self.frame_data.remove(&cur_output_frameno).unwrap().unwrap();

    let mut log_isqrt_mean_scale = 0i64;

    if let Some(coded_data) = frame_data.fi.coded_frame_data.as_mut() {
      if self.config.tune == Tune::Psychovisual {
        let frame =
          self.frame_q[&frame_data.fi.input_frameno].as_ref().unwrap();
        coded_data.activity_mask = ActivityMask::from_plane(&frame.planes[0]);
        coded_data.activity_mask.fill_scales(
          frame_data.fi.sequence.bit_depth,
          &mut coded_data.activity_scales,
        );
        log_isqrt_mean_scale = coded_data.compute_spatiotemporal_scores();
      } else {
        coded_data.activity_mask = ActivityMask::default();
        log_isqrt_mean_scale = coded_data.compute_temporal_scores();
      }
      #[cfg(feature = "dump_lookahead_data")]
      {
        use crate::encoder::Scales::*;
        let input_frameno = frame_data.fi.input_frameno;
        if self.config.tune == Tune::Psychovisual {
          coded_data.dump_scales(
            Self::build_dump_properties(),
            ActivityScales,
            input_frameno,
          );
          coded_data.dump_scales(
            Self::build_dump_properties(),
            SpatiotemporalScales,
            input_frameno,
          );
        }
        coded_data.dump_scales(
          Self::build_dump_properties(),
          DistortionScales,
          input_frameno,
        );
      }
    }

    let fti = frame_data.fi.get_frame_subtype();
    let qps = self.rc_state.select_qi(
      self,
      cur_output_frameno,
      fti,
      self.maybe_prev_log_base_q,
      log_isqrt_mean_scale,
    );
    frame_data.fi.set_quantizers(&qps);

    if self.rc_state.needs_trial_encode(fti) {
      let mut trial_fs = frame_data.fs.clone();
      let data = encode_frame(&frame_data.fi, &mut trial_fs, &self.inter_cfg);
      self.rc_state.update_state(
        (data.len() * 8) as i64,
        fti,
        frame_data.fi.show_frame,
        qps.log_target_q,
        true,
        false,
      );
      let qps = self.rc_state.select_qi(
        self,
        cur_output_frameno,
        fti,
        self.maybe_prev_log_base_q,
        log_isqrt_mean_scale,
      );
      frame_data.fi.set_quantizers(&qps);
    }

    let data =
      encode_frame(&frame_data.fi, &mut frame_data.fs, &self.inter_cfg);
    #[cfg(feature = "dump_lookahead_data")]
    {
      let input_frameno = frame_data.fi.input_frameno;
      let data_location = Self::build_dump_properties();
      frame_data.fs.segmentation.dump_threshold(data_location, input_frameno);
    }
    let enc_stats = frame_data.fs.enc_stats.clone();
    self.maybe_prev_log_base_q = Some(qps.log_base_q);
    // TODO: Add support for dropping frames.
    self.rc_state.update_state(
      (data.len() * 8) as i64,
      fti,
      frame_data.fi.show_frame,
      qps.log_target_q,
      false,
      false,
    );
    self.packet_data.extend(data);

    let planes =
      if frame_data.fi.sequence.chroma_sampling == Cs400 { 1 } else { 3 };

    Arc::get_mut(&mut frame_data.fs.rec).unwrap().pad(
      frame_data.fi.width,
      frame_data.fi.height,
      planes,
    );

    let (rec, source) = if frame_data.fi.show_frame {
      (Some(frame_data.fs.rec.clone()), Some(frame_data.fs.input.clone()))
    } else {
      (None, None)
    };

    update_rec_buffer(cur_output_frameno, &mut frame_data.fi, &frame_data.fs);

    // Copy persistent fields into subsequent FrameInvariants.
    let rec_buffer = frame_data.fi.rec_buffer.clone();
    for subsequent_fi in self
      .frame_data
      .iter_mut()
      .skip_while(|(&output_frameno, _)| output_frameno <= cur_output_frameno)
      // Here we want the next valid non-show-existing-frame inter frame.
      //
      // Copying to show-existing-frame frames isn't actually required
      // for correct encoding, but it's needed for the reconstruction to
      // work correctly.
      .filter_map(|(_, frame_data)| frame_data.as_mut().map(|fd| &mut fd.fi))
      .take_while(|fi| fi.frame_type != FrameType::KEY)
    {
      subsequent_fi.rec_buffer = rec_buffer.clone();
      subsequent_fi.set_ref_frame_sign_bias();

      // Stop after the first non-show-existing-frame.
      if !subsequent_fi.is_show_existing_frame() {
        break;
      }
    }

    self.frame_data.insert(cur_output_frameno, Some(frame_data));
    let frame_data =
      self.frame_data.get(&cur_output_frameno).unwrap().as_ref().unwrap();
    let fi = &frame_data.fi;

    self.output_frameno += 1;

    if fi.show_frame {
      let input_frameno = fi.input_frameno;
      let frame_type = fi.frame_type;
      let qp = fi.base_q_idx;
      self.finalize_packet(
        rec,
        source,
        input_frameno,
        frame_type,
        qp,
        enc_stats,
      )
    } else {
      Err(EncoderStatus::Encoded)
    }
  }

  #[profiling::function]
  pub fn receive_packet(&mut self) -> Result<Packet<T>, EncoderStatus> {
    if self.done_processing() {
      return Err(EncoderStatus::LimitReached);
    }

    if self.needs_more_fi_lookahead() {
      return Err(EncoderStatus::NeedMoreData);
    }

    // Find the next output_frameno corresponding to a non-skipped frame.
    self.output_frameno = self
      .frame_data
      .iter()
      .skip_while(|(&output_frameno, _)| output_frameno < self.output_frameno)
      .find(|(_, data)| data.is_some())
      .map(|(&output_frameno, _)| output_frameno)
      .ok_or(EncoderStatus::NeedMoreData)?; // TODO: doesn't play well with the below check?

    let input_frameno =
      self.frame_data[&self.output_frameno].as_ref().unwrap().fi.input_frameno;
    if !self.needs_more_frames(input_frameno) {
      return Err(EncoderStatus::LimitReached);
    }

    if self.config.temporal_rdo() {
      // Compute the block importances for the current output frame.
      self.compute_block_importances();
    }

    let cur_output_frameno = self.output_frameno;

    let mut ret = self.encode_packet(cur_output_frameno);

    if let Ok(ref mut pkt) = ret {
      self.garbage_collect(pkt.input_frameno);
      pkt.opaque = self.opaque_q.remove(&pkt.input_frameno);
    }

    ret
  }

  fn finalize_packet(
    &mut self, rec: Option<Arc<Frame<T>>>, source: Option<Arc<Frame<T>>>,
    input_frameno: u64, frame_type: FrameType, qp: u8,
    enc_stats: EncoderStats,
  ) -> Result<Packet<T>, EncoderStatus> {
    let data = self.packet_data.clone();
    self.packet_data.clear();
    if write_temporal_delimiter(&mut self.packet_data).is_err() {
      return Err(EncoderStatus::Failure);
    }

    self.frames_processed += 1;
    Ok(Packet {
      data,
      rec,
      source,
      input_frameno,
      frame_type,
      qp,
      enc_stats,
      opaque: None,
    })
  }

  #[profiling::function]
  fn garbage_collect(&mut self, cur_input_frameno: u64) {
    if cur_input_frameno == 0 {
      return;
    }
    let frame_q_start = self.frame_q.keys().next().cloned().unwrap_or(0);
    for i in frame_q_start..cur_input_frameno {
      self.frame_q.remove(&i);
    }

    if self.output_frameno < 2 {
      return;
    }
    let fi_start = self.frame_data.keys().next().cloned().unwrap_or(0);
    for i in fi_start..(self.output_frameno - 1) {
      self.frame_data.remove(&i);
      self.gop_output_frameno_start.remove(&i);
      self.gop_input_frameno_start.remove(&i);
    }
  }

  /// Counts the number of output frames of each subtype in the next
  ///  `reservoir_frame_delay` temporal units (needed for rate control).
  /// Returns the number of output frames (excluding SEF frames) and output TUs
  ///  until the last keyframe in the next `reservoir_frame_delay` temporal units,
  ///  or the end of the interval, whichever comes first.
  /// The former is needed because it indicates the number of rate estimates we
  ///  will make.
  /// The latter is needed because it indicates the number of times new bitrate
  ///  is added to the buffer.
  pub(crate) fn guess_frame_subtypes(
    &self, nframes: &mut [i32; FRAME_NSUBTYPES + 1],
    reservoir_frame_delay: i32,
  ) -> (i32, i32) {
    for fti in 0..=FRAME_NSUBTYPES {
      nframes[fti] = 0;
    }

    // Two-pass calls this function before receive_packet(), and in particular
    // before the very first send_frame(), when the following maps are empty.
    // In this case, return 0 as the default value.
    let mut prev_keyframe_input_frameno = *self
      .gop_input_frameno_start
      .get(&self.output_frameno)
      .unwrap_or_else(|| {
        assert!(self.output_frameno == 0);
        &0
      });
    let mut prev_keyframe_output_frameno = *self
      .gop_output_frameno_start
      .get(&self.output_frameno)
      .unwrap_or_else(|| {
        assert!(self.output_frameno == 0);
        &0
      });

    let mut prev_keyframe_ntus = 0;
    // Does not include SEF frames.
    let mut prev_keyframe_nframes = 0;
    let mut acc: [i32; FRAME_NSUBTYPES + 1] = [0; FRAME_NSUBTYPES + 1];
    // Updates the frame counts with the accumulated values when we hit a
    //  keyframe.
    fn collect_counts(
      nframes: &mut [i32; FRAME_NSUBTYPES + 1],
      acc: &mut [i32; FRAME_NSUBTYPES + 1],
    ) {
      for fti in 0..=FRAME_NSUBTYPES {
        nframes[fti] += acc[fti];
        acc[fti] = 0;
      }
      acc[FRAME_SUBTYPE_I] += 1;
    }
    let mut output_frameno = self.output_frameno;
    let mut ntus = 0;
    // Does not include SEF frames.
    let mut nframes_total = 0;
    while ntus < reservoir_frame_delay {
      let output_frameno_in_gop =
        output_frameno - prev_keyframe_output_frameno;
      let is_kf =
        if let Some(Some(frame_data)) = self.frame_data.get(&output_frameno) {
          if frame_data.fi.frame_type == FrameType::KEY {
            prev_keyframe_input_frameno = frame_data.fi.input_frameno;
            // We do not currently use forward keyframes, so they should always
            //  end the current TU (thus we always increment ntus below).
            debug_assert!(frame_data.fi.show_frame);
            true
          } else {
            false
          }
        } else {
          // It is possible to be invoked for the first time from twopass_out()
          //  before receive_packet() is called, in which case frame_invariants
          //  will not be populated.
          // Force the first frame in each GOP to be a keyframe in that case.
          output_frameno_in_gop == 0
        };
      if is_kf {
        collect_counts(nframes, &mut acc);
        prev_keyframe_output_frameno = output_frameno;
        prev_keyframe_ntus = ntus;
        prev_keyframe_nframes = nframes_total;
        output_frameno += 1;
        ntus += 1;
        nframes_total += 1;
        continue;
      }
      let idx_in_group_output =
        self.inter_cfg.get_idx_in_group_output(output_frameno_in_gop);
      let input_frameno = prev_keyframe_input_frameno
        + self
          .inter_cfg
          .get_order_hint(output_frameno_in_gop, idx_in_group_output)
          as u64;
      // For rate control purposes, ignore any limit on frame count that has
      //  been set.
      // We pretend that we will keep encoding frames forever to prevent the
      //  control loop from driving us into the rails as we come up against a
      //  hard stop (with no more chance to correct outstanding errors).
      let next_keyframe_input_frameno =
        self.next_keyframe_input_frameno(prev_keyframe_input_frameno, true);
      // If we are re-ordering, we may skip some output frames in the final
      //  re-order group of the GOP.
      if input_frameno >= next_keyframe_input_frameno {
        // If we have encoded enough whole groups to reach the next keyframe,
        //  then start the next keyframe gop.
        if 1
          + (output_frameno - prev_keyframe_output_frameno)
            / self.inter_cfg.group_output_len
            * self.inter_cfg.group_input_len
          >= next_keyframe_input_frameno - prev_keyframe_input_frameno
        {
          collect_counts(nframes, &mut acc);
          prev_keyframe_input_frameno = input_frameno;
          prev_keyframe_output_frameno = output_frameno;
          prev_keyframe_ntus = ntus;
          prev_keyframe_nframes = nframes_total;
          // We do not currently use forward keyframes, so they should always
          //  end the current TU.
          output_frameno += 1;
          ntus += 1;
        }
        output_frameno += 1;
        continue;
      }
      if self.inter_cfg.get_show_existing_frame(idx_in_group_output) {
        acc[FRAME_SUBTYPE_SEF] += 1;
      } else {
        // TODO: Implement golden P-frames.
        let fti = FRAME_SUBTYPE_P
          + (self.inter_cfg.get_level(idx_in_group_output) as usize);
        acc[fti] += 1;
        nframes_total += 1;
      }
      if self.inter_cfg.get_show_frame(idx_in_group_output) {
        ntus += 1;
      }
      output_frameno += 1;
    }
    if prev_keyframe_output_frameno <= self.output_frameno {
      // If there were no keyframes at all, or only the first frame was a
      //  keyframe, the accumulators never flushed and still contain counts for
      //  the entire buffer.
      // In both cases, we return these counts.
      collect_counts(nframes, &mut acc);
      (nframes_total, ntus)
    } else {
      // Otherwise, we discard what remains in the accumulators as they contain
      //  the counts from and past the last keyframe.
      (prev_keyframe_nframes, prev_keyframe_ntus)
    }
  }
}
