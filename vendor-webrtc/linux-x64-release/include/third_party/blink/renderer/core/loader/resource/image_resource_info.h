// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_INFO_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_error.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_status.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class ResourceError;
class ResourceFetcher;
class ResourceResponse;

// Delegate class of ImageResource that encapsulates the interface and data
// visible to ImageResourceContent.
// Do not add new members or new call sites unless really needed.
// TODO(hiroshige): reduce the members of this class to further decouple
// ImageResource and ImageResourceContent.
class CORE_EXPORT ImageResourceInfo : public GarbageCollectedMixin {
 public:
  ~ImageResourceInfo() = default;
  virtual const KURL& Url() const = 0;
  virtual base::TimeTicks LoadResponseEnd() const = 0;
  virtual base::TimeTicks LoadStart() const = 0;
  virtual base::TimeTicks LoadEnd() const = 0;
  virtual base::TimeTicks DiscoveryTime() const = 0;
  virtual const ResourceResponse& GetResponse() const = 0;
  virtual bool IsCacheValidator() const = 0;
  enum DoesCurrentFrameHaveSingleSecurityOrigin {
    kHasMultipleSecurityOrigin,
    kHasSingleSecurityOrigin
  };
  virtual bool IsAccessAllowed(
      DoesCurrentFrameHaveSingleSecurityOrigin) const = 0;
  virtual bool HasCacheControlNoStoreHeader() const = 0;
  virtual std::optional<ResourceError> GetResourceError() const = 0;

  // TODO(hiroshige): Remove this once MemoryCache becomes further weaker.
  virtual void SetDecodedSize(size_t) = 0;

  // TODO(hiroshige): Remove these.
  virtual void WillAddClientOrObserver() = 0;
  virtual void DidRemoveClientOrObserver() = 0;

  // TODO(hiroshige): Remove this. crbug.com/666214
  virtual void EmulateLoadStartedForInspector(
      ResourceFetcher*,
      const AtomicString& initiator_name) = 0;

  virtual void LoadDeferredImage(ResourceFetcher* fetcher) = 0;

  virtual bool IsAdResource() const = 0;

  virtual const HashSet<String>* GetUnsupportedImageMimeTypes() const = 0;

  virtual std::optional<WebURLRequest::Priority> RequestPriority() const = 0;

  void Trace(Visitor* visitor) const override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_INFO_H_
