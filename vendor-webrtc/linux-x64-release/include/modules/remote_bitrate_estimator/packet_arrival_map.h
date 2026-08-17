/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_PACKET_ARRIVAL_MAP_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_PACKET_ARRIVAL_MAP_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <memory>

#include "api/units/timestamp.h"
#include "rtc_base/checks.h"

namespace webrtc {

// PacketArrivalTimeMap is an optimized map of packet sequence number to arrival
// time, limited in size to never exceed `kMaxNumberOfPackets`. It will grow as
// needed, and remove old packets, and will expand to allow earlier packets to
// be added (out-of-order).
//
// Not yet received packets have the arrival time zero. The queue will not span
// larger than necessary and the last packet should always be received. The
// first packet in the queue doesn't have to be received in case of receiving
// packets out-of-order.
class PacketArrivalTimeMap {
 public:
  struct PacketArrivalTime {
    Timestamp arrival_time;
    int64_t sequence_number;
  };
  // Impossible to request feedback older than what can be represented by 15
  // bits.
  static constexpr int kMaxNumberOfPackets = (1 << 15);

  PacketArrivalTimeMap() = default;
  PacketArrivalTimeMap(const PacketArrivalTimeMap&) = delete;
  PacketArrivalTimeMap& operator=(const PacketArrivalTimeMap&) = delete;
  ~PacketArrivalTimeMap() = default;

  // Indicates if the packet with `sequence_number` has already been received.
  bool has_received(int64_t sequence_number) const {
    return sequence_number >= begin_sequence_number() &&
           sequence_number < end_sequence_number() &&
           arrival_times_[Index(sequence_number)] >= Timestamp::Zero();
  }

  // Returns the sequence number of the first entry in the map, i.e. the
  // sequence number that a `begin()` iterator would represent.
  int64_t begin_sequence_number() const { return begin_sequence_number_; }

  // Returns the sequence number of the element just after the map, i.e. the
  // sequence number that an `end()` iterator would represent.
  int64_t end_sequence_number() const { return end_sequence_number_; }

  // Returns an element by `sequence_number`, which must be valid, i.e.
  // between [begin_sequence_number, end_sequence_number).
  Timestamp get(int64_t sequence_number) {
    RTC_DCHECK_GE(sequence_number, begin_sequence_number());
    RTC_DCHECK_LT(sequence_number, end_sequence_number());
    return arrival_times_[Index(sequence_number)];
  }

  // Returns timestamp and sequence number of the received packet with sequence
  // number equal or larger than `sequence_number`. `sequence_number` must be in
  // range [begin_sequence_number, end_sequence_number).
  PacketArrivalTime FindNextAtOrAfter(int64_t sequence_number) const {
    RTC_DCHECK_GE(sequence_number, begin_sequence_number());
    RTC_DCHECK_LT(sequence_number, end_sequence_number());
    while (true) {
      Timestamp t = arrival_times_[Index(sequence_number)];
      if (t >= Timestamp::Zero()) {
        return {.arrival_time = t, .sequence_number = sequence_number};
      }
      ++sequence_number;
    }
  }

  // Clamps `sequence_number` between [begin_sequence_number,
  // end_sequence_number].
  int64_t clamp(int64_t sequence_number) const {
    return std::clamp(sequence_number, begin_sequence_number(),
                      end_sequence_number());
  }

  // Erases all elements from the beginning of the map until `sequence_number`.
  void EraseTo(int64_t sequence_number);

  // Records the fact that a packet with `sequence_number` arrived at
  // `arrival_time_ms`.
  void AddPacket(int64_t sequence_number, Timestamp arrival_time);

  // Removes packets from the beginning of the map as long as they are received
  // before `sequence_number` and with an age older than `arrival_time_limit`
  void RemoveOldPackets(int64_t sequence_number, Timestamp arrival_time_limit);

 private:
  static constexpr int kMinCapacity = 128;

  // Returns index in the `arrival_times_` for value for `sequence_number`.
  int Index(int64_t sequence_number) const {
    // Note that sequence_number might be negative, thus taking '%' requires
    // extra handling and can be slow. Because capacity is a power of two, it
    // is much faster to use '&' operator.
    return sequence_number & capacity_minus_1_;
  }

  void SetNotReceived(int64_t begin_sequence_number_inclusive,
                      int64_t end_sequence_number_exclusive);

  // Adjust capacity to match new_size, may reduce capacity.
  // On return guarantees capacity >= new_size.
  void AdjustToSize(int new_size);
  void Reallocate(int new_capacity);

  int capacity() const { return capacity_minus_1_ + 1; }
  bool has_seen_packet() const { return arrival_times_ != nullptr; }

  // Circular buffer. Packet with sequence number `sequence_number`
  // is stored in the slot `sequence_number % capacity_`
  std::unique_ptr<Timestamp[]> arrival_times_ = nullptr;

  // Allocated size of the `arrival_times_`
  // capacity_ is a power of 2 in range [kMinCapacity, kMaxNumberOfPackets]
  // `capacity - 1` is used much more often than `capacity`, thus that value is
  // stored.
  int capacity_minus_1_ = -1;

  // The unwrapped sequence number for valid range of sequence numbers.
  // arrival_times_ entries only valid for sequence numbers in range
  // `begin_sequence_number_ <= sequence_number < end_sequence_number_`
  int64_t begin_sequence_number_ = 0;
  int64_t end_sequence_number_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_PACKET_ARRIVAL_MAP_H_
