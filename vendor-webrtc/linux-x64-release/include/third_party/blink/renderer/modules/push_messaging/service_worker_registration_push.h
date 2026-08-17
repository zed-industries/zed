// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_SERVICE_WORKER_REGISTRATION_PUSH_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_SERVICE_WORKER_REGISTRATION_PUSH_H_

#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class PushManager;
class ServiceWorkerRegistration;

class ServiceWorkerRegistrationPush final
    : public GarbageCollected<ServiceWorkerRegistrationPush>,
      public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  explicit ServiceWorkerRegistrationPush(
      ServiceWorkerRegistration* registration);

  ServiceWorkerRegistrationPush(const ServiceWorkerRegistrationPush&) = delete;
  ServiceWorkerRegistrationPush& operator=(
      const ServiceWorkerRegistrationPush&) = delete;

  ~ServiceWorkerRegistrationPush();
  static ServiceWorkerRegistrationPush& From(
      ServiceWorkerRegistration& registration);

  static PushManager* pushManager(ServiceWorkerRegistration& registration);
  PushManager* pushManager();

  void Trace(Visitor* visitor) const override;

 private:
  Member<PushManager> push_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_SERVICE_WORKER_REGISTRATION_PUSH_H_
