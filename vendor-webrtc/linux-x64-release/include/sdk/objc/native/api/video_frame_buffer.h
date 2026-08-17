/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_API_VIDEO_FRAME_BUFFER_H_
#define SDK_OBJC_NATIVE_API_VIDEO_FRAME_BUFFER_H_

// import
#import "base/RTCVideoFrameBuffer.h"

// include
#include "api/scoped_refptr.h"
#include "common_video/include/video_frame_buffer.h"

namespace webrtc {

webrtc::scoped_refptr<VideoFrameBuffer> ObjCToNativeVideoFrameBuffer(
    id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)> objc_video_frame_buffer);

id<RTC_OBJC_TYPE(RTCVideoFrameBuffer)> NativeToObjCVideoFrameBuffer(
    const webrtc::scoped_refptr<VideoFrameBuffer>& buffer);

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_API_VIDEO_FRAME_BUFFER_H_
