/*
 * DCA compatible decoder data
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_DCADATA_H
#define AVCODEC_DCADATA_H

#include <stdint.h>

#include "dcahuff.h"

#define DCA_ADPCM_COEFFS        4
#define DCA_ADPCM_VQCODEBOOK_SZ 4096

extern const uint32_t ff_dca_bit_rates[32];

extern const uint8_t ff_dca_channels[16];

extern const uint8_t ff_dca_dmix_primary_nch[8];

extern const uint8_t ff_dca_quant_index_sel_nbits[DCA_CODE_BOOKS];
extern const uint8_t ff_dca_quant_index_group_size[DCA_CODE_BOOKS];

extern const int16_t ff_dca_adpcm_vb[DCA_ADPCM_VQCODEBOOK_SZ][DCA_ADPCM_COEFFS];

extern const uint32_t ff_dca_scale_factor_quant6[64];
extern const uint32_t ff_dca_scale_factor_quant7[128];

extern const uint32_t ff_dca_joint_scale_factors[129];

extern const uint32_t ff_dca_scale_factor_adj[4];

extern const uint32_t ff_dca_quant_levels[32];

extern const uint32_t ff_dca_lossy_quant[32];

extern const uint32_t ff_dca_lossless_quant[32];

extern const int8_t ff_dca_high_freq_vq[1024][32];

extern const float ff_dca_fir_32bands_perfect[512];
extern const float ff_dca_fir_32bands_nonperfect[512];

extern const float ff_dca_lfe_fir_64[256];
extern const float ff_dca_lfe_fir_128[256];
extern const float ff_dca_fir_64bands[1024];

extern const int32_t ff_dca_fir_32bands_perfect_fixed[512];
extern const int32_t ff_dca_fir_32bands_nonperfect_fixed[512];
extern const int32_t ff_dca_lfe_fir_64_fixed[256];
extern const int32_t ff_dca_fir_64bands_fixed[1024];

#define FF_DCA_DMIXTABLE_SIZE       242U
#define FF_DCA_INV_DMIXTABLE_SIZE   201U
#define FF_DCA_DMIXTABLE_OFFSET     (FF_DCA_DMIXTABLE_SIZE - FF_DCA_INV_DMIXTABLE_SIZE)

extern const uint16_t ff_dca_dmixtable[FF_DCA_DMIXTABLE_SIZE];
extern const uint32_t ff_dca_inv_dmixtable[FF_DCA_INV_DMIXTABLE_SIZE];

extern const uint16_t ff_dca_xll_refl_coeff[128];

extern const int32_t ff_dca_xll_band_coeff[20];

extern const uint16_t ff_dca_avg_g3_freqs[3];

extern const uint16_t ff_dca_fst_amp[44];

extern const uint8_t ff_dca_freq_to_sb[32];

extern const int8_t ff_dca_ph0_shift[8];

extern const uint8_t ff_dca_grid_1_to_scf[11];
extern const uint8_t ff_dca_grid_2_to_scf[3];

extern const uint8_t ff_dca_scf_to_grid_1[32];
extern const uint8_t ff_dca_scf_to_grid_2[32];

extern const uint8_t ff_dca_grid_1_weights[12][32];

extern const uint8_t ff_dca_sb_reorder[8][8];

extern const int8_t ff_dca_lfe_delta_index_16[8];
extern const int8_t ff_dca_lfe_delta_index_24[32];

extern const uint16_t ff_dca_rsd_pack_5_in_8[256];
extern const uint8_t ff_dca_rsd_pack_3_in_7[128][3];

extern const float ff_dca_rsd_level_2a[2];
extern const float ff_dca_rsd_level_2b[2];
extern const float ff_dca_rsd_level_3[3];
extern const float ff_dca_rsd_level_5[5];
extern const float ff_dca_rsd_level_8[8];
extern const float ff_dca_rsd_level_16[16];

extern const float ff_dca_synth_env[32];

extern const float ff_dca_corr_cf[32][11];

extern const float ff_dca_quant_amp[57];

extern const float ff_dca_st_coeff[34];

extern const float ff_dca_long_window[128];

extern const float ff_dca_lfe_step_size_16[101];
extern const float ff_dca_lfe_step_size_24[144];

extern const float ff_dca_bank_coeff[10];
extern const float ff_dca_lfe_iir[5][4];

#endif /* AVCODEC_DCADATA_H */
