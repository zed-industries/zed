/*
 * Vidvox Hap
 * Copyright (C) 2015 Vittorio Giovara <vittorio.giovara@gmail.com>
 * Copyright (C) 2015 Tom Butterworth <bangnoise@gmail.com>
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

#ifndef AVCODEC_HAP_H
#define AVCODEC_HAP_H

#include <stddef.h>
#include <stdint.h>

#include "bytestream.h"
#include "texturedsp.h"

enum HapTextureFormat {
    HAP_FMT_RGBDXT1   = 0x0B,
    HAP_FMT_RGBADXT5  = 0x0E,
    HAP_FMT_YCOCGDXT5 = 0x0F,
    HAP_FMT_RGTC1     = 0x01,
};

enum HapCompressor {
    HAP_COMP_NONE    = 0xA0,
    HAP_COMP_SNAPPY  = 0xB0,
    HAP_COMP_COMPLEX = 0xC0,
};

enum HapSectionType {
    HAP_ST_DECODE_INSTRUCTIONS = 0x01,
    HAP_ST_COMPRESSOR_TABLE    = 0x02,
    HAP_ST_SIZE_TABLE          = 0x03,
    HAP_ST_OFFSET_TABLE        = 0x04,
};

typedef struct HapChunk {
    enum HapCompressor compressor;
    uint32_t compressed_offset;
    size_t compressed_size;
    int uncompressed_offset;
    size_t uncompressed_size;
} HapChunk;

typedef struct HapContext {
    const struct AVClass *class;

    GetByteContext gbc;

    enum HapTextureFormat opt_tex_fmt; /* Texture type (encoder only) */
    int opt_chunk_count; /* User-requested chunk count (encoder only) */
    int opt_compressor; /* User-requested compressor (encoder only) */

    int chunk_count;
    HapChunk *chunks;
    int *chunk_results;      /* Results from threaded operations */

    uint8_t *tex_buf;        /* Buffer for compressed texture */
    size_t tex_size;         /* Size of the compressed texture */

    size_t max_snappy;       /* Maximum compressed size for snappy buffer */

    int texture_count;      /* 2 for HAQA, 1 for other version */
    int texture_section_size; /* size of the part of the texture section (for HAPQA) */

    TextureDSPThreadContext enc;
    TextureDSPThreadContext dec[2];
} HapContext;

/*
 * Set the number of chunks in the frame. Returns 0 on success or an error if:
 * - first_in_frame is 0 and the number of chunks has changed
 * - any other error occurs
 */
int ff_hap_set_chunk_count(HapContext *ctx, int count, int first_in_frame);

/*
 * Free resources associated with the context
 */
av_cold void ff_hap_free_context(HapContext *ctx);

/* The first three bytes are the size of the section past the header, or zero
 * if the length is stored in the next long word. The fourth byte in the first
 * long word indicates the type of the current section. */
int ff_hap_parse_section_header(GetByteContext *gbc, int *section_size,
                                enum HapSectionType *section_type);

#endif /* AVCODEC_HAP_H */
