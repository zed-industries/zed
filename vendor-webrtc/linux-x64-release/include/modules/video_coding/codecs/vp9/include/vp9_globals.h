/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains codec dependent definitions that are needed in
// order to compile the WebRTC codebase, even if this codec is not used.

#ifndef MODULES_VIDEO_CODING_CODECS_VP9_INCLUDE_VP9_GLOBALS_H_
#define MODULES_VIDEO_CODING_CODECS_VP9_INCLUDE_VP9_GLOBALS_H_

#include <stdint.h>

#include <cstddef>

#include "modules/video_coding/codecs/interface/common_constants.h"
#include "rtc_base/checks.h"

namespace webrtc {

const int16_t kMaxOneBytePictureId = 0x7F;    // 7 bits
const int16_t kMaxTwoBytePictureId = 0x7FFF;  // 15 bits
const uint8_t kNoSpatialIdx = 0xFF;
const uint8_t kNoGofIdx = 0xFF;
const uint8_t kNumVp9Buffers = 8;
const size_t kMaxVp9RefPics = 3;
const size_t kMaxVp9FramesInGof = 0xFF;  // 8 bits
const size_t kMaxVp9NumberOfSpatialLayers = 8;

const int kMinVp9SpatialLayerLongSideLength = 240;
const int kMinVp9SpatialLayerShortSideLength = 135;

enum TemporalStructureMode {
  kTemporalStructureMode1,  // 1 temporal layer structure - i.e., IPPP...
  kTemporalStructureMode2,  // 2 temporal layers 01...
  kTemporalStructureMode3,  // 3 temporal layers 0212...
};

struct GofInfoVP9 {
  void SetGofInfoVP9(TemporalStructureMode tm) {
    switch (tm) {
      case kTemporalStructureMode1:
        num_frames_in_gof = 1;
        temporal_idx[0] = 0;
        temporal_up_switch[0] = true;
        num_ref_pics[0] = 1;
        pid_diff[0][0] = 1;
        break;
      case kTemporalStructureMode2:
        num_frames_in_gof = 2;
        temporal_idx[0] = 0;
        temporal_up_switch[0] = true;
        num_ref_pics[0] = 1;
        pid_diff[0][0] = 2;

        temporal_idx[1] = 1;
        temporal_up_switch[1] = true;
        num_ref_pics[1] = 1;
        pid_diff[1][0] = 1;
        break;
      case kTemporalStructureMode3:
        num_frames_in_gof = 4;
        temporal_idx[0] = 0;
        temporal_up_switch[0] = true;
        num_ref_pics[0] = 1;
        pid_diff[0][0] = 4;

        temporal_idx[1] = 2;
        temporal_up_switch[1] = true;
        num_ref_pics[1] = 1;
        pid_diff[1][0] = 1;

        temporal_idx[2] = 1;
        temporal_up_switch[2] = true;
        num_ref_pics[2] = 1;
        pid_diff[2][0] = 2;

        temporal_idx[3] = 2;
        temporal_up_switch[3] = true;
        num_ref_pics[3] = 1;
        pid_diff[3][0] = 1;
        break;
      default:
        RTC_DCHECK_NOTREACHED();
    }
  }

  void CopyGofInfoVP9(const GofInfoVP9& src) {
    num_frames_in_gof = src.num_frames_in_gof;
    for (size_t i = 0; i < num_frames_in_gof; ++i) {
      temporal_idx[i] = src.temporal_idx[i];
      temporal_up_switch[i] = src.temporal_up_switch[i];
      num_ref_pics[i] = src.num_ref_pics[i];
      for (uint8_t r = 0; r < num_ref_pics[i]; ++r) {
        pid_diff[i][r] = src.pid_diff[i][r];
      }
    }
  }

  friend bool operator==(const GofInfoVP9& lhs, const GofInfoVP9& rhs) {
    if (lhs.num_frames_in_gof != rhs.num_frames_in_gof ||
        lhs.pid_start != rhs.pid_start)
      return false;
    for (size_t i = 0; i < lhs.num_frames_in_gof; ++i) {
      if (lhs.temporal_idx[i] != rhs.temporal_idx[i] ||
          lhs.temporal_up_switch[i] != rhs.temporal_up_switch[i] ||
          lhs.num_ref_pics[i] != rhs.num_ref_pics[i]) {
        return false;
      }
      for (uint8_t r = 0; r < lhs.num_ref_pics[i]; ++r) {
        if (lhs.pid_diff[i][r] != rhs.pid_diff[i][r])
          return false;
      }
    }
    return true;
  }

  friend bool operator!=(const GofInfoVP9& lhs, const GofInfoVP9& rhs) {
    return !(lhs == rhs);
  }

  size_t num_frames_in_gof;
  uint8_t temporal_idx[kMaxVp9FramesInGof];
  bool temporal_up_switch[kMaxVp9FramesInGof];
  uint8_t num_ref_pics[kMaxVp9FramesInGof];
  uint8_t pid_diff[kMaxVp9FramesInGof][kMaxVp9RefPics];
  uint16_t pid_start;
};

struct RTPVideoHeaderVP9 {
  void InitRTPVideoHeaderVP9() {
    inter_pic_predicted = false;
    flexible_mode = false;
    beginning_of_frame = false;
    end_of_frame = false;
    ss_data_available = false;
    non_ref_for_inter_layer_pred = false;
    picture_id = kNoPictureId;
    max_picture_id = kMaxTwoBytePictureId;
    tl0_pic_idx = kNoTl0PicIdx;
    temporal_idx = kNoTemporalIdx;
    spatial_idx = kNoSpatialIdx;
    temporal_up_switch = false;
    inter_layer_predicted = false;
    gof_idx = kNoGofIdx;
    num_ref_pics = 0;
    num_spatial_layers = 1;
    first_active_layer = 0;
    end_of_picture = true;
  }

  friend bool operator==(const RTPVideoHeaderVP9& lhs,
                         const RTPVideoHeaderVP9& rhs) {
    if (lhs.inter_pic_predicted != rhs.inter_pic_predicted ||
        lhs.flexible_mode != rhs.flexible_mode ||
        lhs.beginning_of_frame != rhs.beginning_of_frame ||
        lhs.end_of_frame != rhs.end_of_frame ||
        lhs.ss_data_available != rhs.ss_data_available ||
        lhs.non_ref_for_inter_layer_pred != rhs.non_ref_for_inter_layer_pred ||
        lhs.picture_id != rhs.picture_id ||
        lhs.max_picture_id != rhs.max_picture_id ||
        lhs.tl0_pic_idx != rhs.tl0_pic_idx ||
        lhs.temporal_idx != rhs.temporal_idx ||
        lhs.spatial_idx != rhs.spatial_idx || lhs.gof_idx != rhs.gof_idx ||
        lhs.temporal_up_switch != rhs.temporal_up_switch ||
        lhs.inter_layer_predicted != rhs.inter_layer_predicted ||
        lhs.num_ref_pics != rhs.num_ref_pics ||
        lhs.end_of_picture != rhs.end_of_picture) {
      return false;
    }
    for (uint8_t i = 0; i < lhs.num_ref_pics; ++i) {
      if (lhs.pid_diff[i] != rhs.pid_diff[i] ||
          lhs.ref_picture_id[i] != rhs.ref_picture_id[i]) {
        return false;
      }
    }
    if (lhs.ss_data_available) {
      if (lhs.spatial_layer_resolution_present !=
              rhs.spatial_layer_resolution_present ||
          lhs.num_spatial_layers != rhs.num_spatial_layers ||
          lhs.first_active_layer != rhs.first_active_layer ||
          lhs.gof != rhs.gof) {
        return false;
      }
      if (lhs.spatial_layer_resolution_present) {
        for (size_t i = 0; i < lhs.num_spatial_layers; i++) {
          if (lhs.width[i] != rhs.width[i] || lhs.height[i] != rhs.height[i]) {
            return false;
          }
        }
      }
    }
    return true;
  }

  friend bool operator!=(const RTPVideoHeaderVP9& lhs,
                         const RTPVideoHeaderVP9& rhs) {
    return !(lhs == rhs);
  }

  bool inter_pic_predicted;  // This layer frame is dependent on previously
                             // coded frame(s).
  bool flexible_mode;        // This frame is in flexible mode.
  bool beginning_of_frame;   // True if this packet is the first in a VP9 layer
                             // frame.
  bool end_of_frame;  // True if this packet is the last in a VP9 layer frame.
  bool ss_data_available;  // True if SS data is available in this payload
                           // descriptor.
  bool non_ref_for_inter_layer_pred;  // True for frame which is not used as
                                      // reference for inter-layer prediction.
  int16_t picture_id;                 // PictureID index, 15 bits;
                       // kNoPictureId if PictureID does not exist.
  int16_t max_picture_id;   // Maximum picture ID index; either 0x7F or 0x7FFF;
  int16_t tl0_pic_idx;      // TL0PIC_IDX, 8 bits;
                            // kNoTl0PicIdx means no value provided.
  uint8_t temporal_idx;     // Temporal layer index, or kNoTemporalIdx.
  uint8_t spatial_idx;      // Spatial layer index, or kNoSpatialIdx.
  bool temporal_up_switch;  // True if upswitch to higher frame rate is possible
                            // meaning subsequent higher temporal layer pictures
                            // will not depend on any picture before the current
                            // picture (in coding order) with temporal layer ID
                            // greater than `temporal_idx` of this frame.
  bool inter_layer_predicted;  // Frame is dependent on directly lower spatial
                               // layer frame.

  uint8_t gof_idx;  // Index to predefined temporal frame info in SS data.

  uint8_t num_ref_pics;  // Number of reference pictures used by this layer
                         // frame.
  uint8_t pid_diff[kMaxVp9RefPics];  // P_DIFF signaled to derive the PictureID
                                     // of the reference pictures.
  int16_t ref_picture_id[kMaxVp9RefPics];  // PictureID of reference pictures.

  // SS data.
  size_t num_spatial_layers;  // Always populated.
  size_t first_active_layer;  // Not sent on wire, used to adjust ss data.
  bool spatial_layer_resolution_present;
  uint16_t width[kMaxVp9NumberOfSpatialLayers];
  uint16_t height[kMaxVp9NumberOfSpatialLayers];
  GofInfoVP9 gof;

  bool end_of_picture;  // This frame is the last frame in picture.
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_VP9_INCLUDE_VP9_GLOBALS_H_
