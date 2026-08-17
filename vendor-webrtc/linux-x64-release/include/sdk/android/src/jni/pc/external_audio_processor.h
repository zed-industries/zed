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

#ifndef SDK_ANDROID_SRC_JNI_PC_EXTERNAL_AUDIO_PROCESSOR_H_
#define SDK_ANDROID_SRC_JNI_PC_EXTERNAL_AUDIO_PROCESSOR_H_

#define WEBRTC_APM_DEBUG_DUMP 0

#include "modules/audio_processing/audio_buffer.h"
#include "modules/audio_processing/audio_processing_impl.h"
#include "modules/audio_processing/include/audio_processing.h"
#include "sdk/android/src/jni/pc/external_audio_processing_interface.h"

namespace webrtc {

class ExternalAudioProcessor : public webrtc::CustomProcessing {
 public:
  ExternalAudioProcessor() = default;
  ~ExternalAudioProcessor() override = default;

  void SetExternalAudioProcessing(
      ExternalAudioProcessingInterface* processor);

  void SetBypassFlag(bool bypass);

 private:
  void Initialize(int sample_rate_hz, int num_channels) override;
  void Process(webrtc::AudioBuffer* audio) override;
  std::string ToString() const override;
  void SetRuntimeSetting(
      webrtc::AudioProcessing::RuntimeSetting setting) override;

 private:
  mutable webrtc::Mutex mutex_;
  ExternalAudioProcessingInterface* external_processor_;
  bool bypass_flag_ = false;
  bool initialized_ = false;
  int sample_rate_hz_ = 0;
  int num_channels_ = 0;
};

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_EXTERNAL_AUDIO_PROCESSOR_H_
