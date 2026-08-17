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

#ifndef AVFILTER_METAL_UTILS_H
#define AVFILTER_METAL_UTILS_H

#include <Metal/Metal.h>

#include <AvailabilityMacros.h>

// CoreVideo accidentally(?) preprocessor-gates Metal functionality
// on MAC_OS_X_VERSION_MIN_REQUIRED >= 101100 (FB9816002).
// There doesn't seem to be any particular reason for this,
// so here we temporarily redefine it to at least that value
// so CV will give us CVMetalTextureRef and the related functions.

#if defined(MAC_OS_X_VERSION_MIN_REQUIRED) && (MAC_OS_X_VERSION_MIN_REQUIRED < 101100)
#define ORIG_MAC_OS_X_VERSION_MIN_REQUIRED MAC_OS_X_VERSION_MIN_REQUIRED
#undef MAC_OS_X_VERSION_MIN_REQUIRED
#define MAC_OS_X_VERSION_MIN_REQUIRED 101100
#endif

#include <CoreVideo/CoreVideo.h>

#ifdef ORIG_MAC_OS_X_VERSION_MIN_REQUIRED
#undef MAC_OS_X_VERSION_MIN_REQUIRED
#define MAC_OS_X_VERSION_MIN_REQUIRED ORIG_MAC_OS_X_VERSION_MIN_REQUIRED
#undef ORIG_MAC_OS_X_VERSION_MIN_REQUIRED
#endif

void ff_metal_compute_encoder_dispatch(id<MTLDevice> device,
                                       id<MTLComputePipelineState> pipeline,
                                       id<MTLComputeCommandEncoder> encoder,
                                       NSUInteger width, NSUInteger height)
                                       API_AVAILABLE(macos(10.11), ios(8.0));

CVMetalTextureRef ff_metal_texture_from_pixbuf(void *avclass,
                                               CVMetalTextureCacheRef textureCache,
                                               CVPixelBufferRef pixbuf,
                                               int plane,
                                               MTLPixelFormat format)
                                               API_AVAILABLE(macos(10.11), ios(8.0));

#endif /* AVFILTER_METAL_UTILS_H */
