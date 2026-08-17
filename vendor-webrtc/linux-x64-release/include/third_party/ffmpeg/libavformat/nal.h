/*
 * NAL helper functions for muxers
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

#ifndef AVFORMAT_NAL_H
#define AVFORMAT_NAL_H

#include <stdint.h>
#include "avio.h"

typedef struct NALU {
    int offset;
    uint32_t size;
} NALU;

typedef struct NALUList {
    NALU *nalus;
    unsigned nalus_array_size;
    unsigned nb_nalus;          ///< valid entries in nalus
} NALUList;

/* This function will parse the given annex B buffer and create
 * a NALUList from it. This list can be passed to ff_nal_units_write_list()
 * to write the access unit reformatted to mp4.
 *
 * @param list A NALUList. The list->nalus and list->nalus_array_size
 *             must be valid when calling this function and may be updated.
 *             nb_nalus is set by this function on success.
 * @param buf  buffer containing annex B H.264 or H.265. Must be padded.
 * @param size size of buf, excluding padding.
 * @return < 0 on error, the size of the mp4-style packet on success.
 */
int ff_nal_units_create_list(NALUList *list, const uint8_t *buf, int size);

/* Writes a NALUList to the specified AVIOContext. The list must originate
 * from ff_nal_units_create_list() with the same buf. */
void ff_nal_units_write_list(const NALUList *list, AVIOContext *pb,
                             const uint8_t *buf);

const uint8_t *ff_nal_find_startcode(const uint8_t *p, const uint8_t *end);
const uint8_t *ff_nal_mp4_find_startcode(const uint8_t *start,
                                         const uint8_t *end,
                                         int nal_length_size);

int ff_nal_parse_units(AVIOContext *s, const uint8_t *buf, int size);
int ff_nal_parse_units_buf(const uint8_t *buf_in, uint8_t **buf, int *size);

uint8_t *ff_nal_unit_extract_rbsp(const uint8_t *src, uint32_t src_len,
                                  uint32_t *dst_len, int header_len);

#endif /* AVFORMAT_NAL_H */
