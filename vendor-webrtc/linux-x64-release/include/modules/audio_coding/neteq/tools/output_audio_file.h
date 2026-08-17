/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_OUTPUT_AUDIO_FILE_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_OUTPUT_AUDIO_FILE_H_

#include <stdio.h>

#include <string>

#include "absl/strings/string_view.h"
#include "modules/audio_coding/neteq/tools/audio_sink.h"

namespace webrtc {
namespace test {

class OutputAudioFile : public AudioSink {
 public:
  // Creates an OutputAudioFile, opening a file named `file_name` for writing.
  // The file format is 16-bit signed host-endian PCM.
  explicit OutputAudioFile(absl::string_view file_name) {
    out_file_ = fopen(std::string(file_name).c_str(), "wb");
  }

  virtual ~OutputAudioFile() {
    if (out_file_)
      fclose(out_file_);
  }

  OutputAudioFile(const OutputAudioFile&) = delete;
  OutputAudioFile& operator=(const OutputAudioFile&) = delete;

  bool WriteArray(const int16_t* audio, size_t num_samples) override {
    RTC_DCHECK(out_file_);
    return fwrite(audio, sizeof(*audio), num_samples, out_file_) == num_samples;
  }

 private:
  FILE* out_file_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_OUTPUT_AUDIO_FILE_H_
