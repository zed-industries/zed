/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_URL_LOADER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_URL_LOADER_CLIENT_H_

#include <memory>
#include <variant>
#include <vector>

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "net/http/http_request_headers.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-shared.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace net {
class SiteForCookies;
}

namespace blink {

class WebString;
class WebURL;
class WebURLResponse;
struct WebURLError;

class BLINK_PLATFORM_EXPORT URLLoaderClient {
 public:
  // Called when following a redirect. |new_.*| arguments contain the
  // information about the received redirect. When |report_raw_headers| is
  // updated it'll be used for filtering data of the next redirect or response.
  // |removed_headers| outputs headers that need to be removed from the
  // redirect request. `modified_headers` outputs headers that need to be added
  // to or updated in the redirect request.
  //
  // Implementations should return true to instruct the loader to follow the
  // redirect, or false otherwise.
  virtual bool WillFollowRedirect(
      const WebURL& new_url,
      const net::SiteForCookies& new_site_for_cookies,
      const WebString& new_referrer,
      network::mojom::ReferrerPolicy new_referrer_policy,
      const WebString& new_method,
      const WebURLResponse& passed_redirect_response,
      bool& report_raw_headers,
      std::vector<std::string>* removed_headers,
      net::HttpRequestHeaders& modified_headers,
      bool insecure_scheme_was_upgraded) {
    return true;
  }

  // Called to report upload progress. The bytes reported correspond to
  // the HTTP message body.
  virtual void DidSendData(uint64_t bytes_sent,
                           uint64_t total_bytes_to_be_sent) {}

  // Called when response are received.
  virtual void DidReceiveResponse(
      const WebURLResponse&,
      std::variant<mojo::ScopedDataPipeConsumerHandle, SegmentedBuffer>,
      std::optional<mojo_base::BigBuffer> cached_metadata) {}

  // Called when a chunk of response data is received. |data_length| is the
  // number of bytes pointed to by |data|. This is used only for testing to
  // pass the data to the ResourceLoader.
  virtual void DidReceiveDataForTesting(base::span<const char> data) {}

  // Called when the number of bytes actually received from network including
  // HTTP headers is updated. |transfer_size_diff| is positive.
  virtual void DidReceiveTransferSizeUpdate(int transfer_size_diff) {}

  // Called when the load completes successfully.
  // |total_encoded_data_length| may be equal to kUnknownEncodedDataLength.
  // TODO(crbug.com/798625): use different callback for subresources
  // with responses blocked due to document protection.
  virtual void DidFinishLoading(base::TimeTicks finish_time,
                                int64_t total_encoded_data_length,
                                uint64_t total_encoded_body_length,
                                int64_t total_decoded_body_length) {}

  // Called when the load completes with an error.
  // |finish_time| indicating the time in which the response failed.
  // |total_encoded_data_length| may be equal to kUnknownEncodedDataLength.
  virtual void DidFail(const WebURLError&,
                       base::TimeTicks finish_time,
                       int64_t total_encoded_data_length,
                       uint64_t total_encoded_body_length,
                       int64_t total_decoded_body_length) {}

  // Counts a WebFeature use.
  // TODO(https://crbug.com/1393520): Remove this method once we get enough
  // stats to make a decision.
  virtual void CountFeature(blink::mojom::WebFeature) {}

  // Value passed to DidFinishLoading when total encoded data length isn't
  // known.
  static const int64_t kUnknownEncodedDataLength = -1;

 protected:
  virtual ~URLLoaderClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_URL_LOADER_CLIENT_H_
