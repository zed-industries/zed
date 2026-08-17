/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_DEVICE_IOS_H_
#define SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_DEVICE_IOS_H_

#include <atomic>
#include <memory>

#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "audio_session_observer.h"
#include "modules/audio_device/audio_device_generic.h"
#include "rtc_base/buffer.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "sdk/objc/base/RTCMacros.h"
#include "voice_processing_audio_unit.h"

RTC_FWD_DECL_OBJC_CLASS(RTC_OBJC_TYPE(RTCNativeAudioSessionDelegateAdapter));

namespace webrtc {

class FineAudioBuffer;

namespace ios_adm {

// A callback handler for audio device rendering errors.
// Note: Called on a realtime thread.
// Note: Only applies to input rendering errors, not output.
typedef void (^AudioDeviceIOSRenderErrorHandler)(OSStatus error);

// Implements full duplex 16-bit mono PCM audio support for iOS using a
// Voice-Processing (VP) I/O audio unit in Core Audio. The VP I/O audio unit
// supports audio echo cancellation. It also adds automatic gain control,
// adjustment of voice-processing quality and muting.
//
// An instance must be created and destroyed on one and the same thread.
// All supported public methods must also be called on the same thread.
// A thread checker will RTC_DCHECK if any supported method is called on an
// invalid thread.
//
// Recorded audio will be delivered on a real-time internal I/O thread in the
// audio unit. The audio unit will also ask for audio data to play out on this
// same thread.
class AudioDeviceIOS : public AudioDeviceGeneric,
                       public AudioSessionObserver,
                       public VoiceProcessingAudioUnitObserver {
 public:
  explicit AudioDeviceIOS(
      bool bypass_voice_processing,
      AudioDeviceModule::MutedSpeechEventHandler muted_speech_event_handler,
      AudioDeviceIOSRenderErrorHandler render_error_handler);
  ~AudioDeviceIOS() override;

  void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer) override;

  InitStatus Init() override;
  int32_t Terminate() override;
  bool Initialized() const override;

  int32_t InitPlayout() override;
  bool PlayoutIsInitialized() const override;

  int32_t InitRecording() override;
  bool RecordingIsInitialized() const override;

  int32_t StartPlayout() override;
  int32_t StopPlayout() override;
  bool Playing() const override;

  int32_t StartRecording() override;
  int32_t StopRecording() override;
  bool Recording() const override;

  // These methods returns hard-coded delay values and not dynamic delay
  // estimates. The reason is that iOS supports a built-in AEC and the WebRTC
  // AEC will always be disabled in the Libjingle layer to avoid running two
  // AEC implementations at the same time. And, it saves resources to avoid
  // updating these delay values continuously.
  // TODO(henrika): it would be possible to mark these two methods as not
  // implemented since they are only called for A/V-sync purposes today and
  // A/V-sync is not supported on iOS. However, we avoid adding error messages
  // the log by using these dummy implementations instead.
  int32_t PlayoutDelay(uint16_t& delayMS) const override;

  // No implementation for playout underrun on iOS. We override it to avoid a
  // periodic log that it isn't available from the base class.
  int32_t GetPlayoutUnderrunCount() const override { return -1; }

  // Native audio parameters stored during construction.
  // These methods are unique for the iOS implementation.
  int GetPlayoutAudioParameters(AudioParameters* params) const override;
  int GetRecordAudioParameters(AudioParameters* params) const override;

  // These methods are currently not fully implemented on iOS:

  // See audio_device_not_implemented.cc for trivial implementations.
  int32_t ActiveAudioLayer(
      AudioDeviceModule::AudioLayer& audioLayer) const override;
  int32_t PlayoutIsAvailable(bool& available) override;
  int32_t RecordingIsAvailable(bool& available) override;
  int16_t PlayoutDevices() override;
  int16_t RecordingDevices() override;
  int32_t PlayoutDeviceName(uint16_t index,
                            char name[kAdmMaxDeviceNameSize],
                            char guid[kAdmMaxGuidSize]) override;
  int32_t RecordingDeviceName(uint16_t index,
                              char name[kAdmMaxDeviceNameSize],
                              char guid[kAdmMaxGuidSize]) override;
  int32_t SetPlayoutDevice(uint16_t index) override;
  int32_t SetPlayoutDevice(
      AudioDeviceModule::WindowsDeviceType device) override;
  int32_t SetRecordingDevice(uint16_t index) override;
  int32_t SetRecordingDevice(
      AudioDeviceModule::WindowsDeviceType device) override;
  int32_t InitSpeaker() override;
  bool SpeakerIsInitialized() const override;
  int32_t InitMicrophone() override;
  bool MicrophoneIsInitialized() const override;
  int32_t SpeakerVolumeIsAvailable(bool& available) override;
  int32_t SetSpeakerVolume(uint32_t volume) override;
  int32_t SpeakerVolume(uint32_t& volume) const override;
  int32_t MaxSpeakerVolume(uint32_t& maxVolume) const override;
  int32_t MinSpeakerVolume(uint32_t& minVolume) const override;
  int32_t MicrophoneVolumeIsAvailable(bool& available) override;
  int32_t SetMicrophoneVolume(uint32_t volume) override;
  int32_t MicrophoneVolume(uint32_t& volume) const override;
  int32_t MaxMicrophoneVolume(uint32_t& maxVolume) const override;
  int32_t MinMicrophoneVolume(uint32_t& minVolume) const override;
  int32_t MicrophoneMuteIsAvailable(bool& available) override;
  int32_t SetMicrophoneMute(bool enable) override;
  int32_t MicrophoneMute(bool& enabled) const override;
  int32_t SpeakerMuteIsAvailable(bool& available) override;
  int32_t SetSpeakerMute(bool enable) override;
  int32_t SpeakerMute(bool& enabled) const override;
  int32_t StereoPlayoutIsAvailable(bool& available) override;
  int32_t SetStereoPlayout(bool enable) override;
  int32_t StereoPlayout(bool& enabled) const override;
  int32_t StereoRecordingIsAvailable(bool& available) override;
  int32_t SetStereoRecording(bool enable) override;
  int32_t StereoRecording(bool& enabled) const override;

  // AudioSessionObserver methods. May be called from any thread.
  void OnInterruptionBegin() override;
  void OnInterruptionEnd(bool should_resume) override;
  void OnValidRouteChange() override;
  void OnCanPlayOrRecordChange(bool can_play_or_record) override;
  void OnChangedOutputVolume() override;

  // VoiceProcessingAudioUnitObserver methods.
  OSStatus OnDeliverRecordedData(AudioUnitRenderActionFlags* flags,
                                 const AudioTimeStamp* time_stamp,
                                 UInt32 bus_number,
                                 UInt32 num_frames,
                                 AudioBufferList* io_data) override;
  OSStatus OnGetPlayoutData(AudioUnitRenderActionFlags* flags,
                            const AudioTimeStamp* time_stamp,
                            UInt32 bus_number,
                            UInt32 num_frames,
                            AudioBufferList* io_data) override;
  void OnReceivedMutedSpeechActivity(
      AUVoiceIOSpeechActivityEvent event) override;

  bool IsInterrupted();

  std::optional<AudioDeviceModule::Stats> GetStats() const;

 private:
  // Called by the relevant AudioSessionObserver methods on `thread_`.
  void HandleInterruptionBegin();
  void HandleInterruptionEnd();
  void HandleValidRouteChange();
  void HandleCanPlayOrRecordChange(bool can_play_or_record);
  void HandleSampleRateChange();
  void HandlePlayoutGlitchDetected(uint64_t glitch_duration_ms);
  void HandleOutputVolumeChange();

  bool RestartAudioUnit(bool enable_input);

  // Uses current `playout_parameters_` and `record_parameters_` to inform the
  // audio device buffer (ADB) about our internal audio parameters.
  void UpdateAudioDeviceBuffer();

  // Since the preferred audio parameters are only hints to the OS, the actual
  // values may be different once the AVAudioSession has been activated.
  // This method asks for the current hardware parameters and takes actions
  // if they should differ from what we have asked for initially. It also
  // defines `playout_parameters_` and `record_parameters_`.
  void SetupAudioBuffersForActiveAudioSession();

  // Creates the audio unit.
  bool CreateAudioUnit();

  // Updates the audio unit state based on current state.
  void UpdateAudioUnit(bool can_play_or_record);

  // Configures the audio session for WebRTC.
  bool ConfigureAudioSession();

  // Like above, but requires caller to already hold session lock.
  bool ConfigureAudioSessionLocked();

  // Unconfigures the audio session.
  void UnconfigureAudioSession();

  // Activates our audio session, creates and initializes the voice-processing
  // audio unit and verifies that we got the preferred native audio parameters.
  bool InitPlayOrRecord(bool enable_input);

  // Closes and deletes the voice-processing I/O unit.
  void ShutdownPlayOrRecord();

  // Resets thread-checkers before a call is restarted.
  void PrepareForNewStart();

  // Determines whether voice processing should be enabled or disabled.
  const bool bypass_voice_processing_;

  // Handle a user speaking during muted event
  AudioDeviceModule::MutedSpeechEventHandler muted_speech_event_handler_;

  // Handle microphone rendering errors.
  AudioDeviceIOSRenderErrorHandler render_error_handler_;

  // Copying microphone data (rendering to a buffer) may keep failing. This
  // field makes sure subsequent errors are not reported.
  bool disregard_next_render_error_;

  // Native I/O audio thread checker.
  SequenceChecker io_thread_checker_;

  // Thread that this object is created on.
  webrtc::Thread* thread_;

  // Raw pointer handle provided to us in AttachAudioBuffer(). Owned by the
  // AudioDeviceModuleImpl class and called by AudioDeviceModule::Create().
  // The AudioDeviceBuffer is a member of the AudioDeviceModuleImpl instance
  // and therefore outlives this object.
  AudioDeviceBuffer* audio_device_buffer_;

  // Contains audio parameters (sample rate, #channels, buffer size etc.) for
  // the playout and recording sides. These structure is set in two steps:
  // first, native sample rate and #channels are defined in Init(). Next, the
  // audio session is activated and we verify that the preferred parameters
  // were granted by the OS. At this stage it is also possible to add a third
  // component to the parameters; the native I/O buffer duration.
  // A RTC_CHECK will be hit if we for some reason fail to open an audio session
  // using the specified parameters.
  AudioParameters playout_parameters_;
  AudioParameters record_parameters_;

  // The AudioUnit used to play and record audio.
  std::unique_ptr<VoiceProcessingAudioUnit> audio_unit_;

  // FineAudioBuffer takes an AudioDeviceBuffer which delivers audio data
  // in chunks of 10ms. It then allows for this data to be pulled in
  // a finer or coarser granularity. I.e. interacting with this class instead
  // of directly with the AudioDeviceBuffer one can ask for any number of
  // audio data samples. Is also supports a similar scheme for the recording
  // side.
  // Example: native buffer size can be 128 audio frames at 16kHz sample rate.
  // WebRTC will provide 480 audio frames per 10ms but iOS asks for 128
  // in each callback (one every 8ms). This class can then ask for 128 and the
  // FineAudioBuffer will ask WebRTC for new data only when needed and also
  // cache non-utilized audio between callbacks. On the recording side, iOS
  // can provide audio data frames of size 128 and these are accumulated until
  // enough data to supply one 10ms call exists. This 10ms chunk is then sent
  // to WebRTC and the remaining part is stored.
  std::unique_ptr<FineAudioBuffer> fine_audio_buffer_;

  // Temporary storage for recorded data. AudioUnitRender() renders into this
  // array as soon as a frame of the desired buffer size has been recorded.
  // On real iOS devices, the size will be fixed and set once. For iOS
  // simulators, the size can vary from callback to callback and the size
  // will be changed dynamically to account for this behavior.
  webrtc::BufferT<int16_t> record_audio_buffer_;

  bool recording_is_initialized_;

  // Set to 1 when recording is active and 0 otherwise.
  std::atomic<int> recording_;

  bool playout_is_initialized_;

  // Set to 1 when playout is active and 0 otherwise.
  std::atomic<int> playing_;

  // Set to true after successful call to Init(), false otherwise.
  bool initialized_ RTC_GUARDED_BY(thread_);

  // Set to true if audio session is interrupted, false otherwise.
  bool is_interrupted_;

  // Audio interruption observer instance.
  RTC_OBJC_TYPE(RTCNativeAudioSessionDelegateAdapter)* audio_session_observer_
      RTC_GUARDED_BY(thread_);

  // Set to true if we've activated the audio session.
  bool has_configured_session_ RTC_GUARDED_BY(thread_);

  // Counts number of detected audio glitches on the playout side.
  std::atomic<uint64_t> num_detected_playout_glitches_;
  std::atomic<uint64_t> total_playout_glitches_duration_ms_;
  int64_t last_playout_time_ RTC_GUARDED_BY(io_thread_checker_);

  // Counts number of playout callbacks per call.
  // The value is updated on the native I/O thread and later read on the
  // creating `thread_` but at this stage no audio is active.
  // Hence, it is a "thread safe" design and no lock is needed.
  int64_t num_playout_callbacks_;

  // Contains the time for when the last output volume change was detected.
  int64_t last_output_volume_change_time_ RTC_GUARDED_BY(thread_);

  // Avoids running pending task after `this` is Terminated.
  webrtc::scoped_refptr<PendingTaskSafetyFlag> safety_ =
      PendingTaskSafetyFlag::Create();

  // Playout stats.
  std::atomic<uint64_t> total_playout_samples_count_;
  std::atomic<uint64_t> total_playout_samples_duration_ms_;
  std::atomic<uint64_t> total_playout_delay_ms_;
  std::atomic<double> hw_output_latency_;
  int last_hw_output_latency_update_sample_count_;
  // Ratio between mach tick units and nanosecond. Used to change mach tick
  // units to nanoseconds.
  double machTickUnitsToNanoseconds_;
};
}  // namespace ios_adm
}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_DEVICE_IOS_H_
