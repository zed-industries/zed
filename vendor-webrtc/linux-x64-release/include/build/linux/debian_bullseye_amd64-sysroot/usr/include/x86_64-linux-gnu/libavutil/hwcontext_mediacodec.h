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

#ifndef AVUTIL_HWCONTEXT_MEDIACODEC_H
#define AVUTIL_HWCONTEXT_MEDIACODEC_H

/**
 * MediaCodec details.
 *
 * Allocated as AVHWDeviceContext.hwctx
 */
typedef struct AVMediaCodecDeviceContext {
    /**
     * android/view/Surface handle, to be filled by the user.
     *
     * This is the default surface used by decoders on this device.
     */
    void *surface;
} AVMediaCodecDeviceContext;

#endif /* AVUTIL_HWCONTEXT_MEDIACODEC_H */
