// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_DEDICATED_OR_SHARED_WORKER_FETCH_CONTEXT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_DEDICATED_OR_SHARED_WORKER_FETCH_CONTEXT_IMPL_H_

#include "base/memory/raw_ptr.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/bindings/remote_set.h"
#include "mojo/public/cpp/bindings/shared_remote.h"
#include "services/network/public/mojom/url_loader_factory.mojom-forward.h"
#include "third_party/blink/public/mojom/loader/resource_load_info_notifier.mojom.h"
#include "third_party/blink/public/mojom/renderer_preference_watcher.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_container.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_worker_client.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_worker_client_registry.mojom-blink.h"
#include "third_party/blink/public/mojom/worker/subresource_loader_updater.mojom-blink.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_dedicated_or_shared_worker_fetch_context.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ResourceLoadInfoNotifierWrapper;
class URLLoaderThrottleProvider;
class WeakWrapperResourceLoadInfoNotifier;
class WebServiceWorkerProviderContext;
class WebSocketHandshakeThrottleProvider;

// This class is used for fetching resource requests from workers (dedicated
// worker and shared worker). This class is created on the main thread and
// passed to the worker thread. This class is not used for service workers. For
// service workers, ServiceWorkerFetchContextImpl class is used instead.
class BLINK_PLATFORM_EXPORT DedicatedOrSharedWorkerFetchContextImpl final
    : public WebDedicatedOrSharedWorkerFetchContext,
      public mojom::blink::SubresourceLoaderUpdater,
      public mojom::blink::ServiceWorkerWorkerClient,
      public mojom::blink::RendererPreferenceWatcher {
 public:
  // - |service_worker_client_receiver| receives OnControllerChanged()
  //   notifications.
  // - |service_worker_worker_client_registry| is used to register new
  //   ServiceWorkerWorkerClients, which is needed when creating a
  //   nested worker.
  //
  // Regarding the rest of params, see the comments on Create().
  DedicatedOrSharedWorkerFetchContextImpl(
      const RendererPreferences& renderer_preferences,
      mojo::PendingReceiver<mojom::blink::RendererPreferenceWatcher>
          preference_watcher_receiver,
      mojo::PendingReceiver<mojom::blink::ServiceWorkerWorkerClient>
          service_worker_client_receiver,
      mojo::PendingRemote<mojom::blink::ServiceWorkerWorkerClientRegistry>
          pending_service_worker_worker_client_registry,
      CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>
          service_worker_container_host,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_loader_factory,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_fallback_factory,
      mojo::PendingReceiver<mojom::blink::SubresourceLoaderUpdater>
          pending_subresource_loader_updater,
      std::unique_ptr<URLLoaderThrottleProvider> throttle_provider,
      std::unique_ptr<WebSocketHandshakeThrottleProvider>
          websocket_handshake_throttle_provider,
      Vector<String> cors_exempt_header_list,
      mojo::PendingRemote<mojom::ResourceLoadInfoNotifier>
          pending_resource_load_info_notifier);

  // WebDedicatedOrSharedWorkerFetchContext implementation:
  // The cloned fetch context does not inherit some fields (e.g.,
  // blink::WebServiceWorkerProviderContext) from this fetch context, and
  // instead that takes values passed from the browser process.
  scoped_refptr<WebDedicatedOrSharedWorkerFetchContext> CloneForNestedWorker(
      WebServiceWorkerProviderContext* service_worker_provider_context,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_loader_factory,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_fallback_factory,
      CrossVariantMojoReceiver<mojom::SubresourceLoaderUpdaterInterfaceBase>
          pending_subresource_loader_updater,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) override;
  // Sets properties associated with frames.
  // - For dedicated workers, the property is copied from the ancestor frame
  //   (directly for non-nested workers, or indirectly via its parent worker for
  //   nested workers).
  // - For shared workers, there is no parent frame, so the default value, or a
  //   value calculated in some way is set.
  //
  // TODO(nhiroki): Add more comments about security/privacy implications to
  // each property, for example, site_for_cookies and top_frame_origin.
  void SetAncestorFrameToken(const LocalFrameToken& token) override;
  void set_site_for_cookies(
      const net::SiteForCookies& site_for_cookies) override;
  void set_top_frame_origin(const WebSecurityOrigin& top_frame_origin) override;

  // WebWorkerFetchContext implementation:
  void SetTerminateSyncLoadEvent(base::WaitableEvent*) override;
  void InitializeOnWorkerThread(AcceptLanguagesWatcher*) override;
  URLLoaderFactory* GetURLLoaderFactory() override;
  std::unique_ptr<URLLoaderFactory> WrapURLLoaderFactory(
      CrossVariantMojoRemote<network::mojom::URLLoaderFactoryInterfaceBase>
          url_loader_factory) override;
  std::optional<WebURL> WillSendRequest(const WebURL& url) override;
  void FinalizeRequest(WebURLRequest&) override;
  std::vector<std::unique_ptr<URLLoaderThrottle>> CreateThrottles(
      const network::ResourceRequest& request) override;
  mojom::ControllerServiceWorkerMode GetControllerServiceWorkerMode()
      const override;
  void SetIsOnSubframe(bool) override;
  bool IsOnSubframe() const override;
  net::SiteForCookies SiteForCookies() const override;
  std::optional<WebSecurityOrigin> TopFrameOrigin() const override;
  void SetSubresourceFilterBuilder(
      std::unique_ptr<WebDocumentSubresourceFilter::Builder>) override;
  std::unique_ptr<WebDocumentSubresourceFilter> TakeSubresourceFilter()
      override;
  std::unique_ptr<WebSocketHandshakeThrottle> CreateWebSocketHandshakeThrottle(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) override;
  bool IsDedicatedWorkerOrSharedWorkerFetchContext() const override {
    return true;
  }

  // mojom::blink::ServiceWorkerWorkerClient implementation:
  void OnControllerChanged(mojom::ControllerServiceWorkerMode) override;

  // Sets the controller service worker mode.
  // - For dedicated workers and shared workers, the controller mode is passed
  // from the browser processw when starting the worker.
  void set_controller_service_worker_mode(
      mojom::ControllerServiceWorkerMode mode);

  void set_client_id(const WebString& client_id);

  // TODO(crbug.com/324939068): remove the code when the feature launched.
  void set_container_is_blob_url_shared_worker(bool is_blob_url) {
    container_is_blob_url_shared_worker_ = is_blob_url;
  }
  void set_container_is_shared_worker(bool is_sharedworker) override;

  WebString GetAcceptLanguages() const override;

  std::unique_ptr<ResourceLoadInfoNotifierWrapper>
  CreateResourceLoadInfoNotifierWrapper() override;

 private:
  class Factory;

  ~DedicatedOrSharedWorkerFetchContextImpl() override;

  scoped_refptr<DedicatedOrSharedWorkerFetchContextImpl>
  CloneForNestedWorkerInternal(
      mojo::PendingReceiver<mojom::blink::ServiceWorkerWorkerClient>
          service_worker_client_receiver,
      mojo::PendingRemote<mojom::blink::ServiceWorkerWorkerClientRegistry>
          service_worker_worker_client_registry,
      CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>
          service_worker_container_host,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_loader_factory,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_fallback_factory,
      mojo::PendingReceiver<mojom::blink::SubresourceLoaderUpdater>
          pending_subresource_loader_updater,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  // Resets the service worker url loader factory of a URLLoaderFactoryImpl
  // which was passed to Blink. The url loader factory is connected to the
  // controller service worker. Sets nullptr if the worker context is not
  // controlled by a service worker.
  void ResetServiceWorkerURLLoaderFactory();

  // Implements mojom::blink::SubresourceLoaderUpdater.
  void UpdateSubresourceLoaderFactories(
      std::unique_ptr<PendingURLLoaderFactoryBundle>
          subresource_loader_factories) override;

  // Implements mojom::blink::RendererPreferenceWatcher.
  void NotifyUpdate(const RendererPreferences& new_prefs) override;

  void ResetWeakWrapperResourceLoadInfoNotifier();

  // |receiver_| and |service_worker_worker_client_registry_| may be null if
  // this context can't use service workers. See comments for Create().
  mojo::Receiver<mojom::blink::ServiceWorkerWorkerClient> receiver_{this};
  mojo::Remote<mojom::blink::ServiceWorkerWorkerClientRegistry>
      service_worker_worker_client_registry_;

  // Bound to |this| on the worker thread.
  mojo::PendingReceiver<mojom::blink::ServiceWorkerWorkerClient>
      service_worker_client_receiver_;

  // Consumed on the worker thread to create
  // |service_worker_worker_client_registry_|.
  mojo::PendingRemote<mojom::blink::ServiceWorkerWorkerClientRegistry>
      pending_service_worker_worker_client_registry_;

  // Consumed on the worker thread to create |loader_factory_|.
  std::unique_ptr<network::PendingSharedURLLoaderFactory>
      pending_loader_factory_;
  // Consumed on the worker thread to create |fallback_factory_|.
  std::unique_ptr<network::PendingSharedURLLoaderFactory>
      pending_fallback_factory_;

  // This can be null if the |provider_context| passed to Create() was null or
  // already being destructed (see
  // content::ServiceWorkerProviderContext::OnNetworkProviderDestroyed()).
  CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>
      service_worker_container_host_;

  mojom::ControllerServiceWorkerMode controller_service_worker_mode_ =
      mojom::ControllerServiceWorkerMode::kNoController;

  // The Client#id value of the shared worker or dedicated worker (since
  // dedicated workers are not yet service worker clients, it is the parent
  // document's id in that case). Passed to ControllerServiceWorkerConnector.
  WebString client_id_;

  // TODO(crbug.com/324939068): remove the flag when the feature launched.
  bool container_is_blob_url_shared_worker_ = false;
  bool container_is_shared_worker_ = false;

  // Initialized on the worker thread when InitializeOnWorkerThread() is called.
  // |loader_factory_| is used for regular loading by the worker. In
  // If the worker is controlled by a service worker, it creates a
  // ServiceWorkerSubresourceLoaderFactory instead.
  scoped_refptr<network::SharedURLLoaderFactory> loader_factory_;

  // Initialized on the worker thread when InitializeOnWorkerThread() is called.
  // If the worker is controlled by a service worker, it passes this factory to
  // ServiceWorkerSubresourceLoaderFactory to use for network fallback.
  scoped_refptr<network::SharedURLLoaderFactory> fallback_factory_;

  // Initialized on the worker thread when InitializeOnWorkerThread() is called.
  // Used to reconnect to the Network Service after the Network Service crash.
  // This is only used for dedicated workers. For shared workers, the renderer
  // process detects the crash, and terminates the worker instead of recovery.
  mojo::PendingReceiver<mojom::blink::SubresourceLoaderUpdater>
      pending_subresource_loader_updater_;
  mojo::Receiver<mojom::blink::SubresourceLoaderUpdater>
      subresource_loader_updater_{this};

  std::unique_ptr<WebDocumentSubresourceFilter::Builder>
      subresource_filter_builder_;
  // For dedicated workers, this is the ancestor frame (the parent frame for
  // non-nested workers, the closest ancestor for nested workers). For shared
  // workers, this is the shadow page.
  bool is_on_sub_frame_ = false;
  std::optional<LocalFrameToken> ancestor_frame_token_;
  net::SiteForCookies site_for_cookies_;
  std::optional<url::Origin> top_frame_origin_;

  RendererPreferences renderer_preferences_;

  // |preference_watcher_receiver_| and |child_preference_watchers_| are for
  // keeping track of updates in the renderer preferences.
  mojo::Receiver<mojom::blink::RendererPreferenceWatcher>
      preference_watcher_receiver_{this};
  // Kept while staring up the worker thread. Valid until
  // InitializeOnWorkerThread().
  mojo::PendingReceiver<mojom::blink::RendererPreferenceWatcher>
      preference_watcher_pending_receiver_;
  mojo::RemoteSet<mojom::blink::RendererPreferenceWatcher>
      child_preference_watchers_;

  // This is owned by ThreadedMessagingProxyBase on the main thread.
  raw_ptr<base::WaitableEvent> terminate_sync_load_event_ = nullptr;

  // The URLLoaderFactory which was created and passed to
  // Blink by GetURLLoaderFactory().
  std::unique_ptr<Factory> web_loader_factory_;

  std::unique_ptr<URLLoaderThrottleProvider> throttle_provider_;
  std::unique_ptr<WebSocketHandshakeThrottleProvider>
      websocket_handshake_throttle_provider_;

  Vector<String> cors_exempt_header_list_;

  mojo::PendingRemote<mojom::ResourceLoadInfoNotifier>
      pending_resource_load_info_notifier_;

  // Used to notify the loading stats by ResourceLoadInfo struct for dedicated
  // workers.
  mojo::Remote<mojom::ResourceLoadInfoNotifier> resource_load_info_notifier_;

  // Wrap a raw blink::mojom::ResourceLoadInfoNotifier pointer directed at
  // |resource_load_info_notifier_|'s receiver.
  std::unique_ptr<WeakWrapperResourceLoadInfoNotifier>
      weak_wrapper_resource_load_info_notifier_;

  WeakPersistent<AcceptLanguagesWatcher> accept_languages_watcher_;
};

template <>
struct DowncastTraits<DedicatedOrSharedWorkerFetchContextImpl> {
  static bool AllowFrom(const WebWorkerFetchContext& context) {
    return context.IsDedicatedWorkerOrSharedWorkerFetchContext();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_DEDICATED_OR_SHARED_WORKER_FETCH_CONTEXT_IMPL_H_
