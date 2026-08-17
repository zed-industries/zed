/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_VIDEO_FRAME_H_
#define SDK_ANDROID_SRC_JNI_VIDEO_FRAME_H_

#include <jni.h>

#include "api/video/video_frame.h"
#include "api/video/video_frame_buffer.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

scoped_refptr<VideoFrameBuffer> JavaToNativeFrameBuffer(
    JNIEnv* jni,
    const JavaRef<jobject>& j_video_frame_buffer);

VideoFrame JavaToNativeFrame(JNIEnv* jni,
                             const JavaRef<jobject>& j_video_frame,
                             uint32_t timestamp_rtp);

// NOTE: Returns a new video frame that has to be released by calling
// ReleaseJavaVideoFrame.
ScopedJavaLocalRef<jobject> NativeToJavaVideoFrame(JNIEnv* jni,
                                                   const VideoFrame& frame);
void ReleaseJavaVideoFrame(JNIEnv* jni, const JavaRef<jobject>& j_video_frame);

int64_t GetJavaVideoFrameTimestampNs(JNIEnv* jni,
                                     const JavaRef<jobject>& j_video_frame);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_VIDEO_FRAME_H_
