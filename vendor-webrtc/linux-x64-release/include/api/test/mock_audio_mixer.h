/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_AUDIO_MIXER_H_
#define API_TEST_MOCK_AUDIO_MIXER_H_

#include <cstddef>

#include "api/audio/audio_frame.h"
#include "api/audio/audio_mixer.h"
#include "test/gmock.h"

namespace webrtc {
namespace test {

class MockAudioMixer : public AudioMixer {
 public:
  MOCK_METHOD(bool, AddSource, (Source*), (override));
  MOCK_METHOD(void, RemoveSource, (Source*), (override));
  MOCK_METHOD(void, Mix, (size_t number_of_channels, AudioFrame*), (override));
};
}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_MOCK_AUDIO_MIXER_H_
