/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_NETEQ_SIMULATOR_H_
#define API_TEST_NETEQ_SIMULATOR_H_

#include <stdint.h>

#include <map>
#include <vector>

#include "api/neteq/neteq.h"

namespace webrtc {
namespace test {

class NetEqSimulator {
 public:
  virtual ~NetEqSimulator() = default;

  enum class Action { kNormal, kExpand, kAccelerate, kPreemptiveExpand };

  // The results of one simulation step.
  struct SimulationStepResult {
    SimulationStepResult();
    SimulationStepResult(const SimulationStepResult& other);
    ~SimulationStepResult();

    bool is_simulation_finished = false;
    // The amount of audio produced (in ms) with the actions in this time step.
    std::map<Action, int> action_times_ms;
    // The amount of wall clock time (in ms) that elapsed since the previous
    // event. This is not necessarily equal to the sum of the values in
    // action_times_ms.
    int64_t simulation_step_ms = 0;
  };

  struct NetEqState {
    NetEqState();
    NetEqState(const NetEqState& other);
    ~NetEqState();
    // The sum of the packet buffer and sync buffer delay.
    int current_delay_ms = 0;
    // An indicator that packet loss occurred since the last GetAudio event.
    bool packet_loss_occurred = false;
    // An indicator that the packet buffer has been flushed since the last
    // GetAudio event.
    bool packet_buffer_flushed = false;
    // Indicates if the next needed packet is available in the buffer.
    bool next_packet_available = false;
    // The inter-arrival times in ms of the packets that have arrived since the
    // last GetAudio event.
    std::vector<int> packet_iat_ms;
    // The current packet size in ms.
    int packet_size_ms = 0;
  };

  // Runs the simulation until the end. Returns the duration of the produced
  // audio in ms.
  virtual int64_t Run() = 0;
  // Runs the simulation until we hit the next GetAudio event. If the simulation
  // is finished, is_simulation_finished will be set to true in the returned
  // SimulationStepResult.
  virtual SimulationStepResult RunToNextGetAudio() = 0;

  // Set the next action to be taken by NetEq. This will override any action
  // that NetEq would normally decide to take.
  virtual void SetNextAction(Action next_operation) = 0;

  // Get the current state of NetEq.
  virtual NetEqState GetNetEqState() = 0;

  // Get the underlying NetEq instance.
  virtual NetEq* GetNetEq() = 0;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_NETEQ_SIMULATOR_H_
