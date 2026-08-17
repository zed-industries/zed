/*
 * V4L2 buffer helper functions.
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

#ifndef AVCODEC_V4L2_BUFFERS_H
#define AVCODEC_V4L2_BUFFERS_H

#include <stdatomic.h>
#include <stddef.h>
#include <linux/videodev2.h>

#include "libavutil/frame.h"
#include "packet.h"

enum V4L2Buffer_status {
    V4L2BUF_AVAILABLE,
    V4L2BUF_IN_DRIVER,
    V4L2BUF_RET_USER,
};

/**
 * V4L2Buffer (wrapper for v4l2_buffer management)
 */
typedef struct V4L2Buffer {
    /* each buffer needs to have a reference to its context */
    struct V4L2Context *context;

    /* This object is refcounted per-plane, so we need to keep track
     * of how many context-refs we are holding.
     * This pointer is a RefStruct reference. */
    const struct V4L2m2mContext *context_ref;
    atomic_uint context_refcount;

    /* keep track of the mmap address and mmap length */
    struct V4L2Plane_info {
        int bytesperline;
        void * mm_addr;
        size_t length;
    } plane_info[VIDEO_MAX_PLANES];

    int num_planes;

    /* the v4l2_buffer buf.m.planes pointer uses the planes[] mem */
    struct v4l2_buffer buf;
    struct v4l2_plane planes[VIDEO_MAX_PLANES];

    int flags;
    enum V4L2Buffer_status status;

} V4L2Buffer;

/**
 * Extracts the data from a V4L2Buffer to an AVFrame
 *
 * @param[in] frame The AVFRame to push the information to
 * @param[in] buf The V4L2Buffer to get the information from
 *
 * @returns 0 in case of success, AVERROR(EINVAL) if the number of planes is incorrect,
 * AVERROR(ENOMEM) if the AVBufferRef can't be created.
 */
int ff_v4l2_buffer_buf_to_avframe(AVFrame *frame, V4L2Buffer *buf);

/**
 * Extracts the data from a V4L2Buffer to an AVPacket
 *
 * @param[in] pkt The AVPacket to push the information to
 * @param[in] buf The V4L2Buffer to get the information from
 *
 * @returns 0 in case of success, AVERROR(EINVAL) if the number of planes is incorrect,
 * AVERROR(ENOMEM) if the AVBufferRef can't be created.
 *
 */
int ff_v4l2_buffer_buf_to_avpkt(AVPacket *pkt, V4L2Buffer *buf);

/**
 * Extracts the data from an AVPacket to a V4L2Buffer
 *
 * @param[in]  frame AVPacket to get the data from
 * @param[in]  avbuf V4L2Bfuffer to push the information to
 *
 * @returns 0 in case of success, a negative AVERROR code otherwise
 */
int ff_v4l2_buffer_avpkt_to_buf(const AVPacket *pkt, V4L2Buffer *out);

/**
 * Extracts the data from an AVFrame to a V4L2Buffer
 *
 * @param[in]  frame AVFrame to get the data from
 * @param[in]  avbuf V4L2Bfuffer to push the information to
 *
 * @returns 0 in case of success, a negative AVERROR code otherwise
 */
int ff_v4l2_buffer_avframe_to_buf(const AVFrame *frame, V4L2Buffer *out);

/**
 * Initializes a V4L2Buffer
 *
 * @param[in]  avbuf V4L2Bfuffer to initialize
 * @param[in]  index v4l2 buffer id
 *
 * @returns 0 in case of success, a negative AVERROR code otherwise
 */
int ff_v4l2_buffer_initialize(V4L2Buffer* avbuf, int index);

/**
 * Enqueues a V4L2Buffer
 *
 * @param[in] avbuf V4L2Bfuffer to push to the driver
 *
 * @returns 0 in case of success, a negative AVERROR code otherwise
 */
int ff_v4l2_buffer_enqueue(V4L2Buffer* avbuf);


#endif // AVCODEC_V4L2_BUFFERS_H
