// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_OR_SHARED_WORKER_FETCH_CONTEXT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_OR_SHARED_WORKER_FETCH_CONTEXT_H_

#include <memory>
#include <string_view>
#include <vector>

#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "third_party/blink/public/common/renderer_preferences/renderer_preferences.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/loader/resource_load_info_notifier.mojom-forward.h"
#include "third_party/blink/public/mojom/renderer_preference_watcher.mojom-shared.h"
#include "third_party/blink/public/mojom/worker/subresource_loader_updater.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_worker_fetch_context.h"

namespace blink {

class WebString;
class WebServiceWorkerProviderContext;

// Worker fetch context for dedicated worker or shared worker.
class BLINK_PLATFORM_EXPORT WebDedicatedOrSharedWorkerFetchContext
    : public WebWorkerFetchContext {
 public:
  // Creates a new fetch context for a worker.
  //
  // |provider_context| is used to set up information for using service workers.
  // It can be null if the worker is not allowed to use service workers due to
  // security reasons like sandboxed iframes, insecure origins etc.
  // |pending_loader_factory| is used for regular loading by the worker.
  //
  // If the worker is controlled by a service worker, this class makes another
  // loader factory which sends requests to the service worker, and passes
  // |pending_fallback_factory| to that factory to use for network fallback.
  //
  // |pending_loader_factory| and |pending_fallback_factory| are different
  // because |pending_loader_factory| can possibly include a default factory
  // like AppCache, while |pending_fallback_factory| should not have such a
  // default factory and instead go directly to network for http(s) requests.
  // |pending_fallback_factory| might not be simply the direct network factory,
  // because it might additionally support non-NetworkService schemes (e.g.,
  // chrome-extension://).
  static scoped_refptr<WebDedicatedOrSharedWorkerFetchContext> Create(
      WebServiceWorkerProviderContext* provider_context,
      const RendererPreferences& renderer_preferences,
      CrossVariantMojoReceiver<mojom::RendererPreferenceWatcherInterfaceBase>
          watcher_receiver,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_loader_factory,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_fallback_factory,
      CrossVariantMojoReceiver<mojom::SubresourceLoaderUpdaterInterfaceBase>
          pending_subresource_loader_updater,
      const std::vector<WebString>& cors_exempt_header_list,
      mojo::PendingRemote<mojom::ResourceLoadInfoNotifier>
          pending_resource_load_info_notifier);

  // The cloned fetch context does not inherit some fields (e.g.,
  // blink::WebServiceWorkerProviderContext) from this fetch context, and
  // instead that takes values passed from the browser process.
  virtual scoped_refptr<WebDedicatedOrSharedWorkerFetchContext>
  CloneForNestedWorker(
      WebServiceWorkerProviderContext* service_worker_provider_context,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_loader_factory,
      std::unique_ptr<network::PendingSharedURLLoaderFactory>
          pending_fallback_factory,
      CrossVariantMojoReceiver<mojom::SubresourceLoaderUpdaterInterfaceBase>
          pending_subresource_loader_updater,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) = 0;

  // Sets properties associated with frames.
  // - For dedicated workers, the property is copied from the ancestor frame
  //   (directly for non-nested workers, or indirectly via its parent worker for
  //   nested workers).
  // - For shared workers, there is no parent frame, so the default value, or a
  //   value calculated in some way is set.
  //
  // TODO(nhiroki): Add more comments about security/privacy implications to
  // each property, for example, site_for_cookies and top_frame_origin.
  virtual void SetAncestorFrameToken(const LocalFrameToken&) = 0;
  virtual void set_site_for_cookies(
      const net::SiteForCookies& site_for_cookies) = 0;
  virtual void set_top_frame_origin(
      const blink::WebSecurityOrigin& top_frame_origin) = 0;

  // TODO(crbug.com/324939068): remove the code when the feature launched.
  virtual void set_container_is_shared_worker(bool is_sharedworker) = 0;

  using RewriteURLFunction = WebURL (*)(std::string_view, bool);
  static void InstallRewriteURLFunction(RewriteURLFunction rewrite_url);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_OR_SHARED_WORKER_FETCH_CONTEXT_H_
