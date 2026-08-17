/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_INTER_ARRIVAL_DELTA_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_INTER_ARRIVAL_DELTA_H_

#include <cstddef>

#include "api/units/time_delta.h"
#include "api/units/timestamp.h"

namespace webrtc {

// Helper class to compute the inter-arrival time delta and the size delta
// between two send bursts. This code is branched from
// modules/remote_bitrate_estimator/inter_arrival.
class InterArrivalDelta {
 public:
  // After this many packet groups received out of order InterArrival will
  // reset, assuming that clocks have made a jump.
  static constexpr int kReorderedResetThreshold = 3;
  static constexpr TimeDelta kArrivalTimeOffsetThreshold =
      TimeDelta::Seconds(3);

  // A send time group is defined as all packets with a send time which are at
  // most send_time_group_length older than the first timestamp in that
  // group.
  explicit InterArrivalDelta(TimeDelta send_time_group_length);

  InterArrivalDelta() = delete;
  InterArrivalDelta(const InterArrivalDelta&) = delete;
  InterArrivalDelta& operator=(const InterArrivalDelta&) = delete;

  // This function returns true if a delta was computed, or false if the current
  // group is still incomplete or if only one group has been completed.
  // `send_time` is the send time.
  // `arrival_time` is the time at which the packet arrived.
  // `packet_size` is the size of the packet.
  // `timestamp_delta` (output) is the computed send time delta.
  // `arrival_time_delta` (output) is the computed arrival-time delta.
  // `packet_size_delta` (output) is the computed size delta.
  bool ComputeDeltas(Timestamp send_time,
                     Timestamp arrival_time,
                     Timestamp system_time,
                     size_t packet_size,
                     TimeDelta* send_time_delta,
                     TimeDelta* arrival_time_delta,
                     int* packet_size_delta);

 private:
  struct SendTimeGroup {
    SendTimeGroup()
        : size(0),
          first_send_time(Timestamp::MinusInfinity()),
          send_time(Timestamp::MinusInfinity()),
          first_arrival(Timestamp::MinusInfinity()),
          complete_time(Timestamp::MinusInfinity()),
          last_system_time(Timestamp::MinusInfinity()) {}

    bool IsFirstPacket() const { return complete_time.IsInfinite(); }

    size_t size;
    Timestamp first_send_time;
    Timestamp send_time;
    Timestamp first_arrival;
    Timestamp complete_time;
    Timestamp last_system_time;
  };

  // Returns true if the last packet was the end of the current batch and the
  // packet with `send_time` is the first of a new batch.
  bool NewTimestampGroup(Timestamp arrival_time, Timestamp send_time) const;

  bool BelongsToBurst(Timestamp arrival_time, Timestamp send_time) const;

  void Reset();

  const TimeDelta send_time_group_length_;
  SendTimeGroup current_timestamp_group_;
  SendTimeGroup prev_timestamp_group_;
  int num_consecutive_reordered_packets_;
};
}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_INTER_ARRIVAL_DELTA_H_
