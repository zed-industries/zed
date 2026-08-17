/*
 * Intel MediaSDK QSV encoder utility functions
 *
 * copyright (c) 2013 Yukinori Yamazoe
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

#ifndef AVCODEC_QSVENC_H
#define AVCODEC_QSVENC_H

#include <stdint.h>
#include <sys/types.h>

#include "libavutil/common.h"
#include "libavutil/hwcontext.h"
#include "libavutil/hwcontext_qsv.h"
#include "libavutil/avutil.h"
#include "libavutil/fifo.h"

#include "avcodec.h"
#include "hwconfig.h"
#include "qsv_internal.h"

#define QSV_HAVE_EXT_VP9_TILES QSV_VERSION_ATLEAST(1, 29)
#define QSV_HAVE_EXT_AV1_PARAM QSV_VERSION_ATLEAST(2, 5)

#if defined(_WIN32) || defined(__CYGWIN__)
#define QSV_HAVE_AVBR   1
#define QSV_HAVE_VCM    1
#define QSV_HAVE_MF     0
#define QSV_HAVE_HE     QSV_VERSION_ATLEAST(2, 4)
#else
#define QSV_HAVE_AVBR   0
#define QSV_HAVE_VCM    0
#define QSV_HAVE_MF     !QSV_ONEVPL
#define QSV_HAVE_HE     0
#endif

#define QSV_COMMON_OPTS \
{ "async_depth", "Maximum processing parallelism", OFFSET(qsv.async_depth), AV_OPT_TYPE_INT, { .i64 = ASYNC_DEPTH_DEFAULT }, 1, INT_MAX, VE },                          \
{ "preset", NULL, OFFSET(qsv.preset), AV_OPT_TYPE_INT, { .i64 = MFX_TARGETUSAGE_UNKNOWN }, MFX_TARGETUSAGE_UNKNOWN, MFX_TARGETUSAGE_BEST_SPEED,   VE, .unit = "preset" }, \
{ "veryfast",    NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_TARGETUSAGE_BEST_SPEED  },   INT_MIN, INT_MAX, VE, .unit = "preset" },                                          \
{ "faster",      NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_TARGETUSAGE_6  },            INT_MIN, INT_MAX, VE, .unit = "preset" },                                          \
{ "fast",        NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_TARGETUSAGE_5  },            INT_MIN, INT_MAX, VE, .unit = "preset" },                                          \
{ "medium",      NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_TARGETUSAGE_BALANCED  },     INT_MIN, INT_MAX, VE, .unit = "preset" },                                          \
{ "slow",        NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_TARGETUSAGE_3  },            INT_MIN, INT_MAX, VE, .unit = "preset" },                                          \
{ "slower",      NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_TARGETUSAGE_2  },            INT_MIN, INT_MAX, VE, .unit = "preset" },                                          \
{ "veryslow",    NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_TARGETUSAGE_BEST_QUALITY  }, INT_MIN, INT_MAX, VE, .unit = "preset" },                                          \
{ "forced_idr",     "Forcing I frames as IDR frames",         OFFSET(qsv.forced_idr),     AV_OPT_TYPE_BOOL,{ .i64 = 0  },  0,          1, VE },                         \
{ "low_power", "enable low power mode(experimental: many limitations by mfx version, BRC modes, etc.)", OFFSET(qsv.low_power), AV_OPT_TYPE_BOOL, { .i64 = -1}, -1, 1, VE},\
{ "qsv_params", "Set QSV encoder parameters as key1=value1:key2=value2:...", OFFSET(qsv.qsv_params), AV_OPT_TYPE_DICT, { 0 }, 0, 0, VE },

#if QSV_HAVE_HE
#define QSV_HE_OPTIONS \
{ "dual_gfx", "Prefer processing on both iGfx and dGfx simultaneously",                                             OFFSET(qsv.dual_gfx), AV_OPT_TYPE_INT, { .i64 = MFX_HYPERMODE_OFF }, MFX_HYPERMODE_OFF, MFX_HYPERMODE_ADAPTIVE, VE, .unit = "dual_gfx" }, \
{ "off",      "Disable HyperEncode mode",                                                                           0, AV_OPT_TYPE_CONST, { .i64 = MFX_HYPERMODE_OFF       },   INT_MIN, INT_MAX, VE, .unit = "dual_gfx" }, \
{ "on",       "Enable HyperEncode mode and return error if incompatible parameters during initialization",          0, AV_OPT_TYPE_CONST, { .i64 = MFX_HYPERMODE_ON        },   INT_MIN, INT_MAX, VE, .unit = "dual_gfx" }, \
{ "adaptive", "Enable HyperEncode mode or fallback to single GPU if incompatible parameters during initialization", 0, AV_OPT_TYPE_CONST, { .i64 = MFX_HYPERMODE_ADAPTIVE  },   INT_MIN, INT_MAX, VE, .unit = "dual_gfx" },
#endif

#define QSV_OPTION_RDO \
{ "rdo",            "Enable rate distortion optimization",    OFFSET(qsv.rdo),            AV_OPT_TYPE_INT, { .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_MAX_FRAME_SIZE \
{ "max_frame_size", "Maximum encoded frame size in bytes",    OFFSET(qsv.max_frame_size), AV_OPT_TYPE_INT, { .i64 = -1 }, -1,    INT_MAX, VE },                         \
{ "max_frame_size_i", "Maximum encoded I frame size in bytes",OFFSET(qsv.max_frame_size_i), AV_OPT_TYPE_INT, { .i64 = -1 }, -1,  INT_MAX, VE },                         \
{ "max_frame_size_p", "Maximum encoded P frame size in bytes",OFFSET(qsv.max_frame_size_p), AV_OPT_TYPE_INT, { .i64 = -1 }, -1,  INT_MAX, VE },

#define QSV_OPTION_MAX_SLICE_SIZE \
{ "max_slice_size", "Maximum encoded slice size in bytes",    OFFSET(qsv.max_slice_size), AV_OPT_TYPE_INT, { .i64 = -1 }, -1,    INT_MAX, VE },

#define QSV_OPTION_BITRATE_LIMIT \
{ "bitrate_limit",  "Toggle bitrate limitations",             OFFSET(qsv.bitrate_limit),  AV_OPT_TYPE_INT, { .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_MBBRC \
{ "mbbrc",          "MB level bitrate control",               OFFSET(qsv.mbbrc),          AV_OPT_TYPE_INT, { .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_EXTBRC \
{ "extbrc",         "Extended bitrate control",               OFFSET(qsv.extbrc),         AV_OPT_TYPE_INT, { .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_ADAPTIVE_I \
{ "adaptive_i",     "Adaptive I-frame placement",             OFFSET(qsv.adaptive_i),     AV_OPT_TYPE_INT, { .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_ADAPTIVE_B \
{ "adaptive_b",     "Adaptive B-frame placement",             OFFSET(qsv.adaptive_b),     AV_OPT_TYPE_INT, { .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_P_STRATEGY \
{ "p_strategy",     "Enable P-pyramid: 0-default 1-simple 2-pyramid(bf need to be set to 0).",    OFFSET(qsv.p_strategy), AV_OPT_TYPE_INT,    { .i64 = 0}, 0,    2, VE },

#define QSV_OPTION_B_STRATEGY \
{ "b_strategy",     "Strategy to choose between I/P/B-frames", OFFSET(qsv.b_strategy),    AV_OPT_TYPE_INT, { .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_DBLK_IDC \
{ "dblk_idc", "This option disable deblocking. It has value in range 0~2.",   OFFSET(qsv.dblk_idc),   AV_OPT_TYPE_INT,    { .i64 = 0 },   0,  2,  VE},

#define QSV_OPTION_LOW_DELAY_BRC \
{ "low_delay_brc",   "Allow to strictly obey avg frame size", OFFSET(qsv.low_delay_brc),  AV_OPT_TYPE_BOOL,{ .i64 = -1 }, -1,          1, VE },

#define QSV_OPTION_MAX_MIN_QP \
{ "max_qp_i", "Maximum video quantizer scale for I frame",       OFFSET(qsv.max_qp_i),       AV_OPT_TYPE_INT, { .i64 = -1 },  -1,          51, VE},                         \
{ "min_qp_i", "Minimum video quantizer scale for I frame",       OFFSET(qsv.min_qp_i),       AV_OPT_TYPE_INT, { .i64 = -1 },  -1,          51, VE},                         \
{ "max_qp_p", "Maximum video quantizer scale for P frame",       OFFSET(qsv.max_qp_p),       AV_OPT_TYPE_INT, { .i64 = -1 },  -1,          51, VE},                         \
{ "min_qp_p", "Minimum video quantizer scale for P frame",       OFFSET(qsv.min_qp_p),       AV_OPT_TYPE_INT, { .i64 = -1 },  -1,          51, VE},                         \
{ "max_qp_b", "Maximum video quantizer scale for B frame",       OFFSET(qsv.max_qp_b),       AV_OPT_TYPE_INT, { .i64 = -1 },  -1,          51, VE},                         \
{ "min_qp_b", "Minimum video quantizer scale for B frame",       OFFSET(qsv.min_qp_b),       AV_OPT_TYPE_INT, { .i64 = -1 },  -1,          51, VE},

#define QSV_OPTION_SCENARIO \
{ "scenario", "A hint to encoder about the scenario for the encoding session", OFFSET(qsv.scenario), AV_OPT_TYPE_INT, { .i64 = MFX_SCENARIO_UNKNOWN },          \
  MFX_SCENARIO_UNKNOWN, MFX_SCENARIO_REMOTE_GAMING, VE, .unit = "scenario" }, \
{ "unknown",            NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_UNKNOWN },            .flags = VE, .unit = "scenario" },                                      \
{ "displayremoting",    NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_DISPLAY_REMOTING },   .flags = VE, .unit = "scenario" },                                      \
{ "videoconference",    NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_VIDEO_CONFERENCE },   .flags = VE, .unit = "scenario" },                                      \
{ "archive",            NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_ARCHIVE },            .flags = VE, .unit = "scenario" },                                      \
{ "livestreaming",      NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_LIVE_STREAMING },     .flags = VE, .unit = "scenario" },                                      \
{ "cameracapture",      NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_CAMERA_CAPTURE },     .flags = VE, .unit = "scenario" },                                      \
{ "videosurveillance",  NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_VIDEO_SURVEILLANCE }, .flags = VE, .unit = "scenario" },                                      \
{ "gamestreaming",      NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_GAME_STREAMING },     .flags = VE, .unit = "scenario" },                                      \
{ "remotegaming",       NULL, 0, AV_OPT_TYPE_CONST, { .i64 = MFX_SCENARIO_REMOTE_GAMING },      .flags = VE, .unit = "scenario" },

#define QSV_OPTION_AVBR \
{ "avbr_accuracy",    "Accuracy of the AVBR ratecontrol (unit of tenth of percent)",    OFFSET(qsv.avbr_accuracy),    AV_OPT_TYPE_INT, { .i64 = 0 }, 0, UINT16_MAX, VE }, \
{ "avbr_convergence", "Convergence of the AVBR ratecontrol (unit of 100 frames)", OFFSET(qsv.avbr_convergence), AV_OPT_TYPE_INT, { .i64 = 0 }, 0, UINT16_MAX, VE },

#define QSV_OPTION_SKIP_FRAME \
{ "skip_frame",     "Allow frame skipping", OFFSET(qsv.skip_frame),  AV_OPT_TYPE_INT, { .i64 = MFX_SKIPFRAME_NO_SKIP }, \
   MFX_SKIPFRAME_NO_SKIP, MFX_SKIPFRAME_BRC_ONLY, VE, .unit = "skip_frame" }, \
{ "no_skip",        "Frame skipping is disabled", \
    0, AV_OPT_TYPE_CONST, { .i64 = MFX_SKIPFRAME_NO_SKIP },           .flags = VE, .unit = "skip_frame" },        \
{ "insert_dummy",   "Encoder inserts into bitstream frame where all macroblocks are encoded as skipped",  \
    0, AV_OPT_TYPE_CONST, { .i64 = MFX_SKIPFRAME_INSERT_DUMMY },      .flags = VE, .unit = "skip_frame" },        \
{ "insert_nothing", "Encoder inserts nothing into bitstream",                                             \
    0, AV_OPT_TYPE_CONST, { .i64 = MFX_SKIPFRAME_INSERT_NOTHING },    .flags = VE, .unit = "skip_frame" },        \
{ "brc_only",       "skip_frame metadata indicates the number of missed frames before the current frame", \
    0, AV_OPT_TYPE_CONST, { .i64 = MFX_SKIPFRAME_BRC_ONLY },          .flags = VE, .unit = "skip_frame" },

extern const AVCodecHWConfigInternal *const ff_qsv_enc_hw_configs[];

typedef int SetEncodeCtrlCB (AVCodecContext *avctx,
                             const AVFrame *frame, mfxEncodeCtrl* enc_ctrl);
typedef struct QSVEncContext {
    AVCodecContext *avctx;

    QSVFrame *work_frames;

    mfxSession session;
    QSVSession internal_qs;

    int packet_size;
    int width_align;
    int height_align;

    mfxVideoParam param;
    mfxFrameAllocRequest req;

    mfxExtCodingOption  extco;
    mfxExtCodingOption2 extco2;
    mfxExtCodingOption3 extco3;
#if QSV_HAVE_MF
    mfxExtMultiFrameParam   extmfp;
    mfxExtMultiFrameControl extmfc;
#endif
    mfxExtHEVCTiles exthevctiles;
    mfxExtVP9Param  extvp9param;
#if QSV_HAVE_EXT_AV1_PARAM
    mfxExtAV1TileParam extav1tileparam;
    mfxExtAV1BitstreamParam extav1bsparam;
#endif
#if QSV_HAVE_HE
    mfxExtHyperModeParam exthypermodeparam;
#endif
#if QSV_HAVE_OPAQUE
    mfxExtOpaqueSurfaceAlloc opaque_alloc;
    mfxFrameSurface1       **opaque_surfaces;
    AVBufferRef             *opaque_alloc_buf;
#endif

    mfxExtVideoSignalInfo extvsi;

    mfxExtBuffer  *extparam_internal[5 + (QSV_HAVE_MF * 2) + (QSV_HAVE_EXT_AV1_PARAM * 2) + QSV_HAVE_HE];
    int         nb_extparam_internal;

    mfxExtBuffer  **extparam_str;
    int         nb_extparam_str;

    mfxExtBuffer **extparam;
    int         nb_extparam;

    AVFifo *async_fifo;

    QSVFramesContext frames_ctx;

    mfxVersion          ver;

    int hevc_vps;

    // options set by the caller
    int async_depth;
    int idr_interval;
    int profile;
    int tier;
    int preset;
    int avbr_accuracy;
    int avbr_convergence;
    int pic_timing_sei;
    int look_ahead;
    int look_ahead_depth;
    int look_ahead_downsampling;
    int vcm;
    int rdo;
    int max_frame_size;
    int max_frame_size_i;
    int max_frame_size_p;
    int max_slice_size;
    int dblk_idc;
    int scenario;

    int tile_cols;
    int tile_rows;

    int aud;

    int single_sei_nal_unit;
    int max_dec_frame_buffering;

    int bitrate_limit;
    int mbbrc;
    int extbrc;
    int adaptive_i;
    int adaptive_b;
    int b_strategy;
    int p_strategy;
    int cavlc;

    int int_ref_type;
    int int_ref_cycle_size;
    int int_ref_qp_delta;
    int int_ref_cycle_dist;
    int recovery_point_sei;

    int repeat_pps;
    int low_power;
    int gpb;
    int transform_skip;

    int a53_cc;

#if QSV_HAVE_MF
    int mfmode;
#endif
    char *load_plugins;
    SetEncodeCtrlCB *set_encode_ctrl_cb;
    int forced_idr;
    int low_delay_brc;

    int co2_idx;
    int co3_idx;
    int exthevctiles_idx;
    int exthypermodeparam_idx;
    int vp9_idx;

    int max_qp_i;
    int min_qp_i;
    int max_qp_p;
    int min_qp_p;
    int max_qp_b;
    int min_qp_b;
    // These are used for qp reset
    int old_global_quality;
    float old_i_quant_factor;
    float old_i_quant_offset;
    float old_b_quant_factor;
    float old_b_quant_offset;
    // This is used for max_frame_size reset
    int old_max_frame_size;
    // This is used for gop reset
    int old_gop_size;
    // These are used for intra refresh reset
    int old_int_ref_type;
    int old_int_ref_cycle_size;
    int old_int_ref_qp_delta;
    int old_int_ref_cycle_dist;
    // These are used for max/min qp reset;
    int old_qmax;
    int old_qmin;
    int old_max_qp_i;
    int old_min_qp_i;
    int old_max_qp_p;
    int old_min_qp_p;
    int old_max_qp_b;
    int old_min_qp_b;
    // This is used for low_delay_brc reset
    int old_low_delay_brc;
    // This is used for framerate reset
    AVRational old_framerate;
    // These are used for bitrate control reset
    int old_bit_rate;
    int old_rc_buffer_size;
    int old_rc_initial_buffer_occupancy;
    int old_rc_max_rate;
    // This is used for SEI Timing reset
    int old_pic_timing_sei;
    int skip_frame;
    // This is used for Hyper Encode
    int dual_gfx;

    AVDictionary *qsv_params;
} QSVEncContext;

int ff_qsv_enc_init(AVCodecContext *avctx, QSVEncContext *q);

int ff_qsv_encode(AVCodecContext *avctx, QSVEncContext *q,
                  AVPacket *pkt, const AVFrame *frame, int *got_packet);

int ff_qsv_enc_close(AVCodecContext *avctx, QSVEncContext *q);

#endif /* AVCODEC_QSVENC_H */
