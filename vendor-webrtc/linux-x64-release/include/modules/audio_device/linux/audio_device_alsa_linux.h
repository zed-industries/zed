/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_DEVICE_AUDIO_DEVICE_ALSA_LINUX_H_
#define AUDIO_DEVICE_AUDIO_DEVICE_ALSA_LINUX_H_

#include <memory>

#include "modules/audio_device/audio_device_generic.h"
#include "modules/audio_device/linux/audio_mixer_manager_alsa_linux.h"
#include "rtc_base/platform_thread.h"
#include "rtc_base/synchronization/mutex.h"

#if defined(WEBRTC_USE_X11)
#include <X11/Xlib.h>
#endif
#include <alsa/asoundlib.h>
#include <sys/ioctl.h>
#include <sys/soundcard.h>

typedef webrtc::adm_linux_alsa::AlsaSymbolTable WebRTCAlsaSymbolTable;
WebRTCAlsaSymbolTable* GetAlsaSymbolTable();

namespace webrtc {

class AudioDeviceLinuxALSA : public AudioDeviceGeneric {
 public:
  AudioDeviceLinuxALSA();
  virtual ~AudioDeviceLinuxALSA();

  // Retrieve the currently utilized audio layer
  int32_t ActiveAudioLayer(
      AudioDeviceModule::AudioLayer& audioLayer) const override;

  // Main initializaton and termination
  InitStatus Init() RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t Terminate() RTC_LOCKS_EXCLUDED(mutex_) override;
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
  int32_t SetPlayoutDevice(
      AudioDeviceModule::WindowsDeviceType device) override;
  int32_t SetRecordingDevice(uint16_t index) override;
  int32_t SetRecordingDevice(
      AudioDeviceModule::WindowsDeviceType device) override;

  // Audio transport initialization
  int32_t PlayoutIsAvailable(bool& available) override;
  int32_t InitPlayout() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool PlayoutIsInitialized() const override;
  int32_t RecordingIsAvailable(bool& available) override;
  int32_t InitRecording() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool RecordingIsInitialized() const override;

  // Audio transport control
  int32_t StartPlayout() override;
  int32_t StopPlayout() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool Playing() const override;
  int32_t StartRecording() override;
  int32_t StopRecording() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool Recording() const override;

  // Audio mixer initialization
  int32_t InitSpeaker() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool SpeakerIsInitialized() const override;
  int32_t InitMicrophone() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool MicrophoneIsInitialized() const override;

  // Speaker volume controls
  int32_t SpeakerVolumeIsAvailable(bool& available) override;
  int32_t SetSpeakerVolume(uint32_t volume) override;
  int32_t SpeakerVolume(uint32_t& volume) const override;
  int32_t MaxSpeakerVolume(uint32_t& maxVolume) const override;
  int32_t MinSpeakerVolume(uint32_t& minVolume) const override;

  // Microphone volume controls
  int32_t MicrophoneVolumeIsAvailable(bool& available) override;
  int32_t SetMicrophoneVolume(uint32_t volume) override;
  int32_t MicrophoneVolume(uint32_t& volume) const override;
  int32_t MaxMicrophoneVolume(uint32_t& maxVolume) const override;
  int32_t MinMicrophoneVolume(uint32_t& minVolume) const override;

  // Speaker mute control
  int32_t SpeakerMuteIsAvailable(bool& available) override;
  int32_t SetSpeakerMute(bool enable) override;
  int32_t SpeakerMute(bool& enabled) const override;

  // Microphone mute control
  int32_t MicrophoneMuteIsAvailable(bool& available) override;
  int32_t SetMicrophoneMute(bool enable) override;
  int32_t MicrophoneMute(bool& enabled) const override;

  // Stereo support
  int32_t StereoPlayoutIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t SetStereoPlayout(bool enable) override;
  int32_t StereoPlayout(bool& enabled) const override;
  int32_t StereoRecordingIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t SetStereoRecording(bool enable) override;
  int32_t StereoRecording(bool& enabled) const override;

  // Delay information and control
  int32_t PlayoutDelay(uint16_t& delayMS) const override;

  void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer)
      RTC_LOCKS_EXCLUDED(mutex_) override;

 private:
  int32_t InitRecordingLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t StopRecordingLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t StopPlayoutLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t InitPlayoutLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t InitSpeakerLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t InitMicrophoneLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t GetDevicesInfo(int32_t function,
                         bool playback,
                         int32_t enumDeviceNo = 0,
                         char* enumDeviceName = NULL,
                         int32_t ednLen = 0) const;
  int32_t ErrorRecovery(int32_t error, snd_pcm_t* deviceHandle);

  bool KeyPressed() const;

  void Lock() RTC_EXCLUSIVE_LOCK_FUNCTION(mutex_) { mutex_.Lock(); }
  void UnLock() RTC_UNLOCK_FUNCTION(mutex_) { mutex_.Unlock(); }

  inline int32_t InputSanityCheckAfterUnlockedPeriod() const;
  inline int32_t OutputSanityCheckAfterUnlockedPeriod() const;

  static void RecThreadFunc(void*);
  static void PlayThreadFunc(void*);
  bool RecThreadProcess();
  bool PlayThreadProcess();

  AudioDeviceBuffer* _ptrAudioBuffer;

  Mutex mutex_;

  webrtc::PlatformThread _ptrThreadRec;
  webrtc::PlatformThread _ptrThreadPlay;

  AudioMixerManagerLinuxALSA _mixerManager;

  uint16_t _inputDeviceIndex;
  uint16_t _outputDeviceIndex;
  bool _inputDeviceIsSpecified;
  bool _outputDeviceIsSpecified;

  snd_pcm_t* _handleRecord;
  snd_pcm_t* _handlePlayout;

  snd_pcm_uframes_t _recordingBuffersizeInFrame;
  snd_pcm_uframes_t _recordingPeriodSizeInFrame;
  snd_pcm_uframes_t _playoutBufferSizeInFrame;
  snd_pcm_uframes_t _playoutPeriodSizeInFrame;

  ssize_t _recordingBufferSizeIn10MS;
  ssize_t _playoutBufferSizeIn10MS;
  uint32_t _recordingFramesIn10MS;
  uint32_t _playoutFramesIn10MS;

  uint32_t _recordingFreq;
  uint32_t _playoutFreq;
  uint8_t _recChannels;
  uint8_t _playChannels;

  int8_t* _recordingBuffer;  // in byte
  int8_t* _playoutBuffer;    // in byte
  uint32_t _recordingFramesLeft;
  uint32_t _playoutFramesLeft;

  bool _initialized;
  bool _recording;
  bool _playing;
  bool _recIsInitialized;
  bool _playIsInitialized;

  snd_pcm_sframes_t _recordingDelay;
  snd_pcm_sframes_t _playoutDelay;

  char _oldKeyState[32];
#if defined(WEBRTC_USE_X11)
  Display* _XDisplay;
#endif
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_MAIN_SOURCE_LINUX_AUDIO_DEVICE_ALSA_LINUX_H_
