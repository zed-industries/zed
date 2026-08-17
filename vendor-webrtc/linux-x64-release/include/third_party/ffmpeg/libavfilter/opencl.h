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

#ifndef AVFILTER_OPENCL_H
#define AVFILTER_OPENCL_H

// The intended target is OpenCL 1.2, so disable warnings for APIs
// deprecated after that.  This primarily applies to clCreateCommandQueue(),
// we can't use the replacement clCreateCommandQueueWithProperties() because
// it was introduced in OpenCL 2.0.
#define CL_USE_DEPRECATED_OPENCL_1_2_APIS

#include "libavutil/bprint.h"
#include "libavutil/buffer.h"
#include "libavutil/hwcontext.h"
#include "libavutil/hwcontext_opencl.h"
#include "libavutil/pixfmt.h"

#include "avfilter.h"

typedef struct OpenCLFilterContext {
    const AVClass     *class;

    AVBufferRef       *device_ref;
    AVHWDeviceContext *device;
    AVOpenCLDeviceContext *hwctx;

    cl_program         program;

    enum AVPixelFormat output_format;
    int                output_width;
    int                output_height;
} OpenCLFilterContext;

// Groups together information about a kernel argument
typedef struct OpenCLKernelArg {
    size_t arg_size;
    const void *arg_val;
} OpenCLKernelArg;

/**
 * set argument to specific Kernel.
 * This macro relies on usage of local label "fail" and variables:
 * avctx, cle and err.
 */
#define CL_SET_KERNEL_ARG(kernel, arg_num, type, arg)          \
    cle = clSetKernelArg(kernel, arg_num, sizeof(type), arg);  \
    if (cle != CL_SUCCESS) {                                   \
        av_log(avctx, AV_LOG_ERROR, "Failed to set kernel "    \
               "argument %d: error %d.\n", arg_num, cle);      \
        err = AVERROR(EIO);                                    \
        goto fail;                                             \
    }

/**
 * A helper macro to handle OpenCL errors. It will assign errcode to
 * variable err, log error msg, and jump to fail label on error.
 */
#define CL_FAIL_ON_ERROR(errcode, ...) do {                    \
        if (cle != CL_SUCCESS) {                               \
            av_log(avctx, AV_LOG_ERROR, __VA_ARGS__);          \
            err = errcode;                                     \
            goto fail;                                         \
        }                                                      \
    } while(0)

/**
 * Create a kernel with the given name.
 *
 * The kernel variable in the context structure must have a name of the form
 * kernel_<kernel_name>.
 *
 * The OpenCLFilterContext variable in the context structure must be named ocf.
 *
 * Requires the presence of a local cl_int variable named cle and a fail label for error
 * handling.
 */
#define CL_CREATE_KERNEL(ctx, kernel_name) do {                                                 \
    ctx->kernel_ ## kernel_name = clCreateKernel(ctx->ocf.program, #kernel_name, &cle);         \
    CL_FAIL_ON_ERROR(AVERROR(EIO), "Failed to create %s kernel: %d.\n", #kernel_name, cle);     \
} while(0)

/**
 * release an OpenCL Kernel
 */
#define CL_RELEASE_KERNEL(k)                                  \
do {                                                          \
    if (k) {                                                  \
        cle = clReleaseKernel(k);                             \
        if (cle != CL_SUCCESS)                                \
            av_log(avctx, AV_LOG_ERROR, "Failed to release "  \
                   "OpenCL kernel: %d.\n", cle);              \
    }                                                         \
} while(0)

/**
 * release an OpenCL Memory Object
 */
#define CL_RELEASE_MEMORY(m)                                  \
do {                                                          \
    if (m) {                                                  \
        cle = clReleaseMemObject(m);                          \
        if (cle != CL_SUCCESS)                                \
            av_log(avctx, AV_LOG_ERROR, "Failed to release "  \
                   "OpenCL memory: %d.\n", cle);              \
    }                                                         \
} while(0)

/**
 * release an OpenCL Command Queue
 */
#define CL_RELEASE_QUEUE(q)                                   \
do {                                                          \
    if (q) {                                                  \
        cle = clReleaseCommandQueue(q);                       \
        if (cle != CL_SUCCESS)                                \
            av_log(avctx, AV_LOG_ERROR, "Failed to release "  \
                   "OpenCL command queue: %d.\n", cle);       \
    }                                                         \
} while(0)

/**
 * Enqueue a kernel with the given information.
 *
 * Kernel arguments are provided as KernelArg structures and are set in the order
 * that they are passed.
 *
 * Requires the presence of a local cl_int variable named cle and a fail label for error
 * handling.
 */
#define CL_ENQUEUE_KERNEL_WITH_ARGS(queue, kernel, global_work_size, local_work_size, event, ...)   \
do {                                                                                                \
    OpenCLKernelArg args[] = {__VA_ARGS__};                                                         \
    for (int i = 0; i < FF_ARRAY_ELEMS(args); i++) {                                                \
        cle = clSetKernelArg(kernel, i, args[i].arg_size, args[i].arg_val);                         \
        if (cle != CL_SUCCESS) {                                                                    \
            av_log(avctx, AV_LOG_ERROR, "Failed to set kernel "                                     \
                "argument %d: error %d.\n", i, cle);                                                \
            err = AVERROR(EIO);                                                                     \
            goto fail;                                                                              \
        }                                                                                           \
    }                                                                                               \
                                                                                                    \
    cle = clEnqueueNDRangeKernel(                                                                   \
        queue,                                                                                      \
        kernel,                                                                                     \
        FF_ARRAY_ELEMS(global_work_size),                                                           \
        NULL,                                                                                       \
        global_work_size,                                                                           \
        local_work_size,                                                                            \
        0,                                                                                          \
        NULL,                                                                                       \
        event                                                                                       \
    );                                                                                              \
    CL_FAIL_ON_ERROR(AVERROR(EIO), "Failed to enqueue kernel: %d.\n", cle);                         \
} while (0)

/**
 * Uses the above macro to enqueue the given kernel and then additionally runs it to
 * completion via clFinish.
 *
 * Requires the presence of a local cl_int variable named cle and a fail label for error
 * handling.
 */
#define CL_RUN_KERNEL_WITH_ARGS(queue, kernel, global_work_size, local_work_size, event, ...) do {  \
    CL_ENQUEUE_KERNEL_WITH_ARGS(                                                                    \
        queue, kernel, global_work_size, local_work_size, event, __VA_ARGS__                        \
    );                                                                                              \
                                                                                                    \
    cle = clFinish(queue);                                                                          \
    CL_FAIL_ON_ERROR(AVERROR(EIO), "Failed to finish command queue: %d.\n", cle);                   \
} while (0)

/**
 * Create a buffer with the given information.
 *
 * The buffer variable in the context structure must be named <buffer_name>.
 *
 * Requires the presence of a local cl_int variable named cle and a fail label for error
 * handling.
 */
#define CL_CREATE_BUFFER_FLAGS(ctx, buffer_name, flags, size, host_ptr) do {                    \
    ctx->buffer_name = clCreateBuffer(                                                          \
        ctx->ocf.hwctx->context,                                                                \
        flags,                                                                                  \
        size,                                                                                   \
        host_ptr,                                                                               \
        &cle                                                                                    \
    );                                                                                          \
    CL_FAIL_ON_ERROR(AVERROR(EIO), "Failed to create buffer %s: %d.\n", #buffer_name, cle);     \
} while(0)

/**
 * Perform a blocking write to a buffer.
 *
 * Requires the presence of a local cl_int variable named cle and a fail label for error
 * handling.
 */
#define CL_BLOCKING_WRITE_BUFFER(queue, buffer, size, host_ptr, event) do {                     \
    cle = clEnqueueWriteBuffer(                                                                 \
        queue,                                                                                  \
        buffer,                                                                                 \
        CL_TRUE,                                                                                \
        0,                                                                                      \
        size,                                                                                   \
        host_ptr,                                                                               \
        0,                                                                                      \
        NULL,                                                                                   \
        event                                                                                   \
    );                                                                                          \
    CL_FAIL_ON_ERROR(AVERROR(EIO), "Failed to write buffer to device: %d.\n", cle);             \
} while(0)

/**
 * Create a buffer with the given information.
 *
 * The buffer variable in the context structure must be named <buffer_name>.
 *
 * Requires the presence of a local cl_int variable named cle and a fail label for error
 * handling.
 */
#define CL_CREATE_BUFFER(ctx, buffer_name, size) CL_CREATE_BUFFER_FLAGS(ctx, buffer_name, 0, size, NULL)

/**
 * Check that the input link contains a suitable hardware frames
 * context and extract the device from it.
 */
int ff_opencl_filter_config_input(AVFilterLink *inlink);

/**
 * Create a suitable hardware frames context for the output.
 */
int ff_opencl_filter_config_output(AVFilterLink *outlink);

/**
 * Initialise an OpenCL filter context.
 */
int ff_opencl_filter_init(AVFilterContext *avctx);

/**
 * Uninitialise an OpenCL filter context.
 */
void ff_opencl_filter_uninit(AVFilterContext *avctx);

/**
 * Load a new OpenCL program from strings in memory.
 *
 * Creates a new program and compiles it for the current device.
 * Will log any build errors if compilation fails.
 */
int ff_opencl_filter_load_program(AVFilterContext *avctx,
                                  const char **program_source_array,
                                  int nb_strings);

/**
 * Load a new OpenCL program from a file.
 *
 * Same as ff_opencl_filter_load_program(), but from a file.
 */
int ff_opencl_filter_load_program_from_file(AVFilterContext *avctx,
                                            const char *filename);

/**
 * Find the work size needed needed for a given plane of an image.
 */
int ff_opencl_filter_work_size_from_image(AVFilterContext *avctx,
                                          size_t *work_size,
                                          AVFrame *frame, int plane,
                                          int block_alignment);
/**
 * Print a 3x3 matrix into a buffer as __constant array, which could
 * be included in an OpenCL program.
*/

void ff_opencl_print_const_matrix_3x3(AVBPrint *buf, const char *name_str,
                                      double mat[3][3]);

/**
 * Gets the command start and end times for the given event and returns the
 * difference (the time that the event took).
 */
cl_ulong ff_opencl_get_event_time(cl_event event);

#endif /* AVFILTER_OPENCL_H */
