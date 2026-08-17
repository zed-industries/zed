/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_FRAME_PAIR_CORRUPTION_SCORE_H_
#define VIDEO_CORRUPTION_DETECTION_FRAME_PAIR_CORRUPTION_SCORE_H_

#include <optional>

#include "absl/strings/string_view.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_frame_buffer.h"
#include "video/corruption_detection/corruption_classifier.h"
#include "video/corruption_detection/halton_frame_sampler.h"

namespace webrtc {

// Given a `reference_buffer` and a `test_buffer`, calculates the corruption
// score of the a frame pair. The score is calculated by comparing the sample
// values (each pixel has 3 sample values, the Y, U and V samples) of the
// reference buffer and the test buffer at a set of sampled coordinates.
//
// TODO: bugs.webrtc.org/358039777 - Remove one of the constructors based on
// which mapping function works best in practice.
// There are two constructors for this class. The first one takes a
// `scale_factor` as a parameter, which is used to calculate the scaling
// function. The second one takes a `growth_rate` and a `midpoint` as
// parameters, which are used to calculate the logistic function.
// `sample_fraction` is the fraction of pixels to sample. E.g. if
// `sample_fraction` = 0.5, then we sample 50% of the samples.
//
// The dimension of the `reference_buffer` and `test_buffer` does not need to be
// the same, in order to support downscaling caused by e.g. simulcast and
// scalable encoding. However, the dimensions of the `reference_buffer` must be
// larger than or equal to the dimensions of the `test_buffer`.
class FramePairCorruptionScorer {
 public:
  // `scale_factor` is the parameter constructing the scaling function, which is
  // used to calculate the corruption score. `sample_fraction` is the fraction
  // of pixels to sample.
  FramePairCorruptionScorer(absl::string_view codec_name,
                            float scale_factor,
                            std::optional<float> sample_fraction);

  // `growth_rate` and `midpoint` are parameters constructing a logistic
  // function, which is used to calculate the corruption score.
  // `sample_fraction` is the fraction of pixels to sample.
  FramePairCorruptionScorer(absl::string_view codec_name,
                            float growth_rate,
                            float midpoint,
                            std::optional<float> sample_fraction);

  ~FramePairCorruptionScorer() = default;

  // Returns the corruption score as a probability value between 0 and 1, where
  // 0 means no corruption and 1 means that the compressed frame is corrupted.
  //
  // However, note that the corruption score may not accurately reflect
  // corruption. E.g. even if the corruption score is 0, the compressed frame
  // may still be corrupted and vice versa.
  double CalculateScore(int qp,
                        I420BufferInterface& reference_buffer,
                        I420BufferInterface& test_buffer);

 private:
  const VideoCodecType codec_type_;
  const float sample_fraction_;

  HaltonFrameSampler halton_frame_sampler_;
  CorruptionClassifier corruption_classifier_;
};

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_FRAME_PAIR_CORRUPTION_SCORE_H_
