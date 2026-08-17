/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_REPLACEMENT_INPUT_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_REPLACEMENT_INPUT_H_

#include <memory>
#include <set>

#include "modules/audio_coding/neteq/tools/neteq_input.h"

namespace webrtc {
namespace test {

// This class converts the packets from a NetEqInput to fake encodings to be
// decoded by a FakeDecodeFromFile decoder.
class NetEqReplacementInput : public NetEqInput {
 public:
  NetEqReplacementInput(std::unique_ptr<NetEqInput> source,
                        uint8_t replacement_payload_type,
                        const std::set<uint8_t>& comfort_noise_types,
                        const std::set<uint8_t>& forbidden_types);

  std::optional<int64_t> NextPacketTime() const override;
  std::optional<int64_t> NextOutputEventTime() const override;
  std::optional<SetMinimumDelayInfo> NextSetMinimumDelayInfo() const override;
  std::unique_ptr<PacketData> PopPacket() override;
  void AdvanceOutputEvent() override;
  void AdvanceSetMinimumDelay() override;
  bool ended() const override;
  std::optional<RTPHeader> NextHeader() const override;

 private:
  void ReplacePacket();

  std::unique_ptr<NetEqInput> source_;
  const uint8_t replacement_payload_type_;
  const std::set<uint8_t> comfort_noise_types_;
  const std::set<uint8_t> forbidden_types_;
  std::unique_ptr<PacketData> packet_;         // The next packet to deliver.
  uint32_t last_frame_size_timestamps_ = 960;  // Initial guess: 20 ms @ 48 kHz.
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_REPLACEMENT_INPUT_H_
