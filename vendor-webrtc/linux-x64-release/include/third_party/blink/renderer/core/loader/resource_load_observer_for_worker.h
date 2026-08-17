// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LOAD_OBSERVER_FOR_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LOAD_OBSERVER_FOR_WORKER_H_

#include <inttypes.h>

#include "base/containers/span.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_observer.h"

namespace blink {

class CoreProbeSink;
class ResourceFetcherProperties;
class WorkerFetchContext;

// ResourceLoadObserver implementation associated with a worker or worklet.
class ResourceLoadObserverForWorker final : public ResourceLoadObserver {
 public:
  ResourceLoadObserverForWorker(
      CoreProbeSink& probe,
      const ResourceFetcherProperties& properties,
      WorkerFetchContext& worker_fetch_context,
      const base::UnguessableToken& devtools_worker_token);
  ~ResourceLoadObserverForWorker() override;

  // ResourceLoadObserver implementation.
  void DidStartRequest(const FetchParameters&, ResourceType) override;
  void WillSendRequest(const ResourceRequest&,
                       const ResourceResponse& redirect_response,
                       ResourceType,
                       const ResourceLoaderOptions&,
                       RenderBlockingBehavior,
                       const Resource*) override;
  void DidChangePriority(uint64_t identifier,
                         ResourceLoadPriority,
                         int intra_priority_value) override;
  void DidReceiveResponse(uint64_t identifier,
                          const ResourceRequest& request,
                          const ResourceResponse& response,
                          const Resource* resource,
                          ResponseSource) override;
  void DidReceiveData(uint64_t identifier,
                      base::SpanOrSize<const char> chunk) override;
  void DidReceiveTransferSizeUpdate(uint64_t identifier,
                                    int transfer_size_diff) override;
  void DidDownloadToBlob(uint64_t identifier, BlobDataHandle*) override;
  void DidFinishLoading(uint64_t identifier,
                        base::TimeTicks finish_time,
                        int64_t encoded_data_length,
                        int64_t decoded_body_length) override;
  void DidFailLoading(const KURL&,
                      uint64_t identifier,
                      const ResourceError&,
                      int64_t encoded_data_length,
                      IsInternalRequest) override;
  void DidChangeRenderBlockingBehavior(Resource* resource,
                                       const FetchParameters& params) override {
  }
  bool InterestedInAllRequests() override;
  void Trace(Visitor*) const override;

 private:
  const Member<CoreProbeSink> probe_;
  const Member<const ResourceFetcherProperties> fetcher_properties_;
  const Member<WorkerFetchContext> worker_fetch_context_;
  const base::UnguessableToken devtools_worker_token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LOAD_OBSERVER_FOR_WORKER_H_
