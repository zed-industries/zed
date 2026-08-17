/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_NETEQ_NETEQ_CONTROLLER_H_
#define API_NETEQ_NETEQ_CONTROLLER_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "api/neteq/neteq.h"
#include "api/neteq/tick_timer.h"

namespace webrtc {

// Decides the actions that NetEq should take. This affects the behavior of the
// jitter buffer, and how it reacts to network conditions.
// This class will undergo substantial refactoring in the near future, and the
// API is expected to undergo significant changes. A target API is given below:
//
// class NetEqController {
//  public:
//   // Resets object to a clean state.
//   void Reset();
//   // Given NetEq status, make a decision.
//   Operation GetDecision(NetEqStatus neteq_status);
//   // Register every packet received.
//   void RegisterPacket(PacketInfo packet_info);
//   // Register empty packet.
//   void RegisterEmptyPacket();
//   // Register a codec switching.
//   void CodecSwithed();
//   // Sets the sample rate.
//   void SetSampleRate(int fs_hz);
//   // Sets the packet length in samples.
//   void SetPacketLengthSamples();
//   // Sets maximum delay.
//   void SetMaximumDelay(int delay_ms);
//   // Sets mininum delay.
//   void SetMinimumDelay(int delay_ms);
//   // Sets base mininum delay.
//   void SetBaseMinimumDelay(int delay_ms);
//   // Gets target buffer level.
//   int GetTargetBufferLevelMs() const;
//   // Gets filtered buffer level.
//   int GetFilteredBufferLevel() const;
//   // Gets base minimum delay.
//   int GetBaseMinimumDelay() const;
// }

class NetEqController {
 public:
  // This struct is used to create a NetEqController.
  struct Config {
    bool allow_time_stretching;
    bool enable_rtx_handling;
    int max_packets_in_buffer;
    int base_min_delay_ms;
    TickTimer* tick_timer;
  };

  struct PacketInfo {
    uint32_t timestamp;
    bool is_dtx;
    bool is_cng;
  };

  struct PacketBufferInfo {
    bool dtx_or_cng;
    size_t num_samples;
    size_t span_samples;
    size_t span_samples_wait_time;
    size_t num_packets;
  };

  struct NetEqStatus {
    uint32_t target_timestamp;
    int16_t expand_mutefactor;
    size_t last_packet_samples;
    std::optional<PacketInfo> next_packet;
    NetEq::Mode last_mode;
    bool play_dtmf;
    size_t generated_noise_samples;
    PacketBufferInfo packet_buffer_info;
    size_t sync_buffer_samples;
  };

  struct PacketArrivedInfo {
    size_t packet_length_samples;
    uint32_t main_timestamp;
    uint16_t main_sequence_number;
    bool is_cng_or_dtmf;
    bool is_dtx;
    bool buffer_flush;
  };

  virtual ~NetEqController() = default;

  // Resets object to a clean state.
  virtual void Reset() = 0;

  // Resets parts of the state. Typically done when switching codecs.
  virtual void SoftReset() = 0;

  // Given info about the latest received packet, and current jitter buffer
  // status, returns the operation. `target_timestamp` and `expand_mutefactor`
  // are provided for reference. `last_packet_samples` is the number of samples
  // obtained from the last decoded frame. If there is a packet available, it
  // should be supplied in `packet`. The mode resulting from the last call to
  // NetEqImpl::GetAudio is supplied in `last_mode`. If there is a DTMF event to
  // play, `play_dtmf` should be set to true. The output variable
  // `reset_decoder` will be set to true if a reset is required; otherwise it is
  // left unchanged (i.e., it can remain true if it was true before the call).
  virtual NetEq::Operation GetDecision(const NetEqStatus& status,
                                       bool* reset_decoder) = 0;

  // Inform NetEqController that an empty packet has arrived.
  virtual void RegisterEmptyPacket() = 0;

  // Sets the sample rate and the output block size.
  virtual void SetSampleRate(int fs_hz, size_t output_size_samples) = 0;

  // Sets a minimum or maximum delay in millisecond.
  // Returns true if the delay bound is successfully applied, otherwise false.
  virtual bool SetMaximumDelay(int delay_ms) = 0;
  virtual bool SetMinimumDelay(int delay_ms) = 0;

  // Sets a base minimum delay in milliseconds for packet buffer. The effective
  // minimum delay can't be lower than base minimum delay, even if a lower value
  // is set using SetMinimumDelay.
  // Returns true if the base minimum is successfully applied, otherwise false.
  virtual bool SetBaseMinimumDelay(int delay_ms) = 0;
  virtual int GetBaseMinimumDelay() const = 0;

  // Reports back to DecisionLogic whether the decision to do expand remains or
  // not. Note that this is necessary, since an expand decision can be changed
  // to kNormal in NetEqImpl::GetDecision if there is still enough data in the
  // sync buffer.
  virtual void ExpandDecision(NetEq::Operation operation) = 0;

  // Adds `value` to `sample_memory_`.
  virtual void AddSampleMemory(int32_t value) = 0;

  // Returns the target buffer level in ms.
  virtual int TargetLevelMs() const = 0;

  // Returns the target buffer level in ms as it would be if no minimum or
  // maximum delay was set.
  // TODO(bugs.webrtc.org/14270): Make pure virtual once all implementations are
  // updated.
  virtual int UnlimitedTargetLevelMs() const { return 0; }

  // Notify the NetEqController that a packet has arrived. Returns the relative
  // arrival delay, if it can be computed.
  virtual std::optional<int> PacketArrived(int fs_hz,
                                           bool should_update_stats,
                                           const PacketArrivedInfo& info) = 0;

  // Notify the NetEqController that we are currently in muted state.
  // TODO(bugs.webrtc.org/14270): Make pure virtual when downstream is updated.
  virtual void NotifyMutedState() {}

  // Returns true if a peak was found.
  virtual bool PeakFound() const = 0;

  // Get the filtered buffer level in samples.
  virtual int GetFilteredBufferLevel() const = 0;

  // Accessors and mutators.
  virtual void set_sample_memory(int32_t value) = 0;
  virtual size_t noise_fast_forward() const = 0;
  virtual size_t packet_length_samples() const = 0;
  virtual void set_packet_length_samples(size_t value) = 0;
  virtual void set_prev_time_scale(bool value) = 0;
};

}  // namespace webrtc
#endif  // API_NETEQ_NETEQ_CONTROLLER_H_
