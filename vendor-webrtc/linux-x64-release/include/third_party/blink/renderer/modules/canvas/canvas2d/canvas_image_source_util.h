// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_IMAGE_SOURCE_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_IMAGE_SOURCE_UTIL_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class CanvasImageSource;
class ExceptionState;

MODULES_EXPORT CanvasImageSource* ToCanvasImageSource(
    const V8CanvasImageSource* value,
    ExceptionState& exception_state);

bool WouldTaintCanvasOrigin(CanvasImageSource* image_source);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_IMAGE_SOURCE_UTIL_H_
