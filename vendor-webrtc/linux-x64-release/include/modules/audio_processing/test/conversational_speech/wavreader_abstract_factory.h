/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_WAVREADER_ABSTRACT_FACTORY_H_
#define MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_WAVREADER_ABSTRACT_FACTORY_H_

#include <memory>

#include "absl/strings/string_view.h"
#include "modules/audio_processing/test/conversational_speech/wavreader_interface.h"

namespace webrtc {
namespace test {
namespace conversational_speech {

class WavReaderAbstractFactory {
 public:
  virtual ~WavReaderAbstractFactory() = default;
  virtual std::unique_ptr<WavReaderInterface> Create(
      absl::string_view filepath) const = 0;
};

}  // namespace conversational_speech
}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_WAVREADER_ABSTRACT_FACTORY_H_
