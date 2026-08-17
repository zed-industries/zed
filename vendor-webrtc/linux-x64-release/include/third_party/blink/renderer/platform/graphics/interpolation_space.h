/*
 * Copyright (C) 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_INTERPOLATION_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_INTERPOLATION_SPACE_H_

#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace cc {
class ColorFilter;
}

namespace blink {

enum InterpolationSpace {
  // Interpolation is performed on (assumed to be sRGB) pixel values directly,
  // so a halfway interpolation of 0 and 255 is 127.
  kInterpolationSpaceSRGB,
  // Interpolation is performed in linear physical value space, so a halfway
  // interpolation of 0 and 255 is 188.
  kInterpolationSpaceLinear
};

namespace interpolation_space_utilities {

// Convert a Color assumed to be in the |src_interpolation_space| into the
// |dst_interpolation_space|.
Color ConvertColor(
    const Color& src_color,
    InterpolationSpace dst_interpolation_space,
    InterpolationSpace src_interpolation_space = kInterpolationSpaceSRGB);

// Create a color filter that will convert from |src_interpolation_space| into
// |dst_interpolation_space|.
sk_sp<cc::ColorFilter> CreateInterpolationSpaceFilter(
    InterpolationSpace src_interpolation_space,
    InterpolationSpace dst_interpolation_space);

}  // namespace interpolation_space_utilities

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_INTERPOLATION_SPACE_H_
