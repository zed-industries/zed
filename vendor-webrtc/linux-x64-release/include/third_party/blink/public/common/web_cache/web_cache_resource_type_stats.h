// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_CACHE_WEB_CACHE_RESOURCE_TYPE_STATS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_CACHE_WEB_CACHE_RESOURCE_TYPE_STATS_H_

#include <stddef.h>

namespace blink {

// A struct mirroring blink::MemoryCache::TypeStatistic.
struct WebCacheResourceTypeStat {
  size_t count;
  size_t size;
  size_t decoded_size;
};

// A struct mirroring blink::MemoryCache::Statistics.
struct WebCacheResourceTypeStats {
  WebCacheResourceTypeStat images;
  WebCacheResourceTypeStat css_style_sheets;
  WebCacheResourceTypeStat scripts;
  WebCacheResourceTypeStat xsl_style_sheets;
  WebCacheResourceTypeStat fonts;
  WebCacheResourceTypeStat other;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_CACHE_WEB_CACHE_RESOURCE_TYPE_STATS_H_
