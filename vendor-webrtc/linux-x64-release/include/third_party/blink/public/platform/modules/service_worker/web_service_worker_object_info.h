// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_OBJECT_INFO_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_OBJECT_INFO_H_

#include "third_party/blink/public/mojom/service_worker/service_worker_object.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_state.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

// This is to carry blink.mojom.ServiceWorkerObjectInfo data from //content
// across the boundary into Blink.
// TODO(crbug.com/879019): Remove this class once we make the following Mojo
// interfaces receive blink.mojom.ServiceWorkerObjectInfo directly inside Blink.
//  - content.mojom.ServiceWorkerContainer
struct WebServiceWorkerObjectInfo {
  WebServiceWorkerObjectInfo(
      int64_t version_id,
      mojom::ServiceWorkerState state,
      WebURL url,
      CrossVariantMojoAssociatedRemote<
          mojom::ServiceWorkerObjectHostInterfaceBase> host_remote,
      CrossVariantMojoAssociatedReceiver<
          mojom::ServiceWorkerObjectInterfaceBase> receiver)
      : version_id(version_id),
        state(state),
        url(std::move(url)),
        host_remote(std::move(host_remote)),
        receiver(std::move(receiver)) {}
  WebServiceWorkerObjectInfo(WebServiceWorkerObjectInfo&& other) = default;

  WebServiceWorkerObjectInfo(const WebServiceWorkerObjectInfo&) = delete;
  WebServiceWorkerObjectInfo& operator=(const WebServiceWorkerObjectInfo&) =
      delete;

  int64_t version_id;
  mojom::ServiceWorkerState state;
  WebURL url;
  CrossVariantMojoAssociatedRemote<mojom::ServiceWorkerObjectHostInterfaceBase>
      host_remote;
  CrossVariantMojoAssociatedReceiver<mojom::ServiceWorkerObjectInterfaceBase>
      receiver;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_OBJECT_INFO_H_
