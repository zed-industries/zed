// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_PROVIDER_CONTEXT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_PROVIDER_CONTEXT_H_

#include "third_party/blink/public/mojom/service_worker/controller_service_worker_mode.mojom-forward.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_container.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_fetch_handler_bypass_option.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_fetch_handler_type.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_provider.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"

namespace blink {

class WebString;

// WebServiceWorkerProviderContext is an abstract class implemented by
// content::ServiceWorkerProviderContext.
class WebServiceWorkerProviderContext {
 public:
  // Binds a new ServiceWorkerWorkerClient endpoint.
  virtual void BindServiceWorkerWorkerClientRemote(
      CrossVariantMojoRemote<mojom::ServiceWorkerWorkerClientInterfaceBase>
          pending_client) = 0;

  // Binds a new ServiceWorkerWorkerClientRegistry endpoint.
  virtual void BindServiceWorkerWorkerClientRegistryReceiver(
      CrossVariantMojoReceiver<
          mojom::ServiceWorkerWorkerClientRegistryInterfaceBase> receiver) = 0;

  // Returns a remote to this context's container host. This can return null
  // after OnNetworkProviderDestroyed() is called (in which case |this| will be
  // destroyed soon).
  virtual CrossVariantMojoRemote<mojom::ServiceWorkerContainerHostInterfaceBase>
  CloneRemoteContainerHost() = 0;

  virtual mojom::ControllerServiceWorkerMode GetControllerServiceWorkerMode()
      const = 0;
  virtual mojom::ServiceWorkerFetchHandlerType GetFetchHandlerType() const = 0;
  virtual mojom::ServiceWorkerFetchHandlerBypassOption
  GetFetchHandlerBypassOption() const = 0;

  // The Client#id value of this context.
  virtual const WebString client_id() const = 0;

  // TODO(crbug.com/324939068): remove the code when the feature launched.
  virtual bool container_is_blob_url_shared_worker() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_PROVIDER_CONTEXT_H_
