/*
 * AAC Spectral Band Replication function declarations
 * Copyright (c) 2008-2009 Robert Swain ( rob opendot cl )
 * Copyright (c) 2010      Alex Converse <alex.converse@gmail.com>
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
 * AAC Spectral Band Replication function declarations
 * @author Robert Swain ( rob opendot cl )
 */

#ifndef AVCODEC_AACSBR_H
#define AVCODEC_AACSBR_H

#include "get_bits.h"
#include "aac/aacdec.h"

#include "libavutil/attributes_internal.h"

#define ENVELOPE_ADJUSTMENT_OFFSET 2
#define NOISE_FLOOR_OFFSET 6

/**
 * SBR VLC tables
 */
enum {
    T_HUFFMAN_ENV_1_5DB,
    F_HUFFMAN_ENV_1_5DB,
    T_HUFFMAN_ENV_BAL_1_5DB,
    F_HUFFMAN_ENV_BAL_1_5DB,
    T_HUFFMAN_ENV_3_0DB,
    F_HUFFMAN_ENV_3_0DB,
    T_HUFFMAN_ENV_BAL_3_0DB,
    F_HUFFMAN_ENV_BAL_3_0DB,
    T_HUFFMAN_NOISE_3_0DB,
    T_HUFFMAN_NOISE_BAL_3_0DB,
};

/**
 * bs_frame_class - frame class of current SBR frame (14496-3 sp04 p98)
 */
enum {
    FIXFIX,
    FIXVAR,
    VARFIX,
    VARVAR,
};

enum {
    EXTENSION_ID_PS = 2,
};

FF_VISIBILITY_PUSH_HIDDEN
/** Initialize SBR. */
void ff_aac_sbr_init(void);
void ff_aac_sbr_init_fixed(void);
/**
 * Allocate an ExtChannelElement (if necessary) and
 * initialize the SBR context contained in it.
 */
int ff_aac_sbr_ctx_alloc_init(AACDecContext *ac, ChannelElement **che, int id_aac);
int ff_aac_sbr_ctx_alloc_init_fixed(AACDecContext *ac, ChannelElement **che, int id_aac);

/** Close the SBR context implicitly contained in a ChannelElement. */
void ff_aac_sbr_ctx_close(ChannelElement *che);
void ff_aac_sbr_ctx_close_fixed(ChannelElement *che);

/** Decode one SBR element. */
int ff_aac_sbr_decode_extension(AACDecContext *ac, ChannelElement *che,
                                GetBitContext *gb, int crc, int cnt, int id_aac);
int ff_aac_sbr_decode_extension_fixed(AACDecContext *ac, ChannelElement *che,
                                      GetBitContext *gb, int crc, int cnt, int id_aac);

/** Due to channel allocation not being known upon SBR parameter transmission,
 * supply the parameters separately.
 * Functionally identical to ff_aac_sbr_decode_extension() */
int ff_aac_sbr_config_usac(AACDecContext *ac, ChannelElement *che,
                           AACUsacElemConfig *ue);

/** Decode frame SBR data, USAC. */
int ff_aac_sbr_decode_usac_data(AACDecContext *ac, ChannelElement *che,
                                AACUsacElemConfig *ue, GetBitContext *gb,
                                int sbr_ch, int indep_flag);

/** Apply one SBR element to one AAC element. */
void ff_aac_sbr_apply(AACDecContext *ac, ChannelElement *che,
                      int id_aac, void /* float */ *L, void /* float */ *R);
void ff_aac_sbr_apply_fixed(AACDecContext *ac, ChannelElement *che,
                            int id_aac, void /* int */ *L, void /* int */ *R);

FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_AACSBR_H */
