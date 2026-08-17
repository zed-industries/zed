/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_HALTON_FRAME_SAMPLER_H_
#define VIDEO_CORRUPTION_DETECTION_HALTON_FRAME_SAMPLER_H_

#include <cstdint>
#include <optional>
#include <vector>

#include "api/scoped_refptr.h"
#include "api/video/video_frame_buffer.h"
#include "video/corruption_detection/halton_sequence.h"

namespace webrtc {

enum class ImagePlane { kLuma, kChroma };

struct FilteredSample {
  double value;
  ImagePlane plane;
};

// Determines if a frame should be sampled and, based on the 2 dimensional
// Halton sequence, finds the coordinates for those samples.
class HaltonFrameSampler {
 public:
  struct Coordinates {
    double row = 0;
    double column = 0;
  };

  HaltonFrameSampler();
  HaltonFrameSampler(const HaltonFrameSampler&) = default;
  HaltonFrameSampler(HaltonFrameSampler&&) = default;
  HaltonFrameSampler& operator=(const HaltonFrameSampler&) = default;
  HaltonFrameSampler& operator=(HaltonFrameSampler&&) = default;

  std::vector<Coordinates> GetSampleCoordinatesForFrameIfFrameShouldBeSampled(
      bool is_key_frame,
      uint32_t rtp_timestamp,
      int num_samples);
  std::vector<Coordinates> GetSampleCoordinatesForFrame(int num_samples);
  void Restart();
  int GetCurrentIndex() const;
  void SetCurrentIndex(int index);

 private:
  Coordinates GetNextSampleCoordinates();

  HaltonSequence coordinate_sampler_prng_;
  std::optional<uint32_t> rtp_timestamp_last_frame_sampled_;
  int frames_sampled_ = 0;
  int frames_until_next_sample_ = 0;
};

// 1. Scale the frame buffer to the resolution given by `scaled_width` and
// `scaled_height`.
// 2. Scale the `sample_coordinates` to the frame's resolution.
// 3. Apply the Gaussian filtering given by `std_dev_gaussian_blur`.
// 4. Fetch the values at the scaled coordinates in the filtered frame.
std::vector<FilteredSample> GetSampleValuesForFrame(
    scoped_refptr<I420BufferInterface> i420_frame_buffer,
    std::vector<HaltonFrameSampler::Coordinates> sample_coordinates,
    int scaled_width,
    int scaled_height,
    double std_dev_gaussian_blur);

// Returns the blurred value. The minimum half-kernel size is 3 pixels.
double GetFilteredElement(int width,
                          int height,
                          int stride,
                          const uint8_t* data,
                          int row,
                          int column,
                          double std_dev);

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_HALTON_FRAME_SAMPLER_H_
