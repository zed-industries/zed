/*
 * Common MSMPEG-4 and VC-1 tables
 * Copyright (c) 2001 Fabrice Bellard
 * Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
 *
 * msmpeg4v1 & v2 stuff by Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_MSMPEG4_VC1_DATA_H
#define AVCODEC_MSMPEG4_VC1_DATA_H

#include <stdint.h>

#include "vlc.h"
#include "libavutil/attributes_internal.h"

FF_VISIBILITY_PUSH_HIDDEN
void ff_msmp4_vc1_vlcs_init_once(void);

#define MSMP4_MB_INTRA_VLC_BITS 9
extern VLCElem ff_msmp4_mb_i_vlc[];
#define MSMP4_DC_VLC_BITS 9
extern const VLCElem *ff_msmp4_dc_vlc[2 /* dc_table_index */][2 /* 0: luma, 1: chroma */];

/* intra picture macroblock coded block pattern */
extern const uint16_t ff_msmp4_mb_i_table[64][2];

#define WMV1_SCANTABLE_COUNT 4

extern const uint8_t ff_wmv1_scantable[WMV1_SCANTABLE_COUNT][64];

extern const uint32_t ff_msmp4_dc_tables[2 /* dc_table_index */][2 /* 0: luma, 1: chroma */][120][2];
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_MSMPEG4_VC1_DATA_H */
