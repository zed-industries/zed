/*
 * Copyright (c) 2013 Nicolas George
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

#ifndef AVFILTER_FRAMESYNC_H
#define AVFILTER_FRAMESYNC_H

#include "bufferqueue.h"

enum EOFAction {
    EOF_ACTION_REPEAT,
    EOF_ACTION_ENDALL,
    EOF_ACTION_PASS
};

/*
 * TODO
 * Export convenient options.
 */

/**
 * This API is intended as a helper for filters that have several video
 * input and need to combine them somehow. If the inputs have different or
 * variable frame rate, getting the input frames to match requires a rather
 * complex logic and a few user-tunable options.
 *
 * In this API, when a set of synchronized input frames is ready to be
 * procesed is called a frame event. Frame event can be generated in
 * response to input frames on any or all inputs and the handling of
 * situations where some stream extend beyond the beginning or the end of
 * others can be configured.
 *
 * The basic working of this API is the following: set the on_event
 * callback, then call ff_framesync_activate() from the filter's activate
 * callback.
 */

/**
 * Stream extrapolation mode
 *
 * Describe how the frames of a stream are extrapolated before the first one
 * and after EOF to keep sync with possibly longer other streams.
 */
enum FFFrameSyncExtMode {

    /**
     * Completely stop all streams with this one.
     */
    EXT_STOP,

    /**
     * Ignore this stream and continue processing the other ones.
     */
    EXT_NULL,

    /**
     * Extend the frame to infinity.
     */
    EXT_INFINITY,
};

/**
 * Timestamp syncronization mode
 *
 * Describe how the frames of a stream are syncronized based on timestamp
 * distance.
 */
enum FFFrameTSSyncMode {

    /**
     * Sync to frames from secondary input with the nearest, lower or equal
     * timestamp to the frame event one.
     */
    TS_DEFAULT,

    /**
     * Sync to frames from secondary input with the absolute nearest timestamp
     * to the frame event one.
     */
    TS_NEAREST,
};

/**
 * Input stream structure
 */
typedef struct FFFrameSyncIn {

    /**
     * Extrapolation mode for timestamps before the first frame
     */
    enum FFFrameSyncExtMode before;

    /**
     * Extrapolation mode for timestamps after the last frame
     */
    enum FFFrameSyncExtMode after;

    /**
     * Time base for the incoming frames
     */
    AVRational time_base;

    /**
     * Current frame, may be NULL before the first one or after EOF
     */
    AVFrame *frame;

    /**
     * Next frame, for internal use
     */
    AVFrame *frame_next;

    /**
     * PTS of the current frame
     */
    int64_t pts;

    /**
     * PTS of the next frame, for internal use
     */
    int64_t pts_next;

    /**
     * Boolean flagging the next frame, for internal use
     */
    uint8_t have_next;

    /**
     * State: before first, in stream or after EOF, for internal use
     */
    uint8_t state;

    /**
     * Synchronization level: frames on input at the highest sync level will
     * generate output frame events.
     *
     * For example, if inputs #0 and #1 have sync level 2 and input #2 has
     * sync level 1, then a frame on either input #0 or #1 will generate a
     * frame event, but not a frame on input #2 until both inputs #0 and #1
     * have reached EOF.
     *
     * If sync is 0, no frame event will be generated.
     */
    unsigned sync;

    enum FFFrameTSSyncMode ts_mode;
} FFFrameSyncIn;

/**
 * Frame sync structure.
 */
typedef struct FFFrameSync {
    const AVClass *class;

    /**
     * Parent filter context.
     */
    AVFilterContext *parent;

    /**
     * Number of input streams
     */
    unsigned nb_in;

    /**
     * Time base for the output events
     */
    AVRational time_base;

    /**
     * Timestamp of the current event
     */
    int64_t pts;

    /**
     * Callback called when a frame event is ready
     */
    int (*on_event)(struct FFFrameSync *fs);

    /**
     * Opaque pointer, not used by the API
     */
    void *opaque;

    /**
     * Index of the input that requires a request
     */
    unsigned in_request;

    /**
     * Synchronization level: only inputs with the same sync level are sync
     * sources.
     */
    unsigned sync_level;

    /**
     * Flag indicating that a frame event is ready
     */
    uint8_t frame_ready;

    /**
     * Flag indicating that output has reached EOF.
     */
    uint8_t eof;

    /**
     * Pointer to array of inputs.
     */
    FFFrameSyncIn *in;

    int opt_repeatlast;
    int opt_shortest;
    int opt_eof_action;
    int opt_ts_sync_mode;

} FFFrameSync;

/**
 * Pre-initialize a frame sync structure.
 *
 * It sets the class pointer and inits the options to their default values.
 * The entire structure is expected to be already set to 0.
 * This step is optional, but necessary to use the options.
 */
void ff_framesync_preinit(FFFrameSync *fs);

/**
 * Initialize a frame sync structure.
 *
 * The entire structure is expected to be already set to 0 or preinited.
 *
 * @param  fs      frame sync structure to initialize
 * @param  parent  parent AVFilterContext object
 * @param  nb_in   number of inputs
 * @return  >= 0 for success or a negative error code
 */
int ff_framesync_init(FFFrameSync *fs, AVFilterContext *parent, unsigned nb_in);

/**
 * Configure a frame sync structure.
 *
 * Must be called after all options are set but before all use.
 *
 * @return  >= 0 for success or a negative error code
 */
int ff_framesync_configure(FFFrameSync *fs);

/**
 * Free all memory currently allocated.
 */
void ff_framesync_uninit(FFFrameSync *fs);

/**
 * Get the current frame in an input.
 *
 * @param fs      frame sync structure
 * @param in      index of the input
 * @param rframe  used to return the current frame (or NULL)
 * @param get     if not zero, the calling code needs to get ownership of
 *                the returned frame; the current frame will either be
 *                duplicated or removed from the framesync structure
 */
int ff_framesync_get_frame(FFFrameSync *fs, unsigned in, AVFrame **rframe,
                            unsigned get);

/**
 * Examine the frames in the filter's input and try to produce output.
 *
 * This function can be the complete implementation of the activate
 * method of a filter using framesync.
 */
int ff_framesync_activate(FFFrameSync *fs);

/**
 * Initialize a frame sync structure for dualinput.
 *
 * Compared to generic framesync, dualinput assumes the first input is the
 * main one and the filtering is performed on it. The first input will be
 * the only one with sync set and generic timeline support will just pass it
 * unchanged when disabled.
 *
 * Equivalent to ff_framesync_init(fs, parent, 2) then setting the time
 * base, sync and ext modes on the inputs.
 */
int ff_framesync_init_dualinput(FFFrameSync *fs, AVFilterContext *parent);

/**
 * @param f0  used to return the main frame
 * @param f1  used to return the second frame, or NULL if disabled
 * @return  >=0 for success or AVERROR code
 * @note  The frame returned in f0 belongs to the caller (get = 1 in
 * ff_framesync_get_frame()) while the frame returned in f1 is still owned
 * by the framesync structure.
 */
int ff_framesync_dualinput_get(FFFrameSync *fs, AVFrame **f0, AVFrame **f1);

/**
 * Same as ff_framesync_dualinput_get(), but make sure that f0 is writable.
 */
int ff_framesync_dualinput_get_writable(FFFrameSync *fs, AVFrame **f0, AVFrame **f1);

const AVClass *ff_framesync_child_class_iterate(void **iter);
extern const AVClass ff_framesync_class;

#define FRAMESYNC_DEFINE_PURE_CLASS(name, desc, func_prefix, options) \
static const AVClass name##_class = {                                 \
    .class_name          = desc,                                      \
    .item_name           = av_default_item_name,                      \
    .option              = options,                                   \
    .version             = LIBAVUTIL_VERSION_INT,                     \
    .category            = AV_CLASS_CATEGORY_FILTER,                  \
    .child_class_iterate = ff_framesync_child_class_iterate,          \
    .child_next          = func_prefix##_child_next,                  \
}

/* A filter that uses the *_child_next-function from this macro
 * is required to initialize the FFFrameSync structure in AVFilter.preinit
 * via the *_framesync_preinit function defined alongside it. */
#define FRAMESYNC_AUXILIARY_FUNCS(func_prefix, context, field)        \
static int func_prefix##_framesync_preinit(AVFilterContext *ctx)      \
{                                                                     \
    context *s = ctx->priv; \
    ff_framesync_preinit(&s->field); \
    return 0; \
} \
static void *func_prefix##_child_next(void *obj, void *prev)          \
{                                                                     \
    context *s = obj; \
    return prev ? NULL : &s->field; \
}

#define FRAMESYNC_DEFINE_CLASS_EXT(name, context, field, options)     \
FRAMESYNC_AUXILIARY_FUNCS(name, context, field)                       \
FRAMESYNC_DEFINE_PURE_CLASS(name, #name, name, options)

#define FRAMESYNC_DEFINE_CLASS(name, context, field)                  \
FRAMESYNC_DEFINE_CLASS_EXT(name, context, field, name##_options)

#endif /* AVFILTER_FRAMESYNC_H */
