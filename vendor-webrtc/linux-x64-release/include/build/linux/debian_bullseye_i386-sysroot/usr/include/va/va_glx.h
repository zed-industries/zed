/*
 * Copyright (C) 2009 Splitted-Desktop Systems. All Rights Reserved.
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
 * IN NO EVENT SHALL PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#ifndef VA_GLX_H
#define VA_GLX_H

#include <va/va.h>
#include <GL/glx.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Return a suitable VADisplay for VA API
 *
 * @param[in] dpy the X11 display
 * @return a VADisplay
 */
VADisplay vaGetDisplayGLX(
    Display *dpy
);

/**
 * Create a surface used for display to OpenGL
 *
 * The application shall maintain the live GLX context itself.
 * Implementations are free to use glXGetCurrentContext() and
 * glXGetCurrentDrawable() functions for internal purposes.
 *
 * @param[in]  dpy        the VA display
 * @param[in]  target     the GL target to which the texture needs to be bound
 * @param[in]  texture    the GL texture
 * @param[out] gl_surface the VA/GLX surface
 * @return VA_STATUS_SUCCESS if successful
 */
VAStatus vaCreateSurfaceGLX(
    VADisplay dpy,
    GLenum    target,
    GLuint    texture,
    void    **gl_surface
);

/**
 * Destroy a VA/GLX surface
 *
 * The application shall maintain the live GLX context itself.
 * Implementations are free to use glXGetCurrentContext() and
 * glXGetCurrentDrawable() functions for internal purposes.
 *
 * @param[in]  dpy        the VA display
 * @param[in]  gl_surface the VA surface
 * @return VA_STATUS_SUCCESS if successful
 */
VAStatus vaDestroySurfaceGLX(
    VADisplay dpy,
    void     *gl_surface
);

/**
 * Copy a VA surface to a VA/GLX surface
 *
 * This function will not return until the copy is completed. At this
 * point, the underlying GL texture will contain the surface pixels
 * in an RGB format defined by the user.
 *
 * The application shall maintain the live GLX context itself.
 * Implementations are free to use glXGetCurrentContext() and
 * glXGetCurrentDrawable() functions for internal purposes.
 *
 * @param[in]  dpy        the VA display
 * @param[in]  gl_surface the VA/GLX destination surface
 * @param[in]  surface    the VA source surface
 * @param[in]  flags      the PutSurface flags
 * @return VA_STATUS_SUCCESS if successful
 */
VAStatus vaCopySurfaceGLX(
    VADisplay    dpy,
    void        *gl_surface,
    VASurfaceID  surface,
    unsigned int flags
);

#ifdef __cplusplus
}
#endif

#endif /* VA_GLX_H */
