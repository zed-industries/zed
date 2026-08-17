/*
 * Copyright (c) 2016 Umair Khan <omerjerk@gmail.com>
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

#ifndef AVCODEC_MLZ_H
#define AVCODEC_MLZ_H

#include "get_bits.h"

#define CODE_UNSET          -1
#define CODE_BIT_INIT       9
#define DIC_INDEX_INIT      512     // 2^9
#define DIC_INDEX_MAX       32768   // 2^15
#define FLUSH_CODE          256
#define FREEZE_CODE         257
#define FIRST_CODE          258
#define MAX_CODE            32767
#define TABLE_SIZE          35023   // TABLE_SIZE must be a prime number

/** Dictionary structure for mlz decompression
 */
typedef struct MLZDict {
    int  string_code;
    int  parent_code;
    int  char_code;
    int  match_len;
} MLZDict;

/** MLZ data strucure
 */
typedef struct MLZ {
    int dic_code_bit;
    int current_dic_index_max;
    unsigned int bump_code;
    unsigned int flush_code;
    int next_code;
    int freeze_flag;
    MLZDict* dict;
    void* context;
} MLZ;

/** Initialize the dictionary
 */
int ff_mlz_init_dict(void *context, MLZ *mlz);

/** Flush the dictionary
 */
void ff_mlz_flush_dict(MLZ *dict);

/** Run mlz decompression on the next size bits and the output will be stored in buff
 */
int ff_mlz_decompression(MLZ* mlz, GetBitContext* gb, int size, unsigned char *buff);

#endif /*AVCODEC_MLZ_H*/
