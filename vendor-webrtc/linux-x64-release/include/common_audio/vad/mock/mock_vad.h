/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_VAD_MOCK_MOCK_VAD_H_
#define COMMON_AUDIO_VAD_MOCK_MOCK_VAD_H_

#include "common_audio/vad/include/vad.h"
#include "test/gmock.h"

namespace webrtc {

class MockVad : public Vad {
 public:
  ~MockVad() override { Die(); }
  MOCK_METHOD(void, Die, ());

  MOCK_METHOD(enum Activity,
              VoiceActivity,
              (const int16_t* audio, size_t num_samples, int sample_rate_hz),
              (override));
  MOCK_METHOD(void, Reset, (), (override));
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_VAD_MOCK_MOCK_VAD_H_
