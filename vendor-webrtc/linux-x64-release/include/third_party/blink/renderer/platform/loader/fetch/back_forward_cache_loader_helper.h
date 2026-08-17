// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BACK_FORWARD_CACHE_LOADER_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BACK_FORWARD_CACHE_LOADER_HELPER_H_

#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-forward.h"
#include "third_party/blink/public/mojom/navigation/renderer_eviction_reason.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Helper class for in-flight network request support for back-forward cache.
class PLATFORM_EXPORT BackForwardCacheLoaderHelper
    : public GarbageCollected<BackForwardCacheLoaderHelper> {
 public:
  // Evict the page from BackForwardCache. Should be called when handling an
  // event which can't proceed if the page is in BackForwardCache and can't be
  // easily deferred to handle later, for example network redirect handling.
  virtual void EvictFromBackForwardCache(
      mojom::blink::RendererEvictionReason reason) = 0;

  // Called when a network request buffered an additional `num_bytes` while the
  // in back-forward cache. May be called multiple times.
  virtual void DidBufferLoadWhileInBackForwardCache(
      bool update_process_wide_count,
      size_t num_bytes) = 0;

  virtual void Detach() = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BACK_FORWARD_CACHE_LOADER_HELPER_H_
