/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_VIDEO_ENCODER_FACTORY_WRAPPER_H_
#define SDK_ANDROID_SRC_JNI_VIDEO_ENCODER_FACTORY_WRAPPER_H_

#include <jni.h>

#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

// Wrapper for Java VideoEncoderFactory class. Delegates method calls through
// JNI and wraps the encoder inside VideoEncoderWrapper.
class VideoEncoderFactoryWrapper : public VideoEncoderFactory {
 public:
  VideoEncoderFactoryWrapper(JNIEnv* jni,
                             const JavaRef<jobject>& encoder_factory);
  ~VideoEncoderFactoryWrapper() override;

  std::unique_ptr<VideoEncoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override;

  // Returns a list of supported codecs in order of preference.
  std::vector<SdpVideoFormat> GetSupportedFormats() const override;

  std::vector<SdpVideoFormat> GetImplementations() const override;

  std::unique_ptr<EncoderSelectorInterface> GetEncoderSelector() const override;

 private:
  const ScopedJavaGlobalRef<jobject> encoder_factory_;
  std::vector<SdpVideoFormat> supported_formats_;
  std::vector<SdpVideoFormat> implementations_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_VIDEO_ENCODER_FACTORY_WRAPPER_H_
