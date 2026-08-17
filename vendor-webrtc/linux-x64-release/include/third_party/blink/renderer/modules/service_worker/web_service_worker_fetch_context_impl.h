// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_FETCH_CONTEXT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_FETCH_CONTEXT_IMPL_H_

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/common/renderer_preferences/renderer_preferences.h"
#include "third_party/blink/public/mojom/renderer_preference_watcher.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker.mojom-forward.h"
#include "third_party/blink/public/mojom/worker/subresource_loader_updater.mojom-blink.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_fetch_context.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class WaitableEvent;
}

namespace blink {

class BLINK_EXPORT WebServiceWorkerFetchContextImpl final
    : public WebServiceWorkerFetchContext,
      public mojom::blink::RendererPreferenceWatcher,
      public mojom::blink::SubresourceLoaderUpdater {
 public:
  // |pending_url_loader_factory| is used for regular loads from the service
  // worker (i.e., Fetch API). It typically goes to network, but it might
  // internally contain non-NetworkService factories for handling non-http(s)
  // URLs like chrome-extension://. |pending_script_loader_factory| is used for
  // importScripts() from the service worker when InstalledScriptsManager
  // doesn't have the requested script. It is a
  // ServiceWorkerScriptLoaderFactory, which loads and installs the script.
  // |script_url_to_skip_throttling| is a URL which is already throttled in the
  // browser process so that it doesn't need to be throttled in the renderer
  // again.
  WebServiceWorkerFetchContextImpl(
      const RendererPreferences& renderer_preferences,
      const KURL& worker_script_url,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_url_loader_factory,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_script_loader_factory,
      const KURL& script_url_to_skip_throttling,
      std::unique_ptr<URLLoaderThrottleProvider> throttle_provider,
      std::unique_ptr<WebSocketHandshakeThrottleProvider>
          websocket_handshake_throttle_provider,
      mojo::PendingReceiver<mojom::blink::RendererPreferenceWatcher>
          preference_watcher_receiver,
      mojo::PendingReceiver<mojom::blink::SubresourceLoaderUpdater>
          pending_subresource_loader_updater,
      Vector<String> cors_exempt_header_list,
      bool is_third_party_context);

  // WebServiceWorkerFetchContext implementation:
  void SetTerminateSyncLoadEvent(base::WaitableEvent*) override;
  void InitializeOnWorkerThread(AcceptLanguagesWatcher*) override;
  URLLoaderFactory* GetURLLoaderFactory() override;
  std::unique_ptr<URLLoaderFactory> WrapURLLoaderFactory(
      CrossVariantMojoRemote<network::mojom::URLLoaderFactoryInterfaceBase>
          url_loader_factory) override;
  URLLoaderFactory* GetScriptLoaderFactory() override;
  void FinalizeRequest(WebURLRequest&) override;
  std::vector<std::unique_ptr<URLLoaderThrottle>> CreateThrottles(
      const network::ResourceRequest& request) override;
  mojom::ControllerServiceWorkerMode GetControllerServiceWorkerMode()
      const override;
  net::SiteForCookies SiteForCookies() const override;
  std::optional<WebSecurityOrigin> TopFrameOrigin() const override;
  std::unique_ptr<WebSocketHandshakeThrottle> CreateWebSocketHandshakeThrottle(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) override;
  WebString GetAcceptLanguages() const override;

  // mojom::blink::SubresourceLoaderUpdater implementation:
  void UpdateSubresourceLoaderFactories(
      std::unique_ptr<PendingURLLoaderFactoryBundle>
          subresource_loader_factories) override;

 private:
  ~WebServiceWorkerFetchContextImpl() override;

  // Implements mojom::blink::RendererPreferenceWatcher.
  void NotifyUpdate(const RendererPreferences& new_prefs) override;

  RendererPreferences renderer_preferences_;
  const KURL worker_script_url_;
  // Consumed on the worker thread to create |url_loader_factory_|.
  std::unique_ptr<network::PendingSharedURLLoaderFactory>
      pending_url_loader_factory_;
  // Consumed on the worker thread to create |web_script_loader_factory_|.
  std::unique_ptr<network::PendingSharedURLLoaderFactory>
      pending_script_loader_factory_;

  // A script URL that should skip throttling when loaded because it's already
  // being loaded in the browser process and went through throttles there. It's
  // valid only once and set to invalid KURL once the script is served.
  KURL script_url_to_skip_throttling_;

  // Responsible for regular loads from the service worker (i.e., Fetch API).
  std::unique_ptr<URLLoaderFactory> url_loader_factory_;
  // Responsible for script loads from the service worker (i.e., the
  // classic/module main script, module imported scripts, or importScripts()).
  std::unique_ptr<URLLoaderFactory> web_script_loader_factory_;

  std::unique_ptr<URLLoaderThrottleProvider> throttle_provider_;
  std::unique_ptr<WebSocketHandshakeThrottleProvider>
      websocket_handshake_throttle_provider_;

  mojo::Receiver<mojom::blink::RendererPreferenceWatcher>
      preference_watcher_receiver_{this};
  mojo::Receiver<mojom::blink::SubresourceLoaderUpdater>
      subresource_loader_updater_{this};

  // These mojo objects are kept while starting up the worker thread. Valid
  // until InitializeOnWorkerThread().
  mojo::PendingReceiver<mojom::blink::RendererPreferenceWatcher>
      preference_watcher_pending_receiver_;
  mojo::PendingReceiver<mojom::blink::SubresourceLoaderUpdater>
      pending_subresource_loader_updater_;

  // This is owned by ThreadedMessagingProxyBase on the main thread.
  raw_ptr<base::WaitableEvent> terminate_sync_load_event_ = nullptr;

  WeakPersistent<AcceptLanguagesWatcher> accept_languages_watcher_;

  Vector<String> cors_exempt_header_list_;

  bool is_third_party_context_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_FETCH_CONTEXT_IMPL_H_
