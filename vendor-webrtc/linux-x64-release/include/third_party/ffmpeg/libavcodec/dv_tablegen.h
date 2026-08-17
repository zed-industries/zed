/*
 * Header file for hardcoded DV tables
 *
 * Copyright (c) 2010 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
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

#ifndef AVCODEC_DV_TABLEGEN_H
#define AVCODEC_DV_TABLEGEN_H

#include <stdint.h>
#include "libavutil/attributes.h"

#include "dvdata.h"

#if CONFIG_SMALL
#define DV_VLC_MAP_RUN_SIZE  15
#define DV_VLC_MAP_LEV_SIZE  23
#else
#define DV_VLC_MAP_RUN_SIZE  64
#define DV_VLC_MAP_LEV_SIZE 512 // FIXME sign was removed so this should be /2 but needs check
#endif

/* VLC encoding lookup table */
typedef struct dv_vlc_pair {
    uint32_t vlc;
    uint32_t size;
} dv_vlc_pair;

#if CONFIG_HARDCODED_TABLES
#define dv_vlc_map_tableinit()
#include "libavcodec/dv_tables.h"
#else
static struct dv_vlc_pair dv_vlc_map[DV_VLC_MAP_RUN_SIZE][DV_VLC_MAP_LEV_SIZE];

static av_cold void dv_vlc_map_tableinit(void)
{
    uint32_t code = 0;
    int i, j;
    for (int i = 0; i < NB_DV_VLC; i++) {
        uint32_t cur_code = code >> (32 - ff_dv_vlc_len[i]);
        code += 1U << (32 - ff_dv_vlc_len[i]);
        if (ff_dv_vlc_run[i] >= DV_VLC_MAP_RUN_SIZE)
            continue;
#if CONFIG_SMALL
        if (ff_dv_vlc_level[i] >= DV_VLC_MAP_LEV_SIZE)
            continue;
#endif

        if (dv_vlc_map[ff_dv_vlc_run[i]][ff_dv_vlc_level[i]].size != 0)
            continue;

        dv_vlc_map[ff_dv_vlc_run[i]][ff_dv_vlc_level[i]].vlc  =
            cur_code << (!!ff_dv_vlc_level[i]);
        dv_vlc_map[ff_dv_vlc_run[i]][ff_dv_vlc_level[i]].size =
            ff_dv_vlc_len[i]   + (!!ff_dv_vlc_level[i]);
    }
    for (i = 0; i < DV_VLC_MAP_RUN_SIZE; i++) {
#if CONFIG_SMALL
        for (j = 1; j < DV_VLC_MAP_LEV_SIZE; j++) {
            if (dv_vlc_map[i][j].size == 0) {
                dv_vlc_map[i][j].vlc  = dv_vlc_map[0][j].vlc |
                                        (dv_vlc_map[i - 1][0].vlc <<
                                         dv_vlc_map[0][j].size);
                dv_vlc_map[i][j].size = dv_vlc_map[i - 1][0].size +
                                        dv_vlc_map[0][j].size;
            }
        }
#else
        for (j = 1; j < DV_VLC_MAP_LEV_SIZE / 2; j++) {
            if (dv_vlc_map[i][j].size == 0) {
                dv_vlc_map[i][j].vlc  = dv_vlc_map[0][j].vlc |
                                        (dv_vlc_map[i - 1][0].vlc <<
                                         dv_vlc_map[0][j].size);
                dv_vlc_map[i][j].size = dv_vlc_map[i - 1][0].size +
                                        dv_vlc_map[0][j].size;
            }
            dv_vlc_map[i][((uint16_t) (-j)) & 0x1ff].vlc  = dv_vlc_map[i][j].vlc | 1;
            dv_vlc_map[i][((uint16_t) (-j)) & 0x1ff].size = dv_vlc_map[i][j].size;
        }
#endif
    }
}
#endif /* CONFIG_HARDCODED_TABLES */

#endif /* AVCODEC_DV_TABLEGEN_H */
