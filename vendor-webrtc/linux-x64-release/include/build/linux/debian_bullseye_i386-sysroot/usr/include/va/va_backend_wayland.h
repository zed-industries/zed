/*
 * va_backend_wayland.h - VA driver implementation hooks for Wayland
 *
 * Copyright (c) 2012 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#ifndef VA_BACKEND_WAYLAND_H
#define VA_BACKEND_WAYLAND_H

#include <va/va.h>
#include <wayland-client.h>

/** \brief VA/Wayland API version. */
#define VA_WAYLAND_API_VERSION  (0x574c4400) /* WLD0 */

/* Forward declarations */
struct VADriverContext;

/** \brief VA/Wayland implementation hooks. */
struct VADriverVTableWayland {
    /**
     * \brief Interface version.
     *
     * Implementations shall set this field to \ref VA_WAYLAND_API_VERSION.
     */
    unsigned int version;

    /** \brief Hook to return Wayland buffer associated with the VA surface. */
    VAStatus(*vaGetSurfaceBufferWl)(
        struct VADriverContext *ctx,
        VASurfaceID             surface,
        unsigned int            flags,
        struct wl_buffer      **out_buffer
    );

    /** \brief Hook to return Wayland buffer associated with the VA image. */
    VAStatus(*vaGetImageBufferWl)(
        struct VADriverContext *ctx,
        VAImageID               image,
        unsigned int            flags,
        struct wl_buffer      **out_buffer
    );

    /** \brief Indicate whether buffer sharing with prime fd is supported. */
    unsigned int has_prime_sharing;

    /**
     * Pointer to an implementation of struct wl_interface
     *
     * It is set by libva-wayland when a context is created, then the backend
     * driver may reuse it.
     */
    const void *wl_interface;

    /** \brief Reserved bytes for future use, must be zero */
    unsigned long reserved[7];
};

#endif /* VA_BACKEND_WAYLAND_H */
