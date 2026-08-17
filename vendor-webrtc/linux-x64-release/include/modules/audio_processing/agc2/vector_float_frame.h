/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_VECTOR_FLOAT_FRAME_H_
#define MODULES_AUDIO_PROCESSING_AGC2_VECTOR_FLOAT_FRAME_H_

#include <vector>

#include "api/audio/audio_view.h"
#include "modules/audio_processing/include/audio_frame_view.h"

namespace webrtc {

// A construct consisting of a multi-channel audio frame, and a FloatFrame view
// of it.
class VectorFloatFrame {
 public:
  VectorFloatFrame(int num_channels,
                   int samples_per_channel,
                   float start_value);
  ~VectorFloatFrame();

  AudioFrameView<float> float_frame_view();
  AudioFrameView<const float> float_frame_view() const;

  DeinterleavedView<float> view() { return view_; }
  DeinterleavedView<const float> view() const { return view_; }

 private:
  std::vector<float> channels_;
  DeinterleavedView<float> view_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_VECTOR_FLOAT_FRAME_H_
