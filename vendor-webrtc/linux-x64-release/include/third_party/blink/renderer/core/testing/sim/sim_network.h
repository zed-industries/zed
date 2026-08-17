// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_NETWORK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_NETWORK_H_

#include "third_party/blink/renderer/platform/testing/url_loader_test_delegate.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class SimRequestBase;
class URLLoaderClient;
class WebURLResponse;

// Simulates a network with precise flow control so you can make requests
// return, write data, and finish in a specific order in a unit test. One of
// these must be created before using the SimRequestBase to issue requests.
class SimNetwork final : public URLLoaderTestDelegate {
 public:
  SimNetwork();
  ~SimNetwork() override;

 private:
  friend class SimRequestBase;
  friend class SimSubresourceRequest;

  static SimNetwork& Current();

  void ServePendingRequests();
  void AddRequest(SimRequestBase&);
  void RemoveRequest(SimRequestBase&);

  // URLLoaderTestDelegate
  void DidReceiveResponse(URLLoaderClient*, const WebURLResponse&) override;
  void DidReceiveData(URLLoaderClient*, base::span<const char> data) override;
  void DidFail(URLLoaderClient*,
               const WebURLError&,
               int64_t total_encoded_data_length,
               int64_t total_encoded_body_length,
               int64_t total_decoded_body_length) override;
  void DidFinishLoading(URLLoaderClient*,
                        base::TimeTicks finish_time,
                        int64_t total_encoded_data_length,
                        int64_t total_encoded_body_length,
                        int64_t total_decoded_body_length) override;
  bool FillNavigationParamsResponse(WebNavigationParams*) override;

  SimRequestBase* current_request_;
  HashMap<String, SimRequestBase*> requests_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_NETWORK_H_
