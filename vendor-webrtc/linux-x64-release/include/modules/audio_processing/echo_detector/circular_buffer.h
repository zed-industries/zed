/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_CIRCULAR_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_CIRCULAR_BUFFER_H_

#include <stddef.h>

#include <optional>
#include <vector>

namespace webrtc {

// Ring buffer containing floating point values.
struct CircularBuffer {
 public:
  explicit CircularBuffer(size_t size);
  ~CircularBuffer();

  void Push(float value);
  std::optional<float> Pop();
  size_t Size() const { return nr_elements_in_buffer_; }
  // This function fills the buffer with zeros, but does not change its size.
  void Clear();

 private:
  std::vector<float> buffer_;
  size_t next_insertion_index_ = 0;
  // This is the number of elements that have been pushed into the circular
  // buffer, not the allocated buffer size.
  size_t nr_elements_in_buffer_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_CIRCULAR_BUFFER_H_
