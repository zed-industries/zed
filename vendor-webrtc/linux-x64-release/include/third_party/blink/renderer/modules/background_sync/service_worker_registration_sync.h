// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SERVICE_WORKER_REGISTRATION_SYNC_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SERVICE_WORKER_REGISTRATION_SYNC_H_

#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class PeriodicSyncManager;
class SyncManager;
class ServiceWorkerRegistration;

class ServiceWorkerRegistrationSync final
    : public GarbageCollected<ServiceWorkerRegistrationSync>,
      public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  static ServiceWorkerRegistrationSync& From(
      ServiceWorkerRegistration& registration);

  static PeriodicSyncManager* periodicSync(
      ServiceWorkerRegistration& registration);
  static SyncManager* sync(ServiceWorkerRegistration& registration);

  explicit ServiceWorkerRegistrationSync(
      ServiceWorkerRegistration* registration);

  ServiceWorkerRegistrationSync(const ServiceWorkerRegistrationSync&) = delete;
  ServiceWorkerRegistrationSync& operator=(
      const ServiceWorkerRegistrationSync&) = delete;

  ~ServiceWorkerRegistrationSync();

  PeriodicSyncManager* periodicSync();
  SyncManager* sync();

  void Trace(Visitor*) const override;

 private:
  Member<SyncManager> sync_manager_;
  Member<PeriodicSyncManager> periodic_sync_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SERVICE_WORKER_REGISTRATION_SYNC_H_
