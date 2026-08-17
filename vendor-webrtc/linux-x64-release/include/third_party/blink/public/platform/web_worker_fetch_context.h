// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_WORKER_FETCH_CONTEXT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_WORKER_FETCH_CONTEXT_H_

#include <memory>
#include <optional>
#include <vector>

#include "base/memory/ref_counted.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "services/network/public/mojom/url_loader_factory.mojom-shared.h"
#include "third_party/blink/public/common/loader/url_loader_throttle.h"
#include "third_party/blink/public/mojom/service_worker/controller_service_worker_mode.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/resource_load_info_notifier_wrapper.h"
#include "third_party/blink/public/platform/web_document_subresource_filter.h"
#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/platform/websocket_handshake_throttle.h"

namespace base {
class WaitableEvent;
}  // namespace base

namespace net {
class SiteForCookies;
}  // namespace net

namespace blink {

class AcceptLanguagesWatcher;
class WebDocumentSubresourceFilter;
class URLLoaderFactory;
class WebURLRequest;
class URLLoaderThrottle;

// WebWorkerFetchContext is a per-worker object created on the main thread,
// passed to a worker (dedicated, shared and service worker) and initialized on
// the worker thread by InitializeOnWorkerThread(). It contains information
// about the resource fetching context (ex: service worker provider id), and is
// used to create a new URLLoader instance in the worker thread.
//
// A single WebWorkerFetchContext is used for both worker
// subresource fetch (i.e. "insideSettings") and off-the-main-thread top-level
// worker script fetch (i.e. fetch as "outsideSettings"), as they both should be
// e.g. controlled by the same ServiceWorker (if any) and thus can share a
// single URLLoaderFactory.
//
// Note that WebWorkerFetchContext and WorkerFetchContext do NOT correspond 1:1
// as multiple WorkerFetchContext can be created after crbug.com/880027.
class WebWorkerFetchContext : public base::RefCounted<WebWorkerFetchContext> {
 public:
  REQUIRE_ADOPTION_FOR_REFCOUNTED_TYPE();

  virtual ~WebWorkerFetchContext() = default;

  // Set a raw pointer of a WaitableEvent which will be signaled from the main
  // thread when the worker's GlobalScope is terminated, which will terminate
  // sync loading requests on the worker thread. It is guaranteed that the
  // pointer is valid throughout the lifetime of this context.
  virtual void SetTerminateSyncLoadEvent(base::WaitableEvent*) = 0;

  virtual void InitializeOnWorkerThread(AcceptLanguagesWatcher*) = 0;

  // Returns a URLLoaderFactory which is associated with the worker context.
  // The returned URLLoaderFactory is owned by |this|.
  virtual URLLoaderFactory* GetURLLoaderFactory() = 0;

  // Returns a new URLLoaderFactory that wraps the given
  // network::mojom::URLLoaderFactory.
  virtual std::unique_ptr<URLLoaderFactory> WrapURLLoaderFactory(
      CrossVariantMojoRemote<network::mojom::URLLoaderFactoryInterfaceBase>
          url_loader_factory) = 0;

  // Returns a URLLoaderFactory for loading scripts in this worker context.
  // Unlike GetURLLoaderFactory(), this may return nullptr.
  // The returned URLLoaderFactory is owned by |this|.
  virtual URLLoaderFactory* GetScriptLoaderFactory() { return nullptr; }

  // Called before a request is looked up from the cache. Allows the worker
  // to override the url.
  virtual std::optional<WebURL> WillSendRequest(const WebURL& url) {
    return std::nullopt;
  }

  // Called when a request is about to be sent out to modify the request to
  // handle the request correctly in the loading stack later. (Example: service
  // worker). Clients that need to change the url should do it in
  // OverrideRequestUrl(), not here.
  virtual void FinalizeRequest(WebURLRequest&) = 0;

  // Creates URLLoaderThrottles for the `request`.
  virtual std::vector<std::unique_ptr<URLLoaderThrottle>> CreateThrottles(
      const network::ResourceRequest& request) = 0;

  // Returns whether a controller service worker exists and if it has fetch
  // handler.
  virtual blink::mojom::ControllerServiceWorkerMode
  GetControllerServiceWorkerMode() const = 0;

  // This flag is used to block all mixed content in subframes.
  virtual void SetIsOnSubframe(bool) {}
  virtual bool IsOnSubframe() const { return false; }

  // Will be consulted for the third-party cookie blocking policy, as defined in
  // Section 2.1.1 and 2.1.2 of
  // https://tools.ietf.org/html/draft-ietf-httpbis-cookie-same-site.
  // See content::URLRequest::site_for_cookies() for details.
  virtual net::SiteForCookies SiteForCookies() const = 0;

  // The top-frame-origin for the worker. For a dedicated worker this is the
  // top-frame origin of the page that created the worker. For a shared worker
  // or a service worker this is unset.
  virtual std::optional<WebSecurityOrigin> TopFrameOrigin() const = 0;

  // Sets the builder object of WebDocumentSubresourceFilter on the main thread
  // which will be used in TakeSubresourceFilter() to create a
  // WebDocumentSubresourceFilter on the worker thread.
  virtual void SetSubresourceFilterBuilder(
      std::unique_ptr<WebDocumentSubresourceFilter::Builder>) {}

  // Creates a WebDocumentSubresourceFilter on the worker thread using the
  // WebDocumentSubresourceFilter::Builder which is set on the main thread.
  // This method should only be called once.
  virtual std::unique_ptr<WebDocumentSubresourceFilter>
  TakeSubresourceFilter() {
    return nullptr;
  }

  // Creates a WebSocketHandshakeThrottle on the worker thread. |task_runner| is
  // used for internal IPC handling of the throttle, and must be bound to the
  // same sequence to the current one (which is the worker thread).
  virtual std::unique_ptr<blink::WebSocketHandshakeThrottle>
  CreateWebSocketHandshakeThrottle(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    return nullptr;
  }

  // Returns the current list of user preferred languages.
  virtual blink::WebString GetAcceptLanguages() const = 0;

  // Creates a notifier used to notify loading stats for workers.
  virtual std::unique_ptr<blink::ResourceLoadInfoNotifierWrapper>
  CreateResourceLoadInfoNotifierWrapper() {
    return std::make_unique<blink::ResourceLoadInfoNotifierWrapper>(
        /*resource_load_info_notifier=*/nullptr);
  }

  virtual bool IsDedicatedWorkerOrSharedWorkerFetchContext() const {
    return false;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_WORKER_FETCH_CONTEXT_H_
