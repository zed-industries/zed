/*
 * Copyright (c) 2018 Sergey Lavrushkin
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

/**
 * @file
 * DNN inference engine interface.
 */

#ifndef AVFILTER_DNN_INTERFACE_H
#define AVFILTER_DNN_INTERFACE_H

#include <stdint.h>
#include "libavutil/frame.h"
#include "avfilter.h"

#define DNN_GENERIC_ERROR FFERRTAG('D','N','N','!')

typedef enum {
    DNN_TF = 1,
    DNN_OV = 1 << 1,
    DNN_TH = 1 << 2
} DNNBackendType;

typedef enum {DNN_FLOAT = 1, DNN_UINT8 = 4} DNNDataType;

typedef enum {
    DCO_NONE,
    DCO_BGR,
    DCO_RGB,
} DNNColorOrder;

typedef enum {
    DAST_FAIL,              // something wrong
    DAST_EMPTY_QUEUE,       // no more inference result to get
    DAST_NOT_READY,         // all queued inferences are not finished
    DAST_SUCCESS            // got a result frame successfully
} DNNAsyncStatusType;

typedef enum {
    DFT_NONE,
    DFT_PROCESS_FRAME,      // process the whole frame
    DFT_ANALYTICS_DETECT,   // detect from the whole frame
    DFT_ANALYTICS_CLASSIFY, // classify for each bounding box
}DNNFunctionType;

typedef enum {
    DL_NONE,
    DL_NCHW,
    DL_NHWC,
} DNNLayout;

typedef struct DNNData{
    void *data;
    int dims[4];
    // dt and order together decide the color format
    DNNDataType dt;
    DNNColorOrder order;
    DNNLayout layout;
    float scale;
    float mean;
} DNNData;

typedef struct DNNExecBaseParams {
    const char *input_name;
    const char **output_names;
    uint32_t nb_output;
    AVFrame *in_frame;
    AVFrame *out_frame;
} DNNExecBaseParams;

typedef struct DNNExecClassificationParams {
    DNNExecBaseParams base;
    const char *target;
} DNNExecClassificationParams;

typedef int (*FramePrePostProc)(AVFrame *frame, DNNData *model, AVFilterContext *filter_ctx);
typedef int (*DetectPostProc)(AVFrame *frame, DNNData *output, uint32_t nb, AVFilterContext *filter_ctx);
typedef int (*ClassifyPostProc)(AVFrame *frame, DNNData *output, uint32_t bbox_index, AVFilterContext *filter_ctx);

typedef struct DNNModel{
    // Stores FilterContext used for the interaction between AVFrame and DNNData
    AVFilterContext *filter_ctx;
    // Stores function type of the model
    DNNFunctionType func_type;
    // Gets model input information
    // Just reuse struct DNNData here, actually the DNNData.data field is not needed.
    int (*get_input)(struct DNNModel *model, DNNData *input, const char *input_name);
    // Gets model output width/height with given input w/h
    int (*get_output)(struct DNNModel *model, const char *input_name, int input_width, int input_height,
                                const char *output_name, int *output_width, int *output_height);
    // set the pre process to transfer data from AVFrame to DNNData
    // the default implementation within DNN is used if it is not provided by the filter
    FramePrePostProc frame_pre_proc;
    // set the post process to transfer data from DNNData to AVFrame
    // the default implementation within DNN is used if it is not provided by the filter
    FramePrePostProc frame_post_proc;
    // set the post process to interpret detect result from DNNData
    DetectPostProc detect_post_proc;
    // set the post process to interpret classify result from DNNData
    ClassifyPostProc classify_post_proc;
} DNNModel;

typedef struct TFOptions{
    const AVClass *clazz;

    char *sess_config;
} TFOptions;

typedef struct OVOptions {
    const AVClass *clazz;

    int batch_size;
    int input_resizable;
    DNNLayout layout;
    float scale;
    float mean;
} OVOptions;

typedef struct THOptions {
    const AVClass *clazz;
    int optimize;
} THOptions;

typedef struct DNNModule DNNModule;

typedef struct DnnContext {
    const AVClass *clazz;

    DNNModel *model;

    char *model_filename;
    DNNBackendType backend_type;
    char *model_inputname;
    char *model_outputnames_string;
    char *backend_options;
    int async;

    char **model_outputnames;
    uint32_t nb_outputs;
    const DNNModule *dnn_module;

    int nireq;
    char *device;

#if CONFIG_LIBTENSORFLOW
    TFOptions tf_option;
#endif

#if CONFIG_LIBOPENVINO
    OVOptions ov_option;
#endif
#if CONFIG_LIBTORCH
    THOptions torch_option;
#endif
} DnnContext;

// Stores pointers to functions for loading, executing, freeing DNN models for one of the backends.
struct DNNModule {
    const AVClass clazz;
    DNNBackendType type;
    // Loads model and parameters from given file. Returns NULL if it is not possible.
    DNNModel *(*load_model)(DnnContext *ctx, DNNFunctionType func_type, AVFilterContext *filter_ctx);
    // Executes model with specified input and output. Returns the error code otherwise.
    int (*execute_model)(const DNNModel *model, DNNExecBaseParams *exec_params);
    // Retrieve inference result.
    DNNAsyncStatusType (*get_result)(const DNNModel *model, AVFrame **in, AVFrame **out);
    // Flush all the pending tasks.
    int (*flush)(const DNNModel *model);
    // Frees memory allocated for model.
    void (*free_model)(DNNModel **model);
};

// Initializes DNNModule depending on chosen backend.
const DNNModule *ff_get_dnn_module(DNNBackendType backend_type, void *log_ctx);

void ff_dnn_init_child_class(DnnContext *ctx);
void *ff_dnn_child_next(DnnContext *obj, void *prev);
const AVClass *ff_dnn_child_class_iterate_with_mask(void **iter, uint32_t backend_mask);

static inline int dnn_get_width_idx_by_layout(DNNLayout layout)
{
    return layout == DL_NHWC ? 2 : 3;
}

static inline int dnn_get_height_idx_by_layout(DNNLayout layout)
{
    return layout == DL_NHWC ? 1 : 2;
}

static inline int dnn_get_channel_idx_by_layout(DNNLayout layout)
{
    return layout == DL_NHWC ? 3 : 1;
}

#endif
