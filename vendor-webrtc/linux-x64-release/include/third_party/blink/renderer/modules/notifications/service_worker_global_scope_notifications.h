// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_SERVICE_WORKER_GLOBAL_SCOPE_NOTIFICATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_SERVICE_WORKER_GLOBAL_SCOPE_NOTIFICATIONS_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ServiceWorkerGlobalScopeNotifications {
  STATIC_ONLY(ServiceWorkerGlobalScopeNotifications);

 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(notificationclick, kNotificationclick)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(notificationclose, kNotificationclose)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_SERVICE_WORKER_GLOBAL_SCOPE_NOTIFICATIONS_H_
