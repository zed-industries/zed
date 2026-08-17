// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_FETCH_CONTEXT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_FETCH_CONTEXT_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_worker_fetch_context.h"

#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "third_party/blink/public/mojom/renderer_preference_watcher.mojom-shared.h"
#include "third_party/blink/public/mojom/worker/subresource_loader_updater.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"

namespace blink {

class URLLoaderThrottleProvider;
class WebSocketHandshakeThrottleProvider;
struct RendererPreferences;

// Worker fetch context for service worker. This has a feature to update the
// subresource loader factories through the subresouruce loader updater in
// addition to WebWorkerFetchContext.
class BLINK_EXPORT WebServiceWorkerFetchContext : public WebWorkerFetchContext {
 public:
  static scoped_refptr<WebServiceWorkerFetchContext> Create(
      const RendererPreferences& renderer_preferences,
      const WebURL& worker_script_url,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_url_loader_factory,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_script_loader_factory,
      const WebURL& script_url_to_skip_throttling,
      std::unique_ptr<URLLoaderThrottleProvider> throttle_provider,
      std::unique_ptr<WebSocketHandshakeThrottleProvider>
          websocket_handshake_throttle_provider,
      CrossVariantMojoReceiver<mojom::RendererPreferenceWatcherInterfaceBase>
          preference_watcher_receiver,
      CrossVariantMojoReceiver<mojom::SubresourceLoaderUpdaterInterfaceBase>
          pending_subresource_loader_updater,
      const std::vector<WebString>& cors_exempt_header_list,
      const bool is_third_party_context);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_FETCH_CONTEXT_H_
