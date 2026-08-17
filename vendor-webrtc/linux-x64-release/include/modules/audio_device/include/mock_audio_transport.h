/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_INCLUDE_MOCK_AUDIO_TRANSPORT_H_
#define MODULES_AUDIO_DEVICE_INCLUDE_MOCK_AUDIO_TRANSPORT_H_

#include "api/audio/audio_device_defines.h"
#include "test/gmock.h"

namespace webrtc {
namespace test {

class MockAudioTransport : public AudioTransport {
 public:
  MockAudioTransport() {}
  ~MockAudioTransport() {}

  MOCK_METHOD(int32_t,
              RecordedDataIsAvailable,
              (const void* audioSamples,
               size_t nSamples,
               size_t nBytesPerSample,
               size_t nChannels,
               uint32_t samplesPerSec,
               uint32_t totalDelayMS,
               int32_t clockDrift,
               uint32_t currentMicLevel,
               bool keyPressed,
               uint32_t& newMicLevel),
              (override));

  MOCK_METHOD(int32_t,
              RecordedDataIsAvailable,
              (const void* audioSamples,
               size_t nSamples,
               size_t nBytesPerSample,
               size_t nChannels,
               uint32_t samplesPerSec,
               uint32_t totalDelayMS,
               int32_t clockDrift,
               uint32_t currentMicLevel,
               bool keyPressed,
               uint32_t& newMicLevel,
               std::optional<int64_t> estimated_capture_time_ns),
              (override));

  MOCK_METHOD(int32_t,
              NeedMorePlayData,
              (size_t nSamples,
               size_t nBytesPerSample,
               size_t nChannels,
               uint32_t samplesPerSec,
               void* audioSamples,
               size_t& nSamplesOut,
               int64_t* elapsed_time_ms,
               int64_t* ntp_time_ms),
              (override));

  MOCK_METHOD(void,
              PullRenderData,
              (int bits_per_sample,
               int sample_rate,
               size_t number_of_channels,
               size_t number_of_frames,
               void* audio_data,
               int64_t* elapsed_time_ms,
               int64_t* ntp_time_ms),
              (override));
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_INCLUDE_MOCK_AUDIO_TRANSPORT_H_
