// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_TEST_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_TEST_DELEGATE_H_

#include "base/containers/span.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

struct WebNavigationParams;
class WebURLResponse;
class URLLoaderClient;
struct WebURLError;

// Use with URLLoaderMockFactory::SetLoaderDelegate to intercept calls to a
// URLLoaderClient for controlling network responses in a test. Default
// implementations of all methods just call the original method on the
// URLLoaderClient.
class URLLoaderTestDelegate {
 public:
  URLLoaderTestDelegate();
  virtual ~URLLoaderTestDelegate();

  virtual void DidReceiveResponse(URLLoaderClient* original_client,
                                  const WebURLResponse&);
  virtual void DidReceiveData(URLLoaderClient* original_client,
                              base::span<const char> data);
  virtual void DidFail(URLLoaderClient* original_client,
                       const WebURLError&,
                       int64_t total_encoded_data_length,
                       int64_t total_encoded_body_length,
                       int64_t total_decoded_body_length);
  virtual void DidFinishLoading(URLLoaderClient* original_client,
                                base::TimeTicks finish_time,
                                int64_t total_encoded_data_length,
                                int64_t total_encoded_body_length,
                                int64_t total_decoded_body_length);
  // Default implementation will load mocked url and fill in redirects,
  // response and body loader.
  // To override default behavior, fill in response (always), redirects
  // (if needed) and body loader (if not empty, see WebNavigationParams)
  // and return true.
  virtual bool FillNavigationParamsResponse(WebNavigationParams*) {
    return false;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_TEST_DELEGATE_H_
