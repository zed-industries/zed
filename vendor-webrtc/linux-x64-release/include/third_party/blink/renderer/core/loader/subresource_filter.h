// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_SUBRESOURCE_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_SUBRESOURCE_FILTER_H_

#include <memory>
#include <utility>

#include "services/network/public/mojom/fetch_api.mojom.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_document_subresource_filter.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/reporting_disposition.h"

namespace blink {

class ExecutionContext;
class KURL;

// Wrapper around a WebDocumentSubresourceFilter. This class will make it easier
// to extend the subresource filter with optimizations only possible using blink
// types (e.g. a caching layer using StringImpl).
class CORE_EXPORT SubresourceFilter final
    : public GarbageCollected<SubresourceFilter> {
 public:
  SubresourceFilter(ExecutionContext*,
                    std::unique_ptr<WebDocumentSubresourceFilter>);
  ~SubresourceFilter();

  bool AllowLoad(const KURL& resource_url,
                 network::mojom::RequestDestination,
                 ReportingDisposition);
  bool AllowWebSocketConnection(const KURL&);
  bool AllowWebTransportConnection(const KURL&);

  // Returns if |resource_url| is an ad resource.
  bool IsAdResource(const KURL& resource_url,
                    network::mojom::RequestDestination);

  void Trace(Visitor*) const;

 private:
  void ReportLoad(const KURL& resource_url,
                  WebDocumentSubresourceFilter::LoadPolicy);
  void ReportLoadAsync(const KURL& resource_url,
                       WebDocumentSubresourceFilter::LoadPolicy);

  Member<ExecutionContext> execution_context_;
  std::unique_ptr<WebDocumentSubresourceFilter> subresource_filter_;

  // Save the last resource check's result in the single element cache.
  std::pair<std::pair<KURL, network::mojom::RequestDestination>,
            WebDocumentSubresourceFilter::LoadPolicy>
      last_resource_check_result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_SUBRESOURCE_FILTER_H_
