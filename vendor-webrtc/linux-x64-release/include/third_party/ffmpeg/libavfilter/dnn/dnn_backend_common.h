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
 * DNN common functions different backends.
 */

#ifndef AVFILTER_DNN_DNN_BACKEND_COMMON_H
#define AVFILTER_DNN_DNN_BACKEND_COMMON_H

#include "queue.h"
#include "../dnn_interface.h"
#include "libavutil/thread.h"

#define DNN_DEFINE_CLASS_EXT(name, desc, options) \
    {                                           \
        .class_name = desc,                     \
        .item_name  = av_default_item_name,     \
        .option     = options,                  \
        .version    = LIBAVUTIL_VERSION_INT,    \
        .category   = AV_CLASS_CATEGORY_FILTER, \
    }
#define DNN_DEFINE_CLASS(fname) \
    DNN_DEFINE_CLASS_EXT(fname, #fname, fname##_options)

// one task for one function call from dnn interface
typedef struct TaskItem {
    void *model; // model for the backend
    AVFrame *in_frame;
    AVFrame *out_frame;
    const char *input_name;
    const char **output_names;
    uint8_t async;
    uint8_t do_ioproc;
    uint32_t nb_output;
    uint32_t inference_todo;
    uint32_t inference_done;
} TaskItem;

// one task might have multiple inferences
typedef struct LastLevelTaskItem {
    TaskItem *task;
    uint32_t bbox_index;
} LastLevelTaskItem;

/**
 * Common Async Execution Mechanism for the DNN Backends.
 */
typedef struct DNNAsyncExecModule {
    /**
     * Synchronous inference function for the backend
     * with corresponding request item as the argument.
     */
    int (*start_inference)(void *request);

    /**
     * Completion Callback for the backend.
     * Expected argument type of callback must match that
     * of the inference function.
     */
    void (*callback)(void *args);

    /**
     * Argument for the execution functions.
     * i.e. Request item for the backend.
     */
    void *args;
#if HAVE_PTHREAD_CANCEL
    pthread_t thread_id;
    pthread_attr_t thread_attr;
#endif
} DNNAsyncExecModule;

int ff_check_exec_params(void *ctx, DNNBackendType backend, DNNFunctionType func_type, DNNExecBaseParams *exec_params);

/**
 * Fill the Task for Backend Execution. It should be called after
 * checking execution parameters using ff_check_exec_params.
 *
 * @param task pointer to the allocated task
 * @param exec_param pointer to execution parameters
 * @param backend_model void pointer to the backend model
 * @param async flag for async execution. Must be 0 or 1
 * @param do_ioproc flag for IO processing. Must be 0 or 1
 *
 * @returns 0 if successful or error code otherwise.
 */
int ff_dnn_fill_task(TaskItem *task, DNNExecBaseParams *exec_params, void *backend_model, int async, int do_ioproc);

/**
 * Join the Async Execution thread and set module pointers to NULL.
 *
 * @param async_module pointer to DNNAsyncExecModule module
 *
 * @returns 0 if successful or error code otherwise.
 */
int ff_dnn_async_module_cleanup(DNNAsyncExecModule *async_module);

/**
 * Start asynchronous inference routine for the TensorFlow
 * model on a detached thread. It calls the completion callback
 * after the inference completes. Completion callback and inference
 * function must be set before calling this function.
 *
 * If POSIX threads aren't supported, the execution rolls back
 * to synchronous mode, calling completion callback after inference.
 *
 * @param ctx pointer to the backend context
 * @param async_module pointer to DNNAsyncExecModule module
 *
 * @returns 0 on the start of async inference or error code otherwise.
 */
int ff_dnn_start_inference_async(void *ctx, DNNAsyncExecModule *async_module);

/**
 * Extract input and output frame from the Task Queue after
 * asynchronous inference.
 *
 * @param task_queue pointer to the task queue of the backend
 * @param in double pointer to the input frame
 * @param out double pointer to the output frame
 *
 * @retval DAST_EMPTY_QUEUE if task queue is empty
 * @retval DAST_NOT_READY if inference not completed yet.
 * @retval DAST_SUCCESS if result successfully extracted
 */
DNNAsyncStatusType ff_dnn_get_result_common(Queue *task_queue, AVFrame **in, AVFrame **out);

/**
 * Allocate input and output frames and fill the Task
 * with execution parameters.
 *
 * @param task pointer to the allocated task
 * @param exec_params pointer to execution parameters
 * @param backend_model void pointer to the backend model
 * @param input_height height of input frame
 * @param input_width width of input frame
 * @param ctx pointer to the backend context
 *
 * @returns 0 if successful or error code otherwise.
 */
int ff_dnn_fill_gettingoutput_task(TaskItem *task, DNNExecBaseParams *exec_params, void *backend_model, int input_height, int input_width, void *ctx);

#endif
