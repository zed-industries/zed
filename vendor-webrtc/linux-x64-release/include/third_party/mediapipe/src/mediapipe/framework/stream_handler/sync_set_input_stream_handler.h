// Copyright 2023 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_SYNC_SET_INPUT_STREAM_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_SYNC_SET_INPUT_STREAM_HANDLER_H_

#include <functional>
#include <memory>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/calculator_context_manager.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/input_stream_handler.h"
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/stream_handler/sync_set_input_stream_handler.pb.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/tool/tag_map.h"

namespace mediapipe {

// An input stream handler which separates the inputs into sets which
// are each independently synchronized.  For example, if 5 inputs are
// present, then the first three can be grouped (and will be synchronized
// as if they were in a calculator with only those three streams) and the
// remaining 2 streams can be independently grouped.  The calculator will
// always be called with all the available packets from a single sync set
// (never more than one).  The input timestamps seen by the calculator
// will be ordered sequentially for each sync set but may jump around
// between sync sets.
class SyncSetInputStreamHandler : public InputStreamHandler {
 public:
  SyncSetInputStreamHandler() = delete;
  SyncSetInputStreamHandler(
      std::shared_ptr<tool::TagMap> tag_map,
      CalculatorContextManager* cc_manager,
      const mediapipe::MediaPipeOptions& extendable_options,
      bool calculator_run_in_parallel)
      : InputStreamHandler(std::move(tag_map), cc_manager, extendable_options,
                           calculator_run_in_parallel) {}

  void PrepareForRun(std::function<void()> headers_ready_callback,
                     std::function<void()> notification_callback,
                     std::function<void(CalculatorContext*)> schedule_callback,
                     std::function<void(absl::Status)> error_callback) override;

 protected:
  // In SyncSetInputStreamHandler, a node is "ready" if any
  // of its sync sets are ready in the traditional sense (See
  // DefaultInputStreamHandler).
  NodeReadiness GetNodeReadiness(Timestamp* min_stream_timestamp) override;

  // Only invoked when associated GetNodeReadiness() returned kReadyForProcess.
  // Populates packets for the ready sync-set, and populates timestamp bounds
  // for all sync-sets.
  void FillInputSet(Timestamp input_timestamp,
                    InputStreamShardSet* input_set) override;

  // Populates timestamp bounds for streams outside the ready sync-set.
  void FillInputBounds(Timestamp input_timestamp,
                       InputStreamShardSet* input_set)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Returns the number of sync-sets maintained by this input-handler.
  int SyncSetCount() override;

 private:
  absl::Mutex mutex_;
  // The ids of each set of inputs.
  std::vector<SyncSet> sync_sets_ ABSL_GUARDED_BY(mutex_);
  // The index of the ready sync set.  A value of -1 indicates that no
  // sync sets are ready.
  int ready_sync_set_index_ ABSL_GUARDED_BY(mutex_) = -1;
  // The timestamp at which the sync set is ready.  If no sync set is
  // ready then this variable should be Timestamp::Done() .
  Timestamp ready_timestamp_ ABSL_GUARDED_BY(mutex_);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_STREAM_HANDLER_SYNC_SET_INPUT_STREAM_HANDLER_H_
