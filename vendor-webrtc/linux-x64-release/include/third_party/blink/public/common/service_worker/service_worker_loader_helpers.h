// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_LOADER_HELPERS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_LOADER_HELPERS_H_

#include <optional>

#include "mojo/public/cpp/bindings/remote.h"
#include "net/http/http_request_headers.h"
#include "net/url_request/redirect_info.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/blob/blob.mojom.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_response.mojom.h"

namespace network {
struct ResourceRequest;
}

namespace blink {

// Helper functions for service worker classes that use URLLoader
// (e.g., ServiceWorkerMainResourceLoader and ServiceWorkerSubresourceLoader).
class BLINK_COMMON_EXPORT ServiceWorkerLoaderHelpers {
 public:
  // Populates |out_head| with given |response|.
  static void SaveResponseInfo(const blink::mojom::FetchAPIResponse& response,
                               network::mojom::URLResponseHead* out_head);

  // Returns a redirect info if |response_head| is an redirect response.
  // Otherwise returns std::nullopt.
  static std::optional<net::RedirectInfo> ComputeRedirectInfo(
      const network::ResourceRequest& original_request,
      const network::mojom::URLResponseHead& response_head);

  // Reads |blob| into |handle_out|. Calls |on_blob_read_complete| when done or
  // if an error occurred. Currently this always returns net::OK but
  // the plan is to return an error if reading couldn't start, in
  // which case |on_blob_read_complete| isn't called.
  static int ReadBlobResponseBody(
      mojo::Remote<blink::mojom::Blob>* blob,
      uint64_t blob_size,
      base::OnceCallback<void(int net_error)> on_blob_read_complete,
      mojo::ScopedDataPipeConsumerHandle* handle_out);

  static bool IsMainRequestDestination(
      network::mojom::RequestDestination destination);

  static const char* FetchResponseSourceToSuffix(
      network::mojom::FetchResponseSource source);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_LOADER_HELPERS_H_
