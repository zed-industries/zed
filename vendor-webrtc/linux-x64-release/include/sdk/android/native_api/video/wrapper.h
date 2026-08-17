/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_NATIVE_API_VIDEO_WRAPPER_H_
#define SDK_ANDROID_NATIVE_API_VIDEO_WRAPPER_H_

#include <jni.h>

#include <memory>

#include "api/media_stream_interface.h"
#include "api/video/video_frame.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {

// Creates an instance of webrtc::VideoSinkInterface<VideoFrame> from Java
// VideoSink.
std::unique_ptr<VideoSinkInterface<VideoFrame>> JavaToNativeVideoSink(
    JNIEnv* jni,
    jobject video_sink);

// Creates a Java VideoFrame object from a native VideoFrame. The returned
// object has to be released by calling release.
ScopedJavaLocalRef<jobject> NativeToJavaVideoFrame(JNIEnv* jni,
                                                   const VideoFrame& frame);

}  // namespace webrtc

#endif  // SDK_ANDROID_NATIVE_API_VIDEO_WRAPPER_H_
