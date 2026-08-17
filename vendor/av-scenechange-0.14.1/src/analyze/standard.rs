use std::sync::Arc;

use v_frame::{frame::Frame, math::Fixed, pixel::Pixel};

use super::{SceneChangeDetector, ScenecutResult};
use crate::{
    analyze::{
        importance::estimate_importance_block_difference,
        inter::estimate_inter_costs,
        intra::estimate_intra_costs,
    },
    data::motion::FrameMEStats,
};

impl<T: Pixel> SceneChangeDetector<T> {
    /// Run a comparison between two frames to determine if they qualify for a
    /// scenecut.
    ///
    /// We gather both intra and inter costs for the frames,
    /// as well as an importance-block-based difference,
    /// and use all three metrics.
    pub(super) fn cost_scenecut(
        &mut self,
        frame1: Arc<Frame<T>>,
        frame2: Arc<Frame<T>>,
        input_frameno: usize,
    ) -> ScenecutResult {
        let frame2_inter_ref = Arc::clone(&frame2);
        let frame1_imp_ref = Arc::clone(&frame1);
        let frame2_imp_ref = Arc::clone(&frame2);

        let mut intra_cost = 0.0;
        let mut mv_inter_cost = 0.0;
        let mut imp_block_cost = 0.0;

        let cols = 2 * self.resolution.0.align_power_of_two_and_shift(3);
        let rows = 2 * self.resolution.1.align_power_of_two_and_shift(3);

        let buffer = if let Some(buffer) = &self.frame_me_stats_buffer {
            Arc::clone(buffer)
        } else {
            let frame_me_stats = FrameMEStats::new_arc_array(cols, rows);
            let clone = Arc::clone(&frame_me_stats);
            self.frame_me_stats_buffer = Some(frame_me_stats);
            clone
        };

        rayon::scope(|s| {
            s.spawn(|_| {
                let temp_plane = self
                    .temp_plane
                    .get_or_insert_with(|| frame2.planes[0].clone());

                let intra_costs = estimate_intra_costs(
                    temp_plane,
                    &*frame2,
                    self.bit_depth,
                    self.cpu_feature_level,
                );
                if let Some(ref mut intra_cache) = self.intra_costs {
                    intra_cache.insert(input_frameno, intra_costs.clone());
                }

                intra_cost = intra_costs.iter().map(|&cost| cost as u64).sum::<u64>() as f64
                    / intra_costs.len() as f64;
            });
            s.spawn(|_| {
                mv_inter_cost = estimate_inter_costs(
                    frame2_inter_ref,
                    frame1,
                    self.bit_depth,
                    self.frame_rate,
                    self.chroma_sampling,
                    buffer,
                    self.cpu_feature_level,
                );
            });
            s.spawn(|_| {
                imp_block_cost =
                    estimate_importance_block_difference(frame2_imp_ref, frame1_imp_ref);
            });
        });

        // `BIAS` determines how likely we are
        // to choose a keyframe, between 0.0-1.0.
        // Higher values mean we are more likely to choose a keyframe.
        // This value was chosen based on trials using the new
        // adaptive scenecut code.
        const BIAS: f64 = 0.7;
        let threshold = intra_cost * (1.0 - BIAS);

        ScenecutResult {
            inter_cost: mv_inter_cost,
            imp_block_cost,
            threshold,
            backward_adjusted_cost: 0.0,
            forward_adjusted_cost: 0.0,
        }
    }
}
