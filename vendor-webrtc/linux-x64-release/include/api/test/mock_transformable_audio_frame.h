/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_TRANSFORMABLE_AUDIO_FRAME_H_
#define API_TEST_MOCK_TRANSFORMABLE_AUDIO_FRAME_H_

#include <cstdint>
#include <optional>
#include <string>

#include "api/array_view.h"
#include "api/frame_transformer_interface.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "test/gmock.h"

namespace webrtc {

class MockTransformableAudioFrame : public TransformableAudioFrameInterface {
 public:
  MockTransformableAudioFrame() : TransformableAudioFrameInterface(Passkey()) {}

  MOCK_METHOD(ArrayView<const uint8_t>, GetData, (), (const, override));
  MOCK_METHOD(void, SetData, (webrtc::ArrayView<const uint8_t>), (override));
  MOCK_METHOD(void, SetRTPTimestamp, (uint32_t), (override));
  MOCK_METHOD(uint8_t, GetPayloadType, (), (const, override));
  MOCK_METHOD(uint32_t, GetSsrc, (), (const, override));
  MOCK_METHOD(uint32_t, GetTimestamp, (), (const, override));
  MOCK_METHOD(std::string, GetMimeType, (), (const, override));
  MOCK_METHOD(ArrayView<const uint32_t>,
              GetContributingSources,
              (),
              (const, override));
  MOCK_METHOD(const std::optional<uint16_t>,
              SequenceNumber,
              (),
              (const, override));
  MOCK_METHOD(TransformableFrameInterface::Direction,
              GetDirection,
              (),
              (const, override));
  MOCK_METHOD(std::optional<uint64_t>,
              AbsoluteCaptureTimestamp,
              (),
              (const, override));
  MOCK_METHOD(TransformableAudioFrameInterface::FrameType,
              Type,
              (),
              (const, override));
  MOCK_METHOD(std::optional<uint8_t>, AudioLevel, (), (const, override));

  MOCK_METHOD(std::optional<Timestamp>, ReceiveTime, (), (const, override));
  MOCK_METHOD(std::optional<Timestamp>, CaptureTime, (), (const, override));
  MOCK_METHOD(std::optional<TimeDelta>,
              SenderCaptureTimeOffset,
              (),
              (const, override));
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_TRANSFORMABLE_AUDIO_FRAME_H_
