/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_AUDIO_VOICE_PROCESSING_AUDIO_UNIT_H_
#define SDK_OBJC_NATIVE_SRC_AUDIO_VOICE_PROCESSING_AUDIO_UNIT_H_

#include <AudioUnit/AudioUnit.h>

namespace webrtc {
namespace ios_adm {

class VoiceProcessingAudioUnitObserver {
 public:
  // Callback function called on a real-time priority I/O thread from the audio
  // unit. This method is used to signal that recorded audio is available.
  virtual OSStatus OnDeliverRecordedData(AudioUnitRenderActionFlags* flags,
                                         const AudioTimeStamp* time_stamp,
                                         UInt32 bus_number,
                                         UInt32 num_frames,
                                         AudioBufferList* io_data) = 0;

  // Callback function called on a real-time priority I/O thread from the audio
  // unit. This method is used to provide audio samples to the audio unit.
  virtual OSStatus OnGetPlayoutData(AudioUnitRenderActionFlags* io_action_flags,
                                    const AudioTimeStamp* time_stamp,
                                    UInt32 bus_number,
                                    UInt32 num_frames,
                                    AudioBufferList* io_data) = 0;

  // Callback function called when a user speaking during muted is detected by
  // system.
  virtual void OnReceivedMutedSpeechActivity(
      AUVoiceIOSpeechActivityEvent event) = 0;

 protected:
  ~VoiceProcessingAudioUnitObserver() {}
};

// Convenience class to abstract away the management of a Voice Processing
// I/O Audio Unit. The Voice Processing I/O unit has the same characteristics
// as the Remote I/O unit (supports full duplex low-latency audio input and
// output) and adds AEC for for two-way duplex communication. It also adds AGC,
// adjustment of voice-processing quality, and muting. Hence, ideal for
// VoIP applications.
class VoiceProcessingAudioUnit {
 public:
  VoiceProcessingAudioUnit(bool bypass_voice_processing,
                           bool detect_mute_speech,
                           VoiceProcessingAudioUnitObserver* observer);
  ~VoiceProcessingAudioUnit();

  // TODO(tkchin): enum for state and state checking.
  enum State : int32_t {
    // Init() should be called.
    kInitRequired,
    // Audio unit created but not initialized.
    kUninitialized,
    // Initialized but not started. Equivalent to stopped.
    kInitialized,
    // Initialized and started.
    kStarted,
  };

  // Number of bytes per audio sample for 16-bit signed integer representation.
  static const UInt32 kBytesPerSample;

  // Initializes this class by creating the underlying audio unit instance.
  // Creates a Voice-Processing I/O unit and configures it for full-duplex
  // audio. The selected stream format is selected to avoid internal resampling
  // and to match the 10ms callback rate for WebRTC as well as possible.
  // Does not intialize the audio unit.
  bool Init();

  VoiceProcessingAudioUnit::State GetState() const;

  // Initializes the underlying audio unit with the given sample rate.
  bool Initialize(Float64 sample_rate, bool enable_input);

  // Starts the underlying audio unit.
  OSStatus Start();

  // Stops the underlying audio unit.
  bool Stop();

  // Uninitializes the underlying audio unit.
  bool Uninitialize();

  // Mutes the microphone.
  bool SetMicrophoneMute(bool enable);

  // Calls render on the underlying audio unit.
  OSStatus Render(AudioUnitRenderActionFlags* flags,
                  const AudioTimeStamp* time_stamp,
                  UInt32 output_bus_number,
                  UInt32 num_frames,
                  AudioBufferList* io_data);

 private:
  // The C API used to set callbacks requires static functions. When these are
  // called, they will invoke the relevant instance method by casting
  // in_ref_con to VoiceProcessingAudioUnit*.
  static OSStatus OnGetPlayoutData(void* in_ref_con,
                                   AudioUnitRenderActionFlags* flags,
                                   const AudioTimeStamp* time_stamp,
                                   UInt32 bus_number,
                                   UInt32 num_frames,
                                   AudioBufferList* io_data);
  static OSStatus OnDeliverRecordedData(void* in_ref_con,
                                        AudioUnitRenderActionFlags* flags,
                                        const AudioTimeStamp* time_stamp,
                                        UInt32 bus_number,
                                        UInt32 num_frames,
                                        AudioBufferList* io_data);

  // Notifies observer that samples are needed for playback.
  OSStatus NotifyGetPlayoutData(AudioUnitRenderActionFlags* flags,
                                const AudioTimeStamp* time_stamp,
                                UInt32 bus_number,
                                UInt32 num_frames,
                                AudioBufferList* io_data);
  // Notifies observer that recorded samples are available for render.
  OSStatus NotifyDeliverRecordedData(AudioUnitRenderActionFlags* flags,
                                     const AudioTimeStamp* time_stamp,
                                     UInt32 bus_number,
                                     UInt32 num_frames,
                                     AudioBufferList* io_data);

  // Returns the predetermined format with a specific sample rate. See
  // implementation file for details on format.
  AudioStreamBasicDescription GetFormat(Float64 sample_rate) const;

  // Deletes the underlying audio unit.
  void DisposeAudioUnit();

  const bool bypass_voice_processing_;
  const bool detect_mute_speech_;
  VoiceProcessingAudioUnitObserver* observer_;
  AudioUnit vpio_unit_;
  VoiceProcessingAudioUnit::State state_;
};
}  // namespace ios_adm
}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_AUDIO_VOICE_PROCESSING_AUDIO_UNIT_H_
