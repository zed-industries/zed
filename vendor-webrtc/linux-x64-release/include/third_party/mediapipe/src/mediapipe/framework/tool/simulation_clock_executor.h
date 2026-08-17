// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_SIMULATION_CLOCK_EXECUTOR_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_SIMULATION_CLOCK_EXECUTOR_H_

#include "mediapipe/framework/thread_pool_executor.h"
#include "mediapipe/framework/tool/simulation_clock.h"

namespace mediapipe {

// Simulation clock multithreaded executor. This is intended to be used with
// graphs that are using SimulationClock class to emulate various parts of the
// graph taking specific time to process the incoming packets.
class SimulationClockExecutor : public Executor {
 public:
  explicit SimulationClockExecutor(int num_threads);
  void Schedule(std::function<void()> task) override;

  // Returns a pointer to the instance of SimulationClock used by
  // this executor. This instance can be passed down to graph nodes as input
  // side packet.
  std::shared_ptr<SimulationClock> GetClock();

 private:
  // SimulationClock instance used by this executor.
  std::shared_ptr<SimulationClock> clock_;
  // The delegate ThreadPoolExecutor.  This is declared after clock_
  // so that it is destroyed before clock_.
  ThreadPoolExecutor executor_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_SIMULATION_CLOCK_EXECUTOR_H_
