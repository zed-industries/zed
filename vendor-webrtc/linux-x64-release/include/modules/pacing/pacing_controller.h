/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PACING_PACING_CONTROLLER_H_
#define MODULES_PACING_PACING_CONTROLLER_H_

#include <stddef.h>
#include <stdint.h>

#include <array>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/field_trials_view.h"
#include "api/rtp_packet_sender.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/pacing/bitrate_prober.h"
#include "modules/pacing/prioritized_packet_queue.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtp_packet_to_send.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

// This class implements a leaky-bucket packet pacing algorithm. It handles the
// logic of determining which packets to send when, but the actual timing of
// the processing is done externally (e.g. RtpPacketPacer). Furthermore, the
// forwarding of packets when they are ready to be sent is also handled
// externally, via the PacingController::PacketSender interface.
class PacingController {
 public:
  class PacketSender {
   public:
    virtual ~PacketSender() = default;
    virtual void SendPacket(std::unique_ptr<RtpPacketToSend> packet,
                            const PacedPacketInfo& cluster_info) = 0;
    // Should be called after each call to SendPacket().
    virtual std::vector<std::unique_ptr<RtpPacketToSend>> FetchFec() = 0;
    virtual std::vector<std::unique_ptr<RtpPacketToSend>> GeneratePadding(
        DataSize size) = 0;
    // TODO(bugs.webrtc.org/1439830): Make pure virtual once subclasses adapt.
    virtual void OnBatchComplete() {}

    // TODO(bugs.webrtc.org/11340): Make pure virtual once downstream projects
    // have been updated.
    virtual void OnAbortedRetransmissions(
        uint32_t /* ssrc */,
        ArrayView<const uint16_t> /* sequence_numbers */) {}
    virtual std::optional<uint32_t> GetRtxSsrcForMedia(
        uint32_t /* ssrc */) const {
      return std::nullopt;
    }
  };

  // If no media or paused, wake up at least every `kPausedProcessIntervalMs` in
  // order to send a keep-alive packet so we don't get stuck in a bad state due
  // to lack of feedback.
  static const TimeDelta kPausedProcessInterval;
  // The default minimum time that should elapse calls to `ProcessPackets()`.
  static const TimeDelta kMinSleepTime;
  // When padding should be generated, add packets to the buffer with a size
  // corresponding to this duration times the current padding rate.
  static const TimeDelta kTargetPaddingDuration;
  // The maximum time that the pacer can use when "replaying" passed time where
  // padding should have been generated.
  static const TimeDelta kMaxPaddingReplayDuration;
  // Allow probes to be processed slightly ahead of inteded send time. Currently
  // set to 1ms as this is intended to allow times be rounded down to the
  // nearest millisecond.
  static const TimeDelta kMaxEarlyProbeProcessing;
  // Max total size of packets expected to be sent in a burst in order to not
  // risk loosing packets due to too small send socket buffers. It upper limits
  // the send burst interval.
  // Ex: max send burst interval = 63Kb / 10Mbit/s = 50ms.
  static constexpr DataSize kMaxBurstSize = DataSize::Bytes(63 * 1000);

  // Configuration default values.
  static constexpr TimeDelta kDefaultBurstInterval = TimeDelta::Millis(40);
  static constexpr TimeDelta kMaxExpectedQueueLength = TimeDelta::Millis(2000);

  struct Configuration {
    // If the pacer queue grows longer than the configured max queue limit,
    // pacer sends at the minimum rate needed to keep the max queue limit and
    // ignore the current bandwidth estimate.
    bool drain_large_queues = true;
    // Expected max pacer delay. If ExpectedQueueTime() is higher than
    // this value, the packet producers should wait (eg drop frames rather than
    // encoding them). Bitrate sent may temporarily exceed target set by
    // SetPacingRates() so that this limit will be upheld if
    // `drain_large_queues` is set.
    TimeDelta queue_time_limit = kMaxExpectedQueueLength;
    // If the first packet of a keyframe is enqueued on a RTP stream, pacer
    // skips forward to that packet and drops other enqueued packets on that
    // stream, unless a keyframe is already being paced.
    bool keyframe_flushing = false;
    // Audio retransmission is prioritized before video retransmission packets.
    bool prioritize_audio_retransmission = false;
    // Configure separate timeouts per priority. After a timeout, a packet of
    // that sort will not be paced and instead dropped.
    // Note: to set TTL on audio retransmission,
    // `prioritize_audio_retransmission` must be true.
    PacketQueueTTL packet_queue_ttl;
    // The pacer is allowed to send enqueued packets in bursts and can build up
    // a packet "debt" that correspond to approximately the send rate during the
    // burst interval.
    TimeDelta send_burst_interval = kDefaultBurstInterval;
  };

  static Configuration DefaultConfiguration() { return Configuration{}; }

  PacingController(Clock* clock,
                   PacketSender* packet_sender,
                   const FieldTrialsView& field_trials,
                   Configuration configuration = DefaultConfiguration());

  ~PacingController();

  // Adds the packet to the queue and calls PacketRouter::SendPacket() when
  // it's time to send.
  void EnqueuePacket(std::unique_ptr<RtpPacketToSend> packet);

  void CreateProbeClusters(
      ArrayView<const ProbeClusterConfig> probe_cluster_configs);

  void Pause();   // Temporarily pause all sending.
  void Resume();  // Resume sending packets.
  bool IsPaused() const;

  void SetCongested(bool congested);

  // Sets the pacing rates. Must be called once before packets can be sent.
  void SetPacingRates(DataRate pacing_rate, DataRate padding_rate);
  DataRate pacing_rate() const { return adjusted_media_rate_; }

  // Currently audio traffic is not accounted by pacer and passed through.
  // With the introduction of audio BWE audio traffic will be accounted for
  // the pacer budget calculation. The audio traffic still will be injected
  // at high priority.
  void SetAccountForAudioPackets(bool account_for_audio);
  void SetIncludeOverhead();

  void SetTransportOverhead(DataSize overhead_per_packet);
  // The pacer is allowed to send enqued packets in bursts and can build up a
  // packet "debt" that correspond to approximately the send rate during
  // 'burst_interval'.
  void SetSendBurstInterval(TimeDelta burst_interval);

  // A probe may be sent without first waing for a media packet.
  void SetAllowProbeWithoutMediaPacket(bool allow);

  // Returns the time when the oldest packet was queued.
  Timestamp OldestPacketEnqueueTime() const;

  // Number of packets in the pacer queue.
  size_t QueueSizePackets() const;
  // Number of packets in the pacer queue per media type (RtpPacketMediaType
  // values are used as lookup index).
  const std::array<int, kNumMediaTypes>& SizeInPacketsPerRtpPacketMediaType()
      const;
  // Totals size of packets in the pacer queue.
  DataSize QueueSizeData() const;

  // Current buffer level, i.e. max of media and padding debt.
  DataSize CurrentBufferLevel() const;

  // Returns the time when the first packet was sent.
  std::optional<Timestamp> FirstSentPacketTime() const;

  // Returns the number of milliseconds it will take to send the current
  // packets in the queue, given the current size and bitrate, ignoring prio.
  TimeDelta ExpectedQueueTime() const;

  void SetQueueTimeLimit(TimeDelta limit);

  // Enable bitrate probing. Enabled by default, mostly here to simplify
  // testing. Must be called before any packets are being sent to have an
  // effect.
  void SetProbingEnabled(bool enabled);

  // Returns the next time we expect ProcessPackets() to be called.
  Timestamp NextSendTime() const;

  // Check queue of pending packets and send them or padding packets, if budget
  // is available.
  void ProcessPackets();

  bool IsProbing() const;

  // Note: Intended for debugging purposes only, will be removed.
  // Sets the number of iterations of the main loop in `ProcessPackets()` that
  // is considered erroneous to exceed.
  void SetCircuitBreakerThreshold(int num_iterations);

  // Remove any pending packets matching this SSRC from the packet queue.
  void RemovePacketsForSsrc(uint32_t ssrc);

 private:
  TimeDelta UpdateTimeAndGetElapsed(Timestamp now);
  bool ShouldSendKeepalive(Timestamp now) const;

  // Updates the number of bytes that can be sent for the next time interval.
  void UpdateBudgetWithElapsedTime(TimeDelta delta);
  void UpdateBudgetWithSentData(DataSize size);
  void UpdatePaddingBudgetWithSentData(DataSize size);

  DataSize PaddingToAdd(DataSize recommended_probe_size,
                        DataSize data_sent) const;

  std::unique_ptr<RtpPacketToSend> GetPendingPacket(
      const PacedPacketInfo& pacing_info,
      Timestamp target_send_time,
      Timestamp now);
  void OnPacketSent(RtpPacketMediaType packet_type,
                    DataSize packet_size,
                    Timestamp send_time);
  void MaybeUpdateMediaRateDueToLongQueue(Timestamp now);

  Timestamp CurrentTime() const;

  // Helper methods for packet that may not be paced. Returns a finite Timestamp
  // if a packet type is configured to not be paced and the packet queue has at
  // least one packet of that type. Otherwise returns
  // Timestamp::MinusInfinity().
  Timestamp NextUnpacedSendTime() const;

  Clock* const clock_;
  PacketSender* const packet_sender_;
  const FieldTrialsView& field_trials_;

  const bool drain_large_queues_;
  const bool send_padding_if_silent_;
  const bool pace_audio_;
  const bool ignore_transport_overhead_;
  const bool fast_retransmissions_;
  const bool keyframe_flushing_;
  DataRate max_rate = DataRate::BitsPerSec(100'000'000);
  DataSize transport_overhead_per_packet_;
  TimeDelta send_burst_interval_;

  // TODO(webrtc:9716): Remove this when we are certain clocks are monotonic.
  // The last millisecond timestamp returned by `clock_`.
  mutable Timestamp last_timestamp_;
  bool paused_;

  // Amount of outstanding data for media and padding.
  DataSize media_debt_;
  DataSize padding_debt_;

  // The target pacing rate, signaled via SetPacingRates().
  DataRate pacing_rate_;
  // The media send rate, which might adjusted from pacing_rate_, e.g. if the
  // pacing queue is growing too long.
  DataRate adjusted_media_rate_;
  // The padding target rate. We aim to fill up to this rate with padding what
  // is not already used by media.
  DataRate padding_rate_;

  BitrateProber prober_;
  bool probing_send_failure_;

  Timestamp last_process_time_;
  Timestamp last_send_time_;
  std::optional<Timestamp> first_sent_packet_time_;
  bool seen_first_packet_;

  PrioritizedPacketQueue packet_queue_;

  bool congested_;

  TimeDelta queue_time_limit_;
  bool account_for_audio_;
  bool include_overhead_;

  int circuit_breaker_threshold_;
};
}  // namespace webrtc

#endif  // MODULES_PACING_PACING_CONTROLLER_H_
