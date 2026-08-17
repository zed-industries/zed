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
//
// Defines CalculatorRunner which can be used to run a Calculator in
// isolation. This is useful for testing.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_RUNNER_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_RUNNER_H_

#include <memory>
#include <string>
#include <vector>

#include "absl/base/macros.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

class CalculatorGraph;

// The class for running the Calculator with given inputs and examining outputs.
class CalculatorRunner {
 public:
  // A representation of input or output stream contents.
  struct StreamContents {
    // The Packets in the stream.
    std::vector<Packet> packets;
    // Stream header.
    Packet header;
  };
  // A collection of StreamContents by either index or tag.
  typedef internal::Collection<StreamContents> StreamContentsSet;

  // Preferred constructor.
  // All the needed information comes from the node config.
  // Example:
  //   CalculatorRunner runner(R"(
  //       calculator: "ScaleImageCalculator"
  //       input_stream: "ycbcr_frames"
  //       output_stream: "FRAMES:srgb_frames"
  //       output_stream: "VIDEO_HEADER:srgb_frames_header"
  //       options {
  //         [mediapipe.ScaleImageCalculatorOptions.ext] {
  //           target_height: 10
  //           preserve_aspect_ratio: true
  //           output_format: SRGB
  //           algorithm: AREA
  //         }
  //       }
  //   )");
  explicit CalculatorRunner(const CalculatorGraphConfig::Node& node_config);
#if !defined(MEDIAPIPE_PROTO_LITE)
  // Convenience constructor which takes a node_config string directly.
  explicit CalculatorRunner(const std::string& node_config_string);
  // Convenience constructor to initialize a calculator which uses indexes
  // (not tags) for all its fields.
  // NOTE: This constructor calls proto_ns::TextFormat::ParseFromString(), which
  // is not available when using lite protos.
  CalculatorRunner(const std::string& calculator_type,
                   const std::string& options_string, int num_inputs,
                   int num_outputs, int num_side_packets);
#endif
  // Minimal constructor which requires additional calls to define inputs,
  // outputs, and input side packets.  Prefer using another constructor.
  ABSL_DEPRECATED("Initialize CalculatorRunner with a proto instead.")
  CalculatorRunner(const std::string& calculator_type,
                   const CalculatorOptions& options);

  CalculatorRunner(const CalculatorRunner&) = delete;
  CalculatorRunner& operator=(const CalculatorRunner&) = delete;

  ~CalculatorRunner();

  // Sets the number of input streams, output streams, or input side packets,
  // respectively. May not be called after Run() has been called.
  ABSL_DEPRECATED("Initialize CalculatorRunner with a proto instead.")
  void SetNumInputs(int n);
  ABSL_DEPRECATED("Initialize CalculatorRunner with a proto instead.")
  void SetNumOutputs(int n);
  ABSL_DEPRECATED("Initialize CalculatorRunner with a proto instead.")
  void SetNumInputSidePackets(int n);

  // Initializes the inputs, outputs, or side packets using a
  // TagAndNameInfo.  This sets the corresponding section of node_config_.
  // May not be called after Run() has been called.
  ABSL_DEPRECATED("Initialize CalculatorRunner with a proto instead.")
  void InitializeInputs(const tool::TagAndNameInfo& info);
  ABSL_DEPRECATED("Initialize CalculatorRunner with a proto instead.")
  void InitializeOutputs(const tool::TagAndNameInfo& info);
  ABSL_DEPRECATED("Initialize CalculatorRunner with a proto instead.")
  void InitializeInputSidePackets(const tool::TagAndNameInfo& info);

  // Returns mutable access to the input stream contents.
  StreamContentsSet* MutableInputs() { return inputs_.get(); }
  // Returns mutable access to the input side packets.
  PacketSet* MutableSidePackets() { return input_side_packets_.get(); }

  // Runs the calculator, by calling Open(), Process() with the
  // inputs provided via mutable_inputs(), and Close(). Returns the
  // absl::Status from CalculatorGraph::Run().  Internally, Run()
  // constructs a CalculatorGraph in the first call, and calls
  // CalculatorGraph::Run().  A single instance of CalculatorRunner
  // uses the same instance of CalculatorGraph for all runs.
  absl::Status Run();

  // Returns the vector of contents of the output streams. The .header
  // field contains the stream header and the .packets field contains
  // the Packets from the stream, unless SetOutputPacketCallback()
  // has been called with non-nullptr, in which case .packets will be empty.
  const StreamContentsSet& Outputs() const { return *outputs_; }

  // Returns the access to the output side packets.
  const PacketSet& OutputSidePackets() { return *output_side_packets_; }

  // Returns a graph counter.
  mediapipe::Counter* GetCounter(const std::string& name);

  // Returns all graph counters values.
  std::map<std::string, int64_t> GetCountersValues();

 private:
  static const char kSourcePrefix[];
  static const char kSinkPrefix[];

  // Initialize using a node config (does the constructor's work).
  absl::Status InitializeFromNodeConfig(
      const CalculatorGraphConfig::Node& node_config);

  // Builds the graph if one does not already exist.
  absl::Status BuildGraph();

  CalculatorGraphConfig::Node node_config_;

  // Log the calculator proto after it is created from the provided
  // parameters.  This aids users in migrating to the recommended
  // constructor.
  bool log_calculator_proto_ = false;

  std::unique_ptr<StreamContentsSet> inputs_;
  std::unique_ptr<StreamContentsSet> outputs_;
  std::unique_ptr<PacketSet> input_side_packets_;
  std::unique_ptr<PacketSet> output_side_packets_;
  std::unique_ptr<CalculatorGraph> graph_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_RUNNER_H_
