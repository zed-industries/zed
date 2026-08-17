/*
 * VC-1 and WMV3 decoder
 * copyright (c) 2006 Konstantin Shishkov
 * (c) 2005 anonymous, Alex Beregszaszi, Michael Niedermayer
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
 * VC-1 tables.
 */

#ifndef AVCODEC_VC1DATA_H
#define AVCODEC_VC1DATA_H

#include <stdint.h>

#include "libavutil/attributes_internal.h"
#include "libavutil/rational.h"

#include "vlc.h"

FF_VISIBILITY_PUSH_HIDDEN
/** Table for conversion between TTBLK and TTMB */
extern const int ff_vc1_ttblk_to_tt[3][8];

extern const int ff_vc1_ttfrm_to_tt[4];

/** MV P mode - the 5th element is only used for mode 1 */
extern const uint8_t ff_vc1_mv_pmode_table[2][5];
extern const uint8_t ff_vc1_mv_pmode_table2[2][4];

extern const int ff_vc1_fps_nr[7], ff_vc1_fps_dr[2];
extern const uint8_t ff_vc1_pquant_table[3][32];

/* MBMODE table for interlaced frame P-picture */
extern const uint8_t ff_vc1_mbmode_intfrp[2][15][4];

/** @name VC-1 VLC tables and defines
 *  @todo TODO move this into the context
 */
//@{
#define VC1_IMODE_VLC_BITS 4
extern VLCElem ff_vc1_imode_vlc[1 << VC1_IMODE_VLC_BITS];
#define VC1_NORM2_VLC_BITS 3
extern VLCElem ff_vc1_norm2_vlc[1 << VC1_NORM2_VLC_BITS];
#define VC1_NORM6_VLC_BITS 9
extern VLCElem ff_vc1_norm6_vlc[556];
/* Could be optimized, one table only needs 8 bits */
#define VC1_TTMB_VLC_BITS 9 //12
extern const VLCElem *ff_vc1_ttmb_vlc[3];
#define VC1_MV_DIFF_VLC_BITS 9 //15
extern const VLCElem *ff_vc1_mv_diff_vlc[4];
#define VC1_CBPCY_P_VLC_BITS 9 //14
extern const VLCElem *ff_vc1_cbpcy_p_vlc[4];
#define VC1_ICBPCY_VLC_BITS 9
extern const VLCElem *ff_vc1_icbpcy_vlc[8];
#define VC1_4MV_BLOCK_PATTERN_VLC_BITS 6
extern const VLCElem *ff_vc1_4mv_block_pattern_vlc[4];
#define VC1_2MV_BLOCK_PATTERN_VLC_BITS 3
extern const VLCElem *ff_vc1_2mv_block_pattern_vlc[4];
#define VC1_TTBLK_VLC_BITS 5
extern const VLCElem *ff_vc1_ttblk_vlc[3];
#define VC1_SUBBLKPAT_VLC_BITS 6
extern const VLCElem *ff_vc1_subblkpat_vlc[3];
#define VC1_INTFR_4MV_MBMODE_VLC_BITS 9
extern const VLCElem *ff_vc1_intfr_4mv_mbmode_vlc[4];
#define VC1_INTFR_NON4MV_MBMODE_VLC_BITS 6
extern const VLCElem *ff_vc1_intfr_non4mv_mbmode_vlc[4];
#define VC1_IF_MMV_MBMODE_VLC_BITS 5
extern const VLCElem *ff_vc1_if_mmv_mbmode_vlc[8];
#define VC1_IF_1MV_MBMODE_VLC_BITS 5
extern const VLCElem *ff_vc1_if_1mv_mbmode_vlc[8];
#define VC1_1REF_MVDATA_VLC_BITS 9
extern const VLCElem *ff_vc1_1ref_mvdata_vlc[4];
#define VC1_2REF_MVDATA_VLC_BITS 9
extern const VLCElem *ff_vc1_2ref_mvdata_vlc[8];

extern const VLCElem *ff_vc1_ac_coeff_table[8];

#define VC1_IF_MBMODE_VLC_BITS 5    // as a placeholder for VC1_IF_MMV_MBMODE_VLC_BITS
                                    // or VC1_IF_1MV_MBMODE_VLC_BITS since they are the same
//@}

#define B_FRACTION_DEN  256

/* pre-computed scales for all bfractions and base=256 */
extern const int16_t ff_vc1_bfraction_lut[23];

//Same as H.264
extern const AVRational ff_vc1_pixel_aspect[16];

extern const uint8_t ff_wmv3_dc_scale_table[32];

/* DC differentials low+hi-mo, p217 are the same as in msmpeg4data .h */

/* Scantables/ZZ scan are at 11.9 (p262) and 8.1.1.12 (p10) */
extern const uint8_t ff_vc1_simple_progressive_4x4_zz [16];
extern const uint8_t ff_vc1_adv_progressive_8x4_zz [32];
extern const uint8_t ff_vc1_adv_progressive_4x8_zz [32];
extern const uint8_t ff_vc1_adv_interlaced_8x8_zz [64];
extern const uint8_t ff_vc1_adv_interlaced_8x4_zz [32];
extern const uint8_t ff_vc1_adv_interlaced_4x8_zz [32];
extern const uint8_t ff_vc1_adv_interlaced_4x4_zz [16];

/* DQScale as specified in 8.1.3.9 - almost identical to 0x40000/i */
extern const int32_t ff_vc1_dqscale[63];

/* P Interlaced field picture MV predictor scaling values (Table 114) */
extern const uint16_t ff_vc1_field_mvpred_scales[2][7][4];
/* B Interlaced field picture backward MV predictor scaling values for first field (Table 115) */
extern const uint16_t ff_vc1_b_field_mvpred_scales[7][4];

#define AC_MODES 8

extern const int ff_vc1_ac_sizes[AC_MODES];
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_VC1DATA_H */
