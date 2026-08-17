// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LOAD_OBSERVER_FOR_FRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LOAD_OBSERVER_FOR_FRAME_H_

#include <inttypes.h>

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_observer.h"

namespace blink {

class CoreProbeSink;
class Document;
class DocumentLoader;
class ResourceFetcherProperties;

// ResourceLoadObserver implementation associated with a frame.
class CORE_EXPORT ResourceLoadObserverForFrame final
    : public ResourceLoadObserver {
 public:
  ResourceLoadObserverForFrame(DocumentLoader& loader,
                               Document& document,
                               const ResourceFetcherProperties& properties);
  ~ResourceLoadObserverForFrame() override;

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
                                       const FetchParameters& params) override;
  bool InterestedInAllRequests() override;
  void Trace(Visitor*) const override;

 private:
  CoreProbeSink* GetProbe();
  void CountUsage(WebFeature);

  // There are some overlap between |document_loader_|, |document_| and
  // |fetcher_properties_|. Use |fetcher_properties_| whenever possible.
  const Member<DocumentLoader> document_loader_;
  const Member<Document> document_;
  const Member<const ResourceFetcherProperties> fetcher_properties_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_LOAD_OBSERVER_FOR_FRAME_H_
