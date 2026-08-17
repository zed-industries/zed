/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC_DUMP_MOCK_AEC_DUMP_H_
#define MODULES_AUDIO_PROCESSING_AEC_DUMP_MOCK_AEC_DUMP_H_

#include <memory>

#include "modules/audio_processing/include/aec_dump.h"
#include "test/gmock.h"

namespace webrtc {

namespace test {

class MockAecDump : public AecDump {
 public:
  MockAecDump();
  virtual ~MockAecDump();

  MOCK_METHOD(void,
              WriteInitMessage,
              (const ProcessingConfig& api_format, int64_t time_now_ms),
              (override));

  MOCK_METHOD(void,
              AddCaptureStreamInput,
              (const AudioFrameView<const float>& src),
              (override));
  MOCK_METHOD(void,
              AddCaptureStreamOutput,
              (const AudioFrameView<const float>& src),
              (override));
  MOCK_METHOD(void,
              AddCaptureStreamInput,
              (const int16_t* const data,
               int num_channels,
               int samples_per_channel),
              (override));
  MOCK_METHOD(void,
              AddCaptureStreamOutput,
              (const int16_t* const data,
               int num_channels,
               int samples_per_channel),
              (override));
  MOCK_METHOD(void,
              AddAudioProcessingState,
              (const AudioProcessingState& state),
              (override));
  MOCK_METHOD(void, WriteCaptureStreamMessage, (), (override));

  MOCK_METHOD(void,
              WriteRenderStreamMessage,
              (const int16_t* const data,
               int num_channels,
               int samples_per_channel),
              (override));
  MOCK_METHOD(void,
              WriteRenderStreamMessage,
              (const AudioFrameView<const float>& src),
              (override));

  MOCK_METHOD(void, WriteConfig, (const InternalAPMConfig& config), (override));

  MOCK_METHOD(void,
              WriteRuntimeSetting,
              (const AudioProcessing::RuntimeSetting& config),
              (override));
};

}  // namespace test

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC_DUMP_MOCK_AEC_DUMP_H_
