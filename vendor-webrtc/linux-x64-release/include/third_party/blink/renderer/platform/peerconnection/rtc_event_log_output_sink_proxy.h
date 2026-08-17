// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_EVENT_LOG_OUTPUT_SINK_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_EVENT_LOG_OUTPUT_SINK_PROXY_H_

#include <string_view>

#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/rtc_event_log_output.h"

namespace webrtc {
class RtcEventLogOutput;
}

namespace blink {

class RtcEventLogOutputSink;

class PLATFORM_EXPORT RtcEventLogOutputSinkProxy final
    : public webrtc::RtcEventLogOutput {
 public:
  RtcEventLogOutputSinkProxy(RtcEventLogOutputSink* sink);
  ~RtcEventLogOutputSinkProxy() override;

  bool IsActive() const override;

  bool Write(std::string_view output) override;

 private:
  CrossThreadWeakPersistent<RtcEventLogOutputSink> sink_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_EVENT_LOG_OUTPUT_SINK_PROXY_H_
