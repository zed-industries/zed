/*
 * MOV CENC (Common Encryption) writer
 * Copyright (c) 2015 Eran Kornblau <erankor at gmail dot com>
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

#ifndef AVFORMAT_MOVENCCENC_H
#define AVFORMAT_MOVENCCENC_H

#include "libavutil/aes_ctr.h"
#include "avformat.h"
#include "avio.h"

#define CENC_KID_SIZE (16)

struct MOVTrack;

typedef struct {
    struct AVAESCTR* aes_ctr;
    uint8_t* auxiliary_info;
    size_t auxiliary_info_size;
    size_t auxiliary_info_alloc_size;
    uint32_t auxiliary_info_entries;

    /* subsample support */
    int use_subsamples;
    uint16_t subsample_count;
    size_t auxiliary_info_subsample_start;
    uint8_t* auxiliary_info_sizes;
    size_t  auxiliary_info_sizes_alloc_size;
} MOVMuxCencContext;

/**
 * Initialize a CENC context
 * @param key encryption key, must have a length of AES_CTR_KEY_SIZE
 * @param use_subsamples when enabled parts of a packet can be encrypted, otherwise the whole packet is encrypted
 */
int ff_mov_cenc_init(MOVMuxCencContext* ctx, uint8_t* encryption_key, int use_subsamples, int bitexact);

/**
 * Free a CENC context
 */
void ff_mov_cenc_free(MOVMuxCencContext* ctx);

/**
 * Write a fully encrypted packet
 */
int ff_mov_cenc_write_packet(MOVMuxCencContext* ctx, AVIOContext *pb, const uint8_t *buf_in, int size);

/**
 * Parse AVC NAL units from annex B format, the nal size and type are written in the clear while the body is encrypted
 */
int ff_mov_cenc_avc_parse_nal_units(MOVMuxCencContext* ctx, AVIOContext *pb, const uint8_t *buf_in, int size);

/**
 * Write AVC NAL units that are in MP4 format, the nal size and type are written in the clear while the body is encrypted
 */
int ff_mov_cenc_avc_write_nal_units(AVFormatContext *s, MOVMuxCencContext* ctx, int nal_length_size,
    AVIOContext *pb, const uint8_t *buf_in, int size);

/**
 * Write the cenc atoms that should reside inside stbl
 */
void ff_mov_cenc_write_stbl_atoms(MOVMuxCencContext* ctx, AVIOContext *pb, int64_t moof_offset);

/**
 * Write the sinf atom, contained inside stsd
 */
int ff_mov_cenc_write_sinf_tag(struct MOVTrack* track, AVIOContext *pb, uint8_t* kid);

#endif /* AVFORMAT_MOVENCCENC_H */
