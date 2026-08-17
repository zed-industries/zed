// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_IMAGEBITMAP_IMAGE_BITMAP_SOURCE_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_IMAGEBITMAP_IMAGE_BITMAP_SOURCE_UTIL_H_

#include <optional>

#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/skia/include/core/SkBitmap.h"

namespace blink {

class ExceptionState;
class ScriptState;

// Returns the SkBitmap data from a V8ImageBitmapSource. Throws an exception
// and returns nullopt if the source is inaccessible, incompatible, etc.
// https://html.spec.whatwg.org/C/#imagebitmapsource
std::optional<SkBitmap> GetBitmapFromV8ImageBitmapSource(
    ScriptState* script_state,
    const V8ImageBitmapSource* image_source,
    ExceptionState& exception_state);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_IMAGEBITMAP_IMAGE_BITMAP_SOURCE_UTIL_H_
