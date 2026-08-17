/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_FRAME_ANALYZER_VIDEO_GEOMETRY_ALIGNER_H_
#define RTC_TOOLS_FRAME_ANALYZER_VIDEO_GEOMETRY_ALIGNER_H_

#include "api/video/video_frame_buffer.h"
#include "rtc_tools/video_file_reader.h"

namespace webrtc {
namespace test {

struct CropRegion {
  // Each value represents how much to crop from each side. Left is where x=0,
  // and top is where y=0. All values equal to zero represents no cropping.
  int left = 0;
  int right = 0;
  int top = 0;
  int bottom = 0;
};

// Crops and zooms in on the cropped region so that the returned frame has the
// same resolution as the input frame.
scoped_refptr<I420BufferInterface> CropAndZoom(
    const CropRegion& crop_region,
    const scoped_refptr<I420BufferInterface>& frame);

// Calculate the optimal cropping region on the reference frame to maximize SSIM
// to the test frame.
CropRegion CalculateCropRegion(
    const scoped_refptr<I420BufferInterface>& reference_frame,
    const scoped_refptr<I420BufferInterface>& test_frame);

// Returns a cropped and zoomed version of the reference frame that matches up
// to the test frame. This is a simple helper function on top of
// CalculateCropRegion() and CropAndZoom().
scoped_refptr<I420BufferInterface> AdjustCropping(
    const scoped_refptr<I420BufferInterface>& reference_frame,
    const scoped_refptr<I420BufferInterface>& test_frame);

// Returns a cropped and zoomed version of the reference video that matches up
// to the test video. Frames are individually adjusted for cropping.
scoped_refptr<Video> AdjustCropping(const scoped_refptr<Video>& reference_video,
                                    const scoped_refptr<Video>& test_video);

}  // namespace test
}  // namespace webrtc

#endif  // RTC_TOOLS_FRAME_ANALYZER_VIDEO_GEOMETRY_ALIGNER_H_
