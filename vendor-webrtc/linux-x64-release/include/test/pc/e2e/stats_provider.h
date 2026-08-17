/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_STATS_PROVIDER_H_
#define TEST_PC_E2E_STATS_PROVIDER_H_

#include "api/stats/rtc_stats_collector_callback.h"

namespace webrtc {
namespace webrtc_pc_e2e {

class StatsProvider {
 public:
  virtual ~StatsProvider() = default;

  virtual void GetStats(RTCStatsCollectorCallback* callback) = 0;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_STATS_PROVIDER_H_
