/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_FRAME_ANALYZER_VIDEO_COLOR_ALIGNER_H_
#define RTC_TOOLS_FRAME_ANALYZER_VIDEO_COLOR_ALIGNER_H_

#include <array>

#include "api/scoped_refptr.h"
#include "api/video/video_frame_buffer.h"
#include "rtc_tools/video_file_reader.h"

namespace webrtc {
namespace test {

// Represents a linear color transformation from [y, u, v] to [y', u', v']
// through the equation: [y', u', v'] = [y, u, v, 1] * matrix.
using ColorTransformationMatrix = std::array<std::array<float, 4>, 3>;

// Calculate the optimal color transformation that should be applied to the test
// video to match as closely as possible to the reference video.
ColorTransformationMatrix CalculateColorTransformationMatrix(
    const scoped_refptr<Video>& reference_video,
    const scoped_refptr<Video>& test_video);

// Calculate color transformation for a single I420 frame.
ColorTransformationMatrix CalculateColorTransformationMatrix(
    const scoped_refptr<I420BufferInterface>& reference_frame,
    const scoped_refptr<I420BufferInterface>& test_frame);

// Apply a color transformation to a video.
scoped_refptr<Video> AdjustColors(const ColorTransformationMatrix& color_matrix,
                                  const scoped_refptr<Video>& video);

// Apply a color transformation to a single I420 frame.
scoped_refptr<I420BufferInterface> AdjustColors(
    const ColorTransformationMatrix& color_matrix,
    const scoped_refptr<I420BufferInterface>& frame);

}  // namespace test
}  // namespace webrtc

#endif  // RTC_TOOLS_FRAME_ANALYZER_VIDEO_COLOR_ALIGNER_H_
