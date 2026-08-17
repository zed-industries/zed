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

#ifndef MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_VP8_GLOBALS_H_
#define MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_VP8_GLOBALS_H_

#include <cstdint>

#include "modules/video_coding/codecs/interface/common_constants.h"

namespace webrtc {

struct RTPVideoHeaderVP8 {
  void InitRTPVideoHeaderVP8() {
    nonReference = false;
    pictureId = kNoPictureId;
    tl0PicIdx = kNoTl0PicIdx;
    temporalIdx = kNoTemporalIdx;
    layerSync = false;
    keyIdx = kNoKeyIdx;
    partitionId = 0;
    beginningOfPartition = false;
  }

  friend bool operator==(const RTPVideoHeaderVP8& lhs,
                         const RTPVideoHeaderVP8& rhs) {
    return lhs.nonReference == rhs.nonReference &&
           lhs.pictureId == rhs.pictureId && lhs.tl0PicIdx == rhs.tl0PicIdx &&
           lhs.temporalIdx == rhs.temporalIdx &&
           lhs.layerSync == rhs.layerSync && lhs.keyIdx == rhs.keyIdx &&
           lhs.partitionId == rhs.partitionId &&
           lhs.beginningOfPartition == rhs.beginningOfPartition;
  }

  friend bool operator!=(const RTPVideoHeaderVP8& lhs,
                         const RTPVideoHeaderVP8& rhs) {
    return !(lhs == rhs);
  }

  bool nonReference;          // Frame is discardable.
  int16_t pictureId;          // Picture ID index, 15 bits;
                              // kNoPictureId if PictureID does not exist.
  int16_t tl0PicIdx;          // TL0PIC_IDX, 8 bits;
                              // kNoTl0PicIdx means no value provided.
  uint8_t temporalIdx;        // Temporal layer index, or kNoTemporalIdx.
  bool layerSync;             // This frame is a layer sync frame.
                              // Disabled if temporalIdx == kNoTemporalIdx.
  int keyIdx;                 // 5 bits; kNoKeyIdx means not used.
  int partitionId;            // VP8 partition ID
  bool beginningOfPartition;  // True if this packet is the first
                              // in a VP8 partition. Otherwise false
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_VP8_GLOBALS_H_
