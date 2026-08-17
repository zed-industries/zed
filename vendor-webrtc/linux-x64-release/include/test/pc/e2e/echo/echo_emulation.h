/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ECHO_ECHO_EMULATION_H_
#define TEST_PC_E2E_ECHO_ECHO_EMULATION_H_

#include <atomic>
#include <deque>
#include <memory>
#include <vector>

#include "api/sequence_checker.h"
#include "api/test/pclf/media_configuration.h"
#include "modules/audio_device/include/test_audio_device.h"
#include "rtc_base/swap_queue.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Reduces audio input strength from provided capturer twice and adds input
// provided into EchoEmulatingCapturer::OnAudioRendered(...).
class EchoEmulatingCapturer : public TestAudioDeviceModule::Capturer {
 public:
  EchoEmulatingCapturer(
      std::unique_ptr<TestAudioDeviceModule::Capturer> capturer,
      EchoEmulationConfig config);

  void OnAudioRendered(ArrayView<const int16_t> data);

  int SamplingFrequency() const override {
    return delegate_->SamplingFrequency();
  }
  int NumChannels() const override { return delegate_->NumChannels(); }
  bool Capture(BufferT<int16_t>* buffer) override;

 private:
  std::unique_ptr<TestAudioDeviceModule::Capturer> delegate_;
  const EchoEmulationConfig config_;

  SwapQueue<std::vector<int16_t>> renderer_queue_;

  SequenceChecker renderer_thread_;
  std::vector<int16_t> queue_input_ RTC_GUARDED_BY(renderer_thread_);
  bool recording_started_ RTC_GUARDED_BY(renderer_thread_) = false;

  SequenceChecker capturer_thread_;
  std::vector<int16_t> queue_output_ RTC_GUARDED_BY(capturer_thread_);
  bool delay_accumulated_ RTC_GUARDED_BY(capturer_thread_) = false;
};

// Renders output into provided renderer and also copy output into provided
// EchoEmulationCapturer.
class EchoEmulatingRenderer : public TestAudioDeviceModule::Renderer {
 public:
  EchoEmulatingRenderer(
      std::unique_ptr<TestAudioDeviceModule::Renderer> renderer,
      EchoEmulatingCapturer* echo_emulating_capturer);

  int SamplingFrequency() const override {
    return delegate_->SamplingFrequency();
  }
  int NumChannels() const override { return delegate_->NumChannels(); }
  bool Render(ArrayView<const int16_t> data) override;

 private:
  std::unique_ptr<TestAudioDeviceModule::Renderer> delegate_;
  EchoEmulatingCapturer* echo_emulating_capturer_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ECHO_ECHO_EMULATION_H_
