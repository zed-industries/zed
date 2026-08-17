/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_MOVING_MAX_H_
#define MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_MOVING_MAX_H_

#include <stddef.h>

namespace webrtc {

class MovingMax {
 public:
  explicit MovingMax(size_t window_size);
  ~MovingMax();

  void Update(float value);
  float max() const;
  // Reset all of the state in this class.
  void Clear();

 private:
  float max_value_ = 0.f;
  size_t counter_ = 0;
  size_t window_size_ = 1;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_ECHO_DETECTOR_MOVING_MAX_H_
