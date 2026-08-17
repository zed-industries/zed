// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_NAVIGATION_PRELOAD_REQUEST_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_NAVIGATION_PRELOAD_REQUEST_H_

#include "mojo/public/cpp/bindings/pending_receiver.h"

namespace network {
namespace mojom {
class URLLoaderClient;
}  // namespace mojom
}  // namespace network

namespace blink {

class WebServiceWorkerContextClient;

class BLINK_EXPORT WebNavigationPreloadRequest {
 public:
  virtual ~WebNavigationPreloadRequest() = default;

  static std::unique_ptr<WebNavigationPreloadRequest> Create(
      WebServiceWorkerContextClient* owner,
      int fetch_event_id,
      const WebURL& url,
      mojo::PendingReceiver<network::mojom::URLLoaderClient>
          preload_url_loader_client_receiver);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_NAVIGATION_PRELOAD_REQUEST_H_
