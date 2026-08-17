/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_DEVICE_MODULE_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_DEVICE_MODULE_H_

#include <memory>
#include <optional>

#include "api/audio/audio_device.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {

class AudioDeviceBuffer;

namespace jni {

class AudioInput {
 public:
  virtual ~AudioInput() {}

  virtual int32_t Init() = 0;
  virtual int32_t Terminate() = 0;

  virtual int32_t InitRecording() = 0;
  virtual bool RecordingIsInitialized() const = 0;

  virtual int32_t StartRecording() = 0;
  virtual int32_t StopRecording() = 0;
  virtual bool Recording() const = 0;

  virtual void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) = 0;

  // Returns true if the audio input supports built-in audio effects for AEC and
  // NS.
  virtual bool IsAcousticEchoCancelerSupported() const = 0;
  virtual bool IsNoiseSuppressorSupported() const = 0;

  virtual int32_t EnableBuiltInAEC(bool enable) = 0;
  virtual int32_t EnableBuiltInNS(bool enable) = 0;
};

class AudioOutput {
 public:
  virtual ~AudioOutput() {}

  virtual int32_t Init() = 0;
  virtual int32_t Terminate() = 0;
  virtual int32_t InitPlayout() = 0;
  virtual bool PlayoutIsInitialized() const = 0;
  virtual int32_t StartPlayout() = 0;
  virtual int32_t StopPlayout() = 0;
  virtual bool Playing() const = 0;
  virtual bool SpeakerVolumeIsAvailable() = 0;
  virtual int SetSpeakerVolume(uint32_t volume) = 0;
  virtual std::optional<uint32_t> SpeakerVolume() const = 0;
  virtual std::optional<uint32_t> MaxSpeakerVolume() const = 0;
  virtual std::optional<uint32_t> MinSpeakerVolume() const = 0;
  virtual void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) = 0;
  virtual int GetPlayoutUnderrunCount() = 0;
  virtual std::optional<AudioDeviceModule::Stats> GetStats() const {
    return std::nullopt;
  }
};

// Extract an android.media.AudioManager from an android.content.Context.
ScopedJavaLocalRef<jobject> GetAudioManager(JNIEnv* env,
                                            const JavaRef<jobject>& j_context);

// Get default audio sample rate by querying an android.media.AudioManager.
int GetDefaultSampleRate(JNIEnv* env, const JavaRef<jobject>& j_audio_manager);

// Get audio input and output parameters based on a number of settings.
void GetAudioParameters(JNIEnv* env,
                        const JavaRef<jobject>& j_context,
                        const JavaRef<jobject>& j_audio_manager,
                        int input_sample_rate,
                        int output_sample_rate,
                        bool use_stereo_input,
                        bool use_stereo_output,
                        AudioParameters* input_parameters,
                        AudioParameters* output_parameters);

bool IsLowLatencyInputSupported(JNIEnv* env, const JavaRef<jobject>& j_context);

bool IsLowLatencyOutputSupported(JNIEnv* env,
                                 const JavaRef<jobject>& j_context);

// Glue together an audio input and audio output to get an AudioDeviceModule.
scoped_refptr<AudioDeviceModule> CreateAudioDeviceModuleFromInputAndOutput(
    AudioDeviceModule::AudioLayer audio_layer,
    bool is_stereo_playout_supported,
    bool is_stereo_record_supported,
    uint16_t playout_delay_ms,
    std::unique_ptr<AudioInput> audio_input,
    std::unique_ptr<AudioOutput> audio_output);

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_DEVICE_MODULE_H_
