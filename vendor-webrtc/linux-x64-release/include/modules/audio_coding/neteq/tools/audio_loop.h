/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_LOOP_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_LOOP_H_

#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"

namespace webrtc {
namespace test {

// Class serving as an infinite source of audio, realized by looping an audio
// clip.
class AudioLoop {
 public:
  AudioLoop()
      : next_index_(0), loop_length_samples_(0), block_length_samples_(0) {}

  virtual ~AudioLoop() {}

  AudioLoop(const AudioLoop&) = delete;
  AudioLoop& operator=(const AudioLoop&) = delete;

  // Initializes the AudioLoop by reading from `file_name`. The loop will be no
  // longer than `max_loop_length_samples`, if the length of the file is
  // greater. Otherwise, the loop length is the same as the file length.
  // The audio will be delivered in blocks of `block_length_samples`.
  // Returns false if the initialization failed, otherwise true.
  bool Init(absl::string_view file_name,
            size_t max_loop_length_samples,
            size_t block_length_samples);

  // Returns a (pointer,size) pair for the next block of audio. The size is
  // equal to the `block_length_samples` Init() argument.
  ArrayView<const int16_t> GetNextBlock();

 private:
  size_t next_index_;
  size_t loop_length_samples_;
  size_t block_length_samples_;
  std::unique_ptr<int16_t[]> audio_array_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_LOOP_H_
