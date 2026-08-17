/*
 * NewTek SpeedHQ common header
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

#ifndef AVCODEC_SPEEDHQ_H
#define AVCODEC_SPEEDHQ_H

#include <stdint.h>
#include "libavutil/attributes_internal.h"

#define SPEEDHQ_RL_NB_ELEMS 121

FF_VISIBILITY_PUSH_HIDDEN
extern const uint8_t ff_speedhq_run[SPEEDHQ_RL_NB_ELEMS];
extern const uint8_t ff_speedhq_level[SPEEDHQ_RL_NB_ELEMS];
extern const uint16_t ff_speedhq_vlc_table[SPEEDHQ_RL_NB_ELEMS + 2][2];
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_SPEEDHQ_H */
