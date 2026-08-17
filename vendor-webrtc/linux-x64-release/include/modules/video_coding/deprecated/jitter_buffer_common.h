/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_DEPRECATED_JITTER_BUFFER_COMMON_H_
#define MODULES_VIDEO_CODING_DEPRECATED_JITTER_BUFFER_COMMON_H_

namespace webrtc {

// Used to estimate rolling average of packets per frame.
static const float kFastConvergeMultiplier = 0.4f;
static const float kNormalConvergeMultiplier = 0.2f;

enum { kMaxNumberOfFrames = 300 };
enum { kStartNumberOfFrames = 6 };
enum { kMaxVideoDelayMs = 10000 };
enum { kPacketsPerFrameMultiplier = 5 };
enum { kFastConvergeThreshold = 5 };

enum VCMJitterBufferEnum {
  kMaxConsecutiveOldFrames = 60,
  kMaxConsecutiveOldPackets = 300,
  // TODO(sprang): Reduce this limit once codecs don't sometimes wildly
  // overshoot bitrate target.
  kMaxPacketsInSession = 1400,      // Allows ~2MB frames.
  kBufferIncStepSizeBytes = 30000,  // >20 packets.
  kMaxJBFrameSizeBytes = 4000000    // sanity don't go above 4Mbyte.
};

enum VCMFrameBufferEnum {
  kOutOfBoundsPacket = -7,
  kNotInitialized = -6,
  kOldPacket = -5,
  kGeneralError = -4,
  kFlushIndicator = -3,  // Indicator that a flush has occurred.
  kTimeStampError = -2,
  kSizeError = -1,
  kNoError = 0,
  kIncomplete = 1,       // Frame incomplete.
  kCompleteSession = 3,  // at least one layer in the frame complete.
  kDuplicatePacket = 5   // We're receiving a duplicate packet.
};

enum VCMFrameBufferStateEnum {
  kStateEmpty,       // frame popped by the RTP receiver
  kStateIncomplete,  // frame that have one or more packet(s) stored
  kStateComplete,    // frame that have all packets
};

enum { kH264StartCodeLengthBytes = 4 };
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_DEPRECATED_JITTER_BUFFER_COMMON_H_
