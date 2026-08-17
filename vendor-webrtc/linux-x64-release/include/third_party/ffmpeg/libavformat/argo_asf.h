/*
 * Argonaut Games ASF (de)muxer
 *
 * Copyright (C) 2020 Zane van Iperen (zane@zanevaniperen.com)
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

#ifndef AVFORMAT_ARGO_ASF_H
#define AVFORMAT_ARGO_ASF_H

#include <stdint.h>
#include "libavutil/macros.h"

#include "avformat.h"

#define ASF_TAG                 MKTAG('A', 'S', 'F', '\0')
#define ASF_FILE_HEADER_SIZE    24
#define ASF_CHUNK_HEADER_SIZE   20
#define ASF_SAMPLE_COUNT        32
#define ASF_MIN_BUFFER_SIZE     FFMAX(ASF_FILE_HEADER_SIZE, ASF_CHUNK_HEADER_SIZE)
#define ASF_NAME_SIZE           8

typedef struct ArgoASFFileHeader {
    uint32_t    magic;          /*< Magic Number, {'A', 'S', 'F', '\0'} */
    uint16_t    version_major;  /*< File Major Version. */
    uint16_t    version_minor;  /*< File Minor Version. */
    uint32_t    num_chunks;     /*< No. chunks in the file. */
    uint32_t    chunk_offset;   /*< Offset to the first chunk from the start of the file. */
    char        name[ASF_NAME_SIZE + 1]; /*< Name, +1 for NULL-terminator. */
} ArgoASFFileHeader;

typedef struct ArgoASFChunkHeader {
    uint32_t    num_blocks;     /*< No. blocks in the chunk. */
    uint32_t    num_samples;    /*< No. samples per channel in a block. Always 32. */
    uint32_t    unk1;           /*< Unknown */
    uint16_t    sample_rate;    /*< Sample rate. */
    uint16_t    unk2;           /*< Unknown. */
    uint32_t    flags;          /*< Stream flags. */
} ArgoASFChunkHeader;

enum {
    ASF_CF_BITS_PER_SAMPLE  = (1 << 0), /*< 16-bit if set, 8 otherwise.      */
    ASF_CF_STEREO           = (1 << 1), /*< Stereo if set, mono otherwise.   */
    ASF_CF_ALWAYS1_1        = (1 << 2), /*< Unknown, always seems to be set. */
    ASF_CF_ALWAYS1_2        = (1 << 3), /*< Unknown, always seems to be set. */

    ASF_CF_ALWAYS1          = ASF_CF_ALWAYS1_1 | ASF_CF_ALWAYS1_2,
    ASF_CF_ALWAYS0          = ~(ASF_CF_BITS_PER_SAMPLE | ASF_CF_STEREO | ASF_CF_ALWAYS1)
};

void ff_argo_asf_parse_file_header(ArgoASFFileHeader *hdr, const uint8_t *buf);
int  ff_argo_asf_validate_file_header(AVFormatContext *s, const ArgoASFFileHeader *hdr);
void ff_argo_asf_parse_chunk_header(ArgoASFChunkHeader *hdr, const uint8_t *buf);
int  ff_argo_asf_fill_stream(AVFormatContext *s, AVStream *st, const ArgoASFFileHeader *fhdr,
                             const ArgoASFChunkHeader *ckhdr);

#endif /* AVFORMAT_ARGO_ASF_H */
