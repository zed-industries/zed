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

/**
 * @file
 * common functions for the dnn based filters
 */

#ifndef AVFILTER_DNN_FILTER_COMMON_H
#define AVFILTER_DNN_FILTER_COMMON_H

#include "dnn_interface.h"

#define DNN_FILTER_CHILD_CLASS_ITERATE(name, backend_mask)                  \
    static const AVClass *name##_child_class_iterate(void **iter)           \
    {                                                                       \
        return  ff_dnn_child_class_iterate_with_mask(iter, (backend_mask)); \
    }

#define AVFILTER_DNN_DEFINE_CLASS_EXT(name, desc, options) \
    static const AVClass name##_class = {       \
        .class_name = desc,                     \
        .item_name  = av_default_item_name,     \
        .option     = options,                  \
        .version    = LIBAVUTIL_VERSION_INT,    \
        .category   = AV_CLASS_CATEGORY_FILTER,            \
        .child_next = ff_dnn_filter_child_next,            \
        .child_class_iterate = name##_child_class_iterate, \
    }

#define AVFILTER_DNN_DEFINE_CLASS(fname, backend_mask)      \
    DNN_FILTER_CHILD_CLASS_ITERATE(fname, backend_mask)     \
    AVFILTER_DNN_DEFINE_CLASS_EXT(fname, #fname, fname##_options)

void *ff_dnn_filter_child_next(void *obj, void *prev);

int ff_dnn_filter_init_child_class(AVFilterContext *filter);

int ff_dnn_init(DnnContext *ctx, DNNFunctionType func_type, AVFilterContext *filter_ctx);
int ff_dnn_set_frame_proc(DnnContext *ctx, FramePrePostProc pre_proc, FramePrePostProc post_proc);
int ff_dnn_set_detect_post_proc(DnnContext *ctx, DetectPostProc post_proc);
int ff_dnn_set_classify_post_proc(DnnContext *ctx, ClassifyPostProc post_proc);
int ff_dnn_get_input(DnnContext *ctx, DNNData *input);
int ff_dnn_get_output(DnnContext *ctx, int input_width, int input_height, int *output_width, int *output_height);
int ff_dnn_execute_model(DnnContext *ctx, AVFrame *in_frame, AVFrame *out_frame);
int ff_dnn_execute_model_classification(DnnContext *ctx, AVFrame *in_frame, AVFrame *out_frame, const char *target);
DNNAsyncStatusType ff_dnn_get_result(DnnContext *ctx, AVFrame **in_frame, AVFrame **out_frame);
int ff_dnn_flush(DnnContext *ctx);
void ff_dnn_uninit(DnnContext *ctx);

#endif
