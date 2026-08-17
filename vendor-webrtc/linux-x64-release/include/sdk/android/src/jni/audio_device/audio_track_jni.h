/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_TRACK_JNI_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_TRACK_JNI_H_

#include <jni.h>

#include <memory>
#include <optional>

#include "api/audio/audio_device_defines.h"
#include "api/sequence_checker.h"
#include "modules/audio_device/audio_device_buffer.h"
#include "sdk/android/src/jni/audio_device/audio_common.h"
#include "sdk/android/src/jni/audio_device/audio_device_module.h"

namespace webrtc {

namespace jni {

// Implements 16-bit mono PCM audio output support for Android using the Java
// AudioTrack interface. Most of the work is done by its Java counterpart in
// WebRtcAudioTrack.java. This class is created and lives on a thread in
// C++-land, but decoded audio buffers are requested on a high-priority
// thread managed by the Java class.
//
// An instance can be created on any thread, but must then be used on one and
// the same thread. All public methods must also be called on the same thread. A
// thread checker will RTC_DCHECK if any method is called on an invalid thread
//
// This class uses AttachCurrentThreadIfNeeded to attach to a Java VM if needed.
// Additional thread checking guarantees that no other (possibly non attached)
// thread is used.
class AudioTrackJni : public AudioOutput {
 public:
  static jni_zero::ScopedJavaLocalRef<jobject> CreateJavaWebRtcAudioTrack(
      JNIEnv* env,
      const jni_zero::JavaRef<jobject>& j_context,
      const jni_zero::JavaRef<jobject>& j_audio_manager);

  AudioTrackJni(JNIEnv* env,
                const AudioParameters& audio_parameters,
                const jni_zero::JavaRef<jobject>& j_webrtc_audio_track);
  ~AudioTrackJni() override;

  int32_t Init() override;
  int32_t Terminate() override;

  int32_t InitPlayout() override;
  bool PlayoutIsInitialized() const override;

  int32_t StartPlayout() override;
  int32_t StopPlayout() override;
  bool Playing() const override;

  bool SpeakerVolumeIsAvailable() override;
  int SetSpeakerVolume(uint32_t volume) override;
  std::optional<uint32_t> SpeakerVolume() const override;
  std::optional<uint32_t> MaxSpeakerVolume() const override;
  std::optional<uint32_t> MinSpeakerVolume() const override;
  int GetPlayoutUnderrunCount() override;

  void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) override;

  // Called from Java side so we can cache the address of the Java-manged
  // `byte_buffer` in `direct_buffer_address_`. The size of the buffer
  // is also stored in `direct_buffer_capacity_in_bytes_`.
  // Called on the same thread as the creating thread.
  void CacheDirectBufferAddress(
      JNIEnv* env,
      const jni_zero::JavaParamRef<jobject>& byte_buffer);
  // Called periodically by the Java based WebRtcAudioTrack object when
  // playout has started. Each call indicates that `length` new bytes should
  // be written to the memory area `direct_buffer_address_` for playout.
  // This method is called on a high-priority thread from Java. The name of
  // the thread is 'AudioTrackThread'.
  void GetPlayoutData(JNIEnv* env, size_t length);

 private:
  // Stores thread ID in constructor.
  SequenceChecker thread_checker_;

  // Stores thread ID in first call to OnGetPlayoutData() from high-priority
  // thread in Java. Detached during construction of this object.
  SequenceChecker thread_checker_java_;

  // Wraps the Java specific parts of the AudioTrackJni class.
  JNIEnv* env_ = nullptr;
  jni_zero::ScopedJavaGlobalRef<jobject> j_audio_track_;

  // Contains audio parameters provided to this class at construction by the
  // AudioManager.
  const AudioParameters audio_parameters_;

  // Cached copy of address to direct audio buffer owned by `j_audio_track_`.
  void* direct_buffer_address_;

  // Number of bytes in the direct audio buffer owned by `j_audio_track_`.
  size_t direct_buffer_capacity_in_bytes_;

  // Number of audio frames per audio buffer. Each audio frame corresponds to
  // one sample of PCM mono data at 16 bits per sample. Hence, each audio
  // frame contains 2 bytes (given that the Java layer only supports mono).
  // Example: 480 for 48000 Hz or 441 for 44100 Hz.
  size_t frames_per_buffer_;

  bool initialized_;

  bool playing_;

  // Raw pointer handle provided to us in AttachAudioBuffer(). Owned by the
  // AudioDeviceModuleImpl class and called by AudioDeviceModule::Create().
  // The AudioDeviceBuffer is a member of the AudioDeviceModuleImpl instance
  // and therefore outlives this object.
  AudioDeviceBuffer* audio_device_buffer_;
};

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_TRACK_JNI_H_
