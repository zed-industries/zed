// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_IMAGE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_IMAGE_CACHE_H_

#include "third_party/blink/renderer/platform/geometry/geometry_hash_traits.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/skia/include/core/SkRect.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace cc {
class ColorFilter;
}

namespace blink {

// DarkModeImageCache - Implements dark mode filter cache for different |src|
// rects from the image.
class PLATFORM_EXPORT DarkModeImageCache {
 public:
  DarkModeImageCache() = default;
  DarkModeImageCache(const DarkModeImageCache&) = delete;
  DarkModeImageCache& operator=(const DarkModeImageCache&) = delete;
  ~DarkModeImageCache() = default;

  bool Exists(const SkIRect& src) { return cache_.Contains(src); }

  sk_sp<cc::ColorFilter> Get(const SkIRect& src) {
    auto result = cache_.find(src);
    return (result != cache_.end()) ? result->value : nullptr;
  }

  void Add(const SkIRect& src, sk_sp<cc::ColorFilter> dark_mode_color_filter) {
    DCHECK(!Exists(src));

    cache_.insert(src, std::move(dark_mode_color_filter));
  }

  size_t Size() { return cache_.size(); }

  void Clear() { cache_.clear(); }

 private:
  HashMap<SkIRect, sk_sp<cc::ColorFilter>> cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_IMAGE_CACHE_H_
