/*
 * Copyright 2022 LiveKit
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <jni.h>

#define WEBRTC_APM_DEBUG_DUMP 0

#include "rtc_base/ref_counted_object.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"
#include "sdk/android/src/jni/pc/external_audio_processor.h"
#include "sdk/android/src/jni/pc/external_audio_processing_interface.h"

namespace webrtc {
namespace jni {

class ExternalAudioProcessingJni
    : public webrtc::ExternalAudioProcessingInterface,
      public webrtc::RefCountInterface {
 public:
  ExternalAudioProcessingJni(JNIEnv* jni, const JavaRef<jobject>& j_processing);
  ~ExternalAudioProcessingJni();

 protected:
  virtual void Initialize(int sample_rate_hz, int num_channels) override;
  virtual void Reset(int new_rate) override;
  virtual void Process(int num_bans, int num_frames, int buffer_size, float* buffer) override;

 private:
  const ScopedJavaGlobalRef<jobject> j_processing_global_;
  const ScopedJavaGlobalRef<jobject> j_processing_;
};

class ExternalAudioProcessingFactory : public webrtc::RefCountInterface {
 public:
  ExternalAudioProcessingFactory();
  virtual ~ExternalAudioProcessingFactory() = default;

  ExternalAudioProcessor* capture_post_processor() {
    return capture_post_processor_;
  }

  ExternalAudioProcessor* render_pre_processor() {
    return render_pre_processor_;
  }

  rtc::scoped_refptr<webrtc::AudioProcessing> apm() { return apm_; }

 private:
  rtc::scoped_refptr<webrtc::AudioProcessing> apm_;
  ExternalAudioProcessor* capture_post_processor_;
  ExternalAudioProcessor* render_pre_processor_;
};

}  // namespace jni
}  // namespace webrtc
