// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_EMBEDDED_WORKER_STATUS_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_EMBEDDED_WORKER_STATUS_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/enum_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/service_worker/embedded_worker_status.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_embedded_worker_status.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::ServiceWorkerEmbeddedWorkerStatus,
               blink::EmbeddedWorkerStatus> {
  static blink::mojom::ServiceWorkerEmbeddedWorkerStatus ToMojom(
      blink::EmbeddedWorkerStatus input) {
    switch (input) {
      case blink::EmbeddedWorkerStatus::kStopped:
        return blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kStopped;
      case blink::EmbeddedWorkerStatus::kStarting:
        return blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kStarting;
      case blink::EmbeddedWorkerStatus::kRunning:
        return blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kRunning;
      case blink::EmbeddedWorkerStatus::kStopping:
        return blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kStopping;
    }
  }

  static bool FromMojom(blink::mojom::ServiceWorkerEmbeddedWorkerStatus input,
                        blink::EmbeddedWorkerStatus* output) {
    switch (input) {
      case blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kStopped:
        *output = blink::EmbeddedWorkerStatus::kStopped;
        break;
      case blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kStarting:
        *output = blink::EmbeddedWorkerStatus::kStarting;
        break;
      case blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kRunning:
        *output = blink::EmbeddedWorkerStatus::kRunning;
        break;
      case blink::mojom::ServiceWorkerEmbeddedWorkerStatus::kStopping:
        *output = blink::EmbeddedWorkerStatus::kStopping;
        break;
    }
    return true;
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_EMBEDDED_WORKER_STATUS_MOJOM_TRAITS_H_
