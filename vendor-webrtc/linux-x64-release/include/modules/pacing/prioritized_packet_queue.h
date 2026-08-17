/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PACING_PRIORITIZED_PACKET_QUEUE_H_
#define MODULES_PACING_PRIORITIZED_PACKET_QUEUE_H_

#include <stddef.h>

#include <array>
#include <cstdint>
#include <deque>
#include <list>
#include <memory>
#include <unordered_map>

#include "absl/container/inlined_vector.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_packet_to_send.h"

namespace webrtc {

// Describes how long time a packet may stay in the queue before being dropped.
struct PacketQueueTTL {
  TimeDelta audio_retransmission = TimeDelta::PlusInfinity();
  TimeDelta video_retransmission = TimeDelta::PlusInfinity();
  TimeDelta video = TimeDelta::PlusInfinity();
};

class PrioritizedPacketQueue {
 public:
  explicit PrioritizedPacketQueue(
      Timestamp creation_time,
      bool prioritize_audio_retransmission = false,
      PacketQueueTTL packet_queue_ttl = PacketQueueTTL());
  PrioritizedPacketQueue(const PrioritizedPacketQueue&) = delete;
  PrioritizedPacketQueue& operator=(const PrioritizedPacketQueue&) = delete;

  // Add a packet to the queue. The enqueue time is used for queue time stats
  // and to report the leading packet enqueue time per packet type.
  void Push(Timestamp enqueue_time, std::unique_ptr<RtpPacketToSend> packet);

  // Remove the next packet from the queue. Packets a prioritized first
  // according to packet type, in the following order:
  // - audio, retransmissions, video / fec, padding
  // For each packet type, we use one FIFO-queue per SSRC and emit from
  // those queues in a round-robin fashion.
  std::unique_ptr<RtpPacketToSend> Pop();

  // Number of packets in the queue.
  int SizeInPackets() const;

  // Sum of all payload bytes in the queue, where the payload is calculated
  // as `packet->payload_size() + packet->padding_size()`.
  DataSize SizeInPayloadBytes() const;

  // Convenience method for `SizeInPackets() == 0`.
  bool Empty() const;

  // Total packets in the queue per media type (RtpPacketMediaType values are
  // used as lookup index).
  const std::array<int, kNumMediaTypes>& SizeInPacketsPerRtpPacketMediaType()
      const;

  // The enqueue time of the next packet this queue will return via the Pop()
  // method, for the given packet type. If queue has no packets, of that type,
  // returns Timestamp::MinusInfinity().
  Timestamp LeadingPacketEnqueueTime(RtpPacketMediaType type) const;
  Timestamp LeadingPacketEnqueueTimeForRetransmission() const;

  // Enqueue time of the oldest packet in the queue,
  // Timestamp::MinusInfinity() if queue is empty.
  Timestamp OldestEnqueueTime() const;

  // Average queue time for the packets currently in the queue.
  // The queuing time is calculated from Push() to the last UpdateQueueTime()
  // call - with any time spent in a paused state subtracted.
  // Returns TimeDelta::Zero() for an empty queue.
  TimeDelta AverageQueueTime() const;

  // Called during packet processing or when pause stats changes. Since the
  // AverageQueueTime() method does not look at the wall time, this method
  // needs to be called before querying queue time.
  void UpdateAverageQueueTime(Timestamp now);

  // Set the pause state, while `paused` is true queuing time is not counted.
  void SetPauseState(bool paused, Timestamp now);

  // Remove any packets matching the given SSRC.
  void RemovePacketsForSsrc(uint32_t ssrc);

  // Checks if the queue for the given SSRC has original (retransmissions not
  // counted) video packets containing keyframe data.
  bool HasKeyframePackets(uint32_t ssrc) const;

 private:
  static constexpr int kNumPriorityLevels = 5;

  class QueuedPacket {
   public:
    DataSize PacketSize() const;

    std::unique_ptr<RtpPacketToSend> packet;
    Timestamp enqueue_time;
    std::list<Timestamp>::iterator enqueue_time_iterator;
  };

  // Class containing packets for an RTP stream.
  // For each priority level, packets are simply stored in a fifo queue.
  class StreamQueue {
   public:
    explicit StreamQueue(Timestamp creation_time);
    StreamQueue(StreamQueue&&) = default;
    StreamQueue& operator=(StreamQueue&&) = default;

    StreamQueue(const StreamQueue&) = delete;
    StreamQueue& operator=(const StreamQueue&) = delete;

    // Enqueue packet at the given priority level. Returns true if the packet
    // count for that priority level went from zero to non-zero.
    bool EnqueuePacket(QueuedPacket packet, int priority_level);

    QueuedPacket DequeuePacket(int priority_level);

    bool HasPacketsAtPrio(int priority_level) const;
    bool IsEmpty() const;
    Timestamp LeadingPacketEnqueueTime(int priority_level) const;
    Timestamp LastEnqueueTime() const;
    bool has_keyframe_packets() const { return num_keyframe_packets_ > 0; }

    std::array<std::deque<QueuedPacket>, kNumPriorityLevels> DequeueAll();

   private:
    std::deque<QueuedPacket> packets_[kNumPriorityLevels];
    Timestamp last_enqueue_time_;
    int num_keyframe_packets_;
  };

  // Remove the packet from the internal state, e.g. queue time / size etc.
  void DequeuePacketInternal(QueuedPacket& packet);

  // Check if the queue pointed to by `top_active_prio_level_` is empty and
  // if so move it to the lowest non-empty index.
  void MaybeUpdateTopPrioLevel();

  void PurgeOldPacketsAtPriorityLevel(int prio_level, Timestamp now);

  static absl::InlinedVector<TimeDelta, kNumPriorityLevels> ToTtlPerPrio(
      PacketQueueTTL);

  const bool prioritize_audio_retransmission_;
  const absl::InlinedVector<TimeDelta, kNumPriorityLevels>
      time_to_live_per_prio_;

  // Cumulative sum, over all packets, of time spent in the queue.
  TimeDelta queue_time_sum_;
  // Cumulative sum of time the queue has spent in a paused state.
  TimeDelta pause_time_sum_;
  // Total number of packets stored in this queue.
  int size_packets_;
  // Total number of packets stored in this queue per RtpPacketMediaType.
  std::array<int, kNumMediaTypes> size_packets_per_media_type_;
  // Sum of payload sizes for all packts stored in this queue.
  DataSize size_payload_;
  // The last time queue/pause time sums were updated.
  Timestamp last_update_time_;
  bool paused_;

  // Last time `streams_` was culled for inactive streams.
  Timestamp last_culling_time_;

  // Map from SSRC to packet queues for the associated RTP stream.
  std::unordered_map<uint32_t, std::unique_ptr<StreamQueue>> streams_;

  // For each priority level, a queue of StreamQueues which have at least one
  // packet pending for that prio level.
  std::deque<StreamQueue*> streams_by_prio_[kNumPriorityLevels];

  // The first index into `stream_by_prio_` that is non-empty.
  int top_active_prio_level_;

  // Ordered list of enqueue times. Additions are always increasing and added to
  // the end. QueuedPacket instances have a iterators into this list for fast
  // removal.
  std::list<Timestamp> enqueue_times_;
};

}  // namespace webrtc

#endif  // MODULES_PACING_PRIORITIZED_PACKET_QUEUE_H_
