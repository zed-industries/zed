// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_SERVICE_WORKER_GLOBAL_SCOPE_BACKGROUND_FETCH_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_SERVICE_WORKER_GLOBAL_SCOPE_BACKGROUND_FETCH_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ServiceWorkerGlobalScopeBackgroundFetch {
  STATIC_ONLY(ServiceWorkerGlobalScopeBackgroundFetch);

 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(backgroundfetchsuccess,
                                         kBackgroundfetchsuccess)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(backgroundfetchfail,
                                         kBackgroundfetchfail)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(backgroundfetchabort,
                                         kBackgroundfetchabort)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(backgroundfetchclick,
                                         kBackgroundfetchclick)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_SERVICE_WORKER_GLOBAL_SCOPE_BACKGROUND_FETCH_H_
