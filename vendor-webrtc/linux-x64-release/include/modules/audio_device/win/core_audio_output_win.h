/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_OUTPUT_WIN_H_
#define MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_OUTPUT_WIN_H_

#include <memory>
#include <string>

#include "modules/audio_device/win/audio_device_module_win.h"
#include "modules/audio_device/win/core_audio_base_win.h"

namespace webrtc {

class AudioDeviceBuffer;
class FineAudioBuffer;

namespace webrtc_win {

// Windows specific AudioOutput implementation using a CoreAudioBase class where
// an output direction is set at construction. Supports render device handling
// and streaming of decoded audio from a WebRTC client to the native audio
// layer.
class CoreAudioOutput final : public CoreAudioBase, public AudioOutput {
 public:
  CoreAudioOutput(bool automatic_restart);
  ~CoreAudioOutput() override;

  // AudioOutput implementation.
  int Init() override;
  int Terminate() override;
  int NumDevices() const override;
  int SetDevice(int index) override;
  int SetDevice(AudioDeviceModule::WindowsDeviceType device) override;
  int DeviceName(int index, std::string* name, std::string* guid) override;
  void AttachAudioBuffer(AudioDeviceBuffer* audio_buffer) override;
  bool PlayoutIsInitialized() const override;
  int InitPlayout() override;
  int StartPlayout() override;
  int StopPlayout() override;
  bool Playing() override;
  int VolumeIsAvailable(bool* available) override;
  int RestartPlayout() override;
  bool Restarting() const override;
  int SetSampleRate(uint32_t sample_rate) override;

  CoreAudioOutput(const CoreAudioOutput&) = delete;
  CoreAudioOutput& operator=(const CoreAudioOutput&) = delete;

 private:
  void ReleaseCOMObjects();
  bool OnDataCallback(uint64_t device_frequency);
  bool OnErrorCallback(ErrorType error);
  int EstimateOutputLatencyMillis(uint64_t device_frequency);
  bool HandleStreamDisconnected();

  std::unique_ptr<FineAudioBuffer> fine_audio_buffer_;
  Microsoft::WRL::ComPtr<IAudioRenderClient> audio_render_client_;
  uint64_t num_frames_written_ = 0;
};

}  // namespace webrtc_win
}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_WIN_CORE_AUDIO_OUTPUT_WIN_H_
