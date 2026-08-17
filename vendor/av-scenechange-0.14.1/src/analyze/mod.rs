use std::{cmp, collections::BTreeMap, num::NonZeroUsize, sync::Arc};

use log::debug;
use num_rational::Rational32;
use v_frame::{
    frame::Frame,
    pixel::{ChromaSampling, Pixel},
    plane::Plane,
};

use self::fast::{detect_scale_factor, FAST_THRESHOLD};
use crate::{data::motion::RefMEStats, CpuFeatureLevel, SceneDetectionSpeed};

mod fast;
mod importance;
mod inter;
mod intra;
mod standard;

/// Experiments have determined this to be an optimal threshold
const IMP_BLOCK_DIFF_THRESHOLD: f64 = 7.0;

/// Fast integer division where divisor is a nonzero power of 2
pub(crate) fn fast_idiv(n: usize, d: NonZeroUsize) -> usize {
    debug_assert!(d.is_power_of_two());

    n >> d.trailing_zeros()
}

struct ScaleFunction<T: Pixel> {
    downscale_in_place: fn(/* &self: */ &Plane<T>, /* in_plane: */ &mut Plane<T>),
    downscale: fn(/* &self: */ &Plane<T>) -> Plane<T>,
    factor: NonZeroUsize,
}

impl<T: Pixel> ScaleFunction<T> {
    fn from_scale<const SCALE: usize>() -> Self {
        assert!(
            SCALE.is_power_of_two(),
            "Scaling factor needs to be a nonzero power of two"
        );

        Self {
            downscale: Plane::downscale::<SCALE>,
            downscale_in_place: Plane::downscale_in_place::<SCALE>,
            factor: NonZeroUsize::new(SCALE).unwrap(),
        }
    }
}
/// Runs keyframe detection on frames from the lookahead queue.
///
/// This struct is intended for advanced users who need the ability to analyze
/// a small subset of frames at a time, for example in a streaming fashion.
/// Most users will prefer to use `new_detector` and `detect_scene_changes`
/// at the top level of this crate.
pub struct SceneChangeDetector<T: Pixel> {
    // User configuration options
    /// Scenecut detection mode
    scene_detection_mode: SceneDetectionSpeed,
    /// Deque offset for current
    lookahead_offset: usize,
    /// Minimum number of frames between two scenecuts
    min_key_frame_interval: usize,
    /// Maximum number of frames between two scenecuts
    max_key_frame_interval: usize,
    /// The CPU feature level to be used.
    cpu_feature_level: CpuFeatureLevel,

    // Internal configuration options
    /// Minimum average difference between YUV deltas that will trigger a scene
    /// change.
    threshold: f64,
    /// Width and height of the unscaled frame
    resolution: (usize, usize),
    /// The bit depth of the video.
    bit_depth: usize,
    /// The frame rate of the video.
    frame_rate: Rational32,
    /// The chroma subsampling of the video.
    chroma_sampling: ChromaSampling,
    /// Number of pixels in scaled frame for fast mode
    scaled_pixels: usize,
    /// Downscaling function for fast scene detection
    scale_func: Option<ScaleFunction<T>>,

    // Internal data structures
    /// Start deque offset based on lookahead
    deque_offset: usize,
    /// Frame buffer for scaled frames
    downscaled_frame_buffer: Option<[Plane<T>; 2]>,
    /// Scenechange results for adaptive threshold
    score_deque: Vec<ScenecutResult>,
    /// Temporary buffer used by `estimate_intra_costs`.
    /// We store it on the struct so we only need to allocate it once.
    temp_plane: Option<Plane<T>>,
    /// Buffer for `FrameMEStats` for cost scenecut
    frame_me_stats_buffer: Option<RefMEStats>,

    /// Calculated intra costs for each input frame.
    /// These can be cached for reuse by advanced API users.
    /// Caching will occur if this is not `None`.
    pub intra_costs: Option<BTreeMap<usize, Box<[u32]>>>,
}

impl<T: Pixel> SceneChangeDetector<T> {
    /// Creates a new instance of the `SceneChangeDetector`.
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn new(
        resolution: (usize, usize),
        bit_depth: usize,
        frame_rate: Rational32,
        chroma_sampling: ChromaSampling,
        lookahead_distance: usize,
        scene_detection_mode: SceneDetectionSpeed,
        min_key_frame_interval: usize,
        max_key_frame_interval: usize,
        cpu_feature_level: CpuFeatureLevel,
    ) -> Self {
        // Downscaling function for fast scene detection
        let scale_func = detect_scale_factor(resolution, scene_detection_mode);

        // Set lookahead offset to 5 if normal lookahead available
        let lookahead_offset = if lookahead_distance >= 5 { 5 } else { 0 };
        let deque_offset = lookahead_offset;

        let score_deque = Vec::with_capacity(5 + lookahead_distance);

        // Downscaling factor for fast scenedetect (is currently always a power of 2)
        let factor = scale_func.as_ref().map_or(
            NonZeroUsize::new(1).expect("constant should not panic"),
            |x| x.factor,
        );

        let pixels = if scene_detection_mode == SceneDetectionSpeed::Fast {
            fast_idiv(resolution.1, factor) * fast_idiv(resolution.0, factor)
        } else {
            1
        };

        let threshold = FAST_THRESHOLD * (bit_depth as f64) / 8.0;

        Self {
            threshold,
            scene_detection_mode,
            scale_func,
            lookahead_offset,
            deque_offset,
            score_deque,
            scaled_pixels: pixels,
            bit_depth,
            frame_rate,
            chroma_sampling,
            min_key_frame_interval,
            max_key_frame_interval,
            cpu_feature_level,
            downscaled_frame_buffer: None,
            resolution,
            temp_plane: None,
            frame_me_stats_buffer: None,
            intra_costs: None,
        }
    }

    /// Enables caching of intra costs. For advanced API users.
    #[inline]
    pub fn enable_cache(&mut self) {
        if self.intra_costs.is_none() {
            self.intra_costs = Some(BTreeMap::new());
        }
    }

    /// Runs keyframe detection on the next frame in the lookahead queue.
    ///
    /// This function requires that a subset of input frames
    /// is passed to it in order, and that `keyframes` is only
    /// updated from this method. `input_frameno` should correspond
    /// to the second frame in `frame_set`.
    ///
    /// This will gracefully handle the first frame in the video as well.
    #[inline]
    pub fn analyze_next_frame(
        &mut self,
        frame_set: &[&Arc<Frame<T>>],
        input_frameno: usize,
        previous_keyframe: usize,
    ) -> bool {
        // Use score deque for adaptive threshold for scene cut
        // Declare score_deque offset based on lookahead  for scene change scores

        // Find the distance to the previous keyframe.
        let distance = input_frameno - previous_keyframe;

        if frame_set.len() <= self.lookahead_offset {
            // Don't insert keyframes in the last few frames of the video
            // This is basically a scene flash and a waste of bits
            return false;
        }

        if self.scene_detection_mode == SceneDetectionSpeed::None {
            if let Some(true) = self.handle_min_max_intervals(distance) {
                return true;
            };
            return false;
        }

        // Initialization of score deque
        // based on frame set length
        if self.deque_offset > 0
            && frame_set.len() > self.deque_offset + 1
            && self.score_deque.is_empty()
        {
            self.initialize_score_deque(frame_set, input_frameno, self.deque_offset);
        } else if self.score_deque.is_empty() {
            self.initialize_score_deque(frame_set, input_frameno, frame_set.len() - 1);

            self.deque_offset = frame_set.len() - 2;
        }
        // Running single frame comparison and adding it to deque
        // Decrease deque offset if there is no new frames
        if frame_set.len() > self.deque_offset + 1 {
            self.run_comparison(
                frame_set[self.deque_offset].clone(),
                frame_set[self.deque_offset + 1].clone(),
                input_frameno + self.deque_offset,
            );
        } else {
            self.deque_offset -= 1;
        }

        // Adaptive scenecut check
        let (scenecut, score) = self.adaptive_scenecut();
        let scenecut = self.handle_min_max_intervals(distance).unwrap_or(scenecut);
        debug!(
            "[SC-Detect] Frame {}: Raw={:5.1}  ImpBl={:5.1}  Bwd={:5.1}  Fwd={:5.1}  Th={:.1}  {}",
            input_frameno,
            score.inter_cost,
            score.imp_block_cost,
            score.backward_adjusted_cost,
            score.forward_adjusted_cost,
            score.threshold,
            if scenecut { "Scenecut" } else { "No cut" }
        );

        // Keep score deque of 5 backward frames
        // and forward frames of length of lookahead offset
        if self.score_deque.len() > 5 + self.lookahead_offset {
            self.score_deque.pop();
        }

        scenecut
    }

    fn handle_min_max_intervals(&mut self, distance: usize) -> Option<bool> {
        // Handle minimum and maximum keyframe intervals.
        if distance < self.min_key_frame_interval {
            return Some(false);
        }
        if distance >= self.max_key_frame_interval {
            return Some(true);
        }
        None
    }

    // Initially fill score deque with frame scores
    fn initialize_score_deque(
        &mut self,
        frame_set: &[&Arc<Frame<T>>],
        input_frameno: usize,
        init_len: usize,
    ) {
        for x in 0..init_len {
            self.run_comparison(
                frame_set[x].clone(),
                frame_set[x + 1].clone(),
                input_frameno + x,
            );
        }
    }

    /// Runs scene change comparison between 2 given frames
    /// Insert result to start of score deque
    fn run_comparison(
        &mut self,
        frame1: Arc<Frame<T>>,
        frame2: Arc<Frame<T>>,
        input_frameno: usize,
    ) {
        let mut result = match self.scene_detection_mode {
            SceneDetectionSpeed::Fast => self.fast_scenecut(frame1, frame2),
            SceneDetectionSpeed::Standard => self.cost_scenecut(frame1, frame2, input_frameno),
            _ => unreachable!(),
        };

        // Subtract the highest metric value of surrounding frames from the current one.
        // It makes the peaks in the metric more distinct.
        if self.scene_detection_mode == SceneDetectionSpeed::Standard && self.deque_offset > 0 {
            if input_frameno == 1 {
                // Accounts for the second frame not having a score to adjust against.
                // It should always be 0 because the first frame of the video is always a
                // keyframe.
                result.backward_adjusted_cost = 0.0;
            } else {
                let mut adjusted_cost = f64::MAX;
                for other_cost in self
                    .score_deque
                    .iter()
                    .take(self.deque_offset)
                    .map(|i| i.inter_cost)
                {
                    let this_cost = result.inter_cost - other_cost;
                    if this_cost < adjusted_cost {
                        adjusted_cost = this_cost;
                    }
                    if adjusted_cost < 0.0 {
                        adjusted_cost = 0.0;
                        break;
                    }
                }
                result.backward_adjusted_cost = adjusted_cost;
            }
            if !self.score_deque.is_empty() {
                for i in 0..cmp::min(self.deque_offset, self.score_deque.len()) {
                    let adjusted_cost = self.score_deque[i].inter_cost - result.inter_cost;
                    if i == 0 || adjusted_cost < self.score_deque[i].forward_adjusted_cost {
                        self.score_deque[i].forward_adjusted_cost = adjusted_cost;
                    }
                    if self.score_deque[i].forward_adjusted_cost < 0.0 {
                        self.score_deque[i].forward_adjusted_cost = 0.0;
                    }
                }
            }
        }
        self.score_deque.insert(0, result);
    }

    /// Compares current scene score to adapted threshold based on previous
    /// scores
    ///
    /// Value of current frame is offset by lookahead, if lookahead >=5
    ///
    /// Returns true if current scene score is higher than adapted threshold
    fn adaptive_scenecut(&mut self) -> (bool, ScenecutResult) {
        let score = self.score_deque[self.deque_offset];

        // We use the importance block algorithm's cost metrics as a secondary algorithm
        // because, although it struggles in certain scenarios such as
        // finding the end of a pan, it is very good at detecting hard scenecuts
        // or detecting if a pan exists.
        //
        // Because of this, we only consider a frame for a scenechange if
        // the importance block algorithm is over the threshold either on this frame
        // (hard scenecut) or within the past few frames (pan). This helps
        // filter out a few false positives produced by the cost-based
        // algorithm.
        let imp_block_threshold = IMP_BLOCK_DIFF_THRESHOLD * (self.bit_depth as f64) / 8.0;
        if !&self.score_deque[self.deque_offset..]
            .iter()
            .any(|result| result.imp_block_cost >= imp_block_threshold)
        {
            return (false, score);
        }

        let cost = score.forward_adjusted_cost;
        if cost >= score.threshold {
            let back_deque = &self.score_deque[self.deque_offset + 1..];
            let forward_deque = &self.score_deque[..self.deque_offset];
            let back_over_tr_count = back_deque
                .iter()
                .filter(|result| result.backward_adjusted_cost >= result.threshold)
                .count();
            let forward_over_tr_count = forward_deque
                .iter()
                .filter(|result| result.forward_adjusted_cost >= result.threshold)
                .count();

            // Check for scenecut after the flashes
            // No frames over threshold forward
            // and some frames over threshold backward
            let back_count_req = if self.scene_detection_mode == SceneDetectionSpeed::Fast {
                // Fast scenecut is more sensitive to false flash detection,
                // so we want more "evidence" of there being a flash before creating a keyframe.
                2
            } else {
                1
            };
            if forward_over_tr_count == 0 && back_over_tr_count >= back_count_req {
                return (true, score);
            }

            // Check for scenecut before flash
            // If distance longer than max flash length
            if back_over_tr_count == 0
                && forward_over_tr_count == 1
                && forward_deque[0].forward_adjusted_cost >= forward_deque[0].threshold
            {
                return (true, score);
            }

            if back_over_tr_count != 0 || forward_over_tr_count != 0 {
                return (false, score);
            }
        }

        (cost >= score.threshold, score)
    }
}

#[derive(Debug, Clone, Copy)]
struct ScenecutResult {
    inter_cost: f64,
    imp_block_cost: f64,
    backward_adjusted_cost: f64,
    forward_adjusted_cost: f64,
    threshold: f64,
}
