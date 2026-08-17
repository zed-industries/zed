/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_DECISION_LOGIC_H_
#define MODULES_AUDIO_CODING_NETEQ_DECISION_LOGIC_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>

#include "api/environment/environment.h"
#include "api/neteq/neteq.h"
#include "api/neteq/neteq_controller.h"
#include "api/neteq/tick_timer.h"
#include "modules/audio_coding/neteq/buffer_level_filter.h"
#include "modules/audio_coding/neteq/delay_constraints.h"
#include "modules/audio_coding/neteq/delay_manager.h"
#include "modules/audio_coding/neteq/packet_arrival_history.h"

namespace webrtc {

// This is the class for the decision tree implementation.
class DecisionLogic : public NetEqController {
 public:
  DecisionLogic(const Environment& env, NetEqController::Config config);
  DecisionLogic(
      NetEqController::Config config,
      std::unique_ptr<DelayManager> delay_manager,
      std::unique_ptr<BufferLevelFilter> buffer_level_filter,
      std::unique_ptr<PacketArrivalHistory> packet_arrival_history = nullptr);

  ~DecisionLogic() override;

  DecisionLogic(const DecisionLogic&) = delete;
  DecisionLogic& operator=(const DecisionLogic&) = delete;

  // Not used.
  void Reset() override {}

  // Resets parts of the state. Typically done when switching codecs.
  void SoftReset() override;

  // Sets the sample rate and the output block size.
  void SetSampleRate(int fs_hz, size_t output_size_samples) override;

  // Given info about the latest received packet, and current jitter buffer
  // status, returns the operation. `target_timestamp` and `expand_mutefactor`
  // are provided for reference. `last_packet_samples` is the number of samples
  // obtained from the last decoded frame. If there is a packet available, it
  // should be supplied in `packet`; otherwise it should be NULL. The mode
  // resulting from the last call to NetEqImpl::GetAudio is supplied in
  // `last_mode`. If there is a DTMF event to play, `play_dtmf` should be set to
  // true. The output variable `reset_decoder` will be set to true if a reset is
  // required; otherwise it is left unchanged (i.e., it can remain true if it
  // was true before the call).
  NetEq::Operation GetDecision(const NetEqController::NetEqStatus& status,
                               bool* reset_decoder) override;

  void ExpandDecision(NetEq::Operation /* operation */) override {}

  // Adds `value` to `sample_memory_`.
  void AddSampleMemory(int32_t value) override { sample_memory_ += value; }

  int TargetLevelMs() const override;

  int UnlimitedTargetLevelMs() const override;

  std::optional<int> PacketArrived(int fs_hz,
                                   bool should_update_stats,
                                   const PacketArrivedInfo& info) override;

  void RegisterEmptyPacket() override {}

  bool SetMaximumDelay(int delay_ms) override {
    return delay_constraints_.SetMaximumDelay(delay_ms);
  }
  bool SetMinimumDelay(int delay_ms) override {
    return delay_constraints_.SetMinimumDelay(delay_ms);
  }
  bool SetBaseMinimumDelay(int delay_ms) override {
    return delay_constraints_.SetBaseMinimumDelay(delay_ms);
  }
  int GetBaseMinimumDelay() const override {
    return delay_constraints_.GetBaseMinimumDelay();
  }
  bool PeakFound() const override { return false; }

  int GetFilteredBufferLevel() const override;

  // Accessors and mutators.
  void set_sample_memory(int32_t value) override { sample_memory_ = value; }
  size_t noise_fast_forward() const override { return noise_fast_forward_; }
  size_t packet_length_samples() const override {
    return packet_length_samples_;
  }
  void set_packet_length_samples(size_t value) override {
    packet_length_samples_ = value;
  }
  void set_prev_time_scale(bool value) override { prev_time_scale_ = value; }

 private:
  // The value 5 sets maximum time-stretch rate to about 100 ms/s.
  static const int kMinTimescaleInterval = 5;

  // Updates the `buffer_level_filter_` with the current buffer level
  // `buffer_size_samples`.
  void FilterBufferLevel(size_t buffer_size_samples);

  // Returns the operation given that the next available packet is a comfort
  // noise payload (RFC 3389 only, not codec-internal).
  virtual NetEq::Operation CngOperation(NetEqController::NetEqStatus status);

  // Returns the operation given that no packets are available (except maybe
  // a DTMF event, flagged by setting `play_dtmf` true).
  virtual NetEq::Operation NoPacket(NetEqController::NetEqStatus status);

  // Returns the operation to do given that the expected packet is available.
  virtual NetEq::Operation ExpectedPacketAvailable(
      NetEqController::NetEqStatus status);

  // Returns the operation to do given that the expected packet is not
  // available, but a packet further into the future is at hand.
  virtual NetEq::Operation FuturePacketAvailable(
      NetEqController::NetEqStatus status);

  // Checks if enough time has elapsed since the last successful timescale
  // operation was done (i.e., accelerate or preemptive expand).
  bool TimescaleAllowed() const {
    return !timescale_countdown_ || timescale_countdown_->Finished();
  }

  // Checks if the current (filtered) buffer level is under the target level.
  bool UnderTargetLevel() const;

  // Checks if an ongoing concealment should be continued due to low buffer
  // level, even though the next packet is available.
  bool PostponeDecode(NetEqController::NetEqStatus status) const;

  // Checks if we still have not done enough expands to cover the distance from
  // the last decoded packet to the next available packet.
  bool PacketTooEarly(NetEqController::NetEqStatus status) const;

  int GetPlayoutDelayMs(NetEqController::NetEqStatus status) const;

  std::unique_ptr<DelayManager> delay_manager_;
  DelayConstraints delay_constraints_;
  std::unique_ptr<BufferLevelFilter> buffer_level_filter_;
  std::unique_ptr<PacketArrivalHistory> packet_arrival_history_;
  const TickTimer* tick_timer_;
  int sample_rate_khz_;
  size_t output_size_samples_;
  size_t noise_fast_forward_ = 0;
  size_t packet_length_samples_ = 0;
  int sample_memory_ = 0;
  bool prev_time_scale_ = false;
  bool disallow_time_stretching_;
  std::unique_ptr<TickTimer::Countdown> timescale_countdown_;
  int time_stretched_cn_samples_ = 0;
  bool buffer_flush_ = false;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_DECISION_LOGIC_H_
