/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_INPUT_AUDIO_FILE_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_INPUT_AUDIO_FILE_H_

#include <stdint.h>
#include <stdio.h>

#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
namespace test {

// Class for handling a looping input audio file.
class InputAudioFile {
 public:
  explicit InputAudioFile(absl::string_view file_name, bool loop_at_end = true);

  virtual ~InputAudioFile();

  InputAudioFile(const InputAudioFile&) = delete;
  InputAudioFile& operator=(const InputAudioFile&) = delete;

  // Reads `samples` elements from source file to `destination`. Returns true
  // if the read was successful, otherwise false. If the file end is reached,
  // the file is rewound and reading continues from the beginning.
  // The output `destination` must have the capacity to hold `samples` elements.
  virtual bool Read(size_t samples, int16_t* destination);

  // Fast-forwards (`samples` > 0) or -backwards (`samples` < 0) the file by the
  // indicated number of samples. Just like Read(), Seek() starts over at the
  // beginning of the file if the end is reached. However, seeking backwards
  // past the beginning of the file is not possible.
  virtual bool Seek(int samples);

  // Creates a multi-channel signal from a mono signal. Each sample is repeated
  // `channels` times to create an interleaved multi-channel signal where all
  // channels are identical. The output `destination` must have the capacity to
  // hold samples * channels elements. Note that `source` and `destination` can
  // be the same array (i.e., point to the same address).
  static void DuplicateInterleaved(const int16_t* source,
                                   size_t samples,
                                   size_t channels,
                                   int16_t* destination);

 private:
  FILE* fp_;
  const bool loop_at_end_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_INPUT_AUDIO_FILE_H_
