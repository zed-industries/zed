/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_TEST_AUDIO_DEVICE_IMPL_H_
#define MODULES_AUDIO_DEVICE_TEST_AUDIO_DEVICE_IMPL_H_

#include <memory>
#include <vector>

#include "api/audio/audio_device.h"
#include "api/audio/audio_device_defines.h"
#include "api/task_queue/task_queue_base.h"
#include "api/task_queue/task_queue_factory.h"
#include "modules/audio_device/audio_device_buffer.h"
#include "modules/audio_device/audio_device_generic.h"
#include "modules/audio_device/include/test_audio_device.h"
#include "rtc_base/buffer.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

class TestAudioDevice : public AudioDeviceGeneric {
 public:
  // Creates a new TestAudioDevice. When capturing or playing, 10 ms audio
  // frames will be processed every 10ms / `speed`.
  // `capturer` is an object that produces audio data. Can be nullptr if this
  // device is never used for recording.
  // `renderer` is an object that receives audio data that would have been
  // played out. Can be nullptr if this device is never used for playing.
  TestAudioDevice(TaskQueueFactory* task_queue_factory,
                  std::unique_ptr<TestAudioDeviceModule::Capturer> capturer,
                  std::unique_ptr<TestAudioDeviceModule::Renderer> renderer,
                  float speed = 1);
  TestAudioDevice(const TestAudioDevice&) = delete;
  TestAudioDevice& operator=(const TestAudioDevice&) = delete;
  ~TestAudioDevice() override = default;

  // Retrieve the currently utilized audio layer
  int32_t ActiveAudioLayer(
      AudioDeviceModule::AudioLayer& /* audioLayer */) const override {
    return 0;
  }

  // Main initializaton and termination
  InitStatus Init() override;
  int32_t Terminate() override { return 0; }
  bool Initialized() const override { return true; }

  // Device enumeration
  int16_t PlayoutDevices() override { return 0; }
  int16_t RecordingDevices() override { return 0; }
  int32_t PlayoutDeviceName(uint16_t /* index */,
                            char /* name */[kAdmMaxDeviceNameSize],
                            char /* guid */[kAdmMaxGuidSize]) override {
    return 0;
  }
  int32_t RecordingDeviceName(uint16_t /* index */,
                              char /* name */[kAdmMaxDeviceNameSize],
                              char /* guid */[kAdmMaxGuidSize]) override {
    return 0;
  }

  // Device selection
  int32_t SetPlayoutDevice(uint16_t /* index */) override { return 0; }
  int32_t SetPlayoutDevice(
      AudioDeviceModule::WindowsDeviceType /* device */) override {
    return 0;
  }
  int32_t SetRecordingDevice(uint16_t /* index */) override { return 0; }
  int32_t SetRecordingDevice(
      AudioDeviceModule::WindowsDeviceType /* device */) override {
    return 0;
  }

  // Audio transport initialization
  int32_t PlayoutIsAvailable(bool& available) override;
  int32_t InitPlayout() override;
  bool PlayoutIsInitialized() const override;
  int32_t RecordingIsAvailable(bool& available) override;
  int32_t InitRecording() override;
  bool RecordingIsInitialized() const override;

  // Audio transport control
  int32_t StartPlayout() override;
  int32_t StopPlayout() override;
  bool Playing() const override;
  int32_t StartRecording() override;
  int32_t StopRecording() override;
  bool Recording() const override;

  // Audio mixer initialization
  int32_t InitSpeaker() override { return 0; }
  bool SpeakerIsInitialized() const override { return true; }
  int32_t InitMicrophone() override { return 0; }
  bool MicrophoneIsInitialized() const override { return true; }

  // Speaker volume controls
  int32_t SpeakerVolumeIsAvailable(bool& /* available */) override { return 0; }
  int32_t SetSpeakerVolume(uint32_t /* volume */) override { return 0; }
  int32_t SpeakerVolume(uint32_t& /* volume */) const override { return 0; }
  int32_t MaxSpeakerVolume(uint32_t& /* maxVolume */) const override {
    return 0;
  }
  int32_t MinSpeakerVolume(uint32_t& /* minVolume */) const override {
    return 0;
  }

  // Microphone volume controls
  int32_t MicrophoneVolumeIsAvailable(bool& /* available */) override {
    return 0;
  }
  int32_t SetMicrophoneVolume(uint32_t /* volume */) override { return 0; }
  int32_t MicrophoneVolume(uint32_t& /* volume */) const override { return 0; }
  int32_t MaxMicrophoneVolume(uint32_t& /* maxVolume */) const override {
    return 0;
  }
  int32_t MinMicrophoneVolume(uint32_t& /* minVolume */) const override {
    return 0;
  }

  // Speaker mute control
  int32_t SpeakerMuteIsAvailable(bool& /* available */) override { return 0; }
  int32_t SetSpeakerMute(bool /* enable */) override { return 0; }
  int32_t SpeakerMute(bool& /* enabled */) const override { return 0; }

  // Microphone mute control
  int32_t MicrophoneMuteIsAvailable(bool& /* available */) override {
    return 0;
  }
  int32_t SetMicrophoneMute(bool /* enable */) override { return 0; }
  int32_t MicrophoneMute(bool& /* enabled */) const override { return 0; }

  // Stereo support
  int32_t StereoPlayoutIsAvailable(bool& available) override {
    available = false;
    return 0;
  }
  int32_t SetStereoPlayout(bool /* enable */) override { return 0; }
  int32_t StereoPlayout(bool& /* enabled */) const override { return 0; }
  int32_t StereoRecordingIsAvailable(bool& available) override {
    available = false;
    return 0;
  }
  int32_t SetStereoRecording(bool /* enable */) override { return 0; }
  int32_t StereoRecording(bool& /* enabled */) const override { return 0; }

  // Delay information and control
  int32_t PlayoutDelay(uint16_t& delayMS) const override {
    delayMS = 0;
    return 0;
  }

  // Android only
  bool BuiltInAECIsAvailable() const override { return false; }
  bool BuiltInAGCIsAvailable() const override { return false; }
  bool BuiltInNSIsAvailable() const override { return false; }

  // Windows Core Audio and Android only.
  int32_t EnableBuiltInAEC(bool /* enable */) override { return -1; }
  int32_t EnableBuiltInAGC(bool /* enable */) override { return -1; }
  int32_t EnableBuiltInNS(bool /* enable */) override { return -1; }

  // Play underrun count.
  int32_t GetPlayoutUnderrunCount() const override { return -1; }

// iOS only.
// TODO(henrika): add Android support.
#if defined(WEBRTC_IOS)
  int GetPlayoutAudioParameters(AudioParameters* params) const override {
    return -1;
  }
  int GetRecordAudioParameters(AudioParameters* params) const override {
    return -1;
  }
#endif  // WEBRTC_IOS

  void AttachAudioBuffer(AudioDeviceBuffer* audio_buffer) override;

 private:
  void ProcessAudio();

  TaskQueueFactory* const task_queue_factory_;
  const std::unique_ptr<TestAudioDeviceModule::Capturer> capturer_
      RTC_GUARDED_BY(lock_);
  const std::unique_ptr<TestAudioDeviceModule::Renderer> renderer_
      RTC_GUARDED_BY(lock_);
  const int64_t process_interval_us_;

  mutable Mutex lock_;
  AudioDeviceBuffer* audio_buffer_ RTC_GUARDED_BY(lock_) = nullptr;
  bool rendering_ RTC_GUARDED_BY(lock_) = false;
  bool capturing_ RTC_GUARDED_BY(lock_) = false;
  bool rendering_initialized_ RTC_GUARDED_BY(lock_) = false;
  bool capturing_initialized_ RTC_GUARDED_BY(lock_) = false;

  std::vector<int16_t> playout_buffer_ RTC_GUARDED_BY(lock_);
  BufferT<int16_t> recording_buffer_ RTC_GUARDED_BY(lock_);
  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> task_queue_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_TEST_AUDIO_DEVICE_IMPL_H_
