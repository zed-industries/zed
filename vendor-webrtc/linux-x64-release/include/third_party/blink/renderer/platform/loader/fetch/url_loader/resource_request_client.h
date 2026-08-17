// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_RESOURCE_REQUEST_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_RESOURCE_REQUEST_CLIENT_H_

#include <stdint.h>

#include <string>
#include <vector>

#include "base/functional/callback_forward.h"
#include "base/time/time.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace net {
struct RedirectInfo;
class HttpRequestHeaders;
}

namespace network {
struct URLLoaderCompletionStatus;
}

namespace blink {

// These callbacks mirror net::URLRequest::Delegate and the order and
// conditions in which they will be called are identical. See url_request.h
// for more information.
class BLINK_PLATFORM_EXPORT ResourceRequestClient
    : public WTF::RefCounted<ResourceRequestClient> {
 public:
  // Called as upload progress is made.
  // note: only for requests with upload progress enabled.
  virtual void OnUploadProgress(uint64_t position, uint64_t size) = 0;

  // Called when a redirect occurs. The URLResponseHead provides information
  // about the redirect response and the RedirectInfo includes information about
  // the request to be made if the `follow_redirect_callback` is called.
  // `removed_headers` contains header field names that need to be removed.
  // `modified_headers` contains headers that need to be added or updated.
  using FollowRedirectCallback =
      base::OnceCallback<void(std::vector<std::string> removed_headers,
                              net::HttpRequestHeaders modified_headers)>;
  virtual void OnReceivedRedirect(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr head,
      FollowRedirectCallback follow_redirect_callback) = 0;

  // Called when response headers are available (after all redirects have
  // been followed).
  virtual void OnReceivedResponse(
      network::mojom::URLResponseHeadPtr head,
      mojo::ScopedDataPipeConsumerHandle body,
      std::optional<mojo_base::BigBuffer> cached_metadata) = 0;

  // Called when the transfer size is updated. This method may be called
  // multiple times or not at all. The transfer size is the length of the
  // response (including both headers and the body) over the network.
  // |transfer_size_diff| is the difference from the value previously reported
  // one (including the one in OnReceivedResponse). It must be positive.
  virtual void OnTransferSizeUpdated(int transfer_size_diff) = 0;

  // Called when the response is complete.  This method signals completion of
  // the resource load.
  virtual void OnCompletedRequest(
      const network::URLLoaderCompletionStatus& status) = 0;

  virtual ~ResourceRequestClient() = default;

  friend class WTF::RefCounted<ResourceRequestClient>;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_RESOURCE_REQUEST_CLIENT_H_
