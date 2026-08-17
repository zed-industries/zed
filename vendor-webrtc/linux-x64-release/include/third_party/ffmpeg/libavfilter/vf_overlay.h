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

#ifndef AVFILTER_OVERLAY_H
#define AVFILTER_OVERLAY_H

#include "libavutil/eval.h"
#include "libavutil/pixdesc.h"
#include "framesync.h"
#include "avfilter.h"

enum var_name {
    VAR_MAIN_W,    VAR_MW,
    VAR_MAIN_H,    VAR_MH,
    VAR_OVERLAY_W, VAR_OW,
    VAR_OVERLAY_H, VAR_OH,
    VAR_HSUB,
    VAR_VSUB,
    VAR_X,
    VAR_Y,
    VAR_N,
#if FF_API_FRAME_PKT
    VAR_POS,
#endif
    VAR_T,
    VAR_VARS_NB
};

enum OverlayFormat {
    OVERLAY_FORMAT_YUV420,
    OVERLAY_FORMAT_YUV420P10,
    OVERLAY_FORMAT_YUV422,
    OVERLAY_FORMAT_YUV422P10,
    OVERLAY_FORMAT_YUV444,
    OVERLAY_FORMAT_YUV444P10,
    OVERLAY_FORMAT_RGB,
    OVERLAY_FORMAT_GBRP,
    OVERLAY_FORMAT_AUTO,
    OVERLAY_FORMAT_NB
};

typedef struct OverlayContext {
    const AVClass *class;
    int x, y;                   ///< position of overlaid picture

    uint8_t main_is_packed_rgb;
    uint8_t main_rgba_map[4];
    uint8_t main_has_alpha;
    uint8_t overlay_is_packed_rgb;
    uint8_t overlay_rgba_map[4];
    uint8_t overlay_has_alpha;
    int format;                 ///< OverlayFormat
    int alpha_format;
    int eval_mode;              ///< EvalMode

    FFFrameSync fs;

    int main_pix_step[4];       ///< steps per pixel for each plane of the main output
    int overlay_pix_step[4];    ///< steps per pixel for each plane of the overlay
    int hsub, vsub;             ///< chroma subsampling values
    const AVPixFmtDescriptor *main_desc; ///< format descriptor for main input

    double var_values[VAR_VARS_NB];
    char *x_expr, *y_expr;

    AVExpr *x_pexpr, *y_pexpr;

    int (*blend_row[4])(uint8_t *d, uint8_t *da, uint8_t *s, uint8_t *a, int w,
                        ptrdiff_t alinesize);
    int (*blend_slice)(AVFilterContext *ctx, void *arg, int jobnr, int nb_jobs);
} OverlayContext;

void ff_overlay_init_x86(OverlayContext *s, int format, int pix_format,
                         int alpha_format, int main_has_alpha);

#endif /* AVFILTER_OVERLAY_H */
