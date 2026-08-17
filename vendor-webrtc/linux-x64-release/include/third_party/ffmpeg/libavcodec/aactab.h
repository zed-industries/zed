/*
 * AAC data declarations
 * Copyright (c) 2005-2006 Oded Shimon ( ods15 ods15 dyndns org )
 * Copyright (c) 2006-2007 Maxim Gavrilov ( maxim.gavrilov gmail com )
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

/**
 * @file
 * AAC data declarations
 * @author Oded Shimon  ( ods15 ods15 dyndns org )
 * @author Maxim Gavrilov ( maxim.gavrilov gmail com )
 */

#ifndef AVCODEC_AACTAB_H
#define AVCODEC_AACTAB_H

#include "libavutil/mem_internal.h"

#include <stdint.h>

/* NOTE:
 * Tables in this file are shared by the AAC decoders and encoder
 */

extern float ff_aac_pow2sf_tab[428];
extern float ff_aac_pow34sf_tab[428];

/* @name ltp_coef
 * Table of the LTP coefficients
 */
extern const float ff_ltp_coef[8];

/* @name tns_tmp2_map
 * Tables of the tmp2[] arrays of LPC coefficients used for TNS.
 * @{
 */
extern const float *const ff_tns_tmp2_map[4];
// @}

/* @name window coefficients
 * @{
 */
DECLARE_ALIGNED(32, extern float,  ff_aac_kbd_long_1024)[1024];
DECLARE_ALIGNED(32, extern float,  ff_aac_kbd_short_128)[128];
DECLARE_ALIGNED(32, extern const float, ff_aac_eld_window_512)[1920];
DECLARE_ALIGNED(32, extern const int,   ff_aac_eld_window_512_fixed)[1920];
DECLARE_ALIGNED(32, extern const float, ff_aac_eld_window_480)[1800];
DECLARE_ALIGNED(32, extern const int,   ff_aac_eld_window_480_fixed)[1800];
// @}

extern const float ff_aac_deemph_weights[16];

/* Initializes data shared between float decoder and encoder. */
void ff_aac_float_common_init(void);

/* @name number of scalefactor window bands for long and short transform windows respectively
 * @{
 */
extern const uint8_t ff_aac_num_swb_1024[];
extern const uint8_t ff_aac_num_swb_960 [];
extern const uint8_t ff_aac_num_swb_768 [];
extern const uint8_t ff_aac_num_swb_512 [];
extern const uint8_t ff_aac_num_swb_480 [];
extern const uint8_t ff_aac_num_swb_128 [];
extern const uint8_t ff_aac_num_swb_120 [];
extern const uint8_t ff_aac_num_swb_96  [];
// @}

extern const uint8_t ff_aac_pred_sfb_max [];

extern const uint32_t ff_aac_scalefactor_code[121];
extern const uint8_t  ff_aac_scalefactor_bits[121];

extern const uint16_t * const ff_aac_spectral_codes[11];
extern const uint8_t  * const ff_aac_spectral_bits [11];
extern const uint16_t  ff_aac_spectral_sizes[11];

extern const float *const ff_aac_codebook_vectors[];
extern const float *const ff_aac_codebook_vector_vals[];
extern const uint16_t *const ff_aac_codebook_vector_idx[];

extern const uint16_t ff_aac_ac_msb_cdfs[64][17];
extern const uint16_t ff_aac_ac_lsb_cdfs[3][4];
extern const uint8_t ff_aac_ac_lookup_m[742];
extern const uint32_t ff_aac_ac_hash_m[742];
extern const uint16_t ff_aac_ac_cf_m[64][17];

extern const uint16_t * const ff_swb_offset_1024[13];
extern const uint16_t * const ff_swb_offset_960 [13];
extern const uint16_t * const ff_swb_offset_768 [13];
extern const uint16_t * const ff_swb_offset_512 [13];
extern const uint16_t * const ff_swb_offset_480 [13];
extern const uint16_t * const ff_swb_offset_128 [13];
extern const uint16_t * const ff_swb_offset_120 [13];
extern const uint16_t * const ff_swb_offset_96  [13];

extern const uint8_t ff_tns_max_bands_1024[13];
extern const uint8_t ff_tns_max_bands_512 [13];
extern const uint8_t ff_tns_max_bands_480 [13];
extern const uint8_t ff_tns_max_bands_128 [13];

extern const uint8_t ff_tns_max_bands_usac_1024[13];
extern const uint8_t ff_tns_max_bands_usac_128[13];

/* [x][y], x == 1 -> frame len is 768 frames, y == 1 -> is eight_short */
extern const uint8_t ff_usac_noise_fill_start_offset[2][2];

extern const int ff_aac_usac_samplerate[32];

/* Window type (only long+eight, start/stop/stopstart), sine+sine, kbd+kbd, sine+kbd, kbd+sine */
extern const float ff_aac_usac_mdst_filt_cur[4 /* Window */][4 /* Shape */][7];
/* Window type (everything/longstop+stopstart), sine or kbd */
extern const float ff_aac_usac_mdst_filt_prev[2 /* Window */][2 /* sine/kbd */][7];

#endif /* AVCODEC_AACTAB_H */
