/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AAUDIO_RECORDER_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AAUDIO_RECORDER_H_

#include <aaudio/AAudio.h>

#include <memory>

#include "api/audio/audio_device_defines.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "modules/audio_device/audio_device_buffer.h"
#include "sdk/android/src/jni/audio_device/aaudio_wrapper.h"
#include "sdk/android/src/jni/audio_device/audio_device_module.h"

namespace webrtc {

class FineAudioBuffer;
class AudioDeviceBuffer;

namespace jni {

// Implements low-latency 16-bit mono PCM audio input support for Android
// using the C based AAudio API.
//
// An instance must be created and destroyed on one and the same thread.
// All public methods must also be called on the same thread. A thread checker
// will RTC_DCHECK if any method is called on an invalid thread. Audio buffers
// are delivered on a dedicated high-priority thread owned by AAudio.
//
// The existing design forces the user to call InitRecording() after
// StopRecording() to be able to call StartRecording() again. This is in line
// with how the Java- based implementation works.
//
// TODO(henrika): add comments about device changes and adaptive buffer
// management.
class AAudioRecorder : public AudioInput, public AAudioObserverInterface {
 public:
  explicit AAudioRecorder(const AudioParameters& audio_parameters);
  ~AAudioRecorder() override;

  int Init() override;
  int Terminate() override;

  int InitRecording() override;
  bool RecordingIsInitialized() const override;

  int StartRecording() override;
  int StopRecording() override;
  bool Recording() const override;

  void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) override;

  // TODO(henrika): add support using AAudio APIs when available.
  bool IsAcousticEchoCancelerSupported() const override;
  bool IsNoiseSuppressorSupported() const override;
  int EnableBuiltInAEC(bool enable) override;
  int EnableBuiltInNS(bool enable) override;

 protected:
  // AAudioObserverInterface implementation.

  // For an input stream, this function should read `num_frames` of recorded
  // data, in the stream's current data format, from the `audio_data` buffer.
  // Called on a real-time thread owned by AAudio.
  aaudio_data_callback_result_t OnDataCallback(void* audio_data,
                                               int32_t num_frames) override;

  // AAudio calls this function if any error occurs on a callback thread.
  // Called on a real-time thread owned by AAudio.
  void OnErrorCallback(aaudio_result_t error) override;

 private:
  // Closes the existing stream and starts a new stream.
  void HandleStreamDisconnected();

  // Ensures that methods are called from the same thread as this object is
  // created on.
  SequenceChecker thread_checker_;

  // Stores thread ID in first call to AAudioPlayer::OnDataCallback from a
  // real-time thread owned by AAudio. Detached during construction of this
  // object.
  SequenceChecker thread_checker_aaudio_;

  // The thread on which this object is created on.
  TaskQueueBase* main_thread_;

  // Wraps all AAudio resources. Contains an input stream using the default
  // input audio device.
  AAudioWrapper aaudio_;

  // Raw pointer handle provided to us in AttachAudioBuffer(). Owned by the
  // AudioDeviceModuleImpl class and called by AudioDeviceModule::Create().
  AudioDeviceBuffer* audio_device_buffer_ = nullptr;

  bool initialized_ = false;
  bool recording_ = false;

  // Consumes audio of native buffer size and feeds the WebRTC layer with 10ms
  // chunks of audio.
  std::unique_ptr<FineAudioBuffer> fine_audio_buffer_;

  // Counts number of detected overflow events reported by AAudio.
  int32_t overflow_count_ = 0;

  // Estimated time between an audio frame was recorded by the input device and
  // it can read on the input stream.
  double latency_millis_ = 0;

  // True only for the first data callback in each audio session.
  bool first_data_callback_ = true;
};

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AAUDIO_RECORDER_H_
