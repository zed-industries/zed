/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_TEST_PEER_FACTORY_H_
#define TEST_PC_E2E_TEST_PEER_FACTORY_H_

#include <memory>
#include <optional>
#include <string>

#include "api/test/pclf/media_configuration.h"
#include "api/test/pclf/peer_configurer.h"
#include "api/test/time_controller.h"
#include "pc/test/mock_peer_connection_observers.h"
#include "rtc_base/thread.h"
#include "test/pc/e2e/analyzer/video/video_quality_analyzer_injection_helper.h"
#include "test/pc/e2e/test_peer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

struct RemotePeerAudioConfig {
  explicit RemotePeerAudioConfig(AudioConfig config)
      : sampling_frequency_in_hz(config.sampling_frequency_in_hz),
        output_file_name(config.output_dump_file_name) {}

  static std::optional<RemotePeerAudioConfig> Create(
      std::optional<AudioConfig> config);

  int sampling_frequency_in_hz;
  std::optional<std::string> output_file_name;
};

class TestPeerFactory {
 public:
  // Creates a test peer factory.
  // `signaling_thread` will be used as a signaling thread for all peers created
  // by this factory.
  // `time_controller` will be used to create required threads, task queue
  // factories and call factory.
  // `video_analyzer_helper` will be used to setup video quality analysis for
  // created peers.
  TestPeerFactory(Thread* signaling_thread,
                  TimeController& time_controller,
                  VideoQualityAnalyzerInjectionHelper* video_analyzer_helper)
      : signaling_thread_(signaling_thread),
        time_controller_(time_controller),
        video_analyzer_helper_(video_analyzer_helper) {}

  // Setups all components, that should be provided to WebRTC
  // PeerConnectionFactory and PeerConnection creation methods,
  // also will setup dependencies, that are required for media analyzers
  // injection.
  std::unique_ptr<TestPeer> CreateTestPeer(
      std::unique_ptr<PeerConfigurer> configurer,
      std::unique_ptr<MockPeerConnectionObserver> observer,
      std::optional<RemotePeerAudioConfig> remote_audio_config,
      std::optional<EchoEmulationConfig> echo_emulation_config);

 private:
  Thread* signaling_thread_;
  TimeController& time_controller_;
  VideoQualityAnalyzerInjectionHelper* video_analyzer_helper_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_TEST_PEER_FACTORY_H_
