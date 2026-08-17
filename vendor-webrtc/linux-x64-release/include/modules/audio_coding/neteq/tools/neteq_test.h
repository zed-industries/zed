/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_TEST_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_TEST_H_

#include <fstream>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/environment/environment.h"
#include "api/neteq/neteq.h"
#include "api/neteq/neteq_factory.h"
#include "api/test/neteq_simulator.h"
#include "modules/audio_coding/neteq/tools/audio_sink.h"
#include "modules/audio_coding/neteq/tools/neteq_input.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {
namespace test {

class NetEqTestErrorCallback {
 public:
  virtual ~NetEqTestErrorCallback() = default;
  virtual void OnInsertPacketError(const NetEqInput::PacketData& /* packet */) {
  }
  virtual void OnGetAudioError() {}
};

class DefaultNetEqTestErrorCallback : public NetEqTestErrorCallback {
  void OnInsertPacketError(const NetEqInput::PacketData& packet) override;
  void OnGetAudioError() override;
};

class NetEqPostInsertPacket {
 public:
  virtual ~NetEqPostInsertPacket() = default;
  virtual void AfterInsertPacket(const NetEqInput::PacketData& packet,
                                 NetEq* neteq) = 0;
};

class NetEqGetAudioCallback {
 public:
  virtual ~NetEqGetAudioCallback() = default;
  virtual void BeforeGetAudio(NetEq* neteq) = 0;
  virtual void AfterGetAudio(int64_t time_now_ms,
                             const AudioFrame& audio_frame,
                             bool muted,
                             NetEq* neteq) = 0;
};

class NetEqSimulationEndedCallback {
 public:
  virtual ~NetEqSimulationEndedCallback() = default;
  virtual void SimulationEnded(int64_t simulation_time_ms) = 0;
};

// Class that provides an input--output test for NetEq. The input (both packets
// and output events) is provided by a NetEqInput object, while the output is
// directed to an AudioSink object.
class NetEqTest : public NetEqSimulator {
 public:
  using DecoderMap = std::map<int, SdpAudioFormat>;

  struct Callbacks {
    NetEqTestErrorCallback* error_callback = nullptr;
    NetEqPostInsertPacket* post_insert_packet = nullptr;
    NetEqGetAudioCallback* get_audio_callback = nullptr;
    NetEqSimulationEndedCallback* simulation_ended_callback = nullptr;
  };

  // Sets up the test with given configuration, codec mappings, input, ouput,
  // and callback objects for error reporting.
  NetEqTest(const NetEq::Config& config,
            scoped_refptr<AudioDecoderFactory> decoder_factory,
            const DecoderMap& codecs,
            std::unique_ptr<std::ofstream> text_log,
            NetEqFactory* neteq_factory,
            std::unique_ptr<NetEqInput> input,
            std::unique_ptr<AudioSink> output,
            Callbacks callbacks,
            absl::string_view field_trials = "");

  ~NetEqTest() override;

  // Runs the test. Returns the duration of the produced audio in ms.
  int64_t Run() override;
  // Runs the simulation until we hit the next GetAudio event. If the simulation
  // is finished, is_simulation_finished will be set to true in the returned
  // SimulationStepResult.
  SimulationStepResult RunToNextGetAudio() override;

  void SetNextAction(Action next_operation) override;
  NetEqState GetNetEqState() override;
  NetEq* GetNetEq() override { return neteq_.get(); }

  // Returns the statistics from NetEq.
  NetEqNetworkStatistics SimulationStats();
  NetEqLifetimeStatistics LifetimeStats() const;

  static DecoderMap StandardDecoderMap();

 private:
  void RegisterDecoders(const DecoderMap& codecs);
  std::unique_ptr<NetEqInput> input_;
  SimulatedClock clock_;
  const Environment env_;
  std::optional<Action> next_action_;
  std::optional<int> last_packet_time_ms_;
  std::unique_ptr<NetEq> neteq_;
  std::unique_ptr<AudioSink> output_;
  Callbacks callbacks_;
  int sample_rate_hz_;
  NetEqState current_state_;
  NetEqOperationsAndState prev_ops_state_;
  NetEqLifetimeStatistics prev_lifetime_stats_;
  std::optional<uint32_t> last_packet_timestamp_;
  std::unique_ptr<std::ofstream> text_log_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_NETEQ_TEST_H_
