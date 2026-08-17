/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_VIDEO_SINK_H_
#define SDK_ANDROID_SRC_JNI_VIDEO_SINK_H_

#include <jni.h>

#include "api/media_stream_interface.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

class VideoSinkWrapper : public VideoSinkInterface<VideoFrame> {
 public:
  VideoSinkWrapper(JNIEnv* jni, const JavaRef<jobject>& j_sink);
  ~VideoSinkWrapper() override;

 private:
  void OnFrame(const VideoFrame& frame) override;

  const ScopedJavaGlobalRef<jobject> j_sink_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_VIDEO_SINK_H_
