/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_DEFAULT_H_
#define MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_DEFAULT_H_

#include "api/audio/audio_device.h"

namespace webrtc {
namespace webrtc_impl {

// AudioDeviceModuleDefault template adds default implementation for all
// AudioDeviceModule methods to the class, which inherits from
// AudioDeviceModuleDefault<T>.
template <typename T>
class AudioDeviceModuleDefault : public T {
 public:
  AudioDeviceModuleDefault() {}
  virtual ~AudioDeviceModuleDefault() {}

  int32_t RegisterAudioCallback(AudioTransport* /* audioCallback */) override {
    return 0;
  }
  int32_t Init() override { return 0; }
  int32_t InitSpeaker() override { return 0; }
  int32_t SetPlayoutDevice(uint16_t /* index */) override { return 0; }
  int32_t SetPlayoutDevice(
      AudioDeviceModule::WindowsDeviceType /* device */) override {
    return 0;
  }
  int32_t SetStereoPlayout(bool /* enable */) override { return 0; }
  int32_t StopPlayout() override { return 0; }
  int32_t InitMicrophone() override { return 0; }
  int32_t SetRecordingDevice(uint16_t /* index */) override { return 0; }
  int32_t SetRecordingDevice(
      AudioDeviceModule::WindowsDeviceType /* device */) override {
    return 0;
  }
  int32_t SetStereoRecording(bool /* enable */) override { return 0; }
  int32_t StopRecording() override { return 0; }

  int32_t Terminate() override { return 0; }

  int32_t ActiveAudioLayer(
      AudioDeviceModule::AudioLayer* /* audioLayer */) const override {
    return 0;
  }
  bool Initialized() const override { return true; }
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
  int32_t PlayoutIsAvailable(bool* /* available */) override { return 0; }
  int32_t InitPlayout() override { return 0; }
  bool PlayoutIsInitialized() const override { return true; }
  int32_t RecordingIsAvailable(bool* /* available */) override { return 0; }
  int32_t InitRecording() override { return 0; }
  bool RecordingIsInitialized() const override { return true; }
  int32_t StartPlayout() override { return 0; }
  bool Playing() const override { return false; }
  int32_t StartRecording() override { return 0; }
  bool Recording() const override { return false; }
  bool SpeakerIsInitialized() const override { return true; }
  bool MicrophoneIsInitialized() const override { return true; }
  int32_t SpeakerVolumeIsAvailable(bool* /* available */) override { return 0; }
  int32_t SetSpeakerVolume(uint32_t /* volume */) override { return 0; }
  int32_t SpeakerVolume(uint32_t* /* volume */) const override { return 0; }
  int32_t MaxSpeakerVolume(uint32_t* /* maxVolume */) const override {
    return 0;
  }
  int32_t MinSpeakerVolume(uint32_t* /* minVolume */) const override {
    return 0;
  }
  int32_t MicrophoneVolumeIsAvailable(bool* /* available */) override {
    return 0;
  }
  int32_t SetMicrophoneVolume(uint32_t /* volume */) override { return 0; }
  int32_t MicrophoneVolume(uint32_t* /* volume */) const override { return 0; }
  int32_t MaxMicrophoneVolume(uint32_t* /* maxVolume */) const override {
    return 0;
  }
  int32_t MinMicrophoneVolume(uint32_t* /* minVolume */) const override {
    return 0;
  }
  int32_t SpeakerMuteIsAvailable(bool* /* available */) override { return 0; }
  int32_t SetSpeakerMute(bool /* enable */) override { return 0; }
  int32_t SpeakerMute(bool* /* enabled */) const override { return 0; }
  int32_t MicrophoneMuteIsAvailable(bool* /* available */) override {
    return 0;
  }
  int32_t SetMicrophoneMute(bool /* enable */) override { return 0; }
  int32_t MicrophoneMute(bool* /* enabled */) const override { return 0; }
  int32_t StereoPlayoutIsAvailable(bool* available) const override {
    *available = false;
    return 0;
  }
  int32_t StereoPlayout(bool* /* enabled */) const override { return 0; }
  int32_t StereoRecordingIsAvailable(bool* available) const override {
    *available = false;
    return 0;
  }
  int32_t StereoRecording(bool* /* enabled */) const override { return 0; }
  int32_t PlayoutDelay(uint16_t* delayMS) const override {
    *delayMS = 0;
    return 0;
  }
  bool BuiltInAECIsAvailable() const override { return false; }
  int32_t EnableBuiltInAEC(bool /* enable */) override { return -1; }
  bool BuiltInAGCIsAvailable() const override { return false; }
  int32_t EnableBuiltInAGC(bool /* enable */) override { return -1; }
  bool BuiltInNSIsAvailable() const override { return false; }
  int32_t EnableBuiltInNS(bool /* enable */) override { return -1; }

  int32_t GetPlayoutUnderrunCount() const override { return -1; }

#if defined(WEBRTC_IOS)
  int GetPlayoutAudioParameters(AudioParameters* params) const override {
    return -1;
  }
  int GetRecordAudioParameters(AudioParameters* params) const override {
    return -1;
  }
#endif  // WEBRTC_IOS
};

}  // namespace webrtc_impl
}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_DEFAULT_H_
