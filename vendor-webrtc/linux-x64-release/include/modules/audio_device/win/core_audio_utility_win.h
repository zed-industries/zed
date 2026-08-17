/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_UTILITY_WIN_H_
#define MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_UTILITY_WIN_H_

#include <audioclient.h>
#include <audiopolicy.h>
#include <avrt.h>
#include <comdef.h>
#include <mmdeviceapi.h>
#include <objbase.h>
#include <propidl.h>
#include <wrl/client.h>

#include <string>

#include "absl/strings/string_view.h"
#include "api/audio/audio_device_defines.h"
#include "api/units/time_delta.h"
#include "modules/audio_device/audio_device_name.h"
#include "rtc_base/logging.h"
#include "rtc_base/string_utils.h"

#pragma comment(lib, "Avrt.lib")

namespace webrtc {
namespace webrtc_win {

// Utility class which registers a thread with MMCSS in the constructor and
// deregisters MMCSS in the destructor. The task name is given by `task_name`.
// The Multimedia Class Scheduler service (MMCSS) enables multimedia
// applications to ensure that their time-sensitive processing receives
// prioritized access to CPU resources without denying CPU resources to
// lower-priority applications.
class ScopedMMCSSRegistration {
 public:
  const char* PriorityClassToString(DWORD priority_class) {
    switch (priority_class) {
      case ABOVE_NORMAL_PRIORITY_CLASS:
        return "ABOVE_NORMAL";
      case BELOW_NORMAL_PRIORITY_CLASS:
        return "BELOW_NORMAL";
      case HIGH_PRIORITY_CLASS:
        return "HIGH";
      case IDLE_PRIORITY_CLASS:
        return "IDLE";
      case NORMAL_PRIORITY_CLASS:
        return "NORMAL";
      case REALTIME_PRIORITY_CLASS:
        return "REALTIME";
      default:
        return "INVALID";
    }
  }

  const char* PriorityToString(int priority) {
    switch (priority) {
      case THREAD_PRIORITY_ABOVE_NORMAL:
        return "ABOVE_NORMAL";
      case THREAD_PRIORITY_BELOW_NORMAL:
        return "BELOW_NORMAL";
      case THREAD_PRIORITY_HIGHEST:
        return "HIGHEST";
      case THREAD_PRIORITY_IDLE:
        return "IDLE";
      case THREAD_PRIORITY_LOWEST:
        return "LOWEST";
      case THREAD_PRIORITY_NORMAL:
        return "NORMAL";
      case THREAD_PRIORITY_TIME_CRITICAL:
        return "TIME_CRITICAL";
      default:
        // Can happen in combination with REALTIME_PRIORITY_CLASS.
        return "INVALID";
    }
  }

  explicit ScopedMMCSSRegistration(const wchar_t* task_name) {
    RTC_DLOG(LS_INFO) << "ScopedMMCSSRegistration: "
                      << webrtc::ToUtf8(task_name);
    // Register the calling thread with MMCSS for the supplied `task_name`.
    DWORD mmcss_task_index = 0;
    mmcss_handle_ = AvSetMmThreadCharacteristicsW(task_name, &mmcss_task_index);
    if (mmcss_handle_ == nullptr) {
      RTC_LOG(LS_ERROR) << "Failed to enable MMCSS on this thread: "
                        << GetLastError();
    } else {
      const DWORD priority_class = GetPriorityClass(GetCurrentProcess());
      const int priority = GetThreadPriority(GetCurrentThread());
      RTC_DLOG(LS_INFO) << "priority class: "
                        << PriorityClassToString(priority_class) << "("
                        << priority_class << ")";
      RTC_DLOG(LS_INFO) << "priority: " << PriorityToString(priority) << "("
                        << priority << ")";
    }
  }

  ~ScopedMMCSSRegistration() {
    if (Succeeded()) {
      // Deregister with MMCSS.
      RTC_DLOG(LS_INFO) << "~ScopedMMCSSRegistration";
      AvRevertMmThreadCharacteristics(mmcss_handle_);
    }
  }

  ScopedMMCSSRegistration(const ScopedMMCSSRegistration&) = delete;
  ScopedMMCSSRegistration& operator=(const ScopedMMCSSRegistration&) = delete;

  bool Succeeded() const { return mmcss_handle_ != nullptr; }

 private:
  HANDLE mmcss_handle_ = nullptr;
};

// A PROPVARIANT that is automatically initialized and cleared upon respective
// construction and destruction of this class.
class ScopedPropVariant {
 public:
  ScopedPropVariant() { PropVariantInit(&pv_); }

  ~ScopedPropVariant() { Reset(); }

  ScopedPropVariant(const ScopedPropVariant&) = delete;
  ScopedPropVariant& operator=(const ScopedPropVariant&) = delete;
  bool operator==(const ScopedPropVariant&) const = delete;
  bool operator!=(const ScopedPropVariant&) const = delete;

  // Returns a pointer to the underlying PROPVARIANT for use as an out param in
  // a function call.
  PROPVARIANT* Receive() {
    RTC_DCHECK_EQ(pv_.vt, VT_EMPTY);
    return &pv_;
  }

  // Clears the instance to prepare it for re-use (e.g., via Receive).
  void Reset() {
    if (pv_.vt != VT_EMPTY) {
      HRESULT result = PropVariantClear(&pv_);
      RTC_DCHECK_EQ(result, S_OK);
    }
  }

  const PROPVARIANT& get() const { return pv_; }
  const PROPVARIANT* ptr() const { return &pv_; }

 private:
  PROPVARIANT pv_;
};

// Simple scoped memory releaser class for COM allocated memory.
template <typename T>
class ScopedCoMem {
 public:
  ScopedCoMem() : mem_ptr_(nullptr) {}

  ~ScopedCoMem() { Reset(nullptr); }

  ScopedCoMem(const ScopedCoMem&) = delete;
  ScopedCoMem& operator=(const ScopedCoMem&) = delete;

  T** operator&() {                   // NOLINT
    RTC_DCHECK(mem_ptr_ == nullptr);  // To catch memory leaks.
    return &mem_ptr_;
  }

  operator T*() { return mem_ptr_; }

  T* operator->() {
    RTC_DCHECK(mem_ptr_ != nullptr);
    return mem_ptr_;
  }

  const T* operator->() const {
    RTC_DCHECK(mem_ptr_ != nullptr);
    return mem_ptr_;
  }

  explicit operator bool() const { return mem_ptr_; }

  friend bool operator==(const ScopedCoMem& lhs, std::nullptr_t) {
    return lhs.Get() == nullptr;
  }

  friend bool operator==(std::nullptr_t, const ScopedCoMem& rhs) {
    return rhs.Get() == nullptr;
  }

  friend bool operator!=(const ScopedCoMem& lhs, std::nullptr_t) {
    return lhs.Get() != nullptr;
  }

  friend bool operator!=(std::nullptr_t, const ScopedCoMem& rhs) {
    return rhs.Get() != nullptr;
  }

  void Reset(T* ptr) {
    if (mem_ptr_)
      CoTaskMemFree(mem_ptr_);
    mem_ptr_ = ptr;
  }

  T* Get() const { return mem_ptr_; }

 private:
  T* mem_ptr_;
};

// A HANDLE that is automatically initialized and closed upon respective
// construction and destruction of this class.
class ScopedHandle {
 public:
  ScopedHandle() : handle_(nullptr) {}
  explicit ScopedHandle(HANDLE h) : handle_(nullptr) { Set(h); }

  ~ScopedHandle() { Close(); }

  ScopedHandle& operator=(const ScopedHandle&) = delete;
  bool operator==(const ScopedHandle&) const = delete;
  bool operator!=(const ScopedHandle&) const = delete;

  // Use this instead of comparing to INVALID_HANDLE_VALUE.
  bool IsValid() const { return handle_ != nullptr; }

  void Set(HANDLE new_handle) {
    Close();
    // Windows is inconsistent about invalid handles.
    // See https://blogs.msdn.microsoft.com/oldnewthing/20040302-00/?p=40443
    // for details.
    if (new_handle != INVALID_HANDLE_VALUE) {
      handle_ = new_handle;
    }
  }

  HANDLE Get() const { return handle_; }

  operator HANDLE() const { return handle_; }

  void Close() {
    if (handle_) {
      if (!::CloseHandle(handle_)) {
        RTC_DCHECK_NOTREACHED();
      }
      handle_ = nullptr;
    }
  }

 private:
  HANDLE handle_;
};

// Utility methods for the Core Audio API on Windows.
// Always ensure that Core Audio is supported before using these methods.
// Use webrtc_win::core_audio_utility::IsSupported() for this purpose.
// Also, all methods must be called on a valid COM thread. This can be done
// by using the ScopedCOMInitializer helper class.
// These methods are based on media::CoreAudioUtil in Chrome.
namespace core_audio_utility {

// Helper class which automates casting between WAVEFORMATEX and
// WAVEFORMATEXTENSIBLE raw pointers using implicit constructors and
// operator overloading. Note that, no memory is allocated by this utility
// structure. It only serves as a handle (or a wrapper) of the structure
// provided to it at construction.
class WaveFormatWrapper {
 public:
  WaveFormatWrapper(WAVEFORMATEXTENSIBLE* p)
      : ptr_(reinterpret_cast<WAVEFORMATEX*>(p)) {}
  WaveFormatWrapper(WAVEFORMATEX* p) : ptr_(p) {}
  ~WaveFormatWrapper() = default;

  operator WAVEFORMATEX*() const { return ptr_; }
  WAVEFORMATEX* operator->() const { return ptr_; }
  WAVEFORMATEX* get() const { return ptr_; }
  WAVEFORMATEXTENSIBLE* GetExtensible() const;

  bool IsExtensible() const;
  bool IsPcm() const;
  bool IsFloat() const;
  size_t size() const;

 private:
  WAVEFORMATEX* ptr_;
};

// Returns true if Windows Core Audio is supported.
// Always verify that this method returns true before using any of the
// other methods in this class.
bool IsSupported();

// Returns true if Multimedia Class Scheduler service (MMCSS) is supported.
// The MMCSS enables multimedia applications to ensure that their time-sensitive
// processing receives prioritized access to CPU resources without denying CPU
// resources to lower-priority applications.
bool IsMMCSSSupported();

// The MMDevice API lets clients discover the audio endpoint devices in the
// system and determine which devices are suitable for the application to use.
// Header file Mmdeviceapi.h defines the interfaces in the MMDevice API.

// Number of active audio devices in the specified data flow direction.
// Set `data_flow` to eAll to retrieve the total number of active audio
// devices.
int NumberOfActiveDevices(EDataFlow data_flow);

// Returns 1, 2, or 3 depending on what version of IAudioClient the platform
// supports.
// Example: IAudioClient2 is supported on Windows 8 and higher => 2 is returned.
uint32_t GetAudioClientVersion();

// Creates an IMMDeviceEnumerator interface which provides methods for
// enumerating audio endpoint devices.
// TODO(henrika): IMMDeviceEnumerator::RegisterEndpointNotificationCallback.
Microsoft::WRL::ComPtr<IMMDeviceEnumerator> CreateDeviceEnumerator();

// These functions return the unique device id of the default or
// communications input/output device, or an empty string if no such device
// exists or if the device has been disabled.
std::string GetDefaultInputDeviceID();
std::string GetDefaultOutputDeviceID();
std::string GetCommunicationsInputDeviceID();
std::string GetCommunicationsOutputDeviceID();

// Creates an IMMDevice interface corresponding to the unique device id in
// `device_id`, or by data-flow direction and role if `device_id` is set to
// AudioDeviceName::kDefaultDeviceId.
Microsoft::WRL::ComPtr<IMMDevice> CreateDevice(absl::string_view device_id,
                                               EDataFlow data_flow,
                                               ERole role);

// Returns the unique ID and user-friendly name of a given endpoint device.
// Example: "{0.0.1.00000000}.{8db6020f-18e3-4f25-b6f5-7726c9122574}", and
//          "Microphone (Realtek High Definition Audio)".
webrtc::AudioDeviceName GetDeviceName(IMMDevice* device);

// Gets the user-friendly name of the endpoint device which is represented
// by a unique id in `device_id`, or by data-flow direction and role if
// `device_id` is set to AudioDeviceName::kDefaultDeviceId.
std::string GetFriendlyName(absl::string_view device_id,
                            EDataFlow data_flow,
                            ERole role);

// Query if the audio device is a rendering device or a capture device.
EDataFlow GetDataFlow(IMMDevice* device);

// Enumerates all input devices and adds the names (friendly name and unique
// device id) to the list in `device_names`.
bool GetInputDeviceNames(webrtc::AudioDeviceNames* device_names);

// Enumerates all output devices and adds the names (friendly name and unique
// device id) to the list in `device_names`.
bool GetOutputDeviceNames(webrtc::AudioDeviceNames* device_names);

// The Windows Audio Session API (WASAPI) enables client applications to
// manage the flow of audio data between the application and an audio endpoint
// device. Header files Audioclient.h and Audiopolicy.h define the WASAPI
// interfaces.

// Creates an IAudioSessionManager2 interface for the specified `device`.
// This interface provides access to e.g. the IAudioSessionEnumerator
Microsoft::WRL::ComPtr<IAudioSessionManager2> CreateSessionManager2(
    IMMDevice* device);

// Creates an IAudioSessionEnumerator interface for the specified `device`.
// The client can use the interface to enumerate audio sessions on the audio
// device
Microsoft::WRL::ComPtr<IAudioSessionEnumerator> CreateSessionEnumerator(
    IMMDevice* device);

// Number of active audio sessions for the given `device`. Expired or inactive
// sessions are not included.
int NumberOfActiveSessions(IMMDevice* device);

// Creates an IAudioClient instance for a specific device or the default
// device specified by data-flow direction and role.
Microsoft::WRL::ComPtr<IAudioClient> CreateClient(absl::string_view device_id,
                                                  EDataFlow data_flow,
                                                  ERole role);
Microsoft::WRL::ComPtr<IAudioClient2> CreateClient2(absl::string_view device_id,
                                                    EDataFlow data_flow,
                                                    ERole role);
Microsoft::WRL::ComPtr<IAudioClient3> CreateClient3(absl::string_view device_id,
                                                    EDataFlow data_flow,
                                                    ERole role);

// Sets the AudioCategory_Communications category. Should be called before
// GetSharedModeMixFormat() and IsFormatSupported(). The `client` argument must
// be an IAudioClient2 or IAudioClient3 interface pointer, hence only supported
// on Windows 8 and above.
// TODO(henrika): evaluate effect (if any).
HRESULT SetClientProperties(IAudioClient2* client);

// Returns the buffer size limits of the hardware audio engine in
// 100-nanosecond units given a specified `format`. Does not require prior
// audio stream initialization. The `client` argument must be an IAudioClient2
// or IAudioClient3 interface pointer, hence only supported on Windows 8 and
// above.
// TODO(henrika): always fails with AUDCLNT_E_OFFLOAD_MODE_ONLY.
HRESULT GetBufferSizeLimits(IAudioClient2* client,
                            const WAVEFORMATEXTENSIBLE* format,
                            REFERENCE_TIME* min_buffer_duration,
                            REFERENCE_TIME* max_buffer_duration);

// Get the mix format that the audio engine uses internally for processing
// of shared-mode streams. The client can call this method before calling
// IAudioClient::Initialize. When creating a shared-mode stream for an audio
// endpoint device, the Initialize method always accepts the stream format
// obtained by this method.
HRESULT GetSharedModeMixFormat(IAudioClient* client,
                               WAVEFORMATEXTENSIBLE* format);

// Returns true if the specified `client` supports the format in `format`
// for the given `share_mode` (shared or exclusive). The client can call this
// method before calling IAudioClient::Initialize.
bool IsFormatSupported(IAudioClient* client,
                       AUDCLNT_SHAREMODE share_mode,
                       const WAVEFORMATEXTENSIBLE* format);

// For a shared-mode stream, the audio engine periodically processes the
// data in the endpoint buffer at the period obtained in `device_period`.
// For an exclusive mode stream, `device_period` corresponds to the minimum
// time interval between successive processing by the endpoint device.
// This period plus the stream latency between the buffer and endpoint device
// represents the minimum possible latency that an audio application can
// achieve. The time in `device_period` is expressed in 100-nanosecond units.
HRESULT GetDevicePeriod(IAudioClient* client,
                        AUDCLNT_SHAREMODE share_mode,
                        REFERENCE_TIME* device_period);

// Returns the range of periodicities supported by the engine for the specified
// stream `format`. The periodicity of the engine is the rate at which the
// engine wakes an event-driven audio client to transfer audio data to or from
// the engine. Can be used for low-latency support on some devices.
// The `client` argument must be an IAudioClient3 interface pointer, hence only
// supported on Windows 10 and above.
HRESULT GetSharedModeEnginePeriod(IAudioClient3* client3,
                                  const WAVEFORMATEXTENSIBLE* format,
                                  uint32_t* default_period_in_frames,
                                  uint32_t* fundamental_period_in_frames,
                                  uint32_t* min_period_in_frames,
                                  uint32_t* max_period_in_frames);

// Get the preferred audio parameters for the given `client` corresponding to
// the stream format that the audio engine uses for its internal processing of
// shared-mode streams. The acquired values should only be utilized for shared
// mode streamed since there are no preferred settings for an exclusive mode
// stream.
HRESULT GetPreferredAudioParameters(IAudioClient* client,
                                    webrtc::AudioParameters* params);
// As above but override the preferred sample rate and use `sample_rate`
// instead. Intended mainly for testing purposes and in combination with rate
// conversion.
HRESULT GetPreferredAudioParameters(IAudioClient* client,
                                    webrtc::AudioParameters* params,
                                    uint32_t sample_rate);

// After activating an IAudioClient interface on an audio endpoint device,
// the client must initialize it once, and only once, to initialize the audio
// stream between the client and the device. In shared mode, the client
// connects indirectly through the audio engine which does the mixing.
// If a valid event is provided in `event_handle`, the client will be
// initialized for event-driven buffer handling. If `event_handle` is set to
// nullptr, event-driven buffer handling is not utilized. To achieve the
// minimum stream latency between the client application and audio endpoint
// device, set `buffer_duration` to 0. A client has the option of requesting a
// buffer size that is larger than what is strictly necessary to make timing
// glitches rare or nonexistent. Increasing the buffer size does not necessarily
// increase the stream latency. Each unit of reference time is 100 nanoseconds.
// The `auto_convert_pcm` parameter can be used for testing purposes to ensure
// that the sample rate of the client side does not have to match the audio
// engine mix format. If `auto_convert_pcm` is set to true, a rate converter
// will be inserted to convert between the sample rate in `format` and the
// preferred rate given by GetPreferredAudioParameters().
// The output parameter `endpoint_buffer_size` contains the size of the
// endpoint buffer and it is expressed as the number of audio frames the
// buffer can hold.
HRESULT SharedModeInitialize(IAudioClient* client,
                             const WAVEFORMATEXTENSIBLE* format,
                             HANDLE event_handle,
                             REFERENCE_TIME buffer_duration,
                             bool auto_convert_pcm,
                             uint32_t* endpoint_buffer_size);

// Works as SharedModeInitialize() but adds support for using smaller engine
// periods than the default period.
// The `client` argument must be an IAudioClient3 interface pointer, hence only
// supported on Windows 10 and above.
// TODO(henrika): can probably be merged into SharedModeInitialize() to avoid
// duplicating code. Keeping as separate method for now until decided if we
// need low-latency support.
HRESULT SharedModeInitializeLowLatency(IAudioClient3* client,
                                       const WAVEFORMATEXTENSIBLE* format,
                                       HANDLE event_handle,
                                       uint32_t period_in_frames,
                                       bool auto_convert_pcm,
                                       uint32_t* endpoint_buffer_size);

// Creates an IAudioRenderClient client for an existing IAudioClient given by
// `client`. The IAudioRenderClient interface enables a client to write
// output data to a rendering endpoint buffer. The methods in this interface
// manage the movement of data packets that contain audio-rendering data.
Microsoft::WRL::ComPtr<IAudioRenderClient> CreateRenderClient(
    IAudioClient* client);

// Creates an IAudioCaptureClient client for an existing IAudioClient given by
// `client`. The IAudioCaptureClient interface enables a client to read
// input data from a capture endpoint buffer. The methods in this interface
// manage the movement of data packets that contain capture data.
Microsoft::WRL::ComPtr<IAudioCaptureClient> CreateCaptureClient(
    IAudioClient* client);

// Creates an IAudioClock interface for an existing IAudioClient given by
// `client`. The IAudioClock interface enables a client to monitor a stream's
// data rate and the current position in the stream.
Microsoft::WRL::ComPtr<IAudioClock> CreateAudioClock(IAudioClient* client);

// Creates an AudioSessionControl interface for an existing IAudioClient given
// by `client`. The IAudioControl interface enables a client to configure the
// control parameters for an audio session and to monitor events in the session.
Microsoft::WRL::ComPtr<IAudioSessionControl> CreateAudioSessionControl(
    IAudioClient* client);

// Creates an ISimpleAudioVolume interface for an existing IAudioClient given by
// `client`. This interface enables a client to control the master volume level
// of an active audio session.
Microsoft::WRL::ComPtr<ISimpleAudioVolume> CreateSimpleAudioVolume(
    IAudioClient* client);

// Fills up the endpoint rendering buffer with silence for an existing
// IAudioClient given by `client` and a corresponding IAudioRenderClient
// given by `render_client`.
bool FillRenderEndpointBufferWithSilence(IAudioClient* client,
                                         IAudioRenderClient* render_client);

// Prints/logs all fields of the format structure in `format`.
// Also supports extended versions (WAVEFORMATEXTENSIBLE).
std::string WaveFormatToString(WaveFormatWrapper format);

// Converts Windows internal REFERENCE_TIME (100 nanosecond units) into
// generic webrtc::TimeDelta which then can be converted to any time unit.
webrtc::TimeDelta ReferenceTimeToTimeDelta(REFERENCE_TIME time);

// Converts size expressed in number of audio frames, `num_frames`, into
// milliseconds given a specified `sample_rate`.
double FramesToMilliseconds(uint32_t num_frames, uint16_t sample_rate);

// Converts a COM error into a human-readable string.
std::string ErrorToString(const _com_error& error);

}  // namespace core_audio_utility
}  // namespace webrtc_win
}  // namespace webrtc

#endif  //  MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_UTILITY_WIN_H_
