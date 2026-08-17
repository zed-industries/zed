/*
 * V4L2 context helper functions.
 *
 * Copyright (C) 2017 Alexis Ballier <aballier@gentoo.org>
 * Copyright (C) 2017 Jorge Ramirez <jorge.ramirez-ortiz@linaro.org>
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

#ifndef AVCODEC_V4L2_CONTEXT_H
#define AVCODEC_V4L2_CONTEXT_H

#include <stdint.h>
#include <linux/videodev2.h>

#include "libavutil/pixfmt.h"
#include "libavutil/frame.h"
#include "libavutil/rational.h"
#include "codec_id.h"
#include "packet.h"
#include "v4l2_buffers.h"

typedef struct V4L2Context {
    /**
     * context name.
     */
    const char* name;

    /**
     * Type of this buffer context.
     * See V4L2_BUF_TYPE_VIDEO_* in videodev2.h
     * Readonly after init.
     */
    enum v4l2_buf_type type;

    /**
     * AVPixelFormat corresponding to this buffer context.
     * AV_PIX_FMT_NONE means this is an encoded stream.
     */
    enum AVPixelFormat av_pix_fmt;

    /**
     * AVCodecID corresponding to this buffer context.
     * AV_CODEC_ID_RAWVIDEO means this is a raw stream and av_pix_fmt must be set to a valid value.
     */
    enum AVCodecID av_codec_id;

    /**
     * Format returned by the driver after initializing the buffer context.
     * Readonly after init.
     */
    struct v4l2_format format;

    /**
     * Width and height of the frames it produces (in case of a capture context, e.g. when decoding)
     * or accepts (in case of an output context, e.g. when encoding).
     */
    int width, height;
    AVRational sample_aspect_ratio;

    /**
     * Indexed array of V4L2Buffers
     */
    V4L2Buffer *buffers;

    /**
     * Readonly after init.
     */
    int num_buffers;

    /**
     * Whether the stream has been started (VIDIOC_STREAMON has been sent).
     */
    int streamon;

    /**
     *  Either no more buffers available or an unrecoverable error was notified
     *  by the V4L2 kernel driver: once set the context has to be exited.
     */
    int done;

} V4L2Context;

/**
 * Initializes a V4L2Context.
 *
 * @param[in] ctx A pointer to a V4L2Context. See V4L2Context description for required variables.
 * @return 0 in case of success, a negative value representing the error otherwise.
 */
int ff_v4l2_context_init(V4L2Context* ctx);

/**
 * Sets the V4L2Context format in the v4l2 driver.
 *
 * @param[in] ctx A pointer to a V4L2Context. See V4L2Context description for required variables.
 * @return 0 in case of success, a negative value representing the error otherwise.
 */
int ff_v4l2_context_set_format(V4L2Context* ctx);

/**
 * Queries the driver for a valid v4l2 format and copies it to the context.
 *
 * @param[in] ctx A pointer to a V4L2Context. See V4L2Context description for required variables.
 * @param[in] probe Probe only and ignore changes to the format.
 * @return 0 in case of success, a negative value representing the error otherwise.
 */
int ff_v4l2_context_get_format(V4L2Context* ctx, int probe);

/**
 * Releases a V4L2Context.
 *
 * @param[in] ctx A pointer to a V4L2Context.
 *               The caller is reponsible for freeing it.
 *               It must not be used after calling this function.
 */
void ff_v4l2_context_release(V4L2Context* ctx);

/**
 * Sets the status of a V4L2Context.
 *
 * @param[in] ctx A pointer to a V4L2Context.
 * @param[in] cmd The status to set (VIDIOC_STREAMON or VIDIOC_STREAMOFF).
 *                Warning: If VIDIOC_STREAMOFF is sent to a buffer context that still has some frames buffered,
 *                those frames will be dropped.
 * @return 0 in case of success, a negative value representing the error otherwise.
 */
int ff_v4l2_context_set_status(V4L2Context* ctx, uint32_t cmd);

/**
 * Dequeues a buffer from a V4L2Context to an AVPacket.
 *
 * The pkt must be non NULL.
 * @param[in] ctx The V4L2Context to dequeue from.
 * @param[inout] pkt The AVPacket to dequeue to.
 * @return 0 in case of success, AVERROR(EAGAIN) if no buffer was ready, another negative error in case of error.
 */
int ff_v4l2_context_dequeue_packet(V4L2Context* ctx, AVPacket* pkt);

/**
 * Dequeues a buffer from a V4L2Context to an AVFrame.
 *
 * The frame must be non NULL.
 * @param[in] ctx The V4L2Context to dequeue from.
 * @param[inout] f The AVFrame to dequeue to.
 * @param[in] timeout The timeout for dequeue (-1 to block, 0 to return immediately, or milliseconds)
 * @return 0 in case of success, AVERROR(EAGAIN) if no buffer was ready, another negative error in case of error.
 */
int ff_v4l2_context_dequeue_frame(V4L2Context* ctx, AVFrame* f, int timeout);

/**
 * Enqueues a buffer to a V4L2Context from an AVPacket
 *
 * The packet must be non NULL.
 * When the size of the pkt is null, the buffer is not queued but a V4L2_DEC_CMD_STOP command is sent instead to the driver.
 *
 * @param[in] ctx The V4L2Context to enqueue to.
 * @param[in] pkt A pointer to an AVPacket.
 * @return 0 in case of success, a negative error otherwise.
 */
int ff_v4l2_context_enqueue_packet(V4L2Context* ctx, const AVPacket* pkt);

/**
 * Enqueues a buffer to a V4L2Context from an AVFrame
 *
 * The frame must be non NULL.
 *
 * @param[in] ctx The V4L2Context to enqueue to.
 * @param[in] f A pointer to an AVFrame to enqueue.
 * @return 0 in case of success, a negative error otherwise.
 */
int ff_v4l2_context_enqueue_frame(V4L2Context* ctx, const AVFrame* f);

#endif // AVCODEC_V4L2_CONTEXT_H
