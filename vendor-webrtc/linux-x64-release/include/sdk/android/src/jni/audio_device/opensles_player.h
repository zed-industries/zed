/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_OPENSLES_PLAYER_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_OPENSLES_PLAYER_H_

#include <SLES/OpenSLES.h>
#include <SLES/OpenSLES_Android.h>
#include <SLES/OpenSLES_AndroidConfiguration.h>

#include <memory>
#include <optional>

#include "api/audio/audio_device_defines.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "modules/audio_device/audio_device_buffer.h"
#include "modules/audio_device/fine_audio_buffer.h"
#include "sdk/android/src/jni/audio_device/audio_common.h"
#include "sdk/android/src/jni/audio_device/audio_device_module.h"
#include "sdk/android/src/jni/audio_device/opensles_common.h"

namespace webrtc {

class FineAudioBuffer;

namespace jni {

// Implements 16-bit mono PCM audio output support for Android using the
// C based OpenSL ES API. No calls from C/C++ to Java using JNI is done.
//
// An instance can be created on any thread, but must then be used on one and
// the same thread. All public methods must also be called on the same thread. A
// thread checker will RTC_DCHECK if any method is called on an invalid thread.
// Decoded audio buffers are requested on a dedicated internal thread managed by
// the OpenSL ES layer.
//
// The existing design forces the user to call InitPlayout() after Stoplayout()
// to be able to call StartPlayout() again. This is inline with how the Java-
// based implementation works.
//
// OpenSL ES is a native C API which have no Dalvik-related overhead such as
// garbage collection pauses and it supports reduced audio output latency.
// If the device doesn't claim this feature but supports API level 9 (Android
// platform version 2.3) or later, then we can still use the OpenSL ES APIs but
// the output latency may be higher.
class OpenSLESPlayer : public AudioOutput {
 public:
  // Beginning with API level 17 (Android 4.2), a buffer count of 2 or more is
  // required for lower latency. Beginning with API level 18 (Android 4.3), a
  // buffer count of 1 is sufficient for lower latency. In addition, the buffer
  // size and sample rate must be compatible with the device's native output
  // configuration provided via the audio manager at construction.
  // TODO(henrika): perhaps set this value dynamically based on OS version.
  static const int kNumOfOpenSLESBuffers = 2;

  OpenSLESPlayer(const AudioParameters& audio_parameters,
                 webrtc::scoped_refptr<OpenSLEngineManager> engine_manager);
  ~OpenSLESPlayer() override;

  int Init() override;
  int Terminate() override;

  int InitPlayout() override;
  bool PlayoutIsInitialized() const override;

  int StartPlayout() override;
  int StopPlayout() override;
  bool Playing() const override;

  bool SpeakerVolumeIsAvailable() override;
  int SetSpeakerVolume(uint32_t volume) override;
  std::optional<uint32_t> SpeakerVolume() const override;
  std::optional<uint32_t> MaxSpeakerVolume() const override;
  std::optional<uint32_t> MinSpeakerVolume() const override;

  void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) override;

  int GetPlayoutUnderrunCount() override { return -1; }

 private:
  // These callback methods are called when data is required for playout.
  // They are both called from an internal "OpenSL ES thread" which is not
  // attached to the Dalvik VM.
  static void SimpleBufferQueueCallback(SLAndroidSimpleBufferQueueItf caller,
                                        void* context);
  void FillBufferQueue();
  // Reads audio data in PCM format using the AudioDeviceBuffer.
  // Can be called both on the main thread (during Start()) and from the
  // internal audio thread while output streaming is active.
  // If the `silence` flag is set, the audio is filled with zeros instead of
  // asking the WebRTC layer for real audio data. This procedure is also known
  // as audio priming.
  void EnqueuePlayoutData(bool silence);

  // Allocate memory for audio buffers which will be used to render audio
  // via the SLAndroidSimpleBufferQueueItf interface.
  void AllocateDataBuffers();

  // Obtaines the SL Engine Interface from the existing global Engine object.
  // The interface exposes creation methods of all the OpenSL ES object types.
  // This method defines the `engine_` member variable.
  bool ObtainEngineInterface();

  // Creates/destroys the output mix object.
  bool CreateMix();
  void DestroyMix();

  // Creates/destroys the audio player and the simple-buffer object.
  // Also creates the volume object.
  bool CreateAudioPlayer();
  void DestroyAudioPlayer();

  SLuint32 GetPlayState() const;

  // Ensures that methods are called from the same thread as this object is
  // created on.
  SequenceChecker thread_checker_;

  // Stores thread ID in first call to SimpleBufferQueueCallback() from internal
  // non-application thread which is not attached to the Dalvik JVM.
  // Detached during construction of this object.
  SequenceChecker thread_checker_opensles_;

  const AudioParameters audio_parameters_;

  // Raw pointer handle provided to us in AttachAudioBuffer(). Owned by the
  // AudioDeviceModuleImpl class and called by AudioDeviceModule::Create().
  AudioDeviceBuffer* audio_device_buffer_;

  bool initialized_;
  bool playing_;

  // PCM-type format definition.
  // TODO(henrika): add support for SLAndroidDataFormat_PCM_EX (android-21) if
  // 32-bit float representation is needed.
  SLDataFormat_PCM pcm_format_;

  // Queue of audio buffers to be used by the player object for rendering
  // audio.
  std::unique_ptr<SLint16[]> audio_buffers_[kNumOfOpenSLESBuffers];

  // FineAudioBuffer takes an AudioDeviceBuffer which delivers audio data
  // in chunks of 10ms. It then allows for this data to be pulled in
  // a finer or coarser granularity. I.e. interacting with this class instead
  // of directly with the AudioDeviceBuffer one can ask for any number of
  // audio data samples.
  // Example: native buffer size can be 192 audio frames at 48kHz sample rate.
  // WebRTC will provide 480 audio frames per 10ms but OpenSL ES asks for 192
  // in each callback (one every 4th ms). This class can then ask for 192 and
  // the FineAudioBuffer will ask WebRTC for new data approximately only every
  // second callback and also cache non-utilized audio.
  std::unique_ptr<FineAudioBuffer> fine_audio_buffer_;

  // Keeps track of active audio buffer 'n' in the audio_buffers_[n] queue.
  // Example (kNumOfOpenSLESBuffers = 2): counts 0, 1, 0, 1, ...
  int buffer_index_;

  const webrtc::scoped_refptr<OpenSLEngineManager> engine_manager_;
  // This interface exposes creation methods for all the OpenSL ES object types.
  // It is the OpenSL ES API entry point.
  SLEngineItf engine_;

  // Output mix object to be used by the player object.
  ScopedSLObjectItf output_mix_;

  // The audio player media object plays out audio to the speakers. It also
  // supports volume control.
  ScopedSLObjectItf player_object_;

  // This interface is supported on the audio player and it controls the state
  // of the audio player.
  SLPlayItf player_;

  // The Android Simple Buffer Queue interface is supported on the audio player
  // and it provides methods to send audio data from the source to the audio
  // player for rendering.
  SLAndroidSimpleBufferQueueItf simple_buffer_queue_;

  // This interface exposes controls for manipulating the object’s audio volume
  // properties. This interface is supported on the Audio Player object.
  SLVolumeItf volume_;

  // Last time the OpenSL ES layer asked for audio data to play out.
  uint32_t last_play_time_;
};

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_OPENSLES_PLAYER_H_
