// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CONTENT_DECODING_URL_LOADER_THROTTLE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CONTENT_DECODING_URL_LOADER_THROTTLE_H_

#include "third_party/blink/public/common/loader/url_loader_throttle.h"

namespace blink {

// A URLLoaderThrottle that intercepts network responses and applies content
// decoding (e.g., gzip, brotli, zstd) to the response body before it reaches
// the URLLoaderClient. This class uses `network::ContentDecodingInterceptor` to
// perform the actual decoding.
class ContentDecodingURLLoaderThrottle : public URLLoaderThrottle {
 public:
  ContentDecodingURLLoaderThrottle();
  ~ContentDecodingURLLoaderThrottle() override;

  // blink::URLLoaderThrottle implementation:
  void DetachFromCurrentSequence() override;
  void WillProcessResponse(const GURL& response_url,
                           network::mojom::URLResponseHead* response_head,
                           bool* defer) override;
  const char* NameForLoggingWillProcessResponse() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CONTENT_DECODING_URL_LOADER_THROTTLE_H_
