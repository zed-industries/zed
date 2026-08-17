/*
 * Direct3D 12 HW acceleration video encoder
 *
 * Copyright (c) 2024 Intel Corporation
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

#ifndef AVCODEC_D3D12VA_ENCODE_H
#define AVCODEC_D3D12VA_ENCODE_H

#include "libavutil/fifo.h"
#include "libavutil/hwcontext.h"
#include "libavutil/hwcontext_d3d12va_internal.h"
#include "libavutil/hwcontext_d3d12va.h"
#include "avcodec.h"
#include "internal.h"
#include "hwconfig.h"
#include "hw_base_encode.h"

struct D3D12VAEncodeType;

extern const AVCodecHWConfigInternal *const ff_d3d12va_encode_hw_configs[];

#define MAX_PARAM_BUFFER_SIZE 4096
#define D3D12VA_VIDEO_ENC_ASYNC_DEPTH 8

typedef struct D3D12VAEncodePicture {
    int             header_size;
    int             aligned_header_size;

    AVD3D12VAFrame *input_surface;
    AVD3D12VAFrame *recon_surface;

    AVBufferRef    *output_buffer_ref;
    ID3D12Resource *output_buffer;

    ID3D12Resource *encoded_metadata;
    ID3D12Resource *resolved_metadata;

    D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA pic_ctl;

    int             fence_value;
} D3D12VAEncodePicture;

typedef struct D3D12VAEncodeProfile {
    /**
     * lavc profile value (AV_PROFILE_*).
     */
    int       av_profile;

    /**
     * Supported bit depth.
     */
    int       depth;

    /**
     * Number of components.
     */
    int       nb_components;

    /**
     * Chroma subsampling in width dimension.
     */
    int       log2_chroma_w;

    /**
     * Chroma subsampling in height dimension.
     */
    int       log2_chroma_h;

    /**
     * D3D12 profile value.
     */
    D3D12_VIDEO_ENCODER_PROFILE_DESC d3d12_profile;
} D3D12VAEncodeProfile;

enum {
    RC_MODE_AUTO,
    RC_MODE_CQP,
    RC_MODE_CBR,
    RC_MODE_VBR,
    RC_MODE_QVBR,
    RC_MODE_MAX = RC_MODE_QVBR,
};


typedef struct D3D12VAEncodeRCMode {
    /**
     * Mode from above enum (RC_MODE_*).
     */
    int mode;

    /**
     * Name.
     *
     */
    const char *name;

    /**
     * Uses bitrate parameters.
     *
     */
    int bitrate;

    /**
     * Supports maxrate distinct from bitrate.
     *
     */
    int maxrate;

    /**
     * Uses quality value.
     *
     */
    int quality;

    /**
     * Supports HRD/VBV parameters.
     *
     */
    int hrd;

    /**
     * D3D12 mode value.
     */
    D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE d3d12_mode;
} D3D12VAEncodeRCMode;

typedef struct D3D12VAEncodeContext {
    FFHWBaseEncodeContext base;

    /**
     * Codec-specific hooks.
     */
    const struct D3D12VAEncodeType *codec;

    /**
     * Explicitly set RC mode (otherwise attempt to pick from
     * available modes).
     */
    int explicit_rc_mode;

    /**
     * Explicitly-set QP, for use with the "qp" options.
     * (Forces CQP mode when set, overriding everything else.)
     */
    int explicit_qp;

    /**
     * RC quality level - meaning depends on codec and RC mode.
     * In CQP mode this sets the fixed quantiser value.
     */
    int rc_quality;

    /**
     * Chosen encoding profile details.
     */
    const D3D12VAEncodeProfile *profile;

    AVD3D12VADeviceContext *hwctx;

    /**
     * ID3D12Device3 interface.
     */
    ID3D12Device3 *device3;

    /**
     * ID3D12VideoDevice3 interface.
     */
    ID3D12VideoDevice3 *video_device3;

    /**
     * Pool of (reusable) bitstream output buffers.
     */
    AVBufferPool *output_buffer_pool;

    /**
     * D3D12 video encoder.
     */
    AVBufferRef *encoder_ref;

    ID3D12VideoEncoder *encoder;

    /**
     * D3D12 video encoder heap.
     */
    ID3D12VideoEncoderHeap *encoder_heap;

    /**
     * A cached queue for reusing the D3D12 command allocators.
     *
     * @see https://learn.microsoft.com/en-us/windows/win32/direct3d12/recording-command-lists-and-bundles#id3d12commandallocator
     */
    AVFifo *allocator_queue;

    /**
     * D3D12 command queue.
     */
    ID3D12CommandQueue *command_queue;

    /**
     * D3D12 video encode command list.
     */
    ID3D12VideoEncodeCommandList2 *command_list;

    /**
     * The sync context used to sync command queue.
     */
    AVD3D12VASyncContext sync_ctx;

    /**
     * The bi_not_empty feature.
     */
    int bi_not_empty;

    /**
     * D3D12_FEATURE structures.
     */
    D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS req;

    D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS res_limits;

    /**
     * D3D12_VIDEO_ENCODER structures.
     */
    D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC resolution;

    D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION codec_conf;

    D3D12_VIDEO_ENCODER_RATE_CONTROL rc;

    D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE gop;

    D3D12_VIDEO_ENCODER_LEVEL_SETTING level;
} D3D12VAEncodeContext;

typedef struct D3D12VAEncodeType {
    /**
     * List of supported profiles.
     */
   const D3D12VAEncodeProfile *profiles;

    /**
     * D3D12 codec name.
     */
    D3D12_VIDEO_ENCODER_CODEC d3d12_codec;

    /**
     * Codec feature flags.
     */
    int flags;

    /**
     * Default quality for this codec - used as quantiser or RC quality
     * factor depending on RC mode.
     */
    int default_quality;

    /**
     * Query codec configuration and determine encode parameters like
     * block sizes for surface alignment and slices. If not set, assume
     * that all blocks are 16x16 and that surfaces should be aligned to match
     * this.
     */
    int (*get_encoder_caps)(AVCodecContext *avctx);

    /**
     * Perform any extra codec-specific configuration.
     */
    int (*configure)(AVCodecContext *avctx);

    /**
     * Set codec-specific level setting.
     */
    int (*set_level)(AVCodecContext *avctx);

    /**
     * The size of any private data structure associated with each
     * picture (can be zero if not required).
     */
    size_t picture_priv_data_size;

    /**
     * Fill the corresponding parameters.
     */
    int (*init_sequence_params)(AVCodecContext *avctx);

    int (*init_picture_params)(AVCodecContext *avctx,
                               FFHWBaseEncodePicture *base_pic);

    void (*free_picture_params)(D3D12VAEncodePicture *pic);

    /**
     * Write the packed header data to the provided buffer.
     */
    int (*write_sequence_header)(AVCodecContext *avctx,
                                 char *data, size_t *data_len);
} D3D12VAEncodeType;

int ff_d3d12va_encode_receive_packet(AVCodecContext *avctx, AVPacket *pkt);

int ff_d3d12va_encode_init(AVCodecContext *avctx);
int ff_d3d12va_encode_close(AVCodecContext *avctx);

#define D3D12VA_ENCODE_RC_MODE(name, desc) \
    { #name, desc, 0, AV_OPT_TYPE_CONST, { .i64 = RC_MODE_ ## name }, \
      0, 0, FLAGS, .unit = "rc_mode" }
#define D3D12VA_ENCODE_RC_OPTIONS \
    { "rc_mode",\
      "Set rate control mode", \
      OFFSET(common.explicit_rc_mode), AV_OPT_TYPE_INT, \
      { .i64 = RC_MODE_AUTO }, RC_MODE_AUTO, RC_MODE_MAX, FLAGS, .unit = "rc_mode" }, \
    { "auto", "Choose mode automatically based on other parameters", \
      0, AV_OPT_TYPE_CONST, { .i64 = RC_MODE_AUTO }, 0, 0, FLAGS, .unit = "rc_mode" }, \
    D3D12VA_ENCODE_RC_MODE(CQP,  "Constant-quality"), \
    D3D12VA_ENCODE_RC_MODE(CBR,  "Constant-bitrate"), \
    D3D12VA_ENCODE_RC_MODE(VBR,  "Variable-bitrate"), \
    D3D12VA_ENCODE_RC_MODE(QVBR, "Quality-defined variable-bitrate")

#endif /* AVCODEC_D3D12VA_ENCODE_H */
