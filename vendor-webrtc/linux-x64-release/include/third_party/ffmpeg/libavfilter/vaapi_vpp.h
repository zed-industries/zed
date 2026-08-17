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

#ifndef AVFILTER_VAAPI_VPP_H
#define AVFILTER_VAAPI_VPP_H

#include <va/va.h>
#include <va/va_vpp.h>

#include "libavutil/hwcontext.h"
#include "libavutil/hwcontext_vaapi.h"

#include "avfilter.h"

static inline VASurfaceID ff_vaapi_vpp_get_surface_id(const AVFrame *frame)
{
    return (uintptr_t)frame->data[3];
}

// ARGB black, for VAProcPipelineParameterBuffer.output_background_color.
#define VAAPI_VPP_BACKGROUND_BLACK 0xff000000

typedef struct VAAPIVPPContext {
    const AVClass *class;

    AVVAAPIDeviceContext *hwctx;
    AVBufferRef *device_ref;

    int valid_ids;
    VAConfigID  va_config;
    VAContextID va_context;

    AVBufferRef       *input_frames_ref;
    AVHWFramesContext *input_frames;
    VARectangle        input_region;

    enum AVPixelFormat output_format;
    int output_width;   // computed width
    int output_height;  // computed height

    VABufferID         filter_buffers[VAProcFilterCount];
    int                nb_filter_buffers;

    int passthrough;

    int (*build_filter_params)(AVFilterContext *avctx);

    void (*pipeline_uninit)(AVFilterContext *avctx);
} VAAPIVPPContext;

void ff_vaapi_vpp_ctx_init(AVFilterContext *avctx);

void ff_vaapi_vpp_ctx_uninit(AVFilterContext *avctx);

int ff_vaapi_vpp_query_formats(const AVFilterContext *avctx,
                               AVFilterFormatsConfig **cfg_in,
                               AVFilterFormatsConfig **cfg_out);

void ff_vaapi_vpp_pipeline_uninit(AVFilterContext *avctx);

int ff_vaapi_vpp_config_input(AVFilterLink *inlink);

int ff_vaapi_vpp_config_output(AVFilterLink *outlink);

int ff_vaapi_vpp_init_params(AVFilterContext *avctx,
                             VAProcPipelineParameterBuffer *params,
                             const AVFrame *input_frame,
                             AVFrame *output_frame);

int ff_vaapi_vpp_make_param_buffers(AVFilterContext *avctx,
                                    int type,
                                    const void *data,
                                    size_t size,
                                    int count);

int ff_vaapi_vpp_render_picture(AVFilterContext *avctx,
                                VAProcPipelineParameterBuffer *params,
                                AVFrame *output_frame);

int ff_vaapi_vpp_render_pictures(AVFilterContext *avctx,
                                 VAProcPipelineParameterBuffer *params_list,
                                 int count,
                                 AVFrame *output_frame);

#endif /* AVFILTER_VAAPI_VPP_H */
