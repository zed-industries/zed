/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_RESAMPLE_INPUT_AUDIO_FILE_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_RESAMPLE_INPUT_AUDIO_FILE_H_

#include <string>

#include "absl/strings/string_view.h"
#include "common_audio/resampler/include/resampler.h"
#include "modules/audio_coding/neteq/tools/input_audio_file.h"

namespace webrtc {
namespace test {

// Class for handling a looping input audio file with resampling.
class ResampleInputAudioFile : public InputAudioFile {
 public:
  ResampleInputAudioFile(absl::string_view file_name,
                         int file_rate_hz,
                         bool loop_at_end = true)
      : InputAudioFile(file_name, loop_at_end),
        file_rate_hz_(file_rate_hz),
        output_rate_hz_(-1) {}
  ResampleInputAudioFile(absl::string_view file_name,
                         int file_rate_hz,
                         int output_rate_hz,
                         bool loop_at_end = true)
      : InputAudioFile(file_name, loop_at_end),
        file_rate_hz_(file_rate_hz),
        output_rate_hz_(output_rate_hz) {}

  ResampleInputAudioFile(const ResampleInputAudioFile&) = delete;
  ResampleInputAudioFile& operator=(const ResampleInputAudioFile&) = delete;

  bool Read(size_t samples, int output_rate_hz, int16_t* destination);
  bool Read(size_t samples, int16_t* destination) override;
  void set_output_rate_hz(int rate_hz);

 private:
  const int file_rate_hz_;
  int output_rate_hz_;
  Resampler resampler_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_RESAMPLE_INPUT_AUDIO_FILE_H_
