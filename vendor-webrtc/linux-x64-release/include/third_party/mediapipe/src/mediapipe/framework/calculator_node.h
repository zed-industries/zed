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

// Declares CalculatorNode which is internally used by the Calculator framework
// (in particular, CalculatorGraph and Calculator) to perform the computations.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_NODE_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_NODE_H_

#include <stddef.h>

#include <functional>
#include <map>
#include <memory>
#include <string>
#include <unordered_map>
#include <unordered_set>

#include "absl/base/macros.h"
#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/synchronization/mutex.h"
#include "absl/time/time.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_base.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/calculator_state.h"
#include "mediapipe/framework/graph_runtime_info.pb.h"
#include "mediapipe/framework/graph_service_manager.h"
#include "mediapipe/framework/input_side_packet_handler.h"
#include "mediapipe/framework/input_stream_handler.h"
#include "mediapipe/framework/legacy_calculator_support.h"
#include "mediapipe/framework/output_side_packet_impl.h"
#include "mediapipe/framework/output_stream_handler.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/stream_handler.pb.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/tool/validate_name.h"
#include "mediapipe/framework/validated_graph_config.h"

namespace mediapipe {

class CounterFactory;
class InputStreamManager;
class OutputStreamManager;

namespace internal {
class SchedulerQueue;
}  // namespace internal

class CalculatorNode {
 public:
  // Handy typedef for a map from the name of an output stream to the set of ids
  // of upstream sources that affect it.
  typedef std::unordered_map<std::string, std::unordered_set<int>>
      OutputStreamToSourcesMap;

  CalculatorNode();
  CalculatorNode(const CalculatorNode&) = delete;
  CalculatorNode& operator=(const CalculatorNode&) = delete;
  int Id() const {
    return node_type_info_ ? node_type_info_->Node().index : -1;
  }

  // Returns a value according to which the scheduler queue determines the
  // relative priority between runnable source nodes; a smaller value means
  // running first. If a node is not a source, this method is not called.
  Timestamp SourceProcessOrder(const CalculatorContext* cc) const;

  // Retrieves a string name for the node.  If the node's name was set in the
  // calculator graph config, it will be returned.  Otherwise, a human-readable
  // string that uniquely identifies the node is returned, e.g.
  // "[FooBarCalculator with first output stream \"foo_bar_output\"]" for
  // non-sink nodes and "[FooBarCalculator with node ID: 42 and input streams:
  // \"foo_bar_input\"]" for sink nodes.  This name should be used in error
  // messages where more context info is helpful.
  std::string DebugName() const;

  // Name of the executor which the node will execute on.  If empty, the node
  // will execute on the default executor.
  const std::string& Executor() const { return executor_; }

  // Changes the executor a node is assigned to.
  void SetExecutor(const std::string& executor);

  // Calls Process() on the Calculator corresponding to this node.
  absl::Status ProcessNode(CalculatorContext* calculator_context);

  // Initializes the node.  The buffer_size_hint argument is
  // set to the value specified in the graph proto for this field.
  // input_stream_managers/output_stream_managers is expected to point to
  // a contiguous flat array with Input/OutputStreamManagers corresponding
  // to the input/output stream indexes in validated_graph.
  // output_side_packets is expected to point to a contiguous flat array with
  // OutputSidePacketImpls corresponding to the output side packet indexes in
  // validated_graph.
  absl::Status Initialize(const ValidatedGraphConfig* validated_graph,
                          NodeTypeInfo::NodeRef node_ref,
                          InputStreamManager* input_stream_managers,
                          OutputStreamManager* output_stream_managers,
                          OutputSidePacketImpl* output_side_packets,
                          int* buffer_size_hint,
                          std::shared_ptr<ProfilingContext> profiling_context,
                          const GraphServiceManager* graph_service_manager);

  // Sets up the node at the beginning of CalculatorGraph::Run(). This
  // method is executed before any OpenNode() calls to the nodes
  // within a CalculatorGraph. Creates a Calculator, and clears the
  // input queues. Sets the callback to run when the node wants to
  // schedule itself for later processing (in the order determined by
  // the priority queue). ready_for_open_callback is called when OpenNode()
  // can be scheduled. source_node_opened_callback is called when a source
  // node is opened. schedule_callback is passed to the InputStreamHandler
  // and is called each time a new invocation can be scheduled.
  absl::Status PrepareForRun(
      const std::map<std::string, Packet>& all_side_packets,
      const std::map<std::string, Packet>& service_packets,
      std::function<void()> ready_for_open_callback,
      std::function<void()> source_node_opened_callback,
      std::function<void(CalculatorContext*)> schedule_callback,
      std::function<void(absl::Status)> error_callback,
      CounterFactory* counter_factory) ABSL_LOCKS_EXCLUDED(status_mutex_);
  // Opens the node.
  absl::Status OpenNode() ABSL_LOCKS_EXCLUDED(status_mutex_);
  // Called when a source node's layer becomes active.
  void ActivateNode() ABSL_LOCKS_EXCLUDED(status_mutex_);
  // Cleans up the node after the CalculatorGraph has been run. Deletes
  // the Calculator managed by this node. graph_status is the status of
  // the graph run.
  void CleanupAfterRun(const absl::Status& graph_status)
      ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Returns true iff PrepareForRun() has been called (and types verified).
  bool Prepared() const ABSL_LOCKS_EXCLUDED(status_mutex_);
  // Returns true iff Open() has been called on the calculator.
  bool Opened() const ABSL_LOCKS_EXCLUDED(status_mutex_);
  // Returns true iff a source calculator's layer is active.
  bool Active() const ABSL_LOCKS_EXCLUDED(status_mutex_);
  // Returns true iff Close() has been called on the calculator.
  bool Closed() const ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Returns true iff this is a source node.
  //
  // A source node has no input streams but has at least one output stream. A
  // node with no input streams and no output streams is essentially a packet
  // generator and is not a source node.
  bool IsSource() const {
    return input_stream_handler_->NumInputStreams() == 0 &&
           output_stream_handler_->NumOutputStreams() != 0;
  }

  int source_layer() const { return source_layer_; }

  // Checks if the node can be scheduled; if so, increases current_in_flight_
  // and returns true; otherwise, returns false.
  // If true is returned, the scheduler must commit to executing the node, and
  // then call EndScheduling when finished running it.
  // If false is returned, the scheduler must not execute the node.
  // This method is thread-safe.
  bool TryToBeginScheduling() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Subtracts one from current_in_flight_ to allow a new invocation to be
  // scheduled. Then, it checks scheduling_state_ and invokes SchedulingLoop()
  // if necessary. This method is thread-safe.
  // TODO: this could be done implicitly by the call to ProcessNode
  // or CloseNode.
  void EndScheduling() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Returns true if OpenNode() can be scheduled.
  bool ReadyForOpen() const ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Called by the InputStreamHandler when all the input stream headers
  // become available.
  void InputStreamHeadersReady() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Called by the InputSidePacketHandler when all the input side packets
  // become available.
  void InputSidePacketsReady() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Checks scheduling_state_, and then invokes SchedulingLoop() if necessary.
  // This method is thread-safe.
  void CheckIfBecameReady() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Called by SchedulerQueue when a node is opened.
  void NodeOpened() ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Returns the scheduler queue the node is assigned to.
  internal::SchedulerQueue* GetSchedulerQueue() const {
    return scheduler_queue_;
  }
  // Sets the scheduler queue the node is assigned to.
  void SetSchedulerQueue(internal::SchedulerQueue* queue) {
    scheduler_queue_ = queue;
  }

  // Sets callbacks in the scheduler that should be invoked when an input queue
  // becomes full/non-full.
  void SetQueueSizeCallbacks(
      InputStreamManager::QueueSizeCallback becomes_full_callback,
      InputStreamManager::QueueSizeCallback becomes_not_full_callback);

  // Sets each of this node's input streams to use the specified
  // max_queue_size to trigger callbacks.
  void SetMaxInputStreamQueueSize(int max_queue_size);

  // Closes the node's calculator and input and output streams.
  // graph_status is the current status of the graph run. graph_run_ended
  // indicates whether the graph run has ended.
  absl::Status CloseNode(const absl::Status& graph_status, bool graph_run_ended)
      ABSL_LOCKS_EXCLUDED(status_mutex_);

  // Returns a pointer to the default calculator context that is used for
  // sequential execution. A source node should always reuse its default
  // calculator context.
  CalculatorContext* GetDefaultCalculatorContext() const {
    return calculator_context_manager_.GetDefaultCalculatorContext();
  }

  const CalculatorState& GetCalculatorState() const {
    return *calculator_state_;
  }

  // Returns the node's contract.
  // Must not be called before the CalculatorNode is initialized.
  const CalculatorContract& Contract() const {
    return node_type_info_->Contract();
  }

  // Returns the last timestamp bound used to schedule this node.
  Timestamp GetLastTimestampBound() const { return last_timestamp_bound_; }

  // Returns the stream monitoring info for this node consisting of a vector of
  // tuples of input stream name, queue size, number of packets added, and
  // minimum timestamp or bound.
  CalculatorRuntimeInfo GetStreamMonitoringInfo() const;

 private:
  // Sets up the output side packets from the main flat array.
  absl::Status InitializeOutputSidePackets(
      const PacketTypeSet& output_side_packet_types,
      OutputSidePacketImpl* output_side_packets);
  // Connects the input side packets as mirrors on the output side packets.
  // Output side packets are looked up in the main flat array which is
  // provided.
  absl::Status InitializeInputSidePackets(
      OutputSidePacketImpl* output_side_packets);
  // Sets up the output streams from the main flat array.
  absl::Status InitializeOutputStreams(
      OutputStreamManager* output_stream_managers);
  // Sets up the input streams and connects them as mirrors on the
  // output streams.  Both input streams and output streams are looked
  // up in the main flat arrays which are provided.
  absl::Status InitializeInputStreams(
      InputStreamManager* input_stream_managers,
      OutputStreamManager* output_stream_managers);

  absl::Status InitializeInputStreamHandler(
      const InputStreamHandlerConfig& handler_config,
      const PacketTypeSet& input_stream_types);
  absl::Status InitializeOutputStreamHandler(
      const OutputStreamHandlerConfig& handler_config,
      const PacketTypeSet& output_stream_types);

  // Connects the input/output stream shards in the given calculator context to
  // the input/output streams of the node.
  absl::Status ConnectShardsToStreams(CalculatorContext* calculator_context);

  // The general scheduling logic shared by EndScheduling() and
  // CheckIfBecameReady().
  // Inside the function, a while loop keeps preparing CalculatorContexts and
  // scheduling the node until 1) the node becomes not ready or 2) the max
  // number of in flight invocations is reached. It also attempts to propagate
  // the latest input timestamp bound if no invocations can be scheduled.
  void SchedulingLoop();

  // Closes the input and output streams.
  void CloseInputStreams() ABSL_LOCKS_EXCLUDED(status_mutex_);
  void CloseOutputStreams(OutputStreamShardSet* outputs)
      ABSL_LOCKS_EXCLUDED(status_mutex_);
  // Get a string describing the input streams.
  std::string DebugInputStreamNames() const;

  // Returns true if all outputs will be identical to the previous graph run.
  bool OutputsAreConstant(CalculatorContext* cc);

  // The calculator.
  std::unique_ptr<CalculatorBase> calculator_;
  // Keeps data which a Calculator subclass needs access to.
  std::unique_ptr<CalculatorState> calculator_state_;

  std::string name_;  // Optional user-defined name
  // Name of the executor which the node will execute on. If empty, the node
  // will execute on the default executor.
  std::string executor_;
  // The layer a source calculator operates on.
  int source_layer_ = 0;
  // The status of the current Calculator that this CalculatorNode
  // is wrapping.  kStateActive is currently used only for source nodes.
  enum NodeStatus {
    kStateUninitialized = 0,
    kStatePrepared = 1,
    kStateOpened = 2,
    kStateActive = 3,
    kStateClosed = 4
  };
  NodeStatus status_ ABSL_GUARDED_BY(status_mutex_){kStateUninitialized};

  // The max number of invocations that can be scheduled in parallel.
  int max_in_flight_ = 1;
  // The following two variables are used for the concurrency control of node
  // scheduling.
  //
  // The number of invocations that are scheduled but not finished.
  int current_in_flight_ ABSL_GUARDED_BY(status_mutex_) = 0;
  // SchedulingState incidates the current state of the node scheduling process.
  // There are four possible transitions:
  // (a) From kIdle to kScheduling.
  // Any thread that makes this transition becomes the scheduling thread and
  // will be responsible for preparing and scheduling all possible invocations.
  // (b) From kScheduling to kSchedulingPending.
  // Any thread, except the scheduling thread, can make this transition.
  // kSchedulingPending indicates that some recent changes require the
  // scheduling thread to recheck the node readiness after current scheduling
  // iteration.
  // (c) From kSchedulingPending to kScheduling.
  // Made by the scheduling thread to indicate that it has already caught up
  // with all the recent changes that can affect node readiness.
  // (d) From kScheduling to kIdle. Made by the scheduling thread when there is
  // no more scheduling work to be done.
  enum SchedulingState {
    kIdle = 0,        //
    kScheduling = 1,  //
    kSchedulingPending = 2
  };
  SchedulingState scheduling_state_ ABSL_GUARDED_BY(status_mutex_) = kIdle;

  std::function<void()> ready_for_open_callback_;
  std::function<void()> source_node_opened_callback_;
  bool input_stream_headers_ready_called_ ABSL_GUARDED_BY(status_mutex_) =
      false;
  bool input_side_packets_ready_called_ ABSL_GUARDED_BY(status_mutex_) = false;
  bool input_stream_headers_ready_ ABSL_GUARDED_BY(status_mutex_) = false;
  bool input_side_packets_ready_ ABSL_GUARDED_BY(status_mutex_) = false;

  // Owns and manages all CalculatorContext objects.
  CalculatorContextManager calculator_context_manager_;

  std::shared_ptr<ProfilingContext> profiling_context_;

  // Mutex for node status.
  mutable absl::Mutex status_mutex_;

  // Describes the input side packets required to run this node.
  std::unique_ptr<PacketTypeSet> input_side_packet_types_;

  // Manages the set of input side packets.
  InputSidePacketHandler input_side_packet_handler_;

  // Collection of all OutputSidePacket objects.
  std::unique_ptr<OutputSidePacketSet> output_side_packets_;

  std::unique_ptr<InputStreamHandler> input_stream_handler_;

  std::unique_ptr<OutputStreamHandler> output_stream_handler_;

  // True if CleanupAfterRun() needs to call CloseNode().
  bool needs_to_close_ = false;

  internal::SchedulerQueue* scheduler_queue_ = nullptr;

  const ValidatedGraphConfig* validated_graph_ = nullptr;

  const NodeTypeInfo* node_type_info_ = nullptr;

  // Keeps track of the latest timestamp bound used to schedule this node.
  Timestamp last_timestamp_bound_ = Timestamp::Unset();

  // Keeps track of the runtime info for this node.
  mutable absl::Mutex runtime_info_mutex_;
  absl::Time last_process_start_ts_ ABSL_GUARDED_BY(runtime_info_mutex_) =
      absl::InfinitePast();
  absl::Time last_process_finish_ts_ ABSL_GUARDED_BY(runtime_info_mutex_) =
      absl::InfinitePast();
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_NODE_H_
