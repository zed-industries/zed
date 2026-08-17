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

#ifndef AVCODEC_PROGRESSFRAME_H
#define AVCODEC_PROGRESSFRAME_H

/**
 * ProgressFrame is an API to easily share frames without an underlying
 * av_frame_ref(). Its main usecase is in frame-threading scenarios,
 * yet it could also be used for purely single-threaded decoders that
 * want to keep multiple references to the same frame.
 *
 * The underlying principle behind the API is that all that is needed
 * to share a frame is a reference count and a contract between all parties.
 * The ProgressFrame provides the reference count and the frame is unreferenced
 * via ff_thread_release_buffer() when the reference count reaches zero.
 *
 * In order to make this API also usable for frame-threaded decoders it also
 * provides a way of exchanging simple information about the state of
 * decoding the frame via ff_thread_progress_report() and
 * ff_thread_progress_await().
 *
 * The typical contract for frame-threaded decoders is as follows:
 * Thread A initializes a ProgressFrame via ff_thread_progress_get_buffer()
 * (which already allocates the AVFrame's data buffers), calls
 * ff_thread_finish_setup() and starts decoding the frame. Later threads
 * receive a reference to this frame, which means they get a pointer
 * to the AVFrame and the internal reference count gets incremented.
 * Later threads whose frames use A's frame as reference as well as
 * the thread that will eventually output A's frame will wait for
 * progress on said frame reported by A. As soon as A has reported
 * that it has finished decoding its frame, it must no longer modify it
 * (neither its data nor its properties).
 *
 * Because creating a reference with this API does not involve reads
 * from the actual AVFrame, the decoding thread may modify the properties
 * (i.e. non-data fields) until it has indicated to be done with this
 * frame. This is important for e.g. propagating decode_error_flags;
 * it also allows to add side-data late.
 */

struct AVCodecContext;

/**
 * The ProgressFrame structure.
 * Hint: It is guaranteed that the AVFrame pointer is at the start
 *       of ProgressFrame. This allows to use an unnamed
 *       union {
 *            struct {
 *                AVFrame *f;
 *            };
 *            ProgressFrame pf;
 *       };
 *       to simplify accessing the embedded AVFrame.
 */
typedef struct ProgressFrame {
    struct AVFrame *f;
    struct ProgressInternal *progress;
} ProgressFrame;

/**
 * Notify later decoding threads when part of their reference frame is ready.
 * Call this when some part of the frame is finished decoding.
 * Later calls with lower values of progress have no effect.
 *
 * @param f The frame being decoded.
 * @param progress Value, in arbitrary units, of how much of the frame has decoded.
 *
 * @warning Calling this on a blank ProgressFrame causes undefined behaviour
 */
void ff_progress_frame_report(ProgressFrame *f, int progress);

/**
 * Wait for earlier decoding threads to finish reference frames.
 * Call this before accessing some part of a frame, with a given
 * value for progress, and it will return after the responsible decoding
 * thread calls ff_thread_progress_report() with the same or
 * higher value for progress.
 *
 * @param f The frame being referenced.
 * @param progress Value, in arbitrary units, to wait for.
 *
 * @warning Calling this on a blank ProgressFrame causes undefined behaviour
 */
void ff_progress_frame_await(const ProgressFrame *f, int progress);

/**
 * This function allocates ProgressFrame.f
 * May be called before ff_progress_frame_get_buffer() in the cases where the
 * AVFrame needs to be accessed before the ff_thread_get_buffer() call in
 * ff_progress_frame_alloc().
 *
 * @note: This must only be called by codecs with the
 *        FF_CODEC_CAP_USES_PROGRESSFRAMES internal cap.
 */
int ff_progress_frame_alloc(struct AVCodecContext *avctx, ProgressFrame *f);

/**
 * This function sets up the ProgressFrame, i.e. allocates ProgressFrame.f
 * if needed, and also calls ff_thread_get_buffer() on the frame.
 *
 * @note: This must only be called by codecs with the
 *        FF_CODEC_CAP_USES_PROGRESSFRAMES internal cap.
 * @see ff_progress_frame_alloc
 */
int ff_progress_frame_get_buffer(struct AVCodecContext *avctx,
                                 ProgressFrame *f, int flags);

/**
 * Give up a reference to the underlying frame contained in a ProgressFrame
 * and reset the ProgressFrame, setting all pointers to NULL.
 *
 * @note: This implies that when using this API the check for whether
 *        a frame exists is by checking ProgressFrame.f and not
 *        ProgressFrame.f->data[0] or ProgressFrame.f->buf[0].
 */
void ff_progress_frame_unref(ProgressFrame *f);

/**
 * Set dst->f to src->f and make dst a co-owner of src->f.
 * dst can then be used to wait on progress of the underlying frame.
 *
 * @note: There is no underlying av_frame_ref() here. dst->f and src->f
 *        really point to the same AVFrame. Typically this means that
 *        the decoding thread is allowed to set all the properties of
 *        the AVFrame until it has indicated to have finished decoding.
 *        Afterwards later threads may read all of these fields.
 *        Access to the frame's data is governed by
 *        ff_thread_progress_report/await().
 */
void ff_progress_frame_ref(ProgressFrame *dst, const ProgressFrame *src);

/**
 * Do nothing if dst and src already refer to the same AVFrame;
 * otherwise unreference dst and if src is not blank, put a reference
 * to src's AVFrame in its place (in case src is not blank).
 */
void ff_progress_frame_replace(ProgressFrame *dst, const ProgressFrame *src);

#endif /* AVCODEC_PROGRESSFRAME_H */
