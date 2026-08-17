/*
 * GSM common header
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

#ifndef AVCODEC_GSM_H
#define AVCODEC_GSM_H

/* bytes per block */
#define GSM_BLOCK_SIZE     33
#define GSM_MS_BLOCK_SIZE  65
#define MSN_MIN_BLOCK_SIZE 41

/* samples per block */
#define GSM_FRAME_SIZE 160

enum GSMModes {
    GSM_13000 = 0,
    MSN_12400,
    MSN_11800,
    MSN_11200,
    MSN_10600,
    MSN_10000,
    MSN_9400,
    MSN_8800,
    MSN_8200,
    NUM_GSM_MODES
};

#endif /* AVCODEC_GSM_H */
