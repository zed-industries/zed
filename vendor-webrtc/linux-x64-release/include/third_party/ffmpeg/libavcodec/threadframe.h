/*
 * Copyright (c) 2022 Andreas Rheinhardt <andreas.rheinhardt@outlook.com>
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

#ifndef AVCODEC_THREADFRAME_H
#define AVCODEC_THREADFRAME_H

#include "libavutil/frame.h"
#include "avcodec.h"

typedef struct ThreadFrame {
    AVFrame *f;
    AVCodecContext *owner[2];
    struct ThreadFrameProgress *progress;
} ThreadFrame;

/**
 * Notify later decoding threads when part of their reference picture is ready.
 * Call this when some part of the picture is finished decoding.
 * Later calls with lower values of progress have no effect.
 *
 * @param f The picture being decoded.
 * @param progress Value, in arbitrary units, of how much of the picture has decoded.
 * @param field The field being decoded, for field-picture codecs.
 * 0 for top field or frame pictures, 1 for bottom field.
 */
void ff_thread_report_progress(ThreadFrame *f, int progress, int field);

/**
 * Wait for earlier decoding threads to finish reference pictures.
 * Call this before accessing some part of a picture, with a given
 * value for progress, and it will return after the responsible decoding
 * thread calls ff_thread_report_progress() with the same or
 * higher value for progress.
 *
 * @param f The picture being referenced.
 * @param progress Value, in arbitrary units, to wait for.
 * @param field The field being referenced, for field-picture codecs.
 * 0 for top field or frame pictures, 1 for bottom field.
 */
void ff_thread_await_progress(const ThreadFrame *f, int progress, int field);

/**
 * Wrapper around ff_get_buffer() for frame-multithreaded codecs.
 * Call this function instead of ff_get_buffer() if you might need
 * to wait for progress on this frame.
 * Cannot be called after the codec has called ff_thread_finish_setup().
 *
 * @param avctx The current context.
 * @param f The frame to write into.
 * @note: It is fine to call this with codecs that do not support
 *        frame threading.
 */
int ff_thread_get_ext_buffer(AVCodecContext *avctx, ThreadFrame *f, int flags);

/**
 * Unref a ThreadFrame.
 *
 * This is basically a wrapper around av_frame_unref() and should
 * be called instead of it.
 *
 * @param avctx The current context.
 * @param f The picture being released.
 */
void ff_thread_release_ext_buffer(ThreadFrame *f);

int ff_thread_ref_frame(ThreadFrame *dst, const ThreadFrame *src);

int ff_thread_replace_frame(ThreadFrame *dst, const ThreadFrame *src);

#endif
