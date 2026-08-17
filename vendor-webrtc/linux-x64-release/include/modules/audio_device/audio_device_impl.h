/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_AUDIO_DEVICE_IMPL_H_
#define MODULES_AUDIO_DEVICE_AUDIO_DEVICE_IMPL_H_

#if defined(WEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE)

#include <stdint.h>

#include <memory>

#include "api/audio/audio_device.h"
#include "api/task_queue/task_queue_factory.h"
#include "modules/audio_device/audio_device_buffer.h"

namespace webrtc {

class AudioDeviceGeneric;

class AudioDeviceModuleImpl : public AudioDeviceModuleForTest {
 public:
  enum PlatformType {
    kPlatformNotSupported = 0,
    kPlatformWin32 = 1,
    kPlatformWinCe = 2,
    kPlatformLinux = 3,
    kPlatformMac = 4,
    kPlatformAndroid = 5,
    kPlatformIOS = 6,
    // Fuchsia isn't fully supported, as there is no implementation for
    // AudioDeviceGeneric which will be created for Fuchsia, so
    // `CreatePlatformSpecificObjects()` call will fail unless usable
    // implementation will be provided by the user.
    kPlatformFuchsia = 7,
  };

  int32_t CheckPlatform();
  int32_t CreatePlatformSpecificObjects();
  int32_t AttachAudioBuffer();

  AudioDeviceModuleImpl(AudioLayer audio_layer,
                        TaskQueueFactory* task_queue_factory,
                        bool bypass_voice_processing = false);
  // If `create_detached` is true, created ADM can be used on another thread
  // compared to the one on which it was created. It's useful for testing.
  AudioDeviceModuleImpl(AudioLayer audio_layer,
                        std::unique_ptr<AudioDeviceGeneric> audio_device,
                        TaskQueueFactory* task_queue_factory,
                        bool create_detached);
  ~AudioDeviceModuleImpl() override;

  // Retrieve the currently utilized audio layer
  int32_t ActiveAudioLayer(AudioLayer* audioLayer) const override;

  // Full-duplex transportation of PCM audio
  int32_t RegisterAudioCallback(AudioTransport* audioCallback) override;

  // Main initializaton and termination
  int32_t Init() override;
  int32_t Terminate() override;
  bool Initialized() const override;

  // Device enumeration
  int16_t PlayoutDevices() override;
  int16_t RecordingDevices() override;
  int32_t PlayoutDeviceName(uint16_t index,
                            char name[kAdmMaxDeviceNameSize],
                            char guid[kAdmMaxGuidSize]) override;
  int32_t RecordingDeviceName(uint16_t index,
                              char name[kAdmMaxDeviceNameSize],
                              char guid[kAdmMaxGuidSize]) override;

  // Device selection
  int32_t SetPlayoutDevice(uint16_t index) override;
  int32_t SetPlayoutDevice(WindowsDeviceType device) override;
  int32_t SetRecordingDevice(uint16_t index) override;
  int32_t SetRecordingDevice(WindowsDeviceType device) override;

  // Audio transport initialization
  int32_t PlayoutIsAvailable(bool* available) override;
  int32_t InitPlayout() override;
  bool PlayoutIsInitialized() const override;
  int32_t RecordingIsAvailable(bool* available) override;
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
  int32_t InitSpeaker() override;
  bool SpeakerIsInitialized() const override;
  int32_t InitMicrophone() override;
  bool MicrophoneIsInitialized() const override;

  // Speaker volume controls
  int32_t SpeakerVolumeIsAvailable(bool* available) override;
  int32_t SetSpeakerVolume(uint32_t volume) override;
  int32_t SpeakerVolume(uint32_t* volume) const override;
  int32_t MaxSpeakerVolume(uint32_t* maxVolume) const override;
  int32_t MinSpeakerVolume(uint32_t* minVolume) const override;

  // Microphone volume controls
  int32_t MicrophoneVolumeIsAvailable(bool* available) override;
  int32_t SetMicrophoneVolume(uint32_t volume) override;
  int32_t MicrophoneVolume(uint32_t* volume) const override;
  int32_t MaxMicrophoneVolume(uint32_t* maxVolume) const override;
  int32_t MinMicrophoneVolume(uint32_t* minVolume) const override;

  // Speaker mute control
  int32_t SpeakerMuteIsAvailable(bool* available) override;
  int32_t SetSpeakerMute(bool enable) override;
  int32_t SpeakerMute(bool* enabled) const override;

  // Microphone mute control
  int32_t MicrophoneMuteIsAvailable(bool* available) override;
  int32_t SetMicrophoneMute(bool enable) override;
  int32_t MicrophoneMute(bool* enabled) const override;

  // Stereo support
  int32_t StereoPlayoutIsAvailable(bool* available) const override;
  int32_t SetStereoPlayout(bool enable) override;
  int32_t StereoPlayout(bool* enabled) const override;
  int32_t StereoRecordingIsAvailable(bool* available) const override;
  int32_t SetStereoRecording(bool enable) override;
  int32_t StereoRecording(bool* enabled) const override;

  // Delay information and control
  int32_t PlayoutDelay(uint16_t* delayMS) const override;

  bool BuiltInAECIsAvailable() const override;
  int32_t EnableBuiltInAEC(bool enable) override;
  bool BuiltInAGCIsAvailable() const override;
  int32_t EnableBuiltInAGC(bool enable) override;
  bool BuiltInNSIsAvailable() const override;
  int32_t EnableBuiltInNS(bool enable) override;

  // Play underrun count.
  int32_t GetPlayoutUnderrunCount() const override;

#if defined(WEBRTC_IOS)
  int GetPlayoutAudioParameters(AudioParameters* params) const override;
  int GetRecordAudioParameters(AudioParameters* params) const override;
#endif  // WEBRTC_IOS

  int32_t SetObserver(AudioDeviceObserver* observer) override;
  int32_t GetPlayoutDevice() const override;
  int32_t GetRecordingDevice() const override;

  AudioDeviceBuffer* GetAudioDeviceBuffer() { return &audio_device_buffer_; }

  int RestartPlayoutInternally() override { return -1; }
  int RestartRecordingInternally() override { return -1; }
  int SetPlayoutSampleRate(uint32_t /* sample_rate */) override { return -1; }
  int SetRecordingSampleRate(uint32_t /* sample_rate */) override { return -1; }

 private:
  PlatformType Platform() const;
  AudioLayer PlatformAudioLayer() const;

  AudioLayer audio_layer_;
  PlatformType platform_type_ = kPlatformNotSupported;
  bool initialized_ = false;
#if defined(WEBRTC_IOS)
  bool bypass_voice_processing_;
#endif
  AudioDeviceBuffer audio_device_buffer_;
  std::unique_ptr<AudioDeviceGeneric> audio_device_;
};

}  // namespace webrtc

#endif  // defined(WEBRTC_INCLUDE_INTERNAL_AUDIO_DEVICE)

#endif  // MODULES_AUDIO_DEVICE_AUDIO_DEVICE_IMPL_H_
