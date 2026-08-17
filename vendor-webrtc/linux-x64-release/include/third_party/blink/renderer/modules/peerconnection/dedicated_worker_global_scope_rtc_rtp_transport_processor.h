// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_DEDICATED_WORKER_GLOBAL_SCOPE_RTC_RTP_TRANSPORT_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_DEDICATED_WORKER_GLOBAL_SCOPE_RTC_RTP_TRANSPORT_PROCESSOR_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"

namespace blink {

class DedicatedWorkerGlobalScopeRTCRtpTransportProcessor {
  STATIC_ONLY(DedicatedWorkerGlobalScopeRTCRtpTransportProcessor);

 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(rtcrtptransportprocessor,
                                         kRtcrtptransportprocessor)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_DEDICATED_WORKER_GLOBAL_SCOPE_RTC_RTP_TRANSPORT_PROCESSOR_H_
