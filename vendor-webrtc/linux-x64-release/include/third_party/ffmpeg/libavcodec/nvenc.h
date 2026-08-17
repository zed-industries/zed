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

#ifndef AVCODEC_NVENC_H
#define AVCODEC_NVENC_H

#include "config.h"

#if CONFIG_D3D11VA
#define COBJMACROS
#include "libavutil/hwcontext_d3d11va.h"
#else
typedef void ID3D11Device;
#endif

#include <ffnvcodec/nvEncodeAPI.h>

#include "compat/cuda/dynlink_loader.h"
#include "libavutil/buffer.h"
#include "libavutil/fifo.h"
#include "libavutil/opt.h"
#include "hwconfig.h"

#include "avcodec.h"

#define MAX_REGISTERED_FRAMES 64
#define RC_MODE_DEPRECATED 0x800000
#define RCD(rc_mode) ((rc_mode) | RC_MODE_DEPRECATED)

#define NVENCAPI_CHECK_VERSION(major, minor) \
    ((major) < NVENCAPI_MAJOR_VERSION || ((major) == NVENCAPI_MAJOR_VERSION && (minor) <= NVENCAPI_MINOR_VERSION))

// SDK 8.1 compile time feature checks
#if NVENCAPI_CHECK_VERSION(8, 1)
#define NVENC_HAVE_BFRAME_REF_MODE
#define NVENC_HAVE_QP_MAP_MODE
#endif

// SDK 9.0 compile time feature checks
#if NVENCAPI_CHECK_VERSION(9, 0)
#define NVENC_HAVE_HEVC_BFRAME_REF_MODE
#endif

// SDK 9.1 compile time feature checks
#if NVENCAPI_CHECK_VERSION(9, 1)
#define NVENC_HAVE_MULTIPLE_REF_FRAMES
#define NVENC_HAVE_CUSTREAM_PTR
#define NVENC_HAVE_GETLASTERRORSTRING
#endif

// SDK 10.0 compile time feature checks
#if NVENCAPI_CHECK_VERSION(10, 0)
#define NVENC_HAVE_NEW_PRESETS
#define NVENC_HAVE_MULTIPASS
#define NVENC_HAVE_LDKFS
#define NVENC_HAVE_H264_LVL6
#define NVENC_HAVE_HEVC_CONSTRAINED_ENCODING
#endif

// SDK 11.1 compile time feature checks
#if NVENCAPI_CHECK_VERSION(11, 1)
#define NVENC_HAVE_QP_CHROMA_OFFSETS
#define NVENC_HAVE_SINGLE_SLICE_INTRA_REFRESH
#endif

// SDK 12.0 compile time feature checks
#if NVENCAPI_CHECK_VERSION(12, 0)
#define NVENC_HAVE_HEVC_OUTPUT_RECOVERY_POINT_SEI
#endif

// SDK 12.1 compile time feature checks
#if NVENCAPI_CHECK_VERSION(12, 1)
#define NVENC_NO_DEPRECATED_RC
#define NVENC_HAVE_SPLIT_FRAME_ENCODING
#endif

// SDK 12.2 compile time feature checks
#if NVENCAPI_CHECK_VERSION(12, 2)
#define NVENC_HAVE_NEW_BIT_DEPTH_API
#define NVENC_HAVE_TEMPORAL_FILTER
#define NVENC_HAVE_LOOKAHEAD_LEVEL
#define NVENC_HAVE_UHQ_TUNING
#define NVENC_HAVE_UNIDIR_B
#define NVENC_HAVE_TIME_CODE // added in 12.0, but incomplete until 12.2
#endif

// SDK 13.0 compile time feature checks
#if NVENCAPI_CHECK_VERSION(13, 0)
#define NVENC_HAVE_H264_10BIT_SUPPORT
#define NVENC_HAVE_422_SUPPORT
#define NVENC_HAVE_AV1_UHQ_TUNING
#define NVENC_HAVE_H264_AND_AV1_TEMPORAL_FILTER
#define NVENC_HAVE_HEVC_AND_AV1_MASTERING_METADATA
#endif

typedef struct NvencSurface
{
    NV_ENC_INPUT_PTR input_surface;
    AVFrame *in_ref;
    int reg_idx;
    int width;
    int height;
    int pitch;

    NV_ENC_OUTPUT_PTR output_surface;
    NV_ENC_BUFFER_FORMAT format;
} NvencSurface;

typedef struct NvencFrameData
{
    int64_t duration;

    void        *frame_opaque;
    AVBufferRef *frame_opaque_ref;
} NvencFrameData;

typedef struct NvencDynLoadFunctions
{
    CudaFunctions *cuda_dl;
    NvencFunctions *nvenc_dl;

    NV_ENCODE_API_FUNCTION_LIST nvenc_funcs;
    int nvenc_device_count;
} NvencDynLoadFunctions;

enum {
    PRESET_DEFAULT = 0,
    PRESET_SLOW,
    PRESET_MEDIUM,
    PRESET_FAST,
    PRESET_HP,
    PRESET_HQ,
    PRESET_BD ,
    PRESET_LOW_LATENCY_DEFAULT ,
    PRESET_LOW_LATENCY_HQ ,
    PRESET_LOW_LATENCY_HP,
    PRESET_LOSSLESS_DEFAULT,
    PRESET_LOSSLESS_HP,
#ifdef NVENC_HAVE_NEW_PRESETS
    PRESET_P1,
    PRESET_P2,
    PRESET_P3,
    PRESET_P4,
    PRESET_P5,
    PRESET_P6,
    PRESET_P7,
#endif
};

enum {
    NV_ENC_H264_PROFILE_BASELINE,
    NV_ENC_H264_PROFILE_MAIN,
    NV_ENC_H264_PROFILE_HIGH,
#ifdef NVENC_HAVE_H264_10BIT_SUPPORT
    NV_ENC_H264_PROFILE_HIGH_10,
#endif
#ifdef NVENC_HAVE_422_SUPPORT
    NV_ENC_H264_PROFILE_HIGH_422,
#endif
    NV_ENC_H264_PROFILE_HIGH_444P,
};

enum {
    NV_ENC_HEVC_PROFILE_MAIN,
    NV_ENC_HEVC_PROFILE_MAIN_10,
    NV_ENC_HEVC_PROFILE_REXT,
};

enum {
    NVENC_LOWLATENCY = 1,
    NVENC_LOSSLESS   = 2,
    NVENC_ONE_PASS   = 4,
    NVENC_TWO_PASSES = 8,

    NVENC_DEPRECATED_PRESET = 0x8000,
};

enum {
    LIST_DEVICES = -2,
    ANY_DEVICE,
};

enum {
    NVENC_RGB_MODE_DISABLED,
    NVENC_RGB_MODE_420,
    NVENC_RGB_MODE_444,
};

typedef struct NvencContext
{
    AVClass *avclass;

    NvencDynLoadFunctions nvenc_dload_funcs;

    NV_ENC_INITIALIZE_PARAMS init_encode_params;
    NV_ENC_CONFIG encode_config;
    CUcontext cu_context;
    CUcontext cu_context_internal;
    CUstream cu_stream;
    ID3D11Device *d3d11_device;

    AVFrame *frame;

    int nb_surfaces;
    NvencSurface *surfaces;

    NvencFrameData *frame_data_array;
    int frame_data_array_nb;
    int frame_data_array_pos;

    AVFifo *unused_surface_queue;
    AVFifo *output_surface_queue;
    AVFifo *output_surface_ready_queue;
    AVFifo *timestamp_list;
    // This is for DTS calculating, reset after flush
    uint64_t output_frame_num;
    int64_t initial_delay_time;

    NV_ENC_SEI_PAYLOAD *sei_data;
    int sei_data_size;

    struct {
        void *ptr;
        int ptr_index;
        NV_ENC_REGISTERED_PTR regptr;
        int mapped;
        NV_ENC_MAP_INPUT_RESOURCE in_map;
    } registered_frames[MAX_REGISTERED_FRAMES];
    int nb_registered_frames;

    /* the actual data pixel format, different from
     * AVCodecContext.pix_fmt when using hwaccel frames on input */
    enum AVPixelFormat data_pix_fmt;

    int support_dyn_bitrate;

    void *nvencoder;

    uint32_t frame_idx_counter;

    int preset;
    int profile;
    int level;
    int tier;
    int rc;
    int cbr;
    int tile_rows;
    int tile_cols;
    int twopass;
    int device;
    int flags;
    int async_depth;
    int rc_lookahead;
    int aq;
    int no_scenecut;
    int forced_idr;
    int b_adapt;
    int temporal_aq;
    int zerolatency;
    int nonref_p;
    int strict_gop;
    int aq_strength;
    float quality;
    int aud;
    int bluray_compat;
    int qmin;
    int qmax;
    int init_qp_p;
    int init_qp_b;
    int init_qp_i;
    int cqp;
    int qp_cb_offset;
    int qp_cr_offset;
    int weighted_pred;
    int coder;
    int b_ref_mode;
    int a53_cc;
    int s12m_tc;
    int dpb_size;
    int tuning_info;
    int multipass;
    int ldkfs;
    int extra_sei;
    int intra_refresh;
    int single_slice_intra_refresh;
    int constrained_encoding;
    int udu_sei;
    int timing_info;
    int highbitdepth;
    int max_slice_size;
    int rgb_mode;
    int tf_level;
    int lookahead_level;
    int unidir_b;
    int split_encode_mode;
    int mdm, cll;
} NvencContext;

int ff_nvenc_encode_init(AVCodecContext *avctx);

int ff_nvenc_encode_close(AVCodecContext *avctx);

int ff_nvenc_receive_packet(AVCodecContext *avctx, AVPacket *pkt);

void ff_nvenc_encode_flush(AVCodecContext *avctx);

extern const enum AVPixelFormat ff_nvenc_pix_fmts[];
extern const AVCodecHWConfigInternal *const ff_nvenc_hw_configs[];

#endif /* AVCODEC_NVENC_H */
