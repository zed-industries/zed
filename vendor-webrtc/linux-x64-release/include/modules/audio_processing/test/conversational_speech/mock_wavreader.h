/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MOCK_WAVREADER_H_
#define MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MOCK_WAVREADER_H_

#include <cstddef>
#include <string>

#include "api/array_view.h"
#include "modules/audio_processing/test/conversational_speech/wavreader_interface.h"
#include "test/gmock.h"

namespace webrtc {
namespace test {
namespace conversational_speech {

class MockWavReader : public WavReaderInterface {
 public:
  MockWavReader(int sample_rate, size_t num_channels, size_t num_samples);
  ~MockWavReader();

  MOCK_METHOD(size_t, ReadFloatSamples, (webrtc::ArrayView<float>), (override));
  MOCK_METHOD(size_t,
              ReadInt16Samples,
              (webrtc::ArrayView<int16_t>),
              (override));

  MOCK_METHOD(int, SampleRate, (), (const, override));
  MOCK_METHOD(size_t, NumChannels, (), (const, override));
  MOCK_METHOD(size_t, NumSamples, (), (const, override));

 private:
  const int sample_rate_;
  const size_t num_channels_;
  const size_t num_samples_;
};

}  // namespace conversational_speech
}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MOCK_WAVREADER_H_
