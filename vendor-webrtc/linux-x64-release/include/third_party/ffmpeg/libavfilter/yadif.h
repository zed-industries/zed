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

#ifndef AVFILTER_YADIF_H
#define AVFILTER_YADIF_H

#include "libavutil/opt.h"
#include "libavutil/pixdesc.h"
#include "avfilter.h"
#include "ccfifo.h"

enum YADIFMode {
    YADIF_MODE_SEND_FRAME           = 0, ///< send 1 frame for each frame
    YADIF_MODE_SEND_FIELD           = 1, ///< send 1 frame for each field
    YADIF_MODE_SEND_FRAME_NOSPATIAL = 2, ///< send 1 frame for each frame but skips spatial interlacing check
    YADIF_MODE_SEND_FIELD_NOSPATIAL = 3, ///< send 1 frame for each field but skips spatial interlacing check
};

enum YADIFParity {
    YADIF_PARITY_TFF  =  0, ///< top field first
    YADIF_PARITY_BFF  =  1, ///< bottom field first
    YADIF_PARITY_AUTO = -1, ///< auto detection
};

enum YADIFDeint {
    YADIF_DEINT_ALL        = 0, ///< deinterlace all frames
    YADIF_DEINT_INTERLACED = 1, ///< only deinterlace frames marked as interlaced
};

enum YADIFCurrentField {
    YADIF_FIELD_BACK_END = -1, ///< The last frame in a sequence
    YADIF_FIELD_END      =  0, ///< The first or last field in a sequence
    YADIF_FIELD_NORMAL   =  1, ///< A normal field in the middle of a sequence
};

typedef struct YADIFContext {
    const AVClass *class;

    int mode;           ///< YADIFMode
    int parity;         ///< YADIFParity
    int deint;          ///< YADIFDeint

    int frame_pending;

    AVFrame *cur;
    AVFrame *next;
    AVFrame *prev;
    AVFrame *out;

    void (*filter)(AVFilterContext *ctx, AVFrame *dstpic, int parity, int tff);

    /**
     * Required alignment for filter_line
     */
    void (*filter_line)(void *dst,
                        void *prev, void *cur, void *next,
                        int w, int prefs, int mrefs, int parity, int mode);
    void (*filter_edges)(void *dst, void *prev, void *cur, void *next,
                         int w, int prefs, int mrefs, int parity, int mode);

    const AVPixFmtDescriptor *csp;
    int eof;
    uint8_t *temp_line;
    int temp_line_size;
    CCFifo cc_fifo;

    /*
     * An algorithm that treats first and/or last fields in a sequence
     * differently can use this to detect those cases. It is the algorithm's
     * responsibility to set the value to YADIF_FIELD_NORMAL after processing
     * the first field.
     */
    int current_field;  ///< YADIFCurrentField

    int pts_multiplier;
} YADIFContext;

void ff_yadif_init_x86(YADIFContext *yadif);

int ff_yadif_filter_frame(AVFilterLink *link, AVFrame *frame);

int ff_yadif_request_frame(AVFilterLink *link);

int ff_yadif_config_output_common(AVFilterLink *outlink);

void ff_yadif_uninit(AVFilterContext *ctx);

extern const AVOption ff_yadif_options[];

#endif /* AVFILTER_YADIF_H */
