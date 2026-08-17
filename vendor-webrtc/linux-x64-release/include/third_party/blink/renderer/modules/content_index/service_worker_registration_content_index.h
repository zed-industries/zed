// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_SERVICE_WORKER_REGISTRATION_CONTENT_INDEX_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_SERVICE_WORKER_REGISTRATION_CONTENT_INDEX_H_

#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ContentIndex;

class ServiceWorkerRegistrationContentIndex final
    : public GarbageCollected<ServiceWorkerRegistrationContentIndex>,
      public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  explicit ServiceWorkerRegistrationContentIndex(
      ServiceWorkerRegistration* registration);

  ServiceWorkerRegistrationContentIndex(
      const ServiceWorkerRegistrationContentIndex&) = delete;
  ServiceWorkerRegistrationContentIndex& operator=(
      const ServiceWorkerRegistrationContentIndex&) = delete;

  static ServiceWorkerRegistrationContentIndex& From(
      ServiceWorkerRegistration& registration);

  static ContentIndex* index(ServiceWorkerRegistration& registration);
  ContentIndex* index();

  void Trace(Visitor* visitor) const override;

 private:
  Member<ContentIndex> content_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_SERVICE_WORKER_REGISTRATION_CONTENT_INDEX_H_
