// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TEST_WEBRTC_STATS_REPORT_OBTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TEST_WEBRTC_STATS_REPORT_OBTAINER_H_

#include <memory>

#include "base/run_loop.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_stats.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

// The obtainer is a test-only helper class capable of waiting for a GetStats()
// callback to be called. It takes ownership of and exposes the resulting
// RTCStatsReportPlatform.
// While WaitForReport() is waiting for the report, tasks posted on the current
// thread are executed (see base::RunLoop::Run()) making it safe to wait on the
// same thread that the stats report callback occurs on without blocking the
// callback.
class TestWebRTCStatsReportObtainer
    : public WTF::ThreadSafeRefCounted<TestWebRTCStatsReportObtainer> {
 public:
  TestWebRTCStatsReportObtainer();

  RTCStatsReportCallback GetStatsCallbackWrapper();

  RTCStatsReportPlatform* report() const;
  RTCStatsReportPlatform* WaitForReport();

 private:
  friend class WTF::ThreadSafeRefCounted<TestWebRTCStatsReportObtainer>;
  friend class CallbackWrapper;
  virtual ~TestWebRTCStatsReportObtainer();

  void OnStatsDelivered(std::unique_ptr<RTCStatsReportPlatform> report);

  base::RunLoop run_loop_;
  std::unique_ptr<RTCStatsReportPlatform> report_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TEST_WEBRTC_STATS_REPORT_OBTAINER_H_
