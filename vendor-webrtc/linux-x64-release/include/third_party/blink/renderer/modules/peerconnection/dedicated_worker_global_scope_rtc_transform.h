// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_DEDICATED_WORKER_GLOBAL_SCOPE_RTC_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_DEDICATED_WORKER_GLOBAL_SCOPE_RTC_TRANSFORM_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"

namespace blink {

class DedicatedWorkerGlobalScopeRTCTransform {
  STATIC_ONLY(DedicatedWorkerGlobalScopeRTCTransform);

 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(rtctransform, kRtctransform)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_DEDICATED_WORKER_GLOBAL_SCOPE_RTC_TRANSFORM_H_
