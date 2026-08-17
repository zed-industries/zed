/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_MEDIA_STREAM_H_
#define SDK_ANDROID_SRC_JNI_PC_MEDIA_STREAM_H_

#include <jni.h>

#include <memory>

#include "api/media_stream_interface.h"
#include "pc/media_stream_observer.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

class JavaMediaStream {
 public:
  explicit JavaMediaStream(JNIEnv* env,
                           scoped_refptr<MediaStreamInterface> media_stream);
  ~JavaMediaStream();

  const ScopedJavaGlobalRef<jobject>& j_media_stream() {
    return j_media_stream_;
  }

 private:
  void OnAudioTrackAddedToStream(AudioTrackInterface* track,
                                 MediaStreamInterface* stream);
  void OnVideoTrackAddedToStream(VideoTrackInterface* track,
                                 MediaStreamInterface* stream);
  void OnAudioTrackRemovedFromStream(AudioTrackInterface* track,
                                     MediaStreamInterface* stream);
  void OnVideoTrackRemovedFromStream(VideoTrackInterface* track,
                                     MediaStreamInterface* stream);

  ScopedJavaGlobalRef<jobject> j_media_stream_;
  std::unique_ptr<MediaStreamObserver> observer_;
};

jclass GetMediaStreamClass(JNIEnv* env);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_MEDIA_STREAM_H_
