/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_RECORD_JNI_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_RECORD_JNI_H_

#include <jni.h>

#include <memory>

#include "api/audio/audio_device_defines.h"
#include "api/sequence_checker.h"
#include "modules/audio_device/audio_device_buffer.h"
#include "sdk/android/src/jni/audio_device/audio_device_module.h"

namespace webrtc {

namespace jni {

// Implements 16-bit mono PCM audio input support for Android using the Java
// AudioRecord interface. Most of the work is done by its Java counterpart in
// WebRtcAudioRecord.java. This class is created and lives on a thread in
// C++-land, but recorded audio buffers are delivered on a high-priority
// thread managed by the Java class.
//
// The Java class makes use of AudioEffect features (mainly AEC) which are
// first available in Jelly Bean. If it is instantiated running against earlier
// SDKs, the AEC provided by the APM in WebRTC must be used and enabled
// separately instead.
//
// An instance can be created on any thread, but must then be used on one and
// the same thread. All public methods must also be called on the same thread. A
// thread checker will RTC_DCHECK if any method is called on an invalid thread.
//
// This class uses AttachCurrentThreadIfNeeded to attach to a Java VM if needed.
// Additional thread checking guarantees that no other (possibly non attached)
// thread is used.
class AudioRecordJni : public AudioInput {
 public:
  static jni_zero::ScopedJavaLocalRef<jobject> CreateJavaWebRtcAudioRecord(
      JNIEnv* env,
      const jni_zero::JavaRef<jobject>& j_context,
      const jni_zero::JavaRef<jobject>& j_audio_manager);

  AudioRecordJni(JNIEnv* env,
                 const AudioParameters& audio_parameters,
                 int total_delay_ms,
                 const jni_zero::JavaRef<jobject>& j_webrtc_audio_record);
  ~AudioRecordJni() override;

  int32_t Init() override;
  int32_t Terminate() override;

  int32_t InitRecording() override;
  bool RecordingIsInitialized() const override;

  int32_t StartRecording() override;
  int32_t StopRecording() override;
  bool Recording() const override;

  void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) override;

  bool IsAcousticEchoCancelerSupported() const override;
  bool IsNoiseSuppressorSupported() const override;

  int32_t EnableBuiltInAEC(bool enable) override;
  int32_t EnableBuiltInNS(bool enable) override;

  // Called from Java side so we can cache the address of the Java-manged
  // `byte_buffer` in `direct_buffer_address_`. The size of the buffer
  // is also stored in `direct_buffer_capacity_in_bytes_`.
  // This method will be called by the WebRtcAudioRecord constructor, i.e.,
  // on the same thread that this object is created on.
  void CacheDirectBufferAddress(
      JNIEnv* env,
      const jni_zero::JavaParamRef<jobject>& j_caller,
      const jni_zero::JavaParamRef<jobject>& byte_buffer);

  // Called periodically by the Java based WebRtcAudioRecord object when
  // recording has started. Each call indicates that there are `length` new
  // bytes recorded in the memory area `direct_buffer_address_` and it is
  // now time to send these to the consumer.
  // This method is called on a high-priority thread from Java. The name of
  // the thread is 'AudioRecordThread'.
  void DataIsRecorded(JNIEnv* env,
                      const jni_zero::JavaParamRef<jobject>& j_caller,
                      int length,
                      int64_t capture_timestamp_ns);

 private:
  // Stores thread ID in constructor.
  SequenceChecker thread_checker_;

  // Stores thread ID in first call to OnDataIsRecorded() from high-priority
  // thread in Java. Detached during construction of this object.
  SequenceChecker thread_checker_java_;

  // Wraps the Java specific parts of the AudioRecordJni class.
  JNIEnv* env_ = nullptr;
  jni_zero::ScopedJavaGlobalRef<jobject> j_audio_record_;

  const AudioParameters audio_parameters_;

  // Delay estimate of the total round-trip delay (input + output).
  // Fixed value set once in AttachAudioBuffer() and it can take one out of two
  // possible values. See audio_common.h for details.
  const int total_delay_ms_;

  // Cached copy of address to direct audio buffer owned by `j_audio_record_`.
  void* direct_buffer_address_;

  // Number of bytes in the direct audio buffer owned by `j_audio_record_`.
  size_t direct_buffer_capacity_in_bytes_;

  // Number audio frames per audio buffer. Each audio frame corresponds to
  // one sample of PCM mono data at 16 bits per sample. Hence, each audio
  // frame contains 2 bytes (given that the Java layer only supports mono).
  // Example: 480 for 48000 Hz or 441 for 44100 Hz.
  size_t frames_per_buffer_;

  bool initialized_;

  bool recording_;

  // Raw pointer handle provided to us in AttachAudioBuffer(). Owned by the
  // AudioDeviceModuleImpl class and called by AudioDeviceModule::Create().
  AudioDeviceBuffer* audio_device_buffer_;
};

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_RECORD_JNI_H_
