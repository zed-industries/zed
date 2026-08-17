/*
 * Copyright (C) 2004, 2005, 2006 Apple Computer, Inc.  All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_TYPES_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

using DynamicRangeLimit = ::cc::PaintFlags::DynamicRangeLimitMixture;

enum InterpolationQuality {
  kInterpolationNone = static_cast<int>(cc::PaintFlags::FilterQuality::kNone),
  kInterpolationLow = static_cast<int>(cc::PaintFlags::FilterQuality::kLow),
  kInterpolationMedium =
      static_cast<int>(cc::PaintFlags::FilterQuality::kMedium),
};

enum AntiAliasingMode { kNotAntiAliased, kAntiAliased };

enum TextPaintOrder { kFillStroke, kStrokeFill };

enum TextDrawingMode {
  kTextModeFill = 1 << 0,
  kTextModeStroke = 1 << 1,
};
typedef unsigned TextDrawingModeFlags;

PLATFORM_EXPORT InterpolationQuality GetDefaultInterpolationQuality();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_TYPES_H_
