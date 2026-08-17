/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_PEER_SCENARIO_PEER_SCENARIO_H_
#define TEST_PEER_SCENARIO_PEER_SCENARIO_H_

// The peer connection scenario test framework enables writing end to end unit
// tests on the peer connection level. It's similar to the Scenario test but
// uses the full stack, including SDP and ICE negotiation. This ensures that
// features work end to end. It's also diffferent from the other tests on peer
// connection level in that it does not rely on any mocks or fakes other than
// for media input and networking. Additionally it provides direct access to the
// underlying peer connection class.

#include <list>
#include <vector>

#include "api/test/time_controller.h"
#include "test/gtest.h"
#include "test/logging/log_writer.h"
#include "test/network/network_emulation_manager.h"
#include "test/peer_scenario/peer_scenario_client.h"
#include "test/peer_scenario/signaling_route.h"
#include "test/scenario/stats_collection.h"
#include "test/scenario/video_frame_matcher.h"

namespace webrtc {
namespace test {
// The PeerScenario class represents a PeerConnection simulation scenario. The
// main purpose is to maintain ownership and ensure safe destruction order of
// clients and network emulation. Additionally it reduces the amount of boiler
// plate requited for some actions. For example usage see the existing tests
// using this class. Note that it should be used from a single calling thread.
// This thread will also be assigned as the signaling thread for all peer
// connections that are created. This means that the process methods must be
// used when waiting to ensure that messages are processed on the signaling
// thread.
class PeerScenario {
 public:
  // The name is used for log output when those are enabled by the --peer_logs
  // command line flag. Optionally, the TestInfo struct available in gtest can
  // be used to automatically generate a path based on the test name.
  explicit PeerScenario(const testing::TestInfo& test_info,
                        TimeMode mode = TimeMode::kSimulated);
  explicit PeerScenario(std::string file_name,
                        TimeMode mode = TimeMode::kSimulated);
  explicit PeerScenario(
      std::unique_ptr<LogWriterFactoryInterface> log_writer_manager,
      TimeMode mode = TimeMode::kSimulated);

  NetworkEmulationManagerImpl* net() { return &net_; }

  // Creates a client wrapping a peer connection conforming to the given config.
  // The client  will share the signaling thread with the scenario. To maintain
  // control of destruction order, ownership is kept within the scenario.
  PeerScenarioClient* CreateClient(PeerScenarioClient::Config config);
  PeerScenarioClient* CreateClient(std::string name,
                                   PeerScenarioClient::Config config);

  // Sets up a signaling route that can be used for SDP and ICE.
  SignalingRoute ConnectSignaling(PeerScenarioClient* caller,
                                  PeerScenarioClient* callee,
                                  std::vector<EmulatedNetworkNode*> send_link,
                                  std::vector<EmulatedNetworkNode*> ret_link);

  // Connects two clients over given links. This will also start ICE signaling
  // and SDP negotiation with default behavior. For customized behavior,
  // ConnectSignaling should be used to allow more detailed control, for
  // instance to allow different signaling and media routes.
  void SimpleConnection(PeerScenarioClient* caller,
                        PeerScenarioClient* callee,
                        std::vector<EmulatedNetworkNode*> send_link,
                        std::vector<EmulatedNetworkNode*> ret_link);

  // Starts feeding the results of comparing captured frames from `send_track`
  // with decoded frames on `receiver` to `analyzer`.
  // TODO(srte): Provide a way to detach to allow removal of tracks.
  void AttachVideoQualityAnalyzer(VideoQualityAnalyzer* analyzer,
                                  VideoTrackInterface* send_track,
                                  PeerScenarioClient* receiver);

  // Waits on `event` while processing messages on the signaling thread.
  bool WaitAndProcess(std::atomic<bool>* event,
                      TimeDelta max_duration = TimeDelta::Seconds(5));

  // Process messages on the signaling thread for the given duration.
  void ProcessMessages(TimeDelta duration);

 private:
  // Helper struct to maintain ownership of the matcher and taps.
  struct PeerVideoQualityPair {
   public:
    PeerVideoQualityPair(Clock* capture_clock, VideoQualityAnalyzer* analyzer)
        : matcher_({analyzer->Handler()}),
          capture_tap_(capture_clock, &matcher_),
          decode_tap_(capture_clock, &matcher_, 0) {}
    VideoFrameMatcher matcher_;
    CapturedFrameTap capture_tap_;
    DecodedFrameTap decode_tap_;
  };

  Clock* clock() { return Clock::GetRealTimeClock(); }

  std::unique_ptr<LogWriterFactoryInterface> GetLogWriterFactory(
      std::string name);

  const std::unique_ptr<LogWriterFactoryInterface> log_writer_manager_;
  NetworkEmulationManagerImpl net_;
  Thread* const signaling_thread_;
  std::list<PeerVideoQualityPair> video_quality_pairs_;
  std::list<PeerScenarioClient> peer_clients_;
};

}  // namespace test
}  // namespace webrtc
#endif  // TEST_PEER_SCENARIO_PEER_SCENARIO_H_
