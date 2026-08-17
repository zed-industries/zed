/*
 * Android MediaCodec Wrapper
 *
 * Copyright (c) 2015-2016 Matthieu Bouron <matthieu.bouron stupeflix.com>
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

#ifndef AVCODEC_MEDIACODEC_WRAPPER_H
#define AVCODEC_MEDIACODEC_WRAPPER_H

#include <stdint.h>
#include <sys/types.h>

#include "avcodec.h"
#include "mediacodec_surface.h"

/**
 * The following API around MediaCodec and MediaFormat is based on the
 * NDK one provided by Google since Android 5.0.
 *
 * Differences from the NDK API:
 *
 * Buffers returned by ff_AMediaFormat_toString and ff_AMediaFormat_getString
 * are newly allocated buffer and must be freed by the user after use.
 *
 * The MediaCrypto API is not implemented.
 *
 * ff_AMediaCodec_infoTryAgainLater, ff_AMediaCodec_infoOutputBuffersChanged,
 * ff_AMediaCodec_infoOutputFormatChanged, ff_AMediaCodec_cleanOutputBuffers
 * ff_AMediaCodec_getName and ff_AMediaCodec_getBufferFlagEndOfStream are not
 * part of the original NDK API and are convenience functions to hide JNI
 * implementation.
 *
 * The API around MediaCodecList is not part of the NDK (and is lacking as
 * we still need to retrieve the codec name to work around faulty decoders
 * and encoders).
 *
 * For documentation, please refers to NdkMediaCodec.h NdkMediaFormat.h and
 * http://developer.android.com/reference/android/media/MediaCodec.html.
 *
 */

int ff_AMediaCodecProfile_getProfileFromAVCodecContext(AVCodecContext *avctx);

char *ff_AMediaCodecList_getCodecNameByType(const char *mime, int profile, int encoder, void *log_ctx);

typedef struct FFAMediaFormat FFAMediaFormat;
struct FFAMediaFormat {
    const AVClass *class;

    FFAMediaFormat *(*create)(void);
    int (*delete)(FFAMediaFormat *);

    char* (*toString)(FFAMediaFormat* format);

    int (*getInt32)(FFAMediaFormat* format, const char *name, int32_t *out);
    int (*getInt64)(FFAMediaFormat* format, const char *name, int64_t *out);
    int (*getFloat)(FFAMediaFormat* format, const char *name, float *out);
    int (*getBuffer)(FFAMediaFormat* format, const char *name, void** data, size_t *size);
    int (*getString)(FFAMediaFormat* format, const char *name, const char **out);
    // NDK only, introduced in API level 28
    int (*getRect)(FFAMediaFormat *, const char *name,
                   int32_t *left, int32_t *top, int32_t *right, int32_t *bottom);

    void (*setInt32)(FFAMediaFormat* format, const char* name, int32_t value);
    void (*setInt64)(FFAMediaFormat* format, const char* name, int64_t value);
    void (*setFloat)(FFAMediaFormat* format, const char* name, float value);
    void (*setString)(FFAMediaFormat* format, const char* name, const char* value);
    void (*setBuffer)(FFAMediaFormat* format, const char* name, void* data, size_t size);
    // NDK only, introduced in API level 28
    void (*setRect)(FFAMediaFormat*, const char* name,
                    int32_t left, int32_t top, int32_t right, int32_t bottom);
};

FFAMediaFormat *ff_AMediaFormat_new(int ndk);

static inline int ff_AMediaFormat_delete(FFAMediaFormat* format)
{
    return format->delete(format);
}

static inline char* ff_AMediaFormat_toString(FFAMediaFormat* format)
{
    return format->toString(format);
}

static inline int ff_AMediaFormat_getInt32(FFAMediaFormat* format, const char *name, int32_t *out)
{
    return format->getInt32(format, name, out);
}

static inline int ff_AMediaFormat_getInt64(FFAMediaFormat* format, const char *name, int64_t *out)
{
    return format->getInt64(format, name, out);
}

static inline int ff_AMediaFormat_getFloat(FFAMediaFormat* format, const char *name, float *out)
{
    return format->getFloat(format, name, out);
}

static inline int ff_AMediaFormat_getBuffer(FFAMediaFormat* format, const char *name, void** data, size_t *size)
{
    return format->getBuffer(format, name, data, size);
}

static inline int ff_AMediaFormat_getString(FFAMediaFormat* format, const char *name, const char **out)
{
    return format->getString(format, name, out);
}

static inline int ff_AMediaFormat_getRect(FFAMediaFormat *format, const char *name,
                                          int32_t *left, int32_t *top, int32_t *right, int32_t *bottom)
{
    if (!format->getRect)
        return AVERROR_EXTERNAL;
    return format->getRect(format, name, left, top, right, bottom);
}

static inline void ff_AMediaFormat_setInt32(FFAMediaFormat* format, const char* name, int32_t value)
{
    format->setInt32(format, name, value);
}

static inline void ff_AMediaFormat_setInt64(FFAMediaFormat* format, const char* name, int64_t value)
{
    format->setInt64(format, name, value);
}

static inline void ff_AMediaFormat_setFloat(FFAMediaFormat* format, const char* name, float value)
{
    format->setFloat(format, name, value);
}

static inline void ff_AMediaFormat_setString(FFAMediaFormat* format, const char* name, const char* value)
{
    format->setString(format, name, value);
}

static inline void ff_AMediaFormat_setBuffer(FFAMediaFormat* format, const char* name, void* data, size_t size)
{
    format->setBuffer(format, name, data, size);
}

static inline void ff_AMediaFormat_setRect(FFAMediaFormat* format, const char* name,
                                           int32_t left, int32_t top, int32_t right, int32_t bottom)
{
    if (!format->setRect) {
        av_log(format, AV_LOG_WARNING, "Doesn't support setRect\n");
        return;
    }
    format->setRect(format, name, left, top, right, bottom);
}

typedef struct FFAMediaCodecCryptoInfo FFAMediaCodecCryptoInfo;

struct FFAMediaCodecBufferInfo {
    int32_t offset;
    int32_t size;
    int64_t presentationTimeUs;
    uint32_t flags;
};
typedef struct FFAMediaCodecBufferInfo FFAMediaCodecBufferInfo;

typedef struct FFAMediaCodec FFAMediaCodec;

typedef struct FFAMediaCodecOnAsyncNotifyCallback {
    void (*onAsyncInputAvailable)(FFAMediaCodec *codec, void *userdata,
                                  int32_t index);

    void (*onAsyncOutputAvailable)(FFAMediaCodec *codec, void *userdata,
                                   int32_t index,
                                   FFAMediaCodecBufferInfo *buffer_info);

    void (*onAsyncFormatChanged)(FFAMediaCodec *codec, void *userdata,
                                 FFAMediaFormat *format);

    void (*onAsyncError)(FFAMediaCodec *codec, void *userdata, int error,
                         const char *detail);
} FFAMediaCodecOnAsyncNotifyCallback;

struct FFAMediaCodec {
    const AVClass *class;

    char *(*getName)(FFAMediaCodec *codec);

    FFAMediaCodec* (*createCodecByName)(const char *name);
    FFAMediaCodec* (*createDecoderByType)(const char *mime_type);
    FFAMediaCodec* (*createEncoderByType)(const char *mime_type);
    int (*delete)(FFAMediaCodec* codec);

    int (*configure)(FFAMediaCodec* codec, const FFAMediaFormat* format, FFANativeWindow* surface, void *crypto, uint32_t flags);
    int (*start)(FFAMediaCodec* codec);
    int (*stop)(FFAMediaCodec* codec);
    int (*flush)(FFAMediaCodec* codec);

    uint8_t* (*getInputBuffer)(FFAMediaCodec* codec, size_t idx, size_t *out_size);
    uint8_t* (*getOutputBuffer)(FFAMediaCodec* codec, size_t idx, size_t *out_size);

    ssize_t (*dequeueInputBuffer)(FFAMediaCodec* codec, int64_t timeoutUs);
    int (*queueInputBuffer)(FFAMediaCodec* codec, size_t idx, off_t offset, size_t size, uint64_t time, uint32_t flags);

    ssize_t (*dequeueOutputBuffer)(FFAMediaCodec* codec, FFAMediaCodecBufferInfo *info, int64_t timeoutUs);
    FFAMediaFormat* (*getOutputFormat)(FFAMediaCodec* codec);

    int (*releaseOutputBuffer)(FFAMediaCodec* codec, size_t idx, int render);
    int (*releaseOutputBufferAtTime)(FFAMediaCodec *codec, size_t idx, int64_t timestampNs);

    int (*infoTryAgainLater)(FFAMediaCodec *codec, ssize_t idx);
    int (*infoOutputBuffersChanged)(FFAMediaCodec *codec, ssize_t idx);
    int (*infoOutputFormatChanged)(FFAMediaCodec *codec, ssize_t indx);

    int (*getBufferFlagCodecConfig)(FFAMediaCodec *codec);
    int (*getBufferFlagEndOfStream)(FFAMediaCodec *codec);
    int (*getBufferFlagKeyFrame)(FFAMediaCodec *codec);

    int (*getConfigureFlagEncode)(FFAMediaCodec *codec);

    int (*cleanOutputBuffers)(FFAMediaCodec *codec);

    // For encoder with FFANativeWindow as input.
    int (*signalEndOfInputStream)(FFAMediaCodec *);

    // Introduced in Android API 28
    int (*setAsyncNotifyCallback)(FFAMediaCodec *codec,
                                  const FFAMediaCodecOnAsyncNotifyCallback *callback,
                                  void *userdata);
};

static inline char *ff_AMediaCodec_getName(FFAMediaCodec *codec)
{
    return codec->getName(codec);
}

FFAMediaCodec* ff_AMediaCodec_createCodecByName(const char *name, int ndk);
FFAMediaCodec* ff_AMediaCodec_createDecoderByType(const char *mime_type, int ndk);
FFAMediaCodec* ff_AMediaCodec_createEncoderByType(const char *mime_type, int ndk);

static inline int ff_AMediaCodec_configure(FFAMediaCodec *codec,
                                           const FFAMediaFormat *format,
                                           FFANativeWindow *surface,
                                           void *crypto, uint32_t flags)
{
    return codec->configure(codec, format, surface, crypto, flags);
}

static inline int ff_AMediaCodec_start(FFAMediaCodec* codec)
{
    return codec->start(codec);
}

static inline int ff_AMediaCodec_stop(FFAMediaCodec* codec)
{
    return codec->stop(codec);
}

static inline int ff_AMediaCodec_flush(FFAMediaCodec* codec)
{
    return codec->flush(codec);
}

static inline int ff_AMediaCodec_delete(FFAMediaCodec* codec)
{
    return codec->delete(codec);
}

static inline uint8_t* ff_AMediaCodec_getInputBuffer(FFAMediaCodec* codec, size_t idx, size_t *out_size)
{
    return codec->getInputBuffer(codec, idx, out_size);
}

static inline uint8_t* ff_AMediaCodec_getOutputBuffer(FFAMediaCodec* codec, size_t idx, size_t *out_size)
{
    return codec->getOutputBuffer(codec, idx, out_size);
}

static inline ssize_t ff_AMediaCodec_dequeueInputBuffer(FFAMediaCodec* codec, int64_t timeoutUs)
{
    return codec->dequeueInputBuffer(codec, timeoutUs);
}

static inline int ff_AMediaCodec_queueInputBuffer(FFAMediaCodec *codec, size_t idx, off_t offset, size_t size, uint64_t time, uint32_t flags)
{
    return codec->queueInputBuffer(codec, idx, offset, size, time, flags);
}

static inline ssize_t ff_AMediaCodec_dequeueOutputBuffer(FFAMediaCodec* codec, FFAMediaCodecBufferInfo *info, int64_t timeoutUs)
{
    return codec->dequeueOutputBuffer(codec, info, timeoutUs);
}

static inline FFAMediaFormat* ff_AMediaCodec_getOutputFormat(FFAMediaCodec* codec)
{
    return codec->getOutputFormat(codec);
}

static inline int ff_AMediaCodec_releaseOutputBuffer(FFAMediaCodec* codec, size_t idx, int render)
{
    return codec->releaseOutputBuffer(codec, idx, render);
}

static inline int ff_AMediaCodec_releaseOutputBufferAtTime(FFAMediaCodec *codec, size_t idx, int64_t timestampNs)
{
    return codec->releaseOutputBufferAtTime(codec, idx, timestampNs);
}

static inline int ff_AMediaCodec_infoTryAgainLater(FFAMediaCodec *codec, ssize_t idx)
{
    return codec->infoTryAgainLater(codec, idx);
}

static inline int ff_AMediaCodec_infoOutputBuffersChanged(FFAMediaCodec *codec, ssize_t idx)
{
    return codec->infoOutputBuffersChanged(codec, idx);
}

static inline int ff_AMediaCodec_infoOutputFormatChanged(FFAMediaCodec *codec, ssize_t idx)
{
    return codec->infoOutputFormatChanged(codec, idx);
}

static inline int ff_AMediaCodec_getBufferFlagCodecConfig(FFAMediaCodec *codec)
{
    return codec->getBufferFlagCodecConfig(codec);
}

static inline int ff_AMediaCodec_getBufferFlagEndOfStream(FFAMediaCodec *codec)
{
    return codec->getBufferFlagEndOfStream(codec);
}

static inline int ff_AMediaCodec_getBufferFlagKeyFrame(FFAMediaCodec *codec)
{
    return codec->getBufferFlagKeyFrame(codec);
}

static inline int ff_AMediaCodec_getConfigureFlagEncode(FFAMediaCodec *codec)
{
    return codec->getConfigureFlagEncode(codec);
}

static inline int ff_AMediaCodec_cleanOutputBuffers(FFAMediaCodec *codec)
{
    return codec->cleanOutputBuffers(codec);
}

static inline int ff_AMediaCodec_signalEndOfInputStream(FFAMediaCodec *codec)
{
    return codec->signalEndOfInputStream(codec);
}

static inline int ff_AMediaCodec_setAsyncNotifyCallback(FFAMediaCodec *codec,
        const FFAMediaCodecOnAsyncNotifyCallback *callback,
        void *userdata)
{
    return codec->setAsyncNotifyCallback(codec, callback, userdata);
}

int ff_Build_SDK_INT(AVCodecContext *avctx);

enum FFAMediaFormatColorRange {
    COLOR_RANGE_UNSPECIFIED = 0x0,
    COLOR_RANGE_FULL        = 0x1,
    COLOR_RANGE_LIMITED     = 0x2,
};

enum FFAMediaFormatColorStandard {
    COLOR_STANDARD_UNSPECIFIED  = 0x0,
    COLOR_STANDARD_BT709        = 0x1,
    COLOR_STANDARD_BT601_PAL    = 0x2,
    COLOR_STANDARD_BT601_NTSC   = 0x4,
    COLOR_STANDARD_BT2020       = 0x6,
};

enum FFAMediaFormatColorTransfer {
    COLOR_TRANSFER_UNSPECIFIED = 0x0,
    COLOR_TRANSFER_LINEAR      = 0x1,
    COLOR_TRANSFER_SDR_VIDEO   = 0x3,
    COLOR_TRANSFER_ST2084      = 0x6,
    COLOR_TRANSFER_HLG         = 0x7,
};

/**
 * Map MediaFormat color range to AVColorRange.
 *
 * return AVCOL_RANGE_UNSPECIFIED when failed.
 */
enum AVColorRange ff_AMediaFormatColorRange_to_AVColorRange(int color_range);

/**
 * Map AVColorRange to MediaFormat color range.
 *
 * return COLOR_RANGE_UNSPECIFIED when failed.
 */
int ff_AMediaFormatColorRange_from_AVColorRange(enum AVColorRange color_range);

/**
 * Map MediaFormat color standard to AVColorSpace.
 *
 * return AVCOL_SPC_UNSPECIFIED when failed.
 */
enum AVColorSpace ff_AMediaFormatColorStandard_to_AVColorSpace(int color_standard);

/**
 * Map AVColorSpace to MediaFormat color standard.
 *
 * return COLOR_STANDARD_UNSPECIFIED when failed.
 */
int ff_AMediaFormatColorStandard_from_AVColorSpace(enum AVColorSpace color_space);

/**
 * Map MediaFormat color standard to AVColorPrimaries.
 *
 * return AVCOL_PRI_UNSPECIFIED when failed.
 */
enum AVColorPrimaries ff_AMediaFormatColorStandard_to_AVColorPrimaries(int color_standard);

/**
 * Map MediaFormat color transfer to AVColorTransferCharacteristic.
 *
 * return AVCOL_TRC_UNSPECIFIED when failed.
 */
enum AVColorTransferCharacteristic
ff_AMediaFormatColorTransfer_to_AVColorTransfer(int color_transfer);

/**
 * Map AVColorTransferCharacteristic to MediaFormat color transfer.
 *
 * return COLOR_TRANSFER_UNSPECIFIED when failed.
 */
int ff_AMediaFormatColorTransfer_from_AVColorTransfer(
    enum AVColorTransferCharacteristic color_transfer);

#endif /* AVCODEC_MEDIACODEC_WRAPPER_H */
