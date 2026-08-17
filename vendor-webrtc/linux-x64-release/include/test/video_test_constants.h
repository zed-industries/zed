/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_VIDEO_TEST_CONSTANTS_H_
#define TEST_VIDEO_TEST_CONSTANTS_H_

#include <cstdint>

#include "api/units/time_delta.h"

namespace webrtc {
namespace test {

class VideoTestConstants {
 public:
  static constexpr size_t kNumSsrcs = 6;
  static constexpr int kNumSimulcastStreams = 3;
  static constexpr int kDefaultWidth = 320;
  static constexpr int kDefaultHeight = 180;
  static constexpr int kDefaultFramerate = 30;
  static constexpr TimeDelta kDefaultTimeout = TimeDelta::Seconds(30);
  static constexpr TimeDelta kLongTimeout = TimeDelta::Seconds(120);
  enum classPayloadTypes : uint8_t {
    kSendRtxPayloadType = 98,
    kRtxRedPayloadType = 99,
    kVideoSendPayloadType = 100,
    kAudioSendPayloadType = 103,
    kPayloadTypeH265 = 117,
    kRedPayloadType = 118,
    kUlpfecPayloadType = 119,
    kFlexfecPayloadType = 120,
    kPayloadTypeH264 = 122,
    kPayloadTypeVP8 = 123,
    kPayloadTypeVP9 = 124,
    kPayloadTypeGeneric = 125,
    kFakeVideoSendPayloadType = 126,
  };
  static constexpr uint32_t kSendRtxSsrcs[kNumSsrcs] = {
      0xBADCAFD, 0xBADCAFE, 0xBADCAFF, 0xBADCB00, 0xBADCB01, 0xBADCB02};
  static constexpr uint32_t kVideoSendSsrcs[kNumSsrcs] = {
      0xC0FFED, 0xC0FFEE, 0xC0FFEF, 0xC0FFF0, 0xC0FFF1, 0xC0FFF2};
  static constexpr uint32_t kAudioSendSsrc = 0xDEADBEEF;
  static constexpr uint32_t kFlexfecSendSsrc = 0xBADBEEF;
  static constexpr uint32_t kReceiverLocalVideoSsrc = 0x123456;
  static constexpr uint32_t kReceiverLocalAudioSsrc = 0x1234567;
  static constexpr int kNackRtpHistoryMs = 1000;

 private:
  VideoTestConstants() = default;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_VIDEO_TEST_CONSTANTS_H_
