/*
 * This file is part of FFmpeg.
 *
 * Copyright (c) 2015 Matthieu Bouron <matthieu.bouron stupeflix.com>
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

#ifndef AVFILTER_FRAMEPOOL_H
#define AVFILTER_FRAMEPOOL_H

#include "libavutil/buffer.h"
#include "libavutil/frame.h"
#include "libavutil/internal.h"

/**
 * Frame pool. This structure is opaque and not meant to be accessed
 * directly. It is allocated with ff_frame_pool_init() and freed with
 * ff_frame_pool_uninit().
 */
typedef struct FFFramePool FFFramePool;

/**
 * Allocate and initialize a video frame pool.
 *
 * @param alloc a function that will be used to allocate new frame buffers when
 * the pool is empty. May be NULL, then the default allocator will be used
 * (av_buffer_alloc()).
 * @param width width of each frame in this pool
 * @param height height of each frame in this pool
 * @param format format of each frame in this pool
 * @param align buffers alignement of each frame in this pool
 * @return newly created video frame pool on success, NULL on error.
 */
FFFramePool *ff_frame_pool_video_init(AVBufferRef* (*alloc)(size_t size),
                                      int width,
                                      int height,
                                      enum AVPixelFormat format,
                                      int align);

/**
 * Allocate and initialize an audio frame pool.
 *
 * @param alloc a function that will be used to allocate new frame buffers when
 * the pool is empty. May be NULL, then the default allocator will be used
 * (av_buffer_alloc()).
 * @param channels channels of each frame in this pool
 * @param nb_samples number of samples of each frame in this pool
 * @param format format of each frame in this pool
 * @param align buffers alignement of each frame in this pool
 * @return newly created audio frame pool on success, NULL on error.
 */
FFFramePool *ff_frame_pool_audio_init(AVBufferRef* (*alloc)(size_t size),
                                      int channels,
                                      int samples,
                                      enum AVSampleFormat format,
                                      int align);

/**
 * Deallocate the frame pool. It is safe to call this function while
 * some of the allocated frame are still in use.
 *
 * @param pool pointer to the frame pool to be freed. It will be set to NULL.
 */
void ff_frame_pool_uninit(FFFramePool **pool);

/**
 * Get the video frame pool configuration.
 *
 * @param width width of each frame in this pool
 * @param height height of each frame in this pool
 * @param format format of each frame in this pool
 * @param align buffers alignement of each frame in this pool
 * @return 0 on success, a negative AVERROR otherwise.
 */
int ff_frame_pool_get_video_config(FFFramePool *pool,
                                   int *width,
                                   int *height,
                                   enum AVPixelFormat *format,
                                   int *align);

/**
 * Get the audio frame pool configuration.
 *
 * @param channels channels of each frame in this pool
 * @param nb_samples number of samples of each frame in this pool
 * @param format format of each frame in this pool
 * @param align buffers alignement of each frame in this pool
 * @return 0 on success, a negative AVERROR otherwise.
 */
int ff_frame_pool_get_audio_config(FFFramePool *pool,
                                   int *channels,
                                   int *nb_samples,
                                   enum AVSampleFormat *format,
                                   int *align);


/**
 * Allocate a new AVFrame, reussing old buffers from the pool when available.
 * This function may be called simultaneously from multiple threads.
 *
 * @return a new AVFrame on success, NULL on error.
 */
AVFrame *ff_frame_pool_get(FFFramePool *pool);


#endif /* AVFILTER_FRAMEPOOL_H */
