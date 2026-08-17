/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_AV1_TEST_HELPER_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_AV1_TEST_HELPER_H_

#include <stdint.h>

#include <initializer_list>
#include <vector>

namespace webrtc {
// All obu types offset by 3 to take correct position in the obu_header.
constexpr uint8_t kAv1ObuTypeSequenceHeader = 1 << 3;
constexpr uint8_t kAv1ObuTypeTemporalDelimiter = 2 << 3;
constexpr uint8_t kAv1ObuTypeFrameHeader = 3 << 3;
constexpr uint8_t kAv1ObuTypeTileGroup = 4 << 3;
constexpr uint8_t kAv1ObuTypeMetadata = 5 << 3;
constexpr uint8_t kAv1ObuTypeFrame = 6 << 3;
constexpr uint8_t kAv1ObuTypeTileList = 8 << 3;
constexpr uint8_t kAv1ObuExtensionPresentBit = 0b0'0000'100;
constexpr uint8_t kAv1ObuSizePresentBit = 0b0'0000'010;
constexpr uint8_t kAv1ObuExtensionS1T1 = 0b001'01'000;

class Av1Obu {
 public:
  explicit Av1Obu(uint8_t obu_type);

  Av1Obu& WithExtension(uint8_t extension);
  Av1Obu& WithoutSize();
  Av1Obu& WithPayload(std::vector<uint8_t> payload);

 private:
  friend std::vector<uint8_t> BuildAv1Frame(std::initializer_list<Av1Obu> obus);
  uint8_t header_;
  uint8_t extension_ = 0;
  std::vector<uint8_t> payload_;
};

std::vector<uint8_t> BuildAv1Frame(std::initializer_list<Av1Obu> obus);

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTP_PACKETIZER_AV1_TEST_HELPER_H_
