/*
 * H.26L/H.264/AVC/JVT/14496-10/... encoder/decoder
 * Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
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
 * H.264 / AVC / MPEG-4 part10 codec.
 * @author Michael Niedermayer <michaelni@gmx.at>
 */

#ifndef AVCODEC_H264DEC_H
#define AVCODEC_H264DEC_H

#include "libavutil/buffer.h"
#include "libavutil/mem_internal.h"

#include "cabac.h"
#include "error_resilience.h"
#include "h264_parse.h"
#include "h264_ps.h"
#include "h264_sei.h"
#include "h2645_parse.h"
#include "h264chroma.h"
#include "h264dsp.h"
#include "h264pred.h"
#include "h264qpel.h"
#include "h274.h"
#include "mpegutils.h"
#include "threadframe.h"
#include "videodsp.h"

#define H264_MAX_PICTURE_COUNT 36

/* Compiling in interlaced support reduces the speed
 * of progressive decoding by about 2%. */
#define ALLOW_INTERLACE

#define FMO 0

/**
 * The maximum number of slices supported by the decoder.
 * must be a power of 2
 */
#define MAX_SLICES 32

#ifdef ALLOW_INTERLACE
#define MB_MBAFF(h)    (h)->mb_mbaff
#define MB_FIELD(sl)  (sl)->mb_field_decoding_flag
#define FRAME_MBAFF(h) (h)->mb_aff_frame
#define FIELD_PICTURE(h) ((h)->picture_structure != PICT_FRAME)
#define LEFT_MBS 2
#define LTOP     0
#define LBOT     1
#define LEFT(i)  (i)
#else
#define MB_MBAFF(h)      0
#define MB_FIELD(sl)     0
#define FRAME_MBAFF(h)   0
#define FIELD_PICTURE(h) 0
#undef  IS_INTERLACED
#define IS_INTERLACED(mb_type) 0
#define LEFT_MBS 1
#define LTOP     0
#define LBOT     0
#define LEFT(i)  0
#endif
#define FIELD_OR_MBAFF_PICTURE(h) (FRAME_MBAFF(h) || FIELD_PICTURE(h))

#ifndef CABAC
#define CABAC(h) (h)->ps.pps->cabac
#endif

#define CHROMA(h)    ((h)->ps.sps->chroma_format_idc)
#define CHROMA422(h) ((h)->ps.sps->chroma_format_idc == 2)
#define CHROMA444(h) ((h)->ps.sps->chroma_format_idc == 3)

#define IS_REF0(a)         ((a) & MB_TYPE_REF0)
#define IS_8x8DCT(a)       ((a) & MB_TYPE_8x8DCT)
#define IS_SUB_8X8(a)      ((a) & MB_TYPE_16x16) // note reused
#define IS_SUB_8X4(a)      ((a) & MB_TYPE_16x8)  // note reused
#define IS_SUB_4X8(a)      ((a) & MB_TYPE_8x16)  // note reused
#define IS_SUB_4X4(a)      ((a) & MB_TYPE_8x8)   // note reused
#define IS_DIR(a, part, list) ((a) & (MB_TYPE_P0L0 << ((part) + 2 * (list))))

// does this mb use listX, note does not work if subMBs
#define USES_LIST(a, list) ((a) & ((MB_TYPE_P0L0 | MB_TYPE_P1L0) << (2 * (list))))

/**
 * Memory management control operation.
 */
typedef struct MMCO {
    MMCOOpcode opcode;
    int short_pic_num;  ///< pic_num without wrapping (pic_num & max_pic_num)
    int long_arg;       ///< index, pic_num, or num long refs depending on opcode
} MMCO;

typedef struct H264Picture {
    AVFrame *f;
    ThreadFrame tf;

    AVFrame *f_grain;

    int8_t *qscale_table_base;        ///< RefStruct reference
    int8_t *qscale_table;

    int16_t (*motion_val_base[2])[2]; ///< RefStruct reference
    int16_t (*motion_val[2])[2];

    uint32_t *mb_type_base;           ///< RefStruct reference
    uint32_t *mb_type;

    /// RefStruct reference for hardware accelerator private data
    void *hwaccel_picture_private;

    int8_t *ref_index[2];   ///< RefStruct reference

    int field_poc[2];       ///< top/bottom POC
    int poc;                ///< frame POC
    int frame_num;          ///< frame_num (raw frame_num from slice header)
    int mmco_reset;         /**< MMCO_RESET set this 1. Reordering code must
                                 not mix pictures before and after MMCO_RESET. */
    int pic_id;             /**< pic_num (short -> no wrap version of pic_num,
                                 pic_num & max_pic_num; long -> long_pic_num) */
    int long_ref;           ///< 1->long term reference 0->short term reference
    int ref_poc[2][2][32];  ///< POCs of the frames/fields used as reference (FIXME need per slice)
    int ref_count[2][2];    ///< number of entries in ref_poc         (FIXME need per slice)
    int mbaff;              ///< 1 -> MBAFF frame 0-> not MBAFF
    int field_picture;      ///< whether or not picture was encoded in separate fields

/**
 * H264Picture.reference has this flag set,
 * when the picture is held for delayed output.
 */
#define DELAYED_PIC_REF  (1 << 2)
    int reference;
    int recovered;          ///< picture at IDR or recovery point + recovery count
    int invalid_gap;
    int sei_recovery_frame_cnt;
    int needs_fg;           ///< whether picture needs film grain synthesis (see `f_grain`)

    const PPS   *pps;

    int mb_width, mb_height;
    int mb_stride;

    /// RefStruct reference; its pointee is shared between decoding threads.
    atomic_int *decode_error_flags;

    int gray;
} H264Picture;

typedef struct H264Ref {
    uint8_t *data[3];
    int linesize[3];

    int reference;
    int poc;
    int pic_id;

    const H264Picture *parent;
} H264Ref;

typedef struct H264SliceContext {
    const struct H264Context *h264;
    GetBitContext gb;
    ERContext *er;

    int slice_num;
    int slice_type;
    int slice_type_nos;         ///< S free slice type (SI/SP are remapped to I/P)
    int slice_type_fixed;

    int qscale;
    int chroma_qp[2];   // QPc
    int qp_thresh;      ///< QP threshold to skip loopfilter
    int last_qscale_diff;

    // deblock
    int deblocking_filter;          ///< disable_deblocking_filter_idc with 1 <-> 0
    int slice_alpha_c0_offset;
    int slice_beta_offset;

    H264PredWeightTable pwt;

    int prev_mb_skipped;
    int next_mb_skipped;

    int chroma_pred_mode;
    int intra16x16_pred_mode;

    int8_t intra4x4_pred_mode_cache[5 * 8];
    int8_t(*intra4x4_pred_mode);

    int topleft_mb_xy;
    int top_mb_xy;
    int topright_mb_xy;
    int left_mb_xy[LEFT_MBS];

    int topleft_type;
    int top_type;
    int topright_type;
    int left_type[LEFT_MBS];

    const uint8_t *left_block;
    int topleft_partition;

    unsigned int topleft_samples_available;
    unsigned int top_samples_available;
    unsigned int topright_samples_available;
    unsigned int left_samples_available;

    ptrdiff_t linesize, uvlinesize;
    ptrdiff_t mb_linesize;  ///< may be equal to s->linesize or s->linesize * 2, for mbaff
    ptrdiff_t mb_uvlinesize;

    int mb_x, mb_y;
    int mb_xy;
    int resync_mb_x;
    int resync_mb_y;
    unsigned int first_mb_addr;
    // index of the first MB of the next slice
    int next_slice_idx;
    int mb_skip_run;
    int is_complex;

    int picture_structure;
    int mb_field_decoding_flag;
    int mb_mbaff;               ///< mb_aff_frame && mb_field_decoding_flag

    int redundant_pic_count;

    /**
     * number of neighbors (top and/or left) that used 8x8 dct
     */
    int neighbor_transform_size;

    int direct_spatial_mv_pred;
    int col_parity;
    int col_fieldoff;

    int cbp;
    int top_cbp;
    int left_cbp;

    int dist_scale_factor[32];
    int dist_scale_factor_field[2][32];
    int map_col_to_list0[2][16 + 32];
    int map_col_to_list0_field[2][2][16 + 32];

    /**
     * num_ref_idx_l0/1_active_minus1 + 1
     */
    unsigned int ref_count[2];          ///< counts frames or fields, depending on current mb mode
    unsigned int list_count;
    H264Ref ref_list[2][48];        /**< 0..15: frame refs, 16..47: mbaff field refs.
                                         *   Reordered version of default_ref_list
                                         *   according to picture reordering in slice header */
    struct {
        uint8_t op;
        uint32_t val;
    } ref_modifications[2][32];
    int nb_ref_modifications[2];

    unsigned int pps_id;

    const uint8_t *intra_pcm_ptr;

    uint8_t *bipred_scratchpad;
    uint8_t *edge_emu_buffer;
    uint8_t (*top_borders[2])[(16 * 3) * 2];
    int bipred_scratchpad_allocated;
    int edge_emu_buffer_allocated;
    int top_borders_allocated[2];

    /**
     * non zero coeff count cache.
     * is 64 if not available.
     */
    DECLARE_ALIGNED(8, uint8_t, non_zero_count_cache)[15 * 8];

    /**
     * Motion vector cache.
     */
    DECLARE_ALIGNED(16, int16_t, mv_cache)[2][5 * 8][2];
    DECLARE_ALIGNED(8,  int8_t, ref_cache)[2][5 * 8];
    DECLARE_ALIGNED(16, uint8_t, mvd_cache)[2][5 * 8][2];
    uint8_t direct_cache[5 * 8];

    DECLARE_ALIGNED(8, uint16_t, sub_mb_type)[4];

    /// as a DCT coefficient is int32_t in high depth, we need to reserve twice the space.
    DECLARE_ALIGNED(16, int16_t, mb)[16 * 48 * 2];
    DECLARE_ALIGNED(16, int16_t, mb_luma_dc)[3][16 * 2];
    /// as mb is addressed by scantable[i] and scantable is uint8_t we can either
    /// check that i is not too large or ensure that there is some unused stuff after mb
    int16_t mb_padding[256 * 2];

    uint8_t (*mvd_table[2])[2];

    /**
     * Cabac
     */
    CABACContext cabac;
    uint8_t cabac_state[1024];
    int cabac_init_idc;

    MMCO mmco[H264_MAX_MMCO_COUNT];
    int  nb_mmco;
    int explicit_ref_marking;

    int frame_num;
    int idr_pic_id;
    int poc_lsb;
    int delta_poc_bottom;
    int delta_poc[2];
    int curr_pic_num;
    int max_pic_num;
} H264SliceContext;

/**
 * H264Context
 */
typedef struct H264Context {
    const AVClass *class;
    AVCodecContext *avctx;
    VideoDSPContext vdsp;
    H264DSPContext h264dsp;
    H264ChromaContext h264chroma;
    H264QpelContext h264qpel;
    H274FilmGrainDatabase h274db;

    H264Picture DPB[H264_MAX_PICTURE_COUNT];
    H264Picture *cur_pic_ptr;
    H264Picture cur_pic;
    H264Picture last_pic_for_ec;

    H264SliceContext *slice_ctx;
    int            nb_slice_ctx;
    int            nb_slice_ctx_queued;

    H2645Packet pkt;

    int pixel_shift;    ///< 0 for 8-bit H.264, 1 for high-bit-depth H.264

    /* coded dimensions -- 16 * mb w/h */
    int width, height;
    int chroma_x_shift, chroma_y_shift;

    int droppable;

    int context_initialized;
    int flags;
    int workaround_bugs;
    int x264_build;
    /* Set when slice threading is used and at least one slice uses deblocking
     * mode 1 (i.e. across slice boundaries). Then we disable the loop filter
     * during normal MB decoding and execute it serially at the end.
     */
    int postpone_filter;

    /*
     * Set to 1 when the current picture is IDR, 0 otherwise.
     */
    int picture_idr;

    /*
     * Set to 1 when the current picture contains only I slices, 0 otherwise.
     */
    int picture_intra_only;

    int crop_left;
    int crop_right;
    int crop_top;
    int crop_bottom;

    int8_t(*intra4x4_pred_mode);
    H264PredContext hpc;

    uint8_t (*non_zero_count)[48];

#define LIST_NOT_USED -1 // FIXME rename?

    /**
     * block_offset[ 0..23] for frame macroblocks
     * block_offset[24..47] for field macroblocks
     */
    int block_offset[2 * (16 * 3)];

    uint32_t *mb2b_xy;  // FIXME are these 4 a good idea?
    uint32_t *mb2br_xy;
    int b_stride;       // FIXME use s->b4_stride

    uint16_t *slice_table;      ///< slice_table_base + 2*mb_stride + 1

    // interlacing specific flags
    int mb_aff_frame;
    int picture_structure;
    int first_field;

    uint8_t *list_counts;               ///< Array of list_count per MB specifying the slice type

    /* 0x100 -> non null luma_dc, 0x80/0x40 -> non null chroma_dc (cb/cr), 0x?0 -> chroma_cbp(0, 1, 2), 0x0? luma_cbp */
    uint16_t *cbp_table;

    /* chroma_pred_mode for i4x4 or i16x16, else 0 */
    uint8_t *chroma_pred_mode_table;
    uint8_t (*mvd_table[2])[2];
    uint8_t *direct_table;

    uint8_t scan_padding[16];
    uint8_t zigzag_scan[16];
    uint8_t zigzag_scan8x8[64];
    uint8_t zigzag_scan8x8_cavlc[64];
    uint8_t field_scan[16];
    uint8_t field_scan8x8[64];
    uint8_t field_scan8x8_cavlc[64];
    uint8_t zigzag_scan_q0[16];
    uint8_t zigzag_scan8x8_q0[64];
    uint8_t zigzag_scan8x8_cavlc_q0[64];
    uint8_t field_scan_q0[16];
    uint8_t field_scan8x8_q0[64];
    uint8_t field_scan8x8_cavlc_q0[64];

    int mb_y;
    int mb_height, mb_width;
    int mb_stride;
    int mb_num;

    // =============================================================
    // Things below are not used in the MB or more inner code

    int nal_ref_idc;
    int nal_unit_type;

    int has_slice;          ///< slice NAL is found in the packet, set by decode_nal_units, its state does not need to be preserved outside h264_decode_frame()

    /**
     * Used to parse AVC variant of H.264
     */
    int is_avc;           ///< this flag is != 0 if codec is avc1
    int nal_length_size;  ///< Number of bytes used for nal length (1, 2 or 4)

    int bit_depth_luma;         ///< luma bit depth from sps to detect changes
    int chroma_format_idc;      ///< chroma format from sps to detect changes

    H264ParamSets ps;

    uint16_t *slice_table_base;

    H264POCContext poc;

    H264Ref default_ref[2];
    H264Picture *short_ref[32];
    H264Picture *long_ref[32];
    H264Picture *delayed_pic[H264_MAX_DPB_FRAMES + 2]; // FIXME size?
    int last_pocs[H264_MAX_DPB_FRAMES];
    H264Picture *next_output_pic;
    int next_outputed_poc;
    int poc_offset;         ///< PicOrderCnt_offset from SMPTE RDD-2006

    /**
     * memory management control operations buffer.
     */
    MMCO mmco[H264_MAX_MMCO_COUNT];
    int  nb_mmco;
    int mmco_reset;
    int explicit_ref_marking;

    int long_ref_count;     ///< number of actual long term references
    int short_ref_count;    ///< number of actual short term references

    /**
     * @name Members for slice based multithreading
     * @{
     */
    /**
     * current slice number, used to initialize slice_num of each thread/context
     */
    int current_slice;

    /** @} */

    /**
     * Complement sei_pic_struct
     * SEI_PIC_STRUCT_TOP_BOTTOM and SEI_PIC_STRUCT_BOTTOM_TOP indicate interlaced frames.
     * However, soft telecined frames may have these values.
     * This is used in an attempt to flag soft telecine progressive.
     */
    int prev_interlaced_frame;

    /**
     * Are the SEI recovery points looking valid.
     */
    int valid_recovery_point;

    /**
     * recovery_frame is the frame_num at which the next frame should
     * be fully constructed.
     *
     * Set to -1 when not expecting a recovery point.
     */
    int recovery_frame;

/**
 * We have seen an IDR, so all the following frames in coded order are correctly
 * decodable.
 */
#define FRAME_RECOVERED_IDR  (1 << 0)
/**
 * Sufficient number of frames have been decoded since a SEI recovery point,
 * so all the following frames in presentation order are correct.
 */
#define FRAME_RECOVERED_SEI  (1 << 1)
/**
 * Recovery point detected by heuristic
 */
#define FRAME_RECOVERED_HEURISTIC  (1 << 2)

    /**
     * Initial frame has been completely recovered.
     *
     * Once this is set, all following decoded as well as displayed frames will be marked as recovered
     * If a frame is marked as recovered frame_recovered will be set once this frame is output and thus
     * all subsequently output fraames are also marked as recovered
     *
     * In effect, if you want all subsequent DECODED frames marked as recovered, set frame_recovered
     * If you want all subsequent DISPAYED frames marked as recovered, set the frame->recovered
     */
    int frame_recovered;

    int has_recovery_point;

    int missing_fields;

    /* for frame threading, this is set to 1
     * after finish_setup() has been called, so we cannot modify
     * some context properties (which are supposed to stay constant between
     * slices) anymore */
    int setup_finished;

    int cur_chroma_format_idc;
    int cur_bit_depth_luma;
    int16_t slice_row[MAX_SLICES]; ///< to detect when MAX_SLICES is too low

    /* original AVCodecContext dimensions, used to handle container
     * cropping */
    int width_from_caller;
    int height_from_caller;

    int enable_er;
    ERContext er;
    int16_t *dc_val_base;

    H264SEIContext sei;

    struct AVRefStructPool *qscale_table_pool;
    struct AVRefStructPool *mb_type_pool;
    struct AVRefStructPool *motion_val_pool;
    struct AVRefStructPool *ref_index_pool;
    struct AVRefStructPool *decode_error_flags_pool;
    int ref2frm[MAX_SLICES][2][64];     ///< reference to frame number lists, used in the loop filter, the first 2 are for -2,-1

    int non_gray;                       ///< Did we encounter a intra frame after a gray gap frame
    int noref_gray;
    int skip_gray;
} H264Context;

extern const uint16_t ff_h264_mb_sizes[4];

/**
 * Reconstruct bitstream slice_type.
 */
int ff_h264_get_slice_type(const H264SliceContext *sl);

/**
 * Allocate tables.
 * needs width/height
 */
int ff_h264_alloc_tables(H264Context *h);

int ff_h264_decode_ref_pic_list_reordering(H264SliceContext *sl, void *logctx);
int ff_h264_build_ref_list(H264Context *h, H264SliceContext *sl);
void ff_h264_remove_all_refs(H264Context *h);

/**
 * Execute the reference picture marking (memory management control operations).
 */
int ff_h264_execute_ref_pic_marking(H264Context *h);

int ff_h264_decode_ref_pic_marking(H264SliceContext *sl, GetBitContext *gb,
                                   const H2645NAL *nal, void *logctx);

void ff_h264_hl_decode_mb(const H264Context *h, H264SliceContext *sl);
void ff_h264_decode_init_vlc(void);

/**
 * Decode a macroblock
 * @return 0 if OK, ER_AC_ERROR / ER_DC_ERROR / ER_MV_ERROR on error
 */
int ff_h264_decode_mb_cavlc(const H264Context *h, H264SliceContext *sl);

/**
 * Decode a CABAC coded macroblock
 * @return 0 if OK, ER_AC_ERROR / ER_DC_ERROR / ER_MV_ERROR on error
 */
int ff_h264_decode_mb_cabac(const H264Context *h, H264SliceContext *sl);

void ff_h264_init_cabac_states(const H264Context *h, H264SliceContext *sl);

void ff_h264_direct_dist_scale_factor(const H264Context *const h, H264SliceContext *sl);
void ff_h264_direct_ref_list_init(const H264Context *const h, H264SliceContext *sl);
void ff_h264_pred_direct_motion(const H264Context *const h, H264SliceContext *sl,
                                int *mb_type);

void ff_h264_filter_mb_fast(const H264Context *h, H264SliceContext *sl, int mb_x, int mb_y,
                            uint8_t *img_y, uint8_t *img_cb, uint8_t *img_cr,
                            unsigned int linesize, unsigned int uvlinesize);
void ff_h264_filter_mb(const H264Context *h, H264SliceContext *sl, int mb_x, int mb_y,
                       uint8_t *img_y, uint8_t *img_cb, uint8_t *img_cr,
                       unsigned int linesize, unsigned int uvlinesize);

/*
 * o-o o-o
 *  / / /
 * o-o o-o
 *  ,---'
 * o-o o-o
 *  / / /
 * o-o o-o
 */

/* Scan8 organization:
 *    0 1 2 3 4 5 6 7
 * 0  DY    y y y y y
 * 1        y Y Y Y Y
 * 2        y Y Y Y Y
 * 3        y Y Y Y Y
 * 4        y Y Y Y Y
 * 5  DU    u u u u u
 * 6        u U U U U
 * 7        u U U U U
 * 8        u U U U U
 * 9        u U U U U
 * 10 DV    v v v v v
 * 11       v V V V V
 * 12       v V V V V
 * 13       v V V V V
 * 14       v V V V V
 * DY/DU/DV are for luma/chroma DC.
 */

#define LUMA_DC_BLOCK_INDEX   48
#define CHROMA_DC_BLOCK_INDEX 49

/**
 * Get the chroma qp.
 */
static av_always_inline int get_chroma_qp(const PPS *pps, int t, int qscale)
{
    return pps->chroma_qp_table[t][qscale];
}

int ff_h264_field_end(H264Context *h, H264SliceContext *sl, int in_setup);

int ff_h264_ref_picture(H264Picture *dst, const H264Picture *src);
int ff_h264_replace_picture(H264Picture *dst, const H264Picture *src);
void ff_h264_unref_picture(H264Picture *pic);

void ff_h264_slice_context_init(H264Context *h, H264SliceContext *sl);

void ff_h264_draw_horiz_band(const H264Context *h, H264SliceContext *sl, int y, int height);

/**
 * Submit a slice for decoding.
 *
 * Parse the slice header, starting a new field/frame if necessary. If any
 * slices are queued for the previous field, they are decoded.
 */
int ff_h264_queue_decode_slice(H264Context *h, const H2645NAL *nal);
int ff_h264_execute_decode_slices(H264Context *h);
int ff_h264_update_thread_context(AVCodecContext *dst,
                                  const AVCodecContext *src);
int ff_h264_update_thread_context_for_user(AVCodecContext *dst,
                                           const AVCodecContext *src);

void ff_h264_flush_change(H264Context *h);

void ff_h264_free_tables(H264Context *h);

void ff_h264_set_erpic(ERPicture *dst, const H264Picture *src);

#endif /* AVCODEC_H264DEC_H */
