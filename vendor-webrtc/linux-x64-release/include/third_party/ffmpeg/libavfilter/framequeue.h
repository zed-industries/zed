/*
 * Generic frame queue
 * Copyright (c) 2016 Nicolas George
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with FFmpeg; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVFILTER_FRAMEQUEUE_H
#define AVFILTER_FRAMEQUEUE_H

/**
 * FFFrameQueue: simple AVFrame queue API
 *
 * Note: this API is not thread-safe. Concurrent access to the same queue
 * must be protected by a mutex or any synchronization mechanism.
 */

#include "libavutil/frame.h"

typedef struct FFFrameBucket {
    AVFrame *frame;
} FFFrameBucket;

/**
 * Structure to hold global options and statistics for frame queues.
 *
 * This structure is intended to allow implementing global control of the
 * frame queues, including memory consumption caps.
 *
 * It is currently empty.
 */
typedef struct FFFrameQueueGlobal {
    char dummy; /* C does not allow empty structs */
} FFFrameQueueGlobal;

/**
 * Queue of AVFrame pointers.
 */
typedef struct FFFrameQueue {

    /**
     * Array of allocated buckets, used as a circular buffer.
     */
    FFFrameBucket *queue;

    /**
     * Size of the array of buckets.
     */
    size_t allocated;

    /**
     * Tail of the queue.
     * It is the index in the array of the next frame to take.
     */
    size_t tail;

    /**
     * Number of currently queued frames.
     */
    size_t queued;

    /**
     * Pre-allocated bucket for queues of size 1.
     */
    FFFrameBucket first_bucket;

    /**
     * Total number of frames entered in the queue.
     */
    uint64_t total_frames_head;

    /**
     * Total number of frames dequeued from the queue.
     * queued = total_frames_head - total_frames_tail
     */
    uint64_t total_frames_tail;

    /**
     * Total number of samples entered in the queue.
     */
    uint64_t total_samples_head;

    /**
     * Total number of samples dequeued from the queue.
     * queued_samples = total_samples_head - total_samples_tail
     */
    uint64_t total_samples_tail;

    /**
     * Indicate that samples are skipped
     */
    int samples_skipped;

} FFFrameQueue;

/**
 * Init a global structure.
 */
void ff_framequeue_global_init(FFFrameQueueGlobal *fqg);

/**
 * Init a frame queue and attach it to a global structure.
 */
void ff_framequeue_init(FFFrameQueue *fq, FFFrameQueueGlobal *fqg);

/**
 * Free the queue and all queued frames.
 */
void ff_framequeue_free(FFFrameQueue *fq);

/**
 * Add a frame.
 * @return  >=0 or an AVERROR code.
 */
int ff_framequeue_add(FFFrameQueue *fq, AVFrame *frame);

/**
 * Take the first frame in the queue.
 * Must not be used with empty queues.
 */
AVFrame *ff_framequeue_take(FFFrameQueue *fq);

/**
 * Access a frame in the queue, without removing it.
 * The first frame is numbered 0; the designated frame must exist.
 */
AVFrame *ff_framequeue_peek(FFFrameQueue *fq, size_t idx);

/**
 * Get the number of queued frames.
 */
static inline size_t ff_framequeue_queued_frames(const FFFrameQueue *fq)
{
    return fq->queued;
}

/**
 * Get the number of queued samples.
 */
static inline uint64_t ff_framequeue_queued_samples(const FFFrameQueue *fq)
{
    return fq->total_samples_head - fq->total_samples_tail;
}

/**
 * Update the statistics after a frame accessed using ff_framequeue_peek()
 * was modified.
 * Currently used only as a marker.
 */
static inline void ff_framequeue_update_peeked(FFFrameQueue *fq, size_t idx)
{
}

/**
 * Skip samples from the first frame in the queue.
 *
 * This function must be used when the first frame was accessed using
 * ff_framequeue_peek() and samples were consumed from it.
 * It adapts the data pointers and timestamps of the head frame to account
 * for the skipped samples.
 */
void ff_framequeue_skip_samples(FFFrameQueue *fq, size_t samples, AVRational time_base);

#endif /* AVFILTER_FRAMEQUEUE_H */
