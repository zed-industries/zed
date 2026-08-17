// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;

impl CDFContext {
  // rather than test writing and rolling back the cdf, we just count Q8 bits using the current cdf
  pub fn count_lrf_switchable<W: Writer>(
    &self, w: &W, rs: &TileRestorationState, filter: RestorationFilter,
    pli: usize,
  ) -> u32 {
    match filter {
      RestorationFilter::None => w.symbol_bits(0, &self.lrf_switchable_cdf),
      RestorationFilter::Wiener { .. } => {
        unreachable!() // for now, not permanently
      }
      RestorationFilter::Sgrproj { set, xqd } => {
        // Does *not* use 'RESTORE_SGRPROJ' but rather just '2'
        let rp = &rs.planes[pli];
        let mut bits = w.symbol_bits(2, &self.lrf_switchable_cdf)
          + ((SGRPROJ_PARAMS_BITS as u32) << OD_BITRES);
        for i in 0..2 {
          let s = SGRPROJ_PARAMS_S[set as usize][i];
          let min = SGRPROJ_XQD_MIN[i] as i32;
          let max = SGRPROJ_XQD_MAX[i] as i32;
          if s > 0 {
            bits += w.count_signed_subexp_with_ref(
              xqd[i] as i32,
              min,
              max + 1,
              SGRPROJ_PRJ_SUBEXP_K,
              rp.sgrproj_ref[i] as i32,
            );
          }
        }
        bits
      }
    }
  }
}

impl ContextWriter<'_> {
  fn get_ref_frame_ctx_b0(&self, bo: TileBlockOffset) -> usize {
    let ref_counts = self.bc.blocks[bo].neighbors_ref_counts;

    let fwd_cnt = ref_counts[LAST_FRAME.to_index()]
      + ref_counts[LAST2_FRAME.to_index()]
      + ref_counts[LAST3_FRAME.to_index()]
      + ref_counts[GOLDEN_FRAME.to_index()];

    let bwd_cnt = ref_counts[BWDREF_FRAME.to_index()]
      + ref_counts[ALTREF2_FRAME.to_index()]
      + ref_counts[ALTREF_FRAME.to_index()];

    ContextWriter::ref_count_ctx(fwd_cnt, bwd_cnt)
  }

  /// # Panics
  ///
  /// - If the `comp_mode` setting does not match the reference mode and size.
  pub fn write_ref_frames<T: Pixel, W: Writer>(
    &mut self, w: &mut W, fi: &FrameInvariants<T>, bo: TileBlockOffset,
  ) {
    let rf = self.bc.blocks[bo].ref_frames;
    let sz = self.bc.blocks[bo].n4_w.min(self.bc.blocks[bo].n4_h);

    /* TODO: Handle multiple references */
    let comp_mode = self.bc.blocks[bo].has_second_ref();

    if fi.reference_mode != ReferenceMode::SINGLE && sz >= 2 {
      let ctx = self.get_comp_mode_ctx(bo);
      let cdf = &self.fc.comp_mode_cdf[ctx];
      symbol_with_update!(self, w, comp_mode as u32, cdf);
    } else {
      assert!(!comp_mode);
    }

    if comp_mode {
      let comp_ref_type: u32 = 1; // bidir
      let ctx = self.get_comp_ref_type_ctx(bo);
      let cdf = &self.fc.comp_ref_type_cdf[ctx];
      symbol_with_update!(self, w, comp_ref_type, cdf);

      if comp_ref_type == 0 {
        unimplemented!();
      } else {
        let compref = rf[0] == GOLDEN_FRAME || rf[0] == LAST3_FRAME;
        let ctx = self.get_pred_ctx_ll2_or_l3gld(bo);
        let cdf = &self.fc.comp_ref_cdf[ctx][0];
        symbol_with_update!(self, w, compref as u32, cdf);
        if !compref {
          let compref_p1 = rf[0] == LAST2_FRAME;
          let ctx = self.get_pred_ctx_last_or_last2(bo);
          let cdf = &self.fc.comp_ref_cdf[ctx][1];
          symbol_with_update!(self, w, compref_p1 as u32, cdf);
        } else {
          let compref_p2 = rf[0] == GOLDEN_FRAME;
          let ctx = self.get_pred_ctx_last3_or_gold(bo);
          let cdf = &self.fc.comp_ref_cdf[ctx][2];
          symbol_with_update!(self, w, compref_p2 as u32, cdf);
        }
        let comp_bwdref = rf[1] == ALTREF_FRAME;
        let ctx = self.get_pred_ctx_brfarf2_or_arf(bo);
        let cdf = &self.fc.comp_bwd_ref_cdf[ctx][0];
        symbol_with_update!(self, w, comp_bwdref as u32, cdf);
        if !comp_bwdref {
          let comp_bwdref_p1 = rf[1] == ALTREF2_FRAME;
          let ctx = self.get_pred_ctx_brf_or_arf2(bo);
          let cdf = &self.fc.comp_bwd_ref_cdf[ctx][1];
          symbol_with_update!(self, w, comp_bwdref_p1 as u32, cdf);
        }
      }
    } else {
      let b0_ctx = self.get_ref_frame_ctx_b0(bo);
      let b0 = rf[0] != NONE_FRAME && rf[0].is_bwd_ref();

      let cdf = &self.fc.single_ref_cdfs[b0_ctx][0];
      symbol_with_update!(self, w, b0 as u32, cdf);
      if b0 {
        let b1_ctx = self.get_pred_ctx_brfarf2_or_arf(bo);
        let b1 = rf[0] == ALTREF_FRAME;

        let cdf = &self.fc.single_ref_cdfs[b1_ctx][1];
        symbol_with_update!(self, w, b1 as u32, cdf);
        if !b1 {
          let b5_ctx = self.get_pred_ctx_brf_or_arf2(bo);
          let b5 = rf[0] == ALTREF2_FRAME;

          let cdf = &self.fc.single_ref_cdfs[b5_ctx][5];
          symbol_with_update!(self, w, b5 as u32, cdf);
        }
      } else {
        let b2_ctx = self.get_pred_ctx_ll2_or_l3gld(bo);
        let b2 = rf[0] == LAST3_FRAME || rf[0] == GOLDEN_FRAME;

        let cdf = &self.fc.single_ref_cdfs[b2_ctx][2];
        symbol_with_update!(self, w, b2 as u32, cdf);
        if !b2 {
          let b3_ctx = self.get_pred_ctx_last_or_last2(bo);
          let b3 = rf[0] != LAST_FRAME;

          let cdf = &self.fc.single_ref_cdfs[b3_ctx][3];
          symbol_with_update!(self, w, b3 as u32, cdf);
        } else {
          let b4_ctx = self.get_pred_ctx_last3_or_gold(bo);
          let b4 = rf[0] != LAST3_FRAME;

          let cdf = &self.fc.single_ref_cdfs[b4_ctx][4];
          symbol_with_update!(self, w, b4 as u32, cdf);
        }
      }
    }
  }

  pub fn count_lrf_switchable<W: Writer>(
    &self, w: &W, rs: &TileRestorationState, filter: RestorationFilter,
    pli: usize,
  ) -> u32 {
    self.fc.count_lrf_switchable(w, rs, filter, pli)
  }

  /// # Panics
  ///
  /// - If the LRF has already been written for this superblock
  pub fn write_lrf<W: Writer>(
    &mut self, w: &mut W, rs: &mut TileRestorationStateMut,
    sbo: TileSuperBlockOffset, pli: usize,
  ) {
    let rp = &mut rs.planes[pli];
    if let Some(filter) = rp.restoration_unit(sbo, true).map(|ru| ru.filter) {
      match filter {
        RestorationFilter::None => match rp.rp_cfg.lrf_type {
          RESTORE_WIENER => {
            let cdf = &self.fc.lrf_wiener_cdf;
            symbol_with_update!(self, w, 0, cdf);
          }
          RESTORE_SGRPROJ => {
            let cdf = &self.fc.lrf_sgrproj_cdf;
            symbol_with_update!(self, w, 0, cdf);
          }
          RESTORE_SWITCHABLE => {
            let cdf = &self.fc.lrf_switchable_cdf;
            symbol_with_update!(self, w, 0, cdf);
          }
          RESTORE_NONE => {}
          _ => unreachable!(),
        },
        RestorationFilter::Sgrproj { set, xqd } => {
          match rp.rp_cfg.lrf_type {
            RESTORE_SGRPROJ => {
              let cdf = &self.fc.lrf_sgrproj_cdf;
              symbol_with_update!(self, w, 1, cdf);
            }
            RESTORE_SWITCHABLE => {
              // Does *not* write 'RESTORE_SGRPROJ'
              let cdf = &self.fc.lrf_switchable_cdf;
              symbol_with_update!(self, w, 2, cdf);
            }
            _ => unreachable!(),
          }
          w.literal(SGRPROJ_PARAMS_BITS, set as u32);
          for i in 0..2 {
            let s = SGRPROJ_PARAMS_S[set as usize][i];
            let min = SGRPROJ_XQD_MIN[i] as i32;
            let max = SGRPROJ_XQD_MAX[i] as i32;
            if s > 0 {
              w.write_signed_subexp_with_ref(
                xqd[i] as i32,
                min,
                max + 1,
                SGRPROJ_PRJ_SUBEXP_K,
                rp.sgrproj_ref[i] as i32,
              );
              rp.sgrproj_ref[i] = xqd[i];
            } else {
              // Nothing written, just update the reference
              if i == 0 {
                assert!(xqd[i] == 0);
                rp.sgrproj_ref[0] = 0;
              } else {
                rp.sgrproj_ref[1] = 95; // LOL at spec.  The result is always 95.
              }
            }
          }
        }
        RestorationFilter::Wiener { coeffs } => {
          match rp.rp_cfg.lrf_type {
            RESTORE_WIENER => {
              let cdf = &self.fc.lrf_wiener_cdf;
              symbol_with_update!(self, w, 1, cdf);
            }
            RESTORE_SWITCHABLE => {
              // Does *not* write 'RESTORE_WIENER'
              let cdf = &self.fc.lrf_switchable_cdf;
              symbol_with_update!(self, w, 1, cdf);
            }
            _ => unreachable!(),
          }
          for pass in 0..2 {
            let first_coeff = if pli == 0 {
              0
            } else {
              assert!(coeffs[pass][0] == 0);
              1
            };
            for i in first_coeff..3 {
              let min = WIENER_TAPS_MIN[i] as i32;
              let max = WIENER_TAPS_MAX[i] as i32;
              w.write_signed_subexp_with_ref(
                coeffs[pass][i] as i32,
                min,
                max + 1,
                (i + 1) as u8,
                rp.wiener_ref[pass][i] as i32,
              );
              rp.wiener_ref[pass][i] = coeffs[pass][i];
            }
          }
        }
      }
    }
  }

  pub fn write_cdef<W: Writer>(
    &mut self, w: &mut W, strength_index: u8, bits: u8,
  ) {
    w.literal(bits, strength_index as u32);
  }
}
