/*
 * Generic DCT based hybrid video encoder
 * Copyright (c) 2000, 2001, 2002 Fabrice Bellard
 * Copyright (c) 2002-2004 Michael Niedermayer
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
 * mpegvideo header.
 */

#ifndef AVCODEC_MPEGVIDEO_H
#define AVCODEC_MPEGVIDEO_H

#include "blockdsp.h"
#include "error_resilience.h"
#include "fdctdsp.h"
#include "get_bits.h"
#include "h264chroma.h"
#include "h263dsp.h"
#include "hpeldsp.h"
#include "idctdsp.h"
#include "me_cmp.h"
#include "motion_est.h"
#include "mpegpicture.h"
#include "mpegvideoencdsp.h"
#include "pixblockdsp.h"
#include "put_bits.h"
#include "ratecontrol.h"
#include "qpeldsp.h"
#include "videodsp.h"

#define MAX_THREADS 32

#define MAX_B_FRAMES 16

/**
 * Scantable.
 */
typedef struct ScanTable {
    const uint8_t *scantable;
    uint8_t permutated[64];
    uint8_t raster_end[64];
} ScanTable;

enum OutputFormat {
    FMT_MPEG1,
    FMT_H261,
    FMT_H263,
    FMT_MJPEG,
    FMT_SPEEDHQ,
};

/**
 * MpegEncContext.
 */
typedef struct MpegEncContext {
    AVClass *class;

    int y_dc_scale, c_dc_scale;
    int ac_pred;
    int block_last_index[12];  ///< last non zero coefficient in block
    int h263_aic;              ///< Advanced INTRA Coding (AIC)

    /* scantables */
    ScanTable inter_scantable; ///< if inter == intra then intra should be used to reduce the cache usage

    /* WARNING: changes above this line require updates to hardcoded
     *          offsets used in ASM. */

    ScanTable intra_scantable;
    uint8_t permutated_intra_h_scantable[64];
    uint8_t permutated_intra_v_scantable[64];

    struct AVCodecContext *avctx;
    /* The following pointer is intended for codecs sharing code
     * between decoder and encoder and in need of a common context to do so. */
    void *private_ctx;
    /* the following parameters must be initialized before encoding */
    int width, height;///< picture size. must be a multiple of 16
    int gop_size;
    int intra_only;   ///< if true, only intra pictures are generated
    int64_t bit_rate; ///< wanted bit rate
    enum OutputFormat out_format; ///< output format
    int h263_pred;    ///< use MPEG-4/H.263 ac/dc predictions
    int pb_frame;     ///< PB-frame mode (0 = none, 1 = base, 2 = improved)

/* the following codec id fields are deprecated in favor of codec_id */
    int h263_plus;    ///< H.263+ headers
    int h263_flv;     ///< use flv H.263 header

    enum AVCodecID codec_id;     /* see AV_CODEC_ID_xxx */
    int fixed_qscale; ///< fixed qscale if non zero
    int encoding;     ///< true if we are encoding (vs decoding)
    int max_b_frames; ///< max number of B-frames for encoding
    int luma_elim_threshold;
    int chroma_elim_threshold;
    int workaround_bugs;       ///< workaround bugs in encoders which cannot be detected automatically
    int codec_tag;             ///< internal codec_tag upper case converted from avctx codec_tag
    /* the following fields are managed internally by the encoder */

    /* sequence parameters */
    int context_initialized;
    int input_picture_number;  ///< used to set pic->display_picture_number, should not be used for/by anything else
    int coded_picture_number;  ///< used to set pic->coded_picture_number, should not be used for/by anything else
    int picture_number;       //FIXME remove, unclear definition
    int picture_in_gop_number; ///< 0-> first pic in gop, ...
    int mb_width, mb_height;   ///< number of MBs horizontally & vertically
    int mb_stride;             ///< mb_width+1 used for some arrays to allow simple addressing of left & top MBs without sig11
    int b8_stride;             ///< 2*mb_width+1 used for some 8x8 block arrays to allow simple addressing
    int h_edge_pos, v_edge_pos;///< horizontal / vertical position of the right/bottom edge (pixel replication)
    int mb_num;                ///< number of MBs of a picture
    ptrdiff_t linesize;        ///< line size, in bytes, may be different from width
    ptrdiff_t uvlinesize;      ///< line size, for chroma in bytes, may be different from width
    struct AVRefStructPool *picture_pool; ///< Pool for MPVPictures
    MPVPicture **input_picture;///< next pictures on display order for encoding
    MPVPicture **reordered_input_picture; ///< pointer to the next pictures in coded order for encoding

    BufferPoolContext buffer_pools;

    int64_t user_specified_pts; ///< last non-zero pts from AVFrame which was passed into avcodec_send_frame()
    /**
     * pts difference between the first and second input frame, used for
     * calculating dts of the first frame when there's a delay */
    int64_t dts_delta;
    /**
     * reordered pts to be used as dts for the next output frame when there's
     * a delay */
    int64_t reordered_pts;

    /** bit output */
    PutBitContext pb;

    int start_mb_y;            ///< start mb_y of this thread (so current thread should process start_mb_y <= row < end_mb_y)
    int end_mb_y;              ///< end   mb_y of this thread (so current thread should process start_mb_y <= row < end_mb_y)
    struct MpegEncContext *thread_context[MAX_THREADS];
    int slice_context_count;   ///< number of used thread_contexts

    /**
     * copy of the previous picture structure.
     * note, linesize & data, might not match the previous picture (for field pictures)
     */
    MPVWorkPicture last_pic;

    /**
     * copy of the next picture structure.
     * note, linesize & data, might not match the next picture (for field pictures)
     */
    MPVWorkPicture next_pic;

    /**
     * Reference to the source picture for encoding.
     * note, linesize & data, might not match the source picture (for field pictures)
     */
    AVFrame *new_pic;

    /**
     * copy of the current picture structure.
     * note, linesize & data, might not match the current picture (for field pictures)
     */
    MPVWorkPicture cur_pic;

    int skipped_last_frame;
    int last_dc[3];                ///< last DC values for MPEG-1
    int16_t *dc_val_base;
    int16_t *dc_val[3];            ///< used for MPEG-4 DC prediction, all 3 arrays must be continuous
    const uint8_t *y_dc_scale_table;     ///< qscale -> y_dc_scale table
    const uint8_t *c_dc_scale_table;     ///< qscale -> c_dc_scale table
    const uint8_t *chroma_qscale_table;  ///< qscale -> chroma_qscale (H.263)
    uint8_t *coded_block_base;
    uint8_t *coded_block;          ///< used for coded block pattern prediction (msmpeg4v3, wmv1)
    int16_t (*ac_val_base)[16];
    int16_t (*ac_val[3])[16];      ///< used for MPEG-4 AC prediction, all 3 arrays must be continuous
    int mb_skipped;                ///< MUST BE SET only during DECODING
    uint8_t *mbskip_table;        /**< used to avoid copy if macroblock skipped (for black regions for example)
                                   and used for B-frame encoding & decoding (contains skip table of next P-frame) */
    uint8_t *mbintra_table;       ///< used to avoid setting {ac, dc, cbp}-pred stuff to zero on inter MB decoding
    uint8_t *cbp_table;           ///< used to store cbp, ac_pred for partitioned decoding
    uint8_t *pred_dir_table;      ///< used to store pred_dir for partitioned decoding

    ScratchpadContext sc;

    int qscale;                 ///< QP
    int chroma_qscale;          ///< chroma QP
    unsigned int lambda;        ///< Lagrange multiplier used in rate distortion
    unsigned int lambda2;       ///< (lambda*lambda) >> FF_LAMBDA_SHIFT
    int *lambda_table;
    int adaptive_quant;         ///< use adaptive quantization
    int dquant;                 ///< qscale difference to prev qscale
    int pict_type;              ///< AV_PICTURE_TYPE_I, AV_PICTURE_TYPE_P, AV_PICTURE_TYPE_B, ...
    int last_pict_type; //FIXME removes
    int last_non_b_pict_type;   ///< used for MPEG-4 gmc B-frames & ratecontrol
    int droppable;
    int last_lambda_for[5];     ///< last lambda for a specific pict type
    int skipdct;                ///< skip dct and code zero residual

    /* motion compensation */
    int unrestricted_mv;        ///< mv can point outside of the coded picture
    int h263_long_vectors;      ///< use horrible H.263v1 long vector mode

    BlockDSPContext bdsp;
    FDCTDSPContext fdsp;
    H264ChromaContext h264chroma;
    HpelDSPContext hdsp;
    IDCTDSPContext idsp;
    MpegvideoEncDSPContext mpvencdsp;
    PixblockDSPContext pdsp;
    QpelDSPContext qdsp;
    VideoDSPContext vdsp;
    H263DSPContext h263dsp;
    int f_code;                 ///< forward MV resolution
    int b_code;                 ///< backward MV resolution for B-frames (MPEG-4)
    int16_t (*p_mv_table_base)[2];
    int16_t (*b_forw_mv_table_base)[2];
    int16_t (*b_back_mv_table_base)[2];
    int16_t (*b_bidir_forw_mv_table_base)[2];
    int16_t (*b_bidir_back_mv_table_base)[2];
    int16_t (*b_direct_mv_table_base)[2];
    int16_t (*p_field_mv_table_base)[2];
    int16_t (*b_field_mv_table_base)[2];
    int16_t (*p_mv_table)[2];            ///< MV table (1MV per MB) P-frame encoding
    int16_t (*b_forw_mv_table)[2];       ///< MV table (1MV per MB) forward mode B-frame encoding
    int16_t (*b_back_mv_table)[2];       ///< MV table (1MV per MB) backward mode B-frame encoding
    int16_t (*b_bidir_forw_mv_table)[2]; ///< MV table (1MV per MB) bidir mode B-frame encoding
    int16_t (*b_bidir_back_mv_table)[2]; ///< MV table (1MV per MB) bidir mode B-frame encoding
    int16_t (*b_direct_mv_table)[2];     ///< MV table (1MV per MB) direct mode B-frame encoding
    int16_t (*p_field_mv_table[2][2])[2];   ///< MV table (2MV per MB) interlaced P-frame encoding
    int16_t (*b_field_mv_table[2][2][2])[2];///< MV table (4MV per MB) interlaced B-frame encoding
    uint8_t (*p_field_select_table[2]);  ///< Only the first element is allocated
    uint8_t (*b_field_select_table[2][2]); ///< Only the first element is allocated

    /* The following fields are encoder-only */
    uint16_t *mb_var;           ///< Table for MB variances
    uint16_t *mc_mb_var;        ///< Table for motion compensated MB variances
    uint8_t *mb_mean;           ///< Table for MB luminance
    int64_t mb_var_sum;         ///< sum of MB variance for current frame
    int64_t mc_mb_var_sum;      ///< motion compensated MB variance for current frame
    uint64_t encoding_error[MPV_MAX_PLANES];

    int motion_est;                      ///< ME algorithm
    int me_penalty_compensation;
    int me_pre;                          ///< prepass for motion estimation
    int mv_dir;
#define MV_DIR_FORWARD   1
#define MV_DIR_BACKWARD  2
#define MV_DIRECT        4 ///< bidirectional mode where the difference equals the MV of the last P/S/I-Frame (MPEG-4)
    int mv_type;
#define MV_TYPE_16X16       0   ///< 1 vector for the whole mb
#define MV_TYPE_8X8         1   ///< 4 vectors (H.263, MPEG-4 4MV)
#define MV_TYPE_16X8        2   ///< 2 vectors, one per 16x8 block
#define MV_TYPE_FIELD       3   ///< 2 vectors, one per field
#define MV_TYPE_DMV         4   ///< 2 vectors, special mpeg2 Dual Prime Vectors
    /**motion vectors for a macroblock
       first coordinate : 0 = forward 1 = backward
       second "         : depend on type
       third  "         : 0 = x, 1 = y
    */
    int mv[2][4][2];
    int field_select[2][2];
    int last_mv[2][2][2];             ///< last MV, used for MV prediction in MPEG-1 & B-frame MPEG-4
    const uint8_t *fcode_tab;         ///< smallest fcode needed for each MV
    int16_t direct_scale_mv[2][64];   ///< precomputed to avoid divisions in ff_mpeg4_set_direct_mv

    MotionEstContext me;

    int no_rounding;  /**< apply no rounding to motion compensation (MPEG-4, msmpeg4, ...)
                        for B-frames rounding mode is always 0 */

    /* macroblock layer */
    int mb_x, mb_y;
    int mb_skip_run;
    int mb_intra;
    uint16_t *mb_type;  ///< Table for candidate MB types for encoding (defines in mpegvideoenc.h)

    int block_index[6]; ///< index to current MB in block based arrays with edges
    int block_wrap[6];
    uint8_t *dest[3];

    int *mb_index2xy;        ///< mb_index -> mb_x + mb_y*mb_stride

    /** matrix transmitted in the bitstream */
    uint16_t intra_matrix[64];
    uint16_t chroma_intra_matrix[64];
    uint16_t inter_matrix[64];
    uint16_t chroma_inter_matrix[64];

    int intra_quant_bias;    ///< bias for the quantizer
    int inter_quant_bias;    ///< bias for the quantizer
    int min_qcoeff;          ///< minimum encodable coefficient
    int max_qcoeff;          ///< maximum encodable coefficient
    int ac_esc_length;       ///< num of bits needed to encode the longest esc
    uint8_t *intra_ac_vlc_length;
    uint8_t *intra_ac_vlc_last_length;
    uint8_t *intra_chroma_ac_vlc_length;
    uint8_t *intra_chroma_ac_vlc_last_length;
    uint8_t *inter_ac_vlc_length;
    uint8_t *inter_ac_vlc_last_length;
    uint8_t *luma_dc_vlc_length;

    int coded_score[12];

    /** precomputed matrix (combine qscale and DCT renorm) */
    int (*q_intra_matrix)[64];
    int (*q_chroma_intra_matrix)[64];
    int (*q_inter_matrix)[64];
    /** identical to the above but for MMX & these are not permutated, second 64 entries are bias*/
    uint16_t (*q_intra_matrix16)[2][64];
    uint16_t (*q_chroma_intra_matrix16)[2][64];
    uint16_t (*q_inter_matrix16)[2][64];

    /* noise reduction */
    int (*dct_error_sum)[64];
    int dct_count[2];
    uint16_t (*dct_offset)[64];

    /* bit rate control */
    int64_t total_bits;
    int frame_bits;                ///< bits used for the current frame
    int stuffing_bits;             ///< bits used for stuffing
    int next_lambda;               ///< next lambda used for retrying to encode a frame
    RateControlContext rc_context; ///< contains stuff only accessed in ratecontrol.c

    /* statistics, used for 2-pass encoding */
    int mv_bits;
    int header_bits;
    int i_tex_bits;
    int p_tex_bits;
    int i_count;
    int misc_bits; ///< cbp, mb_type
    int last_bits; ///< temp var used for calculating the above vars

    /* error concealment / resync */
    int resync_mb_x;                 ///< x position of last resync marker
    int resync_mb_y;                 ///< y position of last resync marker
    GetBitContext last_resync_gb;    ///< used to search for the next resync marker
    int mb_num_left;                 ///< number of MBs left in this video packet (for partitioned Slices only)

    /* H.263 specific */
    int gob_index;
    int obmc;                       ///< overlapped block motion compensation
    int mb_info;                    ///< interval for outputting info about mb offsets as side data
    int prev_mb_info, last_mb_info;
    uint8_t *mb_info_ptr;
    int mb_info_size;
    int ehc_mode;

    /* H.263+ specific */
    int umvplus;                    ///< == H.263+ && unrestricted_mv
    int h263_aic_dir;               ///< AIC direction: 0 = left, 1 = top
    int h263_slice_structured;
    int alt_inter_vlc;              ///< alternative inter vlc
    int modified_quant;
    int loop_filter;
    int custom_pcf;

    /* MPEG-4 specific */
    int studio_profile;
    int dct_precision;
    /// number of bits to represent the fractional part of time (encoder only)
    int time_increment_bits;
    int last_time_base;
    int time_base;                  ///< time in seconds of last I,P,S Frame
    int64_t time;                   ///< time of current frame
    int64_t last_non_b_time;
    uint16_t pp_time;               ///< time distance between the last 2 p,s,i frames
    uint16_t pb_time;               ///< time distance between the last b and p,s,i frame
    uint16_t pp_field_time;
    uint16_t pb_field_time;         ///< like above, just for interlaced
    int mcsel;
    int quarter_sample;              ///< 1->qpel, 0->half pel ME/MC
    int data_partitioning;           ///< data partitioning flag from header
    int partitioned_frame;           ///< is current frame partitioned
    int low_delay;                   ///< no reordering needed / has no B-frames
    PutBitContext tex_pb;            ///< used for data partitioned VOPs
    PutBitContext pb2;               ///< used for data partitioned VOPs
    int mpeg_quant;
    int padding_bug_score;             ///< used to detect the VERY common padding bug in MPEG-4

    /* divx specific, used to workaround (many) bugs in divx5 */
    int divx_packed;

    /* RV10 specific */
    int rv10_version; ///< RV10 version: 0 or 3
    int rv10_first_dc_coded[3];

    /* MJPEG specific */
    struct MJpegContext *mjpeg_ctx;
    int esc_pos;

    /* MSMPEG4 specific */
    int mv_table_index;
    int rl_table_index;
    int rl_chroma_table_index;
    int dc_table_index;
    int use_skip_mb_code;
    int slice_height;      ///< in macroblocks
    int first_slice_line;  ///< used in MPEG-4 too to handle resync markers
    int flipflop_rounding;
    enum {
        MSMP4_UNUSED,
        MSMP4_V1,
        MSMP4_V2,
        MSMP4_V3,
        MSMP4_WMV1,
        MSMP4_WMV2,
        MSMP4_VC1,        ///< for VC1 (image), WMV3 (image) and MSS2.
    } msmpeg4_version;
    int per_mb_rl_table;
    int esc3_level_length;
    int esc3_run_length;
    int inter_intra_pred;
    int mspel;

    /* decompression specific */
    GetBitContext gb;

    /* MPEG-1 specific */
    int last_mv_dir;         ///< last mv_dir, used for B-frame encoding
    int vbv_delay_pos;       ///< offset of vbv_delay in the bitstream

    /* MPEG-2-specific - I wished not to have to support this mess. */
    int progressive_sequence;
    int mpeg_f_code[2][2];

    // picture structure defines are loaded from mpegutils.h
    int picture_structure;

    int intra_dc_precision;
    int frame_pred_frame_dct;
    int top_field_first;
    int concealment_motion_vectors;
    int q_scale_type;
    int brd_scale;
    int intra_vlc_format;
    int alternate_scan;
    int repeat_first_field;
    int chroma_420_type;
    int chroma_format;
#define CHROMA_420 1
#define CHROMA_422 2
#define CHROMA_444 3
    int chroma_x_shift;//depend on pix_format, that depend on chroma_format
    int chroma_y_shift;

    int progressive_frame;
    int full_pel[2];
    int interlaced_dct;
    int first_field;         ///< is 1 for the first field of a field picture 0 otherwise

    /* RTP specific */
    int rtp_mode;
    int rtp_payload_size;

    uint8_t *ptr_lastgob;

    int16_t (*block)[64]; ///< points to one of the following blocks
    int16_t (*blocks)[12][64]; // for HQ mode we need to keep the best block
    int (*decode_mb)(struct MpegEncContext *s, int16_t block[12][64]); // used by some codecs to avoid a switch()

#define SLICE_OK         0
#define SLICE_ERROR     -1
#define SLICE_END       -2 ///<end marker found
#define SLICE_NOEND     -3 ///<no end marker or error found but mb count exceeded

    void (*dct_unquantize_mpeg1_intra)(struct MpegEncContext *s,
                           int16_t *block/*align 16*/, int n, int qscale);
    void (*dct_unquantize_mpeg1_inter)(struct MpegEncContext *s,
                           int16_t *block/*align 16*/, int n, int qscale);
    void (*dct_unquantize_mpeg2_intra)(struct MpegEncContext *s,
                           int16_t *block/*align 16*/, int n, int qscale);
    void (*dct_unquantize_mpeg2_inter)(struct MpegEncContext *s,
                           int16_t *block/*align 16*/, int n, int qscale);
    void (*dct_unquantize_h263_intra)(struct MpegEncContext *s,
                           int16_t *block/*align 16*/, int n, int qscale);
    void (*dct_unquantize_h263_inter)(struct MpegEncContext *s,
                           int16_t *block/*align 16*/, int n, int qscale);
    void (*dct_unquantize_intra)(struct MpegEncContext *s, // unquantizer to use (MPEG-4 can use both)
                           int16_t *block/*align 16*/, int n, int qscale);
    void (*dct_unquantize_inter)(struct MpegEncContext *s, // unquantizer to use (MPEG-4 can use both)
                           int16_t *block/*align 16*/, int n, int qscale);
    int (*dct_quantize)(struct MpegEncContext *s, int16_t *block/*align 16*/, int n, int qscale, int *overflow);
    void (*denoise_dct)(struct MpegEncContext *s, int16_t *block);

    int mpv_flags;      ///< flags set by private options
    int quantizer_noise_shaping;

    me_cmp_func ildct_cmp[2]; ///< 0 = intra, 1 = non-intra
    me_cmp_func n_sse_cmp[2]; ///< either SSE or NSSE cmp func
    me_cmp_func sad_cmp[2];
    me_cmp_func sse_cmp[2];
    int (*sum_abs_dctelem)(const int16_t *block);

    float border_masking;
    int lmin, lmax;
    int vbv_ignore_qmax;

    /* flag to indicate a reinitialization is required, e.g. after
     * a frame size change */
    int context_reinit;

    ERContext er;

    int error_rate;

    /* temporary frames used by b_frame_strategy = 2 */
    AVFrame *tmp_frames[MAX_B_FRAMES + 2];
    int b_frame_strategy;
    int b_sensitivity;

    /* frame skip options for encoding */
    int frame_skip_threshold;
    int frame_skip_factor;
    int frame_skip_exp;
    int frame_skip_cmp;
    me_cmp_func frame_skip_cmp_fn;

    int scenechange_threshold;
    int noise_reduction;

    int intra_penalty;
} MpegEncContext;


/**
 * Set the given MpegEncContext to common defaults (same for encoding
 * and decoding).  The changed fields will not depend upon the prior
 * state of the MpegEncContext.
 */
void ff_mpv_common_defaults(MpegEncContext *s);

int ff_mpv_common_init(MpegEncContext *s);
void ff_mpv_common_init_arm(MpegEncContext *s);
void ff_mpv_common_init_neon(MpegEncContext *s);
void ff_mpv_common_init_ppc(MpegEncContext *s);
void ff_mpv_common_init_x86(MpegEncContext *s);
void ff_mpv_common_init_mips(MpegEncContext *s);
/**
 * Initialize an MpegEncContext's thread contexts. Presumes that
 * slice_context_count is already set and that all the fields
 * that are freed/reset in free_duplicate_context() are NULL.
 */
int ff_mpv_init_duplicate_contexts(MpegEncContext *s);
/**
 * Initialize and allocates MpegEncContext fields dependent on the resolution.
 */
int ff_mpv_init_context_frame(MpegEncContext *s);
/**
 * Frees and resets MpegEncContext fields depending on the resolution
 * as well as the slice thread contexts.
 * Is used during resolution changes to avoid a full reinitialization of the
 * codec.
 */
void ff_mpv_free_context_frame(MpegEncContext *s);

void ff_mpv_common_end(MpegEncContext *s);

void ff_clean_intra_table_entries(MpegEncContext *s);

int ff_update_duplicate_context(MpegEncContext *dst, const MpegEncContext *src);
void ff_set_qscale(MpegEncContext * s, int qscale);

void ff_mpv_idct_init(MpegEncContext *s);
void ff_init_scantable(const uint8_t *permutation, ScanTable *st,
                       const uint8_t *src_scantable);
void ff_init_block_index(MpegEncContext *s);

void ff_mpv_motion(MpegEncContext *s,
                   uint8_t *dest_y, uint8_t *dest_cb,
                   uint8_t *dest_cr, int dir,
                   uint8_t *const *ref_picture,
                   const op_pixels_func (*pix_op)[4],
                   const qpel_mc_func (*qpix_op)[16]);

static inline void ff_update_block_index(MpegEncContext *s, int bits_per_raw_sample,
                                         int lowres, int chroma_x_shift)
{
    const int bytes_per_pixel = 1 + (bits_per_raw_sample > 8);
    const int block_size = (8 * bytes_per_pixel) >> lowres;

    s->block_index[0]+=2;
    s->block_index[1]+=2;
    s->block_index[2]+=2;
    s->block_index[3]+=2;
    s->block_index[4]++;
    s->block_index[5]++;
    s->dest[0]+= 2*block_size;
    s->dest[1] += (2 >> chroma_x_shift) * block_size;
    s->dest[2] += (2 >> chroma_x_shift) * block_size;
}

#endif /* AVCODEC_MPEGVIDEO_H */
