/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_API_VIDEO_CAPTURER_H_
#define SDK_OBJC_NATIVE_API_VIDEO_CAPTURER_H_

// import
#import "base/RTCVideoCapturer.h"

// include
#include "api/media_stream_interface.h"
#include "api/scoped_refptr.h"
#include "rtc_base/thread.h"

namespace webrtc {

webrtc::scoped_refptr<webrtc::VideoTrackSourceInterface>
ObjCToNativeVideoCapturer(RTC_OBJC_TYPE(RTCVideoCapturer) * objc_video_capturer,
                          webrtc::Thread* signaling_thread,
                          webrtc::Thread* worker_thread);

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_API_VIDEO_CAPTURER_H_
