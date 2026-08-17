use v_frame::{
    pixel::Pixel,
    plane::{Plane, PlaneConfig, PlaneOffset, PlaneSlice},
};

use crate::{
    cpu::CpuFeatureLevel,
    data::{
        frame::{FrameInvariants, RefType},
        mc::put_8tap,
        motion::MotionVector,
        plane::PlaneRegionMut,
        tile::TileRect,
    },
};

// There are more modes than in the spec because every allowed
// drl index for NEAR modes is considered its own mode.
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Default)]
pub enum PredictionMode {
    #[default]
    DC_PRED, // Average of above and left pixels
    V_PRED,      // Vertical
    H_PRED,      // Horizontal
    D45_PRED,    // Directional 45  degree
    D135_PRED,   // Directional 135 degree
    D113_PRED,   // Directional 113 degree
    D157_PRED,   // Directional 157 degree
    D203_PRED,   // Directional 203 degree
    D67_PRED,    // Directional 67  degree
    SMOOTH_PRED, // Combination of horizontal and vertical interpolation
    SMOOTH_V_PRED,
    SMOOTH_H_PRED,
    PAETH_PRED,
    UV_CFL_PRED,
    NEARESTMV,
    NEAR0MV,
    NEAR1MV,
    NEAR2MV,
    GLOBALMV,
    NEWMV,
    // Compound ref compound modes
    NEAREST_NEARESTMV,
    NEAR_NEAR0MV,
    NEAR_NEAR1MV,
    NEAR_NEAR2MV,
    NEAREST_NEWMV,
    NEW_NEARESTMV,
    NEAR_NEW0MV,
    NEAR_NEW1MV,
    NEAR_NEW2MV,
    NEW_NEAR0MV,
    NEW_NEAR1MV,
    NEW_NEAR2MV,
    GLOBAL_GLOBALMV,
    NEW_NEWMV,
}

impl PredictionMode {
    pub fn is_intra(self) -> bool {
        self < PredictionMode::NEARESTMV
    }

    /// Inter prediction with a single reference (i.e. not compound mode)
    ///
    /// # Panics
    ///
    /// - If called on an intra `PredictionMode`
    #[allow(clippy::too_many_arguments)]
    pub fn predict_inter_single<T: Pixel>(
        self,
        fi: &FrameInvariants<T>,
        tile_rect: TileRect,
        p: usize,
        po: PlaneOffset,
        dst: &mut PlaneRegionMut<'_, T>,
        width: usize,
        height: usize,
        ref_frame: RefType,
        mv: MotionVector,
        bit_depth: usize,
        cpu_feature_level: CpuFeatureLevel,
    ) {
        assert!(!self.is_intra());
        let frame_po = tile_rect.to_frame_plane_offset(po);

        if let Some(ref rec) = fi.rec_buffer.frames[fi.ref_frames[ref_frame.to_index()] as usize] {
            let (row_frac, col_frac, src) =
                PredictionMode::get_mv_params(&rec.frame.planes[p], frame_po, mv);
            put_8tap(
                dst,
                src,
                width,
                height,
                col_frac,
                row_frac,
                bit_depth,
                cpu_feature_level,
            );
        }
    }

    // Used by inter prediction to extract the fractional component of a mv and
    // obtain the correct PlaneSlice to operate on.
    fn get_mv_params<T: Pixel>(
        rec_plane: &Plane<T>,
        po: PlaneOffset,
        mv: MotionVector,
    ) -> (i32, i32, PlaneSlice<T>) {
        let &PlaneConfig { xdec, ydec, .. } = &rec_plane.cfg;
        let row_offset = mv.row as i32 >> (3 + ydec);
        let col_offset = mv.col as i32 >> (3 + xdec);
        let row_frac = ((mv.row as i32) << (1 - ydec)) & 0xf;
        let col_frac = ((mv.col as i32) << (1 - xdec)) & 0xf;
        let qo = PlaneOffset {
            x: po.x + col_offset as isize - 3,
            y: po.y + row_offset as isize - 3,
        };
        (
            row_frac,
            col_frac,
            rec_plane.slice(qo).clamp().subslice(3, 3),
        )
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum PredictionVariant {
    NONE,
    LEFT,
    TOP,
    BOTH,
}

impl PredictionVariant {
    pub const fn new(x: usize, y: usize) -> Self {
        match (x, y) {
            (0, 0) => PredictionVariant::NONE,
            (_, 0) => PredictionVariant::LEFT,
            (0, _) => PredictionVariant::TOP,
            _ => PredictionVariant::BOTH,
        }
    }
}
