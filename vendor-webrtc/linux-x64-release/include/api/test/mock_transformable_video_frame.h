/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_TRANSFORMABLE_VIDEO_FRAME_H_
#define API_TEST_MOCK_TRANSFORMABLE_VIDEO_FRAME_H_

#include <cstdint>
#include <optional>
#include <string>
#include <type_traits>

#include "api/array_view.h"
#include "api/frame_transformer_interface.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/video_frame_metadata.h"
#include "test/gmock.h"

namespace webrtc {

class MockTransformableVideoFrame
    : public webrtc::TransformableVideoFrameInterface {
 public:
  MockTransformableVideoFrame() : TransformableVideoFrameInterface(Passkey()) {}
  MOCK_METHOD(ArrayView<const uint8_t>, GetData, (), (const, override));
  MOCK_METHOD(void,
              SetData,
              (webrtc::ArrayView<const uint8_t> data),
              (override));
  MOCK_METHOD(uint32_t, GetTimestamp, (), (const, override));
  MOCK_METHOD(void, SetRTPTimestamp, (uint32_t), (override));
  MOCK_METHOD(uint32_t, GetSsrc, (), (const, override));
  MOCK_METHOD(bool, IsKeyFrame, (), (const, override));
  MOCK_METHOD(void,
              SetMetadata,
              (const webrtc::VideoFrameMetadata&),
              (override));
  MOCK_METHOD(uint8_t, GetPayloadType, (), (const, override));
  MOCK_METHOD(TransformableFrameInterface::Direction,
              GetDirection,
              (),
              (const, override));
  MOCK_METHOD(std::string, GetMimeType, (), (const, override));
  MOCK_METHOD(VideoFrameMetadata, Metadata, (), (const, override));
  MOCK_METHOD(RTPVideoHeader&, header, (), (const, override));
  MOCK_METHOD(std::optional<Timestamp>,
              GetPresentationTimestamp,
              (),
              (const, override));
  MOCK_METHOD(std::optional<Timestamp>, ReceiveTime, (), (const, override));
  MOCK_METHOD(std::optional<Timestamp>, CaptureTime, (), (const, override));
  MOCK_METHOD(std::optional<TimeDelta>,
              SenderCaptureTimeOffset,
              (),
              (const, override));
};

static_assert(!std::is_abstract_v<MockTransformableVideoFrame>, "");

}  // namespace webrtc

#endif  // API_TEST_MOCK_TRANSFORMABLE_VIDEO_FRAME_H_
