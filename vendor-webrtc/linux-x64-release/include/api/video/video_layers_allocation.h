/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_VIDEO_LAYERS_ALLOCATION_H_
#define API_VIDEO_VIDEO_LAYERS_ALLOCATION_H_

#include <cstdint>

#include "absl/container/inlined_vector.h"
#include "api/units/data_rate.h"

namespace webrtc {

// This struct contains additional stream-level information needed by a
// Selective Forwarding Middlebox to make relay decisions of RTP streams.
struct VideoLayersAllocation {
  static constexpr int kMaxSpatialIds = 4;
  static constexpr int kMaxTemporalIds = 4;

  friend bool operator==(const VideoLayersAllocation& lhs,
                         const VideoLayersAllocation& rhs) {
    return lhs.rtp_stream_index == rhs.rtp_stream_index &&
           lhs.resolution_and_frame_rate_is_valid ==
               rhs.resolution_and_frame_rate_is_valid &&
           lhs.active_spatial_layers == rhs.active_spatial_layers;
  }

  friend bool operator!=(const VideoLayersAllocation& lhs,
                         const VideoLayersAllocation& rhs) {
    return !(lhs == rhs);
  }

  struct SpatialLayer {
    friend bool operator==(const SpatialLayer& lhs, const SpatialLayer& rhs) {
      return lhs.rtp_stream_index == rhs.rtp_stream_index &&
             lhs.spatial_id == rhs.spatial_id &&
             lhs.target_bitrate_per_temporal_layer ==
                 rhs.target_bitrate_per_temporal_layer &&
             lhs.width == rhs.width && lhs.height == rhs.height &&
             lhs.frame_rate_fps == rhs.frame_rate_fps;
    }

    friend bool operator!=(const SpatialLayer& lhs, const SpatialLayer& rhs) {
      return !(lhs == rhs);
    }
    int rtp_stream_index = 0;
    // Index of the spatial layer per `rtp_stream_index`.
    int spatial_id = 0;
    // Target bitrate per decode target.
    absl::InlinedVector<DataRate, kMaxTemporalIds>
        target_bitrate_per_temporal_layer;

    // These fields are only valid if `resolution_and_frame_rate_is_valid` is
    // true
    uint16_t width = 0;
    uint16_t height = 0;
    // Max frame rate used in any temporal layer of this spatial layer.
    uint8_t frame_rate_fps = 0;
  };

  // Index of the rtp stream this allocation is sent on. Used for mapping
  // a SpatialLayer to a rtp stream.
  int rtp_stream_index = 0;
  bool resolution_and_frame_rate_is_valid = false;
  absl::InlinedVector<SpatialLayer, kMaxSpatialIds> active_spatial_layers;
};

}  // namespace webrtc

#endif  // API_VIDEO_VIDEO_LAYERS_ALLOCATION_H_
