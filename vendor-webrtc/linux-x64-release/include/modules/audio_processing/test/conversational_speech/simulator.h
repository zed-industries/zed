/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_SIMULATOR_H_
#define MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_SIMULATOR_H_

#include <map>
#include <memory>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "modules/audio_processing/test/conversational_speech/multiend_call.h"

namespace webrtc {
namespace test {
namespace conversational_speech {

struct SpeakerOutputFilePaths {
  SpeakerOutputFilePaths(absl::string_view new_near_end,
                         absl::string_view new_far_end)
      : near_end(new_near_end), far_end(new_far_end) {}
  // Paths to the near-end and far-end audio track files.
  const std::string near_end;
  const std::string far_end;
};

// Generates the near-end and far-end audio track pairs for each speaker.
std::unique_ptr<std::map<std::string, SpeakerOutputFilePaths>> Simulate(
    const MultiEndCall& multiend_call,
    absl::string_view output_path);

}  // namespace conversational_speech
}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_SIMULATOR_H_
