/*
 * Copyright (c) 2008 Alexander Strange <astrange@ithinksw.com>
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

/**
 * @file
 * Multithreading API for decoders
 * @author Alexander Strange <astrange@ithinksw.com>
 */

#ifndef AVCODEC_THREAD_H
#define AVCODEC_THREAD_H

#include "libavutil/buffer.h"

#include "avcodec.h"

int ff_thread_can_start_frame(AVCodecContext *avctx);

/**
 * If the codec defines update_thread_context(), call this
 * when they are ready for the next thread to start decoding
 * the next frame. After calling it, do not change any variables
 * read by the update_thread_context() method, or call ff_thread_get_buffer().
 *
 * @param avctx The context.
 */
void ff_thread_finish_setup(AVCodecContext *avctx);

/**
 * Wrapper around get_buffer() for frame-multithreaded codecs.
 * Call this function instead of ff_get_buffer(f).
 * Cannot be called after the codec has called ff_thread_finish_setup().
 *
 * @param avctx The current context.
 * @param f The frame to write into.
 */
int ff_thread_get_buffer(AVCodecContext *avctx, AVFrame *f, int flags);

int ff_slice_thread_execute_with_mainfunc(AVCodecContext *avctx,
        int (*action_func2)(AVCodecContext *c, void *arg, int jobnr, int threadnr),
        int (*main_func)(AVCodecContext *c), void *arg, int *ret, int job_count);

enum ThreadingStatus {
    FF_THREAD_IS_COPY,
    FF_THREAD_IS_FIRST_THREAD,
    FF_THREAD_NO_FRAME_THREADING,
};

/**
 * Allows to synchronize objects whose lifetime is the whole decoding
 * process among all frame threads.
 *
 * When called from a non-copy thread, do nothing.
 * When called from another thread, place a new RefStruct reference
 * at the given offset in the calling thread's private data from
 * the RefStruct reference in the private data of the first decoding thread.
 * The first thread must have a valid RefStruct reference at the given
 * offset in its private data; the calling thread must not have
 * a reference at this offset in its private data (must be NULL).
 *
 * @param avctx  an AVCodecContext
 * @param offset offset of the RefStruct reference in avctx's private data
 *
 * @retval FF_THREAD_IS_COPY if frame-threading is in use and the
 *         calling thread is a copy; in this case, the RefStruct reference
 *         will be set.
 * @retval FF_THREAD_IS_MAIN_THREAD if frame-threading is in use
 *         and the calling thread is the main thread.
 * @retval FF_THREAD_NO_FRAME_THREADING if frame-threading is not in use.
 */
enum ThreadingStatus ff_thread_sync_ref(AVCodecContext *avctx, size_t offset);

#endif /* AVCODEC_THREAD_H */
