// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BACK_FORWARD_CACHE_LOADER_HELPER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BACK_FORWARD_CACHE_LOADER_HELPER_IMPL_H_

#include <memory>

#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-blink.h"
#include "third_party/blink/public/mojom/navigation/renderer_eviction_reason.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/back_forward_cache_loader_helper.h"

namespace blink {

class SourceLocation;

class BackForwardCacheLoaderHelperImpl : public BackForwardCacheLoaderHelper {
 public:
  // A delegate to notify the loader states to the back-forward cache.
  class Delegate : public GarbageCollectedMixin {
   public:
    virtual ~Delegate() = default;

    // Triggers eviction of this delegate by notifying the browser side.
    // We want to capture the source location only when the eviction reason is
    // JavaScript execution, and this delegate shouldn't be called for
    // JavaScript execution. Therefore, nullptr should be passed as
    // source_location.
    virtual void EvictFromBackForwardCache(
        mojom::blink::RendererEvictionReason reason,
        std::unique_ptr<SourceLocation> source_location) = 0;

    // Called when a network request buffered an additional `num_bytes` while
    // the delegate is in back-forward cache. Updates the total amount of bytes
    // buffered for back-forward cache in the delegate and in the process. Note
    // that `num_bytes` is the amount of additional bytes that are newly
    // buffered, on top of any previously buffered bytes for this delegate.
    virtual void DidBufferLoadWhileInBackForwardCache(
        bool update_process_wide_count,
        size_t num_bytes) = 0;
  };

  explicit BackForwardCacheLoaderHelperImpl(Delegate& delegate);

  void EvictFromBackForwardCache(
      mojom::blink::RendererEvictionReason reason) override;
  void DidBufferLoadWhileInBackForwardCache(bool update_process_wide_count,
                                            size_t num_bytes) override;
  void Detach() override;
  void Trace(Visitor*) const override;

 private:
  WeakMember<Delegate> delegate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_BACK_FORWARD_CACHE_LOADER_HELPER_IMPL_H_
