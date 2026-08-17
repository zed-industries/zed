/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_DEVICE_AUDIO_MIXER_MANAGER_PULSE_LINUX_H_
#define AUDIO_DEVICE_AUDIO_MIXER_MANAGER_PULSE_LINUX_H_

#include <pulse/pulseaudio.h>
#include <stdint.h>

#include "api/sequence_checker.h"

#ifndef UINT32_MAX
#define UINT32_MAX ((uint32_t)-1)
#endif

namespace webrtc {

class AudioMixerManagerLinuxPulse {
 public:
  int32_t SetPlayStream(pa_stream* playStream);
  int32_t SetRecStream(pa_stream* recStream);
  int32_t OpenSpeaker(uint16_t deviceIndex);
  int32_t OpenMicrophone(uint16_t deviceIndex);
  int32_t SetSpeakerVolume(uint32_t volume);
  int32_t SpeakerVolume(uint32_t& volume) const;
  int32_t MaxSpeakerVolume(uint32_t& maxVolume) const;
  int32_t MinSpeakerVolume(uint32_t& minVolume) const;
  int32_t SpeakerVolumeIsAvailable(bool& available);
  int32_t SpeakerMuteIsAvailable(bool& available);
  int32_t SetSpeakerMute(bool enable);
  int32_t StereoPlayoutIsAvailable(bool& available);
  int32_t StereoRecordingIsAvailable(bool& available);
  int32_t SpeakerMute(bool& enabled) const;
  int32_t MicrophoneMuteIsAvailable(bool& available);
  int32_t SetMicrophoneMute(bool enable);
  int32_t MicrophoneMute(bool& enabled) const;
  int32_t MicrophoneVolumeIsAvailable(bool& available);
  int32_t SetMicrophoneVolume(uint32_t volume);
  int32_t MicrophoneVolume(uint32_t& volume) const;
  int32_t MaxMicrophoneVolume(uint32_t& maxVolume) const;
  int32_t MinMicrophoneVolume(uint32_t& minVolume) const;
  int32_t SetPulseAudioObjects(pa_threaded_mainloop* mainloop,
                               pa_context* context);
  int32_t Close();
  int32_t CloseSpeaker();
  int32_t CloseMicrophone();
  bool SpeakerIsInitialized() const;
  bool MicrophoneIsInitialized() const;

 public:
  AudioMixerManagerLinuxPulse();
  ~AudioMixerManagerLinuxPulse();

 private:
  static void PaSinkInfoCallback(pa_context* c,
                                 const pa_sink_info* i,
                                 int eol,
                                 void* pThis);
  static void PaSinkInputInfoCallback(pa_context* c,
                                      const pa_sink_input_info* i,
                                      int eol,
                                      void* pThis);
  static void PaSourceInfoCallback(pa_context* c,
                                   const pa_source_info* i,
                                   int eol,
                                   void* pThis);
  static void PaSetVolumeCallback(pa_context* /*c*/,
                                  int success,
                                  void* /*pThis*/);
  void PaSinkInfoCallbackHandler(const pa_sink_info* i, int eol);
  void PaSinkInputInfoCallbackHandler(const pa_sink_input_info* i, int eol);
  void PaSourceInfoCallbackHandler(const pa_source_info* i, int eol);

  void WaitForOperationCompletion(pa_operation* paOperation) const;

  bool GetSinkInputInfo() const;
  bool GetSinkInfoByIndex(int device_index) const;
  bool GetSourceInfoByIndex(int device_index) const;

 private:
  int16_t _paOutputDeviceIndex;
  int16_t _paInputDeviceIndex;

  pa_stream* _paPlayStream;
  pa_stream* _paRecStream;

  pa_threaded_mainloop* _paMainloop;
  pa_context* _paContext;

  mutable uint32_t _paVolume;
  mutable uint32_t _paMute;
  mutable uint32_t _paVolSteps;
  bool _paSpeakerMute;
  mutable uint32_t _paSpeakerVolume;
  mutable uint8_t _paChannels;
  bool _paObjectsSet;

  // Stores thread ID in constructor.
  // We can then use RTC_DCHECK_RUN_ON(&worker_thread_checker_) to ensure that
  // other methods are called from the same thread.
  // Currently only does RTC_DCHECK(thread_checker_.IsCurrent()).
  SequenceChecker thread_checker_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_MAIN_SOURCE_LINUX_AUDIO_MIXER_MANAGER_PULSE_LINUX_H_
