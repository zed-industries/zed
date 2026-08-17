// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_EVENT_LOG_OUTPUT_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_EVENT_LOG_OUTPUT_SINK_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT RtcEventLogOutputSink : public GarbageCollectedMixin {
 public:
  virtual ~RtcEventLogOutputSink() = default;

  virtual void OnWebRtcEventLogWrite(const WTF::Vector<uint8_t>& output) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_EVENT_LOG_OUTPUT_SINK_H_
