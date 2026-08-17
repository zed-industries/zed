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

#ifndef AVCODEC_AMFENC_H
#define AVCODEC_AMFENC_H

#include <AMF/core/Factory.h>

#include <AMF/components/ColorSpace.h>
#include <AMF/components/VideoEncoderVCE.h>
#include <AMF/components/VideoEncoderHEVC.h>
#include <AMF/components/VideoEncoderAV1.h>

#include "libavutil/fifo.h"

#include "avcodec.h"
#include "hwconfig.h"

#define  MAX_LOOKAHEAD_DEPTH 41

/**
* AMF encoder context
*/

typedef struct AMFEncoderContext {
    AVClass            *avclass;
    // access to AMF runtime
    AVBufferRef        *device_ctx_ref;

    //encoder
    AMFComponent       *encoder; ///< AMF encoder object
    amf_bool            eof;     ///< flag indicating EOF happened
    AMF_SURFACE_FORMAT  format;  ///< AMF surface format

    int                 hwsurfaces_in_queue;
    int                 hwsurfaces_in_queue_max;
    int                 query_timeout_supported;

    // helpers to handle async calls
    int                 delayed_drain;

    // shift dts back by max_b_frames in timing
    AVFifo             *timestamp_list;
    int64_t             dts_delay;
    int64_t             submitted_frame;
    int64_t             encoded_frame;

    // common encoder options

    // Static options, have to be set before Init() call
    int                 usage;
    int                 profile;
    int                 level;
    int                 latency;
    int                 preencode;
    int                 quality;
    int                 b_frame_delta_qp;
    int                 ref_b_frame_delta_qp;
    int                 bit_depth;

    // Dynamic options, can be set after Init() call

    int                 rate_control_mode;
    int                 enforce_hrd;
    int                 filler_data;
    int                 enable_vbaq;
    int                 skip_frame;
    int                 qp_i;
    int                 qp_p;
    int                 qp_b;
    int                 max_au_size;
    int                 header_spacing;
    int                 b_frame_ref;
    int                 intra_refresh_mb;
    int                 coding_mode;
    int                 me_half_pel;
    int                 me_quarter_pel;
    int                 aud;
    int                 max_consecutive_b_frames;
    int                 max_b_frames;
    int                 qvbr_quality_level;
    int                 hw_high_motion_quality_boost;
    int                 forced_idr;

    // HEVC - specific options

    int                 gops_per_idr;
    int                 header_insertion_mode;
    int                 min_qp_i;
    int                 max_qp_i;
    int                 min_qp_p;
    int                 max_qp_p;
    int                 tier;

    // AV1 - specific options

    enum AMF_VIDEO_ENCODER_AV1_ALIGNMENT_MODE_ENUM                 align;
    enum AMF_VIDEO_ENCODER_AV1_AQ_MODE_ENUM                        aq_mode;

    // Preanalysis - specific options

    int                 preanalysis;
    int                 pa_activity_type;
    int                 pa_scene_change_detection;
    int                 pa_scene_change_detection_sensitivity;
    int                 pa_static_scene_detection;
    int                 pa_static_scene_detection_sensitivity;
    int                 pa_initial_qp;
    int                 pa_max_qp;
    int                 pa_caq_strength;
    int                 pa_frame_sad;
    int                 pa_ltr;
    int                 pa_lookahead_buffer_depth;
    int                 pa_paq_mode;
    int                 pa_taq_mode;
    int                 pa_high_motion_quality_boost_mode;
    int                 pa_adaptive_mini_gop;


} AMFEncoderContext;

extern const AVCodecHWConfigInternal *const ff_amfenc_hw_configs[];

/**
* Common encoder initization function
*/
int ff_amf_encode_init(AVCodecContext *avctx);
/**
* Common encoder termination function
*/
int ff_amf_encode_close(AVCodecContext *avctx);

/**
* Ecoding one frame - common function for all AMF encoders
*/
int ff_amf_receive_packet(AVCodecContext *avctx, AVPacket *avpkt);

/**
* Supported formats
*/
extern const enum AVPixelFormat ff_amf_pix_fmts[];

int ff_amf_get_color_profile(AVCodecContext *avctx);

/**
* Error handling helper
*/
#define AMF_RETURN_IF_FALSE(avctx, exp, ret_value, /*message,*/ ...) \
    if (!(exp)) { \
        av_log(avctx, AV_LOG_ERROR, __VA_ARGS__); \
        return ret_value; \
    }

#endif //AVCODEC_AMFENC_H
