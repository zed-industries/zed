/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef VIDEO_TEST_MOCK_VIDEO_STREAM_ENCODER_H_
#define VIDEO_TEST_MOCK_VIDEO_STREAM_ENCODER_H_

#include <vector>

#include "test/gmock.h"
#include "video/video_stream_encoder_interface.h"

namespace webrtc {

class MockVideoStreamEncoder : public VideoStreamEncoderInterface {
 public:
  MOCK_METHOD(void,
              AddAdaptationResource,
              (webrtc::scoped_refptr<Resource>),
              (override));
  MOCK_METHOD(std::vector<scoped_refptr<Resource>>,
              GetAdaptationResources,
              (),
              (override));
  MOCK_METHOD(void,
              SetSource,
              (webrtc::VideoSourceInterface<VideoFrame>*,
               const DegradationPreference&),
              (override));
  MOCK_METHOD(void, SetSink, (EncoderSink*, bool), (override));
  MOCK_METHOD(void, SetStartBitrate, (int), (override));
  MOCK_METHOD(void,
              SendKeyFrame,
              (const std::vector<VideoFrameType>&),
              (override));
  MOCK_METHOD(void,
              OnLossNotification,
              (const VideoEncoder::LossNotification&),
              (override));
  MOCK_METHOD(void,
              OnBitrateUpdated,
              (DataRate, DataRate, DataRate, uint8_t, int64_t, double),
              (override));
  MOCK_METHOD(void,
              SetFecControllerOverride,
              (FecControllerOverride*),
              (override));
  MOCK_METHOD(void, Stop, (), (override));

  MOCK_METHOD(void,
              MockedConfigureEncoder,
              (const VideoEncoderConfig&, size_t));
  MOCK_METHOD(void,
              MockedConfigureEncoder,
              (const VideoEncoderConfig&, size_t, SetParametersCallback));
  // gtest generates implicit copy which is not allowed on VideoEncoderConfig,
  // so we can't mock ConfigureEncoder directly.
  void ConfigureEncoder(VideoEncoderConfig config,
                        size_t max_data_payload_length) {
    MockedConfigureEncoder(config, max_data_payload_length);
  }
  void ConfigureEncoder(VideoEncoderConfig config,
                        size_t max_data_payload_length,
                        SetParametersCallback) {
    MockedConfigureEncoder(config, max_data_payload_length);
  }
};

}  // namespace webrtc

#endif  // VIDEO_TEST_MOCK_VIDEO_STREAM_ENCODER_H_
