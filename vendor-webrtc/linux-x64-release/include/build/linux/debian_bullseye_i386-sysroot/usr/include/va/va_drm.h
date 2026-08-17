/*
 * va_drm.h - Raw DRM API
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

#ifndef VA_DRM_H
#define VA_DRM_H

#include <va/va.h>

/**
 * \file va_drm.h
 * \brief The raw DRM API
 *
 * This file contains the \ref api_drm "Raw DRM API".
 */

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Returns a VA display derived from the specified DRM connection.
 *
 * This function returns a (possibly cached) VA display from the
 * specified DRM connection @fd.
 *
 * @param[in]   fd      the DRM connection descriptor
 * @return the VA display
 */
VADisplay
vaGetDisplayDRM(int fd);

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_DRM_H */
