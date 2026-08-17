/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_WIN_AUDIO_DEVICE_CORE_WIN_H_
#define MODULES_AUDIO_DEVICE_WIN_AUDIO_DEVICE_CORE_WIN_H_

#if (_MSC_VER >= 1400)  // only include for VS 2005 and higher

#include <wmcodecdsp.h>  // CLSID_CWMAudioAEC
//(must be before audioclient.h)

#include <audioclient.h>  // WASAPI
#include <audiopolicy.h>
#include <avrt.h>  // Avrt
#include <endpointvolume.h>
#include <mediaobj.h>     // IMediaObject
#include <mmdeviceapi.h>  // MMDevice
#include <comdef.h>
#include <objbase.h>

#include "api/scoped_refptr.h"
#include "modules/audio_device/audio_device_generic.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/win/scoped_com_initializer.h"
#include "rtc_base/win32.h"

// Use Multimedia Class Scheduler Service (MMCSS) to boost the thread priority
#pragma comment(lib, "avrt.lib")
// AVRT function pointers
typedef BOOL(WINAPI* PAvRevertMmThreadCharacteristics)(HANDLE);
typedef HANDLE(WINAPI* PAvSetMmThreadCharacteristicsA)(LPCSTR, LPDWORD);
typedef BOOL(WINAPI* PAvSetMmThreadPriority)(HANDLE, AVRT_PRIORITY);

namespace webrtc {

const float MAX_CORE_SPEAKER_VOLUME = 255.0f;
const float MIN_CORE_SPEAKER_VOLUME = 0.0f;
const float MAX_CORE_MICROPHONE_VOLUME = 255.0f;
const float MIN_CORE_MICROPHONE_VOLUME = 0.0f;
const uint16_t CORE_SPEAKER_VOLUME_STEP_SIZE = 1;
const uint16_t CORE_MICROPHONE_VOLUME_STEP_SIZE = 1;

class AudioDeviceWindowsCore : public AudioDeviceGeneric {
 public:
  AudioDeviceWindowsCore();
  ~AudioDeviceWindowsCore();

  class DeviceStateListener : public IMMNotificationClient {
   public:
    virtual ~DeviceStateListener() = default;
    HRESULT __stdcall OnDeviceStateChanged(LPCWSTR pwstrDeviceId,
                                           DWORD dwNewState) override;
    HRESULT __stdcall OnDeviceAdded(LPCWSTR pwstrDeviceId) override;

    HRESULT __stdcall OnDeviceRemoved(LPCWSTR pwstrDeviceId) override;

    HRESULT
    __stdcall OnDefaultDeviceChanged(EDataFlow flow,
                                     ERole role,
                                     LPCWSTR pwstrDefaultDeviceId) override;

    HRESULT __stdcall OnPropertyValueChanged(LPCWSTR pwstrDeviceId,
                                             const PROPERTYKEY key) override;
    // IUnknown (required by IMMNotificationClient).
    ULONG __stdcall AddRef() override;
    ULONG __stdcall Release() override;
    HRESULT __stdcall QueryInterface(REFIID iid, void** object) override;

    void SetObserver(AudioDeviceObserver *sink);

   private:
    LONG ref_count_ = 1;
    AudioDeviceObserver *callback_ = nullptr;
  };

  static bool CoreAudioIsSupported();

  // Retrieve the currently utilized audio layer
  virtual int32_t ActiveAudioLayer(
      AudioDeviceModule::AudioLayer& audioLayer) const;

  // Main initializaton and termination
  virtual InitStatus Init() RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t Terminate() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool Initialized() const;

  // Device enumeration
  virtual int16_t PlayoutDevices() RTC_LOCKS_EXCLUDED(mutex_);
  virtual int16_t RecordingDevices() RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t PlayoutDeviceName(uint16_t index,
                                    char name[kAdmMaxDeviceNameSize],
                                    char guid[kAdmMaxGuidSize])
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t RecordingDeviceName(uint16_t index,
                                      char name[kAdmMaxDeviceNameSize],
                                      char guid[kAdmMaxGuidSize])
      RTC_LOCKS_EXCLUDED(mutex_);

  // Device selection
  virtual int32_t SetPlayoutDevice(uint16_t index) RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetPlayoutDevice(AudioDeviceModule::WindowsDeviceType device);
  virtual int32_t SetRecordingDevice(uint16_t index) RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetRecordingDevice(
      AudioDeviceModule::WindowsDeviceType device) RTC_LOCKS_EXCLUDED(mutex_);

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
  virtual int32_t StopRecording();
  virtual bool Recording() const;

  // Audio mixer initialization
  virtual int32_t InitSpeaker() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool SpeakerIsInitialized() const;
  virtual int32_t InitMicrophone() RTC_LOCKS_EXCLUDED(mutex_);
  virtual bool MicrophoneIsInitialized() const;

  // Speaker volume controls
  virtual int32_t SpeakerVolumeIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetSpeakerVolume(uint32_t volume) RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SpeakerVolume(uint32_t& volume) const
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t MaxSpeakerVolume(uint32_t& maxVolume) const;
  virtual int32_t MinSpeakerVolume(uint32_t& minVolume) const;

  // Microphone volume controls
  virtual int32_t MicrophoneVolumeIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetMicrophoneVolume(uint32_t volume)
      RTC_LOCKS_EXCLUDED(mutex_, volume_mutex_);
  virtual int32_t MicrophoneVolume(uint32_t& volume) const
      RTC_LOCKS_EXCLUDED(mutex_, volume_mutex_);
  virtual int32_t MaxMicrophoneVolume(uint32_t& maxVolume) const;
  virtual int32_t MinMicrophoneVolume(uint32_t& minVolume) const;

  // Speaker mute control
  virtual int32_t SpeakerMuteIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetSpeakerMute(bool enable) RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SpeakerMute(bool& enabled) const;

  // Microphone mute control
  virtual int32_t MicrophoneMuteIsAvailable(bool& available)
      RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t SetMicrophoneMute(bool enable);
  virtual int32_t MicrophoneMute(bool& enabled) const;

  // Stereo support
  virtual int32_t StereoPlayoutIsAvailable(bool& available);
  virtual int32_t SetStereoPlayout(bool enable) RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t StereoPlayout(bool& enabled) const;
  virtual int32_t StereoRecordingIsAvailable(bool& available);
  virtual int32_t SetStereoRecording(bool enable) RTC_LOCKS_EXCLUDED(mutex_);
  virtual int32_t StereoRecording(bool& enabled) const
      RTC_LOCKS_EXCLUDED(mutex_);

  // Delay information and control
  virtual int32_t PlayoutDelay(uint16_t& delayMS) const
      RTC_LOCKS_EXCLUDED(mutex_);

  virtual bool BuiltInAECIsAvailable() const;

  virtual int32_t EnableBuiltInAEC(bool enable);

  virtual int32_t SetObserver(AudioDeviceObserver* observer);

 public:
  virtual void AttachAudioBuffer(AudioDeviceBuffer* audioBuffer);

 private:
  bool KeyPressed() const;

 private:  // avrt function pointers
  PAvRevertMmThreadCharacteristics _PAvRevertMmThreadCharacteristics;
  PAvSetMmThreadCharacteristicsA _PAvSetMmThreadCharacteristicsA;
  PAvSetMmThreadPriority _PAvSetMmThreadPriority;
  HMODULE _avrtLibrary;
  bool _winSupportAvrt;

 private:  // thread functions
  int32_t InitSpeakerLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int32_t InitMicrophoneLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int16_t PlayoutDevicesLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  int16_t RecordingDevicesLocked() RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  DWORD InitCaptureThreadPriority();
  void RevertCaptureThreadPriority();
  static DWORD WINAPI WSAPICaptureThread(LPVOID context);
  DWORD DoCaptureThread();

  static DWORD WINAPI WSAPICaptureThreadPollDMO(LPVOID context);
  DWORD DoCaptureThreadPollDMO() RTC_LOCKS_EXCLUDED(mutex_);

  static DWORD WINAPI WSAPIRenderThread(LPVOID context);
  DWORD DoRenderThread();

  void _Lock();
  void _UnLock();

  int SetDMOProperties();

  int SetBoolProperty(IPropertyStore* ptrPS,
                      REFPROPERTYKEY key,
                      VARIANT_BOOL value);

  int SetVtI4Property(IPropertyStore* ptrPS, REFPROPERTYKEY key, LONG value);

  int32_t _EnumerateEndpointDevicesAll(EDataFlow dataFlow) const;
  void _TraceCOMError(HRESULT hr) const;

  int32_t _RefreshDeviceList(EDataFlow dir);
  int16_t _DeviceListCount(EDataFlow dir);
  int32_t _GetDefaultDeviceName(EDataFlow dir,
                                ERole role,
                                LPWSTR szBuffer,
                                int bufferLen);
  int32_t _GetListDeviceName(EDataFlow dir,
                             int index,
                             LPWSTR szBuffer,
                             int bufferLen);
  int32_t _GetDeviceName(IMMDevice* pDevice, LPWSTR pszBuffer, int bufferLen);
  int32_t _GetListDeviceID(EDataFlow dir,
                           int index,
                           LPWSTR szBuffer,
                           int bufferLen);
  int32_t _GetDefaultDeviceID(EDataFlow dir,
                              ERole role,
                              LPWSTR szBuffer,
                              int bufferLen);
  int32_t _GetDefaultDeviceIndex(EDataFlow dir, ERole role, int* index);
  int32_t _GetDeviceID(IMMDevice* pDevice, LPWSTR pszBuffer, int bufferLen);
  int32_t _GetDefaultDevice(EDataFlow dir, ERole role, IMMDevice** ppDevice);
  int32_t _GetListDevice(EDataFlow dir, int index, IMMDevice** ppDevice);

  int32_t InitRecordingDMO();

  ScopedCOMInitializer _comInit;
  AudioDeviceBuffer* _ptrAudioBuffer;
  mutable Mutex mutex_;
  mutable Mutex volume_mutex_ RTC_ACQUIRED_AFTER(mutex_);

  IMMDeviceEnumerator* _ptrEnumerator;
  IMMDeviceCollection* _ptrRenderCollection;
  IMMDeviceCollection* _ptrCaptureCollection;
  IMMDevice* _ptrDeviceOut;
  IMMDevice* _ptrDeviceIn;

  IAudioClient* _ptrClientOut;
  IAudioClient* _ptrClientIn;
  IAudioRenderClient* _ptrRenderClient;
  IAudioCaptureClient* _ptrCaptureClient;
  IAudioEndpointVolume* _ptrCaptureVolume;
  ISimpleAudioVolume* _ptrRenderSimpleVolume;

  DeviceStateListener *_deviceStateListener = nullptr;
  // DirectX Media Object (DMO) for the built-in AEC.
  webrtc::scoped_refptr<IMediaObject> _dmo;
  webrtc::scoped_refptr<IMediaBuffer> _mediaBuffer;
  bool _builtInAecEnabled;

  HANDLE _hRenderSamplesReadyEvent;
  HANDLE _hPlayThread;
  HANDLE _hRenderStartedEvent;
  HANDLE _hShutdownRenderEvent;

  HANDLE _hCaptureSamplesReadyEvent;
  HANDLE _hRecThread;
  HANDLE _hCaptureStartedEvent;
  HANDLE _hShutdownCaptureEvent;

  HANDLE _hMmTask;

  UINT _playAudioFrameSize;
  uint32_t _playSampleRate;
  uint32_t _devicePlaySampleRate;
  uint32_t _playBlockSize;
  uint32_t _devicePlayBlockSize;
  uint32_t _playChannels;
  uint32_t _sndCardPlayDelay;
  UINT64 _writtenSamples;
  UINT64 _readSamples;

  UINT _recAudioFrameSize;
  uint32_t _recSampleRate;
  uint32_t _recBlockSize;
  uint32_t _recChannels;

  uint16_t _recChannelsPrioList[3];
  uint16_t _playChannelsPrioList[2];

  LARGE_INTEGER _perfCounterFreq;
  double _perfCounterFactor;

 private:
  bool _initialized;
  bool _recording;
  bool _playing;
  bool _recIsInitialized;
  bool _playIsInitialized;
  bool _speakerIsInitialized;
  bool _microphoneIsInitialized;

  bool _usingInputDeviceIndex;
  bool _usingOutputDeviceIndex;
  AudioDeviceModule::WindowsDeviceType _inputDevice;
  AudioDeviceModule::WindowsDeviceType _outputDevice;
  uint16_t _inputDeviceIndex;
  uint16_t _outputDeviceIndex;
};

#endif  // #if (_MSC_VER >= 1400)

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_WIN_AUDIO_DEVICE_CORE_WIN_H_
