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

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTRACT_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTRACT_H_

#include <map>
#include <memory>
#include <string>
#include <typeindex>

// TODO: Move protos in another CL after the C++ code migration.
#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/graph_service.h"
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/packet_generator.pb.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/any_proto.h"
#include "mediapipe/framework/status_handler.pb.h"
#include "mediapipe/framework/tool/options_map.h"

namespace mediapipe {

// CalculatorContract contains the expectations and properties of a Node
// object, such as the expected packet types of input and output streams and
// input and output side packets.
//
// Setters and getters are available for specifying an InputStreamHandler and
// it's options from inside a calculator's GetContract() method. Ex:
//  cc->SetInputStreamHandler("FixedSizeInputStreamHandler");
//  MediaPipeOptions options;
//  options.MutableExtension(FixedSizeInputStreamHandlerOptions::ext)
//      ->set_fixed_min_size(2);
//  cc->SetInputStreamHandlerOptions(options);
//
class CalculatorContract {
 public:
  absl::Status Initialize(const CalculatorGraphConfig::Node& node);
  absl::Status Initialize(const PacketGeneratorConfig& node,
                          const std::string& package);
  absl::Status Initialize(const StatusHandlerConfig& node);
  void SetNodeName(const std::string& node_name) { node_name_ = node_name; }

  // Returns the options given to this node.
  const CalculatorOptions& Options() const { return node_config_->options(); }

  // Returns the name given to this node.
  const std::string& GetNodeName() const { return node_name_; }

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

  // Returns the PacketTypeSet for the input streams.
  PacketTypeSet& Inputs() { return *inputs_; }
  const PacketTypeSet& Inputs() const { return *inputs_; }

  // Returns the PacketTypeSet for the output streams.
  PacketTypeSet& Outputs() { return *outputs_; }
  const PacketTypeSet& Outputs() const { return *outputs_; }

  // Returns the PacketTypeSet for the input side packets.
  PacketTypeSet& InputSidePackets() { return *input_side_packets_; }
  const PacketTypeSet& InputSidePackets() const { return *input_side_packets_; }

  // Returns the PacketTypeSet for the output side packets.
  PacketTypeSet& OutputSidePackets() { return *output_side_packets_; }
  const PacketTypeSet& OutputSidePackets() const {
    return *output_side_packets_;
  }

  // Specifies the preferred InputStreamHandler for this Node.
  // If there is an InputStreamHandler specified in the graph (.pbtxt) for this
  // Node, then the graph's InputStreamHandler will take priority.
  void SetInputStreamHandler(const std::string& name) {
    input_stream_handler_ = name;
  }
  void SetInputStreamHandlerOptions(const MediaPipeOptions& options) {
    input_stream_handler_options_ = options;
  }

  // Returns the name of this Nodes's InputStreamHandler, or empty string if
  // none is set.
  std::string GetInputStreamHandler() const { return input_stream_handler_; }

  // Returns the MediaPipeOptions of this Node's InputStreamHandler, or empty
  // options if none is set.
  MediaPipeOptions GetInputStreamHandlerOptions() const {
    return input_stream_handler_options_;
  }

  // The next few methods are concerned with timestamp bound propagation
  // (see scheduling_sync.md#input-policies). Every calculator that processes
  // live inputs should specify either ProcessTimestampBounds or
  // TimestampOffset.  Calculators that produce output at the same timestamp as
  // the input, or with a fixed offset, should declare this fact using
  // SetTimestampOffset.  Calculators that require custom timestamp bound
  // calculations should use SetProcessTimestampBounds.

  // When true, Process is called for every new timestamp bound, with or without
  // new packets. A call to Process with only an input timestamp bound is
  // normally used to compute a new output timestamp bound.
  // NOTE: Also, when true, Process is called when input streams become done,
  // which means, Process needs to handle input streams in "done" state.
  // (Usually, by closing calculators' outputs where and when appropriate.)
  void SetProcessTimestampBounds(bool process_timestamps) {
    process_timestamps_ = process_timestamps;
  }
  bool GetProcessTimestampBounds() const { return process_timestamps_; }

  // Specifies the maximum difference between input and output timestamps.
  // When specified, the mediapipe framework automatically computes output
  // timestamp bounds based on input timestamps.  The special value
  // TimestampDiff::Unset disables the timestamp offset.
  void SetTimestampOffset(TimestampDiff offset) { timestamp_offset_ = offset; }
  TimestampDiff GetTimestampOffset() const { return timestamp_offset_; }

  class GraphServiceRequest {
   public:
    // APIs that should be used by calculators.
    //
    // Indicates that requested service is optional and calculator can operate
    // correctly without it.
    //
    // NOTE: `CalculatorGraph` will still try to create services which allow
    // default initialization. (See `CalculatorGraph::UseService`)
    GraphServiceRequest& Optional() {
      optional_ = true;
      return *this;
    }

    // Internal use.
    GraphServiceRequest(const GraphServiceBase& service) : service_(service) {}

    const GraphServiceBase& Service() const { return service_; }

    bool IsOptional() const { return optional_; }

   private:
    const GraphServiceBase& service_;
    bool optional_ = false;
  };

  // Indicates specific `service` is required for graph execution.
  //
  // For services which allow default initialization:
  // - `CalculatorGraph` will try to create corresponding service object by
  //   default even if request is made optional
  //   (`GraphServiceRequest::Optional()`).
  //
  // For services which disallow default initialization:
  // - `CalculatorGraph` requires client to set corresponding service object and
  //   otherwise fails, unless request is made optional
  //   (`GraphServiceRequest::Optional()`).
  GraphServiceRequest& UseService(const GraphServiceBase& service) {
    auto it = service_requests_.emplace(service.key, service).first;
    return it->second;
  }

  // A GraphService's key is always a static constant, so we can use string_view
  // as the key type without lifetime issues.
  using ServiceReqMap =
      absl::flat_hash_map<absl::string_view, GraphServiceRequest>;

  const ServiceReqMap& ServiceRequests() const { return service_requests_; }

 private:
  template <class T>
  void GetNodeOptions(T* result) const;

  // When creating a contract for a PacketGenerator, we define a configuration
  // for a wrapper calculator, for use by CalculatorNode.
  const CalculatorGraphConfig::Node& GetWrapperConfig() const {
    return *wrapper_config_;
  }

  const CalculatorGraphConfig::Node* node_config_ = nullptr;
  std::unique_ptr<CalculatorGraphConfig::Node> wrapper_config_;
  tool::OptionsMap options_;
  std::unique_ptr<PacketTypeSet> inputs_;
  std::unique_ptr<PacketTypeSet> outputs_;
  std::unique_ptr<PacketTypeSet> input_side_packets_;
  std::unique_ptr<PacketTypeSet> output_side_packets_;
  std::string input_stream_handler_;
  MediaPipeOptions input_stream_handler_options_;
  std::string node_name_;
  ServiceReqMap service_requests_;
  bool process_timestamps_ = false;
  TimestampDiff timestamp_offset_ = TimestampDiff::Unset();

  friend class CalculatorNode;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_CONTRACT_H_
