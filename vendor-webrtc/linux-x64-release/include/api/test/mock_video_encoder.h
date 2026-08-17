/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_VIDEO_ENCODER_H_
#define API_TEST_MOCK_VIDEO_ENCODER_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "api/fec_controller_override.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_type.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_encoder.h"
#include "test/gmock.h"

namespace webrtc {

class MockEncodedImageCallback : public EncodedImageCallback {
 public:
  MOCK_METHOD(Result,
              OnEncodedImage,
              (const EncodedImage&, const CodecSpecificInfo*),
              (override));
  MOCK_METHOD(void, OnDroppedFrame, (DropReason reason), (override));
};

class MockVideoEncoder : public VideoEncoder {
 public:
  MOCK_METHOD(void,
              SetFecControllerOverride,
              (FecControllerOverride*),
              (override));
  MOCK_METHOD(int32_t,
              InitEncode,
              (const VideoCodec*, int32_t numberOfCores, size_t maxPayloadSize),
              (override));
  MOCK_METHOD(int32_t,
              InitEncode,
              (const VideoCodec*, const VideoEncoder::Settings& settings),
              (override));

  MOCK_METHOD(int32_t,
              Encode,
              (const VideoFrame& inputImage,
               const std::vector<VideoFrameType>*),
              (override));
  MOCK_METHOD(int32_t,
              RegisterEncodeCompleteCallback,
              (EncodedImageCallback*),
              (override));
  MOCK_METHOD(int32_t, Release, (), (override));
  MOCK_METHOD(void,
              SetRates,
              (const RateControlParameters& parameters),
              (override));
  MOCK_METHOD(void,
              OnPacketLossRateUpdate,
              (float packet_loss_rate),
              (override));
  MOCK_METHOD(void, OnRttUpdate, (int64_t rtt_ms), (override));
  MOCK_METHOD(void,
              OnLossNotification,
              (const LossNotification& loss_notification),
              (override));
  MOCK_METHOD(EncoderInfo, GetEncoderInfo, (), (const, override));
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_VIDEO_ENCODER_H_
