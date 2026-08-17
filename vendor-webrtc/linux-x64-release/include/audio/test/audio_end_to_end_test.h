/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef AUDIO_TEST_AUDIO_END_TO_END_TEST_H_
#define AUDIO_TEST_AUDIO_END_TO_END_TEST_H_

#include <memory>
#include <string>
#include <vector>

#include "api/audio/audio_device.h"
#include "api/task_queue/task_queue_base.h"
#include "api/test/simulated_network.h"
#include "modules/audio_device/include/test_audio_device.h"
#include "test/call_test.h"

namespace webrtc {
namespace test {

class AudioEndToEndTest : public test::EndToEndTest {
 public:
  AudioEndToEndTest();

 protected:
  AudioDeviceModule* send_audio_device() { return send_audio_device_; }
  const AudioSendStream* send_stream() const { return send_stream_; }
  const AudioReceiveStreamInterface* receive_stream() const {
    return receive_stream_;
  }

  size_t GetNumVideoStreams() const override;
  size_t GetNumAudioStreams() const override;
  size_t GetNumFlexfecStreams() const override;

  std::unique_ptr<TestAudioDeviceModule::Capturer> CreateCapturer() override;
  std::unique_ptr<TestAudioDeviceModule::Renderer> CreateRenderer() override;

  void OnFakeAudioDevicesCreated(AudioDeviceModule* send_audio_device,
                                 AudioDeviceModule* recv_audio_device) override;

  void ModifyAudioConfigs(AudioSendStream::Config* send_config,
                          std::vector<AudioReceiveStreamInterface::Config>*
                              receive_configs) override;
  void OnAudioStreamsCreated(AudioSendStream* send_stream,
                             const std::vector<AudioReceiveStreamInterface*>&
                                 receive_streams) override;

 private:
  AudioDeviceModule* send_audio_device_ = nullptr;
  AudioSendStream* send_stream_ = nullptr;
  AudioReceiveStreamInterface* receive_stream_ = nullptr;
};

}  // namespace test
}  // namespace webrtc

#endif  // AUDIO_TEST_AUDIO_END_TO_END_TEST_H_
