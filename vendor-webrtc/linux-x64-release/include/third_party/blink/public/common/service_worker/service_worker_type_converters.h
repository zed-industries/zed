// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_TYPE_CONVERTERS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/service_worker/service_worker_status_code.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_event_status.mojom.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    TypeConverter<blink::ServiceWorkerStatusCode,
                  blink::mojom::ServiceWorkerEventStatus> {
  static blink::ServiceWorkerStatusCode Convert(
      blink::mojom::ServiceWorkerEventStatus status);
};

template <>
struct BLINK_COMMON_EXPORT
    TypeConverter<blink::ServiceWorkerStatusCode,
                  blink::mojom::ServiceWorkerStartStatus> {
  static blink::ServiceWorkerStatusCode Convert(
      blink::mojom::ServiceWorkerStartStatus status);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_TYPE_CONVERTERS_H_
