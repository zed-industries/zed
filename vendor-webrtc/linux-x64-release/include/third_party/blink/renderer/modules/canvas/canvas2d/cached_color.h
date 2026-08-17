// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CACHED_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CACHED_COLOR_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/modules/canvas/canvas2d/canvas_style.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8-local-handle.h"
#include "v8/include/v8-primitive.h"

namespace blink {

// Used by Canvas2DRecorderContext to track cached colors.
struct CachedColor final : public GarbageCollected<CachedColor> {
  CachedColor(v8::Isolate* isolate,
              const v8::Local<v8::String>& color_string,
              const Color& color,
              ColorParseResult parse_result)
      : color_string(isolate, color_string),
        color(color),
        parse_result(parse_result),
        hash_code(color_string->GetIdentityHash()) {
    // Color-mix is not cached.
    DCHECK_NE(parse_result, ColorParseResult::kColorFunction);
  }

  void Trace(Visitor* visitor) const { visitor->Trace(color_string); }

  TraceWrapperV8Reference<v8::String> color_string;
  Color color;
  ColorParseResult parse_result;
  // There are two options to get the hash:
  // 1. Keep around the isolate so that we can use it with `color_string` and
  //    then call GetIdentityHash().
  // 2. Cache it.
  // 2 is chosen as the hash is the only thing we need from `color_string`, so
  // no point in keeping it around.
  unsigned hash_code;
};

// Allows using CachedColor in a HashMap.
struct CachedColorTraits final
    : public WTF::BaseMemberHashTraits<CachedColor, Member<CachedColor>> {
  STATIC_ONLY(CachedColorTraits);
  static unsigned GetHash(const CachedColor* cached_color) {
    return cached_color->hash_code;
  }
  static bool Equal(const CachedColor* a, const CachedColor* b) {
    return a->color_string == b->color_string;
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

// Used for looking up CachedColors by v8::String.
struct ColorCacheHashTranslator final {
  STATIC_ONLY(ColorCacheHashTranslator);
  static unsigned GetHash(const v8::Local<v8::String>& string) {
    return string->GetIdentityHash();
  }
  static bool Equal(const CachedColor* cached_color,
                    const v8::Local<v8::String>& string) {
    return cached_color->color_string == string;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CACHED_COLOR_H_
