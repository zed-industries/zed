// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_SERVICE_WORKER_REGISTRATION_BACKGROUND_FETCH_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_SERVICE_WORKER_REGISTRATION_BACKGROUND_FETCH_H_

#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class BackgroundFetchManager;

class ServiceWorkerRegistrationBackgroundFetch final
    : public GarbageCollected<ServiceWorkerRegistrationBackgroundFetch>,
      public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  explicit ServiceWorkerRegistrationBackgroundFetch(
      ServiceWorkerRegistration* registration);

  ServiceWorkerRegistrationBackgroundFetch(
      const ServiceWorkerRegistrationBackgroundFetch&) = delete;
  ServiceWorkerRegistrationBackgroundFetch& operator=(
      const ServiceWorkerRegistrationBackgroundFetch&) = delete;

  ~ServiceWorkerRegistrationBackgroundFetch();

  static ServiceWorkerRegistrationBackgroundFetch& From(
      ServiceWorkerRegistration& registration);

  static BackgroundFetchManager* backgroundFetch(
      ServiceWorkerRegistration& registration);
  BackgroundFetchManager* backgroundFetch();

  void Trace(Visitor* visitor) const override;

 private:
  Member<BackgroundFetchManager> background_fetch_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_SERVICE_WORKER_REGISTRATION_BACKGROUND_FETCH_H_
