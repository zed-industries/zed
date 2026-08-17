// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SERVICE_WORKER_GLOBAL_SCOPE_SYNC_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SERVICE_WORKER_GLOBAL_SCOPE_SYNC_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"

namespace blink {

class ServiceWorkerGlobalScopeSync {
 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(periodicsync, kPeriodicsync)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(sync, kSync)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_SYNC_SERVICE_WORKER_GLOBAL_SCOPE_SYNC_H_
