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

#ifndef MEDIAPIPE_FRAMEWORK_VALIDATED_GRAPH_CONFIG_H_
#define MEDIAPIPE_FRAMEWORK_VALIDATED_GRAPH_CONFIG_H_

#include <map>
#include <string>
#include <vector>

#include "absl/container/flat_hash_set.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/graph_service_manager.h"
#include "mediapipe/framework/packet_generator.pb.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/map_util.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/status_builder.h"
#include "mediapipe/framework/status_handler.pb.h"
#include "mediapipe/framework/subgraph.h"

namespace mediapipe {

class ValidatedGraphConfig;

std::string DebugEdgeNames(
    const std::string& edge_type,
    const proto_ns::RepeatedPtrField<ProtoString>& edges);

std::string DebugName(const CalculatorGraphConfig::Node& node_config);

// Type information for a graph node (Calculator, Generator, etc).
class NodeTypeInfo {
 public:
  enum class NodeType {
    UNKNOWN = 0,
    CALCULATOR = 1,
    PACKET_GENERATOR = 2,
    GRAPH_INPUT_STREAM = 3,  // The virtual node parent of a graph input stream.
    STATUS_HANDLER = 4,
  };

  struct NodeRef {
    NodeRef() = default;
    NodeRef(NodeType node_type, int node_index)
        : type(node_type), index(node_index) {}

    NodeType type = NodeType::UNKNOWN;
    // The index of a graph node among the nodes of the same type in the
    // validated graph config.
    int index = -1;
  };

  NodeTypeInfo() = default;
  ~NodeTypeInfo() = default;
  // Don't allow copy or assign (PacketTypeSet does not support copy).
  NodeTypeInfo(const NodeTypeInfo& other) = delete;
  NodeTypeInfo& operator=(const NodeTypeInfo& other) = delete;
  // Allow move (so we can create a std::vector with this type).
  NodeTypeInfo(NodeTypeInfo&& other) = default;

  // node_index is the index of this node among the nodes of the same type
  // in the validated graph config.
  absl::Status Initialize(const ValidatedGraphConfig& validated_graph,
                          const CalculatorGraphConfig::Node& node,
                          int node_index);
  absl::Status Initialize(const ValidatedGraphConfig& validated_graph,
                          const PacketGeneratorConfig& node, int node_index);
  absl::Status Initialize(const ValidatedGraphConfig& validated_graph,
                          const StatusHandlerConfig& node, int node_index);

  // TODO: many of these accessors can be replaced by Contract().
  const PacketTypeSet& InputSidePacketTypes() const {
    return contract_.InputSidePackets();
  }
  const PacketTypeSet& OutputSidePacketTypes() const {
    return contract_.OutputSidePackets();
  }
  const PacketTypeSet& InputStreamTypes() const { return contract_.Inputs(); }
  const PacketTypeSet& OutputStreamTypes() const { return contract_.Outputs(); }

  const CalculatorContract& Contract() const { return contract_; }

  // Non-const accessors.
  PacketTypeSet& InputSidePacketTypes() { return contract_.InputSidePackets(); }
  PacketTypeSet& OutputSidePacketTypes() {
    return contract_.OutputSidePackets();
  }
  PacketTypeSet& InputStreamTypes() { return contract_.Inputs(); }
  PacketTypeSet& OutputStreamTypes() { return contract_.Outputs(); }

  // Get the input/output side packet/stream index that is the first
  // for the PacketTypeSets.  Subsequent id's in the collection are
  // guaranteed to be contiguous in the main flat array.
  int InputSidePacketBaseIndex() const { return input_side_packet_base_index_; }
  int OutputSidePacketBaseIndex() const {
    return output_side_packet_base_index_;
  }
  int InputStreamBaseIndex() const { return input_stream_base_index_; }
  int OutputStreamBaseIndex() const { return output_stream_base_index_; }

  // Get the type and index of this node.
  const NodeRef& Node() const { return node_; }

  // Setter methods for the indexes.  This should only be used by
  // ValidatedGraphConfig.
  void SetInputSidePacketBaseIndex(int index) {
    input_side_packet_base_index_ = index;
  }
  void SetOutputSidePacketBaseIndex(int index) {
    output_side_packet_base_index_ = index;
  }
  void SetInputStreamBaseIndex(int index) { input_stream_base_index_ = index; }
  void SetOutputStreamBaseIndex(int index) {
    output_stream_base_index_ = index;
  }
  void SetNodeIndex(int index) { node_.index = index; }

  // Get the indexes (in ValidatedGraphConfig::Calculator's flat array)
  // of the source nodes which affect this node.  The index can also
  // be a virtual node corresponding to a graph input stream (which are
  // listed by index contiguously after all calculators).
  // This function is only valid for a NodeTypeInfo of NodeType CALCULATOR.
  const absl::flat_hash_set<int>& AncestorSources() const {
    return ancestor_sources_;
  }
  // Returns True if the source was not already there.
  // This function is only valid for a NodeTypeInfo of NodeType CALCULATOR.
  bool AddSource(int index) { return ancestor_sources_.insert(index).second; }

  // Convert the NodeType enum into a string (generally for error messaging).
  static std::string NodeTypeToString(NodeType node_type);

  // Returns the name of the specified InputStreamHandler, or empty string if
  // none set.
  std::string GetInputStreamHandler() const {
    return contract_.GetInputStreamHandler();
  }

  // Returns the MediaPipeOptions specified, or empty options if none set.
  MediaPipeOptions GetInputStreamHandlerOptions() const {
    return contract_.GetInputStreamHandlerOptions();
  }

 private:
  // This object owns the PacketType objects (which are referenced by
  // ValidatedGraphConfig::EdgeInfo objects).
  CalculatorContract contract_;

  // The base indexes of the first entry belonging to this node in
  // the main flat arrays of ValidatedGraphConfig.  Subsequent
  // entries are guaranteed to be sequential and in the order of the
  // CollectionItemIds.
  // Example:
  //   all_input_streams
  //     [node_info.InputStreamBaseIndex() +
  //      node_info.InputStreamTypes().GetId("TAG", 2).value()];
  int input_side_packet_base_index_ = 0;
  int output_side_packet_base_index_ = 0;
  int input_stream_base_index_ = 0;
  int output_stream_base_index_ = 0;

  // The type and index of this node.
  NodeRef node_;

  // The set of sources which affect this node.
  absl::flat_hash_set<int> ancestor_sources_;
};

// Information for either the input or output side of an edge.  An edge
// is either a side packet or a stream.
struct EdgeInfo {
  // For an input edge (input side packet, or input stream) this is the
  // index of the corresponding output side which produces the data this
  // edge will see.
  int upstream = -1;
  // The parent node which owns this edge.  For graph input streams this
  // is a virtual node (in which case there is no corresponding owning
  // node in calculators_).
  NodeTypeInfo::NodeRef parent_node;
  std::string name;
  PacketType* packet_type = nullptr;
  bool back_edge = false;  // Only applicable to input streams.
};

// This class is used to validate and canonicalize a CalculatorGraphConfig.
class ValidatedGraphConfig {
 public:
  // Initializes the ValidatedGraphConfig.  This function must be called
  // before any other functions.  Subgraphs are specified through the
  // global graph registry or an optional local graph registry.
  absl::Status Initialize(
      CalculatorGraphConfig input_config,
      const GraphRegistry* graph_registry = nullptr,
      const Subgraph::SubgraphOptions* graph_options = nullptr,
      const GraphServiceManager* service_manager = nullptr);

  // Initializes the ValidatedGraphConfig from registered graph and subgraph
  // configs.  Subgraphs are retrieved from the specified graph registry or from
  // the global graph registry.  A subgraph can be instantiated directly by
  // specifying its type in |graph_type|.
  absl::Status Initialize(
      const std::string& graph_type,
      const GraphRegistry* graph_registry = nullptr,
      const Subgraph::SubgraphOptions* graph_options = nullptr,
      const GraphServiceManager* service_manager = nullptr);

  // Initializes the ValidatedGraphConfig from the specified graph and subgraph
  // configs.  Template graph and subgraph configs can be specified through
  // |input_templates|.  Every subgraph must have its graph type specified in
  // CalclatorGraphConfig.type.  A subgraph can be instantiated directly by
  // specifying its type in |graph_type|.  A template graph can be instantiated
  // directly by specifying its template arguments in |arguments|.
  absl::Status Initialize(
      const std::vector<CalculatorGraphConfig>& input_configs,
      const std::vector<CalculatorGraphTemplate>& input_templates,
      const std::string& graph_type = "",
      const Subgraph::SubgraphOptions* graph_options = nullptr,
      const GraphServiceManager* service_manager = nullptr);

  // Returns true if the ValidatedGraphConfig has been initialized.
  bool Initialized() const { return initialized_; }

  // Returns an error if the provided side packets will be generated by
  // the PacketGenerators in this graph.
  template <typename T>
  absl::Status CanAcceptSidePackets(
      const std::map<std::string, T>& side_packets) const;

  // Validate that all the required side packets are provided, and the
  // packets have the required type.
  absl::Status ValidateRequiredSidePackets(
      const std::map<std::string, Packet>& side_packets) const;
  // Same as ValidateRequiredSidePackets but only provide the type.
  absl::Status ValidateRequiredSidePacketTypes(
      const std::map<std::string, PacketType>& side_packet_types) const;

  // The proto configuration (canonicalized).
  const CalculatorGraphConfig& Config() const { return config_; }

  // Accessors for the info objects.
  const std::vector<NodeTypeInfo>& CalculatorInfos() const {
    return calculators_;
  }
  const std::vector<NodeTypeInfo>& GeneratorInfos() const {
    return generators_;
  }
  const std::vector<NodeTypeInfo>& StatusHandlerInfos() const {
    return status_handlers_;
  }
  const std::vector<EdgeInfo>& InputStreamInfos() const {
    return input_streams_;
  }
  const std::vector<EdgeInfo>& OutputStreamInfos() const {
    return output_streams_;
  }
  const std::vector<EdgeInfo>& InputSidePacketInfos() const {
    return input_side_packets_;
  }
  const std::vector<EdgeInfo>& OutputSidePacketInfos() const {
    return output_side_packets_;
  }

  int OutputStreamIndex(const std::string& name) const {
    return FindWithDefault(stream_to_producer_, name, -1);
  }

  int OutputSidePacketIndex(const std::string& name) const {
    return FindWithDefault(side_packet_to_producer_, name, -1);
  }

  int OutputStreamToNode(const std::string& name) const {
    auto iter = stream_to_producer_.find(name);
    if (iter == stream_to_producer_.end()) {
      return -1;
    }
    return output_streams_[iter->second].parent_node.index;
  }

  std::vector<int> OutputStreamToConsumers(int idx) const {
    auto iter = output_streams_to_consumer_nodes_.find(idx);
    if (iter == output_streams_to_consumer_nodes_.end()) {
      return {};
    }
    return iter->second;
  }

  // Returns the registered type name of the specified side packet if
  // it can be determined, otherwise an appropriate error is returned.
  absl::StatusOr<std::string> RegisteredSidePacketTypeName(
      const std::string& name);
  // Returns the registered type name of the specified stream if it can
  // be determined, otherwise an appropriate error is returned.
  absl::StatusOr<std::string> RegisteredStreamTypeName(const std::string& name);

  // The namespace used for class name lookup.
  std::string Package() const { return config_.package(); }

  // Returns true if |name| is a reserved executor name.
  static bool IsReservedExecutorName(const std::string& name);

  // Returns true if a side packet is provided as an input to the graph.
  bool IsExternalSidePacket(const std::string& name) const {
    return required_side_packets_.count(name) > 0;
  }

 private:
  // Perform transforms such as converting legacy features, expanding
  // subgraphs, and popluting input stream handler.
  absl::Status PerformBasicTransforms(
      const GraphRegistry* graph_registry,
      const Subgraph::SubgraphOptions* graph_options,
      const GraphServiceManager* service_manager);

  // Initialize the PacketGenerator information.
  absl::Status InitializeGeneratorInfo();
  // Initialize the Calculator information.
  absl::Status InitializeCalculatorInfo();
  // Initialize the StatusHandler information.
  absl::Status InitializeStatusHandlerInfo();

  // Initialize the EdgeInfo objects for side packets.
  //
  // If need_sorting_ptr is non-null it will be set to true iff the side
  // packet graph is not topologically sorted.  If the nodes in the side
  // packet graph are not in sorted order, then side_packet_to_producer_
  // will still be complete, but the upstream field of input_side_packets_
  // may not be accurate.
  //
  // If need_sorting_ptr is nullptr then an error will be returned if the
  // nodes in the side packet graph are not in topologically sorted order.
  absl::Status InitializeSidePacketInfo(bool* need_sorting_ptr);
  // Adds EdgeInfo objects to input_side_packets_ for all the input side
  // packets required by the node_type_info.  If nodes are processed
  // with AddInputSidePacketsForNode and AddOutputSidePacketsForNode
  // sequentially, then side_packet_to_producer_ and
  // required_side_packets_ are used to ensure that the graph is
  // topologically sorted.  node_type_info is updated with the proper
  // initial index for input side packets.
  absl::Status AddInputSidePacketsForNode(NodeTypeInfo* node_type_info);
  // Adds EdgeInfo objects to output_side_packets_ for all the output side
  // packets produced by the node_type_info.  side_packet_to_producer_ is
  // updated.  need_sorting_ptr will be set to true if the nodes in the
  // side packet graph are detected to be in unsorted order (a side packet
  // is output after something that required it), otherwise need_sorting_ptr
  // is left as is.  node_type_info is updated with the proper initial index
  // for output side packets.
  absl::Status AddOutputSidePacketsForNode(NodeTypeInfo* node_type_info,
                                           bool* need_sorting_ptr);

  // These functions are analogous to the same operations for side
  // packets, with the small difference that it is an error to use an
  // undefined stream (whereas it is allowed to use an undefined side
  // packet).
  absl::Status InitializeStreamInfo(bool* need_sorting_ptr);
  absl::Status AddOutputStreamsForNode(NodeTypeInfo* node_type_info);
  absl::Status AddInputStreamsForNode(NodeTypeInfo* node_type_info,
                                      bool* need_sorting_ptr);
  // A helper function for adding a single output stream EdgeInfo.
  absl::Status AddOutputStream(NodeTypeInfo::NodeRef node,
                               const std::string& name,
                               PacketType* packet_type);

  // Return the index of the node adjusted for the topological sorter.
  int SorterIndexForNode(NodeTypeInfo::NodeRef node) const;

  // Convert the index for the topological sorter back to the node type
  // and node index.
  NodeTypeInfo::NodeRef NodeForSorterIndex(int index) const;

  // Sort the nodes based on the information gotten from
  // InitializeSidePacketInfo and InitializeStreamInfo.  After this
  // function, the InitializeSidePacketInfo and InitializeStreamInfo
  // functions must be run again (after clearing the data structures they
  // fill).
  //
  // NOTE: Only the generators and calculators need to be sorted. The other
  // two node types, graph input streams and status handlers, can be safely
  // ignored in the analysis of output side packet generation or stream
  // header packet propagation.
  absl::Status TopologicalSortNodes();

  // TODO Add InputStreamHandler.
  // TODO Add OutputStreamHandler.

  // Fill the "upstream" field for all back edges.
  absl::Status FillUpstreamFieldForBackEdges();

  // Compute the dependence of nodes on sources.
  absl::Status ComputeSourceDependence();

  // Infer the type of types set to "Any" by what they are connected to.
  absl::Status ResolveAnyTypes(std::vector<EdgeInfo>* input_edges,
                               std::vector<EdgeInfo>* output_edges);
  // Narrow down OneOf types if they other end is a single type.
  absl::Status ResolveOneOfTypes(std::vector<EdgeInfo>* input_edges,
                                 std::vector<EdgeInfo>* output_edges);

  // Returns an error if the generator graph does not have consistent
  // type specifications for side packets.
  absl::Status ValidateSidePacketTypes();
  // Returns an error if the graph of calculators does not have consistent
  // type specifications for streams.
  absl::Status ValidateStreamTypes();
  // Returns an error if the graph does not have valid ExecutorConfigs, or
  // if the executor name in a node config is reserved or is not declared
  // in an ExecutorConfig.
  absl::Status ValidateExecutors();

  bool initialized_ = false;

  CalculatorGraphConfig config_;

  // The type information for each node type.
  std::vector<NodeTypeInfo> calculators_;
  std::vector<NodeTypeInfo> generators_;
  std::vector<NodeTypeInfo> status_handlers_;

  // NodeTypeInfo's of generators and calculators, topologically sorted.
  std::vector<NodeTypeInfo*> sorted_nodes_;

  // Mapping from stream name to the output_streams_ index which produces it.
  std::map<std::string, int> stream_to_producer_;

  // Mapping from output streams to consumer node ids. Used for profiling.
  std::map<int, std::vector<int>> output_streams_to_consumer_nodes_;

  // Mapping from side packet name to the output_side_packets_ index
  // which produces it.
  std::map<std::string, int> side_packet_to_producer_;

  // A structure to manage deletion of PacketType objects which need to
  // be owned by this object (used for graph input stream PacketType).
  std::vector<std::unique_ptr<PacketType>> owned_packet_types_;

  // For each side packet which must still be supplied, a list of
  // input_side_packets_ indexes which must be validated against it.
  // TODO Use the information stored here for more thorough
  // validation.
  std::map<std::string, std::vector<int>> required_side_packets_;

  // The EdgeInfo objects for input/output side packets/streams.
  std::vector<EdgeInfo> input_streams_;
  std::vector<EdgeInfo> output_streams_;
  std::vector<EdgeInfo> input_side_packets_;
  std::vector<EdgeInfo> output_side_packets_;
};

template <typename T>
absl::Status ValidatedGraphConfig::CanAcceptSidePackets(
    const std::map<std::string, T>& side_packets) const {
  for (const auto& output_side_packet : output_side_packets_) {
    if (ContainsKey(side_packets, output_side_packet.name)) {
      return mediapipe::UnknownErrorBuilder(MEDIAPIPE_LOC)
             << "Side packet \"" << output_side_packet.name
             << "\" is both provided and generated by a PacketGenerator.";
    }
  }
  return absl::OkStatus();
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_VALIDATED_GRAPH_CONFIG_H_
