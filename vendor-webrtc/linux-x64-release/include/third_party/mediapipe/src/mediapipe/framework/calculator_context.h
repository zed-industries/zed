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

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTEXT_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTEXT_H_

#include <memory>
#include <queue>
#include <string>
#include <utility>

#include "absl/log/absl_check.h"
#include "absl/status/status.h"
#include "mediapipe/framework/calculator_state.h"
#include "mediapipe/framework/counter.h"
#include "mediapipe/framework/graph_service.h"
#include "mediapipe/framework/graph_service_manager.h"
#include "mediapipe/framework/input_stream_shard.h"
#include "mediapipe/framework/output_stream_shard.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/any_proto.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/resources.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

// A CalculatorContext provides information about the graph it is running
// inside of through a number of accessor functions: Inputs(), Outputs(),
// InputSidePackets(), Options(), etc.
//
// CalculatorBase APIs, such as CalculatorBase::Open(CalculatorContext* cc),
// CalculatorBase::Process(CalculatorContext* cc), and
// CalculatorBase::Close(CalculatorContext* cc), will only interact with
// its own CalculatorContext object for exchanging data with the framework.
class CalculatorContext {
 public:
  CalculatorContext(CalculatorState* calculator_state,
                    std::shared_ptr<tool::TagMap> input_tag_map,
                    std::shared_ptr<tool::TagMap> output_tag_map)
      : calculator_state_(calculator_state),
        inputs_(std::move(input_tag_map)),
        outputs_(std::move(output_tag_map)) {}

  CalculatorContext(const CalculatorContext&) = delete;
  CalculatorContext& operator=(const CalculatorContext&) = delete;

  const std::string& NodeName() const;
  int NodeId() const;
  const std::string& CalculatorType() const;
  // Returns the options given to this calculator. The Calculator or
  // CalculatorBase implementation may get its options by calling
  // GetExtension() on the result.
  const CalculatorOptions& Options() const;

  // Returns the options given to this calculator.  Template argument T must
  // be the type of the protobuf extension message or the protobuf::Any
  // message containing the options.
  template <class T>
  const T& Options() const {
    return calculator_state_->Options<T>();
  }

  template <class T>
  bool HasOptions() const {
    return calculator_state_->HasOptions<T>();
  }

  // Returns a counter using the graph's counter factory. The counter's name is
  // the passed-in name, prefixed by the calculator node's name (if present) or
  // the calculator's type (if not).
  Counter* GetCounter(const std::string& name);

  // Returns the counter set, which can be used to create new counters.
  // No prefix is added to counters created in this way.
  CounterFactory* GetCounterFactory();

  // Returns the current input timestamp, or Timestamp::Unset if there are
  // no input packets.
  Timestamp InputTimestamp() const {
    return input_timestamps_.empty() ? Timestamp::Unset()
                                     : input_timestamps_.front();
  }

  // Returns a reference to the input side packet set.
  const PacketSet& InputSidePackets() const;
  // Returns a reference to the output side packet collection.
  OutputSidePacketSet& OutputSidePackets();
  // Returns a reference to the input stream collection.
  // You may consume or move the value packets from the Inputs.
  InputStreamShardSet& Inputs();
  // Returns a const reference to the input stream collection.
  const InputStreamShardSet& Inputs() const;
  // Returns a reference to the output stream collection.
  OutputStreamShardSet& Outputs();
  // Returns a const reference to the output stream collection.
  const OutputStreamShardSet& Outputs() const;

  // Sets this packet timestamp offset for Packets going to all outputs.
  // If you only want to set the offset for a single output stream then
  // use OutputStream::SetOffset() directly.
  void SetOffset(TimestampDiff offset);

  // DEPRECATED: This was intended to get graph run status during
  // `CalculatorBase::Close` call. However, `Close` can run simultaneously with
  // other calculators `CalculatorBase::Process`, hence the actual graph
  // status may change any time and returned graph status here does not
  // necessarily reflect the actual graph status.
  //
  // As an alternative, instead of checking graph status in `Close` and doing
  // work for "done" state, you can enable timestamp bound processing for your
  // calculator (`CalculatorContract::SetProcessTimestampBounds`) to trigger
  // `Process` on timestamp bound updates and handle "done" state there.
  // Check examples in:
  // mediapipe/framework/calculator_graph_summary_packet_test.cc.
  //
  ABSL_DEPRECATED("Does not reflect the actual graph status.")
  absl::Status GraphStatus() const { return graph_status_; }

  ProfilingContext* GetProfilingContext() const {
    return calculator_state_->GetSharedProfilingContext().get();
  }

  template <typename T>
  ServiceBinding<T> Service(const GraphService<T>& service) {
    return ServiceBinding<T>(calculator_state_->GetServiceObject(service));
  }

  // Gets interface to access resources (file system, assets, etc.) from
  // calculators.
  //
  // NOTE: this is the preferred way to access resources from subgraphs and
  // calculators as it allows for fine grained per graph configuration.
  //
  // Resources can be configured by setting a custom `kResourcesService` graph
  // service on `CalculatorGraph`. The default resources service can be created
  // and reused through `CreateDefaultResources`.
  const Resources& GetResources() const {
    return calculator_state_->GetResources();
  }

  // Enables access to private GetGraphServiceManager() method.
  friend class CalculatorGraph;

 private:
  // Returns the graph-level service manager for sharing its services with
  // calculator-nested MP graphs.
  // Note: For accessing MP services from a calculator, use the
  // ServiceBinding<T> Service(kService) method above.
  const GraphServiceManager* GetGraphServiceManager() const {
    return calculator_state_->GetGraphServiceManager();
  }

  int NumberOfTimestamps() const {
    return static_cast<int>(input_timestamps_.size());
  }

  bool HasInputTimestamp() const { return !input_timestamps_.empty(); }

  // Adds a new input timestamp by the friend class CalculatorContextManager.
  void PushInputTimestamp(Timestamp input_timestamp) {
    input_timestamps_.push(input_timestamp);
  }

  void PopInputTimestamp() {
    ABSL_CHECK(!input_timestamps_.empty());
    input_timestamps_.pop();
  }

  void SetGraphStatus(const absl::Status& status) { graph_status_ = status; }

  // Interface for the friend class Calculator.
  const InputStreamSet& InputStreams() const;
  const OutputStreamSet& OutputStreams() const;

  // Stores the shared data across all CalculatorContext objects, including
  // input side packets, calculator options, node name, etc.
  // TODO: Removes unnecessary fields from CalculatorState after
  // migrating all clients to CalculatorContext.
  CalculatorState* calculator_state_;

  InputStreamShardSet inputs_;
  OutputStreamShardSet outputs_;
  // Created on-demand when needed by legacy APIs. No synchronization needed
  // because all possible callers are already serialized.
  mutable std::unique_ptr<InputStreamSet> input_streams_;
  mutable std::unique_ptr<OutputStreamSet> output_streams_;
  // The queue of timestamp values to Process() in this calculator context.
  std::queue<Timestamp> input_timestamps_;

  // The status of the graph run. Only used when Close() is called.
  absl::Status graph_status_;

  // Accesses CalculatorContext for setting input timestamp.
  friend class CalculatorContextManager;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTEXT_H_
