// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_URL_LOADER_THROTTLE_PROVIDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_URL_LOADER_THROTTLE_PROVIDER_H_

#include <memory>
#include <vector>

#include "base/types/optional_ref.h"
#include "third_party/blink/public/common/loader/url_loader_throttle.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_url_request.h"

namespace network {
struct ResourceRequest;
}  // namespace network

namespace blink {

enum class URLLoaderThrottleProviderType {
  // Used for requests from frames. Please note that the requests could be
  // frame or subresource requests.
  kFrame,
  // Used for requests from workers, including dedicated, shared and service
  // workers.
  kWorker
};

// How Clone() and CreateThrottles() are called:
//
// Frame subresource fetches:
// - [Main Thread]
//   - Startup: Create a URLLoaderThrottleProvider for all frames to share.
//   - If BackgroundResourceFetch is off or the resource is not handled by
//     BackgroundURLLoader, call CreateThrottles() (per request).
//   - First supported BackgroundURLLoader request: Clone() provider and move it
//     to the background thread. Call CreateThrottles() for subsequent requests
//     on the background thread.
//
// Dedicated/Shared Worker resource fetches:
// - [Main Thread]
//    - Create a URLLoaderThrottleProvider and pass it to the worker thread.
// - [Worker Thread]
//    - Call CreateThrottles() for worker initiated resource fetch requests.
//
// Service Worker resource fetches:
// - [Initiator Thread]
//    - Create a URLLoaderThrottleProvider and pass it to the worker thread.
// - [Worker Thread]
//    - Call CreateThrottles() for worker initiated resource fetch requests.
//
// Nested Worker resource fetches:
// - [Initiator Worker Thread]
//    - Clone() the existing URLLoaderThrottleProvider and pass it to the nested
//      worker thread.
// - [Nested Worker Thread]
//    - Call CreateThrottles() for each resource fetch request initiated by the
//      nested worker.
//
// Note: BackgroundResourceFetch is not supported for worker resource fetches.
//
// TODO(crbug.com/1379780): This class name should have Web prefix according to
// third_party/blink/public/README.md#naming-conventions
class BLINK_PLATFORM_EXPORT URLLoaderThrottleProvider {
 public:
  virtual ~URLLoaderThrottleProvider() = default;

  // Used to copy a URLLoaderThrottleProvider between worker threads, and to
  // copy a URLLoaderThrottleProvider for the BackgroundResourceFetch feature.
  virtual std::unique_ptr<URLLoaderThrottleProvider> Clone() = 0;

  // For frame requests, this is called on the main thread if
  // BackgroundResourceFetch feature is disabled. Otherwise, it is called on the
  // background thread for some types of requests.
  // Dedicated, shared and service workers call it on the worker thread.
  //`local_frame_token` will be set to the corresponding frame for frame and
  // dedicated worker requests, otherwise it will not be set.
  virtual std::vector<std::unique_ptr<URLLoaderThrottle>> CreateThrottles(
      base::optional_ref<const LocalFrameToken> local_frame_token,
      const network::ResourceRequest& request) = 0;

  // Set the network status online state as specified in |is_online|.
  virtual void SetOnline(bool is_online) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_URL_LOADER_THROTTLE_PROVIDER_H_
