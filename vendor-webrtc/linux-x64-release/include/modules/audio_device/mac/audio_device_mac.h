/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_DEVICE_AUDIO_DEVICE_MAC_H_
#define AUDIO_DEVICE_AUDIO_DEVICE_MAC_H_

#include <AudioToolbox/AudioConverter.h>
#include <CoreAudio/CoreAudio.h>
#include <mach/semaphore.h>

#include <atomic>
#include <memory>

#include "absl/strings/string_view.h"
#include "modules/audio_device/audio_device_generic.h"
#include "modules/audio_device/mac/audio_mixer_manager_mac.h"
#include "rtc_base/event.h"
#include "rtc_base/logging.h"
#include "rtc_base/platform_thread.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

struct PaUtilRingBuffer;

namespace webrtc {

const uint32_t N_REC_SAMPLES_PER_SEC = 48000;
const uint32_t N_PLAY_SAMPLES_PER_SEC = 48000;

const uint32_t N_REC_CHANNELS = 1;   // default is mono recording
const uint32_t N_PLAY_CHANNELS = 2;  // default is stereo playout
const uint32_t N_DEVICE_CHANNELS = 64;

const int kBufferSizeMs = 10;

const uint32_t ENGINE_REC_BUF_SIZE_IN_SAMPLES =
    N_REC_SAMPLES_PER_SEC * kBufferSizeMs / 1000;
const uint32_t ENGINE_PLAY_BUF_SIZE_IN_SAMPLES =
    N_PLAY_SAMPLES_PER_SEC * kBufferSizeMs / 1000;

const int N_BLOCKS_IO = 2;
const int N_BUFFERS_IN = 2;   // Must be at least N_BLOCKS_IO.
const int N_BUFFERS_OUT = 3;  // Must be at least N_BLOCKS_IO.

const uint32_t TIMER_PERIOD_MS = 2 * 10 * N_BLOCKS_IO * 1000000;

const uint32_t REC_BUF_SIZE_IN_SAMPLES =
    ENGINE_REC_BUF_SIZE_IN_SAMPLES * N_DEVICE_CHANNELS * N_BUFFERS_IN;
const uint32_t PLAY_BUF_SIZE_IN_SAMPLES =
    ENGINE_PLAY_BUF_SIZE_IN_SAMPLES * N_PLAY_CHANNELS * N_BUFFERS_OUT;

const int kGetMicVolumeIntervalMs = 1000;

class AudioDeviceMac : public AudioDeviceGeneric {
 public:
  AudioDeviceMac();
  ~AudioDeviceMac();

  // Retrieve the currently utilized audio layer
  virtual int32_t ActiveAudioLayer(
      AudioDeviceModule::AudioLayer& audioLayer) const;

  // Main initializaton and termination
  virtual InitStatus Init() RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t Terminate() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool Initialized() const;

  // Device enumeration
  virtual int16_t PlayoutDevices();
  virtual int16_t RecordingDevices();
  virtual int32_t PlayoutDeviceName(uint16_t index,
                                    char name[kAdmMaxDeviceNameSize],
                                    char guid[kAdmMaxGuidSize]);
  virtual int32_t RecordingDeviceName(uint16_t index,
                                      char name[kAdmMaxDeviceNameSize],
                                      char guid[kAdmMaxGuidSize]);

  // Device selection
  virtual int32_t SetPlayoutDevice(uint16_t index) RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetPlayoutDevice(AudioDeviceModule::WindowsDeviceType device);
  virtual int32_t SetRecordingDevice(uint16_t index);
  virtual int32_t SetRecordingDevice(
      AudioDeviceModule::WindowsDeviceType device);

  // Audio transport initialization
  virtual int32_t PlayoutIsAvailable(bool& available);
  virtual int32_t InitPlayout() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool PlayoutIsInitialized() const;
  virtual int32_t RecordingIsAvailable(bool& available);
  virtual int32_t InitRecording() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool RecordingIsInitialized() const;

  // Audio transport control
  virtual int32_t StartPlayout() RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t StopPlayout() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool Playing() const;
  virtual int32_t StartRecording() RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t StopRecording() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool Recording() const;

  // Audio mixer initialization
  virtual int32_t InitSpeaker() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool SpeakerIsInitialized() const;
  virtual int32_t InitMicrophone() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool MicrophoneIsInitialized() const;

  // Speaker volume controls
  virtual int32_t SpeakerVolumeIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetSpeakerVolume(uint32_t volume);
  virtual int32_t SpeakerVolume(uint32_t& volume) const;
  virtual int32_t MaxSpeakerVolume(uint32_t& maxVolume) const;
  virtual int32_t MinSpeakerVolume(uint32_t& minVolume) const;

  // Microphone volume controls
  virtual int32_t MicrophoneVolumeIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetMicrophoneVolume(uint32_t volume);
  virtual int32_t MicrophoneVolume(uint32_t& volume) const;
  virtual int32_t MaxMicrophoneVolume(uint32_t& maxVolume) const;
  virtual int32_t MinMicrophoneVolume(uint32_t& minVolume) const;

  // Microphone mute control
  virtual int32_t MicrophoneMuteIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetMicrophoneMute(bool enable);
  virtual int32_t MicrophoneMute(bool& enabled) const;

  // Speaker mute control
  virtual int32_t SpeakerMuteIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetSpeakerMute(bool enable);
  virtual int32_t SpeakerMute(bool& enabled) const;

  // Stereo support
  virtual int32_t StereoPlayoutIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetStereoPlayout(bool enable);
  virtual int32_t StereoPlayout(bool& enabled) const;
  virtual int32_t StereoRecordingIsAvailable(bool& available);
  virtual int32_t SetStereoRecording(bool enable);
  virtual int32_t StereoRecording(bool& enabled) const;

  // Delay information and control
  virtual int32_t PlayoutDelay(uint16_t& delayMS) const;

  virtual void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer)
      RTC_LOCKS_EXCLUDED(mutex_);

  virtual int32_t SetObserver(AudioDeviceObserver* observer) RTC_LOCKS_EXCLUDED(mutex_) {
    audio_device_module_sink_ = observer;
    return 0;
  }
  virtual int32_t GetPlayoutDevice() const;
  virtual int32_t GetRecordingDevice() const;

 private:
  int32_t InitSpeakerLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t InitMicrophoneLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  virtual int32_t MicrophoneIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t MicrophoneIsAvailableLocked(bool& available)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  virtual int32_t SpeakerIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SpeakerIsAvailableLocked(bool& available)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  static void AtomicSet32(int32_t* theValue, int32_t newValue);
  static int32_t AtomicGet32(int32_t* theValue);

  static void logCAMsg(webrtc::LoggingSeverity sev,
                       const char* msg,
                       const char* err);

  int32_t GetNumberDevices(AudioObjectPropertyScope scope,
                           AudioDeviceID scopedDeviceIds[],
                           uint32_t deviceListLength);

  int32_t GetDeviceName(AudioObjectPropertyScope scope,
                        uint16_t index,
                        webrtc::ArrayView<char> name,
                        webrtc::ArrayView<char> guid);

  int32_t InitDevice(uint16_t userDeviceIndex,
                     AudioDeviceID& deviceId,
                     bool isInput);

  // Always work with our preferred playout format inside VoE.
  // Then convert the output to the OS setting using an AudioConverter.
  OSStatus SetDesiredPlayoutFormat();

  static OSStatus objectListenerProc(
      AudioObjectID objectId,
      UInt32 numberAddresses,
      const AudioObjectPropertyAddress addresses[],
      void* clientData);

  OSStatus implObjectListenerProc(AudioObjectID objectId,
                                  UInt32 numberAddresses,
                                  const AudioObjectPropertyAddress addresses[]);

  int32_t HandleDeviceChange();
  int32_t HandleDefaultOutputDeviceChange();
  int32_t HandleDefaultInputDeviceChange();

  int32_t HandleStreamFormatChange(AudioObjectID objectId,
                                   AudioObjectPropertyAddress propertyAddress);

  int32_t HandleDataSourceChange(AudioObjectID objectId,
                                 AudioObjectPropertyAddress propertyAddress);

  int32_t HandleProcessorOverload(AudioObjectPropertyAddress propertyAddress);

  static OSStatus deviceIOProc(AudioDeviceID device,
                               const AudioTimeStamp* now,
                               const AudioBufferList* inputData,
                               const AudioTimeStamp* inputTime,
                               AudioBufferList* outputData,
                               const AudioTimeStamp* outputTime,
                               void* clientData);

  static OSStatus outConverterProc(
      AudioConverterRef audioConverter,
      UInt32* numberDataPackets,
      AudioBufferList* data,
      AudioStreamPacketDescription** dataPacketDescription,
      void* userData);

  static OSStatus inDeviceIOProc(AudioDeviceID device,
                                 const AudioTimeStamp* now,
                                 const AudioBufferList* inputData,
                                 const AudioTimeStamp* inputTime,
                                 AudioBufferList* outputData,
                                 const AudioTimeStamp* outputTime,
                                 void* clientData);

  static OSStatus inConverterProc(
      AudioConverterRef audioConverter,
      UInt32* numberDataPackets,
      AudioBufferList* data,
      AudioStreamPacketDescription** dataPacketDescription,
      void* inUserData);

  OSStatus implDeviceIOProc(const AudioBufferList* inputData,
                            const AudioTimeStamp* inputTime,
                            AudioBufferList* outputData,
                            const AudioTimeStamp* outputTime)
      RTC_LOCKS_EXCLUDED(mutex_);

  OSStatus implOutConverterProc(UInt32* numberDataPackets,
                                AudioBufferList* data);

  OSStatus implInDeviceIOProc(const AudioBufferList* inputData,
                              const AudioTimeStamp* inputTime)
      RTC_LOCKS_EXCLUDED(mutex_);

  OSStatus implInConverterProc(UInt32* numberDataPackets,
                               AudioBufferList* data);

  static void RunCapture(void*);
  static void RunRender(void*);
  bool CaptureWorkerThread();
  bool RenderWorkerThread();

  bool KeyPressed();

  AudioDeviceBuffer* _ptrAudioBuffer;

  Mutex mutex_;

  webrtc::Event _stopEventRec;
  webrtc::Event _stopEvent;

  // Only valid/running between calls to StartRecording and StopRecording.
  webrtc::PlatformThread capture_worker_thread_;

  // Only valid/running between calls to StartPlayout and StopPlayout.
  webrtc::PlatformThread render_worker_thread_;

  AudioMixerManagerMac _mixerManager;

  uint16_t _inputDeviceIndex;
  uint16_t _outputDeviceIndex;
  AudioDeviceID _inputDeviceID;
  AudioDeviceID _outputDeviceID;
  AudioDeviceIOProcID _inDeviceIOProcID;
  AudioDeviceIOProcID _deviceIOProcID;
  bool _inputDeviceIsSpecified;
  bool _outputDeviceIsSpecified;

  uint8_t _recChannels;
  uint8_t _playChannels;

  Float32* _captureBufData;
  SInt16* _renderBufData;

  SInt16 _renderConvertData[PLAY_BUF_SIZE_IN_SAMPLES];

  bool _initialized;
  bool _isShutDown;
  bool _recording;
  bool _playing;
  bool _recIsInitialized;
  bool _playIsInitialized;

  // Atomically set varaibles
  std::atomic<int32_t> _renderDeviceIsAlive;
  std::atomic<int32_t> _captureDeviceIsAlive;

  bool _twoDevices;
  bool _doStop;  // For play if not shared device or play+rec if shared device
  bool _doStopRec;  // For rec if not shared device
  bool _macBookPro;
  bool _macBookProPanRight;

  AudioConverterRef _captureConverter;
  AudioConverterRef _renderConverter;

  AudioStreamBasicDescription _outStreamFormat;
  AudioStreamBasicDescription _outDesiredFormat;
  AudioStreamBasicDescription _inStreamFormat;
  AudioStreamBasicDescription _inDesiredFormat;

  uint32_t _captureLatencyUs;
  uint32_t _renderLatencyUs;

  // Atomically set variables
  mutable std::atomic<int32_t> _captureDelayUs;
  mutable std::atomic<int32_t> _renderDelayUs;

  int32_t _renderDelayOffsetSamples;

  PaUtilRingBuffer* _paCaptureBuffer;
  PaUtilRingBuffer* _paRenderBuffer;

  semaphore_t _renderSemaphore;
  semaphore_t _captureSemaphore;

  int _captureBufSizeSamples;
  int _renderBufSizeSamples;

  // Typing detection
  // 0x5c is key "9", after that comes function keys.
  bool prev_key_state_[0x5d];

  AudioDeviceObserver *audio_device_module_sink_ = nullptr;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_MAIN_SOURCE_MAC_AUDIO_DEVICE_MAC_H_
