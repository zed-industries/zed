/*
 * tables and functions for demuxing SIPR audio muxed RealMedia style
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

#ifndef AVFORMAT_RMSIPR_H
#define AVFORMAT_RMSIPR_H

#include <stdint.h>

extern const unsigned char ff_sipr_subpk_size[4];

/**
 * Perform 4-bit block reordering for SIPR data.
 *
 * @param buf SIPR data
 */
void ff_rm_reorder_sipr_data(uint8_t *buf, int sub_packet_h, int framesize);

#endif /* AVFORMAT_RMSIPR_H */
