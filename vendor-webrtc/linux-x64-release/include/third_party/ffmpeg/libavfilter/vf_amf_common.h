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

#ifndef AVFILTER_AMF_COMMON_H
#define AVFILTER_AMF_COMMON_H

#include "avfilter.h"

#include "AMF/core/Surface.h"
#include "AMF/components/Component.h"
#include "libavutil/hwcontext_amf.h"

typedef struct AMFFilterContext {
    const AVClass *class;

    int width, height;
    enum AVPixelFormat format;
    int scale_type;
    int color_profile;
    int color_range;
    int primaries;
    int trc;
    int fill;
    int fill_color;
    int keep_ratio;

    // HQScaler properties
    int algorithm;
    float sharpness;

    char *w_expr;
    char *h_expr;
    char *format_str;
    int force_original_aspect_ratio;
    int force_divisible_by;
    int reset_sar;

    AMFComponent        *component;
    AVBufferRef         *amf_device_ref;

    AVBufferRef         *hwframes_in_ref;
    AVBufferRef         *hwframes_out_ref;
    AVBufferRef         *hwdevice_ref;

    AVAMFDeviceContext  *amf_device_ctx;
    int                  local_context;
} AMFFilterContext;

int amf_filter_init(AVFilterContext *avctx);
void amf_filter_uninit(AVFilterContext *avctx);
int amf_init_filter_config(AVFilterLink *outlink, enum AVPixelFormat *in_format);
int amf_copy_surface(AVFilterContext *avctx, const AVFrame *frame, AMFSurface* surface);
void amf_free_amfsurface(void *opaque, uint8_t *data);
AVFrame *amf_amfsurface_to_avframe(AVFilterContext *avctx, AMFSurface* pSurface);
int amf_avframe_to_amfsurface(AVFilterContext *avctx, const AVFrame *frame, AMFSurface** ppSurface);
int amf_setup_input_output_formats(AVFilterContext *avctx, const enum AVPixelFormat *input_pix_fmts, const enum AVPixelFormat *output_pix_fmts);
int amf_filter_filter_frame(AVFilterLink *inlink, AVFrame *in);

#endif /* AVFILTER_AMF_COMMON_H */
