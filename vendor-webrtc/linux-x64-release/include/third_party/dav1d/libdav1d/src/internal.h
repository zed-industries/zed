/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_INTERNAL_H
#define DAV1D_SRC_INTERNAL_H

#include <stdatomic.h>

#include "dav1d/data.h"

typedef struct Dav1dFrameContext Dav1dFrameContext;
typedef struct Dav1dTileState Dav1dTileState;
typedef struct Dav1dTaskContext Dav1dTaskContext;
typedef struct Dav1dTask Dav1dTask;

#include "common/attributes.h"

#include "src/cdef.h"
#include "src/cdf.h"
#include "src/data.h"
#include "src/env.h"
#include "src/filmgrain.h"
#include "src/intra_edge.h"
#include "src/ipred.h"
#include "src/itx.h"
#include "src/levels.h"
#include "src/lf_mask.h"
#include "src/loopfilter.h"
#include "src/looprestoration.h"
#include "src/mc.h"
#include "src/msac.h"
#include "src/pal.h"
#include "src/picture.h"
#include "src/recon.h"
#include "src/refmvs.h"
#include "src/thread.h"

typedef struct Dav1dDSPContext {
    Dav1dFilmGrainDSPContext fg;
    Dav1dIntraPredDSPContext ipred;
    Dav1dMCDSPContext mc;
    Dav1dInvTxfmDSPContext itx;
    Dav1dLoopFilterDSPContext lf;
    Dav1dCdefDSPContext cdef;
    Dav1dLoopRestorationDSPContext lr;
} Dav1dDSPContext;

struct Dav1dTileGroup {
    Dav1dData data;
    int start, end;
};

enum TaskType {
    DAV1D_TASK_TYPE_INIT,
    DAV1D_TASK_TYPE_INIT_CDF,
    DAV1D_TASK_TYPE_TILE_ENTROPY,
    DAV1D_TASK_TYPE_ENTROPY_PROGRESS,
    DAV1D_TASK_TYPE_TILE_RECONSTRUCTION,
    DAV1D_TASK_TYPE_DEBLOCK_COLS,
    DAV1D_TASK_TYPE_DEBLOCK_ROWS,
    DAV1D_TASK_TYPE_CDEF,
    DAV1D_TASK_TYPE_SUPER_RESOLUTION,
    DAV1D_TASK_TYPE_LOOP_RESTORATION,
    DAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS,
    DAV1D_TASK_TYPE_FG_PREP,
    DAV1D_TASK_TYPE_FG_APPLY,
};

struct Dav1dContext {
    Dav1dFrameContext *fc;
    unsigned n_fc;

    Dav1dTaskContext *tc;
    unsigned n_tc;

    // cache of OBUs that make up a single frame before we submit them
    // to a frame worker to be decoded
    struct Dav1dTileGroup *tile;
    int n_tile_data_alloc;
    int n_tile_data;
    int n_tiles;
    Dav1dMemPool *seq_hdr_pool;
    Dav1dRef *seq_hdr_ref;
    Dav1dSequenceHeader *seq_hdr;
    Dav1dMemPool *frame_hdr_pool;
    Dav1dRef *frame_hdr_ref;
    Dav1dFrameHeader *frame_hdr;

    Dav1dRef *content_light_ref;
    Dav1dContentLightLevel *content_light;
    Dav1dRef *mastering_display_ref;
    Dav1dMasteringDisplay *mastering_display;
    Dav1dRef *itut_t35_ref;
    Dav1dITUTT35 *itut_t35;
    int n_itut_t35;

    // decoded output picture queue
    Dav1dData in;
    Dav1dThreadPicture out, cache;
    // dummy is a pointer to prevent compiler errors about atomic_load()
    // not taking const arguments
    atomic_int flush_mem, *flush;
    struct {
        Dav1dThreadPicture *out_delayed;
        unsigned next;
    } frame_thread;

    // task threading (refer to tc[] for per_thread thingies)
    struct TaskThreadData {
        pthread_mutex_t lock;
        pthread_cond_t cond;
        atomic_uint first;
        unsigned cur;
        // This is used for delayed reset of the task cur pointer when
        // such operation is needed but the thread doesn't enter a critical
        // section (typically when executing the next sbrow task locklessly).
        // See src/thread_task.c:reset_task_cur().
        atomic_uint reset_task_cur;
        atomic_int cond_signaled;
        struct {
            int exec, finished;
            pthread_cond_t cond;
            const Dav1dPicture *in;
            Dav1dPicture *out;
            enum TaskType type;
            atomic_int progress[2]; /* [0]=started, [1]=completed */
            union {
                struct {
                    ALIGN(int8_t grain_lut_8bpc[3][GRAIN_HEIGHT + 1][GRAIN_WIDTH], 16);
                    ALIGN(uint8_t scaling_8bpc[3][256], 64);
                };
                struct {
                    ALIGN(int16_t grain_lut_16bpc[3][GRAIN_HEIGHT + 1][GRAIN_WIDTH], 16);
                    ALIGN(uint8_t scaling_16bpc[3][4096], 64);
                };
            };
        } delayed_fg;
        int inited;
    } task_thread;

    // reference/entropy state
    Dav1dMemPool *segmap_pool;
    Dav1dMemPool *refmvs_pool;
    struct {
        Dav1dThreadPicture p;
        Dav1dRef *segmap;
        Dav1dRef *refmvs;
        unsigned refpoc[7];
    } refs[8];
    Dav1dMemPool *cdf_pool;
    CdfThreadContext cdf[8];

    Dav1dDSPContext dsp[3 /* 8, 10, 12 bits/component */];
    Dav1dPalDSPContext pal_dsp;
    Dav1dRefmvsDSPContext refmvs_dsp;

    Dav1dPicAllocator allocator;
    int apply_grain;
    int operating_point;
    unsigned operating_point_idc;
    int all_layers;
    int max_spatial_id;
    unsigned frame_size_limit;
    int strict_std_compliance;
    int output_invisible_frames;
    enum Dav1dInloopFilterType inloop_filters;
    enum Dav1dDecodeFrameType decode_frame_type;
    int drain;
    enum PictureFlags frame_flags;
    enum Dav1dEventFlags event_flags;
    Dav1dDataProps cached_error_props;
    int cached_error;

    Dav1dLogger logger;

    Dav1dMemPool *picture_pool;
    Dav1dMemPool *pic_ctx_pool;
};

struct Dav1dTask {
    unsigned frame_idx;         // frame thread id
    enum TaskType type;         // task work
    int sby;                    // sbrow

    // task dependencies
    int recon_progress, deblock_progress;
    int deps_skip;
    struct Dav1dTask *next; // only used in task queue
};

struct Dav1dFrameContext {
    Dav1dRef *seq_hdr_ref;
    Dav1dSequenceHeader *seq_hdr;
    Dav1dRef *frame_hdr_ref;
    Dav1dFrameHeader *frame_hdr;
    Dav1dThreadPicture refp[7];
    Dav1dPicture cur; // during block coding / reconstruction
    Dav1dThreadPicture sr_cur; // after super-resolution upscaling
    Dav1dRef *mvs_ref;
    refmvs_temporal_block *mvs, *ref_mvs[7];
    Dav1dRef *ref_mvs_ref[7];
    Dav1dRef *cur_segmap_ref, *prev_segmap_ref;
    uint8_t *cur_segmap;
    const uint8_t *prev_segmap;
    unsigned refpoc[7], refrefpoc[7][7];
    uint8_t gmv_warp_allowed[7];
    CdfThreadContext in_cdf, out_cdf;
    struct Dav1dTileGroup *tile;
    int n_tile_data_alloc;
    int n_tile_data;

    // for scalable references
    struct ScalableMotionParams {
        int scale; // if no scaling, this is 0
        int step;
    } svc[7][2 /* x, y */];
    int resize_step[2 /* y, uv */], resize_start[2 /* y, uv */];

    const Dav1dContext *c;
    Dav1dTileState *ts;
    int n_ts;
    const Dav1dDSPContext *dsp;
    struct {
        recon_b_intra_fn recon_b_intra;
        recon_b_inter_fn recon_b_inter;
        filter_sbrow_fn filter_sbrow;
        filter_sbrow_fn filter_sbrow_deblock_cols;
        filter_sbrow_fn filter_sbrow_deblock_rows;
        void (*filter_sbrow_cdef)(Dav1dTaskContext *tc, int sby);
        filter_sbrow_fn filter_sbrow_resize;
        filter_sbrow_fn filter_sbrow_lr;
        backup_ipred_edge_fn backup_ipred_edge;
        read_coef_blocks_fn read_coef_blocks;
        copy_pal_block_fn copy_pal_block_y;
        copy_pal_block_fn copy_pal_block_uv;
        read_pal_plane_fn read_pal_plane;
        read_pal_uv_fn read_pal_uv;
    } bd_fn;

    int ipred_edge_sz;
    pixel *ipred_edge[3];
    ptrdiff_t b4_stride;
    int w4, h4, bw, bh, sb128w, sb128h, sbh, sb_shift, sb_step, sr_sb128w;
    uint16_t dq[DAV1D_MAX_SEGMENTS][3 /* plane */][2 /* dc/ac */];
    const uint8_t *qm[N_RECT_TX_SIZES][3 /* plane */];
    BlockContext *a;
    int a_sz /* w*tile_rows */;
    refmvs_frame rf;
    uint8_t jnt_weights[7][7];
    int bitdepth_max;

    struct {
        int next_tile_row[2 /* 0: reconstruction, 1: entropy */];
        atomic_int entropy_progress;
        atomic_int deblock_progress; // in sby units
        atomic_uint *frame_progress, *copy_lpf_progress;
        // indexed using t->by * f->b4_stride + t->bx
        Av1Block *b;
        int16_t *cbi; /* bits 0-4: txtp, bits 5-15: eob */
        // indexed using (t->by >> 1) * (f->b4_stride >> 1) + (t->bx >> 1)
        pixel (*pal)[3 /* plane */][8 /* idx */];
        // iterated over inside tile state
        uint8_t *pal_idx;
        coef *cf;
        int prog_sz;
        int cbi_sz, pal_sz, pal_idx_sz, cf_sz;
        // start offsets per tile
        unsigned *tile_start_off;
    } frame_thread;

    // loopfilter
    struct {
        uint8_t (*level)[4];
        Av1Filter *mask;
        Av1Restoration *lr_mask;
        int mask_sz /* w*h */, lr_mask_sz;
        int cdef_buf_plane_sz[2]; /* stride*sbh*4 */
        int cdef_buf_sbh;
        int lr_buf_plane_sz[2]; /* (stride*sbh*4) << sb128 if n_tc > 1, else stride*4 */
        int re_sz /* h */;
        ALIGN(Av1FilterLUT lim_lut, 16);
        ALIGN(uint8_t lvl[8 /* seg_id */][4 /* dir */][8 /* ref */][2 /* is_gmv */], 16);
        int last_sharpness;
        uint8_t *tx_lpf_right_edge[2];
        uint8_t *cdef_line_buf, *lr_line_buf;
        pixel *cdef_line[2 /* pre, post */][3 /* plane */];
        pixel *cdef_lpf_line[3 /* plane */];
        pixel *lr_lpf_line[3 /* plane */];

        // in-loop filter per-frame state keeping
        uint8_t *start_of_tile_row;
        int start_of_tile_row_sz;
        int need_cdef_lpf_copy;
        pixel *p[3], *sr_p[3];
        int restore_planes; // enum LrRestorePlanes
    } lf;

    struct {
        pthread_mutex_t lock;
        pthread_cond_t cond;
        struct TaskThreadData *ttd;
        struct Dav1dTask *tasks, *tile_tasks[2], init_task;
        int num_tasks, num_tile_tasks;
        atomic_int init_done;
        atomic_int done[2];
        int retval;
        int update_set; // whether we need to update CDF reference
        atomic_int error;
        atomic_int task_counter;
        struct Dav1dTask *task_head, *task_tail;
        // Points to the task directly before the cur pointer in the queue.
        // This cur pointer is theoretical here, we actually keep track of the
        // "prev_t" variable. This is needed to not loose the tasks in
        // [head;cur-1] when picking one for execution.
        struct Dav1dTask *task_cur_prev;
        struct { // async task insertion
            atomic_int merge;
            pthread_mutex_t lock;
            Dav1dTask *head, *tail;
        } pending_tasks;
    } task_thread;

    // threading (refer to tc[] for per-thread things)
    struct FrameTileThreadData {
        int (*lowest_pixel_mem)[7][2];
        int lowest_pixel_mem_sz;
    } tile_thread;
};

struct Dav1dTileState {
    CdfContext cdf;
    MsacContext msac;

    struct {
        int col_start, col_end, row_start, row_end; // in 4px units
        int col, row; // in tile units
    } tiling;

    // in sby units, TILE_ERROR after a decoding error
    atomic_int progress[2 /* 0: reconstruction, 1: entropy */];
    struct {
        uint8_t *pal_idx;
        int16_t *cbi;
        coef *cf;
    } frame_thread[2 /* 0: reconstruction, 1: entropy */];

    // in fullpel units, [0] = Y, [1] = UV, used for progress requirements
    // each entry is one tile-sbrow; middle index is refidx
    int (*lowest_pixel)[7][2];

    uint16_t dqmem[DAV1D_MAX_SEGMENTS][3 /* plane */][2 /* dc/ac */];
    const uint16_t (*dq)[3][2];
    int last_qidx;

    union {
        int8_t i8[4];
        uint32_t u32;
    } last_delta_lf;
    ALIGN(uint8_t lflvlmem[8 /* seg_id */][4 /* dir */][8 /* ref */][2 /* is_gmv */], 16);
    const uint8_t (*lflvl)[4][8][2];

    Av1RestorationUnit *lr_ref[3];
};

struct Dav1dTaskContext {
    const Dav1dContext *c;
    const Dav1dFrameContext *f;
    Dav1dTileState *ts;
    int bx, by;
    BlockContext l, *a;
    refmvs_tile rt;
    ALIGN(union, 64) {
        int16_t cf_8bpc [32 * 32];
        int32_t cf_16bpc[32 * 32];
    };
    union {
        uint8_t  al_pal_8bpc [2 /* a/l */][32 /* bx/y4 */][3 /* plane */][8 /* palette_idx */];
        uint16_t al_pal_16bpc[2 /* a/l */][32 /* bx/y4 */][3 /* plane */][8 /* palette_idx */];
    };
    uint8_t pal_sz_uv[2 /* a/l */][32 /* bx4/by4 */];
    ALIGN(union, 64) {
        struct {
            union {
                uint8_t  lap_8bpc [128 * 32];
                uint16_t lap_16bpc[128 * 32];
                struct {
                    int16_t compinter[2][128 * 128];
                    uint8_t seg_mask[128 * 128];
                };
            };
            union {
                // stride=192 for non-SVC, or 320 for SVC
                uint8_t  emu_edge_8bpc [320 * (256 + 7)];
                uint16_t emu_edge_16bpc[320 * (256 + 7)];
            };
        };
        struct {
            union {
                uint8_t levels[32 * 34];
                struct {
                    uint8_t pal_order[64][8];
                    uint8_t pal_ctx[64];
                };
            };
            union {
                int16_t ac[32 * 32]; // intra-only
                uint8_t txtp_map[32 * 32]; // inter-only
            };
            uint8_t pal_idx_y[32 * 64];
            uint8_t pal_idx_uv[64 * 64]; /* also used as pre-pack scratch buffer */
            union {
                struct {
                    uint8_t interintra_8bpc[64 * 64];
                    uint8_t edge_8bpc[257];
                    ALIGN(uint8_t pal_8bpc[3 /* plane */][8 /* palette_idx */], 8);
                };
                struct {
                    uint16_t interintra_16bpc[64 * 64];
                    uint16_t edge_16bpc[257];
                    ALIGN(uint16_t pal_16bpc[3 /* plane */][8 /* palette_idx */], 16);
                };
            };
        };
    } scratch;

    Dav1dWarpedMotionParams warpmv;
    Av1Filter *lf_mask;
    int top_pre_cdef_toggle;
    int8_t *cur_sb_cdef_idx_ptr;
    // for chroma sub8x8, we need to know the filter for all 4 subblocks in
    // a 4x4 area, but the top/left one can go out of cache already, so this
    // keeps it accessible
    enum Filter2d tl_4x4_filter;

    struct {
        int pass;
    } frame_thread;
    struct {
        struct thread_data td;
        struct TaskThreadData *ttd;
        struct FrameTileThreadData *fttd;
        int flushed;
        int die;
    } task_thread;
};

#endif /* DAV1D_SRC_INTERNAL_H */
