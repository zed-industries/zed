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

#ifndef MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_HANDLER_H_

#include <functional>
#include <memory>
#include <set>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>

// TODO: Move protos in another CL after the C++ code migration.
#include "absl/base/thread_annotations.h"
#include "absl/log/absl_check.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/collection.h"
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/output_stream_manager.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {

// Abstract base class for output stream handlers.
class OutputStreamHandler {
 public:
  // Handy typedef for a map from the name of an output stream to the set of
  // ids of upstream sources that affect it.
  typedef std::unordered_map<std::string, std::unordered_set<int>>
      OutputStreamToSourcesMap;
  typedef internal::Collection<OutputStreamManager*> OutputStreamManagerSet;

  // The constructor of the OutputStreamHandler takes four arguments.
  // The tag_map argument holds the information needed for tag/index retrieval
  // for the output streams; the calculator_context_manager for managing the
  // calculator context objects; the options argument varies for different
  // output stream handler subclasses; the calculator_run_in_parallel argument
  // indicates the execution mode (sequential executions vs parallel executions)
  // of the calcualtor;
  OutputStreamHandler(std::shared_ptr<tool::TagMap> tag_map,
                      CalculatorContextManager* calculator_context_manager,
                      const MediaPipeOptions& options,
                      bool calculator_run_in_parallel)
      : output_stream_managers_(std::move(tag_map)),
        calculator_context_manager_(calculator_context_manager),
        options_(options),
        calculator_run_in_parallel_(calculator_run_in_parallel) {
    ABSL_CHECK(calculator_context_manager_);
  }

  virtual ~OutputStreamHandler() = default;

  const MediaPipeOptions& Options() const { return options_; }

  // Initializes the OutputStreamManagerSet object.
  // flat_output_stream_managers is expected to point to a contiguous
  // flat array with OutputStreamManagers corresponding to the id's in
  // OutputStreamHandler::output_stream_managers_ (meaning it should
  // point to somewhere in the middle of the main flat array of all
  // output stream managers).
  absl::Status InitializeOutputStreamManagers(
      OutputStreamManager* flat_output_stream_managers);

  // Sets up output shards by connecting to the managers.
  absl::Status SetupOutputShards(OutputStreamShardSet* output_shards);

  int NumOutputStreams() const { return output_stream_managers_.NumEntries(); }

  // Returns the tag map of the output streams.
  const std::shared_ptr<tool::TagMap>& OutputTagMap() const {
    return output_stream_managers_.TagMap();
  }

  // Calls OutputStreamManager::PrepareForRun(error_callback) per stream, and
  // resets data memebers.
  void PrepareForRun(const std::function<void(absl::Status)>& error_callback)
      ABSL_LOCKS_EXCLUDED(timestamp_mutex_);

  // Marks the output streams as started and propagates any changes made in
  // Calculator::Open().
  void Open(OutputStreamShardSet* output_shards);

  // Prepares the OutputStreamShardSet before a call to Calculator's Open(),
  // Process(), or Close().
  void PrepareOutputs(Timestamp input_timestamp,
                      OutputStreamShardSet* output_shards);

  // Propagates timestamp directly if there is no ongoing parallel invocation.
  // Otherwise, updates task_timestamp_bound_.
  void UpdateTaskTimestampBound(Timestamp timestamp)
      ABSL_LOCKS_EXCLUDED(timestamp_mutex_);

  // Invoked after a call to Calculator::Process() function.
  void PostProcess(Timestamp input_timestamp)
      ABSL_LOCKS_EXCLUDED(timestamp_mutex_);

  // Propagates the output shards and closes all managed output streams.
  void Close(OutputStreamShardSet* output_shards);

  // Returns the name of the first stream in the output stream manager
  // collection for debugging purpose.
  std::string FirstStreamName() const;

  const OutputStreamManagerSet& OutputStreams() {
    return output_stream_managers_;
  }

 protected:
  // Checks if the given input bound should be propagated or not. If any output
  // streams with OffsetEnabled() need to have the timestamp bounds updated,
  // then propagates the timestamp bounds of all output streams with
  // OffsetEnabled() by adding their offsets to the given input bound.
  void TryPropagateTimestampBound(Timestamp input_bound);

  // Computes the output timestamp bound and propagates output packets to
  // downstream nodes.
  void PropagateOutputPackets(Timestamp input_timestamp,
                              OutputStreamShardSet* output_shards);

  // The packets and timestamp propagation logic for parallel execution.
  virtual void PropagationLoop()
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(timestamp_mutex_) = 0;

  // Collection of all OutputStreamManager objects.
  OutputStreamManagerSet output_stream_managers_;
  // A pointer to the calculator context manager of the calculator node.
  CalculatorContextManager* const calculator_context_manager_;
  MediaPipeOptions options_;
  const bool calculator_run_in_parallel_;

  absl::Mutex timestamp_mutex_;
  // A set of the completed input timestamps in ascending order.
  std::set<Timestamp> completed_input_timestamps_
      ABSL_GUARDED_BY(timestamp_mutex_);
  // The current minimum timestamp for which a new packet could possibly arrive.
  // TODO: Rename the variable to be more descriptive.
  Timestamp task_timestamp_bound_ ABSL_GUARDED_BY(timestamp_mutex_);

  // PropagationState indicates the current state of the propagation process.
  // There are eight possible transitions:
  // (a) From kIdle to kPropagatingPackets.
  // Any thread that makes this transition becomes the propagation thread, and
  // will be responsible for propagating all available packets and the timestamp
  // bound.
  // (b) From kIdle to kPropagatingBound.
  // Any thread that makes this transition becomes the propagation thread, and
  // will be responsible for propagating all available packets and timestamp
  // bound.
  // (c) From kPropagatingPackets to kIdle.
  // Made by the propagation thread after all available packets have been
  // propagated. If timestamp bound propagation isn't necessary, the propagation
  // process is considered to be completed.
  // (d) From kPropagatingPackets to kPropagatingBound.
  // Made by the propagation thread to indicate that packets propagation is
  // completed, and it is going to do timestamp propagation without holding the
  // timestamp_mutex_.
  // (e) From kPropagatingBound to kPropagationPending.
  // Any thread, except the propagation thread, can make this transition.
  // kPropagationPending indicates that some recent changes require the
  // propagation thread to recheck if any new packets or new timestamp bound can
  // be propagated.
  // (f) From kPropagationPending to kPropagatingPackets.
  // Made by the propagation thread to mark that there are some new packets that
  // need to be propagated.
  // (g) From kPropagatingBound to kIdle.
  // Made by the propagation thread when there is no more propagation work to
  // be done.
  // (h) From kPropagationPending to kIdle.
  // Made by the propagation thread when the newly arrived packets and timestamp
  // bound are still not ready to be propagated.
  enum PropagationState {
    kIdle = 0,                //
    kPropagatingPackets = 1,  //
    kPropagatingBound = 2,    //
    kPropagationPending = 3
  };
  PropagationState propagation_state_ ABSL_GUARDED_BY(timestamp_mutex_) = kIdle;
};

using OutputStreamHandlerRegistry = GlobalFactoryRegistry<
    std::unique_ptr<OutputStreamHandler>, std::shared_ptr<tool::TagMap>,
    CalculatorContextManager*, const MediaPipeOptions&, bool>;

}  // namespace mediapipe

// Macro for registering the output stream handler.
#define REGISTER_OUTPUT_STREAM_HANDLER(name)                               \
  REGISTER_FACTORY_FUNCTION_QUALIFIED(                                     \
      mediapipe::OutputStreamHandlerRegistry, output_handler_registration, \
      name,                                                                \
      std::make_unique<name, std::shared_ptr<tool::TagMap>,                \
                       CalculatorContextManager*, const MediaPipeOptions&, \
                       bool>)

#endif  // MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_HANDLER_H_
