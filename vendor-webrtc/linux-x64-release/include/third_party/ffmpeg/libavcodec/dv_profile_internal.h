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

#ifndef AVCODEC_DV_PROFILE_INTERNAL_H
#define AVCODEC_DV_PROFILE_INTERNAL_H

#include "avcodec.h"
#include "dv_profile.h"

/**
 *  Print all allowed DV profiles into logctx at specified logging level.
 */
void ff_dv_print_profiles(void *logctx, int loglevel);

/**
 * Get a DV profile for the provided compressed frame.
 */
const AVDVProfile* ff_dv_frame_profile(AVCodecContext* codec, const AVDVProfile *sys,
                                       const uint8_t *frame, unsigned buf_size);

#endif /* AVCODEC_DV_PROFILE_INTERNAL_H */
