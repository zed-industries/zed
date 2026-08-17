use crate::api::internal::InterConfig;
use crate::config::EncoderConfig;
use crate::context::{BlockOffset, FrameBlocks, TileBlockOffset};
use crate::cpu_features::CpuFeatureLevel;
use crate::dist::get_satd;
use crate::encoder::{
  FrameInvariants, FrameState, Sequence, IMPORTANCE_BLOCK_SIZE,
};
use crate::frame::{AsRegion, PlaneOffset};
use crate::me::{estimate_tile_motion, RefMEStats};
use crate::partition::{get_intra_edges, BlockSize};
use crate::predict::{IntraParam, PredictionMode};
use crate::tiling::{Area, PlaneRegion, TileRect};
use crate::transform::TxSize;
use crate::util::Aligned;
use crate::Pixel;
use rayon::iter::*;
use std::sync::Arc;
use v_frame::frame::Frame;
use v_frame::pixel::CastFromPrimitive;
use v_frame::plane::Plane;

pub(crate) const IMP_BLOCK_MV_UNITS_PER_PIXEL: i64 = 8;
pub(crate) const IMP_BLOCK_SIZE_IN_MV_UNITS: i64 =
  IMPORTANCE_BLOCK_SIZE as i64 * IMP_BLOCK_MV_UNITS_PER_PIXEL;
pub(crate) const IMP_BLOCK_AREA_IN_MV_UNITS: i64 =
  IMP_BLOCK_SIZE_IN_MV_UNITS * IMP_BLOCK_SIZE_IN_MV_UNITS;

#[profiling::function]
pub(crate) fn estimate_intra_costs<T: Pixel>(
  temp_plane: &mut Plane<T>, frame: &Frame<T>, bit_depth: usize,
  cpu_feature_level: CpuFeatureLevel,
) -> Box<[u32]> {
  let plane = &frame.planes[0];
  let plane_after_prediction = temp_plane;

  let bsize = BlockSize::from_width_and_height(
    IMPORTANCE_BLOCK_SIZE,
    IMPORTANCE_BLOCK_SIZE,
  );
  let tx_size = bsize.tx_size();

  let h_in_imp_b = plane.cfg.height / IMPORTANCE_BLOCK_SIZE;
  let w_in_imp_b = plane.cfg.width / IMPORTANCE_BLOCK_SIZE;
  let mut intra_costs = Vec::with_capacity(h_in_imp_b * w_in_imp_b);

  for y in 0..h_in_imp_b {
    for x in 0..w_in_imp_b {
      let plane_org = plane.region(Area::Rect {
        x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
        y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
        width: IMPORTANCE_BLOCK_SIZE,
        height: IMPORTANCE_BLOCK_SIZE,
      });

      // TODO: other intra prediction modes.
      let mut edge_buf = Aligned::uninit_array();
      let edge_buf = get_intra_edges(
        &mut edge_buf,
        &plane.as_region(),
        TileBlockOffset(BlockOffset { x, y }),
        0,
        0,
        bsize,
        PlaneOffset {
          x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
          y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
        },
        TxSize::TX_8X8,
        bit_depth,
        Some(PredictionMode::DC_PRED),
        false,
        IntraParam::None,
      );

      let mut plane_after_prediction_region = plane_after_prediction
        .region_mut(Area::Rect {
          x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
          y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
          width: IMPORTANCE_BLOCK_SIZE,
          height: IMPORTANCE_BLOCK_SIZE,
        });

      PredictionMode::DC_PRED.predict_intra(
        TileRect {
          x: x * IMPORTANCE_BLOCK_SIZE,
          y: y * IMPORTANCE_BLOCK_SIZE,
          width: IMPORTANCE_BLOCK_SIZE,
          height: IMPORTANCE_BLOCK_SIZE,
        },
        &mut plane_after_prediction_region,
        tx_size,
        bit_depth,
        &[], // Not used by DC_PRED
        IntraParam::None,
        None, // Not used by DC_PRED
        &edge_buf,
        cpu_feature_level,
      );

      let plane_after_prediction_region =
        plane_after_prediction.region(Area::Rect {
          x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
          y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
          width: IMPORTANCE_BLOCK_SIZE,
          height: IMPORTANCE_BLOCK_SIZE,
        });

      let intra_cost = get_satd(
        &plane_org,
        &plane_after_prediction_region,
        bsize.width(),
        bsize.height(),
        bit_depth,
        cpu_feature_level,
      );

      intra_costs.push(intra_cost);
    }
  }

  intra_costs.into_boxed_slice()
}

#[profiling::function]
pub(crate) fn estimate_importance_block_difference<T: Pixel>(
  frame: Arc<Frame<T>>, ref_frame: Arc<Frame<T>>,
) -> f64 {
  let plane_org = &frame.planes[0];
  let plane_ref = &ref_frame.planes[0];
  let h_in_imp_b = plane_org.cfg.height / IMPORTANCE_BLOCK_SIZE;
  let w_in_imp_b = plane_org.cfg.width / IMPORTANCE_BLOCK_SIZE;

  let mut imp_block_costs = 0;

  (0..h_in_imp_b).for_each(|y| {
    (0..w_in_imp_b).for_each(|x| {
      // Coordinates of the top-left corner of the reference block, in MV
      // units.
      let region_org = plane_org.region(Area::Rect {
        x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
        y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
        width: IMPORTANCE_BLOCK_SIZE,
        height: IMPORTANCE_BLOCK_SIZE,
      });

      let region_ref = plane_ref.region(Area::Rect {
        x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
        y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
        width: IMPORTANCE_BLOCK_SIZE,
        height: IMPORTANCE_BLOCK_SIZE,
      });

      let sum_8x8_block = |region: &PlaneRegion<T>| {
        region
          .rows_iter()
          .map(|row| {
            // 16-bit precision is sufficient for an 8px row, as IMPORTANCE_BLOCK_SIZE * (2^12 - 1) < 2^16 - 1,
            // so overflow is not possible
            row.iter().map(|pixel| u16::cast_from(*pixel)).sum::<u16>() as i64
          })
          .sum::<i64>()
      };

      let histogram_org_sum = sum_8x8_block(&region_org);
      let histogram_ref_sum = sum_8x8_block(&region_ref);

      let count = (IMPORTANCE_BLOCK_SIZE * IMPORTANCE_BLOCK_SIZE) as i64;

      let mean = (((histogram_org_sum + count / 2) / count)
        - ((histogram_ref_sum + count / 2) / count))
        .abs();

      imp_block_costs += mean as u64;
    });
  });

  imp_block_costs as f64 / (w_in_imp_b * h_in_imp_b) as f64
}

#[profiling::function]
pub(crate) fn estimate_inter_costs<T: Pixel>(
  frame: Arc<Frame<T>>, ref_frame: Arc<Frame<T>>, bit_depth: usize,
  mut config: EncoderConfig, sequence: Arc<Sequence>, buffer: RefMEStats,
) -> f64 {
  config.low_latency = true;
  config.speed_settings.multiref = false;
  let inter_cfg = InterConfig::new(&config);
  let last_fi = FrameInvariants::new_key_frame(
    Arc::new(config),
    sequence,
    0,
    Box::new([]),
  );
  let mut fi = FrameInvariants::new_inter_frame(
    &last_fi,
    &inter_cfg,
    0,
    1,
    2,
    false,
    Box::new([]),
  )
  .unwrap();

  // Compute the motion vectors.
  let mut fs = FrameState::new_with_frame_and_me_stats_and_rec(
    &fi,
    Arc::clone(&frame),
    buffer,
    // We do not use this field, so we can avoid the expensive allocation
    Arc::new(Frame {
      planes: [
        Plane::new(0, 0, 0, 0, 0, 0),
        Plane::new(0, 0, 0, 0, 0, 0),
        Plane::new(0, 0, 0, 0, 0, 0),
      ],
    }),
  );
  compute_motion_vectors(&mut fi, &mut fs, &inter_cfg);

  // Estimate inter costs
  let plane_org = &frame.planes[0];
  let plane_ref = &ref_frame.planes[0];
  let h_in_imp_b = plane_org.cfg.height / IMPORTANCE_BLOCK_SIZE;
  let w_in_imp_b = plane_org.cfg.width / IMPORTANCE_BLOCK_SIZE;
  let stats = &fs.frame_me_stats.read().expect("poisoned lock")[0];
  let bsize = BlockSize::from_width_and_height(
    IMPORTANCE_BLOCK_SIZE,
    IMPORTANCE_BLOCK_SIZE,
  );

  let mut inter_costs = 0;
  (0..h_in_imp_b).for_each(|y| {
    (0..w_in_imp_b).for_each(|x| {
      let mv = stats[y * 2][x * 2].mv;

      // Coordinates of the top-left corner of the reference block, in MV
      // units.
      let reference_x = x as i64 * IMP_BLOCK_SIZE_IN_MV_UNITS + mv.col as i64;
      let reference_y = y as i64 * IMP_BLOCK_SIZE_IN_MV_UNITS + mv.row as i64;

      let region_org = plane_org.region(Area::Rect {
        x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
        y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
        width: IMPORTANCE_BLOCK_SIZE,
        height: IMPORTANCE_BLOCK_SIZE,
      });

      let region_ref = plane_ref.region(Area::Rect {
        x: reference_x as isize / IMP_BLOCK_MV_UNITS_PER_PIXEL as isize,
        y: reference_y as isize / IMP_BLOCK_MV_UNITS_PER_PIXEL as isize,
        width: IMPORTANCE_BLOCK_SIZE,
        height: IMPORTANCE_BLOCK_SIZE,
      });

      inter_costs += get_satd(
        &region_org,
        &region_ref,
        bsize.width(),
        bsize.height(),
        bit_depth,
        fi.cpu_feature_level,
      ) as u64;
    });
  });
  inter_costs as f64 / (w_in_imp_b * h_in_imp_b) as f64
}

#[profiling::function]
pub(crate) fn compute_motion_vectors<T: Pixel>(
  fi: &mut FrameInvariants<T>, fs: &mut FrameState<T>, inter_cfg: &InterConfig,
) {
  let mut blocks = FrameBlocks::new(fi.w_in_b, fi.h_in_b);
  fi.sequence
    .tiling
    .tile_iter_mut(fs, &mut blocks)
    .collect::<Vec<_>>()
    .into_par_iter()
    .for_each(|mut ctx| {
      let ts = &mut ctx.ts;
      estimate_tile_motion(fi, ts, inter_cfg);
    });
}
