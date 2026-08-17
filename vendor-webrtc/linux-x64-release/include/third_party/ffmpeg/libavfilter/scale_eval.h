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

#ifndef AVFILTER_SCALE_EVAL_H
#define AVFILTER_SCALE_EVAL_H

#include "avfilter.h"

/**
 * Parse and evaluate string expressions for width and height. Upon success,
 * ff_scale_adjust_dimensions must be called with evaluated width and height
 * to obtain actual target dimensions.
 *
 * Returns 0 upon success, negative value if one of the expressions could
 * not be parsed or if NaN was the result of their evaluation.
 */
int ff_scale_eval_dimensions(void *ctx,
    const char *w_expr, const char *h_expr,
    AVFilterLink *inlink, AVFilterLink *outlink,
    int *ret_w, int *ret_h);

/**
 * Transform evaluated width and height obtained from ff_scale_eval_dimensions
 * into actual target width and height for scaling. Adjustment can occur if one
 * or both of the evaluated values are of the form '-n' or if
 * force_original_aspect_ratio is set. force_divisible_by is used only when
 * force_original_aspect_ratio is set and must be at least 1.
 * w_adj is the input SAR when the output dimensions are intended to be square
 * pixels, else should be 1.
 *
 * Returns negative error code on error or non negative on success
 */
int ff_scale_adjust_dimensions(AVFilterLink *inlink,
    int *ret_w, int *ret_h,
    int force_original_aspect_ratio, int force_divisible_by,
    double w_adj);
#endif
