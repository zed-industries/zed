/*
 * Audio Frame Queue
 * Copyright (c) 2012 Justin Ruggles
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

#ifndef AVCODEC_AUDIO_FRAME_QUEUE_H
#define AVCODEC_AUDIO_FRAME_QUEUE_H

#include "avcodec.h"

typedef struct AudioFrame {
    int64_t pts;
    int duration;
} AudioFrame;

typedef struct AudioFrameQueue {
    AVCodecContext *avctx;
    int remaining_delay;
    int remaining_samples;
    AudioFrame *frames;
    unsigned frame_count;
    unsigned frame_alloc;
} AudioFrameQueue;

/**
 * Initialize AudioFrameQueue.
 *
 * @param avctx context to use for time_base and av_log
 * @param afq   queue context
 */
void ff_af_queue_init(AVCodecContext *avctx, AudioFrameQueue *afq);

/**
 * Close AudioFrameQueue.
 *
 * Frees memory if needed.
 *
 * @param afq queue context
 */
void ff_af_queue_close(AudioFrameQueue *afq);

/**
 * Add a frame to the queue.
 *
 * @param afq queue context
 * @param f   frame to add to the queue
 */
int ff_af_queue_add(AudioFrameQueue *afq, const AVFrame *f);

/**
 * Remove frame(s) from the queue.
 *
 * Retrieves the pts of the next available frame, or a generated pts based on
 * the last frame duration if there are no frames left in the queue. The number
 * of requested samples should be the full number of samples represented by the
 * packet that will be output by the encoder. If fewer samples are available
 * in the queue, a smaller value will be used for the output duration.
 *
 * @param afq           queue context
 * @param nb_samples    number of samples to remove from the queue
 * @param[out] pts      output packet pts
 * @param[out] duration output packet duration
 */
void ff_af_queue_remove(AudioFrameQueue *afq, int nb_samples, int64_t *pts,
                        int64_t *duration);

#endif /* AVCODEC_AUDIO_FRAME_QUEUE_H */
