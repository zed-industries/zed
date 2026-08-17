/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_WAVREADER_INTERFACE_H_
#define MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_WAVREADER_INTERFACE_H_

#include <stddef.h>

#include "api/array_view.h"

namespace webrtc {
namespace test {
namespace conversational_speech {

class WavReaderInterface {
 public:
  virtual ~WavReaderInterface() = default;

  // Returns the number of samples read.
  virtual size_t ReadFloatSamples(ArrayView<float> samples) = 0;
  virtual size_t ReadInt16Samples(ArrayView<int16_t> samples) = 0;

  // Getters.
  virtual int SampleRate() const = 0;
  virtual size_t NumChannels() const = 0;
  virtual size_t NumSamples() const = 0;
};

}  // namespace conversational_speech
}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_WAVREADER_INTERFACE_H_
