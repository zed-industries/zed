/*
 * JPEG-related tables
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

#ifndef AVCODEC_JPEGTABLES_H
#define AVCODEC_JPEGTABLES_H

#include <stdint.h>

#include "libavutil/attributes_internal.h"

FF_VISIBILITY_PUSH_HIDDEN
extern const uint8_t ff_mjpeg_bits_dc_luminance[];
extern const uint8_t ff_mjpeg_val_dc[];

extern const uint8_t ff_mjpeg_bits_dc_chrominance[];

extern const uint8_t ff_mjpeg_bits_ac_luminance[];
extern const uint8_t ff_mjpeg_val_ac_luminance[];

extern const uint8_t ff_mjpeg_bits_ac_chrominance[];
extern const uint8_t ff_mjpeg_val_ac_chrominance[];
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_JPEGTABLES_H */
