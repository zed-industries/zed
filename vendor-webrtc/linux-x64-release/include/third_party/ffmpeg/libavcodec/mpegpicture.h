/*
 * Mpeg video formats-related defines and utility functions
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

#ifndef AVCODEC_MPEGPICTURE_H
#define AVCODEC_MPEGPICTURE_H

#include <limits.h>
#include <stddef.h>
#include <stdint.h>

#include "avcodec.h"
#include "threadprogress.h"

#define MPV_MAX_PLANES 3
#define EDGE_WIDTH 16

typedef struct ScratchpadContext {
    uint8_t *edge_emu_buffer;     ///< temporary buffer for if MVs point to out-of-frame data
    uint8_t *obmc_scratchpad;
    union {
        uint8_t *scratchpad_buf;  ///< the other *_scratchpad point into this buffer
        uint8_t *rd_scratchpad;   ///< scratchpad for rate distortion mb decision
    };
    int      linesize;            ///< linesize that the buffers in this context have been allocated for
} ScratchpadContext;

typedef struct BufferPoolContext {
    struct AVRefStructPool *mbskip_table_pool;
    struct AVRefStructPool *qscale_table_pool;
    struct AVRefStructPool *mb_type_pool;
    struct AVRefStructPool *motion_val_pool;
    struct AVRefStructPool *ref_index_pool;
    int alloc_mb_width;                         ///< mb_width  used to allocate tables
    int alloc_mb_height;                        ///< mb_height used to allocate tables
    int alloc_mb_stride;                        ///< mb_stride used to allocate tables
} BufferPoolContext;

/**
 * MPVPicture.
 */
typedef struct MPVPicture {
    struct AVFrame *f;

    int8_t *qscale_table_base;
    int8_t *qscale_table;

    int16_t (*motion_val_base[2])[2];
    int16_t (*motion_val[2])[2];

    uint32_t *mb_type_base;
    uint32_t *mb_type;          ///< types and macros are defined in mpegutils.h

    uint8_t *mbskip_table;

    int8_t *ref_index[2];

    /// RefStruct reference for hardware accelerator private data
    void *hwaccel_picture_private;

    int mb_width;               ///< mb_width  of the tables
    int mb_height;              ///< mb_height of the tables
    int mb_stride;              ///< mb_stride of the tables

    int dummy;                  ///< Picture is a dummy and should not be output
    int field_picture;          ///< whether or not the picture was encoded in separate fields

    int b_frame_score;

    int reference;
    int shared;

    int display_picture_number;
    int coded_picture_number;

    ThreadProgress progress;
} MPVPicture;

typedef struct MPVWorkPicture {
    uint8_t  *data[MPV_MAX_PLANES];
    ptrdiff_t linesize[MPV_MAX_PLANES];

    MPVPicture *ptr;            ///< RefStruct reference

    int8_t *qscale_table;

    int16_t (*motion_val[2])[2];

    uint32_t *mb_type;          ///< types and macros are defined in mpegutils.h

    uint8_t *mbskip_table;

    int8_t *ref_index[2];

    int reference;
} MPVWorkPicture;

/**
 * Allocate a pool of MPVPictures.
 */
struct AVRefStructPool *ff_mpv_alloc_pic_pool(int init_progress);

/**
 * Allocate an MPVPicture's accessories (but not the AVFrame's buffer itself)
 * and set the MPVWorkPicture's fields.
 */
int ff_mpv_alloc_pic_accessories(AVCodecContext *avctx, MPVWorkPicture *pic,
                                 ScratchpadContext *sc,
                                 BufferPoolContext *pools, int mb_height);

/**
 * Check that the linesizes of an AVFrame are consistent with the requirements
 * of mpegvideo.
 * FIXME: There should be no need for this function. mpegvideo should be made
 *        to work with changing linesizes.
 */
int ff_mpv_pic_check_linesize(void *logctx, const struct AVFrame *f,
                              ptrdiff_t *linesizep, ptrdiff_t *uvlinesizep);

int ff_mpv_framesize_alloc(AVCodecContext *avctx,
                           ScratchpadContext *sc, int linesize);

/**
 * Disable allocating the ScratchpadContext's buffers in future calls
 * to ff_mpv_framesize_alloc().
 */
static inline void ff_mpv_framesize_disable(ScratchpadContext *sc)
{
    sc->linesize = INT_MAX;
}

void ff_mpv_unref_picture(MPVWorkPicture *pic);
void ff_mpv_workpic_from_pic(MPVWorkPicture *wpic, MPVPicture *pic);
void ff_mpv_replace_picture(MPVWorkPicture *dst, const MPVWorkPicture *src);

#endif /* AVCODEC_MPEGPICTURE_H */
