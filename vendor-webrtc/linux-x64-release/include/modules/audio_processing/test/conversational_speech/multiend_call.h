/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MULTIEND_CALL_H_
#define MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MULTIEND_CALL_H_

#include <stddef.h>

#include <map>
#include <memory>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "modules/audio_processing/test/conversational_speech/timing.h"
#include "modules/audio_processing/test/conversational_speech/wavreader_abstract_factory.h"
#include "modules/audio_processing/test/conversational_speech/wavreader_interface.h"

namespace webrtc {
namespace test {
namespace conversational_speech {

class MultiEndCall {
 public:
  struct SpeakingTurn {
    // Constructor required in order to use std::vector::emplace_back().
    SpeakingTurn(absl::string_view new_speaker_name,
                 absl::string_view new_audiotrack_file_name,
                 size_t new_begin,
                 size_t new_end,
                 int gain)
        : speaker_name(new_speaker_name),
          audiotrack_file_name(new_audiotrack_file_name),
          begin(new_begin),
          end(new_end),
          gain(gain) {}
    std::string speaker_name;
    std::string audiotrack_file_name;
    size_t begin;
    size_t end;
    int gain;
  };

  MultiEndCall(
      ArrayView<const Turn> timing,
      absl::string_view audiotracks_path,
      std::unique_ptr<WavReaderAbstractFactory> wavreader_abstract_factory);
  ~MultiEndCall();

  MultiEndCall(const MultiEndCall&) = delete;
  MultiEndCall& operator=(const MultiEndCall&) = delete;

  const std::set<std::string>& speaker_names() const { return speaker_names_; }
  const std::map<std::string, std::unique_ptr<WavReaderInterface>>&
  audiotrack_readers() const {
    return audiotrack_readers_;
  }
  bool valid() const { return valid_; }
  int sample_rate() const { return sample_rate_hz_; }
  size_t total_duration_samples() const { return total_duration_samples_; }
  const std::vector<SpeakingTurn>& speaking_turns() const {
    return speaking_turns_;
  }

 private:
  // Finds unique speaker names.
  void FindSpeakerNames();

  // Creates one WavReader instance for each unique audiotrack. It returns false
  // if the audio tracks do not have the same sample rate or if they are not
  // mono.
  bool CreateAudioTrackReaders();

  // Validates the speaking turns timing information. Accepts cross-talk, but
  // only up to 2 speakers. Rejects unordered turns and self cross-talk.
  bool CheckTiming();

  ArrayView<const Turn> timing_;
  std::string audiotracks_path_;
  std::unique_ptr<WavReaderAbstractFactory> wavreader_abstract_factory_;
  std::set<std::string> speaker_names_;
  std::map<std::string, std::unique_ptr<WavReaderInterface>>
      audiotrack_readers_;
  bool valid_;
  int sample_rate_hz_;
  size_t total_duration_samples_;
  std::vector<SpeakingTurn> speaking_turns_;
};

}  // namespace conversational_speech
}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_MULTIEND_CALL_H_
