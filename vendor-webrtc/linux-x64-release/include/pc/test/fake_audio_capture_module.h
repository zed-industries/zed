/*
 *  Copyright 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This class implements an AudioCaptureModule that can be used to detect if
// audio is being received properly if it is fed by another AudioCaptureModule
// in some arbitrary audio pipeline where they are connected. It does not play
// out or record any audio so it does not need access to any hardware and can
// therefore be used in the gtest testing framework.

// Note P postfix of a function indicates that it should only be called by the
// processing thread.

#ifndef PC_TEST_FAKE_AUDIO_CAPTURE_MODULE_H_
#define PC_TEST_FAKE_AUDIO_CAPTURE_MODULE_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>

#include "api/audio/audio_device.h"
#include "api/audio/audio_device_defines.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

class FakeAudioCaptureModule : public webrtc::AudioDeviceModule {
 public:
  typedef uint16_t Sample;

  // The value for the following constants have been derived by running VoE
  // using a real ADM. The constants correspond to 10ms of mono audio at 44kHz.
  static const size_t kNumberSamples = 440;
  static const size_t kNumberBytesPerSample = sizeof(Sample);

  // Creates a FakeAudioCaptureModule or returns NULL on failure.
  static webrtc::scoped_refptr<FakeAudioCaptureModule> Create();

  // Returns the number of frames that have been successfully pulled by the
  // instance. Note that correctly detecting success can only be done if the
  // pulled frame was generated/pushed from a FakeAudioCaptureModule.
  int frames_received() const RTC_LOCKS_EXCLUDED(mutex_);

  int32_t ActiveAudioLayer(AudioLayer* audio_layer) const override;

  // Note: Calling this method from a callback may result in deadlock.
  int32_t RegisterAudioCallback(webrtc::AudioTransport* audio_callback) override
      RTC_LOCKS_EXCLUDED(mutex_);

  int32_t Init() override;
  int32_t Terminate() override;
  bool Initialized() const override;

  int16_t PlayoutDevices() override;
  int16_t RecordingDevices() override;
  int32_t PlayoutDeviceName(uint16_t index,
                            char name[webrtc::kAdmMaxDeviceNameSize],
                            char guid[webrtc::kAdmMaxGuidSize]) override;
  int32_t RecordingDeviceName(uint16_t index,
                              char name[webrtc::kAdmMaxDeviceNameSize],
                              char guid[webrtc::kAdmMaxGuidSize]) override;

  int32_t SetPlayoutDevice(uint16_t index) override;
  int32_t SetPlayoutDevice(WindowsDeviceType device) override;
  int32_t SetRecordingDevice(uint16_t index) override;
  int32_t SetRecordingDevice(WindowsDeviceType device) override;

  int32_t PlayoutIsAvailable(bool* available) override;
  int32_t InitPlayout() override;
  bool PlayoutIsInitialized() const override;
  int32_t RecordingIsAvailable(bool* available) override;
  int32_t InitRecording() override;
  bool RecordingIsInitialized() const override;

  int32_t StartPlayout() RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t StopPlayout() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool Playing() const RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t StartRecording() RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t StopRecording() RTC_LOCKS_EXCLUDED(mutex_) override;
  bool Recording() const RTC_LOCKS_EXCLUDED(mutex_) override;

  int32_t InitSpeaker() override;
  bool SpeakerIsInitialized() const override;
  int32_t InitMicrophone() override;
  bool MicrophoneIsInitialized() const override;

  int32_t SpeakerVolumeIsAvailable(bool* available) override;
  int32_t SetSpeakerVolume(uint32_t volume) override;
  int32_t SpeakerVolume(uint32_t* volume) const override;
  int32_t MaxSpeakerVolume(uint32_t* max_volume) const override;
  int32_t MinSpeakerVolume(uint32_t* min_volume) const override;

  int32_t MicrophoneVolumeIsAvailable(bool* available) override;
  int32_t SetMicrophoneVolume(uint32_t volume)
      RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t MicrophoneVolume(uint32_t* volume) const
      RTC_LOCKS_EXCLUDED(mutex_) override;
  int32_t MaxMicrophoneVolume(uint32_t* max_volume) const override;

  int32_t MinMicrophoneVolume(uint32_t* min_volume) const override;

  int32_t SpeakerMuteIsAvailable(bool* available) override;
  int32_t SetSpeakerMute(bool enable) override;
  int32_t SpeakerMute(bool* enabled) const override;

  int32_t MicrophoneMuteIsAvailable(bool* available) override;
  int32_t SetMicrophoneMute(bool enable) override;
  int32_t MicrophoneMute(bool* enabled) const override;

  int32_t StereoPlayoutIsAvailable(bool* available) const override;
  int32_t SetStereoPlayout(bool enable) override;
  int32_t StereoPlayout(bool* enabled) const override;
  int32_t StereoRecordingIsAvailable(bool* available) const override;
  int32_t SetStereoRecording(bool enable) override;
  int32_t StereoRecording(bool* enabled) const override;

  int32_t PlayoutDelay(uint16_t* delay_ms) const override;

  bool BuiltInAECIsAvailable() const override { return false; }
  int32_t EnableBuiltInAEC(bool enable) override { return -1; }
  bool BuiltInAGCIsAvailable() const override { return false; }
  int32_t EnableBuiltInAGC(bool enable) override { return -1; }
  bool BuiltInNSIsAvailable() const override { return false; }
  int32_t EnableBuiltInNS(bool enable) override { return -1; }

  int32_t GetPlayoutUnderrunCount() const override { return -1; }

  std::optional<webrtc::AudioDeviceModule::Stats> GetStats() const override {
    return webrtc::AudioDeviceModule::Stats();
  }
#if defined(WEBRTC_IOS)
  int GetPlayoutAudioParameters(
      webrtc::AudioParameters* params) const override {
    return -1;
  }
  int GetRecordAudioParameters(webrtc::AudioParameters* params) const override {
    return -1;
  }
#endif  // WEBRTC_IOS

  // End of functions inherited from webrtc::AudioDeviceModule.

 protected:
  // The constructor is protected because the class needs to be created as a
  // reference counted object (for memory managment reasons). It could be
  // exposed in which case the burden of proper instantiation would be put on
  // the creator of a FakeAudioCaptureModule instance. To create an instance of
  // this class use the Create(..) API.
  FakeAudioCaptureModule();
  // The destructor is protected because it is reference counted and should not
  // be deleted directly.
  virtual ~FakeAudioCaptureModule();

 private:
  // Initializes the state of the FakeAudioCaptureModule. This API is called on
  // creation by the Create() API.
  bool Initialize();
  // SetBuffer() sets all samples in send_buffer_ to `value`.
  void SetSendBuffer(int value);
  // Resets rec_buffer_. I.e., sets all rec_buffer_ samples to 0.
  void ResetRecBuffer();
  // Returns true if rec_buffer_ contains one or more sample greater than or
  // equal to `value`.
  bool CheckRecBuffer(int value);

  // Returns true/false depending on if recording or playback has been
  // enabled/started.
  bool ShouldStartProcessing() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Starts or stops the pushing and pulling of audio frames.
  void UpdateProcessing(bool start) RTC_LOCKS_EXCLUDED(mutex_);

  // Starts the periodic calling of ProcessFrame() in a thread safe way.
  void StartProcessP();
  // Periodcally called function that ensures that frames are pulled and pushed
  // periodically if enabled/started.
  void ProcessFrameP() RTC_LOCKS_EXCLUDED(mutex_);
  // Pulls frames from the registered webrtc::AudioTransport.
  void ReceiveFrameP() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  // Pushes frames to the registered webrtc::AudioTransport.
  void SendFrameP() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Callback for playout and recording.
  webrtc::AudioTransport* audio_callback_ RTC_GUARDED_BY(mutex_);

  bool recording_ RTC_GUARDED_BY(
      mutex_);  // True when audio is being pushed from the instance.
  bool playing_ RTC_GUARDED_BY(
      mutex_);  // True when audio is being pulled by the instance.

  bool play_is_initialized_;  // True when the instance is ready to pull audio.
  bool rec_is_initialized_;   // True when the instance is ready to push audio.

  // Input to and output from RecordedDataIsAvailable(..) makes it possible to
  // modify the current mic level. The implementation does not care about the
  // mic level so it just feeds back what it receives.
  uint32_t current_mic_level_ RTC_GUARDED_BY(mutex_);

  // next_frame_time_ is updated in a non-drifting manner to indicate the next
  // wall clock time the next frame should be generated and received. started_
  // ensures that next_frame_time_ can be initialized properly on first call.
  bool started_ RTC_GUARDED_BY(mutex_);
  int64_t next_frame_time_ RTC_GUARDED_BY(process_thread_checker_);

  std::unique_ptr<webrtc::Thread> process_thread_;

  // Buffer for storing samples received from the webrtc::AudioTransport.
  char rec_buffer_[kNumberSamples * kNumberBytesPerSample];
  // Buffer for samples to send to the webrtc::AudioTransport.
  char send_buffer_[kNumberSamples * kNumberBytesPerSample];

  // Counter of frames received that have samples of high enough amplitude to
  // indicate that the frames are not faked somewhere in the audio pipeline
  // (e.g. by a jitter buffer).
  int frames_received_;

  // Protects variables that are accessed from process_thread_ and
  // the main thread.
  mutable webrtc::Mutex mutex_;
  webrtc::SequenceChecker process_thread_checker_{
      webrtc::SequenceChecker::kDetached};
};

#endif  // PC_TEST_FAKE_AUDIO_CAPTURE_MODULE_H_
