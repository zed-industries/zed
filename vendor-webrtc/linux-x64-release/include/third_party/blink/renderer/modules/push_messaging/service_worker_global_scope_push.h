// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_SERVICE_WORKER_GLOBAL_SCOPE_PUSH_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_SERVICE_WORKER_GLOBAL_SCOPE_PUSH_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"

namespace blink {

class ServiceWorkerGlobalScopePush {
 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(push, kPush)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(pushsubscriptionchange,
                                         kPushsubscriptionchange)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_SERVICE_WORKER_GLOBAL_SCOPE_PUSH_H_
