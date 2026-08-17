/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_PACKET_ARRIVAL_HISTORY_H_
#define MODULES_AUDIO_CODING_NETEQ_PACKET_ARRIVAL_HISTORY_H_

#include <cstddef>
#include <cstdint>
#include <deque>
#include <map>

#include "api/neteq/tick_timer.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"

namespace webrtc {

// Stores timing information about previously received packets.
// The history has a fixed window size beyond which old data is automatically
// pruned.
class PacketArrivalHistory {
 public:
  explicit PacketArrivalHistory(const TickTimer* tick_timer,
                                int window_size_ms);
  virtual ~PacketArrivalHistory() = default;

  // Insert packet with `rtp_timestamp` into the history. Returns true if the
  // packet was inserted, false if the timestamp is too old or if the timestamp
  // already exists.
  bool Insert(uint32_t rtp_timestamp, int packet_length_samples);

  // The delay for `rtp_timestamp` at time `now` is calculated as
  // `(now - p.arrival_timestamp) - (rtp_timestamp - p.rtp_timestamp)` where `p`
  // is chosen as the packet arrival in the history that maximizes the delay.
  virtual int GetDelayMs(uint32_t rtp_timestamp) const;

  // Get the maximum packet arrival delay observed in the history, excluding
  // reordered packets.
  virtual int GetMaxDelayMs() const;

  bool IsNewestRtpTimestamp(uint32_t rtp_timestamp) const;

  void Reset();

  void set_sample_rate(int sample_rate) {
    sample_rate_khz_ = sample_rate / 1000;
  }

  size_t size() const { return history_.size(); }

 private:
  struct PacketArrival {
    PacketArrival(int64_t rtp_timestamp,
                  int64_t arrival_timestamp,
                  int length_samples)
        : rtp_timestamp(rtp_timestamp),
          arrival_timestamp(arrival_timestamp),
          length_samples(length_samples) {}
    PacketArrival() = default;
    int64_t rtp_timestamp;
    int64_t arrival_timestamp;
    int length_samples;
    bool operator==(const PacketArrival& other) const {
      return rtp_timestamp == other.rtp_timestamp &&
             arrival_timestamp == other.arrival_timestamp &&
             length_samples == other.length_samples;
    }
    bool operator!=(const PacketArrival& other) const {
      return !(*this == other);
    }
    bool operator<=(const PacketArrival& other) const {
      return arrival_timestamp - rtp_timestamp <=
             other.arrival_timestamp - other.rtp_timestamp;
    }
    bool operator>=(const PacketArrival& other) const {
      return arrival_timestamp - rtp_timestamp >=
             other.arrival_timestamp - other.rtp_timestamp;
    }
    bool contains(const PacketArrival& other) const {
      return rtp_timestamp <= other.rtp_timestamp &&
             rtp_timestamp + length_samples >=
                 other.rtp_timestamp + other.length_samples;
    }
  };
  int GetPacketArrivalDelayMs(const PacketArrival& packet_arrival) const;
  // Checks if the packet is older than the window size.
  bool IsObsolete(const PacketArrival& packet_arrival) const;
  // Check if the packet exists or fully overlaps with a packet in the history.
  bool Contains(const PacketArrival& packet_arrival) const;
  const TickTimer* tick_timer_;
  const int window_size_ms_;
  int sample_rate_khz_ = 0;
  RtpTimestampUnwrapper timestamp_unwrapper_;
  // Packet history ordered by rtp timestamp.
  std::map<int64_t, PacketArrival> history_;
  // Tracks min/max packet arrivals in `history_` in ascending/descending order.
  // Reordered packets are excluded.
  std::deque<PacketArrival> min_packet_arrivals_;
  std::deque<PacketArrival> max_packet_arrivals_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_PACKET_ARRIVAL_HISTORY_H_
