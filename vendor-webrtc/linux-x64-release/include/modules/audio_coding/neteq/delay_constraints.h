/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_DELAY_CONSTRAINTS_H_
#define MODULES_AUDIO_CODING_NETEQ_DELAY_CONSTRAINTS_H_

namespace webrtc {

class DelayConstraints {
 public:
  DelayConstraints(int max_packets_in_buffer, int base_minimum_delay_ms);

  // Returns the delay (in ms) clamped to the range of valid delays.
  int Clamp(int delay_ms) const;

  // Notifies the DelayManager of how much audio data is carried in each packet.
  bool SetPacketAudioLength(int length_ms);

  // Accessors and mutators.
  // Assuming `delay` is in valid range.
  bool SetMinimumDelay(int delay_ms);
  bool SetMaximumDelay(int delay_ms);
  bool SetBaseMinimumDelay(int delay_ms);
  int GetBaseMinimumDelay() const;

  // These accessors are only intended for testing purposes.
  int effective_minimum_delay_ms_for_test() const {
    return effective_minimum_delay_ms_;
  }

 private:
  // Provides value which minimum delay can't exceed based on current buffer
  // size and given `maximum_delay_ms_`. Lower bound is a constant 0.
  int MinimumDelayUpperBound() const;

  // Updates `effective_minimum_delay_ms_` delay based on current
  // `minimum_delay_ms_`, `base_minimum_delay_ms_`, `maximum_delay_ms_` and
  // buffer size.
  void UpdateEffectiveMinimumDelay();

  // Makes sure that `delay_ms` is less than maximum delay, if any maximum
  // is set. Also, if possible check `delay_ms` to be less than 75% of
  // `max_packets_in_buffer_`.
  bool IsValidMinimumDelay(int delay_ms) const;

  // Checks that `delay_ms` is in the range of valid base minimum delays.
  bool IsValidBaseMinimumDelay(int delay_ms) const;

  // TODO(jakobi): set maximum buffer delay instead of number of packets.
  const int max_packets_in_buffer_;

  int base_minimum_delay_ms_;
  int effective_minimum_delay_ms_;  // Used as lower bound for target delay.
  int minimum_delay_ms_;            // Externally set minimum delay.
  int maximum_delay_ms_;            // Externally set maximum delay. No maximum
                                    // delay is enforced if <= 0.

  int packet_len_ms_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_DELAY_CONSTRAINTS_H_
