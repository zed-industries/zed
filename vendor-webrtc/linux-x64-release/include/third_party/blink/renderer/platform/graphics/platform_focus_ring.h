// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PLATFORM_FOCUS_RING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PLATFORM_FOCUS_RING_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkColor.h"

class SkPath;
class SkRRect;

namespace cc {
class PaintCanvas;
}

namespace blink {

PLATFORM_EXPORT void DrawPlatformFocusRing(const SkRRect&,
                                           cc::PaintCanvas*,
                                           SkColor4f,
                                           float width);
PLATFORM_EXPORT void DrawPlatformFocusRing(const SkPath&,
                                           cc::PaintCanvas*,
                                           SkColor4f,
                                           float width,
                                           float corner_radius);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PLATFORM_FOCUS_RING_H_
