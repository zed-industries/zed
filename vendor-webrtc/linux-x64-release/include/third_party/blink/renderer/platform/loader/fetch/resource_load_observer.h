// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_OBSERVER_H_

#include <inttypes.h>

#include "base/containers/span_or_size.h"
#include "base/types/strong_alias.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class KURL;
class FetchParameters;
class ResourceError;
class ResourceRequest;
class ResourceResponse;
enum class ResourceType : uint8_t;

// ResourceLoadObserver is a collection of functions which meet following
// conditions.
// 1. It's not possible to implement it in platform/loader.
// 2. It's about individual loading operation. For example, a function
//    notifying that all requests are gone would not belong to this class.
// 3. It is called when the loader gets some input, typically from the network
//    subsystem. There may be some cases where the source of the input is not
//    from network - For example, this class may have a function which is called
//    when ResourceFetcher::RequestResource is called. On the other hand, this
//    class will not have "operation"s, such as PrepareRequest.
//
// All functions except for the destructor and the trace method must be pure
// virtual, and must not be called when the associated fetcher is detached.
class PLATFORM_EXPORT ResourceLoadObserver
    : public GarbageCollected<ResourceLoadObserver> {
 public:
  virtual ~ResourceLoadObserver() = default;

  // Called when ResourceFetcher::RequestResource is called.
  virtual void DidStartRequest(const FetchParameters&, ResourceType) = 0;

  // Called when the request is about to be sent. This is called on initial and
  // every redirect request.
  virtual void WillSendRequest(const ResourceRequest&,
                               const ResourceResponse& redirect_response,
                               ResourceType,
                               const ResourceLoaderOptions&,
                               RenderBlockingBehavior,
                               const Resource*) = 0;

  // Called when the priority of the request changes.
  virtual void DidChangePriority(uint64_t identifier,
                                 ResourceLoadPriority,
                                 int intra_priority_value) = 0;

  enum ResponseSource { kFromMemoryCache, kNotFromMemoryCache };
  // Called when a response is received.
  // |request| and |resource| are provided separately because when it's from
  // the memory cache |request| and |resource->GetResourceRequest()| don't
  // match. |response| may not yet be set to |resource| when this function is
  // called.
  virtual void DidReceiveResponse(uint64_t identifier,
                                  const ResourceRequest& request,
                                  const ResourceResponse& response,
                                  const Resource* resource,
                                  ResponseSource) = 0;

  // Called when a response body chunk is received.
  virtual void DidReceiveData(uint64_t identifier,
                              base::SpanOrSize<const char> chunk) = 0;

  // Called when receiving an update for "network transfer size" for a request.
  virtual void DidReceiveTransferSizeUpdate(uint64_t identifier,
                                            int transfer_size_diff) = 0;

  // Called when receiving a Blob as a response.
  virtual void DidDownloadToBlob(uint64_t identifier, BlobDataHandle*) = 0;

  // Called when a request finishes successfully.
  virtual void DidFinishLoading(uint64_t identifier,
                                base::TimeTicks finish_time,
                                int64_t encoded_data_length,
                                int64_t decoded_body_length) = 0;

  using IsInternalRequest = base::StrongAlias<class IsInternalRequestTag, bool>;
  // Called when a request fails.
  virtual void DidFailLoading(const KURL&,
                              uint64_t identifier,
                              const ResourceError&,
                              int64_t encoded_data_length,
                              IsInternalRequest) = 0;

  // Called when the RenderBlockingBehavior given to WillSendRequest changes.
  virtual void DidChangeRenderBlockingBehavior(
      Resource* resource,
      const FetchParameters& params) = 0;

  // Called when ResourceFetcher::DidLoadResourceFromMemoryCache is called to
  // check if there is an inspector attached and if we should report information
  // about all requests to the inspector.
  virtual bool InterestedInAllRequests() = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_LOAD_OBSERVER_H_
