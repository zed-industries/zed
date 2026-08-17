/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_SCENARIO_H_
#define TEST_SCENARIO_SCENARIO_H_
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/task_queue/task_queue_base.h"
#include "api/test/time_controller.h"
#include "rtc_base/fake_clock.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "test/gtest.h"
#include "test/logging/log_writer.h"
#include "test/network/network_emulation_manager.h"
#include "test/scenario/audio_stream.h"
#include "test/scenario/call_client.h"
#include "test/scenario/column_printer.h"
#include "test/scenario/network_node.h"
#include "test/scenario/scenario_config.h"
#include "test/scenario/video_stream.h"

namespace webrtc {
namespace test {
// Scenario is a class owning everything for a test scenario. It creates and
// holds network nodes, call clients and media streams. It also provides methods
// for changing behavior at runtime. Since it always keeps ownership of the
// created components, it generally returns non-owning pointers. It maintains
// the life of its objects until it is destroyed.
// For methods accepting configuration structs, a modifier function interface is
// generally provided. This allows simple partial overriding of the default
// configuration.
class Scenario {
 public:
  Scenario();
  explicit Scenario(const testing::TestInfo* test_info);
  explicit Scenario(absl::string_view file_name);
  Scenario(absl::string_view file_name, bool real_time);
  Scenario(std::unique_ptr<LogWriterFactoryInterface> log_writer_manager,
           bool real_time);

  ~Scenario();

  Scenario(const Scenario&) = delete;
  Scenario& operator=(const Scenario&) = delete;

  NetworkEmulationManagerImpl* net() { return &network_manager_; }

  EmulatedNetworkNode* CreateSimulationNode(NetworkSimulationConfig config);
  EmulatedNetworkNode* CreateSimulationNode(
      std::function<void(NetworkSimulationConfig*)> config_modifier);

  SimulationNode* CreateMutableSimulationNode(NetworkSimulationConfig config);
  SimulationNode* CreateMutableSimulationNode(
      std::function<void(NetworkSimulationConfig*)> config_modifier);

  CallClient* CreateClient(absl::string_view name, CallClientConfig config);
  CallClient* CreateClient(
      absl::string_view name,
      std::function<void(CallClientConfig*)> config_modifier);

  CallClientPair* CreateRoutes(CallClient* first,
                               std::vector<EmulatedNetworkNode*> send_link,
                               CallClient* second,
                               std::vector<EmulatedNetworkNode*> return_link);

  CallClientPair* CreateRoutes(CallClient* first,
                               std::vector<EmulatedNetworkNode*> send_link,
                               DataSize first_overhead,
                               CallClient* second,
                               std::vector<EmulatedNetworkNode*> return_link,
                               DataSize second_overhead);

  void ChangeRoute(std::pair<CallClient*, CallClient*> clients,
                   std::vector<EmulatedNetworkNode*> over_nodes);

  void ChangeRoute(std::pair<CallClient*, CallClient*> clients,
                   std::vector<EmulatedNetworkNode*> over_nodes,
                   DataSize overhead);

  VideoStreamPair* CreateVideoStream(
      std::pair<CallClient*, CallClient*> clients,
      std::function<void(VideoStreamConfig*)> config_modifier);
  VideoStreamPair* CreateVideoStream(
      std::pair<CallClient*, CallClient*> clients,
      VideoStreamConfig config);

  AudioStreamPair* CreateAudioStream(
      std::pair<CallClient*, CallClient*> clients,
      std::function<void(AudioStreamConfig*)> config_modifier);
  AudioStreamPair* CreateAudioStream(
      std::pair<CallClient*, CallClient*> clients,
      AudioStreamConfig config);

  // Runs the provided function with a fixed interval. For real time tests,
  // `function` starts being called after `interval` from the call to Every().
  void Every(TimeDelta interval, absl::AnyInvocable<void(TimeDelta)> function);
  void Every(TimeDelta interval, absl::AnyInvocable<void()> function);

  // Runs the provided function on the internal task queue. This ensure that
  // it's run on the main thread for simulated time tests.
  void Post(absl::AnyInvocable<void() &&> function);

  // Runs the provided function after given duration has passed. For real time
  // tests, `function` is called after `target_time_since_start` from the call
  // to Every().
  void At(TimeDelta offset, absl::AnyInvocable<void() &&> function);

  // Sends a packet over the nodes and runs `action` when it has been delivered.
  void NetworkDelayedAction(std::vector<EmulatedNetworkNode*> over_nodes,
                            size_t packet_size,
                            std::function<void()> action);

  // Runs the scenario for the given time.
  void RunFor(TimeDelta duration);
  // Runs the scenario until `target_time_since_start`.
  void RunUntil(TimeDelta target_time_since_start);
  // Runs the scenario until `target_time_since_start` or `exit_function`
  // returns true. `exit_function` is polled after each `check_interval` has
  // passed.
  void RunUntil(TimeDelta target_time_since_start,
                TimeDelta check_interval,
                std::function<bool()> exit_function);
  void Start();
  void Stop();

  // Triggers sending of dummy packets over the given nodes.
  void TriggerPacketBurst(std::vector<EmulatedNetworkNode*> over_nodes,
                          size_t num_packets,
                          size_t packet_size);

  ColumnPrinter TimePrinter();
  StatesPrinter* CreatePrinter(absl::string_view name,
                               TimeDelta interval,
                               std::vector<ColumnPrinter> printers);

  // Returns the current time.
  Timestamp Now();
  // Return the duration of the current session so far.
  TimeDelta TimeSinceStart();

  std::unique_ptr<RtcEventLogOutput> GetLogWriter(absl::string_view name) {
    if (!log_writer_factory_ || name.empty())
      return nullptr;
    return log_writer_factory_->Create(name);
  }
  std::unique_ptr<LogWriterFactoryInterface> GetLogWriterFactory(
      absl::string_view name) {
    if (!log_writer_factory_ || name.empty())
      return nullptr;
    return std::make_unique<LogWriterFactoryAddPrefix>(
        log_writer_factory_.get(), name);
  }

 private:
  TimeDelta TimeUntilTarget(TimeDelta target_time_offset);

  const std::unique_ptr<LogWriterFactoryInterface> log_writer_factory_;
  NetworkEmulationManagerImpl network_manager_;
  Clock* clock_;

  std::vector<std::unique_ptr<CallClient>> clients_;
  std::vector<std::unique_ptr<CallClientPair>> client_pairs_;
  std::vector<std::unique_ptr<VideoStreamPair>> video_streams_;
  std::vector<std::unique_ptr<AudioStreamPair>> audio_streams_;
  std::vector<std::unique_ptr<SimulationNode>> simulation_nodes_;
  std::vector<std::unique_ptr<StatesPrinter>> printers_;

  scoped_refptr<AudioDecoderFactory> audio_decoder_factory_;
  scoped_refptr<AudioEncoderFactory> audio_encoder_factory_;

  Timestamp start_time_ = Timestamp::PlusInfinity();
  // Defined last so it's destroyed first.
  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> task_queue_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_SCENARIO_SCENARIO_H_
