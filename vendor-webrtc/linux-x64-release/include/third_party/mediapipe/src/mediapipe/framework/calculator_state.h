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

// Defines CalculatorState.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_STATE_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_STATE_H_

#include <map>
#include <memory>
#include <string>

// TODO: Move protos in another CL after the C++ code migration.
#include "absl/base/macros.h"
#include "absl/status/status.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/counter.h"
#include "mediapipe/framework/counter_factory.h"
#include "mediapipe/framework/graph_service.h"
#include "mediapipe/framework/graph_service_manager.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/any_proto.h"
#include "mediapipe/framework/resources.h"
#include "mediapipe/framework/tool/options_map.h"

namespace mediapipe {

class ProfilingContext;
// Holds data that the Calculator needs access to.  This data is not
// stored in Calculator directly since Calculator will be destroyed after
// every CalculatorGraph::Run() .  It is not stored in CalculatorNode
// because Calculator should not depend on CalculatorNode.  All
// information conveyed in this class is flowing from the CalculatorNode
// to the Calculator.
class CalculatorState {
 public:
  CalculatorState(const std::string& node_name, int node_id,
                  const std::string& calculator_type,
                  const CalculatorGraphConfig::Node& node_config,
                  std::shared_ptr<ProfilingContext> profiling_context,
                  const GraphServiceManager* graph_service_manager);
  CalculatorState(const CalculatorState&) = delete;
  CalculatorState& operator=(const CalculatorState&) = delete;
  ~CalculatorState();

  // Called before every call to Calculator::Open() (during the PrepareForRun
  // phase).
  void ResetBetweenRuns();

  const std::string& CalculatorType() const { return calculator_type_; }
  const CalculatorOptions& Options() const { return node_config_.options(); }
  // Returns the options given to this calculator.  Template argument T must
  // be the type of the protobuf extension message or the protobuf::Any
  // message containing the options.
  template <class T>
  const T& Options() const {
    return options_.Get<T>();
  }
  template <class T>
  bool HasOptions() const {
    return options_.Has<T>();
  }
  const std::string& NodeName() const { return node_name_; }
  const int& NodeId() const { return node_id_; }

  ////////////////////////////////////////
  // Interface for Calculator.
  ////////////////////////////////////////
  const PacketSet& InputSidePackets() const { return *input_side_packets_; }
  OutputSidePacketSet& OutputSidePackets() { return *output_side_packets_; }

  // Returns a counter using the graph's counter factory. The counter's
  // name is the passed-in name, prefixed by the calculator NodeName.
  Counter* GetCounter(const std::string& name);

  // Returns a counter set, which can be passed to other classes, to generate
  // counters.  NOTE: This differs from GetCounter, in that the counters
  // created by this counter set do not have the NodeName prefix.
  CounterFactory* GetCounterFactory();

  std::shared_ptr<ProfilingContext> GetSharedProfilingContext() const {
    return profiling_context_;
  }

  // Returns the graph-level service manager for sharing its services with
  // calculator-nested MP graphs.
  const GraphServiceManager* GetGraphServiceManager() const {
    return graph_service_manager_;
  }

  // Returns calculator interface for loading resources.
  const Resources& GetResources() { return *resources_; }

  ////////////////////////////////////////
  // Interface for CalculatorNode.
  ////////////////////////////////////////
  // Sets the input side packets.
  void SetInputSidePackets(const PacketSet* input_side_packets);
  // Sets the output side packets.
  void SetOutputSidePackets(OutputSidePacketSet* output_side_packets);
  // Sets the counter factory.
  void SetCounterFactory(CounterFactory* counter_factory) {
    counter_factory_ = counter_factory;
  }

  absl::Status SetServicePacket(const GraphServiceBase& service,
                                Packet packet) {
    return calculator_service_manager_.SetServicePacket(service, packet);
  }

  template <typename T>
  std::shared_ptr<T> GetServiceObject(const GraphService<T>& service) {
    return calculator_service_manager_.GetServiceObject(service);
  }

 private:
  ////////////////////////////////////////
  // Persistent variables that are not cleared by ResetBetweenRuns().
  ////////////////////////////////////////
  // The name associated with this calculator's node.
  const std::string node_name_;
  // The ID associated with this calculator's node.
  const int node_id_;
  // The registered type name of the Calculator.
  const std::string calculator_type_;
  // The Node protobuf containing the options for the calculator.
  const CalculatorGraphConfig::Node node_config_;
  // The unpacked protobuf options for the calculator.
  tool::OptionsMap options_;
  // The graph tracing and profiling interface.
  std::shared_ptr<ProfilingContext> profiling_context_;

  // Const pointer to the graph-level service manager.
  const GraphServiceManager* graph_service_manager_ = nullptr;

  // calculator_service_manager_ contains only the services that are requested
  // by the calculator in UpdateContract() via cc->UseService(...).
  GraphServiceManager calculator_service_manager_;

  // Graph/calculator resource loading interface.
  std::shared_ptr<Resources> resources_;

  ////////////////////////////////////////
  // Variables which ARE cleared by ResetBetweenRuns().
  ////////////////////////////////////////
  // The set of input side packets set by CalculatorNode::PrepareForRun().
  // ResetBetweenRuns() clears this PacketSet pointer.
  const PacketSet* input_side_packets_;
  // The OutputSidePacketSet object is owned by the CalculatorNode.
  // CalculatorState obtains its pointer in CalculatorNode::PrepareForRun.
  OutputSidePacketSet* output_side_packets_;

  CounterFactory* counter_factory_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_STATE_H_
