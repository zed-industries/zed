/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_INCLUDE_BWE_DEFINES_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_INCLUDE_BWE_DEFINES_H_

#include <stdint.h>

#include <optional>

#include "api/transport/bandwidth_usage.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"

namespace webrtc {

inline constexpr DataRate kCongestionControllerMinBitrate =
    DataRate::BitsPerSec(5'000);
inline constexpr TimeDelta kBitrateWindow = TimeDelta::Seconds(1);

extern const char kBweTypeHistogram[];

enum BweNames {
  kReceiverNoExtension = 0,
  kReceiverTOffset = 1,
  kReceiverAbsSendTime = 2,
  kSendSideTransportSeqNum = 3,
  kBweNamesMax = 4
};

struct RateControlInput {
  RateControlInput(BandwidthUsage bw_state,
                   const std::optional<DataRate>& estimated_throughput);
  ~RateControlInput();

  BandwidthUsage bw_state;
  std::optional<DataRate> estimated_throughput;
};
}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_INCLUDE_BWE_DEFINES_H_
