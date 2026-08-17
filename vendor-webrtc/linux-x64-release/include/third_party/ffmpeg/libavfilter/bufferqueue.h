/*
 * Generic buffer queue
 * Copyright (c) 2012 Nicolas George
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

#ifndef AVFILTER_BUFFERQUEUE_H
#define AVFILTER_BUFFERQUEUE_H

/**
 * FFBufQueue: simple AVFrame queue API
 *
 * Note: this API is not thread-safe. Concurrent access to the same queue
 * must be protected by a mutex or any synchronization mechanism.
 */

/**
 * Maximum size of the queue.
 *
 * This value can be overridden by definying it before including this
 * header.
 * Powers of 2 are recommended.
 */
#ifndef FF_BUFQUEUE_SIZE
#define FF_BUFQUEUE_SIZE 64
#endif

#include "avfilter.h"
#include "libavutil/avassert.h"

/**
 * Structure holding the queue
 */
struct FFBufQueue {
    AVFrame *queue[FF_BUFQUEUE_SIZE];
    unsigned short head;
    unsigned short available; /**< number of available buffers */
};

#define BUCKET(i) queue->queue[(queue->head + (i)) % FF_BUFQUEUE_SIZE]

/**
 * Test if a buffer queue is full.
 */
static inline int ff_bufqueue_is_full(struct FFBufQueue *queue)
{
    return queue->available == FF_BUFQUEUE_SIZE;
}

/**
 * Add a buffer to the queue.
 *
 * If the queue is already full, then the current last buffer is dropped
 * (and unrefed) with a warning before adding the new buffer.
 */
static inline void ff_bufqueue_add(void *log, struct FFBufQueue *queue,
                                   AVFrame *buf)
{
    if (ff_bufqueue_is_full(queue)) {
        av_log(log, AV_LOG_WARNING, "Buffer queue overflow, dropping.\n");
        av_frame_free(&BUCKET(--queue->available));
    }
    BUCKET(queue->available++) = buf;
}

/**
 * Get a buffer from the queue without altering it.
 *
 * Buffer with index 0 is the first buffer in the queue.
 * Return NULL if the queue has not enough buffers.
 */
static inline AVFrame *ff_bufqueue_peek(struct FFBufQueue *queue,
                                        unsigned index)
{
    return index < queue->available ? BUCKET(index) : NULL;
}

/**
 * Get the first buffer from the queue and remove it.
 *
 * Do not use on an empty queue.
 */
static inline AVFrame *ff_bufqueue_get(struct FFBufQueue *queue)
{
    AVFrame *ret = queue->queue[queue->head];
    av_assert0(queue->available);
    queue->available--;
    queue->queue[queue->head] = NULL;
    queue->head = (queue->head + 1) % FF_BUFQUEUE_SIZE;
    return ret;
}

/**
 * Unref and remove all buffers from the queue.
 */
static inline void ff_bufqueue_discard_all(struct FFBufQueue *queue)
{
    while (queue->available) {
        AVFrame *buf = ff_bufqueue_get(queue);
        av_frame_free(&buf);
    }
}

#undef BUCKET

#endif /* AVFILTER_BUFFERQUEUE_H */
