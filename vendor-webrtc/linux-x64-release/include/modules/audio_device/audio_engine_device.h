/*
 * Copyright 2024 LiveKit
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_DEVICE_AUDIOENGINE_H_
#define SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_DEVICE_AUDIOENGINE_H_

#include <atomic>
#include <memory>

#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "modules/audio_device/audio_device_generic.h"
#include "rtc_base/buffer.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "sdk/objc/base/RTCMacros.h"
#include "sdk/objc/native/src/audio/audio_session_observer.h"

#if TARGET_OS_OSX
#import <CoreAudio/CoreAudio.h>
#endif
#import <AVFAudio/AVFAudio.h>
#import <AudioToolbox/AudioToolbox.h>

RTC_FWD_DECL_OBJC_CLASS(RTC_OBJC_TYPE(RTCNativeAudioSessionDelegateAdapter));

namespace webrtc {

// Error codes for AudioEngineDevice.
// Apple recommends that you use values in the range -1000 through -9999 inclusive. Values outside
// of this range are reserved by Apple for internal use.
enum AudioEngineErrorCode {
  // Success (no error)
  kAudioEngineNoError = 0,

  // General errors
  kAudioEngineUnknownError = -1000,
  kAudioEngineInitError = -1001,
  kAudioEngineTerminateError = -1002,
  kAudioEngineNotInitializedError = -1003,
  kAudioEngineAlreadyInitializedError = -1004,

  // Device errors
  kAudioEngineDeviceNotFoundError = -2000,
  kAudioEngineDeviceUnavailableError = -2001,
  kAudioEngineDeviceDisconnectedError = -2002,
  kAudioEngineDeviceUnauthorizedError = -2003,
  kAudioEngineDeviceInUseError = -2004,
  kAudioEngineDeviceFormatError = -2005,

  // Playback errors
  kAudioEnginePlayoutInitError = -3000,
  kAudioEnginePlayoutStartError = -3001,
  kAudioEnginePlayoutStopError = -3002,
  kAudioEnginePlayoutAlreadyInitializedError = -3003,
  kAudioEnginePlayoutNotInitializedError = -3004,
  kAudioEnginePlayoutDeviceNotAvailableError = -3010,

  // Recording errors
  kAudioEngineRecordingInitError = -4000,
  kAudioEngineRecordingStartError = -4001,
  kAudioEngineRecordingStopError = -4002,
  kAudioEngineRecordingAlreadyInitializedError = -4003,
  kAudioEngineRecordingNotInitializedError = -4004,
  kAudioEngineRecordingPermissionDeniedError = -4005,
  kAudioEngineRecordingDeviceNotAvailableError = -4010,

  // Engine state errors
  kAudioEngineInvalidStateError = -5000,
  kAudioEngineStateTransitionError = -5001,
  kAudioEngineInterruptionError = -5002,

  // Resource errors
  kAudioEngineOutOfMemoryError = -6000,
  kAudioEngineResourceLimitError = -6001,

  // Render mode errors
  kAudioEngineRenderModeError = -7000,
  kAudioEngineManualRenderingError = -7001,

  // Voice processing errors
  kAudioEngineVoiceProcessingError = -8000,
  kAudioEngineAGCError = -8001,

  // Permission and session errors
  kAudioEngineErrorInsufficientDevicePermission = -9000,
  kAudioEngineErrorAudioSessionInvalidCategory = -9001
};

class FineAudioBuffer;

extern NSString* const kAudioEngineInputMixerNodeKey;

class AudioEngineDevice : public AudioDeviceModule, public AudioSessionObserver {
 public:
  enum RenderMode { Device = 0, Manual = 1 };
  enum MuteMode {
    // Mute input using voice processing
    VoiceProcessing = 0,
    // Mute by restarting engine
    RestartEngine = 1,
    // Mute input by muting the input mixer node
    InputMixer = 2,
  };

  // Ducking level for voice processing.
  // Maps to AVAudioVoiceProcessingOtherAudioDuckingLevel (iOS 17.0+, macOS 14.0+).
  enum AudioDuckingLevel {
    AudioDuckingLevelDefault = 0,
    AudioDuckingLevelMin = 1,
    AudioDuckingLevelMid = 2,
    AudioDuckingLevelMax = 3,
  };

  // Represents the state of the audio engine, including input/output status,
  // rendering mode, and various configuration flags.
  struct EngineState {
    bool input_enabled = false;
    bool input_running = false;
    bool output_enabled = false;
    bool output_running = false;

    bool output_available = true;
    bool input_available = true;

    bool input_enabled_persistent_mode = false;

    bool input_muted = true;
    bool is_interrupted = false;

    RenderMode render_mode = RenderMode::Device;
    MuteMode mute_mode = MuteMode::VoiceProcessing;

    bool voice_processing_enabled = true;
    bool voice_processing_bypassed = false;
    bool voice_processing_agc_enabled = true;
    bool advanced_ducking = false;
    AudioDuckingLevel ducking_level = AudioDuckingLevelMin;

    uint32_t output_device_id = 0;  // kAudioObjectUnknown
    uint32_t input_device_id = 0;   // kAudioObjectUnknown

    uint32_t default_output_device_update_count = 0;  // Track default switch count
    uint32_t default_input_device_update_count = 0;

    bool operator==(const EngineState& rhs) const {
      return input_enabled == rhs.input_enabled && input_running == rhs.input_running &&
             output_enabled == rhs.output_enabled && output_running == rhs.output_running &&
             input_available == rhs.input_available && output_available == rhs.output_available &&
             input_enabled_persistent_mode == rhs.input_enabled_persistent_mode &&
             input_muted == rhs.input_muted && is_interrupted == rhs.is_interrupted &&
             render_mode == rhs.render_mode && mute_mode == rhs.mute_mode &&
             voice_processing_enabled == rhs.voice_processing_enabled &&
             voice_processing_bypassed == rhs.voice_processing_bypassed &&
             voice_processing_agc_enabled == rhs.voice_processing_agc_enabled &&
             advanced_ducking == rhs.advanced_ducking && ducking_level == rhs.ducking_level &&
             output_device_id == rhs.output_device_id && input_device_id == rhs.input_device_id &&
             default_output_device_update_count == rhs.default_output_device_update_count &&
             default_input_device_update_count == rhs.default_input_device_update_count;
    }

    bool operator!=(const EngineState& rhs) const { return !(*this == rhs); }

    // AUDIO STATE LOGIC
    //
    // Device Mode:
    // - Output follows input only when voice_processing_enabled=true (for AEC)
    // - Input respects mute mode restrictions (RestartEngine + input_muted)
    // - Independent operation when voice processing is disabled
    //
    // Manual Mode:
    // - Bidirectional coupling: if ANY component is enabled/running, BOTH are considered
    // enabled/running
    // - No mute mode restrictions (manual control bypasses automatic muting)
    // - Required for AVAudioEngine graph connectivity (input must connect to output)
    //
    // All modes respect availability flags (input_available/output_available)

    bool IsOutputEnabled() const {
      if (!output_available) return false;

      switch (render_mode) {
        case RenderMode::Device:
          return voice_processing_enabled ? (IsInputEnabled() || output_enabled) : output_enabled;
        case RenderMode::Manual:
          return output_enabled || input_enabled || input_enabled_persistent_mode;
      }
    }

    bool IsOutputRunning() const {
      if (!output_available) return false;

      switch (render_mode) {
        case RenderMode::Device:
          return voice_processing_enabled ? (IsInputRunning() || output_running) : output_running;
        case RenderMode::Manual:
          return output_running || input_running;
      }
    }

    bool IsInputEnabled() const {
      if (!input_available) return false;

      switch (render_mode) {
        case RenderMode::Device:
          return !(mute_mode == MuteMode::RestartEngine && input_muted) &&
                 (input_enabled || input_enabled_persistent_mode);
        case RenderMode::Manual:
          return input_enabled || input_enabled_persistent_mode || output_enabled;
      }
    }

    bool IsInputRunning() const {
      if (!input_available) return false;

      switch (render_mode) {
        case RenderMode::Device:
          return !(mute_mode == MuteMode::RestartEngine && input_muted) && input_running;
        case RenderMode::Manual:
          return input_running || output_running;
      }
    }

    bool IsAnyEnabled() const { return IsInputEnabled() || IsOutputEnabled(); }
    bool IsAnyRunning() const { return IsInputRunning() || IsOutputRunning(); }

    bool IsOutputDefaultDevice() const {
#if TARGET_OS_OSX
      return output_device_id == kAudioObjectUnknown;
#else
      return output_device_id == 0;
#endif
    }

    bool IsInputDefaultDevice() const {
#if TARGET_OS_OSX
      return input_device_id == kAudioObjectUnknown;
#else
      return input_device_id == 0;
#endif
    }
  };

  explicit AudioEngineDevice(bool voice_processing_bypassed);
  ~AudioEngineDevice() override;

  int32_t Init() override;
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

  int32_t PlayoutDelay(uint16_t* delayMS) const override;
  int32_t GetPlayoutUnderrunCount() const override { return -1; }

#if defined(WEBRTC_IOS)
  int GetPlayoutAudioParameters(AudioParameters* params) const override;
  int GetRecordAudioParameters(AudioParameters* params) const override;
#endif

  int32_t ActiveAudioLayer(AudioDeviceModule::AudioLayer* audioLayer) const override;
  int32_t PlayoutIsAvailable(bool* available) override;
  int32_t RecordingIsAvailable(bool* available) override;
  int16_t PlayoutDevices() override;
  int16_t RecordingDevices() override;
  int32_t PlayoutDeviceName(uint16_t index, char name[kAdmMaxDeviceNameSize],
                            char guid[kAdmMaxGuidSize]) override;
  int32_t RecordingDeviceName(uint16_t index, char name[kAdmMaxDeviceNameSize],
                              char guid[kAdmMaxGuidSize]) override;
  int32_t SetPlayoutDevice(uint16_t index) override;
  int32_t SetPlayoutDevice(AudioDeviceModule::WindowsDeviceType device) override;
  int32_t SetRecordingDevice(uint16_t index) override;
  int32_t SetRecordingDevice(AudioDeviceModule::WindowsDeviceType device) override;
  int32_t InitSpeaker() override;
  bool SpeakerIsInitialized() const override;
  int32_t InitMicrophone() override;
  bool MicrophoneIsInitialized() const override;
  int32_t SpeakerVolumeIsAvailable(bool* available) override;
  int32_t SetSpeakerVolume(uint32_t volume) override;
  int32_t SpeakerVolume(uint32_t* volume) const override;
  int32_t MaxSpeakerVolume(uint32_t* maxVolume) const override;
  int32_t MinSpeakerVolume(uint32_t* minVolume) const override;
  int32_t MicrophoneVolumeIsAvailable(bool* available) override;
  int32_t SetMicrophoneVolume(uint32_t volume) override;
  int32_t MicrophoneVolume(uint32_t* volume) const override;
  int32_t MaxMicrophoneVolume(uint32_t* maxVolume) const override;
  int32_t MinMicrophoneVolume(uint32_t* minVolume) const override;
  int32_t MicrophoneMuteIsAvailable(bool* available) override;
  int32_t SetMicrophoneMute(bool enable) override;
  int32_t MicrophoneMute(bool* enabled) const override;
  int32_t SpeakerMuteIsAvailable(bool* available) override;
  int32_t SetSpeakerMute(bool enable) override;
  int32_t SpeakerMute(bool* enabled) const override;
  int32_t StereoPlayoutIsAvailable(bool* available) const override;
  int32_t SetStereoPlayout(bool enable) override;
  int32_t StereoPlayout(bool* enabled) const override;
  int32_t StereoRecordingIsAvailable(bool* available) const override;
  int32_t SetStereoRecording(bool enable) override;
  int32_t StereoRecording(bool* enabled) const override;

  int32_t RegisterAudioCallback(AudioTransport* audioCallback) override;

  // Only supported on Android.
  bool BuiltInAECIsAvailable() const override;
  bool BuiltInAGCIsAvailable() const override;
  bool BuiltInNSIsAvailable() const override;

  // Enables the built-in audio effects. Only supported on Android.
  int32_t EnableBuiltInAEC(bool enable) override;
  int32_t EnableBuiltInAGC(bool enable) override;
  int32_t EnableBuiltInNS(bool enable) override;

  // AudioSessionObserver methods. May be called from any thread.
  void OnInterruptionBegin() override;
  void OnInterruptionEnd(bool should_resume) override;
  void OnValidRouteChange() override;
  void OnCanPlayOrRecordChange(bool can_play_or_record) override;
  void OnChangedOutputVolume() override;

  bool IsInterrupted();

  bool IsEngineRunning();

  int32_t SetEngineState(EngineState enable);
  int32_t GetEngineState(EngineState* enabled);

  int32_t SetEngineAvailability(bool input_available, bool output_available);
  int32_t EngineAvailability(bool* input_available, bool* output_available);

  int32_t SetObserver(AudioDeviceObserver* observer) override;

  int32_t SetManualRenderingMode(bool enable);
  int32_t ManualRenderingMode(bool* enabled);

  int32_t SetMuteMode(MuteMode mode);
  int32_t GetMuteMode(MuteMode* mode);

  int32_t SetAdvancedDucking(bool enable);
  int32_t AdvancedDucking(bool* enabled);

  int32_t SetDuckingLevel(AudioDuckingLevel level);
  int32_t DuckingLevel(AudioDuckingLevel* level);

  int32_t SetInitRecordingPersistentMode(bool enable);
  int32_t InitRecordingPersistentMode(bool* enabled);

  int32_t SetVoiceProcessingEnabled(bool enable);
  int32_t VoiceProcessingEnabled(bool* enabled);

  int32_t SetVoiceProcessingBypassed(bool enable);
  int32_t VoiceProcessingBypassed(bool* enabled);

  int32_t SetVoiceProcessingAGCEnabled(bool enable);
  int32_t VoiceProcessingAGCEnabled(bool* enabled);

  int32_t InitAndStartRecording();

  bool IsStopOnMuteModeEnabled() const override;

 private:
  struct EngineStateUpdate {
    EngineState prev;
    EngineState next;

    bool HasNoChanges() const { return prev == next; }

    bool DidEnableOutput() const { return !prev.IsOutputEnabled() && next.IsOutputEnabled(); }

    bool DidEnableInput() const { return !prev.IsInputEnabled() && next.IsInputEnabled(); }

    bool DidDisableOutput() const { return prev.IsOutputEnabled() && !next.IsOutputEnabled(); }

    bool DidDisableInput() const { return prev.IsInputEnabled() && !next.IsInputEnabled(); }

    bool DidAnyEnable() const { return DidEnableOutput() || DidEnableInput(); }

    bool DidAnyDisable() const { return DidDisableOutput() || DidDisableInput(); }

    bool DidBeginInterruption() const { return !prev.is_interrupted && next.is_interrupted; }

    bool DidEndInterruption() const { return prev.is_interrupted && !next.is_interrupted; }

    bool DidUpdateAudioGraph() const {
      return (prev.IsInputEnabled() != next.IsInputEnabled()) ||
             (prev.IsOutputEnabled() != next.IsOutputEnabled());
    }

    bool DidUpdateVoiceProcessingEnabled() const {
      return prev.voice_processing_enabled != next.voice_processing_enabled;
    }

    bool DidUpdateOutputDevice() const { return prev.output_device_id != next.output_device_id; }

    bool DidUpdateInputDevice() const { return prev.input_device_id != next.input_device_id; }

    bool DidUpdateDefaultOutputDevice() const {
      return prev.default_output_device_update_count != next.default_output_device_update_count;
    }

    bool DidUpdateDefaultInputDevice() const {
      return prev.default_input_device_update_count != next.default_input_device_update_count;
    }

    bool DidUpdateMuteMode() const { return prev.mute_mode != next.mute_mode; }

    bool IsEngineRestartRequired() const {
      return DidUpdateAudioGraph() ||
             // Voice processing enable state updates
             DidUpdateVoiceProcessingEnabled();
    }

    bool IsEngineRecreateRequired() const {
      // Device id specified
      bool device = DidUpdateOutputDevice() || DidUpdateInputDevice();

      // Default device updated
      bool default_device = (DidUpdateDefaultOutputDevice() && next.IsOutputDefaultDevice()) ||
                            (DidUpdateDefaultInputDevice() && next.IsInputDefaultDevice());

      // Special case to re-create engine when switching from Speaker & Mic ->
      // Speaker only.
      bool special_case = (prev.IsOutputEnabled() && next.IsOutputEnabled()) &&
                          (prev.IsInputEnabled() && !next.IsInputEnabled());

      return device || default_device || special_case;
    }

    bool DidEnableManualRenderingMode() const {
      return prev.render_mode != RenderMode::Manual && next.render_mode == RenderMode::Manual;
    }

    bool DidEnableDeviceRenderingMode() const {
      return prev.render_mode != RenderMode::Device && next.render_mode == RenderMode::Device;
    }
  };

  EngineState engine_state_ RTC_GUARDED_BY(thread_);

  int32_t ModifyEngineState(std::function<EngineState(EngineState)> state_transform);

  int32_t ApplyDeviceEngineState(EngineStateUpdate state);
  int32_t ApplyManualEngineState(EngineStateUpdate state);

  // AudioEngine observer methods. May be called from any thread.
  void ReconfigureEngine();

// Device related
#if TARGET_OS_OSX
  static OSStatus objectListenerProc(AudioObjectID objectId, UInt32 numberAddresses,
                                     const AudioObjectPropertyAddress addresses[],
                                     void* clientData);
  void HandleDeviceListenerEvent(AudioObjectPropertySelector selector);
  void UpdateAllDeviceIDs();

  // Debounce flags for device updates
  webrtc::scoped_refptr<PendingTaskSafetyFlag> default_device_update_safety_ =
      PendingTaskSafetyFlag::Create();
  const int kDefaultDeviceUpdateDebounceMs = 500;  // Debounce delay in milliseconds

  std::vector<AudioObjectID> input_device_ids_;
  std::vector<AudioObjectID> output_device_ids_;
  std::vector<std::string> output_device_labels_;
  std::vector<std::string> input_device_labels_;
#endif

  bool IsMicrophonePermissionGranted();
  bool EnsureMicrophonePermissionSync();

#if !TARGET_OS_OSX
  bool IsAudioSessionCategoryValid(NSString* category, bool is_input_enabled,
                                   bool is_output_enabled);
#endif
  void DebugAudioEngine();

  void StartRenderLoop();
  AVAudioEngineManualRenderingBlock render_block_;

  // Thread that this object is created on.
  webrtc::Thread* thread_;
  std::unique_ptr<webrtc::Thread> render_thread_;
  AVAudioPCMBuffer* render_buffer_;
  AVAudioPCMBuffer* read_buffer_;

  const std::unique_ptr<TaskQueueFactory> task_queue_factory_;
  std::unique_ptr<AudioDeviceBuffer> audio_device_buffer_;
  std::unique_ptr<FineAudioBuffer> fine_audio_buffer_;

  AudioParameters playout_parameters_;
  AudioParameters record_parameters_;

  // Set to true after successful call to Init(), false otherwise.
  bool initialized_ RTC_GUARDED_BY(thread_);

  AudioDeviceObserver* observer_ RTC_GUARDED_BY(thread_);

#if defined(WEBRTC_IOS)
  // Audio interruption observer instance.
  RTC_OBJC_TYPE(RTCNativeAudioSessionDelegateAdapter) * audio_session_observer_
      RTC_GUARDED_BY(thread_);
#endif

  // Avoids running pending task after `this` is Terminated.
  webrtc::scoped_refptr<PendingTaskSafetyFlag> safety_ = PendingTaskSafetyFlag::Create();

  // Ratio between mach tick units and nanosecond. Used to change mach tick
  // units to nanoseconds.
  double machTickUnitsToNanoseconds_;

  // AVAudioEngine objects
  AVAudioEngine* engine_device_ RTC_GUARDED_BY(thread_);
  AVAudioEngine* engine_manual_input_ RTC_GUARDED_BY(thread_);

  // Used for manual rendering mode
  AVAudioFormat* manual_render_rtc_format_;  // Int16

  // Output related
  AVAudioSourceNode* source_node_ RTC_GUARDED_BY(thread_);

  // Input related nodes
  AVAudioSinkNode* sink_node_ RTC_GUARDED_BY(thread_);
  AVAudioMixerNode* input_mixer_node_ RTC_GUARDED_BY(thread_);

  // Float32 -> Int16 converter.
  AudioConverterRef converter_ref_;
  AVAudioPCMBuffer* converter_buffer_;

  void* configuration_observer_ RTC_GUARDED_BY(thread_);
};
}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_DEVICE_AUDIOENGINE_H_
