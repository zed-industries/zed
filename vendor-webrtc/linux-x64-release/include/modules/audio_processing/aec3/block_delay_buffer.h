/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_BLOCK_DELAY_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_BLOCK_DELAY_BUFFER_H_

#include <stddef.h>

#include <vector>

#include "modules/audio_processing/audio_buffer.h"

namespace webrtc {

// Class for applying a fixed delay to the samples in a signal partitioned using
// the audiobuffer band-splitting scheme.
class BlockDelayBuffer {
 public:
  BlockDelayBuffer(size_t num_channels,
                   size_t num_bands,
                   size_t frame_length,
                   size_t delay_samples);
  ~BlockDelayBuffer();

  // Delays the samples by the specified delay.
  void DelaySignal(AudioBuffer* frame);

 private:
  const size_t frame_length_;
  const size_t delay_;
  std::vector<std::vector<std::vector<float>>> buf_;
  size_t last_insert_ = 0;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_BLOCK_DELAY_BUFFER_H_
