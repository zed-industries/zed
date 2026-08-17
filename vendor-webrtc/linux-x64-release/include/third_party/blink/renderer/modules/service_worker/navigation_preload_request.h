// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATION_PRELOAD_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATION_PRELOAD_REQUEST_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "services/network/public/mojom/url_loader.mojom.h"
#include "third_party/blink/public/mojom/service_worker/dispatch_fetch_event_params.mojom-blink.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_error.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/platform/web_url_response.h"
#include "third_party/blink/public/web/modules/service_worker/web_navigation_preload_request.h"

namespace blink {

class WebServiceWorkerContextClient;

// The URLLoaderClient for receiving a navigation preload response. It reports
// the response back to WebServiceWorkerContextClient.
//
// This class lives on the service worker thread and is owned by
// WebServiceWorkerContextClient.
class NavigationPreloadRequest final : public WebNavigationPreloadRequest,
                                       public network::mojom::URLLoaderClient {
 public:
  // |owner| must outlive |this|.
  NavigationPreloadRequest(
      WebServiceWorkerContextClient* owner,
      int fetch_event_id,
      const WebURL& url,
      mojo::PendingReceiver<network::mojom::URLLoaderClient>
          preload_url_loader_client_receiver);
  ~NavigationPreloadRequest() override;

  // network::mojom::URLLoaderClient:
  void OnReceiveEarlyHints(network::mojom::EarlyHintsPtr early_hints) override;
  void OnReceiveResponse(
      network::mojom::URLResponseHeadPtr response_head,
      mojo::ScopedDataPipeConsumerHandle body,
      std::optional<mojo_base::BigBuffer> cached_metadata) override;
  void OnReceiveRedirect(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr response_head) override;
  void OnUploadProgress(int64_t current_position,
                        int64_t total_size,
                        OnUploadProgressCallback ack_callback) override;
  void OnTransferSizeUpdated(int32_t transfer_size_diff) override;
  void OnComplete(const network::URLLoaderCompletionStatus& status) override;

 private:
  void MaybeReportResponseToOwner();
  void ReportErrorToOwner(const WebString& message,
                          WebServiceWorkerError::Mode error_mode);

  raw_ptr<WebServiceWorkerContextClient> owner_ = nullptr;

  const int fetch_event_id_ = -1;
  const WebURL url_;
  mojo::Receiver<network::mojom::URLLoaderClient> receiver_;

  mojo::PendingRemote<network::mojom::URLLoader> decoder_loader_;

  std::unique_ptr<WebURLResponse> response_;
  mojo::ScopedDataPipeConsumerHandle body_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_NAVIGATION_PRELOAD_REQUEST_H_
