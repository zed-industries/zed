/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_UTILS_H_
#define VIDEO_CORRUPTION_DETECTION_UTILS_H_

#include "absl/strings/string_view.h"
#include "api/scoped_refptr.h"
#include "api/video/i420_buffer.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_frame_buffer.h"

namespace webrtc {

VideoCodecType GetVideoCodecType(absl::string_view codec_name);

scoped_refptr<I420Buffer> GetAsI420Buffer(
    scoped_refptr<I420BufferInterface> i420_buffer_interface);

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_UTILS_H_
