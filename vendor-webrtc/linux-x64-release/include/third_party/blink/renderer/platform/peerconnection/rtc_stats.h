// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_STATS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_STATS_H_

#include <vector>

#include "base/feature_list.h"
#include "base/functional/callback.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/api/stats/rtc_stats.h"
#include "third_party/webrtc/api/stats/rtc_stats_collector_callback.h"
#include "third_party/webrtc/api/stats/rtc_stats_report.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace webrtc {
class RTCStatsCollectorCallback;
enum class NonStandardGroupId;
}  // namespace webrtc

namespace blink {

PLATFORM_EXPORT BASE_DECLARE_FEATURE(WebRtcUnshipDeprecatedStats);

// A thin wrapper around a webrtc::RTCStatsReport.
// TODO(https://crbug.com/1443999): Delete this class, it does not provide any
// value anymore and blink is allowed to use webrtc classes directly.
class PLATFORM_EXPORT RTCStatsReportPlatform {
 public:
  explicit RTCStatsReportPlatform(
      const scoped_refptr<const webrtc::RTCStatsReport>& stats_report);
  virtual ~RTCStatsReportPlatform();

  // Creates a new report object that is a handle to the same underlying stats
  // report (the stats are not copied). The new report's iterator is reset,
  // useful when needing multiple iterators.
  std::unique_ptr<RTCStatsReportPlatform> CopyHandle() const;

  const webrtc::RTCStatsReport& stats_report() const { return *stats_report_; }
  const webrtc::RTCStats* NextStats();

  // The number of stats objects.
  size_t Size() const;

 private:
  const scoped_refptr<const webrtc::RTCStatsReport> stats_report_;
  webrtc::RTCStatsReport::ConstIterator it_;
  const webrtc::RTCStatsReport::ConstIterator end_;
};

using RTCStatsReportCallback =
    base::OnceCallback<void(std::unique_ptr<RTCStatsReportPlatform>)>;

PLATFORM_EXPORT
webrtc::scoped_refptr<webrtc::RTCStatsCollectorCallback>
CreateRTCStatsCollectorCallback(
    scoped_refptr<base::SingleThreadTaskRunner> main_thread,
    RTCStatsReportCallback callback);

// A stats collector callback.
// It is invoked on the WebRTC signaling thread and will post a task to invoke
// |callback| on the thread given in the |main_thread| argument.
// The argument to the callback will be a |RTCStatsReportPlatform|.
class PLATFORM_EXPORT RTCStatsCollectorCallbackImpl
    : public webrtc::RTCStatsCollectorCallback {
 public:
  void OnStatsDelivered(
      const webrtc::scoped_refptr<const webrtc::RTCStatsReport>& report)
      override;

 protected:
  RTCStatsCollectorCallbackImpl(
      scoped_refptr<base::SingleThreadTaskRunner> main_thread,
      RTCStatsReportCallback callback);
  ~RTCStatsCollectorCallbackImpl() override;

  void OnStatsDeliveredOnMainThread(
      webrtc::scoped_refptr<const webrtc::RTCStatsReport> report);

  const scoped_refptr<base::SingleThreadTaskRunner> main_thread_;
  RTCStatsReportCallback callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_STATS_H_
