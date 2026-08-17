/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_NETEQ_CONTROLLER_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_NETEQ_CONTROLLER_H_

#include "api/neteq/neteq_controller.h"
#include "test/gmock.h"

namespace webrtc {

class MockNetEqController : public NetEqController {
 public:
  MockNetEqController() = default;
  ~MockNetEqController() override { Die(); }
  MOCK_METHOD(void, Die, ());
  MOCK_METHOD(void, Reset, (), (override));
  MOCK_METHOD(void, SoftReset, (), (override));
  MOCK_METHOD(NetEq::Operation,
              GetDecision,
              (const NetEqStatus& neteq_status, bool* reset_decoder),
              (override));
  MOCK_METHOD(void, RegisterEmptyPacket, (), (override));
  MOCK_METHOD(void,
              SetSampleRate,
              (int fs_hz, size_t output_size_samples),
              (override));
  MOCK_METHOD(bool, SetMaximumDelay, (int delay_ms), (override));
  MOCK_METHOD(bool, SetMinimumDelay, (int delay_ms), (override));
  MOCK_METHOD(bool, SetBaseMinimumDelay, (int delay_ms), (override));
  MOCK_METHOD(int, GetBaseMinimumDelay, (), (const, override));
  MOCK_METHOD(void, ExpandDecision, (NetEq::Operation operation), (override));
  MOCK_METHOD(void, AddSampleMemory, (int32_t value), (override));
  MOCK_METHOD(int, TargetLevelMs, (), (const, override));
  MOCK_METHOD(std::optional<int>,
              PacketArrived,
              (int fs_hz,
               bool should_update_stats,
               const PacketArrivedInfo& info),
              (override));
  MOCK_METHOD(void, NotifyMutedState, (), (override));
  MOCK_METHOD(bool, PeakFound, (), (const, override));
  MOCK_METHOD(int, GetFilteredBufferLevel, (), (const, override));
  MOCK_METHOD(void, set_sample_memory, (int32_t value), (override));
  MOCK_METHOD(size_t, noise_fast_forward, (), (const, override));
  MOCK_METHOD(size_t, packet_length_samples, (), (const, override));
  MOCK_METHOD(void, set_packet_length_samples, (size_t value), (override));
  MOCK_METHOD(void, set_prev_time_scale, (bool value), (override));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_NETEQ_CONTROLLER_H_
