/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_NATIVE_API_VIDEO_VIDEO_SOURCE_H_
#define SDK_ANDROID_NATIVE_API_VIDEO_VIDEO_SOURCE_H_

#include <jni.h>

#include "api/media_stream_interface.h"
#include "rtc_base/thread.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {

// Interface for class that implements VideoTrackSourceInterface and provides a
// Java object that can be used to feed frames to the source.
class JavaVideoTrackSourceInterface : public VideoTrackSourceInterface {
 public:
  // Returns CapturerObserver object that can be used to feed frames to the
  // video source.
  virtual ScopedJavaLocalRef<jobject> GetJavaVideoCapturerObserver(
      JNIEnv* env) = 0;
};

// Creates an instance of JavaVideoTrackSourceInterface,
scoped_refptr<JavaVideoTrackSourceInterface> CreateJavaVideoSource(
    JNIEnv* env,
    Thread* signaling_thread,
    bool is_screencast,
    bool align_timestamps);

}  // namespace webrtc

#endif  // SDK_ANDROID_NATIVE_API_VIDEO_VIDEO_SOURCE_H_
