// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_REGISTRATION_OBJECT_INFO_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_REGISTRATION_OBJECT_INFO_H_

#include "third_party/blink/public/mojom/service_worker/service_worker_registration.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_registration_options.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_object_info.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

// This is to carry blink.mojom.ServiceWorkerRegistrationObjectInfo data from
// //content across the boundary into Blink.
// TODO(crbug.com/879019): Remove this class once we make the following Mojo
// interfaces receive blink.mojom.ServiceWorkerRegistrationObjectInfo directly
// inside Blink.
//  - content.mojom.ServiceWorker
//  - content.mojom.ServiceWorkerContainer
struct WebServiceWorkerRegistrationObjectInfo {
  WebServiceWorkerRegistrationObjectInfo(
      int64_t registration_id,
      WebURL scope,
      mojom::ServiceWorkerUpdateViaCache update_via_cache,
      CrossVariantMojoAssociatedRemote<
          mojom::ServiceWorkerRegistrationObjectHostInterfaceBase> host_remote,
      CrossVariantMojoAssociatedReceiver<
          mojom::ServiceWorkerRegistrationObjectInterfaceBase> receiver,
      WebServiceWorkerObjectInfo installing,
      WebServiceWorkerObjectInfo waiting,
      WebServiceWorkerObjectInfo active)
      : registration_id(registration_id),
        scope(std::move(scope)),
        update_via_cache(update_via_cache),
        host_remote(std::move(host_remote)),
        receiver(std::move(receiver)),
        installing(std::move(installing)),
        waiting(std::move(waiting)),
        active(std::move(active)) {}
  WebServiceWorkerRegistrationObjectInfo(
      WebServiceWorkerRegistrationObjectInfo&& other) = default;

  WebServiceWorkerRegistrationObjectInfo(
      const WebServiceWorkerRegistrationObjectInfo&) = delete;
  WebServiceWorkerRegistrationObjectInfo& operator=(
      const WebServiceWorkerRegistrationObjectInfo&) = delete;

  int64_t registration_id;

  WebURL scope;
  mojom::ServiceWorkerUpdateViaCache update_via_cache;

  CrossVariantMojoAssociatedRemote<
      mojom::ServiceWorkerRegistrationObjectHostInterfaceBase>
      host_remote;
  CrossVariantMojoAssociatedReceiver<
      mojom::ServiceWorkerRegistrationObjectInterfaceBase>
      receiver;

  WebServiceWorkerObjectInfo installing;
  WebServiceWorkerObjectInfo waiting;
  WebServiceWorkerObjectInfo active;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_REGISTRATION_OBJECT_INFO_H_
