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

#ifndef AVCODEC_CODEC_INTERNAL_H
#define AVCODEC_CODEC_INTERNAL_H

#include <stdint.h>

#include "libavutil/attributes.h"
#include "codec.h"
#include "config.h"

/**
 * The codec is not known to be init-threadsafe (i.e. it might be unsafe
 * to initialize this codec and another codec concurrently, typically because
 * the codec calls external APIs that are not known to be thread-safe).
 * Therefore calling the codec's init function needs to be guarded with a lock.
 */
#define FF_CODEC_CAP_NOT_INIT_THREADSAFE    (1 << 0)
/**
 * The codec allows calling the close function for deallocation even if
 * the init function returned a failure. Without this capability flag, a
 * codec does such cleanup internally when returning failures from the
 * init function and does not expect the close function to be called at
 * all.
 */
#define FF_CODEC_CAP_INIT_CLEANUP           (1 << 1)
/**
 * Decoders marked with FF_CODEC_CAP_SETS_PKT_DTS want to set
 * AVFrame.pkt_dts manually. If the flag is set, decode.c won't overwrite
 * this field. If it's unset, decode.c tries to guess the pkt_dts field
 * from the input AVPacket.
 */
#define FF_CODEC_CAP_SETS_PKT_DTS           (1 << 2)
/**
 * The decoder extracts and fills its parameters even if the frame is
 * skipped due to the skip_frame setting.
 */
#define FF_CODEC_CAP_SKIP_FRAME_FILL_PARAM  (1 << 3)
/**
 * The decoder sets the cropping fields in the output frames manually.
 * If this cap is set, the generic code will initialize output frame
 * dimensions to coded rather than display values.
 */
#define FF_CODEC_CAP_EXPORTS_CROPPING       (1 << 4)
/**
 * Codec initializes slice-based threading with a main function
 */
#define FF_CODEC_CAP_SLICE_THREAD_HAS_MF    (1 << 5)
/**
 * The decoder might make use of the ProgressFrame API.
 */
#define FF_CODEC_CAP_USES_PROGRESSFRAMES    (1 << 6)
/**
 * Codec handles avctx->thread_count == 0 (auto) internally.
 */
#define FF_CODEC_CAP_AUTO_THREADS           (1 << 7)
/**
 * Codec handles output frame properties internally instead of letting the
 * internal logic derive them from AVCodecInternal.last_pkt_props.
 */
#define FF_CODEC_CAP_SETS_FRAME_PROPS       (1 << 8)
/**
 * Codec supports embedded ICC profiles (AV_FRAME_DATA_ICC_PROFILE).
 */
#define FF_CODEC_CAP_ICC_PROFILES           (1 << 9)
/**
 * The encoder has AV_CODEC_CAP_DELAY set, but does not actually have delay - it
 * only wants to be flushed at the end to update some context variables (e.g.
 * 2pass stats) or produce a trailing packet. Besides that it immediately
 * produces exactly one output packet per each input frame, just as no-delay
 * encoders do.
 */
#define FF_CODEC_CAP_EOF_FLUSH              (1 << 10)

/**
 * FFCodec.codec_tags termination value
 */
#define FF_CODEC_TAGS_END -1

typedef struct FFCodecDefault {
    const char *key;
    const char *value;
} FFCodecDefault;

struct AVCodecContext;
struct AVSubtitle;
struct AVPacket;
enum AVCodecConfig;

enum FFCodecType {
    /* The codec is a decoder using the decode callback;
     * audio and video codecs only. */
    FF_CODEC_CB_TYPE_DECODE,
    /* The codec is a decoder using the decode_sub callback;
     * subtitle codecs only. */
    FF_CODEC_CB_TYPE_DECODE_SUB,
    /* The codec is a decoder using the receive_frame callback;
     * audio and video codecs only. */
    FF_CODEC_CB_TYPE_RECEIVE_FRAME,
    /* The codec is an encoder using the encode callback;
     * audio and video codecs only. */
    FF_CODEC_CB_TYPE_ENCODE,
    /* The codec is an encoder using the encode_sub callback;
     * subtitle codecs only. */
    FF_CODEC_CB_TYPE_ENCODE_SUB,
    /* The codec is an encoder using the receive_packet callback;
     * audio and video codecs only. */
    FF_CODEC_CB_TYPE_RECEIVE_PACKET,
};

typedef struct FFCodec {
    /**
     * The public AVCodec. See codec.h for it.
     */
    AVCodec p;

    /**
     * Internal codec capabilities FF_CODEC_CAP_*.
     */
    unsigned caps_internal:26;

    /**
     * Is this a decoder?
     */
    unsigned is_decoder:1;

    /**
     * This field determines the video color ranges supported by an encoder.
     * Should be set to a bitmask of AVCOL_RANGE_MPEG and AVCOL_RANGE_JPEG.
     */
    unsigned color_ranges:2;

    /**
     * This field determines the type of the codec (decoder/encoder)
     * and also the exact callback cb implemented by the codec.
     * cb_type uses enum FFCodecType values.
     */
    unsigned cb_type:3;

    int priv_data_size;
    /**
     * @name Frame-level threading support functions
     * @{
     */
    /**
     * Copy necessary context variables from a previous thread context to the current one.
     * If not defined, the next thread will start automatically; otherwise, the codec
     * must call ff_thread_finish_setup().
     *
     * dst and src will (rarely) point to the same context, in which case memcpy should be skipped.
     */
    int (*update_thread_context)(struct AVCodecContext *dst, const struct AVCodecContext *src);

    /**
     * Copy variables back to the user-facing context
     */
    int (*update_thread_context_for_user)(struct AVCodecContext *dst, const struct AVCodecContext *src);
    /** @} */

    /**
     * Private codec-specific defaults.
     */
    const FFCodecDefault *defaults;

    int (*init)(struct AVCodecContext *);

    union {
        /**
         * Decode to an AVFrame.
         * cb is in this state if cb_type is FF_CODEC_CB_TYPE_DECODE.
         *
         * @param      avctx          codec context
         * @param[out] frame          AVFrame for output
         * @param[out] got_frame_ptr  decoder sets to 0 or 1 to indicate that
         *                            a non-empty frame was returned in frame.
         * @param[in]  avpkt          AVPacket containing the data to be decoded
         * @return amount of bytes read from the packet on success,
         *         negative error code on failure
         */
        int (*decode)(struct AVCodecContext *avctx, struct AVFrame *frame,
                      int *got_frame_ptr, struct AVPacket *avpkt);
        /**
         * Decode subtitle data to an AVSubtitle.
         * cb is in this state if cb_type is FF_CODEC_CB_TYPE_DECODE_SUB.
         *
         * Apart from that this is like the decode callback.
         */
        int (*decode_sub)(struct AVCodecContext *avctx, struct AVSubtitle *sub,
                          int *got_frame_ptr, const struct AVPacket *avpkt);
        /**
         * Decode API with decoupled packet/frame dataflow.
         * cb is in this state if cb_type is FF_CODEC_CB_TYPE_RECEIVE_FRAME.
         *
         * This function is called to get one output frame. It should call
         * ff_decode_get_packet() to obtain input data.
         */
        int (*receive_frame)(struct AVCodecContext *avctx, struct AVFrame *frame);
        /**
         * Encode data to an AVPacket.
         * cb is in this state if cb_type is FF_CODEC_CB_TYPE_ENCODE
         *
         * @param      avctx          codec context
         * @param[out] avpkt          output AVPacket
         * @param[in]  frame          AVFrame containing the input to be encoded
         * @param[out] got_packet_ptr encoder sets to 0 or 1 to indicate that a
         *                            non-empty packet was returned in avpkt.
         * @return 0 on success, negative error code on failure
         */
        int (*encode)(struct AVCodecContext *avctx, struct AVPacket *avpkt,
                      const struct AVFrame *frame, int *got_packet_ptr);
        /**
         * Encode subtitles to a raw buffer.
         * cb is in this state if cb_type is FF_CODEC_CB_TYPE_ENCODE_SUB.
         */
        int (*encode_sub)(struct AVCodecContext *avctx, uint8_t *buf,
                          int buf_size, const struct AVSubtitle *sub);
        /**
         * Encode API with decoupled frame/packet dataflow.
         * cb is in this state if cb_type is FF_CODEC_CB_TYPE_RECEIVE_PACKET.
         *
         * This function is called to get one output packet.
         * It should call ff_encode_get_frame() to obtain input data.
         */
        int (*receive_packet)(struct AVCodecContext *avctx, struct AVPacket *avpkt);
    } cb;

    int (*close)(struct AVCodecContext *);

    /**
     * Flush buffers.
     * Will be called when seeking
     */
    void (*flush)(struct AVCodecContext *);

    /**
     * Decoding only, a comma-separated list of bitstream filters to apply to
     * packets before decoding.
     */
    const char *bsfs;

    /**
     * Array of pointers to hardware configurations supported by the codec,
     * or NULL if no hardware supported.  The array is terminated by a NULL
     * pointer.
     *
     * The user can only access this field via avcodec_get_hw_config().
     */
    const struct AVCodecHWConfigInternal *const *hw_configs;

    /**
     * List of supported codec_tags, terminated by FF_CODEC_TAGS_END.
     */
    const uint32_t *codec_tags;

    /**
     * Custom callback for avcodec_get_supported_config(). If absent,
     * ff_default_get_supported_config() will be used. `out_num_configs` will
     * always be set to a valid pointer.
     */
    int (*get_supported_config)(const struct AVCodecContext *avctx,
                                const AVCodec *codec,
                                enum AVCodecConfig config,
                                unsigned flags,
                                const void **out_configs,
                                int *out_num_configs);
} FFCodec;

static av_always_inline const FFCodec *ffcodec(const AVCodec *codec)
{
    return (const FFCodec*)codec;
}

/**
 * Internal version of av_codec_is_encoder(). Must not be called with
 * a NULL AVCodec*.
 */
static inline int ff_codec_is_encoder(const AVCodec *avcodec)
{
    const FFCodec *const codec = ffcodec(avcodec);
    return !codec->is_decoder;
}

/**
 * Internal version of av_codec_is_decoder(). Must not be called with
 * a NULL AVCodec*.
 */
static inline int ff_codec_is_decoder(const AVCodec *avcodec)
{
    const FFCodec *const codec = ffcodec(avcodec);
    return codec->is_decoder;
}

/**
 * Default implementation for avcodec_get_supported_config(). Will return the
 * relevant fields from AVCodec if present, or NULL otherwise.
 *
 * For AVCODEC_CONFIG_COLOR_RANGE, the output will depend on the bitmask in
 * FFCodec.color_ranges, with a value of 0 returning NULL.
 */
int ff_default_get_supported_config(const struct AVCodecContext *avctx,
                                    const AVCodec *codec,
                                    enum AVCodecConfig config,
                                    unsigned flags,
                                    const void **out_configs,
                                    int *out_num_configs);

#if CONFIG_SMALL
#define CODEC_LONG_NAME(str) .p.long_name = NULL
#else
#define CODEC_LONG_NAME(str) .p.long_name = str
#endif

#if HAVE_THREADS
#define UPDATE_THREAD_CONTEXT(func) \
        .update_thread_context          = (func)
#define UPDATE_THREAD_CONTEXT_FOR_USER(func) \
        .update_thread_context_for_user = (func)
#else
#define UPDATE_THREAD_CONTEXT(func) \
        .update_thread_context          = NULL
#define UPDATE_THREAD_CONTEXT_FOR_USER(func) \
        .update_thread_context_for_user = NULL
#endif

#define FF_CODEC_DECODE_CB(func)                          \
    .is_decoder        = 1,                               \
    .cb_type           = FF_CODEC_CB_TYPE_DECODE,         \
    .cb.decode         = (func)
#define FF_CODEC_DECODE_SUB_CB(func)                      \
    .is_decoder        = 1,                               \
    .cb_type           = FF_CODEC_CB_TYPE_DECODE_SUB,     \
    .cb.decode_sub     = (func)
#define FF_CODEC_RECEIVE_FRAME_CB(func)                   \
    .is_decoder        = 1,                               \
    .cb_type           = FF_CODEC_CB_TYPE_RECEIVE_FRAME,  \
    .cb.receive_frame  = (func)
#define FF_CODEC_ENCODE_CB(func)                          \
    .is_decoder        = 0,                               \
    .cb_type           = FF_CODEC_CB_TYPE_ENCODE,         \
    .cb.encode         = (func)
#define FF_CODEC_ENCODE_SUB_CB(func)                      \
    .is_decoder        = 0,                               \
    .cb_type           = FF_CODEC_CB_TYPE_ENCODE_SUB,     \
    .cb.encode_sub     = (func)
#define FF_CODEC_RECEIVE_PACKET_CB(func)                  \
    .is_decoder        = 0,                               \
    .cb_type           = FF_CODEC_CB_TYPE_RECEIVE_PACKET, \
    .cb.receive_packet = (func)

#ifdef __clang__
#define DISABLE_DEPRECATION_WARNINGS FF_DISABLE_DEPRECATION_WARNINGS
#define ENABLE_DEPRECATION_WARNINGS  FF_ENABLE_DEPRECATION_WARNINGS
#else
#define DISABLE_DEPRECATION_WARNINGS
#define ENABLE_DEPRECATION_WARNINGS
#endif

#define CODEC_CH_LAYOUTS(...) CODEC_CH_LAYOUTS_ARRAY(((const AVChannelLayout[]) { __VA_ARGS__, { 0 } }))
#define CODEC_CH_LAYOUTS_ARRAY(array) CODEC_ARRAY(ch_layouts, (array))

#define CODEC_SAMPLERATES(...) CODEC_SAMPLERATES_ARRAY(((const int[]) { __VA_ARGS__, 0 }))
#define CODEC_SAMPLERATES_ARRAY(array) CODEC_ARRAY(supported_samplerates, (array))

#define CODEC_SAMPLEFMTS(...) CODEC_SAMPLEFMTS_ARRAY(((const enum AVSampleFormat[]) { __VA_ARGS__, AV_SAMPLE_FMT_NONE }))
#define CODEC_SAMPLEFMTS_ARRAY(array) CODEC_ARRAY(sample_fmts, (array))

#define CODEC_FRAMERATES(...) CODEC_FRAMERATES_ARRAY(((const AVRational[]) { __VA_ARGS__, { 0, 0 } }))
#define CODEC_FRAMERATES_ARRAY(array) CODEC_ARRAY(supported_framerates, (array))

#define CODEC_PIXFMTS(...) CODEC_PIXFMTS_ARRAY(((const enum AVPixelFormat[]) { __VA_ARGS__, AV_PIX_FMT_NONE }))
#define CODEC_PIXFMTS_ARRAY(array) CODEC_ARRAY(pix_fmts, (array))

#define CODEC_ARRAY(field, array) \
    DISABLE_DEPRECATION_WARNINGS  \
    .p.field = (array)            \
    ENABLE_DEPRECATION_WARNINGS

#endif /* AVCODEC_CODEC_INTERNAL_H */
