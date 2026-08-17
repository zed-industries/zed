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
 * Header providing the internals of AVHWAccel.
 */

#ifndef AVCODEC_HWACCEL_INTERNAL_H
#define AVCODEC_HWACCEL_INTERNAL_H

#include <stdint.h>

#include "avcodec.h"
#include "libavutil/refstruct.h"

#define HWACCEL_CAP_ASYNC_SAFE      (1 << 0)
#define HWACCEL_CAP_THREAD_SAFE     (1 << 1)

typedef struct FFHWAccel {
    /**
     * The public AVHWAccel. See avcodec.h for it.
     */
    AVHWAccel p;

    /**
     * Allocate a custom buffer
     */
    int (*alloc_frame)(AVCodecContext *avctx, AVFrame *frame);

    /**
     * Called at the beginning of each frame or field picture.
     *
     * Meaningful frame information (codec specific) is guaranteed to
     * be parsed at this point. This function is mandatory.
     *
     * Note that buf can be NULL along with buf_size set to 0.
     * Otherwise, this means the whole frame is available at this point.
     *
     * @param avctx the codec context
     * @param buf_ref the frame data buffer reference (optional)
     * @param buf the frame data buffer base
     * @param buf_size the size of the frame in bytes
     * @return zero if successful, a negative value otherwise
     */
    int (*start_frame)(AVCodecContext *avctx, const AVBufferRef *buf_ref,
                       const uint8_t *buf, uint32_t buf_size);

    /**
     * Callback for parameter data (SPS/PPS/VPS etc).
     *
     * Useful for hardware decoders which keep persistent state about the
     * video parameters, and need to receive any changes to update that state.
     *
     * @param avctx the codec context
     * @param type the nal unit type
     * @param buf the nal unit data buffer
     * @param buf_size the size of the nal unit in bytes
     * @return zero if successful, a negative value otherwise
     */
    int (*decode_params)(AVCodecContext *avctx, int type, const uint8_t *buf, uint32_t buf_size);

    /**
     * Callback for each slice.
     *
     * Meaningful slice information (codec specific) is guaranteed to
     * be parsed at this point. This function is mandatory.
     *
     * @param avctx the codec context
     * @param buf the slice data buffer base
     * @param buf_size the size of the slice in bytes
     * @return zero if successful, a negative value otherwise
     */
    int (*decode_slice)(AVCodecContext *avctx, const uint8_t *buf, uint32_t buf_size);

    /**
     * Called at the end of each frame or field picture.
     *
     * The whole picture is parsed at this point and can now be sent
     * to the hardware accelerator. This function is mandatory.
     *
     * @param avctx the codec context
     * @return zero if successful, a negative value otherwise
     */
    int (*end_frame)(AVCodecContext *avctx);

    /**
     * Size of per-frame hardware accelerator private data.
     *
     * Private data is allocated with av_mallocz() before
     * AVCodecContext.get_buffer() and deallocated after
     * AVCodecContext.release_buffer().
     */
    int frame_priv_data_size;

    /**
     * Size of the private data to allocate in
     * AVCodecInternal.hwaccel_priv_data.
     */
    int priv_data_size;

    /**
     * Internal hwaccel capabilities.
     */
    int caps_internal;

    /**
     * Initialize the hwaccel private data.
     *
     * This will be called from ff_get_format(), after hwaccel and
     * hwaccel_context are set and the hwaccel private data in AVCodecInternal
     * is allocated.
     */
    int (*init)(AVCodecContext *avctx);

    /**
     * Uninitialize the hwaccel private data.
     *
     * This will be called from get_format() or ff_codec_close(), after hwaccel
     * and hwaccel_context are already uninitialized.
     */
    int (*uninit)(AVCodecContext *avctx);

    /**
     * Fill the given hw_frames context with current codec parameters. Called
     * from get_format. Refer to avcodec_get_hw_frames_parameters() for
     * details.
     *
     * This CAN be called before AVHWAccel.init is called, and you must assume
     * that avctx->hwaccel_priv_data is invalid.
     */
    int (*frame_params)(AVCodecContext *avctx, AVBufferRef *hw_frames_ctx);

    /**
     * Copy necessary context variables from a previous thread context to the current one.
     * For thread-safe hwaccels only.
     */
    int (*update_thread_context)(AVCodecContext *dst, const AVCodecContext *src);

    /**
     * Callback to free the hwaccel-specific frame data.
     *
     * @param hwctx a pointer to an AVHWDeviceContext.
     * @param data the per-frame hardware accelerator private data to be freed.
     */
    void (*free_frame_priv)(AVRefStructOpaque hwctx, void *data);

    /**
     * Callback to flush the hwaccel state.
     */
    void (*flush)(AVCodecContext *avctx);
} FFHWAccel;

static inline const FFHWAccel *ffhwaccel(const AVHWAccel *codec)
{
    return (const FFHWAccel*)codec;
}

#define FF_HW_CALL(avctx, function, ...) \
        (ffhwaccel((avctx)->hwaccel)->function((avctx), __VA_ARGS__))

#define FF_HW_SIMPLE_CALL(avctx, function) \
        (ffhwaccel((avctx)->hwaccel)->function(avctx))

#define FF_HW_HAS_CB(avctx, function) \
        ((avctx)->hwaccel && ffhwaccel((avctx)->hwaccel)->function)

#endif /* AVCODEC_HWACCEL_INTERNAL */
