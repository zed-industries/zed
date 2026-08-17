/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_CALL_BITRATE_ALLOCATION_H_
#define API_CALL_BITRATE_ALLOCATION_H_

#include "api/units/data_rate.h"
#include "api/units/time_delta.h"

namespace webrtc {

// BitrateAllocationUpdate provides information to allocated streams about their
// bitrate allocation. It originates from the BitrateAllocater class and is
// propagated from there.
struct BitrateAllocationUpdate {
  // The allocated target bitrate. Media streams should produce this amount of
  // data. (Note that this may include packet overhead depending on
  // configuration.)
  DataRate target_bitrate = DataRate::Zero();
  // The allocated part of the estimated link capacity. This is more stable than
  // the target as it is based on the underlying link capacity estimate. This
  // should be used to change encoder configuration when the cost of change is
  // high.
  DataRate stable_target_bitrate = DataRate::Zero();
  // Predicted packet loss ratio.
  double packet_loss_ratio = 0;
  // Predicted round trip time.
  TimeDelta round_trip_time = TimeDelta::PlusInfinity();
  // `bwe_period` is deprecated, use `stable_target_bitrate` allocation instead.
  TimeDelta bwe_period = TimeDelta::PlusInfinity();
  // Congestion window pushback bitrate reduction fraction. Used in
  // VideoStreamEncoder to reduce the bitrate by the given fraction
  // by dropping frames.
  double cwnd_reduce_ratio = 0;
};

}  // namespace webrtc

#endif  // API_CALL_BITRATE_ALLOCATION_H_
