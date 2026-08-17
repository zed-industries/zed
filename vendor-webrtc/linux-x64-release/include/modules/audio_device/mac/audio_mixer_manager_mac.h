/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_DEVICE_AUDIO_MIXER_MANAGER_MAC_H_
#define AUDIO_DEVICE_AUDIO_MIXER_MANAGER_MAC_H_

#include <CoreAudio/CoreAudio.h>

#include "api/audio/audio_device.h"
#include "rtc_base/logging.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

class AudioMixerManagerMac {
 public:
  int32_t OpenSpeaker(AudioDeviceID deviceID) RTC_LOCKS_EXCLUDED(mutex_);
  int32_t OpenMicrophone(AudioDeviceID deviceID) RTC_LOCKS_EXCLUDED(mutex_);
  int32_t SetSpeakerVolume(uint32_t volume) RTC_LOCKS_EXCLUDED(mutex_);
  int32_t SpeakerVolume(uint32_t& volume) const;
  int32_t MaxSpeakerVolume(uint32_t& maxVolume) const;
  int32_t MinSpeakerVolume(uint32_t& minVolume) const;
  int32_t SpeakerVolumeIsAvailable(bool& available);
  int32_t SpeakerMuteIsAvailable(bool& available);
  int32_t SetSpeakerMute(bool enable) RTC_LOCKS_EXCLUDED(mutex_);
  int32_t SpeakerMute(bool& enabled) const;
  int32_t StereoPlayoutIsAvailable(bool& available);
  int32_t StereoRecordingIsAvailable(bool& available);
  int32_t MicrophoneMuteIsAvailable(bool& available);
  int32_t SetMicrophoneMute(bool enable) RTC_LOCKS_EXCLUDED(mutex_);
  int32_t MicrophoneMute(bool& enabled) const;
  int32_t MicrophoneVolumeIsAvailable(bool& available);
  int32_t SetMicrophoneVolume(uint32_t volume) RTC_LOCKS_EXCLUDED(mutex_);
  int32_t MicrophoneVolume(uint32_t& volume) const;
  int32_t MaxMicrophoneVolume(uint32_t& maxVolume) const;
  int32_t MinMicrophoneVolume(uint32_t& minVolume) const;
  int32_t Close() RTC_LOCKS_EXCLUDED(mutex_);
  int32_t CloseSpeaker() RTC_LOCKS_EXCLUDED(mutex_);
  int32_t CloseMicrophone() RTC_LOCKS_EXCLUDED(mutex_);
  bool SpeakerIsInitialized() const;
  bool MicrophoneIsInitialized() const;

 public:
  AudioMixerManagerMac();
  ~AudioMixerManagerMac();

 private:
  int32_t CloseSpeakerLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t CloseMicrophoneLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  static void logCAMsg(webrtc::LoggingSeverity sev,
                       const char* msg,
                       const char* err);

 private:
  Mutex mutex_;

  AudioDeviceID _inputDeviceID;
  AudioDeviceID _outputDeviceID;

  uint16_t _noInputChannels;
  uint16_t _noOutputChannels;
};

}  // namespace webrtc

#endif  // AUDIO_MIXER_MAC_H
