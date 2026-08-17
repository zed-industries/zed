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

#ifndef AVFILTER_STACK_INTERNAL_H
#define AVFILTER_STACK_INTERNAL_H

enum {
    STACK_H = 0,
    STACK_V = 1,
    STACK_X = 2
};

typedef struct StackItemRegion {
    int x;
    int y;
    int width;
    int height;
} StackItemRegion;

typedef struct StackBaseContext {
    HWContext hwctx; /**< must be the first field */

    FFFrameSync fs;
    int mode;
    uint8_t fillcolor[4];
    int fillcolor_enable;
    StackItemRegion *regions;

    /* Options */
    int nb_inputs;
    int shortest;
    int tile_width;
    int tile_height;
    int nb_grid_columns;
    int nb_grid_rows;
    char *layout;
    char *fillcolor_str;
} StackBaseContext;

static int config_comm_output(AVFilterLink *outlink);
static int stack_init(AVFilterContext *avctx);
static av_cold void stack_uninit(AVFilterContext *avctx);
static int stack_activate(AVFilterContext *avctx);

#endif /* AVFILTER_STACK_INTERNAL_H */
