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

#ifndef FFTOOLS_THREAD_QUEUE_H
#define FFTOOLS_THREAD_QUEUE_H

#include <string.h>

enum ThreadQueueType {
    THREAD_QUEUE_FRAMES,
    THREAD_QUEUE_PACKETS,
};

typedef struct ThreadQueue ThreadQueue;

/**
 * Allocate a queue for sending data between threads.
 *
 * @param nb_streams number of streams for which a distinct EOF state is
 *                   maintained
 * @param queue_size number of items that can be stored in the queue without
 *                   blocking
 */
ThreadQueue *tq_alloc(unsigned int nb_streams, size_t queue_size,
                      enum ThreadQueueType type);
void         tq_free(ThreadQueue **tq);

/**
 * Send an item for the given stream to the queue.
 *
 * @param data the item to send, its contents will be moved using the callback
 *             provided to tq_alloc(); on failure the item will be left
 *             untouched
 * @return
 * - 0 the item was successfully sent
 * - AVERROR(ENOMEM) could not allocate an item for writing to the FIFO
 * - AVERROR(EINVAL) the sending side has previously been marked as finished
 * - AVERROR_EOF the receiving side has marked the given stream as finished
 */
int tq_send(ThreadQueue *tq, unsigned int stream_idx, void *data);
/**
 * Mark the given stream finished from the sending side.
 */
void tq_send_finish(ThreadQueue *tq, unsigned int stream_idx);

/**
 * Read the next item from the queue.
 *
 * @param stream_idx the index of the stream that was processed or -1 will be
 *                   written here
 * @param data the data item will be written here on success using the
 *             callback provided to tq_alloc()
 * @return
 * - 0 a data item was successfully read; *stream_idx contains a non-negative
 *   stream index
 * - AVERROR_EOF When *stream_idx is non-negative, this signals that the sending
 *   side has marked the given stream as finished. This will happen at most once
 *   for each stream. When *stream_idx is -1, all streams are done.
 */
int tq_receive(ThreadQueue *tq, int *stream_idx, void *data);
/**
 * Mark the given stream finished from the receiving side.
 */
void tq_receive_finish(ThreadQueue *tq, unsigned int stream_idx);

#endif // FFTOOLS_THREAD_QUEUE_H
