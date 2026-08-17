// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_V8_CANVAS_STYLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_V8_CANVAS_STYLE_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "v8/include/v8-local-handle.h"
#include "v8/include/v8-value.h"

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink {

class CanvasGradient;
class CanvasPattern;
class CanvasStyle;
class ExceptionState;
class ScriptState;

// Types of values supported for canvas style.
enum class V8CanvasStyleType {
  kCSSColorValue,
  kGradient,
  kPattern,
  kString,
};

// Temporary structure used when extracting the canvas style from v8.
struct MODULES_EXPORT V8CanvasStyle {
  STACK_ALLOCATED();

 public:
  V8CanvasStyleType type;
  CanvasPattern* pattern = nullptr;
  CanvasGradient* gradient = nullptr;
  Color css_color_value = Color::kTransparent;
  AtomicString string;
};

// Sets `style` from v8. Returns true on success, false if there is a conversion
// error.
MODULES_EXPORT bool ExtractV8CanvasStyle(v8::Isolate* isolate,
                                         v8::Local<v8::Value> value,
                                         V8CanvasStyle& style,
                                         ExceptionState& exception_state);

// Converts `style` to a v8 value.
MODULES_EXPORT v8::Local<v8::Value> CanvasStyleToV8(ScriptState* script_state,
                                                    const CanvasStyle& style);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_V8_CANVAS_STYLE_H_
