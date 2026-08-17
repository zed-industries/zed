// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_IMAGE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_IMAGE_CACHE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class FetchParameters;
class ImageResourceContent;
class ResourceFetcher;

// A per-StyleEngine cache for ImageResourceContent for StyleImages. A
// CSSImageValue points to a StyleImage, but different CSSImageValue objects
// with the same URL may not have shared the same ImageResourceContent without
// this cache.
class CORE_EXPORT StyleImageCache {
  DISALLOW_NEW();

 public:
  StyleImageCache() = default;

  // Look up an existing ImageResourceContent in the cache, or create a new one,
  // add it to the cache, and start the fetch.
  ImageResourceContent* CacheImageContent(ResourceFetcher*, FetchParameters&);

  void Trace(Visitor*) const;

 private:
  // Map from resolved URL (string) to ImageResourceContent. A weak reference
  // makes sure the entry is removed when no style declarations nor computed
  // styles have a reference to the image.
  HeapHashMap<String, WeakMember<ImageResourceContent>> fetched_image_map_;

  friend class StyleImageCacheTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_IMAGE_CACHE_H_
