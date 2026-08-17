/*
 * Direct3D 12 HW acceleration video decoder
 *
 * copyright (c) 2022-2023 Wu Jianhua <toqsxw@outlook.com>
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

#ifndef AVCODEC_D3D12VA_DECODE_H
#define AVCODEC_D3D12VA_DECODE_H

#include "libavutil/fifo.h"
#include "libavutil/hwcontext.h"
#include "libavutil/hwcontext_d3d12va.h"
#include "avcodec.h"
#include "internal.h"
#include "hwaccel_internal.h"

/**
 * @brief This structure is used to provide the necessary configurations and data
 * to the FFmpeg Direct3D 12 HWAccel implementation for video decoder.
 */
typedef struct D3D12VADecodeContext {
    AVBufferRef *decoder_ref;

    /**
     * D3D12 video decoder
     */
    ID3D12VideoDecoder *decoder;

    /**
     * D3D12 video decoder heap
     */
    ID3D12VideoDecoderHeap *decoder_heap;

    /**
     * D3D12 configuration used to create the decoder
     *
     * Specified by decoders
     */
    D3D12_VIDEO_DECODE_CONFIGURATION cfg;

    /**
     * A cached queue for reusing the D3D12 command allocators and upload buffers
     *
     * @see https://learn.microsoft.com/en-us/windows/win32/direct3d12/recording-command-lists-and-bundles#id3d12commandallocator
     */
    AVFifo *objects_queue;

    /**
     * D3D12 command queue
     */
    ID3D12CommandQueue *command_queue;

    /**
     * D3D12 video decode command list
     */
    ID3D12VideoDecodeCommandList *command_list;

    /**
     * The array of resources used for reference frames
     *
     * The ref_resources.length is the same as D3D12VADecodeContext.max_num_ref
     */
    ID3D12Resource **ref_resources;

    /**
     * The array of subresources used for reference frames
     *
     * The ref_subresources.length is the same as D3D12VADecodeContext.max_num_ref
     */
    UINT *ref_subresources;

    /**
     * Maximum number of reference frames
     */
    UINT max_num_ref;

    /**
     * Used mask used to record reference frames indices
     */
    UINT used_mask;

    /**
     * Bitstream size for each frame
     */
    UINT bitstream_size;

    /**
     * The sync context used to sync command queue
     */
    AVD3D12VASyncContext sync_ctx;

    /**
     * A pointer to AVD3D12VADeviceContext used to create D3D12 objects
     */
    AVD3D12VADeviceContext *device_ctx;

    /**
     * Pixel format
     */
    enum AVPixelFormat pix_fmt;

    /**
     * Private to the FFmpeg AVHWAccel implementation
     */
    unsigned report_id;
} D3D12VADecodeContext;

/**
 * @}
 */
#define D3D12VA_VIDEO_DEC_ASYNC_DEPTH 36
#define D3D12VA_DECODE_CONTEXT(avctx) ((D3D12VADecodeContext *)((avctx)->internal->hwaccel_priv_data))
#define D3D12VA_FRAMES_CONTEXT(avctx) ((AVHWFramesContext *)(avctx)->hw_frames_ctx->data)

/**
 * @brief Get a suitable maximum bitstream size
 *
 * Creating and destroying a resource on d3d12 needs sync and reallocation, so use this function
 * to help allocate a big enough bitstream buffer to avoid recreating resources when decoding.
 *
 * @return the suitable size
 */
int ff_d3d12va_get_suitable_max_bitstream_size(AVCodecContext *avctx);

/**
 * @brief init D3D12VADecodeContext
 *
 * @return Error code (ret < 0 if failed)
 */
int ff_d3d12va_decode_init(AVCodecContext *avctx);

/**
 * @brief uninit D3D12VADecodeContext
 *
 * @return Error code (ret < 0 if failed)
 */
int ff_d3d12va_decode_uninit(AVCodecContext *avctx);

/**
 * @brief d3d12va common frame params
 *
 * @return Error code (ret < 0 if failed)
 */
int ff_d3d12va_common_frame_params(AVCodecContext *avctx, AVBufferRef *hw_frames_ctx);

/**
 * @brief d3d12va common end frame
 *
 * @param avctx    codec context
 * @param frame    current output frame
 * @param pp       picture parameters
 * @param pp_size  the size of the picture parameters
 * @param qm       quantization matrix
 * @param qm_size  the size of the quantization matrix
 * @param callback update decoder-specified input stream arguments
 * @return Error code (ret < 0 if failed)
 */
int ff_d3d12va_common_end_frame(AVCodecContext *avctx, AVFrame *frame,
    const void *pp, unsigned pp_size,
    const void *qm, unsigned qm_size,
    int(*)(AVCodecContext *, D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS *, ID3D12Resource *));

#endif /* AVCODEC_D3D12VA_DEC_H */
