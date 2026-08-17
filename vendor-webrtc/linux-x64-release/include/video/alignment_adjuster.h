/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ALIGNMENT_ADJUSTER_H_
#define VIDEO_ALIGNMENT_ADJUSTER_H_

#include "api/video_codecs/video_encoder.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {

class AlignmentAdjuster {
 public:
  // Returns the resolution alignment requested by the encoder (i.e
  // `EncoderInfo::requested_resolution_alignment` which ensures that delivered
  // frames to the encoder are divisible by this alignment).
  //
  // If `EncoderInfo::apply_alignment_to_all_simulcast_layers` is enabled, the
  // alignment will be adjusted to ensure that each simulcast layer also is
  // divisible by `requested_resolution_alignment`. The configured scale factors
  // `scale_resolution_down_by` may be adjusted to a common multiple to limit
  // the alignment value to avoid largely cropped frames and possibly with an
  // aspect ratio far from the original.

  // Note: `max_layers` currently only taken into account when using default
  // scale factors.
  static int GetAlignmentAndMaybeAdjustScaleFactors(
      const VideoEncoder::EncoderInfo& info,
      VideoEncoderConfig* config,
      std::optional<size_t> max_layers);
};

}  // namespace webrtc

#endif  // VIDEO_ALIGNMENT_ADJUSTER_H_
