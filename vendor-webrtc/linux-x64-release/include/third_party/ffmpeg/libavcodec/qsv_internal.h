/*
 * Intel MediaSDK QSV encoder/decoder shared code
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

#ifndef AVCODEC_QSV_INTERNAL_H
#define AVCODEC_QSV_INTERNAL_H

#include "config.h"

#if CONFIG_VAAPI && !defined(_WIN32) // Do not enable for libva-win32 on Windows
#define AVCODEC_QSV_LINUX_SESSION_HANDLE
#endif //CONFIG_VAAPI && !defined(_WIN32)

#ifdef AVCODEC_QSV_LINUX_SESSION_HANDLE
#include <stdio.h>
#include <string.h>
#if HAVE_UNISTD_H
#include <unistd.h>
#endif
#include <fcntl.h>
#include <va/va.h>
#include "libavutil/hwcontext_vaapi.h"
#endif

#include <mfxvideo.h>

#include "libavutil/frame.h"

#include "avcodec.h"

#define QSV_VERSION_MAJOR 1
#define QSV_VERSION_MINOR 1

#define ASYNC_DEPTH_DEFAULT 4       // internal parallelism

#define QSV_MAX_ENC_PAYLOAD 2       // # of mfxEncodeCtrl payloads supported
#define QSV_MAX_ENC_EXTPARAM 8      // # of mfxEncodeCtrl extparam supported

#define QSV_MAX_ROI_NUM 256

#define QSV_MAX_FRAME_EXT_PARAMS 4

#define QSV_VERSION_ATLEAST(MAJOR, MINOR)   \
    (MFX_VERSION_MAJOR > (MAJOR) ||         \
     MFX_VERSION_MAJOR == (MAJOR) && MFX_VERSION_MINOR >= (MINOR))

#define QSV_RUNTIME_VERSION_ATLEAST(MFX_VERSION, MAJOR, MINOR) \
    ((MFX_VERSION.Major > (MAJOR)) ||                           \
    (MFX_VERSION.Major == (MAJOR) && MFX_VERSION.Minor >= (MINOR)))

#define QSV_ONEVPL       QSV_VERSION_ATLEAST(2, 0)
#define QSV_HAVE_OPAQUE  !QSV_ONEVPL

typedef struct QSVMid {
    AVBufferRef *hw_frames_ref;
    mfxHDLPair *handle_pair;

    AVFrame *locked_frame;
    AVFrame *hw_frame;
    mfxFrameSurface1 surf;
} QSVMid;

typedef struct QSVFrame {
    AVFrame *frame;
    mfxFrameSurface1 surface;
    mfxEncodeCtrl enc_ctrl;
    mfxExtDecodedFrameInfo dec_info;
#if QSV_VERSION_ATLEAST(1, 34)
    mfxExtAV1FilmGrainParam av1_film_grain_param;
#endif

#if QSV_VERSION_ATLEAST(1, 35)
    mfxExtMasteringDisplayColourVolume mdcv;
    mfxExtContentLightLevelInfo clli;
#endif

    mfxExtBuffer *ext_param[QSV_MAX_FRAME_EXT_PARAMS];
    int num_ext_params;

    mfxPayload *payloads[QSV_MAX_ENC_PAYLOAD]; ///< used for enc_ctrl.Payload
    mfxExtBuffer *extparam[QSV_MAX_ENC_EXTPARAM]; ///< used for enc_ctrl.ExtParam

    int queued;
    int used;

    struct QSVFrame *next;
} QSVFrame;

typedef struct QSVSession {
    mfxSession session;
#ifdef AVCODEC_QSV_LINUX_SESSION_HANDLE
    AVBufferRef *va_device_ref;
    AVHWDeviceContext *va_device_ctx;
#endif
    void *loader;
} QSVSession;

typedef struct QSVFramesContext {
    AVBufferRef *hw_frames_ctx;
    void *logctx;

    /**
     * The memory ids for the external frames.
     * Refcounted (via the RefStruct API), since we need one reference
     * owned by the QSVFramesContext (i.e. by the encoder/decoder) and
     * another one given to the MFX session from the frame allocator.
     */
    QSVMid *mids;
    int  nb_mids;
} QSVFramesContext;

int ff_qsv_print_iopattern(void *log_ctx, int mfx_iopattern,
                           const char *extra_string);

int ff_qsv_print_error(void *log_ctx, mfxStatus err,
                       const char *error_string);

int ff_qsv_print_warning(void *log_ctx, mfxStatus err,
                         const char *warning_string);

int ff_qsv_codec_id_to_mfx(enum AVCodecID codec_id);

enum AVPixelFormat ff_qsv_map_fourcc(uint32_t fourcc);

int ff_qsv_map_pixfmt(enum AVPixelFormat format, uint32_t *fourcc, uint16_t *shift);
enum AVPictureType ff_qsv_map_pictype(int mfx_pic_type);

enum AVFieldOrder ff_qsv_map_picstruct(int mfx_pic_struct);

int ff_qsv_init_internal_session(AVCodecContext *avctx, QSVSession *qs,
                                 const char *load_plugins, int gpu_copy);

int ff_qsv_close_internal_session(QSVSession *qs);

int ff_qsv_init_session_device(AVCodecContext *avctx, mfxSession *psession,
                               AVBufferRef *device_ref, const char *load_plugins,
                               int gpu_copy);

int ff_qsv_init_session_frames(AVCodecContext *avctx, mfxSession *session,
                               QSVFramesContext *qsv_frames_ctx,
                               const char *load_plugins, int opaque, int gpu_copy);

int ff_qsv_find_surface_idx(QSVFramesContext *ctx, QSVFrame *frame);

void ff_qsv_frame_add_ext_param(AVCodecContext *avctx, QSVFrame *frame,
                                mfxExtBuffer *param);

int ff_qsv_map_frame_to_surface(const AVFrame *frame, mfxFrameSurface1 *surface);


#endif /* AVCODEC_QSV_INTERNAL_H */
