/*
 * Copyright (c) 2000-2002 Fabrice Bellard
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
 * rl header.
 */

#ifndef AVCODEC_RL_H
#define AVCODEC_RL_H

#include <stdint.h>

#include "vlc.h"

/* run length table */
#define MAX_RUN    64
#define MAX_LEVEL  64

/** RLTable. */
typedef struct RLTable {
    int n;                         ///< number of entries of table_vlc minus 1
    int last;                      ///< number of values for last = 0
    const uint16_t (*table_vlc)[2];
    const int8_t *table_run;
    const int8_t *table_level;
    uint8_t *index_run[2];         ///< encoding only
    int8_t *max_level[2];          ///< encoding & decoding
    int8_t *max_run[2];            ///< encoding & decoding
    RL_VLC_ELEM *rl_vlc[32];       ///< decoding only
} RLTable;

/**
 * Initialize max_level and index_run from table_run and table_level;
 * this is equivalent to initializing RLTable.max_level[0] and
 * RLTable.index_run[0] with ff_rl_init().
 */
void ff_rl_init_level_run(uint8_t max_level[MAX_LEVEL + 1],
                          uint8_t index_run[MAX_RUN + 1],
                          const uint8_t table_run[/* n */],
                          const uint8_t table_level[/* n*/], int n);

/**
 * Initialize index_run, max_level and max_run from n, last, table_vlc,
 * table_run and table_level.
 * @param static_store static uint8_t array[2][2*MAX_RUN + MAX_LEVEL + 3]
 *                     to hold the level and run tables.
 * @note  This function does not touch rl_vlc at all, hence there is no need
 *        to synchronize calls to ff_rl_init() and ff_rl_init_vlc() using the
 *        same RLTable.
 */
void ff_rl_init(RLTable *rl, uint8_t static_store[2][2*MAX_RUN + MAX_LEVEL + 3]);

/**
 * Initialize rl_vlc from n, last, table_vlc, table_run and table_level.
 * All rl_vlc pointers to be initialized must already point to a static
 * buffer of `static_size` RL_VLC_ELEM elements; if a pointer is NULL,
 * initializing further VLCs stops.
 * @note  This function does not touch what ff_rl_init() initializes at all,
 *        hence there is no need to synchronize calls to ff_rl_init() and
 *        ff_rl_init_vlc() using the same RLTable.
 */
void ff_rl_init_vlc(RLTable *rl, unsigned static_size);

#define VLC_INIT_RL(rl, static_size)\
{\
    static RL_VLC_ELEM rl_vlc_table[32][static_size];\
\
    for (int q = 0; q < 32; q++) \
        rl.rl_vlc[q] = rl_vlc_table[q]; \
\
    ff_rl_init_vlc(&rl, static_size); \
}

#define INIT_FIRST_VLC_RL(rl, static_size)              \
do {                                                    \
    static RL_VLC_ELEM rl_vlc_table[static_size];       \
                                                        \
    rl.rl_vlc[0] = rl_vlc_table;                        \
    ff_rl_init_vlc(&rl, static_size);                   \
} while (0)

static inline int get_rl_index(const RLTable *rl, int last, int run, int level)
{
    int index;
    index = rl->index_run[last][run];
    if (index >= rl->n)
        return rl->n;
    if (level > rl->max_level[last][run])
        return rl->n;
    return index + level - 1;
}

#endif /* AVCODEC_RL_H */
