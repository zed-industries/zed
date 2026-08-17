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

#ifndef AVCODEC_DYNAMIC_HDR_VIVID_H
#define AVCODEC_DYNAMIC_HDR_VIVID_H

#include "libavutil/hdr_dynamic_vivid_metadata.h"

/**
 * Parse the user data registered ITU-T T.35 to AVbuffer (AVDynamicHDRVivid).
 * @param s A pointer containing the decoded AVDynamicHDRVivid structure.
 * @param data The byte array containing the raw ITU-T T.35 data.
 * @param size Size of the data array in bytes.
 *
 * @return 0 if succeed. Otherwise, returns the appropriate AVERROR.
 */
int ff_parse_itu_t_t35_to_dynamic_hdr_vivid(AVDynamicHDRVivid *s, const uint8_t *data,
                                             int size);

#endif /* AVCODEC_DYNAMIC_HDR_VIVID_H */
