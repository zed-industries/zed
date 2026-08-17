/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_OBJC_AUDIO_DEVICE_H_
#define SDK_OBJC_NATIVE_SRC_OBJC_AUDIO_DEVICE_H_

#include <memory>

#import "components/audio/RTCAudioDevice.h"

#include "api/audio/audio_device.h"
#include "modules/audio_device/audio_device_buffer.h"
#include "rtc_base/thread.h"

@class RTC_OBJC_TYPE(RTCObjCAudioDeviceDelegate);

namespace webrtc {

class FineAudioBuffer;

namespace objc_adm {

class ObjCAudioDeviceModule : public AudioDeviceModule {
 public:
  explicit ObjCAudioDeviceModule(
      id<RTC_OBJC_TYPE(RTCAudioDevice)> audio_device);
  ~ObjCAudioDeviceModule() override;

  // Retrieve the currently utilized audio layer
  int32_t ActiveAudioLayer(AudioLayer* audioLayer) const override;

  // Full-duplex transportation of PCM audio
  int32_t RegisterAudioCallback(AudioTransport* audioCallback) override;

  // Main initialization and termination
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

  // Playout delay
  int32_t PlayoutDelay(uint16_t* delayMS) const override;

  // Only supported on Android.
  bool BuiltInAECIsAvailable() const override;
  bool BuiltInAGCIsAvailable() const override;
  bool BuiltInNSIsAvailable() const override;

  // Enables the built-in audio effects. Only supported on Android.
  int32_t EnableBuiltInAEC(bool enable) override;
  int32_t EnableBuiltInAGC(bool enable) override;
  int32_t EnableBuiltInNS(bool enable) override;

  // Play underrun count. Only supported on Android.
  int32_t GetPlayoutUnderrunCount() const override;

#if defined(WEBRTC_IOS)
  int GetPlayoutAudioParameters(AudioParameters* params) const override;
  int GetRecordAudioParameters(AudioParameters* params) const override;
#endif  // WEBRTC_IOS

 public:
  OSStatus OnDeliverRecordedData(
      AudioUnitRenderActionFlags* flags,
      const AudioTimeStamp* time_stamp,
      NSInteger bus_number,
      UInt32 num_frames,
      const AudioBufferList* io_data,
      void* render_context,
      RTC_OBJC_TYPE(RTCAudioDeviceRenderRecordedDataBlock) render_block);

  OSStatus OnGetPlayoutData(AudioUnitRenderActionFlags* flags,
                            const AudioTimeStamp* time_stamp,
                            NSInteger bus_number,
                            UInt32 num_frames,
                            AudioBufferList* io_data);

  // Notifies `ObjCAudioDeviceModule` that at least one of the audio input
  // parameters or audio input latency of `RTCAudioDevice` has changed. It
  // necessary to update `record_parameters_` with current audio parameter of
  // `RTCAudioDevice` via `UpdateAudioParameters` and if parameters are actually
  // change then ADB parameters are updated with `UpdateInputAudioDeviceBuffer`.
  // Audio input latency stored in `cached_recording_delay_ms_` is also updated
  // with current latency of `RTCAudioDevice`.
  void HandleAudioInputParametersChange();

  // Same as `HandleAudioInputParametersChange` but should be called when audio
  // output parameters of `RTCAudioDevice` has changed.
  void HandleAudioOutputParametersChange();

  // Notifies `ObjCAudioDeviceModule` about audio input interruption happen due
  // to any reason so `ObjCAudioDeviceModule` is can prepare to restart of audio
  // IO.
  void HandleAudioInputInterrupted();

  // Same as `ObjCAudioDeviceModule` but should be called when audio output
  // is interrupted.
  void HandleAudioOutputInterrupted();

 private:
  // Update our audio parameters if they are different from current device audio
  // parameters Returns true when our parameters are update, false - otherwise.
  // `ObjCAudioDeviceModule` has audio device buffer (ADB) which has audio
  // parameters of playout & recording. The ADB is configured to work with
  // specific sample rate & channel count. `ObjCAudioDeviceModule` stores audio
  // parameters which were used to configure ADB in the fields
  // `playout_parameters_` and `recording_parameters_`. `RTCAudioDevice`
  // protocol has its own audio parameters exposed as individual properties.
  // `RTCAudioDevice` audio parameters might change when playout/recording is
  // already in progress, for example, when device is switched. `RTCAudioDevice`
  // audio parameters must be kept in sync with ADB audio parameters. This
  // method is invoked when `RTCAudioDevice` reports that it's audio parameters
  // (`device_params`) are changed and it detects if there any difference with
  // our current audio parameters (`params`). Our parameters are updated in case
  // of actual change and method returns true. In case of actual change there is
  // follow-up call to either `UpdateOutputAudioDeviceBuffer` or
  // `UpdateInputAudioDeviceBuffer` to apply updated `playout_parameters_` or
  // `recording_parameters_` to ADB.

  bool UpdateAudioParameters(AudioParameters& params,
                             const AudioParameters& device_params);

  // Update our cached audio latency with device latency. Device latency is
  // reported by `RTCAudioDevice` object. Whenever latency is changed,
  // `RTCAudioDevice` is obliged to notify ADM about the change via
  // `HandleAudioInputParametersChange` or `HandleAudioOutputParametersChange`.
  // Current device IO latency is cached in the atomic field and used from audio
  // IO thread to be reported to audio device buffer. It is highly recommended
  // by Apple not to call any ObjC methods from audio IO thread, that is why
  // implementation relies on caching latency into a field and being notified
  // when latency is changed, which is the case when device is switched.
  void UpdateAudioDelay(std::atomic<int>& delay_ms,
                        const NSTimeInterval device_latency);

  // Uses current `playout_parameters_` to inform the audio device buffer (ADB)
  // about our internal audio parameters.
  void UpdateOutputAudioDeviceBuffer();

  // Uses current `record_parameters_` to inform the audio device buffer (ADB)
  // about our internal audio parameters.
  void UpdateInputAudioDeviceBuffer();

 private:
  id<RTC_OBJC_TYPE(RTCAudioDevice)> audio_device_;

  const std::unique_ptr<TaskQueueFactory> task_queue_factory_;

  // AudioDeviceBuffer is a buffer to consume audio recorded by `RTCAudioDevice`
  // and provide audio to be played via `RTCAudioDevice`.
  // Audio PCMs could have different sample rate and channels count, but
  // expected to be in 16-bit integer interleaved linear PCM format. The current
  // parameters ADB configured to work with is stored in field
  // `playout_parameters_` for playout and `record_parameters_` for recording.
  // These parameters and ADB must kept in sync with `RTCAudioDevice` audio
  // parameters.
  std::unique_ptr<AudioDeviceBuffer> audio_device_buffer_;

  // Set to 1 when recording is active and 0 otherwise.
  std::atomic<bool> recording_ = false;

  // Set to 1 when playout is active and 0 otherwise.
  std::atomic<bool> playing_ = false;

  // Stores cached value of `RTCAudioDevice outputLatency` to be used from
  // audio IO thread. Latency is updated on audio output parameters change.
  std::atomic<int> cached_playout_delay_ms_ = 0;

  // Same as `cached_playout_delay_ms_` but for audio input
  std::atomic<int> cached_recording_delay_ms_ = 0;

  // Thread that is initialized audio device module.
  Thread* thread_;

  // Ensures that methods are called from the same thread as this object is
  // initialized on.
  SequenceChecker thread_checker_;

  // I/O audio thread checker.
  SequenceChecker io_playout_thread_checker_;
  SequenceChecker io_record_thread_checker_;

  bool is_initialized_ RTC_GUARDED_BY(thread_checker_) = false;
  bool is_playout_initialized_ RTC_GUARDED_BY(thread_checker_) = false;
  bool is_recording_initialized_ RTC_GUARDED_BY(thread_checker_) = false;

  // Contains audio parameters (sample rate, #channels, buffer size etc.) for
  // the playout and recording sides.
  AudioParameters playout_parameters_;
  AudioParameters record_parameters_;

  // `FineAudioBuffer` takes an `AudioDeviceBuffer` which delivers audio data
  // in chunks of 10ms. `RTCAudioDevice` might deliver recorded data in
  // chunks which are not 10ms long. `FineAudioBuffer` implements adaptation
  // from undetermined chunk size to 10ms chunks.
  std::unique_ptr<FineAudioBuffer> record_fine_audio_buffer_;

  // Same as `record_fine_audio_buffer_` but for audio output.
  std::unique_ptr<FineAudioBuffer> playout_fine_audio_buffer_;

  // Temporary storage for recorded data.
  webrtc::BufferT<int16_t> record_audio_buffer_;

  // Delegate object provided to RTCAudioDevice during initialization
  RTC_OBJC_TYPE(RTCObjCAudioDeviceDelegate)* audio_device_delegate_;
};

}  // namespace objc_adm

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_OBJC_AUDIO_DEVICE_H_
