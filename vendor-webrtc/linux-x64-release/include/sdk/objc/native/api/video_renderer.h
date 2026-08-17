/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_API_VIDEO_RENDERER_H_
#define SDK_OBJC_NATIVE_API_VIDEO_RENDERER_H_

#include <memory>

#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#import "base/RTCVideoRenderer.h"

namespace webrtc {

std::unique_ptr<webrtc::VideoSinkInterface<VideoFrame>>
ObjCToNativeVideoRenderer(
    id<RTC_OBJC_TYPE(RTCVideoRenderer)> objc_video_renderer);

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_API_VIDEO_RENDERER_H_
