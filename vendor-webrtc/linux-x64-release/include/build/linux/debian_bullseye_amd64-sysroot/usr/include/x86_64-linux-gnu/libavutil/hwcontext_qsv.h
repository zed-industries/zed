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

#ifndef AVUTIL_HWCONTEXT_QSV_H
#define AVUTIL_HWCONTEXT_QSV_H

#include <mfx/mfxvideo.h>

/**
 * @file
 * An API-specific header for AV_HWDEVICE_TYPE_QSV.
 *
 * This API does not support dynamic frame pools. AVHWFramesContext.pool must
 * contain AVBufferRefs whose data pointer points to an mfxFrameSurface1 struct.
 */

/**
 * This struct is allocated as AVHWDeviceContext.hwctx
 */
typedef struct AVQSVDeviceContext {
    mfxSession session;
} AVQSVDeviceContext;

/**
 * This struct is allocated as AVHWFramesContext.hwctx
 */
typedef struct AVQSVFramesContext {
    mfxFrameSurface1 *surfaces;
    int            nb_surfaces;

    /**
     * A combination of MFX_MEMTYPE_* describing the frame pool.
     */
    int frame_type;
} AVQSVFramesContext;

#endif /* AVUTIL_HWCONTEXT_QSV_H */

