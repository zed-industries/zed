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
// Functions for adding Calculators that dump data from a Graph.
//
// Specifically this is accomplished by adding a CallbackCalculator to
// the CalculatorGraphConfig and adding a corresponding InputSidePacket
// to a CalculatorGraph such that data which is sent on a stream will
// be captured in the desired way.  These functions are meant to isolate
// clients from such messy details.
//
// Although these functions are basically manipulations on a
// CalculatorGraphConfig they are not placed in tool/graph.h since they
// also depend on CalculatorGraph and having them in tool/graph.h would
// introduce a circular dependency.

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_SINK_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_SINK_H_

#include <functional>
#include <map>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/macros.h"
#include "absl/status/status.h"
#include "mediapipe/framework/calculator_base.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

class CalculatorGraph;
class Packet;

namespace tool {

////////////////////////////////////////
// All functions in this file should be avoided when possible, since they
// only work when the CalculatorGraph is being explicitly instantiated
// on the local machine.  Instead, prefer methods which only manipulate
// the CalculatorGraphConfig.
////////////////////////////////////////

// Add a CallbackCalculator to the config and an InputSidePacket to
// the uninitialized_graph such that the packets from stream_name
// will get dumped to dumped_data when the CalculatorGraph is Run.
// The provided graph must have been constructed but not initialized
// (this allows an InputSidePacket to be added to the CalculatorGraph,
// and also allows the CalculatorGraphConfig to still be changed before
// it is used for initialization).  Any number of these functions can
// be called on graph.
//
// NOTE: `dumped_data` (vector sink you pass) must outlive graph initialized
// with the `config` you add the sink to.
//
// Example usage:
//   CalculatorGraphConfig config = tool::ParseGraphFromFileOrDie("config.txt");
//   std::vector<Packet> packet_dump;
//   tool::AddVectorSink("output_samples", &config, &packet_dump);
//   // Call tool::AddVectorSink() more times if you wish. Note that each stream
//   // needs to get its own packet vector.
//   CalculatorGraph graph;
//   ABSL_CHECK_OK(graph.Initialize(config));
//   // Set other input side packets.
//   ABSL_CHECK_OK(graph.Run());
//   for (const Packet& packet : packet_dump) {
//     // Do something.
//   }
void AddVectorSink(const std::string& stream_name,  //
                   CalculatorGraphConfig* config,   //
                   std::vector<Packet>* dumped_data);

// Same as above, but only extract the Timestamp::PostStream() packet
// of the stream.
//
// NOTE: `post_stream_packet` (post stream packet sink you pass) must outlive
// graph initialized with the `config` you add the sink to.
void AddPostStreamPacketSink(const std::string& stream_name,
                             CalculatorGraphConfig* config,
                             Packet* post_stream_packet);

// Gets a side packet from a graph.
// Adds a conversion calculator to convert a side packet to a stream with a
// single packet at timestamp PostStream and then calls AddPostStreamPacketSink
// to dump the packet.
//
// NOTE: `dumped_packet` (side packet sink you pass) must outlive graph
// initialized with the `config` you add the sink to.
ABSL_DEPRECATED("Use CalculatorGraph::GetOutputSidePacket(const std::string&)")
void AddSidePacketSink(const std::string& side_packet_name,
                       CalculatorGraphConfig* config, Packet* dumped_packet);

// Add a CallbackCalculator to intercept packets sent on stream
// stream_name.  The input side packet with the produced name
// callback_side_packet_name must be set to an appropriate callback
// before the Graph is run. If use_std_function is true, the input side packet
// of the CallbackCalculator must be a std::function.
void AddCallbackCalculator(const std::string& stream_name,
                           CalculatorGraphConfig* config,
                           std::string* callback_side_packet_name,
                           bool use_std_function = false);

// Adds a CallbackCalculator that collects multiple streams. The callback will
// receive a vector with one packet per stream, in the order specified by the
// streams argument. All streams will be synchronized according to their
// timestamp, using the standard synchronization policy. If some streams are
// missing a packet for a given input timestamp, the vector will contain empty
// packets at their positions.
//
// Once a graph is constructed from the modified config, the packet in
// side_packet.second must be passed to it, with the name in side_packet.first.
// TODO: remove the need to pass the side packet manually.
void AddMultiStreamCallback(
    const std::vector<std::string>& streams,
    std::function<void(const std::vector<Packet>&)> callback,
    CalculatorGraphConfig* config, std::pair<std::string, Packet>* side_packet);

void AddMultiStreamCallback(
    const std::vector<std::string>& streams,
    std::function<void(const std::vector<Packet>&)> callback,
    CalculatorGraphConfig* config, std::map<std::string, Packet>* side_packets,
    bool observe_timestamp_bounds = false);

// Add a CallbackWithHeaderCalculator to intercept packets sent on
// stream stream_name, and the header packet on stream stream_header.
// The input side packet with the produced name callback_side_packet_name
// must be set to an appropriate callback before the Graph is run.
// If use_std_function is true, the input side packet of the
// AddCallbackWithHeaderCalculator must be a std::function.
ABSL_DEPRECATED("Header packets are being deprecated.")
void AddCallbackWithHeaderCalculator(const std::string& stream_name,
                                     const std::string& stream_header,
                                     CalculatorGraphConfig* config,
                                     std::string* callback_side_packet_name,
                                     bool use_std_function = false);

// TODO Move CallbackCalculator and CallbackWithHeaderCalculator to
// a separate library, and the library will be alwayslink. Then, the "sink"
// cc_library can depend on that library, and it does not need to be alwayslink.
//
// CallbackCalculator calls a user settable callback on every incoming
// packet.  It must have a single input stream and no output streams.
// A single input side packet must be given which contains a std::function of
// void(const Packet&). The input side packet must have the tag "CALLBACK" in
// the graph config.
//
// Example Usage:
//
// // Callback function.
// void MyClass::MyFunction(const Packet& packet) {
//   count_ += packet.Get<int>();
// }
//
// void MyClass::Run() {
//   CalculatorGraphConfig config;
//   LoadPartialConfigSomehow(&config);
//   std::string input_side_packet_name;
//   tool::AddCallbackCalculator("the_output_stream", &config,
//                               &input_side_packet_name, true);
//   CalculatorGraph graph(config);
//   ABSL_CHECK_OK(graph.Run(
//       {{input_side_packet_name,
//         MakePacket<std::function<void(const Packet&)>>(
//             std::bind(&MyClass::MyFunction, this, std::placeholders::_1))}}
//   ));
// }
class CallbackCalculator : public CalculatorBase {
 public:
  CallbackCalculator() {}
  CallbackCalculator(const CallbackCalculator&) = delete;
  CallbackCalculator& operator=(const CallbackCalculator&) = delete;

  ~CallbackCalculator() override {}

  static absl::Status GetContract(CalculatorContract* cc);

  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;

 private:
  std::function<void(const Packet&)> callback_;
  std::function<void(const std::vector<Packet>&)> vector_callback_;
};

class CallbackWithHeaderCalculator : public CalculatorBase {
 public:
  CallbackWithHeaderCalculator() : callback_(nullptr) {}
  CallbackWithHeaderCalculator(const CallbackWithHeaderCalculator&) = delete;
  CallbackWithHeaderCalculator& operator=(const CallbackWithHeaderCalculator&) =
      delete;

  ~CallbackWithHeaderCalculator() override {}

  static absl::Status GetContract(CalculatorContract* cc);

  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;

 private:
  std::function<void(const Packet&, const Packet&)> callback_;
  // The header packet read from the stream.
  // Header packet is only going to be read once, either during the Open() for
  // the current implementation, or in the Process() call when the header stream
  // has the packet.
  Packet header_packet_;
};

// Produces an output packet with the PostStream timestamp containing the
// input side packet.
class MediaPipeInternalSidePacketToPacketStreamCalculator
    : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc);
  absl::Status Open(CalculatorContext* cc) final;
  absl::Status Process(CalculatorContext* cc) final;
};

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_SINK_H_
