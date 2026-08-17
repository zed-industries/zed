/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_STATS_RTC_STATS_COLLECTOR_CALLBACK_H_
#define API_STATS_RTC_STATS_COLLECTOR_CALLBACK_H_

#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "api/stats/rtc_stats_report.h"

namespace webrtc {

class RTCStatsCollectorCallback : public RefCountInterface {
 public:
  ~RTCStatsCollectorCallback() override = default;

  virtual void OnStatsDelivered(
      const scoped_refptr<const RTCStatsReport>& report) = 0;
};

}  // namespace webrtc

#endif  // API_STATS_RTC_STATS_COLLECTOR_CALLBACK_H_
