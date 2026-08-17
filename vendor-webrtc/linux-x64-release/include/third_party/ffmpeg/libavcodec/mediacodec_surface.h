/*
 * Android MediaCodec Surface functions
 *
 * Copyright (c) 2016 Matthieu Bouron <matthieu.bouron stupeflix.com>
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

#ifndef AVCODEC_MEDIACODEC_SURFACE_H
#define AVCODEC_MEDIACODEC_SURFACE_H

#include "libavcodec/avcodec.h"

typedef struct FFANativeWindow {
    void *surface;
    void *native_window;
} FFANativeWindow;

FFANativeWindow *ff_mediacodec_surface_ref(void *surface, void *native_window, void *log_ctx);
int ff_mediacodec_surface_unref(FFANativeWindow *window, void *log_ctx);

#endif /* AVCODEC_MEDIACODEC_SURFACE_H */
