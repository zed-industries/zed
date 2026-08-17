/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_CONFIG_H_
#define MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_CONFIG_H_

#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
namespace test {
namespace conversational_speech {

struct Config {
  Config(absl::string_view audiotracks_path,
         absl::string_view timing_filepath,
         absl::string_view output_path)
      : audiotracks_path_(audiotracks_path),
        timing_filepath_(timing_filepath),
        output_path_(output_path) {}

  const std::string& audiotracks_path() const;
  const std::string& timing_filepath() const;
  const std::string& output_path() const;

  const std::string audiotracks_path_;
  const std::string timing_filepath_;
  const std::string output_path_;
};

}  // namespace conversational_speech
}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_CONVERSATIONAL_SPEECH_CONFIG_H_
