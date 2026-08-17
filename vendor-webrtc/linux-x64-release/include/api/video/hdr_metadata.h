/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_HDR_METADATA_H_
#define API_VIDEO_HDR_METADATA_H_

namespace webrtc {

// SMPTE ST 2086 mastering metadata,
// see https://ieeexplore.ieee.org/document/8353899.
struct HdrMasteringMetadata {
  struct Chromaticity {
    Chromaticity();

    bool operator==(const Chromaticity& rhs) const {
      return x == rhs.x && y == rhs.y;
    }

    bool Validate() const {
      return x >= 0.0 && x <= 1.0 && y >= 0.0 && y <= 1.0;
    }

    // xy chromaticity coordinates must be calculated as specified in ISO
    // 11664-3:2012 Section 7, and must be specified with four decimal places.
    // The x coordinate should be in the range [0.0001, 0.7400] and the y
    // coordinate should be in the range [0.0001, 0.8400]. Valid range [0.0000,
    // 1.0000].
    float x = 0.0f;
    float y = 0.0f;
  };

  HdrMasteringMetadata();

  bool operator==(const HdrMasteringMetadata& rhs) const {
    return ((primary_r == rhs.primary_r) && (primary_g == rhs.primary_g) &&
            (primary_b == rhs.primary_b) && (white_point == rhs.white_point) &&
            (luminance_max == rhs.luminance_max) &&
            (luminance_min == rhs.luminance_min));
  }

  bool Validate() const {
    return luminance_max >= 0.0 && luminance_max <= 20000.0 &&
           luminance_min >= 0.0 && luminance_min <= 5.0 &&
           primary_r.Validate() && primary_g.Validate() &&
           primary_b.Validate() && white_point.Validate();
  }

  // The nominal primaries of the mastering display.
  Chromaticity primary_r;
  Chromaticity primary_g;
  Chromaticity primary_b;

  // The nominal chromaticity of the white point of the mastering display.
  Chromaticity white_point;

  // The nominal maximum display luminance of the mastering display. Specified
  // in the unit candela/m2. The value should be in the range [5, 10000] with
  // zero decimal places. Valid range [0, 20000].
  float luminance_max = 0.0f;

  // The nominal minimum display luminance of the mastering display. Specified
  // in the unit candela/m2. The value should be in the range [0.0001, 5.0000]
  // with four decimal places. Valid range [0.0000, 5.0000].
  float luminance_min = 0.0f;
};

// High dynamic range (HDR) metadata common for HDR10 and WebM/VP9-based HDR
// formats. This struct replicates the HDRMetadata struct defined in
// https://cs.chromium.org/chromium/src/media/base/hdr_metadata.h
struct HdrMetadata {
  HdrMetadata();

  bool operator==(const HdrMetadata& rhs) const {
    return (
        (max_content_light_level == rhs.max_content_light_level) &&
        (max_frame_average_light_level == rhs.max_frame_average_light_level) &&
        (mastering_metadata == rhs.mastering_metadata));
  }

  bool Validate() const {
    return max_content_light_level >= 0 && max_content_light_level <= 20000 &&
           max_frame_average_light_level >= 0 &&
           max_frame_average_light_level <= 20000 &&
           mastering_metadata.Validate();
  }

  HdrMasteringMetadata mastering_metadata;
  // Max content light level (CLL), i.e. maximum brightness level present in the
  // stream, in nits. 1 nit = 1 candela/m2. Valid range [0, 20000].
  int max_content_light_level = 0;
  // Max frame-average light level (FALL), i.e. maximum average brightness of
  // the brightest frame in the stream, in nits. Valid range [0, 20000].
  int max_frame_average_light_level = 0;
};

}  // namespace webrtc

#endif  // API_VIDEO_HDR_METADATA_H_
