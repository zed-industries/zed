/*
 * Copyright (c) 2022 Niklas Haas
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

/**
 * @file
 * Various functions for dealing with ICC profiles
 */

#ifndef AVCODEC_FFLCMS2_H
#define AVCODEC_FFLCMS2_H

#include "libavutil/csp.h"
#include "libavutil/frame.h"
#include "libavutil/pixfmt.h"

#include <lcms2.h>

typedef struct FFIccContext {
    void *avctx;
    cmsContext ctx;
    cmsToneCurve *curves[AVCOL_TRC_NB]; /* tone curve cache */
} FFIccContext;

/**
 * Initializes an FFIccContext. This must be done prior to using it.
 *
 * Returns 0 on success, or a negative error code.
 */
int ff_icc_context_init(FFIccContext *s, void *avctx);
void ff_icc_context_uninit(FFIccContext *s);

/**
 * Generate an ICC profile for a given combination of color primaries and
 * transfer function. Both values must be set to valid entries (not
 * "undefined") for this function to work.
 *
 * Returns 0 on success, or a negative error code.
 */
int ff_icc_profile_generate(FFIccContext *s,
                            enum AVColorPrimaries color_prim,
                            enum AVColorTransferCharacteristic color_trc,
                            cmsHPROFILE *out_profile);

/**
 * Attach an ICC profile to a frame. Helper wrapper around cmsSaveProfileToMem
 * and av_frame_new_side_data_from_buf.
 *
 * Returns 0 on success, or a negative error code.
 */
int ff_icc_profile_attach(FFIccContext *s, cmsHPROFILE profile, AVFrame *frame);

/**
 * Sanitize an ICC profile to try and fix badly broken values.
 *
 * Returns 0 on success, or a negative error code.
 */
int ff_icc_profile_sanitize(FFIccContext *s, cmsHPROFILE profile);

/**
 * Read the color primaries and white point coefficients encoded by an ICC
 * profile, and return the raw values in `out_primaries`.
 *
 * Returns 0 on success, or a negative error code.
 */
int ff_icc_profile_read_primaries(FFIccContext *s, cmsHPROFILE profile,
                                  AVColorPrimariesDesc *out_primaries);

/**
 * Attempt detecting the transfer characteristic that best approximates the
 * transfer function encoded by an ICC profile. Sets `out_trc` to
 * AVCOL_TRC_UNSPECIFIED if no clear match can be identified.
 *
 * Returns 0 on success (including no match), or a negative error code.
 */
int ff_icc_profile_detect_transfer(FFIccContext *s, cmsHPROFILE profile,
                                   enum AVColorTransferCharacteristic *out_trc);

#endif /* AVCODEC_FFLCMS2_H */
