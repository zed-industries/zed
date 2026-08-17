// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_STALE_REVALIDATION_RESOURCE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_STALE_REVALIDATION_RESOURCE_CLIENT_H_

#include "third_party/blink/renderer/platform/loader/fetch/raw_resource.h"

namespace blink {

// Stale Revalidation Resources are requests to the network stack without the
// allow staleness bit set on. This should cause the network stack's http cache
// to revalidate the resource. When the request has been completed the original
// resource will be removed from the memory cache.
class StaleRevalidationResourceClient
    : public GarbageCollected<StaleRevalidationResourceClient>,
      public RawResourceClient {
 public:
  explicit StaleRevalidationResourceClient(Resource* stale_resource);
  ~StaleRevalidationResourceClient() override;

  // RawResourceClient overloads.
  void NotifyFinished(Resource* resource) override;
  void Trace(Visitor* visitor) const override;
  String DebugName() const override;

 private:
  // |stale_resource_| is the original resource that will be removed from the
  // MemoryCache when this revalidation request is completed. Note that it is
  // different than the active resource for this resource client which accessed
  // via |GetResource()|.
  WeakMember<Resource> stale_resource_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_STALE_REVALIDATION_RESOURCE_CLIENT_H_
