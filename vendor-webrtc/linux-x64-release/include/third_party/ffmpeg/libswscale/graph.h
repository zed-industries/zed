/*
 * Copyright (C) 2024 Niklas Haas
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

#ifndef SWSCALE_GRAPH_H
#define SWSCALE_GRAPH_H

#include "libavutil/slicethread.h"
#include "swscale.h"
#include "format.h"

/**
 * Represents a view into a single field of frame data.
 */
typedef struct SwsImg {
    enum AVPixelFormat fmt;
    uint8_t *data[4]; /* points to y=0 */
    int linesize[4];
} SwsImg;

typedef struct SwsPass  SwsPass;
typedef struct SwsGraph SwsGraph;

/**
 * Output `h` lines of filtered data. `out` and `in` point to the
 * start of the image buffer for this pass.
 */
typedef void (*sws_filter_run_t)(const SwsImg *out, const SwsImg *in,
                                 int y, int h, const SwsPass *pass);

/**
 * Represents a single filter pass in the scaling graph. Each filter will
 * read from some previous pass's output, and write to a buffer associated
 * with the pass (or into the final output image).
 */
struct SwsPass {
    const SwsGraph *graph;

    /**
     * Filter main execution function. Called from multiple threads, with
     * the granularity dictated by `slice_h`. Individual slices sent to `run`
     * are always equal to (or smaller than, for the last slice) `slice_h`.
     */
    sws_filter_run_t run;
    enum AVPixelFormat format; /* new pixel format */
    int width, height; /* new output size */
    int slice_h;       /* filter granularity */
    int num_slices;

    /**
     * Filter input. This pass's output will be resolved to form this pass's.
     * input. If NULL, the original input image is used.
     */
    const SwsPass *input;

    /**
     * Filter output buffer. Allocated on demand and freed automatically.
     */
    SwsImg output;

    /**
     * Called once from the main thread before running the filter. Optional.
     * `out` and `in` always point to the main image input/output, regardless
     * of `input` and `output` fields.
     */
    void (*setup)(const SwsImg *out, const SwsImg *in, const SwsPass *pass);

    /**
     * Optional private state and associated free() function.
     */
    void (*free)(void *priv);
    void *priv;
};

/**
 * Filter graph, which represents a 'baked' pixel format conversion.
 */
typedef struct SwsGraph {
    SwsContext *ctx;
    AVSliceThread *slicethread;
    int num_threads; /* resolved at init() time */
    int incomplete;  /* set during init() if formats had to be inferred */
    int noop;        /* set during init() if the graph is a no-op */

    /** Sorted sequence of filter passes to apply */
    SwsPass **passes;
    int num_passes;

    /**
     * Cached copy of the public options that were used to construct this
     * SwsGraph. Used only to detect when the graph needs to be reinitialized.
     */
    SwsContext opts_copy;

    /**
     * Currently active format and processing parameters.
     */
    SwsFormat src, dst;
    int field;

    /** Temporary execution state inside ff_sws_graph_run */
    struct {
        const SwsPass *pass; /* current filter pass */
        SwsImg input;
        SwsImg output;
    } exec;
} SwsGraph;

/**
 * Allocate and initialize the filter graph. Returns 0 or a negative error.
 */
int ff_sws_graph_create(SwsContext *ctx, const SwsFormat *dst, const SwsFormat *src,
                        int field, SwsGraph **out_graph);

/**
 * Uninitialize any state associate with this filter graph and free it.
 */
void ff_sws_graph_free(SwsGraph **graph);

/**
 * Update dynamic per-frame HDR metadata without requiring a full reinit.
 */
void ff_sws_graph_update_metadata(SwsGraph *graph, const SwsColor *color);

/**
 * Wrapper around ff_sws_graph_create() that reuses the existing graph if the
 * format is compatible. This will also update dynamic per-frame metadata.
 * Must be called after changing any of the fields in `ctx`, or else they will
 * have no effect.
 */
int ff_sws_graph_reinit(SwsContext *ctx, const SwsFormat *dst, const SwsFormat *src,
                        int field, SwsGraph **graph);

/**
 * Dispatch the filter graph on a single field. Internally threaded.
 */
void ff_sws_graph_run(SwsGraph *graph, uint8_t *const out_data[4],
                      const int out_linesize[4],
                      const uint8_t *const in_data[4],
                      const int in_linesize[4]);

#endif /* SWSCALE_GRAPH_H */
