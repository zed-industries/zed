/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MOCK_WAVREADER_FACTORY_H_
#define MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MOCK_WAVREADER_FACTORY_H_

#include <map>
#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "modules/audio_processing/test/conversational_speech/wavreader_abstract_factory.h"
#include "modules/audio_processing/test/conversational_speech/wavreader_interface.h"
#include "test/gmock.h"

namespace webrtc {
namespace test {
namespace conversational_speech {

class MockWavReaderFactory : public WavReaderAbstractFactory {
 public:
  struct Params {
    int sample_rate;
    size_t num_channels;
    size_t num_samples;
  };

  MockWavReaderFactory(const Params& default_params,
                       const std::map<std::string, const Params>& params);
  explicit MockWavReaderFactory(const Params& default_params);
  ~MockWavReaderFactory();

  MOCK_METHOD(std::unique_ptr<WavReaderInterface>,
              Create,
              (absl::string_view),
              (const, override));

 private:
  // Creates a MockWavReader instance using the parameters in
  // audiotrack_names_params_ if the entry corresponding to filepath exists,
  // otherwise creates a MockWavReader instance using the default parameters.
  std::unique_ptr<WavReaderInterface> CreateMock(absl::string_view filepath);

  const Params& default_params_;
  std::map<std::string, const Params> audiotrack_names_params_;
};

}  // namespace conversational_speech
}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MOCK_WAVREADER_FACTORY_H_
