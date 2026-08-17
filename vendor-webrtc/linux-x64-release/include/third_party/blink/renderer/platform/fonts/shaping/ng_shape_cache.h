/*
 * Copyright (C) 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2015 Google Inc. All rights reserved.
 * Copyright (C) 2023 Igalia S.L. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_NG_SHAPE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_NG_SHAPE_CACHE_H_

#include "base/hash/hash.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/platform/fonts/shaping/shape_result.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class NGShapeCache : public GarbageCollected<NGShapeCache> {
 public:
  static constexpr unsigned kMaxTextLengthOfEntries = 30;
  static constexpr unsigned kMaxSize = 2048;

  explicit NGShapeCache(const SimpleFontData* primary_font)
      : primary_font_(primary_font) {
    DCHECK(RuntimeEnabledFeatures::LayoutNGShapeCacheEnabled());
  }
  NGShapeCache(const NGShapeCache&) = delete;
  NGShapeCache& operator=(const NGShapeCache&) = delete;

  void Trace(Visitor* visitor) const {
    visitor->Trace(ltr_string_map_);
    visitor->Trace(rtl_string_map_);
    visitor->Trace(primary_font_);
  }

  template <typename ShapeResultFunc>
  const ShapeResult* GetOrCreate(const String& text,
                                 TextDirection direction,
                                 const ShapeResultFunc& shape_result_func) {
    if (text.length() > kMaxTextLengthOfEntries) {
      return shape_result_func();
    }

    auto& map = IsLtr(direction) ? ltr_string_map_ : rtl_string_map_;
    if (map.size() >= kMaxSize) [[unlikely]] {
      const auto it = map.find(text);
      return (it != map.end() && it->value) ? it->value.Get()
                                            : shape_result_func();
    }

    const auto add_result = map.insert(text, nullptr);
    if (add_result.stored_value->value) {
      return add_result.stored_value->value;
    }

    const ShapeResult* result = shape_result_func();

    // Only shape-results without font-fallback are valid, because the cache is
    // in the `primary_font_`.
    if (!result->HasFallbackFonts(primary_font_)) {
      add_result.stored_value->value = result;
    }

    return result;
  }

 private:
  typedef HeapHashMap<String, WeakMember<const ShapeResult>> SmallStringMap;

  SmallStringMap ltr_string_map_;
  SmallStringMap rtl_string_map_;
  Member<const SimpleFontData> primary_font_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_NG_SHAPE_CACHE_H_
