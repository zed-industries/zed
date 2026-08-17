/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_INTER_ARRIVAL_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_INTER_ARRIVAL_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {

// Helper class to compute the inter-arrival time delta and the size delta
// between two timestamp groups. A timestamp is a 32 bit unsigned number with
// a client defined rate.
class InterArrival {
 public:
  // After this many packet groups received out of order InterArrival will
  // reset, assuming that clocks have made a jump.
  static constexpr int kReorderedResetThreshold = 3;
  static constexpr int64_t kArrivalTimeOffsetThresholdMs = 3000;

  // A timestamp group is defined as all packets with a timestamp which are at
  // most timestamp_group_length_ticks older than the first timestamp in that
  // group.
  InterArrival(uint32_t timestamp_group_length_ticks,
               double timestamp_to_ms_coeff);

  InterArrival() = delete;
  InterArrival(const InterArrival&) = delete;
  InterArrival& operator=(const InterArrival&) = delete;

  // This function returns true if a delta was computed, or false if the current
  // group is still incomplete or if only one group has been completed.
  // `timestamp` is the timestamp.
  // `arrival_time_ms` is the local time at which the packet arrived.
  // `packet_size` is the size of the packet.
  // `timestamp_delta` (output) is the computed timestamp delta.
  // `arrival_time_delta_ms` (output) is the computed arrival-time delta.
  // `packet_size_delta` (output) is the computed size delta.
  bool ComputeDeltas(uint32_t timestamp,
                     int64_t arrival_time_ms,
                     int64_t system_time_ms,
                     size_t packet_size,
                     uint32_t* timestamp_delta,
                     int64_t* arrival_time_delta_ms,
                     int* packet_size_delta);

 private:
  struct TimestampGroup {
    TimestampGroup()
        : size(0),
          first_timestamp(0),
          timestamp(0),
          first_arrival_ms(-1),
          complete_time_ms(-1) {}

    bool IsFirstPacket() const { return complete_time_ms == -1; }

    size_t size;
    uint32_t first_timestamp;
    uint32_t timestamp;
    int64_t first_arrival_ms;
    int64_t complete_time_ms;
    int64_t last_system_time_ms;
  };

  // Returns true if the packet with timestamp `timestamp` arrived in order.
  bool PacketInOrder(uint32_t timestamp);

  // Returns true if the last packet was the end of the current batch and the
  // packet with `timestamp` is the first of a new batch.
  bool NewTimestampGroup(int64_t arrival_time_ms, uint32_t timestamp) const;

  bool BelongsToBurst(int64_t arrival_time_ms, uint32_t timestamp) const;

  void Reset();

  const uint32_t kTimestampGroupLengthTicks;
  TimestampGroup current_timestamp_group_;
  TimestampGroup prev_timestamp_group_;
  double timestamp_to_ms_coeff_;
  int num_consecutive_reordered_packets_;
};
}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_INTER_ARRIVAL_H_
