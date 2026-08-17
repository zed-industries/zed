// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FONT_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FONT_CACHE_H_

#include "third_party/blink/renderer/platform/fonts/font_metrics.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

struct HarfBuzzFontData;

// Though we have FontCache class, which provides the cache mechanism for
// WebKit's font objects, we also need additional caching layer for HarfBuzz to
// reduce the number of hb_font_t objects created. Without it, we would create
// an hb_font_t object for every FontPlatformData object. But insted, we only
// need one for each unique SkTypeface.
// FIXME, crbug.com/609099: We should fix the FontCache to only keep one
// FontPlatformData object independent of size, then consider using this here.

class HarfBuzzFontCache final {
  DISALLOW_NEW();

 public:
  void Trace(Visitor* visitor) const;
  // See "harfbuzz_face.cc" for |HarfBuzzFontCache::GetOrCreateFontData()|
  // implementation.
  HarfBuzzFontData* GetOrCreate(uint64_t unique_id,
                                const FontPlatformData* platform_data);

 private:
  HeapHashMap<uint64_t,
              WeakMember<HarfBuzzFontData>,
              IntWithZeroKeyHashTraits<uint64_t>>
      font_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FONT_CACHE_H_
