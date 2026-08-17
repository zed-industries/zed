/*
 * CEA-708 Closed Captioning FIFO
 * Copyright (c) 2023 LTN Global Communications
 *
 * Author: Devin Heitmueller <dheitmueller@ltnglobal.com>
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
 * CC FIFO Buffer
 */

#ifndef AVFILTER_CCFIFO_H
#define AVFILTER_CCFIFO_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/frame.h"
#include "libavutil/rational.h"

#define CC_BYTES_PER_ENTRY 3

typedef struct CCFifo {
    struct AVFifo *cc_608_fifo;
    struct AVFifo *cc_708_fifo;
    AVRational framerate;
    int expected_cc_count;
    int expected_608;
    int cc_detected;
    int passthrough;
    int passthrough_warning;
    void *log_ctx;
} CCFifo;

/**
 * Initialize a CCFifo.
 *
 * @param framerate   output framerate
 * @param log_ctx     used for any av_log() calls
 * @return            Zero on success, or negative AVERROR code on failure.
 */
int ff_ccfifo_init(CCFifo *ccf, AVRational framerate, void *log_ctx);

/**
 * Free all memory allocated in a CCFifo and clear the context.
 *
 * @param ccf Pointer to the CCFifo which should be uninitialized
 */
void ff_ccfifo_uninit(CCFifo *ccf);

/**
 * Extract CC data from an AVFrame
 *
 * Extract CC bytes from the AVFrame, insert them into our queue, and
 * remove the side data from the AVFrame.  The side data is removed
 * as it will be re-inserted at the appropriate rate later in the
 * filter.
 *
 * @param af          CCFifo to write to
 * @param frame       AVFrame with the video frame to operate on
 * @return            Zero on success, or negative AVERROR
 *                    code on failure.
 */
int ff_ccfifo_extract(CCFifo *ccf, AVFrame *frame);

/**
 *Just like ff_ccfifo_extract(), but takes the raw bytes instead of an AVFrame
 */
int ff_ccfifo_extractbytes(CCFifo *ccf, uint8_t *data, size_t len);

/**
 * Provide the size in bytes of an output buffer to allocate
 *
 * Ask for how many bytes the output will contain, so the caller can allocate
 * an appropriately sized buffer and pass it to ff_ccfifo_injectbytes()
 *
 */
static inline int ff_ccfifo_getoutputsize(const CCFifo *ccf)
{
    return ccf->expected_cc_count * CC_BYTES_PER_ENTRY;
}


/**
 * Insert CC data from the FIFO into an AVFrame (as side data)
 *
 * Dequeue the appropriate number of CC tuples based on the
 * frame rate, and insert them into the AVFrame
 *
 * @param af          CCFifo to read from
 * @param frame       AVFrame with the video frame to operate on
 * @return            Zero on success, or negative AVERROR
 *                    code on failure.
 */
int ff_ccfifo_inject(CCFifo *ccf, AVFrame *frame);

/**
 * Just like ff_ccfifo_inject(), but takes the raw bytes to insert the CC data
 * int rather than an AVFrame
 */
int ff_ccfifo_injectbytes(CCFifo *ccf, uint8_t *data, size_t len);

/**
 * Returns 1 if captions have been found as a prior call
 * to ff_ccfifo_extract() or ff_ccfifo_extractbytes()
 */
static inline int ff_ccfifo_ccdetected(const CCFifo *ccf)
{
    return ccf->cc_detected;
}

#endif /* AVFILTER_CCFIFO_H */
