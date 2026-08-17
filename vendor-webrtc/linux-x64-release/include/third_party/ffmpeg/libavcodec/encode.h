/*
 * generic encoding-related code
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

#ifndef AVCODEC_ENCODE_H
#define AVCODEC_ENCODE_H

#include "libavutil/frame.h"

#include "avcodec.h"
#include "packet.h"

/**
 * Used by some encoders as upper bound for the length of headers.
 * TODO: Use proper codec-specific upper bounds.
 */
#define FF_INPUT_BUFFER_MIN_SIZE 16384

/**
 * Called by encoders to get the next frame for encoding.
 *
 * @param frame An empty frame to be filled with data.
 * @return 0 if a new reference has been successfully written to frame
 *         AVERROR(EAGAIN) if no data is currently available
 *         AVERROR_EOF if end of stream has been reached, so no more data
 *                     will be available
 */
int ff_encode_get_frame(AVCodecContext *avctx, AVFrame *frame);

/**
 * Get a buffer for a packet. This is a wrapper around
 * AVCodecContext.get_encode_buffer() and should be used instead calling get_encode_buffer()
 * directly.
 */
int ff_get_encode_buffer(AVCodecContext *avctx, AVPacket *avpkt, int64_t size, int flags);

/**
 * Allocate buffers for a frame. Encoder equivalent to ff_get_buffer().
 */
int ff_encode_alloc_frame(AVCodecContext *avctx, AVFrame *frame);

/**
 * Check AVPacket size and allocate data.
 *
 * Encoders of type FF_CODEC_CB_TYPE_ENCODE can use this as a convenience to
 * obtain a big enough buffer for the encoded bitstream.
 *
 * @param avctx   the AVCodecContext of the encoder
 * @param avpkt   The AVPacket: on success, avpkt->data will point to a buffer
 *                of size at least `size`; the packet will not be refcounted.
 *                This packet must be initially blank.
 * @param size    an upper bound of the size of the packet to encode
 * @return        non negative on success, negative error code on failure
 */
int ff_alloc_packet(AVCodecContext *avctx, AVPacket *avpkt, int64_t size);

/**
 * Propagate user opaque values from the frame to avctx/pkt as needed.
 */
int ff_encode_reordered_opaque(AVCodecContext *avctx,
                               AVPacket *pkt, const AVFrame *frame);

int ff_encode_encode_cb(AVCodecContext *avctx, AVPacket *avpkt,
                        AVFrame *frame, int *got_packet);

/**
 * Add a CPB properties side data to an encoding context.
 */
AVCPBProperties *ff_encode_add_cpb_side_data(AVCodecContext *avctx);

/**
 * Rescale from sample rate to AVCodecContext.time_base.
 */
static av_always_inline int64_t ff_samples_to_time_base(const AVCodecContext *avctx,
                                                        int64_t samples)
{
    if (samples == AV_NOPTS_VALUE)
        return AV_NOPTS_VALUE;
    return av_rescale_q(samples, (AVRational){ 1, avctx->sample_rate },
                        avctx->time_base);
}

/**
 * Check if the elements of codec context matrices (intra_matrix, inter_matrix or
 * chroma_intra_matrix) are within the specified range.
 */
#define FF_MATRIX_TYPE_INTRA        (1U << 0)
#define FF_MATRIX_TYPE_INTER        (1U << 1)
#define FF_MATRIX_TYPE_CHROMA_INTRA (1U << 2)
int ff_check_codec_matrices(AVCodecContext *avctx, unsigned types, uint16_t min, uint16_t max);

#endif /* AVCODEC_ENCODE_H */
