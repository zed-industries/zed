/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_FRAME_INSTRUMENTATION_DATA_H_
#define COMMON_VIDEO_FRAME_INSTRUMENTATION_DATA_H_

#include <vector>

namespace webrtc {

// TODO: bugs.webrtc.org/358039777 - Error handling: negative values etc.
struct FrameInstrumentationSyncData {
  int sequence_index;
  bool communicate_upper_bits;
};

struct FrameInstrumentationData {
  int sequence_index;
  bool communicate_upper_bits;
  double std_dev;
  int luma_error_threshold;
  int chroma_error_threshold;
  std::vector<double> sample_values;
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_FRAME_INSTRUMENTATION_DATA_H_
