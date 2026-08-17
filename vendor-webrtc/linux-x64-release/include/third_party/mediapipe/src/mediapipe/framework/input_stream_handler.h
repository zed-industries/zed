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

#ifndef MEDIAPIPE_FRAMEWORK_INPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_INPUT_STREAM_HANDLER_H_

#include <atomic>
#include <functional>
#include <list>
#include <memory>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

// TODO: Move protos in another CL after the C++ code migration.
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/collection.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/input_stream_manager.h"
#include "mediapipe/framework/input_stream_shard.h"
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {

// Indicates the operation the node is ready for.
enum class NodeReadiness {
  // The node is not ready.
  kNotReady,
  // The node is ready and we should run Process().
  kReadyForProcess,
  // The node is ready and we should run Close().
  kReadyForClose,
};

// Abstract base class for input stream handlers.
//
// The input stream handler is invoked every time a Packet is enqueued or the
// next timestamp bound is set in any of the input streams and actually decides
// whether a calculator node's Process() should be called. The input stream
// handler owns and manages the InputStreamManager objects. The operations
// performed by OutputStreamHandler on an InputStreamManager object need to go
// through its input stream handler.
//
// Derived classes are required to implement the following member functions:
//
//   NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp);
//   void FillInputSet(Timestamp input_timestamp,
//                     InputStreamShardSet* input_set);
//
class InputStreamHandler {
 public:
  InputStreamHandler(std::shared_ptr<tool::TagMap> tag_map,
                     CalculatorContextManager* calculator_context_manager,
                     const MediaPipeOptions& options,
                     bool calculator_run_in_parallel)
      : input_stream_managers_(std::move(tag_map)),
        calculator_context_manager_(calculator_context_manager),
        options_(options),
        calculator_run_in_parallel_(calculator_run_in_parallel) {}

  virtual ~InputStreamHandler() = default;

  // Initializes the InputStreamManagerSet object.
  // flat_input_stream_managers is expected to point to a contiguous
  // flat array with InputStreamManagers corresponding to the id's in
  // InputStreamHandler::input_stream_managers_ (meaning it should point
  // to somewhere in the middle of the main flat array of all input
  // stream managers).
  absl::Status InitializeInputStreamManagers(
      InputStreamManager* flat_input_stream_managers);

  InputStreamManager* GetInputStreamManager(CollectionItemId id);

  // Sets up the InputStreamShardSet by propagating data from the managers.
  absl::Status SetupInputShards(InputStreamShardSet* input_shards);

  // Returns a vector of tuples of stream name, queue size, number of packets
  // added, and minimum timestamp or bound for monitoring purpose.
  std::vector<std::tuple<std::string, int, int, Timestamp>> GetMonitoringInfo();

  // Resets the input stream handler and its underlying input streams for
  // another run of the graph.
  // InputStreamHandler subclass can override this function to meet some special
  // needs while preparing the calculator node for run.
  // TODO: Makes PrepareForRun() non-virtual and makes
  // InitializeInputStreamManagers() virtual instead.
  virtual void PrepareForRun(
      std::function<void()> headers_ready_callback,
      std::function<void()> notification_callback,
      std::function<void(CalculatorContext*)> schedule_callback,
      std::function<void(absl::Status)> error_callback);

  int NumInputStreams() const { return input_stream_managers_.NumEntries(); }

  // Returns the tag map of the input streams.
  const std::shared_ptr<tool::TagMap>& InputTagMap() const {
    return input_stream_managers_.TagMap();
  }

  // Sets header into a particular stream.
  void SetHeader(CollectionItemId id, const Packet& header);

  // Updates the header packets in the input shards.
  void UpdateInputShardHeaders(InputStreamShardSet* input_shards);

  // Sets max queue size of every stream.
  void SetMaxQueueSize(int max_queue_size);

  // Sets max queue size of a particular stream.
  void SetMaxQueueSize(CollectionItemId id, int max_queue_size);

  void SetQueueSizeCallbacks(
      InputStreamManager::QueueSizeCallback becomes_full_callback,
      InputStreamManager::QueueSizeCallback becomes_not_full_callback);

  // Add packets into a particular stream.
  virtual void AddPackets(CollectionItemId id,
                          const std::list<Packet>& packets);

  // Moves packets into a particular stream.
  virtual void MovePackets(CollectionItemId id, std::list<Packet>* packets);

  // Sets next timestamp bound in a particular stream.
  void SetNextTimestampBound(CollectionItemId id, Timestamp bound);

  // Clears the current packet of every stream shard and removes the current
  // timestamp from the calculator context.
  void ClearCurrentInputs(CalculatorContext* calculator_context);

  void Close();

  // Returns a string that concatenates the stream names of all managed streams.
  std::string DebugStreamNames() const;

  // Return the stream name for an input stream in the format:
  // stream_tag:stream_index:stream_name.
  std::string DebugStreamName(CollectionItemId id) const;

  // Returns the node name of the calculator node.
  std::string GetNodeName() const;

  // Keeps scheduling new invocations until 1) the node is not ready or 2) the
  // max number of invocations that are allowed to be scheduled is reached.
  // Returns true if at least one invocation has been scheduled.
  // The latest minimum timestamp bound of the input streams is returned in
  // *input_bound if the latest readiness of the node is kNotReady when the
  // function returns. During batching, this value will be equal to the
  // timestamp of the first set of inputs in the batch. In other cases,
  // Timestamp::Unset() is returned.
  // This method can only be invoked in the schedule phase.
  bool ScheduleInvocations(int max_allowance, Timestamp* input_bound);

  // Finalizes the input set before the calculator node's Process() is
  // called at the given input timestamp.
  void FinalizeInputSet(Timestamp timestamp, InputStreamShardSet* input_set);

  // Returns the number of input stream headers (excluding headers of back
  // edges) that are not set.
  int UnsetHeaderCount() const {
    return unset_header_count_.load(std::memory_order_relaxed);
  }

  // When true, Calculator::Process is called for any increase in the
  // timestamp bound, whether or not any packets are available.
  // Calculator::Process is called when the minimum timestamp bound
  // increases for any synchronized set of input streams.
  // DefaultInputStreamHandler groups all input streams into a single set.
  // ImmediateInputStreamHandler treats each input stream as a separate set.
  void SetProcessTimestampBounds(bool process_ts) {
    process_timestamps_ = process_ts;
  }

  // When true, Calculator::Process is called for every input timestamp bound.
  bool ProcessTimestampBounds() { return process_timestamps_; }

  // Returns the number of sync-sets populated by this input stream handler.
  virtual int SyncSetCount() { return 1; }

  // A helper class to build input packet sets for a certain set of streams.
  //
  // ReadyForProcess requires all of the streams to be fully determined
  // at the same input-timestamp.
  // This is the readiness policy for all streams in DefaultInputStreamHandler.
  // It is also the policy for each sync-set in SyncSetInputStreamHandler.
  // It is also the policy for each input-stream in ImmediateInputStreamHandler.
  //
  // If ProcessTimestampBounds() is set, then a fully determined input timestamp
  // with only empty input packets will qualify as ReadyForProcess.
  class SyncSet {
   public:
    // Creates a SyncSet for a certain set of streams, |stream_ids|.
    SyncSet(InputStreamHandler* input_stream_handler,
            std::vector<CollectionItemId> stream_ids);

    // Reinitializes this SyncSet before each CalculatorGraph run.
    void PrepareForRun();

    // Answers whether this stream is ready for Process or Close.
    NodeReadiness GetReadiness(Timestamp* min_stream_timestamp);

    // Returns the latest timestamp returned for processing.
    Timestamp LastProcessed() const;

    // The earliest available packet timestamp, or Timestamp::Done.
    Timestamp MinPacketTimestamp() const;

    // Moves packets from all input streams to the input_set.
    void FillInputSet(Timestamp input_timestamp,
                      InputStreamShardSet* input_set);

    // Copies timestamp bounds from all input streams to the input_set.
    void FillInputBounds(InputStreamShardSet* input_set);

   private:
    InputStreamHandler* input_stream_handler_;
    std::vector<CollectionItemId> stream_ids_;
    Timestamp last_processed_ts_ = Timestamp::Unset();
  };

 protected:
  typedef internal::Collection<InputStreamManager*> InputStreamManagerSet;

  const MediaPipeOptions& options() const { return options_; }

  // Subclasses must set batch size to greater than 1 to enable batching.
  // Batching cannot be combined with late_preparation_ behavior.
  void SetBatchSize(int batch_size);

  // Subclasses can enable late preparation; however it cannot be used along
  // with batching.
  void SetLatePreparation(bool late_preparation);

  // Accesses InputStreamShard's private method to add packets. All
  // InputStreamHandler subclasses are only able to add packets to shards
  // through this method.
  // TODO: Renames this method to "SetStreamContents" when
  // InputStreamShard is renamed.
  void AddPacketToShard(InputStreamShard* shard, Packet&& value, bool is_done) {
    shard->AddPacket(std::move(value), is_done);
  }

  // Returns the operation the calculator node is ready for.
  // Specifically:
  // - NodeReadiness::kNotReady if the node's Process() or Close() cannot be
  //   called yet. The minimum timestamp bound of the input streams is
  //   returned in *min_stream_timestamp.
  // - NodeReadiness::kReadyForProcess if the node is not waiting for a packet
  //   on any input stream and is thus "ready" for its Process() to be
  //   called. For a source node, Timestamp::Done() is returned in
  //   *min_stream_timestamp. For a non-source node, the input timestamp for
  //   the Process() call is returned in *min_stream_timestamp.
  // - NodeReadiness::kReadyForClose if the node's Close() can be called.
  //   Timestamp::Done() is returned in *min_stream_timestamp.
  virtual NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) = 0;

  // Moves input packets from the input streams into the input set for the given
  // input timestamp.
  virtual void FillInputSet(Timestamp input_timestamp,
                            InputStreamShardSet* input_set) = 0;

  // Collection of InputStreamManager objects.
  InputStreamManagerSet input_stream_managers_;
  // A pointer to the calculator context manager of the calculator node.
  CalculatorContextManager* const calculator_context_manager_;
  MediaPipeOptions options_;
  const bool calculator_run_in_parallel_;
  // Indicates if a calculator context has been prepared for Close().
  bool prepared_context_for_close_;

  // Note: You must not hold any locks while invoking any of these callbacks.
  //
  // A callback to notify the observer of the changes on the packet queue or
  // the timestamp bound.
  std::function<void()> notification_;
  // A callback to schedule the node with the prepared calculator context.
  std::function<void(CalculatorContext*)> schedule_callback_;
  std::function<void(absl::Status)> error_callback_;

 private:
  // Indicates when to fill the input set. If true, every input set will be
  // prepared in FinalizeInputSet(). Otherwise, the input sets will be filled
  // in ScheduleInvocations() in the scheduling phase.
  // The variable is set to false by default. A subclass should set it to true
  // with SetLatePreparation(true) in the constructor if the input sets need to
  // be filled in ProcessNode().
  bool late_preparation_ = false;

  // Determines how many sets of input packets are collected before a
  // CalculatorNode is scheduled.
  int batch_size_ = 1;

  // When true, any increase in timestamp bound invokes Calculator::Process.
  bool process_timestamps_ = false;

  // A callback to notify the observer when all the input stream headers
  // (excluding headers of back edges) become available.
  std::function<void()> headers_ready_callback_;

  std::atomic<int> unset_header_count_{0};
};

using InputStreamHandlerRegistry = GlobalFactoryRegistry<
    std::unique_ptr<InputStreamHandler>, std::shared_ptr<tool::TagMap>,
    CalculatorContextManager*, const MediaPipeOptions&, bool>;

}  // namespace mediapipe

// Macro for registering the input stream handler.
#define REGISTER_INPUT_STREAM_HANDLER(name)                                    \
  REGISTER_FACTORY_FUNCTION_QUALIFIED(                                         \
      mediapipe::InputStreamHandlerRegistry, input_handler_registration, name, \
      std::make_unique<name, std::shared_ptr<tool::TagMap>,                    \
                       CalculatorContextManager*, const MediaPipeOptions&,     \
                       bool>)

#endif  // MEDIAPIPE_FRAMEWORK_INPUT_STREAM_HANDLER_H_
