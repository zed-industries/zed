/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DTMF_TONE_GENERATOR_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DTMF_TONE_GENERATOR_H_

#include "modules/audio_coding/neteq/dtmf_tone_generator.h"
#include "test/gmock.h"

namespace webrtc {

class MockDtmfToneGenerator : public DtmfToneGenerator {
 public:
  ~MockDtmfToneGenerator() override { Die(); }
  MOCK_METHOD(void, Die, ());
  MOCK_METHOD(int, Init, (int fs, int event, int attenuation), (override));
  MOCK_METHOD(void, Reset, (), (override));
  MOCK_METHOD(int,
              Generate,
              (size_t num_samples, AudioMultiVector* output),
              (override));
  MOCK_METHOD(bool, initialized, (), (const, override));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DTMF_TONE_GENERATOR_H_
