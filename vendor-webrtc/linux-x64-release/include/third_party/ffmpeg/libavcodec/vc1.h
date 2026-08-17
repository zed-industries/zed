/*
 * VC-1 and WMV3 decoder
 * Copyright (c) 2006-2007 Konstantin Shishkov
 * Partly based on vc9.c (c) 2005 Anonymous, Alex Beregszaszi, Michael Niedermayer
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

#ifndef AVCODEC_VC1_H
#define AVCODEC_VC1_H

#include "avcodec.h"
#include "h264chroma.h"
#include "mpegvideo.h"
#include "intrax8.h"
#include "vc1_common.h"
#include "vc1dsp.h"

#define AC_VLC_BITS 9

/** Sequence quantizer mode */
//@{
enum QuantMode {
    QUANT_FRAME_IMPLICIT,    ///< Implicitly specified at frame level
    QUANT_FRAME_EXPLICIT,    ///< Explicitly specified at frame level
    QUANT_NON_UNIFORM,       ///< Non-uniform quant used for all frames
    QUANT_UNIFORM            ///< Uniform quant used for all frames
};
//@}

/** Where quant can be changed */
//@{
enum DQProfile {
    DQPROFILE_FOUR_EDGES,
    DQPROFILE_DOUBLE_EDGES,
    DQPROFILE_SINGLE_EDGE,
    DQPROFILE_ALL_MBS
};
//@}

/** @name Where quant can be changed
 */
//@{
enum DQSingleEdge {
    DQSINGLE_BEDGE_LEFT,
    DQSINGLE_BEDGE_TOP,
    DQSINGLE_BEDGE_RIGHT,
    DQSINGLE_BEDGE_BOTTOM
};
//@}

/** Which pair of edges is quantized with ALTPQUANT */
//@{
enum DQDoubleEdge {
    DQDOUBLE_BEDGE_TOPLEFT,
    DQDOUBLE_BEDGE_TOPRIGHT,
    DQDOUBLE_BEDGE_BOTTOMRIGHT,
    DQDOUBLE_BEDGE_BOTTOMLEFT
};
//@}

/** MV modes for P-frames */
//@{
enum MVModes {
    MV_PMODE_1MV_HPEL_BILIN,
    MV_PMODE_1MV,
    MV_PMODE_1MV_HPEL,
    MV_PMODE_MIXED_MV,
    MV_PMODE_INTENSITY_COMP
};
//@}

/** MBMODE for interlaced frame P-picture */
//@{
enum MBModesIntfr {
    MV_PMODE_INTFR_1MV,
    MV_PMODE_INTFR_2MV_FIELD,
    MV_PMODE_INTFR_2MV,
    MV_PMODE_INTFR_4MV_FIELD,
    MV_PMODE_INTFR_4MV,
    MV_PMODE_INTFR_INTRA,
};
//@}

/** @name MV types for B-frames */
//@{
enum BMVTypes {
    BMV_TYPE_BACKWARD,
    BMV_TYPE_FORWARD,
    BMV_TYPE_INTERPOLATED,
    BMV_TYPE_DIRECT
};
//@}

/** @name Block types for P/B-frames */
//@{
enum TransformTypes {
    TT_8X8,
    TT_8X4_BOTTOM,
    TT_8X4_TOP,
    TT_8X4,         // both halves
    TT_4X8_RIGHT,
    TT_4X8_LEFT,
    TT_4X8,         // both halves
    TT_4X4
};
//@}

enum CodingSet {
    CS_HIGH_MOT_INTRA = 0,
    CS_HIGH_MOT_INTER,
    CS_LOW_MOT_INTRA,
    CS_LOW_MOT_INTER,
    CS_MID_RATE_INTRA,
    CS_MID_RATE_INTER,
    CS_HIGH_RATE_INTRA,
    CS_HIGH_RATE_INTER
};

/** @name Overlap conditions for Advanced Profile */
//@{
enum COTypes {
    CONDOVER_NONE = 0,
    CONDOVER_ALL,
    CONDOVER_SELECT
};
//@}

/**
 * FCM Frame Coding Mode
 * @note some content might be marked interlaced
 * but have fcm set to 0 as well (e.g. HD-DVD)
 */
enum FrameCodingMode {
    PROGRESSIVE = 0,    ///<  in the bitstream is reported as 00b
    ILACE_FRAME,        ///<  in the bitstream is reported as 10b
    ILACE_FIELD         ///<  in the bitstream is reported as 11b
};

/**
 * Imode types
 * @{
 */
enum Imode {
    IMODE_RAW,
    IMODE_NORM2,
    IMODE_DIFF2,
    IMODE_NORM6,
    IMODE_DIFF6,
    IMODE_ROWSKIP,
    IMODE_COLSKIP
};
/** @} */ //imode defines

/** The VC1 Context
 * @todo Change size wherever another size is more efficient
 * Many members are only used for Advanced Profile
 */
typedef struct VC1Context{
    MpegEncContext s;
    IntraX8Context x8;
    H264ChromaContext h264chroma;
    VC1DSPContext vc1dsp;

    /** Simple/Main Profile sequence header */
    //@{
    int res_sprite;       ///< reserved, sprite mode
    int res_y411;         ///< reserved, old interlaced mode
    int res_x8;           ///< reserved
    int multires;         ///< frame-level RESPIC syntax element present
    int res_fasttx;       ///< reserved, always 1
    int res_transtab;     ///< reserved, always 0
    int rangered;         ///< RANGEREDFRM (range reduction) syntax element present
                          ///< at frame level
    int res_rtm_flag;     ///< reserved, set to 1
    int reserved;         ///< reserved
    //@}

    /** Advanced Profile */
    //@{
    int level;            ///< 3 bits, for Advanced/Simple Profile, provided by TS layer
    int chromaformat;     ///< 2 bits, 2=4:2:0, only defined
    int postprocflag;     ///< Per-frame processing suggestion flag present
    int broadcast;        ///< TFF/RFF present
    int interlace;        ///< Progressive/interlaced (RPTFTM syntax element)
    int tfcntrflag;       ///< TFCNTR present
    int panscanflag;      ///< NUMPANSCANWIN, TOPLEFT{X,Y}, BOTRIGHT{X,Y} present
    int refdist_flag;     ///< REFDIST syntax element present in II, IP, PI or PP field picture headers
    int extended_dmv;     ///< Additional extended dmv range at P/B-frame-level
    int color_prim;       ///< 8 bits, chroma coordinates of the color primaries
    int transfer_char;    ///< 8 bits, Opto-electronic transfer characteristics
    int matrix_coef;      ///< 8 bits, Color primaries->YCbCr transform matrix
    int hrd_param_flag;   ///< Presence of Hypothetical Reference
                          ///< Decoder parameters
    int psf;              ///< Progressive Segmented Frame
    //@}

    /** Sequence header data for all Profiles
     * TODO: choose between ints, uint8_ts and monobit flags
     */
    //@{
    int profile;          ///< 2 bits, Profile
    int frmrtq_postproc;  ///< 3 bits,
    int bitrtq_postproc;  ///< 5 bits, quantized framerate-based postprocessing strength
    int max_coded_width, max_coded_height;
    int fastuvmc;         ///< Rounding of qpel vector to hpel ? (not in Simple)
    int extended_mv;      ///< Ext MV in P/B (not in Simple)
    int dquant;           ///< How qscale varies with MBs, 2 bits (not in Simple)
    int vstransform;      ///< variable-size [48]x[48] transform type + info
    int overlap;          ///< overlapped transforms in use
    int max_b_frames;     ///< max number of B-frames
    int quantizer_mode;   ///< 2 bits, quantizer mode used for sequence, see QUANT_*
    int finterpflag;      ///< INTERPFRM present
    //@}

    /** Frame decoding info for all profiles */
    //@{
    uint8_t mv_mode;             ///< MV coding mode
    uint8_t mv_mode2;            ///< Secondary MV coding mode (B-frames)
    int k_x;                     ///< Number of bits for MVs (depends on MV range)
    int k_y;                     ///< Number of bits for MVs (depends on MV range)
    int range_x, range_y;        ///< MV range
    uint8_t pq, altpq;           ///< Current/alternate frame quantizer scale
    uint8_t zz_8x8[4][64];       ///< Zigzag table for TT_8x8, permuted for IDCT
    int left_blk_sh, top_blk_sh; ///< Either 3 or 0, positions of l/t in blk[]
    const uint8_t* zz_8x4;       ///< Zigzag scan table for TT_8x4 coding mode
    const uint8_t* zz_4x8;       ///< Zigzag scan table for TT_4x8 coding mode
    /** pquant parameters */
    //@{
    uint8_t dquantfrm;
    uint8_t dqprofile;
    uint8_t dqsbedge;
    uint8_t dqbilevel;
    //@}
    /** AC coding set indexes
     * @see 8.1.1.10, p(1)10
     */
    //@{
    int c_ac_table_index;    ///< Chroma index from ACFRM element
    int y_ac_table_index;    ///< Luma index from AC2FRM element
    //@}
    int ttfrm;               ///< Transform type info present at frame level
    uint8_t ttmbf;           ///< Transform type flag
    int *ttblk_base, *ttblk; ///< Transform type at the block level
    int codingset;           ///< index of current table set from 11.8 to use for luma block decoding
    int codingset2;          ///< index of current table set from 11.8 to use for chroma block decoding
    int pqindex;             ///< raw pqindex used in coding set selection
    int a_avail, c_avail;
    uint8_t *mb_type_base, *mb_type[3];


    /** Luma compensation parameters */
    //@{
    uint8_t lumscale;
    uint8_t lumshift;
    //@}
    int16_t bfraction;    ///< Relative position % anchors=> how to scale MVs
    uint8_t halfpq;       ///< Uniform quant over image and qp+.5
    uint8_t respic;       ///< Frame-level flag for resized images
    int buffer_fullness;  ///< HRD info
    /** Ranges:
     * -# 0 -> [-64n 63.f] x [-32, 31.f]
     * -# 1 -> [-128, 127.f] x [-64, 63.f]
     * -# 2 -> [-512, 511.f] x [-128, 127.f]
     * -# 3 -> [-1024, 1023.f] x [-256, 255.f]
     */
    uint8_t mvrange;                ///< Extended MV range flag
    uint8_t pquantizer;             ///< Uniform (over sequence) quantizer in use
    const VLCElem *cbpcy_vlc;       ///< CBPCY VLC table
    int tt_index;                   ///< Index for Transform Type tables (to decode TTMB)
    uint8_t* mv_type_mb_plane;      ///< bitplane for mv_type == (4MV)
    uint8_t* direct_mb_plane;       ///< bitplane for "direct" MBs
    uint8_t* forward_mb_plane;      ///< bitplane for "forward" MBs
    int mv_type_is_raw;             ///< mv type mb plane is not coded
    int dmb_is_raw;                 ///< direct mb plane is raw
    int fmb_is_raw;                 ///< forward mb plane is raw
    int skip_is_raw;                ///< skip mb plane is not coded
    uint8_t last_luty[2][256], last_lutuv[2][256];  ///< lookup tables used for intensity compensation
    uint8_t  aux_luty[2][256],  aux_lutuv[2][256];  ///< lookup tables used for intensity compensation
    uint8_t next_luty[2][256], next_lutuv[2][256];  ///< lookup tables used for intensity compensation
    uint8_t (*curr_luty)[256]  ,(*curr_lutuv)[256];
    int last_use_ic, *curr_use_ic, next_use_ic, aux_use_ic;
    int last_interlaced, next_interlaced; ///< whether last_pic, next_pic is interlaced
    int rnd;                        ///< rounding control
    int cbptab;

    /** Frame decoding info for S/M profiles only */
    //@{
    uint8_t rangeredfrm;            ///< out_sample = CLIP((in_sample-128)*2+128)
    uint8_t interpfrm;
    //@}

    /** Frame decoding info for Advanced profile */
    //@{
    enum FrameCodingMode fcm;
    uint8_t numpanscanwin;
    uint8_t tfcntr;
    uint8_t rptfrm, tff, rff;
    uint16_t topleftx;
    uint16_t toplefty;
    uint16_t bottomrightx;
    uint16_t bottomrighty;
    uint8_t uvsamp;
    uint8_t postproc;
    int hrd_num_leaky_buckets;
    uint8_t bit_rate_exponent;
    uint8_t buffer_size_exponent;
    uint8_t* acpred_plane;       ///< AC prediction flags bitplane
    int acpred_is_raw;
    uint8_t* over_flags_plane;   ///< Overflags bitplane
    int overflg_is_raw;
    uint8_t condover;
    uint8_t range_mapy_flag;
    uint8_t range_mapuv_flag;
    uint8_t range_mapy;
    uint8_t range_mapuv;
    //@}

    /** Frame decoding info for interlaced picture */
    uint8_t dmvrange;   ///< Extended differential MV range flag
    int fourmvswitch;
    int intcomp;
    uint8_t lumscale2;  ///< for interlaced field P picture
    uint8_t lumshift2;
    const VLCElem *mbmode_vlc;
    const VLCElem *imv_vlc;
    const VLCElem *twomvbp_vlc;
    const VLCElem *fourmvbp_vlc;
    uint8_t twomvbp;
    uint8_t fourmvbp;
    uint8_t* fieldtx_plane;
    int fieldtx_is_raw;
    uint8_t zzi_8x8[64];
    uint8_t *blk_mv_type_base, *blk_mv_type;    ///< 0: frame MV, 1: field MV (interlaced frame)
    uint8_t *mv_f_base, *mv_f[2];               ///< 0: MV obtained from same field, 1: opposite field
    uint8_t *mv_f_next_base, *mv_f_next[2];
    int field_mode;         ///< 1 for interlaced field pictures
    int fptype;
    int second_field;
    int refdist;            ///< distance of the current picture from reference
    int numref;             ///< number of past field pictures used as reference
                            // 0 corresponds to 1 and 1 corresponds to 2 references
    int reffield;           ///< if numref = 0 (1 reference) then reffield decides which
                            // field to use among the two fields from previous frame
    int intcompfield;       ///< which of the two fields to be intensity compensated
                            // 0: both fields, 1: bottom field, 2: top field
    int cur_field_type;     ///< 0: top, 1: bottom
    int ref_field_type[2];  ///< forward and backward reference field type (top or bottom)
    int blocks_off, mb_off;
    int qs_last;            ///< if qpel has been used in the previous (tr.) picture
    int bmvtype;
    int frfd, brfd;         ///< reference frame distance (forward or backward)
    int first_pic_header_flag;
    int pic_header_flag;
    int mbmodetab;
    int icbptab;
    int imvtab;
    int twomvbptab;
    int fourmvbptab;

    /** Frame decoding info for sprite modes */
    //@{
    int new_sprite;
    int two_sprites;
    AVFrame *sprite_output_frame;
    int output_width, output_height, sprite_width, sprite_height;
    uint8_t* sr_rows[2][2];      ///< Sprite resizer line cache
    //@}

    int p_frame_skipped;
    int bi_type;
    int x8_type;

    int16_t (*block)[6][64];
    int n_allocated_blks, cur_blk_idx, left_blk_idx, topleft_blk_idx, top_blk_idx;
    uint32_t *cbp_base, *cbp;
    uint8_t *is_intra_base, *is_intra;
    int16_t (*luma_mv_base)[2], (*luma_mv)[2];
    uint8_t bfraction_lut_index; ///< Index for BFRACTION value (see Table 40, reproduced into ff_vc1_bfraction_lut[])
    uint8_t broken_link;         ///< Broken link flag (BROKEN_LINK syntax element)
    uint8_t closed_entry;        ///< Closed entry point flag (CLOSED_ENTRY syntax element)

    int end_mb_x;                ///< Horizontal macroblock limit (used only by mss2)

    int parse_only;              ///< Context is used within parser
    int resync_marker;           ///< could this stream contain resync markers
} VC1Context;

/**
 * Decode Simple/Main Profiles sequence header
 * @see Figure 7-8, p16-17
 * @param avctx Codec context
 * @param gb GetBit context initialized from Codec context extra_data
 * @return Status
 */
int ff_vc1_decode_sequence_header(AVCodecContext *avctx, VC1Context *v, GetBitContext *gb);

int ff_vc1_decode_entry_point(AVCodecContext *avctx, VC1Context *v, GetBitContext *gb);

int ff_vc1_parse_frame_header    (VC1Context *v, GetBitContext *gb);
int ff_vc1_parse_frame_header_adv(VC1Context *v, GetBitContext *gb);
void ff_vc1_init_common(VC1Context *v);

int  ff_vc1_decode_init(AVCodecContext *avctx);
void ff_vc1_init_transposed_scantables(VC1Context *v);
int  ff_vc1_decode_end(AVCodecContext *avctx);
void ff_vc1_decode_blocks(VC1Context *v);

void ff_vc1_i_overlap_filter(VC1Context *v);
void ff_vc1_p_overlap_filter(VC1Context *v);
void ff_vc1_i_loop_filter(VC1Context *v);
void ff_vc1_p_loop_filter(VC1Context *v);
void ff_vc1_p_intfr_loop_filter(VC1Context *v);
void ff_vc1_b_intfi_loop_filter(VC1Context *v);

void ff_vc1_mc_1mv(VC1Context *v, int dir);
void ff_vc1_mc_4mv_luma(VC1Context *v, int n, int dir, int avg);
void ff_vc1_mc_4mv_chroma(VC1Context *v, int dir);
void ff_vc1_mc_4mv_chroma4(VC1Context *v, int dir, int dir2, int avg);

void ff_vc1_interp_mc(VC1Context *v);

#endif /* AVCODEC_VC1_H */
