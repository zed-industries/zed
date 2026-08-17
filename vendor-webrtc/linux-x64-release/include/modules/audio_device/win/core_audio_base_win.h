/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_BASE_WIN_H_
#define MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_BASE_WIN_H_

#include <atomic>
#include <functional>
#include <memory>
#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "api/sequence_checker.h"
#include "modules/audio_device/win/core_audio_utility_win.h"
#include "rtc_base/platform_thread.h"

namespace webrtc {

class AudioDeviceBuffer;
class FineAudioBuffer;

namespace webrtc_win {

// Serves as base class for CoreAudioInput and CoreAudioOutput and supports
// device handling and audio streaming where the direction (input or output)
// is set at constructions by the parent.
// The IAudioSessionEvents interface provides notifications of session-related
// events such as changes in the volume level, display name, and session state.
// This class does not use the default ref-counting memory management method
// provided by IUnknown: calling CoreAudioBase::Release() will not delete the
// object. The client will receive notification from the session manager on
// a separate thread owned and controlled by the manager.
// TODO(henrika): investigate if CoreAudioBase should implement
// IMMNotificationClient as well (might improve support for device changes).
class CoreAudioBase : public IAudioSessionEvents {
 public:
  enum class Direction {
    kInput,
    kOutput,
  };

  // TODO(henrika): add more error types.
  enum class ErrorType {
    kStreamDisconnected,
  };

  template <typename T>
  auto as_integer(T const value) -> typename std::underlying_type<T>::type {
    return static_cast<typename std::underlying_type<T>::type>(value);
  }

  // Callback definition for notifications of new audio data. For input clients,
  // it means that "new audio data has now been captured", and for output
  // clients, "the output layer now needs new audio data".
  typedef std::function<bool(uint64_t device_frequency)> OnDataCallback;

  // Callback definition for notifications of run-time error messages. It can
  // be called e.g. when an active audio device is removed and an audio stream
  // is disconnected (`error` is then set to kStreamDisconnected). Both input
  // and output clients implements OnErrorCallback() and will trigger an
  // internal restart sequence for kStreamDisconnected.
  // This method is currently always called on the audio thread.
  // TODO(henrika): add support for more error types.
  typedef std::function<bool(ErrorType error)> OnErrorCallback;

  void ThreadRun();

  CoreAudioBase(const CoreAudioBase&) = delete;
  CoreAudioBase& operator=(const CoreAudioBase&) = delete;

 protected:
  explicit CoreAudioBase(Direction direction,
                         bool automatic_restart,
                         OnDataCallback data_callback,
                         OnErrorCallback error_callback);
  ~CoreAudioBase();

  std::string GetDeviceID(int index) const;
  int SetDevice(int index);
  int DeviceName(int index, std::string* name, std::string* guid) const;

  // Checks if the current device ID is no longer in use (e.g. due to a
  // disconnected stream), and if so, switches device to the default audio
  // device. Called on the audio thread during restart attempts.
  bool SwitchDeviceIfNeeded();

  bool Init();
  bool Start();
  bool Stop();
  bool IsVolumeControlAvailable(bool* available) const;
  bool Restart();

  Direction direction() const { return direction_; }
  bool automatic_restart() const { return automatic_restart_; }

  // Releases all allocated COM resources in the base class.
  void ReleaseCOMObjects();

  // Returns number of active devices given the specified `direction_` set
  // by the parent (input or output).
  int NumberOfActiveDevices() const;

  // Returns total number of enumerated audio devices which is the sum of all
  // active devices plus two extra (one default and one default
  // communications). The value in `direction_` determines if capture or
  // render devices are counted.
  int NumberOfEnumeratedDevices() const;

  bool IsInput() const;
  bool IsOutput() const;
  bool IsDefaultDevice(int index) const;
  bool IsDefaultCommunicationsDevice(int index) const;
  bool IsDefaultDeviceId(absl::string_view device_id) const;
  bool IsDefaultCommunicationsDeviceId(absl::string_view device_id) const;
  EDataFlow GetDataFlow() const;
  bool IsRestarting() const;
  int64_t TimeSinceStart() const;

  // TODO(henrika): is the existing thread checker in WindowsAudioDeviceModule
  // sufficient? As is, we have one top-level protection and then a second
  // level here. In addition, calls to Init(), Start() and Stop() are not
  // included to allow for support of internal restart (where these methods are
  // called on the audio thread).
  SequenceChecker thread_checker_;
  SequenceChecker thread_checker_audio_;
  AudioDeviceBuffer* audio_device_buffer_ = nullptr;
  bool initialized_ = false;
  WAVEFORMATEXTENSIBLE format_ = {};
  uint32_t endpoint_buffer_size_frames_ = 0;
  Microsoft::WRL::ComPtr<IAudioClock> audio_clock_;
  Microsoft::WRL::ComPtr<IAudioClient> audio_client_;
  bool is_active_ = false;
  int64_t num_data_callbacks_ = 0;
  int latency_ms_ = 0;
  std::optional<uint32_t> sample_rate_;

 private:
  const Direction direction_;
  const bool automatic_restart_;
  const OnDataCallback on_data_callback_;
  const OnErrorCallback on_error_callback_;
  ScopedHandle audio_samples_event_;
  ScopedHandle stop_event_;
  ScopedHandle restart_event_;
  int64_t start_time_ = 0;
  std::string device_id_;
  int device_index_ = -1;
  // Used by the IAudioSessionEvents implementations. Currently only utilized
  // for debugging purposes.
  LONG ref_count_ = 1;
  // Set when restart process starts and cleared when restart stops
  // successfully. Accessed atomically.
  std::atomic<bool> is_restarting_;
  webrtc::PlatformThread audio_thread_;
  Microsoft::WRL::ComPtr<IAudioSessionControl> audio_session_control_;

  void StopThread();
  AudioSessionState GetAudioSessionState() const;

  // Called on the audio thread when a restart event has been set.
  // It will then trigger calls to the installed error callbacks with error
  // type set to kStreamDisconnected.
  bool HandleRestartEvent();

  // IUnknown (required by IAudioSessionEvents and IMMNotificationClient).
  ULONG __stdcall AddRef() override;
  ULONG __stdcall Release() override;
  HRESULT __stdcall QueryInterface(REFIID iid, void** object) override;

  // IAudioSessionEvents implementation.
  // These methods are called on separate threads owned by the session manager.
  // More than one thread can be involved depending on the type of callback
  // and audio session.
  HRESULT __stdcall OnStateChanged(AudioSessionState new_state) override;
  HRESULT __stdcall OnSessionDisconnected(
      AudioSessionDisconnectReason disconnect_reason) override;
  HRESULT __stdcall OnDisplayNameChanged(LPCWSTR new_display_name,
                                         LPCGUID event_context) override;
  HRESULT __stdcall OnIconPathChanged(LPCWSTR new_icon_path,
                                      LPCGUID event_context) override;
  HRESULT __stdcall OnSimpleVolumeChanged(float new_simple_volume,
                                          BOOL new_mute,
                                          LPCGUID event_context) override;
  HRESULT __stdcall OnChannelVolumeChanged(DWORD channel_count,
                                           float new_channel_volumes[],
                                           DWORD changed_channel,
                                           LPCGUID event_context) override;
  HRESULT __stdcall OnGroupingParamChanged(LPCGUID new_grouping_param,
                                           LPCGUID event_context) override;
};

}  // namespace webrtc_win
}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_BASE_WIN_H_
