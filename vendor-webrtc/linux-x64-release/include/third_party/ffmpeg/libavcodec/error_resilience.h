/*
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

#ifndef AVCODEC_ERROR_RESILIENCE_H
#define AVCODEC_ERROR_RESILIENCE_H

#include <stdint.h>
#include <stdatomic.h>

#include "avcodec.h"
#include "me_cmp.h"

/// current MB is the first after a resync marker
#define VP_START               1
#define ER_AC_ERROR            2
#define ER_DC_ERROR            4
#define ER_MV_ERROR            8
#define ER_AC_END              16
#define ER_DC_END              32
#define ER_MV_END              64

#define ER_MB_ERROR (ER_AC_ERROR|ER_DC_ERROR|ER_MV_ERROR)
#define ER_MB_END   (ER_AC_END|ER_DC_END|ER_MV_END)

typedef struct ERPicture {
    AVFrame *f;
    const struct ThreadFrame *tf;
    const struct ThreadProgress *progress;

    // it is the caller's responsibility to allocate these buffers
    int16_t (*motion_val[2])[2];
    int8_t *ref_index[2];

    uint32_t *mb_type;
    int field_picture;
} ERPicture;

typedef struct ERContext {
    AVCodecContext *avctx;

    me_cmp_func sad;
    int mecc_inited;

    int *mb_index2xy;
    int mb_num;
    int mb_width, mb_height;
    ptrdiff_t mb_stride;
    ptrdiff_t b8_stride;

    atomic_int error_count;
    int error_occurred;
    uint8_t *error_status_table;
    uint8_t *er_temp_buffer;
    int16_t *dc_val[3];
    uint8_t *mbskip_table;
    uint8_t *mbintra_table;
    int mv[2][4][2];

    ERPicture cur_pic;
    ERPicture last_pic;
    ERPicture next_pic;

    int8_t *ref_index[2];
    int16_t (*motion_val_base[2])[2];

    uint16_t pp_time;
    uint16_t pb_time;
    int quarter_sample;
    int partitioned_frame;

    void (*decode_mb)(void *opaque, int ref, int mv_dir, int mv_type,
                      int (*mv)[2][4][2],
                      int mb_x, int mb_y, int mb_intra, int mb_skipped);
    void *opaque;
} ERContext;

void ff_er_frame_start(ERContext *s);

/**
 * Indicate that a frame has finished decoding and perform error concealment
 * in case it has been enabled and is necessary and supported.
 *
 * @param s                  ERContext in use
 * @param decode_error_flags pointer where updated decode_error_flags are written
 *                           if supplied; if not, the new flags are directly
 *                           applied to the AVFrame whose errors are concealed
 */
void ff_er_frame_end(ERContext *s, int *decode_error_flags);
void ff_er_add_slice(ERContext *s, int startx, int starty, int endx, int endy,
                     int status);

#endif /* AVCODEC_ERROR_RESILIENCE_H */
