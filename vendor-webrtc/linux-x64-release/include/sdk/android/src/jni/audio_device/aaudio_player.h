/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AAUDIO_PLAYER_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AAUDIO_PLAYER_H_

#include <aaudio/AAudio.h>

#include <memory>
#include <optional>

#include "api/audio/audio_device_defines.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "modules/audio_device/audio_device_buffer.h"
#include "rtc_base/thread_annotations.h"
#include "sdk/android/src/jni/audio_device/aaudio_wrapper.h"
#include "sdk/android/src/jni/audio_device/audio_device_module.h"

namespace webrtc {

class AudioDeviceBuffer;
class FineAudioBuffer;

namespace jni {

// Implements low-latency 16-bit mono PCM audio output support for Android
// using the C based AAudio API.
//
// An instance must be created and destroyed on one and the same thread.
// All public methods must also be called on the same thread. A thread checker
// will DCHECK if any method is called on an invalid thread. Audio buffers
// are requested on a dedicated high-priority thread owned by AAudio.
//
// The existing design forces the user to call InitPlayout() after StopPlayout()
// to be able to call StartPlayout() again. This is in line with how the Java-
// based implementation works.
//
// An audio stream can be disconnected, e.g. when an audio device is removed.
// This implementation will restart the audio stream using the new preferred
// device if such an event happens.
//
// Also supports automatic buffer-size adjustment based on underrun detections
// where the internal AAudio buffer can be increased when needed. It will
// reduce the risk of underruns (~glitches) at the expense of an increased
// latency.
class AAudioPlayer final : public AudioOutput, public AAudioObserverInterface {
 public:
  explicit AAudioPlayer(const AudioParameters& audio_parameters);
  ~AAudioPlayer() override;

  int Init() override;
  int Terminate() override;

  int InitPlayout() override;
  bool PlayoutIsInitialized() const override;

  int StartPlayout() override;
  int StopPlayout() override;
  bool Playing() const override;

  void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) override;

  // Not implemented in AAudio.
  bool SpeakerVolumeIsAvailable() override;
  int SetSpeakerVolume(uint32_t volume) override;
  std::optional<uint32_t> SpeakerVolume() const override;
  std::optional<uint32_t> MaxSpeakerVolume() const override;
  std::optional<uint32_t> MinSpeakerVolume() const override;

 protected:
  // AAudioObserverInterface implementation.

  // For an output stream, this function should render and write `num_frames`
  // of data in the streams current data format to the `audio_data` buffer.
  // Called on a real-time thread owned by AAudio.
  aaudio_data_callback_result_t OnDataCallback(void* audio_data,
                                               int32_t num_frames) override;
  // AAudio calls this functions if any error occurs on a callback thread.
  // Called on a real-time thread owned by AAudio.
  void OnErrorCallback(aaudio_result_t error) override;

 private:
  // TODO(henrika): Implement.
  int GetPlayoutUnderrunCount() override { return 0; }

  // Closes the existing stream and starts a new stream.
  void HandleStreamDisconnected();

  // Ensures that methods are called from the same thread as this object is
  // created on.
  SequenceChecker main_thread_checker_;

  // Stores thread ID in first call to AAudioPlayer::OnDataCallback from a
  // real-time thread owned by AAudio. Detached during construction of this
  // object.
  SequenceChecker thread_checker_aaudio_;

  // The thread on which this object is created on.
  TaskQueueBase* main_thread_;

  // Wraps all AAudio resources. Contains an output stream using the default
  // output audio device. Can be accessed on both the main thread and the
  // real-time thread owned by AAudio. See separate AAudio documentation about
  // thread safety.
  AAudioWrapper aaudio_;

  // FineAudioBuffer takes an AudioDeviceBuffer which delivers audio data
  // in chunks of 10ms. It then allows for this data to be pulled in
  // a finer or coarser granularity. I.e. interacting with this class instead
  // of directly with the AudioDeviceBuffer one can ask for any number of
  // audio data samples.
  // Example: native buffer size can be 192 audio frames at 48kHz sample rate.
  // WebRTC will provide 480 audio frames per 10ms but AAudio asks for 192
  // in each callback (once every 4th ms). This class can then ask for 192 and
  // the FineAudioBuffer will ask WebRTC for new data approximately only every
  // second callback and also cache non-utilized audio.
  std::unique_ptr<FineAudioBuffer> fine_audio_buffer_;

  // Counts number of detected underrun events reported by AAudio.
  int32_t underrun_count_ = 0;

  // True only for the first data callback in each audio session.
  bool first_data_callback_ = true;

  // Raw pointer handle provided to us in AttachAudioBuffer(). Owned by the
  // AudioDeviceModuleImpl class and set by AudioDeviceModule::Create().
  AudioDeviceBuffer* audio_device_buffer_ RTC_GUARDED_BY(main_thread_checker_) =
      nullptr;

  bool initialized_ RTC_GUARDED_BY(main_thread_checker_) = false;
  bool playing_ RTC_GUARDED_BY(main_thread_checker_) = false;

  // Estimated latency between writing an audio frame to the output stream and
  // the time that same frame is played out on the output audio device.
  double latency_millis_ RTC_GUARDED_BY(thread_checker_aaudio_) = 0;
};

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AAUDIO_PLAYER_H_
