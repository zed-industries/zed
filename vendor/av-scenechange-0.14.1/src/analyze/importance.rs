use std::sync::Arc;

use v_frame::{
    frame::Frame,
    pixel::{CastFromPrimitive, Pixel},
};

use super::intra::BLOCK_TO_PLANE_SHIFT;
use crate::data::plane::{Area, AsRegion, PlaneRegion, Rect};

/// Size of blocks for the importance computation, in pixels.
pub const IMPORTANCE_BLOCK_SIZE: usize =
    1 << (IMPORTANCE_BLOCK_TO_BLOCK_SHIFT + BLOCK_TO_PLANE_SHIFT);
pub const IMPORTANCE_BLOCK_TO_BLOCK_SHIFT: usize = 1;
pub const IMP_BLOCK_MV_UNITS_PER_PIXEL: i64 = 8;
pub const IMP_BLOCK_SIZE_IN_MV_UNITS: i64 =
    IMPORTANCE_BLOCK_SIZE as i64 * IMP_BLOCK_MV_UNITS_PER_PIXEL;

pub(crate) fn estimate_importance_block_difference<T: Pixel>(
    frame: Arc<Frame<T>>,
    ref_frame: Arc<Frame<T>>,
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
            let region_org = plane_org.region(Area::Rect(Rect {
                x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
                y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
                width: IMPORTANCE_BLOCK_SIZE,
                height: IMPORTANCE_BLOCK_SIZE,
            }));

            let region_ref = plane_ref.region(Area::Rect(Rect {
                x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
                y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
                width: IMPORTANCE_BLOCK_SIZE,
                height: IMPORTANCE_BLOCK_SIZE,
            }));

            let sum_8x8_block = |region: &PlaneRegion<T>| {
                region
                    .rows_iter()
                    .map(|row| {
                        // 16-bit precision is sufficient for an 8 px row,
                        // as `IMPORTANCE_BLOCK_SIZE * (2^12 - 1) < 2^16 - 1`,
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
