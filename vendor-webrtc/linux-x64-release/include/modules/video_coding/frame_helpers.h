/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_FRAME_HELPERS_H_
#define MODULES_VIDEO_CODING_FRAME_HELPERS_H_

#include <memory>

#include "absl/container/inlined_vector.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/encoded_frame.h"

namespace webrtc {

bool FrameHasBadRenderTiming(Timestamp render_time, Timestamp now);

bool TargetVideoDelayIsTooLarge(TimeDelta target_video_delay);

std::unique_ptr<EncodedFrame> CombineAndDeleteFrames(
    absl::InlinedVector<std::unique_ptr<EncodedFrame>, 4> frames);

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_FRAME_HELPERS_H_
