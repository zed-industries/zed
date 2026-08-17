/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_SIMULCAST_DUMMY_BUFFER_HELPER_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_SIMULCAST_DUMMY_BUFFER_HELPER_H_

#include "api/video/video_frame.h"
#include "api/video/video_frame_buffer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Creates a special video frame buffer that should be used to create frames
// during Selective Forwarding Unit (SFU) emulation. Such frames are used when
// original was discarded and some frame is required to be passed upstream
// to make WebRTC pipeline happy and not request key frame on the received
// stream due to lack of incoming frames.
scoped_refptr<webrtc::VideoFrameBuffer> CreateDummyFrameBuffer();

// Tests if provided frame contains a buffer created by
// `CreateDummyFrameBuffer`.
bool IsDummyFrame(const webrtc::VideoFrame& video_frame);

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_SIMULCAST_DUMMY_BUFFER_HELPER_H_
